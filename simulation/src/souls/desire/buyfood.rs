use serde::{Deserialize, Serialize};

use egui_inspect::Inspect;
use geom::Transform;
use prototypes::{GameInstant, GameTime, ItemID};

use crate::economy::{find_trade_place, Bought, Market};
use crate::map::{BuildingID, Map};
use crate::map_dynamic::{BuildingInfos, Destination};
use crate::souls::human::HumanDecisionKind;
use crate::transportation::Location;
use crate::world::{HumanEnt, HumanID};
use crate::{ParCommandBuffer, SoulID};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum BuyFoodState {
    Empty,
    WaitingForTrade,
    BoughtAt(BuildingID),
}

debug_inspect_impl!(BuyFoodState);

#[derive(Inspect, Clone, Serialize, Deserialize, Debug)]
pub struct BuyFood {
    pub last_ate: GameInstant,
    state: BuyFoodState,
    pub last_score: f32,
}

impl BuyFood {
    pub fn new(start: GameInstant) -> Self {
        BuyFood {
            last_ate: start,
            state: BuyFoodState::Empty,
            last_score: 0.0,
        }
    }

    /// Test-only: force straight into `BoughtAt(b)` without driving the real
    /// `Empty -> WaitingForTrade -> BoughtAt` transitions, so a test can
    /// exercise the eat-time-vs-expired-claim branch in isolation.
    #[cfg(test)]
    pub(crate) fn set_state_bought_at_for_test(&mut self, b: BuildingID) {
        self.state = BuyFoodState::BoughtAt(b);
    }

    pub fn score(&self, time: &GameTime, loc: &Location, bought: &Bought) -> f32 {
        if matches!(self.state, BuyFoodState::WaitingForTrade)
            && bought
                .0
                .get(&ItemID::new("bread"))
                .map(Vec::is_empty)
                .unwrap_or(false)
        {
            return 0.0;
        }
        if let BuyFoodState::BoughtAt(id) = self.state {
            if loc == &Location::Building(id) {
                return 1.0;
            }
        }
        self.last_ate.elapsed(time).seconds() as f32 / GameTime::DAY as f32 - 1.0
    }

    pub fn apply(
        &mut self,
        cbuf: &ParCommandBuffer<HumanEnt>,
        binfos: &BuildingInfos,
        map: &Map,
        market: &Market,
        time: &GameTime,
        id: HumanID,
        trans: &Transform,
        loc: &Location,
        bought: &mut Bought,
    ) -> HumanDecisionKind {
        use HumanDecisionKind::*;
        match self.state {
            BuyFoodState::Empty => {
                let pos = trans.pos;
                cbuf.exec_on(id, move |market: &mut Market| {
                    market.buy(SoulID::Human(id), pos.xy(), ItemID::new("bread"), 1)
                });
                self.state = BuyFoodState::WaitingForTrade;
                Yield
            }
            BuyFoodState::WaitingForTrade => {
                for trade in bought.0.entry(ItemID::new("bread")).or_default().drain(..) {
                    if let Some(b) = find_trade_place(trade.seller, binfos) {
                        self.state = BuyFoodState::BoughtAt(b);
                    }
                }
                // A live buy order means never matched yet: keep waiting
                // (parked at score 0.0 above). Once the order is gone (a
                // match consumes it, see `Market::make_trades`) and there's
                // no retail claim either, the claim that stood in for it
                // has since expired or been released -- going without is the
                // intended outcome, not a bug, so re-queue instead of
                // waiting on a reservation that no longer exists. Checking
                // the order rather than "was a claim ever observed" matters
                // because `apply` isn't polled every tick: a claim can be
                // created and expire entirely between two calls. `last_ate`
                // must NOT advance here: hunger keeps rising (never game
                // over).
                if matches!(self.state, BuyFoodState::WaitingForTrade)
                    && market
                        .inner()
                        .get(&ItemID::new("bread"))
                        .and_then(|sm| sm.buy_order(SoulID::Human(id)))
                        .is_none()
                    && market.retail_claim(SoulID::Human(id)).is_none()
                {
                    self.state = BuyFoodState::Empty;
                }
                Yield
            }
            BuyFoodState::BoughtAt(b) => {
                // The store can be demolished while the customer is still
                // walking to it. `routing_changed_system` force-sets
                // `cur_dest` on a dead building WITHOUT pushing
                // `GetInBuilding` (map_dynamic/router.rs), so `loc` never
                // becomes `Building(b)` and every exit below is unreachable.
                // Reset to `Empty` so the desire re-queues; `last_ate` must
                // NOT advance -- went without, like the expired-claim branch.
                //
                // The seller's reservation is deliberately not this arm's to
                // release, and it is NOT already gone: on the demolition tick
                // the seller soul is still alive, because
                // `update_decision_system` is registered before
                // `company_system` (init.rs) and `company_system` is what
                // kills a company whose building vanished. The reservation is
                // released either by `Market::remove(seller)` when that kill
                // lands, or by the unconditional retail-claim TTL sweep at the
                // end of `advance_dispatches` -- abandoning `BoughtAt` does not
                // remove the claim, so the sweep still reaches it. Re-queueing
                // before then is safe because `retail_claims` is keyed by
                // BUYER: one human holds one claim, and `make_trades` releases
                // a displaced claim's reservation on the old seller's row
                // before overwriting it.
                if !map.buildings().contains_key(b) {
                    log::debug!("{:?}'s store {:?} was demolished, going without", id, b);
                    self.state = BuyFoodState::Empty;
                    return Yield;
                }
                if loc == &Location::Building(b) {
                    // The claim can have expired (TTL) or been released
                    // (despawn raced this) during the walk over: check
                    // BEFORE deciding whether this is a real meal.
                    // `update_decision_system` runs before `market_update`
                    // each tick (see init.rs registration order), so this
                    // reads the Market as of the end of the previous tick —
                    // consistent with what `advance_dispatches`' TTL sweep
                    // will do to it later this same tick; no race within a
                    // tick since everything here is single-threaded.
                    if market.retail_claim(SoulID::Human(id)).is_some() {
                        // Settle at eat-time: seller debited, reservation
                        // freed, buyer credited nothing (the loaf is
                        // destroyed by being eaten, not added to the buyer's
                        // capital).
                        cbuf.exec_on(id, move |market: &mut Market| {
                            market.settle_retail(SoulID::Human(id));
                        });
                        self.last_ate = time.instant();
                        log::debug!("{:?} ate at {:?}", id, b);
                    } else {
                        // Claim gone: nothing to eat. Went without —
                        // `last_ate` must NOT advance (never game over).
                        log::debug!("{:?} arrived at {:?} too late, claim expired", id, b);
                    }
                    self.state = BuyFoodState::Empty;
                    Yield
                } else {
                    GoTo(Destination::Building(b))
                }
            }
        }
    }
}
