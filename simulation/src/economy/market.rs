use std::collections::btree_map::Entry;
use std::collections::BTreeMap;

use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};

use geom::{Vec2, Vec3};
use prototypes::{
    prototypes_iter, GoodsCompanyID, GoodsCompanyPrototype, ItemPrototype, Money, Tick,
    TICKS_PER_HOUR, TICKS_PER_MINUTE,
};

use crate::economy::{ItemID, WORKER_CONSUMPTION_PER_MINUTE};
use crate::map::{BuildingID, Map, PathKind};
use crate::map_dynamic::router::park;
use crate::map_dynamic::{
    BuildingInfos, DispatchID, DispatchKind, DispatchQueryTarget, Dispatcher, Itinerary,
    ParkingManagement,
};
use crate::transportation::{unpark, VehicleState};
use crate::world::VehicleEnt;
use crate::{ParCommandBuffer, SoulID, World};

#[derive(Debug, Serialize, Deserialize)]
pub struct SellOrder {
    pub pos: Vec2,
    pub qty: u32,
    /// When selling less than stock, should not enable external trading
    pub stock: u32,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct BuyOrder {
    pub pos: Vec2,
    pub qty: u32,
    /// Market passes survived since posting (sov-b70): an unmatched order
    /// ages here and only becomes border-eligible at
    /// `BORDER_ELIGIBILITY_TICKS`, so a domestic producer that gains stock
    /// the next tick wins the order instead of the border. Domestic matching
    /// ignores age entirely. Re-posting (`buy` over an existing order) keeps
    /// the age; only a genuinely new order starts at 0.
    pub age: u32,
}

#[derive(Serialize, Deserialize)]
pub struct SingleMarket {
    // todo: change i32 to Quantity
    capital: BTreeMap<SoulID, i32>,
    buy_orders: BTreeMap<SoulID, BuyOrder>,
    sell_orders: BTreeMap<SoulID, SellOrder>,
    /// Stock that has been matched to a buyer but not yet picked up by a dispatch
    /// (still physically at the seller, so not resellable, but also not yet debited
    /// from `capital` until the dispatch reaches the `Loading` state).
    reserved: BTreeMap<SoulID, u32>,
    /// The quantity a soul wants to request per cycle, which may exceed what its
    /// recipe actually consumes. Defaults to the recipe amount when unset.
    requested: BTreeMap<SoulID, u32>,
    pub ext_value: Money,
    optout_exttrade: bool,
}

impl SingleMarket {
    pub fn new(ext_value: Money, optout_exttrade: bool) -> Self {
        Self {
            capital: Default::default(),
            buy_orders: Default::default(),
            sell_orders: Default::default(),
            reserved: Default::default(),
            requested: Default::default(),
            ext_value,
            optout_exttrade,
        }
    }

    pub fn capital(&self, soul: SoulID) -> Option<i32> {
        self.capital.get(&soul).copied()
    }
    pub fn buy_order(&self, soul: SoulID) -> Option<&BuyOrder> {
        self.buy_orders.get(&soul)
    }
    pub fn sell_order(&self, soul: SoulID) -> Option<&SellOrder> {
        self.sell_orders.get(&soul)
    }
    pub fn requested(&self, soul: SoulID) -> Option<u32> {
        self.requested.get(&soul).copied()
    }
    pub fn reserved(&self, soul: SoulID) -> u32 {
        self.reserved.get(&soul).copied().unwrap_or(0)
    }

    pub fn capital_map(&self) -> &BTreeMap<SoulID, i32> {
        &self.capital
    }
}

/// Market handles good exchanging between souls themselves and the external market.
/// When goods are exchanges between souls, money is not involved.
/// When goods are exchanged with the external market, money is involved.
#[derive(Serialize, Deserialize)]
pub struct Market {
    markets: BTreeMap<ItemID, SingleMarket>,
    /// Goods that have been matched to a buyer but haven't finished their
    /// travel-to-source/loading/travel-to-destination/unloading cycle yet.
    dispatches: Vec<Dispatch>,
    /// Store→consumer purchases matched but not yet collected (see
    /// `RetailClaim`). Keyed by buyer since a human only ever has one
    /// outstanding retail purchase at a time (`buyfood` never issues a
    /// second buy order before the first resolves).
    retail_claims: BTreeMap<SoulID, RetailClaim>,
    /// Named honest-loss sink (sov-bub): one row per deleted dispatch
    /// (`LostEntry`), written by every deletion site alongside its warning.
    lost: Vec<LostEntry>,
    /// Border money owed back to the treasury (sov-5ut): `advance_dispatches`
    /// returns each tick's settlement directly, but `Market::remove` runs
    /// outside it (from `sim_drop`) with no `Government` access, so a remove
    /// path that unwinds an already-settled border leg parks the reversal
    /// here instead. Drained by `market_update` via `take_refunds` on the
    /// next pass — a one-tick delay, never a loss.
    refunds_due: Money,
    // reuse the trade vec to avoid allocations
    #[serde(skip)]
    all_trades: Vec<Trade>,
    // reuse the potential vec to avoid allocations
    #[serde(skip)]
    potential: Vec<(Trade, f32)>,
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Copy, Clone, Debug, Serialize, Deserialize)]
pub struct TradeTarget(pub SoulID);

debug_inspect_impl!(TradeTarget);

#[derive(Inspect, Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Trade {
    pub buyer: TradeTarget,
    pub seller: TradeTarget,
    pub qty: i32,
    pub kind: ItemID,
    pub money_delta: Money, // money delta from the govt point of view, positive means we gained money
}

pub fn find_trade_place(target: TradeTarget, binfos: &BuildingInfos) -> Option<BuildingID> {
    binfos.building_owned_by(target.0)
}

/// How many ticks a dispatch dwells at the source (`Loading`) and again at the
/// destination (`Unloading`) once the truck has physically arrived. Travel time
/// itself is no longer a fixed proxy: it's however long the truck actually takes
/// to drive there.
const DISPATCH_DWELL_TICKS: u32 = 3;
/// How many consecutive tick-retries a `Loading` dispatch gets to find a
/// route out — either onward to a live buyer (`DispatchState::ToDestination`)
/// or back to the seller after the buyer was demolished
/// (`DispatchState::Returning`) — before giving up and treating the goods as
/// lost. A severed road can make the route search fail forever; without a
/// bound this reintroduces the exact wedge shape this ticket exists to close.
///
/// 300 ticks at 50 ticks/s is 6 seconds of wall clock at 1x (one
/// game-minute). That is deliberately generous (sov-13h): the outbound
/// trigger is usually TRANSIENT — `Itinerary::route` fails while the player
/// drags a road, and the old bound of 20 ticks (0.4 s) sat inside a single
/// road-drag, silently deleting city-wide loads. The bound still exists for
/// the PERMANENT trigger (a demolished building), just later.
///
/// Both legs share `Dispatch::return_route_retries`: a dispatch only ever
/// leaves `Loading` by one of them, and a budget already spent failing to
/// reach the buyer is not worth re-granting to reach the seller.
const MAX_RETURN_ROUTE_RETRIES: u32 = 300;

/// Market passes an unmatched non-human buy order must survive before the
/// border may serve it (sov-b70). Domestic matching runs first every pass
/// and ignores age, so with this at 1 an enterprise queues one pass on a
/// domestic producer that gains stock the next tick instead of going abroad
/// by default. The border stays the RESIDUAL supplier, not the default one.
const BORDER_ELIGIBILITY_TICKS: u32 = 1;

/// Border throughput per freight station per market pass, in units (sov-eix).
/// The border is a HARD external constraint (Kornai): once a station has
/// moved this much in one pass, further border matches wait for the next
/// pass — they QUEUE, they are never refused and never priced out (clearing
/// by price is forbidden). Domestic matching is uncapped; only the two
/// ext-trade legs draw from this budget, imports and exports together.
const MAX_BORDER_THROUGHPUT_PER_TICK: u32 = 100;

/// How many consecutive ticks a `ToSource` dispatch may sit with NO truck
/// assigned before it is given up on and the buyer's demand handed back to the
/// market. Unlike `MAX_RETURN_ROUTE_RETRIES` this bounds a condition that is
/// perfectly ordinary — every truck in the city being busy — so it is
/// deliberately generous (one game-minute); the point is only that the wait
/// can never be infinite.
///
/// It has to exist because `ToSource` is otherwise a terminal state for the
/// ENTERPRISE, not just for the dispatch (sov-ahw). `make_trades` removes the
/// buy order at match time, and `Market::buy_until` is only ever called again
/// from `souls::goods_company::recipe_act`, which needs
/// `recipe_should_produce` — which needs the capital the undelivered import
/// was going to provide. So a dispatch that waits forever leaves its buyer
/// with no standing order, no capital and no way to ever ask again, even after
/// the player lays the road that would have fixed it. That is a "never game
/// over" violation, which is why the timeout below re-posts the buy order
/// rather than only cleaning the dispatch up.
///
/// No money moves in the rollback: since sov-7f7 (ADR-0003 §1) the border
/// commitment settles at delivery (the `Loading`/`ToDestination` arrivals),
/// never at match, so a dispatch that never left `ToSource` settled nothing
/// and refunds nothing.
///
/// Reached in an ordinary city: `market_update`'s `find_external` offers any
/// freight station whose door is within `DISPATCH_LANE_CUTOFF` of a driving
/// lane, while `DispatchOne::query` runs a real backward BFS over the lane
/// graph, so a station on a road island of its own passes the first test and
/// fails the second every tick.
const MAX_SOURCE_WAIT_TICKS: u32 = TICKS_PER_MINUTE as u32;

/// How long a human's retail claim (a matched-but-uncollected store purchase,
/// e.g. bread) waits before it's released. Retail has no dispatch/truck to
/// time out on the road, so this is the only backstop: without it, a human
/// that dies or gets stuck mid-journey would freeze the seller's reservation
/// forever, same failure shape as the dispatch wedges this ticket fixes.
const RETAIL_CLAIM_TTL_TICKS: u32 = TICKS_PER_HOUR as u32;

/// A store→consumer purchase matched but not yet collected. Unlike a
/// `Dispatch`, this never moves a truck: the buyer's own walk to the seller's
/// building *is* the physical movement (see `souls::desire::buyfood`). The
/// seller's stock stays reserved (see `SingleMarket::reserved`) until the
/// buyer physically arrives and eats, despawns, or the claim times out.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct RetailClaim {
    pub seller: SoulID,
    pub kind: ItemID,
    pub qty: u32,
    ticks_left: u32,
}

/// Named honest-loss sink (glossary: Lost, ADR-0003 §4, sov-bub).
///
/// Every dispatch deletion site records the destroyed goods here (item +
/// qty) in addition to its `log::warn`. Purely observational: recording
/// never changes deletion outcomes, bounds, or retry counts — the goods
/// stay gone, the Planner just finally sees where they went.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct LostEntry {
    pub kind: ItemID,
    pub qty: u32,
}

/// The stage of a goods dispatch. A trade match doesn't move stock immediately:
/// the quantity is only debited from the seller when the truck arrives and
/// enters `Loading`, and only credited to the buyer when it arrives and enters
/// `Unloading`. Between those two transitions, the quantity is held by the
/// dispatch itself and counted in neither soul's capital.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DispatchState {
    ToSource,
    Loading,
    ToDestination,
    Unloading,
    /// The buyer's building vanished (demolished) after the truck was
    /// already loaded: the goods are physically on the truck (already
    /// debited from the seller, never credited to the buyer), so they get
    /// physically driven back and re-credited on arrival rather than
    /// teleport-refunded. See sov-dispatch-wedge-ab4.
    Returning,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Dispatch {
    pub buyer: SoulID,
    pub seller: SoulID,
    pub kind: ItemID,
    pub qty: u32,
    /// The border commitment's money movement (ADR-0003 §1, sov-7f7): ZERO
    /// for domestic/retail/job legs by construction, nonzero only for border
    /// legs. It settles at delivery — on the `Loading` arrival when the
    /// seller is the border (import), on the `ToDestination` arrival when
    /// the buyer is (export) — never at match.
    pub money_delta: Money,
    pub state: DispatchState,
    ticks_left: u32,
    /// The truck carrying this dispatch. `None` while waiting for the
    /// `Dispatcher` to find one available.
    truck: Option<crate::world::VehicleID>,
    /// Failed `Itinerary::route` attempts made from `DispatchState::Loading`.
    /// Despite the name it counts BOTH exits from `Loading` — onward to a live
    /// buyer, and back to the seller after the buyer's building was demolished
    /// (see `MAX_RETURN_ROUTE_RETRIES`, which explains why one shared budget).
    /// A severed road can make either fail forever, so it's bounded rather
    /// than retried indefinitely — that would just reintroduce the wedge shape.
    return_route_retries: u32,
    /// Ticks spent in `ToSource` with no truck assigned, bounded by
    /// `MAX_SOURCE_WAIT_TICKS`. Never reset: a dispatch that keeps being
    /// offered a truck and keeps losing it again (the deferred-`unpark`
    /// refusal path, `release_tosource_truck`) is just as stuck as one that is
    /// never offered anything, and must be bounded too.
    source_wait_ticks: u32,
    /// The seller's `SellOrder` shape (`pos`, `stock`) as it stood just before
    /// `make_trades` consumed part of it for this dispatch. `None` for an
    /// ext-trade import, whose border seller posts no sell order at all.
    /// Kept so the `MAX_SOURCE_WAIT_TICKS` rollback can put the offer back on
    /// the market: the match removes the order once `qty` reaches 0, and
    /// releasing the reservation without restoring the offer leaves the stock
    /// physically present but invisible, so the re-posted buy order can never
    /// be served by the very seller that was about to serve it.
    sell_order: Option<(Vec2, u32)>,
}

impl Dispatch {
    /// The truck currently reserved for this dispatch, if one has been
    /// assigned yet.
    pub fn truck(&self) -> Option<crate::world::VehicleID> {
        self.truck
    }
}

impl Default for Market {
    fn default() -> Self {
        let prices = calculate_prices(1.25);
        Self {
            markets: prototypes_iter::<ItemPrototype>()
                .map(|v| (v.id, SingleMarket::new(prices[&v.id], v.optout_exttrade)))
                .collect(),
            dispatches: Default::default(),
            retail_claims: Default::default(),
            lost: Default::default(),
            refunds_due: Money::ZERO,
            all_trades: Default::default(),
            potential: Default::default(),
        }
    }
}

impl Market {
    pub fn m(&mut self, kind: ItemID) -> &mut SingleMarket {
        self.markets.get_mut(&kind).unwrap()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&ItemID, &SingleMarket)> {
        self.markets.iter()
    }

    /// Called when an agent tells the world it wants to sell something
    /// If an order is already placed, it will be updated.
    /// Beware that you need capital to sell anything, using produce.
    pub fn sell(&mut self, soul: SoulID, near: Vec2, kind: ItemID, qty: u32, stock: u32) {
        log::debug!("{:?} sell {:?} {:?} near {:?}", soul, qty, kind, near);
        self.m(kind).sell_orders.insert(
            soul,
            SellOrder {
                pos: near,
                qty,
                stock,
            },
        );
    }

    pub fn sell_all(&mut self, soul: SoulID, near: Vec2, kind: ItemID, stock: u32) {
        let c = self.capital(soul, kind);
        if c <= 0 {
            return;
        }
        self.sell(soul, near, kind, c as u32, stock);
    }
}

/// Everything a dispatch exit needs that is not the market itself (sov-5ut).
/// `parking` is `None` on the `Market::remove` path, which runs from
/// `sim_drop` without `ParkingManagement` access: those exits free the truck
/// bare (as before) instead of parking it first.
struct ExitCtx<'a> {
    map: &'a Map,
    binfos: &'a BuildingInfos,
    world: &'a mut World,
    dispatcher: &'a mut Dispatcher,
    parking: Option<&'a mut ParkingManagement>,
    tick: Tick,
}

/// How one return-to-seller attempt ends (sov-otw).
enum ReturnOutcome {
    /// The truck is routed to the seller and the dispatch is `Returning`.
    Returning,
    /// The seller's door is gone: nothing left to return the goods to.
    SellerGone,
    /// The seller stands but no route reaches it right now.
    NoRouteBack,
}

impl Market {

    /// One shared return-to-seller attempt (sov-otw), used by the dead-buyer
    /// arm of `Market::remove` and the Loading buyer-gone branch of
    /// `advance_dispatches` alike: resolve the seller's door, route the
    /// truck there from wherever it is, and transition to `Returning` on
    /// success. Failure is reported, never acted on: the ADVANCE path counts
    /// it against `MAX_RETURN_ROUTE_RETRIES` (bounded retries, then honest
    /// loss), while the REMOVE path gets exactly one attempt (it never had
    /// retries). That asymmetry is deliberate — do not unify it.
    fn try_return_to_seller(
        &mut self,
        index: usize,
        map: &Map,
        binfos: &BuildingInfos,
        world: &mut World,
        tick: Tick,
    ) -> ReturnOutcome {
        let d = self.dispatches[index];
        let Some(seller_pos) = door_pos(d.seller, map, binfos) else {
            return ReturnOutcome::SellerGone;
        };
        let start = d
            .truck
            .and_then(|v| world.vehicles.get(v))
            .map(|ve| ve.trans.pos);
        let route = start.and_then(|start| {
            Itinerary::route(tick, start, seller_pos, map, PathKind::Vehicle)
        });
        let Some(route) = route else {
            return ReturnOutcome::NoRouteBack;
        };
        if let Some(v) = d.truck {
            if let Some(ve) = world.vehicles.get_mut(v) {
                ve.it = route;
            }
        }
        self.dispatches[index].state = DispatchState::Returning;
        ReturnOutcome::Returning
    }

    /// One shared dispatch exit (sov-5ut): every dispatch termination that
    /// does NOT deliver runs through here — the sov-ahw `ToSource` timeout
    /// block is the model. Three duties, then the dispatch is gone:
    ///
    /// 1. Re-post the buyer's demand via `buy_until` (unless `repost_buyer`
    ///    is false: the buyer is gone, or it already got the goods). Skipped
    ///    automatically when the buyer's door is gone. The dispatch is
    ///    removed BEFORE re-posting so `inbound_to` (sov-q5p) never counts
    ///    the teardown dispatch itself and computes the full shortfall.
    /// 2. Refund an already-settled border leg into `refunds_due`
    ///    (ADR-0003 §5: border legs dead between match and delivery). That
    ///    is an import past `ToSource` — settlement happens on the `Loading`
    ///    arrival — and nothing else: `ToSource` exits settled nothing, and
    ///    domestic legs carry a ZERO delta by construction (never "fix" it).
    /// 3. Restore the seller's sell order from the pre-match shape (unless
    ///    `restore_seller` is false: the goods are lost, or the seller is
    ///    gone). Only `ToSource` exits and live-seller rollbacks qualify —
    ///    restoring an offer for goods already debited-and-destroyed would
    ///    conjure stock from nothing.
    ///
    /// `record_loss` writes the `Lost` row for goods debited but neither
    /// delivered nor returned. The truck is parked-then-freed (sov-2c4
    /// convention, sov-91e) wherever `parking` is available.
    ///
    /// Removing is `swap_remove`, so a second exit of the same dispatch is a
    /// no-op by construction: there is nothing left at that shape to exit.
    fn terminate_dispatch(
        &mut self,
        index: usize,
        ctx: &mut ExitCtx,
        repost_buyer: bool,
        restore_seller: bool,
        record_loss: bool,
        reason: &str,
    ) {
        let d = self.dispatches[index];
        // The match is undone first: everything below (notably the
        // `buy_until` re-post) must observe a market WITHOUT this dispatch.
        self.dispatches.swap_remove(index);
        if let Some(v) = d.truck {
            if let Some(parking) = ctx.parking.as_deref_mut() {
                if let Some(pos) = ctx.world.vehicles.get(v).map(|ve| ve.trans.pos) {
                    if let Ok(spot) = parking.reserve_near(pos, ctx.map) {
                        if let Some(ve) = ctx.world.vehicles.get_mut(v) {
                            park(ctx.map, ve, spot);
                        }
                    }
                }
            }
            ctx.dispatcher.free(DispatchID::SmallTruck(v));
        }
        // The goods never physically left (or the seller row is gone with
        // the seller): freeing the reservation is saturating, so releasing
        // a row that was never reserved — an ext-trade import — is a no-op.
        if let Some(r) = self.m(d.kind).reserved.get_mut(&d.seller) {
            *r = r.saturating_sub(d.qty);
        }
        if restore_seller {
            if let Some((pos, stock)) = d.sell_order {
                // The sov-ahw shape: the match deletes the order outright at
                // 0, so put the offer back — clamped to the seller's capital
                // because `sell_all` may have re-posted off the full
                // (still-undebited) capital in the meantime.
                let m = self.m(d.kind);
                let cap = m.capital(d.seller).unwrap_or(0).max(0) as u32;
                match m.sell_orders.entry(d.seller) {
                    Entry::Occupied(mut o) => {
                        let order = o.get_mut();
                        order.qty = (order.qty + d.qty).min(cap);
                    }
                    Entry::Vacant(v) => {
                        v.insert(SellOrder {
                            pos,
                            qty: d.qty.min(cap),
                            stock,
                        });
                    }
                }
            }
        }
        if repost_buyer && !matches!(d.buyer, SoulID::FreightStation(_)) {
            // A freight-station buyer never consumes (it holds Border
            // custody, a separate ledger): re-posting its "demand" would let
            // it import from itself next pass. Export timeouts therefore
            // restore the seller's offer only.
            if let Some(buyer_pos) = door_pos(d.buyer, ctx.map, ctx.binfos) {
                let want = self.requested(d.buyer, d.kind).unwrap_or(d.qty).max(d.qty);
                self.buy_until(d.buyer, buyer_pos.xy(), d.kind, want);
            }
        }
        if matches!(d.seller, SoulID::FreightStation(_)) && d.state != DispatchState::ToSource {
            // The import leg settled on the `Loading` arrival and the goods
            // never reached the buyer: reverse exactly that settlement.
            self.refunds_due -= d.money_delta;
        }
        if record_loss {
            self.record_lost(d.kind, d.qty);
        }
        log::warn!(
            "dispatch {:?} {:?} ({:?} -> {:?}) terminated in {:?}: {}",
            d.qty,
            d.kind,
            d.seller,
            d.buyer,
            d.state,
            reason
        );
    }
}
impl Market {


    /// An agent was removed from the world, we need to clean after him
    /// `map`/`binfos`/`world`/`dispatcher`/`tick` are only needed to hand a
    /// dead buyer's in-flight goods to the physical return-to-seller path
    /// (see below) -- without them a demolished buyer would either strand
    /// the seller's reservation forever or destroy the shipment outright.
    pub fn remove(
        &mut self,
        soul: SoulID,
        map: &Map,
        binfos: &BuildingInfos,
        world: &mut World,
        dispatcher: &mut Dispatcher,
        tick: Tick,
    ) {
        // A live retail claim reserves stock on the SELLER's row, not the
        // buyer's, so it must be released before that row is touched below.
        if let Some(claim) = self.retail_claims.remove(&soul) {
            if let Some(m) = self.markets.get_mut(&claim.kind) {
                if let Some(r) = m.reserved.get_mut(&claim.seller) {
                    *r = r.saturating_sub(claim.qty);
                }
            }
        }
        // If the soul being removed is itself a seller holding outstanding
        // claims from (still-alive) buyers, those claims now point at a
        // capital/reserved row that's about to be wiped; drop them too so
        // eat-time/TTL settlement doesn't resurrect a phantom seller row.
        self.retail_claims.retain(|_, c| c.seller != soul);

        for market in self.markets.values_mut() {
            market.sell_orders.remove(&soul);
            market.buy_orders.remove(&soul);
            market.capital.remove(&soul);
            market.reserved.remove(&soul);
            market.requested.remove(&soul);
        }

        // Any dispatch naming this soul as buyer or seller is now a dangling
        // reference. A dead SELLER's own rows were wiped above, so there is
        // nothing left to credit or reserve for it — but a LIVE buyer on the
        // other end still needs its demand back (sov-5ut: the
        // FreightStationEnt::sim_drop seller-half used to drop the buyer's
        // dispatch silently, killing the enterprise permanently).
        //
        // A dead BUYER with a surviving seller is different: the seller's
        // row is still live, so silently dropping the dispatch either
        // strands `reserved[seller]` forever (`ToSource`, nothing debited
        // yet) or destroys goods already debited from the seller with no
        // sink (`Loading`/`ToDestination`/`Returning`) — see
        // sov-dispatch-wedge-ab4. Route each such dispatch through the same
        // fate a live buyer-demolition takes in `advance_dispatches`.
        //
        // Every arm below terminates through `terminate_dispatch` (re-post
        // unless the buyer is gone, refund of settled border legs, seller
        // offer restored where the goods never left) and every return
        // attempt through `try_return_to_seller` (one attempt here, never
        // retries — the advance path's bound is its own, see sov-otw).
        let mut i = 0;
        while i < self.dispatches.len() {
            let d = &self.dispatches[i];
            if d.buyer != soul || d.seller == soul {
                i += 1;
                continue;
            }
            let state = d.state;
            match state {
                DispatchState::ToSource => {
                    // Nothing was ever debited and the buyer is gone: free
                    // the reservation and put the seller's offer back (the
                    // sov-5ut TRAP — this arm used to free without
                    // restoring), but re-post nothing for a dead buyer.
                    let mut ctx = ExitCtx {
                        map,
                        binfos,
                        world: &mut *world,
                        dispatcher: &mut *dispatcher,
                        parking: None,
                        tick,
                    };
                    self.terminate_dispatch(
                        i,
                        &mut ctx,
                        false,
                        true,
                        false,
                        "buyer removed while waiting for a truck; seller offer restored",
                    );
                    continue;
                }
                DispatchState::Loading
                | DispatchState::ToDestination
                | DispatchState::Returning => {
                    // Seller already debited (or is mid-return): drive the
                    // goods physically back instead of teleport-refunding or
                    // destroying them — one attempt, no retries.
                    match self.try_return_to_seller(i, map, binfos, &mut *world, tick) {
                        ReturnOutcome::Returning => {}
                        ReturnOutcome::NoRouteBack => {
                            // No route back to the seller: an honest
                            // physical loss (the goods are already debited
                            // from the seller and never credited to anyone).
                            // A settled border leg is refunded inside.
                            let mut ctx = ExitCtx {
                                map,
                                binfos,
                                world: &mut *world,
                                dispatcher: &mut *dispatcher,
                                parking: None,
                                tick,
                            };
                            self.terminate_dispatch(
                                i,
                                &mut ctx,
                                false,
                                false,
                                true,
                                "no route back to seller for a dead buyer's goods, treating as lost",
                            );
                            continue;
                        }
                        ReturnOutcome::SellerGone => {
                            // Seller is also gone: nothing left to return the
                            // goods to.
                            let mut ctx = ExitCtx {
                                map,
                                binfos,
                                world: &mut *world,
                                dispatcher: &mut *dispatcher,
                                parking: None,
                                tick,
                            };
                            self.terminate_dispatch(
                                i,
                                &mut ctx,
                                false,
                                false,
                                true,
                                "buyer and seller are both gone",
                            );
                            continue;
                        }
                    }
                }
                DispatchState::Unloading => {
                    // The buyer was about to be credited; it no longer
                    // exists to receive the goods, so nothing is credited.
                    // The seller was already debited when the truck loaded
                    // and does not get the goods back (mirrors Unloading's
                    // own honest-loss shape: once loaded, goods that never
                    // reach a buyer are gone, not refunded).
                    let mut ctx = ExitCtx {
                        map,
                        binfos,
                        world: &mut *world,
                        dispatcher: &mut *dispatcher,
                        parking: None,
                        tick,
                    };
                    self.terminate_dispatch(
                        i,
                        &mut ctx,
                        false,
                        false,
                        true,
                        "buyer removed while unloading",
                    );
                    continue;
                }
            }
            i += 1;
        }
        // A dead SELLER's rows were wiped above, so there is nothing left to
        // credit or reserve and the dispatch is simply dropped — but the
        // truck it holds must go back to the pool first. `Dispatcher::query`
        // skips anything still in `reserved_by` and only `free` clears it, so
        // dropping the dispatch without freeing removes that truck from the
        // city permanently (sov-dispatch-wedge-ab4 round 4). The live buyer
        // on the other end gets its demand re-posted (sov-5ut) and a settled
        // border leg refunded; the seller's offer cannot be restored — its
        // rows are gone with it. `repost` is skipped when the buyer is the
        // removed soul itself (both ends gone).
        let mut i = 0;
        while i < self.dispatches.len() {
            if self.dispatches[i].seller != soul {
                i += 1;
                continue;
            }
            let state = self.dispatches[i].state;
            let repost = self.dispatches[i].buyer != soul;
            let mut ctx = ExitCtx {
                map,
                binfos,
                world: &mut *world,
                dispatcher: &mut *dispatcher,
                parking: None,
                tick,
            };
            // Only a dispatch that already debited its seller destroys
            // goods here; a `ToSource` exit never moved anything.
            let debited = state != DispatchState::ToSource;
            self.terminate_dispatch(
                i,
                &mut ctx,
                repost,
                false,
                debited,
                "seller removed mid-flight; live buyer's demand handed back to the market",
            );
        }
    }

    /// Called when an agent tells the world it wants to buy something
    /// If an order is already placed, it will be updated (pos/qty), keeping
    /// its age so `recipe_act`'s every-cycle `buy_until` does not reset the
    /// sov-b70 border-eligibility clock on standing demand.
    pub fn buy(&mut self, soul: SoulID, near: Vec2, kind: ItemID, qty: u32) {
        log::debug!("{:?} buy {:?} {:?} near {:?}", soul, qty, kind, near);

        self.m(kind)
            .buy_orders
            .entry(soul)
            .and_modify(|o| {
                o.pos = near;
                o.qty = qty;
            })
            .or_insert(BuyOrder { pos: near, qty, age: 0 });
    }

    pub fn buy_until(&mut self, soul: SoulID, near: Vec2, kind: ItemID, qty: u32) {
        let c = self.capital(soul, kind);
        if c >= qty as i32 {
            return;
        }
        // Capital can go negative (FreightStation routinely does): clamp to
        // zero before the u32 cast, otherwise `c as u32` wraps to ~4.29e9
        // and `qty - huge` underflows/panics.
        let have = c.max(0) as u32;
        // In-flight inbound already covers part of the want (sov-q5p): a
        // company that re-posts while an import is en route must order only
        // its CURRENT shortfall, or every cycle stacks another full target
        // onto the border. Without this a k=4 hoarder with a reachable
        // station holds 1+2+3... in flight against a target of 4.
        let shortfall = qty
            .saturating_sub(have)
            .saturating_sub(self.inbound_to(soul, kind));
        if shortfall == 0 {
            // Covered by stock on hand plus goods already coming: a stale
            // standing order would re-match and over-supply, and a fresh
            // qty-0 order would match a zero-quantity trade, so drop it.
            self.m(kind).buy_orders.remove(&soul);
            return;
        }
        self.buy(soul, near, kind, shortfall);
    }

    /// Quantity of `kind` already inbound to `soul`: matched into a dispatch
    /// that has not delivered yet (`ToSource`/`Loading`/`ToDestination`).
    /// `Returning` drives goods back to the SELLER, and `Unloading` already
    /// credited the buyer, so neither counts. The honest-vs-dishonest signal
    /// survives this (sov-q5p): an honest k=1 enterprise consumes to 0 and
    /// stalls with nothing inbound, while a hoarder's surplus is capital on
    /// hand, not goods in flight.
    pub fn inbound_to(&self, soul: SoulID, kind: ItemID) -> u32 {
        self.dispatches
            .iter()
            .filter(|d| {
                d.buyer == soul
                    && d.kind == kind
                    && matches!(
                        d.state,
                        DispatchState::ToSource
                            | DispatchState::Loading
                            | DispatchState::ToDestination
                    )
            })
            .map(|d| d.qty)
            .sum()
    }

    /// Drains the sov-5ut refund buffer (see `refunds_due`): border money a
    /// `Market::remove` exit unwound, applied by `market_update` on the next
    /// pass. Returns `Money::ZERO` when nothing is owed.
    pub fn take_refunds(&mut self) -> Money {
        std::mem::replace(&mut self.refunds_due, Money::ZERO)
    }

    /// Get the capital that this agent owns
    pub fn capital(&self, soul: SoulID, kind: ItemID) -> i32 {
        self.markets.get(&kind).unwrap().capital(soul).unwrap_or(0)
    }

    /// Stock matched to a buyer but not yet collected (dispatch in flight, or
    /// an uncollected retail claim): physically still at the seller, but not
    /// resellable and not available to produce more of.
    pub fn reserved(&self, soul: SoulID, kind: ItemID) -> u32 {
        self.markets
            .get(&kind)
            .map(|m| m.reserved(soul))
            .unwrap_or(0)
    }

    /// Set the quantity a soul wants to request per cycle for an item. May exceed
    /// (or fall under) what its recipe actually consumes.
    pub fn set_requested(&mut self, soul: SoulID, kind: ItemID, qty: u32) {
        self.m(kind).requested.insert(soul, qty);
    }

    /// The requested quantity for a soul/item, if one was set with `set_requested`.
    pub fn requested(&self, soul: SoulID, kind: ItemID) -> Option<u32> {
        self.markets.get(&kind).and_then(|m| m.requested(soul))
    }
    /// Requested quantity with the recipe amount as the fallback
    /// (STORY-0107): `requested` returns `None` when no inflation was ever
    /// declared, which is the NORMAL honest case meaning "use the recipe
    /// amount" — never zero. The panel binds to this instead of
    /// `unwrap_or(0)`, which would show every honest company requesting
    /// nothing. Pure getter; changes nothing.
    pub fn requested_or(&self, soul: SoulID, kind: ItemID, recipe_amount: u32) -> u32 {
        self.requested(soul, kind).unwrap_or(recipe_amount)
    }

    pub fn dispatches(&self) -> &[Dispatch] {
        &self.dispatches
    }
    /// Planner-visible stalled-dispatch count (sov-8lu): dispatches sitting
    /// in `ToSource` with no truck assigned. This is the escalation the
    /// ticket asks for — today such a dispatch only ever reaches
    /// `log::warn!`, which no player sees — and it is bounded, not
    /// terminal: `MAX_SOURCE_WAIT_TICKS` eventually rolls the match back
    /// onto the market. Pure getter; changes nothing.
    pub fn stalled_dispatch_count(&self) -> usize {
        self.dispatches
            .iter()
            .filter(|d| d.state == DispatchState::ToSource && d.truck.is_none())
            .count()
    }

    /// The named honest-loss sink (sov-bub): one row per dispatch deletion,
    /// in deletion order. Purely observational — nothing reads this back
    /// into the simulation.
    pub fn lost(&self) -> &[LostEntry] {
        &self.lost
    }

    /// Records destroyed goods in `Lost`. Called by every deletion site
    /// next to its `log::warn`; must not change any other state.
    fn record_lost(&mut self, kind: ItemID, qty: u32) {
        self.lost.push(LostEntry { kind, qty });
    }

    /// The buyer's outstanding retail claim (a matched-but-uncollected store
    /// purchase), if any.
    pub fn retail_claim(&self, buyer: SoulID) -> Option<&RetailClaim> {
        self.retail_claims.get(&buyer)
    }

    /// Settles a retail claim at consumption time: the seller is debited and
    /// its reservation released, the buyer is credited NOTHING (the good is
    /// destroyed by being eaten, not transferred into the buyer's capital —
    /// see `souls::desire::buyfood`'s `BoughtAt` arm). No money moves; the
    /// domestic match already settled `money_delta: Money::ZERO`. Returns
    /// whether a claim was found and settled.
    pub fn settle_retail(&mut self, buyer: SoulID) -> bool {
        let Some(claim) = self.retail_claims.remove(&buyer) else {
            return false;
        };
        let m = self.m(claim.kind);
        *m.capital.entry(claim.seller).or_default() -= claim.qty as i32;
        if let Some(r) = m.reserved.get_mut(&claim.seller) {
            *r = r.saturating_sub(claim.qty);
        }
        true
    }

    /// Registers a soul to the market, not obligatory
    pub fn register(&mut self, soul: SoulID, kind: ItemID) {
        self.m(kind).capital.entry(soul).or_default();
    }

    /// Called whenever an agent (like a farm) produces something on it's own
    /// for example wheat is harvested or turned into flour. Returns the new quantity owned.
    pub fn produce(&mut self, soul: SoulID, kind: ItemID, delta: i32) -> i32 {
        log::debug!("{:?} produced {:?} {:?}", soul, delta, kind);

        let v = self.m(kind).capital.entry(soul).or_default();
        *v += delta;
        *v
    }

    /// Returns a list of buy and sell orders matched together.
    /// A trade updates the buy and sell orders from the market, and the capital of the buyers and sellers.
    /// A trade can only be completed if the seller has enough capital.
    /// Please do not keep the trades around much, it needs to be destroyed by the next time you call this function.
    pub fn make_trades(&mut self, find_external: impl Fn(Vec2) -> Option<SoulID>) -> &[Trade] {
        self.make_trades_split(&find_external, &find_external)
    }

    /// Same match as `make_trades`, with separate border lookups per
    /// direction (sov-nun): imports need a driving lane (a truck must run
    /// from the station door), so `find_import` keeps the
    /// `DISPATCH_LANE_CUTOFF` reachability filter; exports ride a dispatch
    /// the seller's own truck drives to the border door and need no lane at
    /// the station end, so `find_export` is the unfiltered nearest-station
    /// lookup. Gating exports on a lane they never use left a new city
    /// unable to export at all until the player laid road near the station.
    pub fn make_trades_split(
        &mut self,
        find_import: impl Fn(Vec2) -> Option<SoulID>,
        find_export: impl Fn(Vec2) -> Option<SoulID>,
    ) -> &[Trade] {
        self.all_trades.clear();
        // Border throughput drawn per station by the two ext-trade legs
        // below, in units (sov-eix). Reset every pass: this is a per-tick
        // capacity, not a budget that accumulates.
        let mut border_used: BTreeMap<SoulID, u32> = BTreeMap::new();
        // sov-64b: displaced retail claims whose kind differs from the loop's
        // kind cannot release inside the loop (the loop holds that market's
        // borrow); they queue here and release against their OWN kind's
        // market after the pass.
        let mut retail_releases: Vec<RetailClaim> = Vec::new();
        for (&kind, market) in &mut self.markets {
            // Naive O(n²) alg
            // We don't immediatly apply the trades, because we want to find the nearest-positioned trades
            for (&seller, sorder) in &market.sell_orders {
                let qty_sell = sorder.qty as i32;

                let capital_sell = unwrap_or!(market.capital(seller), continue);
                if qty_sell > capital_sell {
                    continue;
                }
                for (&buyer, &border) in &market.buy_orders {
                    if seller == buyer {
                        log::warn!(
                            "{:?} is both selling and buying same commodity: {:?}",
                            seller,
                            kind
                        );
                        continue;
                    }
                    let qty_buy = border.qty as i32;
                    if qty_buy > qty_sell {
                        continue;
                    }
                    let score = sorder.pos.distance2(border.pos);
                    self.potential.push((
                        Trade {
                            buyer: TradeTarget(buyer),
                            seller: TradeTarget(seller),
                            qty: qty_buy,
                            kind,
                            money_delta: Money::ZERO,
                        },
                        score,
                    ))
                }
            }
            self.potential
                .sort_unstable_by_key(|(_, x)| OrderedFloat(*x));
            let SingleMarket {
                buy_orders,
                sell_orders,
                capital,
                reserved,
                optout_exttrade,
                ext_value,
                ..
            } = market;

            // The shape (`pos`, `stock`) of each seller's `SellOrder` as it
            // was BEFORE this pass consumed part of it. A match decrements
            // `sorder.qty` and removes the order outright when that hits 0,
            // so a dispatch that later gives up in `ToSource` has nothing to
            // rebuild the offer from -- and the goods, which never moved,
            // would sit at the seller unadvertised forever. Captured here
            // because this is the only place the pre-match order is in hand.
            let mut sold_from: BTreeMap<SoulID, (Vec2, u32)> = BTreeMap::new();

            let dispatch_start = self.all_trades.len();

            self.all_trades
                .extend(self.potential.drain(..).filter_map(|(trade, _)| {
                    let cap_seller = *capital.entry(trade.seller.0).or_default();
                    let already_reserved =
                        reserved.get(&trade.seller.0).copied().unwrap_or(0) as i32;
                    if cap_seller - already_reserved < trade.qty {
                        return None;
                    }

                    let border = buy_orders.entry(trade.buyer.0);

                    match border {
                        Entry::Vacant(_) => return None,
                        Entry::Occupied(o) => o.remove(),
                    };

                    let sorderent = sell_orders.entry(trade.seller.0);

                    let mut sorderocc = match sorderent {
                        Entry::Vacant(_) => return None,
                        Entry::Occupied(o) => o,
                    };

                    let sorder = sorderocc.get_mut();

                    if sorder.qty < trade.qty as u32 {
                        return None;
                    }

                    sold_from.insert(trade.seller.0, (sorder.pos, sorder.stock));

                    sorder.qty -= trade.qty as u32;

                    if sorder.qty == 0 {
                        sorderocc.remove();
                    }

                    if kind == ItemID::new("job-opening") {
                        // A "job-opening" match never gets a dispatch (see
                        // below), so nothing would ever debit capital or
                        // release a reservation for it; settle immediately
                        // instead of reserving.
                        *capital.entry(trade.seller.0).or_default() -= trade.qty;
                    } else {
                        // The goods stay physically at the seller (neither capital bucket
                        // moves yet) until a dispatch actually loads/unloads them; only
                        // reserve them so they can't be sold again.
                        *reserved.entry(trade.seller.0).or_default() += trade.qty as u32;
                    }

                    Some(trade)
                }));

            // External trading, buy side. Pushed BEFORE the dispatch loop
            // below, so an import lands in `all_trades[dispatch_start..]`
            // and gets a `Dispatch` out of the freight station like any
            // other physical delivery (sov-abs). It used to be pushed after
            // that loop, with `capital[buyer]` credited right here: goods
            // appeared in the buyer's larder in the same tick, having moved
            // nowhere, and no enterprise in a city with a freight station
            // could ever experience an unmet input need.
            if !*optout_exttrade {
                // Humans never clear through the external market: retail
                // clears by queue and going-without only (never by money —
                // an ext-trade buy attaches a money_delta, forbidden on the
                // human path, see `RetailClaim`). Their unmatched buy orders
                // must survive this pass untouched so they're still there
                // for next tick's domestic match, not silently dropped.
                let btaken: BTreeMap<_, _> = buy_orders
                    .extract_if(.., |s, o| {
                        // Humans never clear through the external market (see
                        // below), and young orders queue one pass for a
                        // domestic seller first (sov-b70): only non-human
                        // orders old enough to have survived a domestic pass
                        // may go abroad. Everything left behind stays for
                        // next tick's domestic match, not silently dropped.
                        !matches!(s, SoulID::Human(_)) && o.age >= BORDER_ELIGIBILITY_TICKS
                    })
                    .collect();
                // All remaining (non-human) buyers can fulfil since they can buy externally
                self.all_trades.reserve(btaken.len());
                for (buyer, order) in btaken {
                    let Some(ext) = find_import(order.pos) else {
                        // The border cannot serve this buyer (no freight
                        // station in range, or none at all). That is not a
                        // reason to DESTROY its demand: `extract_if` above
                        // already took the order out of `buy_orders`, and
                        // dropping it here is what killed enterprises in a
                        // city with a closed border -- `recipe_init`'s very
                        // first order is eaten exactly this way, and so was
                        // every order the `MAX_SOURCE_WAIT_TICKS` rollback in
                        // `advance_dispatches` re-posts. Put it back so the
                        // next domestic match can still see it.
                        buy_orders.insert(buyer, order);
                        continue;
                    };

                    // Border throughput (sov-eix): a station moves at most
                    // `MAX_BORDER_THROUGHPUT_PER_TICK` units per pass. The
                    // excess is served partially and the remainder re-queued
                    // — never refused, never priced.
                    let used = border_used.entry(ext).or_insert(0);
                    let avail = MAX_BORDER_THROUGHPUT_PER_TICK.saturating_sub(*used);
                    if avail == 0 {
                        buy_orders.insert(buyer, order);
                        continue;
                    }
                    let fill = order.qty.min(avail);
                    *used += fill;
                    if fill < order.qty {
                        buy_orders.insert(
                            buyer,
                            BuyOrder {
                                pos: order.pos,
                                qty: order.qty - fill,
                                age: order.age,
                            },
                        );
                    }
                    let qty_buy = fill as i32;

                    // No capital moves here. The border is debited when the
                    // truck loads at the freight station and the buyer is
                    // credited when it unloads at their door, exactly like a
                    // domestic trade (`Market::advance_dispatches`).
                    self.all_trades.push(Trade {
                        buyer: TradeTarget(buyer),
                        seller: TradeTarget(ext),
                        qty: qty_buy,
                        kind,
                        money_delta: -(*ext_value * qty_buy as i64), // we buy from external so we pay
                    });
                }
            }

            // External trading, sell side. Pushed BEFORE the dispatch loop
            // below, so an export lands in `all_trades[dispatch_start..]`
            // and gets a `Dispatch` to the border door like any other
            // physical delivery (sov-20g). It used to be pushed after
            // that loop, with `capital[seller]` debited right here: goods
            // left the seller's building and reached the border in the
            // same tick, having moved nowhere.
            if !*optout_exttrade {
                // Seller surplus goes to external trading. Stock already
                // reserved for a domestic buyer (matched but not yet picked
                // up by a dispatch) isn't free to export: `sell_all` re-posts
                // the order off the seller's full capital and forgets any
                // outstanding reservation, so the surplus here must be
                // computed against what's actually unreserved.
                for (&seller, order) in sell_orders.iter_mut() {
                    let already_reserved = reserved.get(&seller).copied().unwrap_or(0);
                    let free_qty = order.qty.saturating_sub(already_reserved);
                    let qty_sell = free_qty as i32 - order.stock as i32;
                    if qty_sell <= 0 {
                        continue;
                    }
                    let cap = capital.entry(seller).or_default();
                    if *cap - (already_reserved as i32) < qty_sell {
                        log::warn!("{:?} is selling more than it has: {:?}", &seller, qty_sell);
                        continue;
                    }

                    // The export lookup is lane-UNFILTERED (sov-nun): an
                    // export creates no Dispatch at the station end (the
                    // seller's truck drives to the border door), so a lane
                    // the import leg needs must not gate it.
                    let Some(ext) = find_export(order.pos) else {
                        continue;
                    };

                    // Border throughput (sov-eix), same budget as the import
                    // leg above: partial fill, remainder stays offered.
                    let used = border_used.entry(ext).or_insert(0);
                    let avail = MAX_BORDER_THROUGHPUT_PER_TICK.saturating_sub(*used);
                    if avail == 0 {
                        continue;
                    }
                    let fill = (qty_sell as u32).min(avail);
                    *used += fill;
                    let qty_sell = fill as i32;

                    // Remember the pre-match offer shape for the exit helper:
                    // unlike the domestic match (which records it above), the
                    // export loop used to leave an export-only seller with no
                    // `sell_order`, so a `ToSource` timeout released the
                    // reservation but never put the offer back (sov-5ut).
                    sold_from.entry(seller).or_insert((order.pos, order.stock));

                    // No capital moves here. The seller is debited when the
                    // truck loads at their door and the border is credited
                    // when it unloads at the freight station, exactly like
                    // a domestic trade (`Market::advance_dispatches`).
                    *reserved.entry(seller).or_default() += qty_sell as u32;
                    order.qty -= qty_sell as u32;

                    self.all_trades.push(Trade {
                        buyer: TradeTarget(ext),
                        seller: TradeTarget(seller),
                        qty: qty_sell,
                        kind,
                        money_delta: *ext_value * qty_sell as i64,
                    });
                }
            }

            // Labor isn't cargo: hiring already happens synchronously off
            // `trades` in `economy::market_update` the moment a match is
            // made, so a "job-opening" match never needs a truck to
            // physically deliver it.
            if kind != ItemID::new("job-opening") {
                for &trade in &self.all_trades[dispatch_start..] {
                    if let SoulID::Human(_) = trade.buyer.0 {
                        // Retail leg (store -> consumer): the human's own walk
                        // to the seller's building is the physical movement,
                        // so no truck/Dispatch is created; settlement happens
                        // at eat-time (`Market::settle_retail`). A buyer can
                        // already hold a live claim here (see gate defect 1),
                        // so release its reservation before overwriting it.
                        if let Some(old) = self.retail_claims.insert(
                            trade.buyer.0,
                            RetailClaim {
                                seller: trade.seller.0,
                                kind,
                                qty: trade.qty as u32,
                                ticks_left: RETAIL_CLAIM_TTL_TICKS,
                            },
                        ) {
                            log::warn!(
                                "buyer {:?} matched a new retail claim while one was still \
                                 outstanding; releasing the orphaned reservation",
                                trade.buyer.0
                            );
                            if old.kind == kind {
                                // Same-kind overwrite (the only shape `buyfood`
                                // produces today: it hardcodes bread): the
                                // displaced reservation lives in this loop's
                                // `reserved` map.
                                if let Some(r) = reserved.get_mut(&old.seller) {
                                    *r = r.saturating_sub(old.qty);
                                }
                            } else {
                                // sov-64b: cross-kind overwrite. The displaced
                                // claim's reservation lives in its OWN kind's
                                // market, never in this loop's `reserved` map
                                // (releasing here would unreserve the wrong
                                // seller's stock and freeze the right one).
                                retail_releases.push(old);
                            }
                        }
                        continue;
                    }
                    self.dispatches.push(Dispatch {
                        buyer: trade.buyer.0,
                        seller: trade.seller.0,
                        kind,
                        qty: trade.qty as u32,
                        // The commitment's settlement travels with the goods
                        // (sov-7f7): the arrival hooks below apply it.
                        money_delta: trade.money_delta,
                        state: DispatchState::ToSource,
                        ticks_left: 0,
                        truck: None,
                        source_wait_ticks: 0,
                        sell_order: sold_from.get(&trade.seller.0).copied(),
                        return_route_retries: 0,
                    });
                }
            }
            // Age every surviving buy order (sov-b70): eligibility is read
            // off the PRE-increment age in the ext-buy block above, so a
            // fresh order queues exactly one full pass for a domestic
            // seller before the border may take it. Incremented last so the
            // pass that posted (or re-posted) an order never counts itself.
            for o in buy_orders.values_mut() {
                o.age = o.age.saturating_add(1);
            }
        }
        // sov-64b: release cross-kind displaced claims (queued above) against
        // their OWN kind's market. Never-game-over: a missing market or seller
        // row degrades to the warn above, never a panic.
        for old in retail_releases {
            if let Some(other) = self.markets.get_mut(&old.kind) {
                if let Some(r) = other.reserved.get_mut(&old.seller) {
                    *r = r.saturating_sub(old.qty);
                }
            }
        }

        &self.all_trades
    }

    pub fn inner(&self) -> &BTreeMap<ItemID, SingleMarket> {
        &self.markets
    }

    /// Rolls back the truck reservation `advance_dispatches` recorded for a
    /// `ToSource` dispatch, putting it back to "waiting for a truck" so the
    /// next tick retries. Returns whether a dispatch was actually rolled back.
    ///
    /// Needed because the truck is assigned here but `unpark` runs deferred
    /// through `ParCommandBuffer<VehicleEnt>`: if the truck stopped being
    /// `Parked` in between, `unpark` refuses (sov-6qx) and the truck never
    /// moves. `MAX_SOURCE_WAIT_TICKS` would eventually tear such a dispatch
    /// down, but tearing it down throws the match away; releasing the truck
    /// here keeps the dispatch alive to retry on the very next tick, which is
    /// the cheaper and more accurate outcome. Identified by the truck rather
    /// than by index because
    /// `dispatches` is `swap_remove`d and indices do not survive a tick; a
    /// truck is reserved by at most one dispatch at a time.
    pub(crate) fn release_tosource_truck(&mut self, v: crate::world::VehicleID) -> bool {
        for d in &mut self.dispatches {
            if d.state == DispatchState::ToSource && d.truck == Some(v) {
                d.truck = None;
                return true;
            }
        }
        false
    }

    /// Drives every in-flight dispatch one tick (see the state machine below),
    /// sequencing each ToSource -> Loading -> ToDestination -> Unloading: the
    /// seller's capital is debited exactly when the truck arrives and enters
    /// `Loading`, the buyer's credited exactly when it arrives and enters
    /// `Unloading`; must be called once per tick for dispatches to ever
    /// complete. With no truck available, a dispatch waits in `ToSource` — no
    /// capital moves — but only for `MAX_SOURCE_WAIT_TICKS`, after which the
    /// whole match is rolled back onto the market (see that arm).
    ///
    /// Returns the border money that settled on arrivals this tick (sov-7f7,
    /// ADR-0003 §1): each dispatch carries its commitment's `money_delta`
    /// (ZERO for domestic legs), applied on the `Loading` arrival for imports
    /// and the `ToDestination` arrival for exports. The caller adds it to
    /// `Government.money`.
    pub fn advance_dispatches(
        &mut self,
        world: &mut World,
        map: &Map,
        binfos: &BuildingInfos,
        dispatcher: &mut Dispatcher,
        cbuf_vehicle: &ParCommandBuffer<VehicleEnt>,
        parking: &mut ParkingManagement,
        tick: Tick,
    ) -> Money {
        let mut settled = Money::ZERO;
        let mut i = 0;
        while i < self.dispatches.len() {
            let (
                seller,
                buyer,
                kind,
                qty,
                money_delta,
                state,
                ticks_left,
                truck,
                return_route_retries,
                source_wait_ticks,
                _sell_order,
            ) = {
                let d = &self.dispatches[i];
                (
                    d.seller,
                    d.buyer,
                    d.kind,
                    d.qty,
                    d.money_delta,
                    d.state,
                    d.ticks_left,
                    d.truck,
                    d.return_route_retries,
                    d.source_wait_ticks,
                    d.sell_order,
                )
            };

            let mut remove = false;
            match state {
                DispatchState::ToSource => match truck {
                    None => {
                        if let Some(seller_pos) = door_pos(seller, map, binfos) {
                            if let Some(DispatchID::SmallTruck(v)) = dispatcher.query(
                                map,
                                DispatchKind::SmallTruck,
                                DispatchQueryTarget::Pos(seller_pos),
                            ) {
                                // `unpark` only makes sense on a truck that
                                // is actually `Parked` -- one still mid-
                                // `RoadToPark` (queued by the Unloading fix
                                // below, sov-2c4) isn't done arriving at its
                                // spot yet. Grabbing it anyway made `unpark`
                                // warn "wasn't parked" and clobber its
                                // collider without freeing the old grid
                                // entry, leaving a permanent phantom blocker
                                // (sov-7pg). Free it back and let the
                                // dispatcher offer a different truck (or the
                                // same one, once it's really parked) next
                                // tick, instead of grabbing it prematurely.
                                let parked = world.vehicles.get(v).is_some_and(|ve| {
                                    matches!(ve.vehicle.state, VehicleState::Parked(_))
                                });
                                if !parked {
                                    dispatcher.free(DispatchID::SmallTruck(v));
                                } else {
                                    let start = world.vehicles.get(v).map(|ve| ve.trans.pos);
                                    let route = start.and_then(|start| {
                                        Itinerary::route(
                                            tick,
                                            start,
                                            seller_pos,
                                            map,
                                            PathKind::Vehicle,
                                        )
                                    });
                                    if let Some(route) = route {
                                        if let Some(ve) = world.vehicles.get_mut(v) {
                                            ve.it = route;
                                        }
                                        // `unpark` is deferred, so the
                                        // `Parked` check above and the actual
                                        // unpark do not observe the same
                                        // world. If the truck left `Parked`
                                        // in between, `unpark` refuses
                                        // (sov-6qx) and never moves it --
                                        // and a `Parked` vehicle has no
                                        // collider, so `vehicle_decision_system`
                                        // skips it and its itinerary never
                                        // ends. `MAX_SOURCE_WAIT_TICKS` only
                                        // counts down on the `truck: None`
                                        // arm, so logging the refusal and
                                        // keeping `truck = Some(v)` would
                                        // freeze this dispatch, the truck and
                                        // the seller's reserved quantity
                                        // forever, with no bound at all.
                                        // Undo our own bookkeeping instead:
                                        // release the truck and drop back to
                                        // "no truck yet" so the next tick
                                        // retries.
                                        cbuf_vehicle.exec_ent(v, move |sim| {
                                            if unpark(sim, v) {
                                                return;
                                            }
                                            log::warn!(
                                                "{:?} left Parked before its deferred unpark; \
                                                 releasing it back to the dispatcher",
                                                v
                                            );
                                            sim.write::<Market>().release_tosource_truck(v);
                                            sim.write::<Dispatcher>()
                                                .free(DispatchID::SmallTruck(v));
                                        });
                                        self.dispatches[i].truck = Some(v);
                                    } else {
                                        dispatcher.free(DispatchID::SmallTruck(v));
                                    }
                                }
                            }
                        }
                        // No truck available (or no route found, or the
                        // seller's building is gone): stay in ToSource and
                        // retry next tick. Nothing is debited.
                        //
                        // But not forever (sov-ahw). This arm used to have no
                        // counter and no `else` at all, so a seller the
                        // `Dispatcher`'s BFS can never reach — the ordinary
                        // case of a freight station the player has not
                        // connected by road yet — left the dispatch immortal.
                        // Immortal here does not just wedge the dispatch: the
                        // buy order was consumed by the match, and its only
                        // re-post path (`recipe_act` -> `buy_until`) is gated
                        // on capital this import was supposed to deliver, so
                        // the ENTERPRISE dies permanently. Undo our own
                        // bookkeeping in full instead — reservation, sell
                        // order, and the buy order — so both halves of
                        // the match go back on the market and the city can
                        // serve it the moment a route exists.
                        //
                        // No money moves here: since sov-7f7 the border
                        // commitment settles at delivery, never at match, so
                        // a dispatch that never left `ToSource` settled
                        // nothing. (Under match-time settlement this arm also
                        // refunded `money_delta`; that would now pay back
                        // money that was never taken.)
                        if self.dispatches[i].truck.is_none() {
                            if source_wait_ticks + 1 < MAX_SOURCE_WAIT_TICKS {
                                self.dispatches[i].source_wait_ticks = source_wait_ticks + 1;
                            } else {
                                // Bounded wait, full rollback (sov-ahw): the
                                // goods never left `ToSource`, so nothing was
                                // ever debited and (since sov-7f7) no border
                                // leg settled either. Hand both halves of the
                                // match back — reservation, sell order, AND
                                // the buy order (a bare countdown without the
                                // re-post leaves the enterprise just as dead)
                                // — so the city can serve it the moment a
                                // route exists.
                                let mut ctx = ExitCtx {
                                    map,
                                    binfos,
                                    world: &mut *world,
                                    dispatcher: &mut *dispatcher,
                                    parking: Some(&mut *parking),
                                    tick,
                                };
                                self.terminate_dispatch(
                                    i,
                                    &mut ctx,
                                    true,
                                    true,
                                    false,
                                    "gave up waiting for a truck; reservation released and the buy order re-posted",
                                );
                                continue;
                            }
                        }
                    }
                    Some(v) => {
                        if world.vehicles.get(v).is_none() {
                            // Wedge (b): the reserved vehicle entity is gone
                            // (e.g. despawned) before it ever arrived.
                            // Nothing was ever debited from the seller, so
                            // the goods never physically left — but the
                            // MATCH is still consumed, so the full rollback
                            // (reservation, sell order, AND buy order) runs
                            // here too (sov-5ut). Before, this arm freed and
                            // dropped without re-posting, stranding the
                            // enterprise exactly like the pre-sov-ahw hang.
                            let mut ctx = ExitCtx {
                                map,
                                binfos,
                                world: &mut *world,
                                dispatcher: &mut *dispatcher,
                                parking: Some(&mut *parking),
                                tick,
                            };
                            self.terminate_dispatch(
                                i,
                                &mut ctx,
                                true,
                                true,
                                false,
                                "reserved truck vanished before arriving; match rolled back",
                            );
                            continue;
                        } else {
                            let arrived = world
                                .vehicles
                                .get(v)
                                .map(|ve| ve.it.has_ended(0.0))
                                .unwrap_or(false);
                            if arrived {
                                // sov-uo5 Border custody: a FreightStation
                                // seller draws from its bounded stock ledger,
                                // not from capital and not from nothing. On
                                // empty stock the dispatch waits visibly in
                                // ToSource (going-without at the border):
                                // no debit, no settlement, no transition.
                                // Domestic sellers keep the capital debit.
                                // Money flow untouched by the branch itself.
                                let drew = match seller {
                                    SoulID::FreightStation(fid) => world
                                        .freight_stations
                                        .get_mut(fid)
                                        .is_some_and(|e| e.f.try_draw_border_stock(qty)),
                                    _ => true,
                                };
                                if !drew {
                                    log::warn!(
                                        "import dispatch waiting: border stock empty \
                                         at {:?} for {:?} x{} to {:?}",
                                        seller,
                                        kind,
                                        qty,
                                        buyer
                                    );
                                } else {
                                    if !matches!(seller, SoulID::FreightStation(_)) {
                                        let m = self.m(kind);
                                        *m.capital.entry(seller).or_default() -= qty as i32;
                                    }
                                    let m = self.m(kind);
                                    if let Some(r) = m.reserved.get_mut(&seller) {
                                        *r = r.saturating_sub(qty);
                                    }
                                    // sov-7f7 (ADR-0003 §1): the truck is loaded,
                                    // so an import's border commitment settles
                                    // now — not at the match, thousands of ticks
                                    // ago. Domestic legs carry a ZERO delta, so
                                    // this only ever moves border money.
                                    if matches!(seller, SoulID::FreightStation(_)) {
                                        settled += money_delta;
                                    }
                                    let d = &mut self.dispatches[i];
                                    d.state = DispatchState::Loading;
                                    d.ticks_left = DISPATCH_DWELL_TICKS;
                                }
                            }
                        }
                    }
                },
                DispatchState::Loading => {
                    if ticks_left == 0 {
                        // Invariant: truck is always Some by the time a
                        // dispatch reaches Loading (set on arrival in ToSource).
                        if let Some(v) = truck {
                            if world.vehicles.get(v).is_none() {
                                // Truck vanished mid-loading: the goods were
                                // on it and are gone with it. Seller was
                                // already debited (ToSource arrival), buyer
                                // never credited — a real physical loss, not
                                // a teleport, so nothing is re-credited. The
                                // buyer lives on: its demand is re-posted
                                // and a settled border leg refunded inside.
                                let mut ctx = ExitCtx {
                                    map,
                                    binfos,
                                    world: &mut *world,
                                    dispatcher: &mut *dispatcher,
                                    parking: Some(&mut *parking),
                                    tick,
                                };
                                self.terminate_dispatch(
                                    i,
                                    &mut ctx,
                                    true,
                                    false,
                                    true,
                                    "truck vanished while loading",
                                );
                                continue;
                            } else if let Some(buyer_pos) = door_pos(buyer, map, binfos) {
                                let start = world.vehicles.get(v).map(|ve| ve.trans.pos);
                                let route = start.and_then(|start| {
                                    Itinerary::route(tick, start, buyer_pos, map, PathKind::Vehicle)
                                });
                                if let Some(route) = route {
                                    if let Some(ve) = world.vehicles.get_mut(v) {
                                        ve.it = route;
                                    }
                                    self.dispatches[i].state = DispatchState::ToDestination;
                                } else if return_route_retries >= MAX_RETURN_ROUTE_RETRIES {
                                    // sov-jcl: the buyer's building is still
                                    // standing but no route reaches it (e.g.
                                    // the road between them was bulldozed).
                                    // Retrying unbounded here keeps the truck
                                    // reserved out of the dispatcher pool and
                                    // the dispatch immortal, so bound it
                                    // exactly like the return leg below. The
                                    // goods were debited at `ToSource`
                                    // arrival and are physically on the
                                    // truck: this is an honest logged loss,
                                    // never a teleport-refund. The truck is
                                    // parked-then-freed inside (sov-91e), the
                                    // live buyer's demand re-posted, a
                                    // settled border leg refunded.
                                    let mut ctx = ExitCtx {
                                        map,
                                        binfos,
                                        world: &mut *world,
                                        dispatcher: &mut *dispatcher,
                                        parking: Some(&mut *parking),
                                        tick,
                                    };
                                    self.terminate_dispatch(
                                        i,
                                        &mut ctx,
                                        true,
                                        false,
                                        true,
                                        "no route to a live buyer after repeated attempts",
                                    );
                                    continue;
                                } else {
                                    // No route found: stay in Loading
                                    // (ticks_left at 0) and retry next tick.
                                    self.dispatches[i].return_route_retries =
                                        return_route_retries + 1;
                                }
                            } else {
                                // Wedge (a): buyer's building was demolished.
                                // The goods are already debited from the
                                // seller and physically on the truck; drive
                                // them back instead of teleport-refunding
                                // (sov-otw: one shared attempt, retries
                                // counted below on failure only).
                                match self.try_return_to_seller(i, map, binfos, &mut *world, tick)
                                {
                                    ReturnOutcome::Returning => {}
                                    ReturnOutcome::NoRouteBack => {
                                        if return_route_retries >= MAX_RETURN_ROUTE_RETRIES
                                        {
                                            // No route back after repeated tries
                                            // (e.g. the road home was severed):
                                            // treat as an honest physical loss
                                            // rather than retry forever. No
                                            // re-post: the buyer is gone.
                                            let mut ctx = ExitCtx {
                                                map,
                                                binfos,
                                                world: &mut *world,
                                                dispatcher: &mut *dispatcher,
                                                parking: Some(&mut *parking),
                                                tick,
                                            };
                                            self.terminate_dispatch(
                                                i,
                                                &mut ctx,
                                                false,
                                                false,
                                                true,
                                                "no route back to seller after repeated attempts",
                                            );
                                            continue;
                                        } else {
                                            self.dispatches[i].return_route_retries =
                                                return_route_retries + 1;
                                        }
                                    }
                                    ReturnOutcome::SellerGone => {
                                        // Seller is also gone: nothing left
                                        // to return the goods to.
                                        let mut ctx = ExitCtx {
                                            map,
                                            binfos,
                                            world: &mut *world,
                                            dispatcher: &mut *dispatcher,
                                            parking: Some(&mut *parking),
                                            tick,
                                        };
                                        self.terminate_dispatch(
                                            i,
                                            &mut ctx,
                                            false,
                                            false,
                                            true,
                                            "both buyer and seller buildings are gone",
                                        );
                                        continue;
                                    }
                                }
                            }
                        }
                    } else {
                        self.dispatches[i].ticks_left = ticks_left - 1;
                    }
                }
                DispatchState::ToDestination => {
                    if truck.is_some_and(|v| world.vehicles.get(v).is_none()) {
                        // Truck vanished mid-transit: same physical loss as
                        // the Loading case above. The buyer lives on: its
                        // demand is re-posted and a settled border leg
                        // refunded inside.
                        let mut ctx = ExitCtx {
                            map,
                            binfos,
                            world: &mut *world,
                            dispatcher: &mut *dispatcher,
                            parking: Some(&mut *parking),
                            tick,
                        };
                        self.terminate_dispatch(
                            i,
                            &mut ctx,
                            true,
                            false,
                            true,
                            "truck vanished in transit",
                        );
                        continue;
                    } else {
                        let arrived = truck
                            .and_then(|v| world.vehicles.get(v))
                            .map(|ve| ve.it.has_ended(0.0))
                            .unwrap_or(false);
                        if arrived {
                            let m = self.m(kind);
                            *m.capital.entry(buyer).or_default() += qty as i32;
                            // sov-7f7 (ADR-0003 §1): the truck unloaded at
                            // the border door, so an export's commitment
                            // settles now — mirror of the Loading hook above.
                            // Domestic legs carry a ZERO delta.
                            if matches!(buyer, SoulID::FreightStation(_)) {
                                settled += money_delta;
                            }
                            let d = &mut self.dispatches[i];
                            d.state = DispatchState::Unloading;
                            d.ticks_left = DISPATCH_DWELL_TICKS;
                        }
                    }
                }
                DispatchState::Returning => {
                    if truck.is_some_and(|v| world.vehicles.get(v).is_none()) {
                        // Truck vanished mid-return: same physical loss as
                        // the other in-flight states. The goods were already
                        // debited from the seller and never re-credited. A
                        // settled border leg is refunded inside; the buyer
                        // re-post is skipped automatically when its door is
                        // gone (the usual reason a dispatch is Returning).
                        let mut ctx = ExitCtx {
                            map,
                            binfos,
                            world: &mut *world,
                            dispatcher: &mut *dispatcher,
                            parking: Some(&mut *parking),
                            tick,
                        };
                        self.terminate_dispatch(
                            i,
                            &mut ctx,
                            true,
                            false,
                            true,
                            "truck vanished while returning",
                        );
                        continue;
                    } else {
                        let arrived = truck
                            .and_then(|v| world.vehicles.get(v))
                            .map(|ve| ve.it.has_ended(0.0))
                            .unwrap_or(false);
                        if arrived {
                            // Physically back at the seller: re-credit the goods
                            // that were debited on the way out.
                            *self.m(kind).capital.entry(seller).or_default() += qty as i32;
                            if let Some(v) = truck {
                                // Same abandoned-truck shape as Unloading
                                // (sov-2c4): park it for real instead of
                                // leaving it Driving at the seller's door.
                                if let Some(pos) = world.vehicles.get(v).map(|ve| ve.trans.pos) {
                                    if let Ok(spot) = parking.reserve_near(pos, map) {
                                        if let Some(ve) = world.vehicles.get_mut(v) {
                                            park(map, ve, spot);
                                        }
                                    }
                                }
                                dispatcher.free(DispatchID::SmallTruck(v));
                            }
                            remove = true;
                        }
                    }
                }
                DispatchState::Unloading => {
                    if ticks_left == 0 {
                        if let Some(v) = truck {
                            // Genuinely park the truck instead of leaving it
                            // Driving wherever it stopped -- an abandoned
                            // truck sits in the door's live lane and blocks
                            // every later dispatch to the same door
                            // (sov-2c4). `dispatcher.free` is deferred until
                            // AFTER parking succeeds: freeing it first made
                            // the truck re-grabbable while still Driving
                            // toward its spot, and the grab path's
                            // unconditional `unpark` call then warned
                            // "wasn't parked" and clobbered its collider
                            // without removing the stale grid entry --
                            // exactly the phantom-blocker shape sov-2c4 was
                            // filed for, confirmed while building this fix
                            // (sov-7pg). `park` (map_dynamic::router) is the
                            // same spline machinery `RoutingStep::Park`
                            // already uses; it owns the `SpotReservation`
                            // going forward and `vehicle_state_update`
                            // (transportation/road.rs) frees the truck's
                            // collider properly on arrival and flips it to
                            // `VehicleState::Parked`, so the NEXT `unpark`
                            // call on this truck is legitimate, not a warn.
                            if let Some(pos) = world.vehicles.get(v).map(|ve| ve.trans.pos) {
                                if let Ok(spot) = parking.reserve_near(pos, map) {
                                    if let Some(ve) = world.vehicles.get_mut(v) {
                                        park(map, ve, spot);
                                    }
                                }
                            }
                            dispatcher.free(DispatchID::SmallTruck(v));
                        }
                        remove = true;
                    } else {
                        self.dispatches[i].ticks_left = ticks_left - 1;
                    }
                }
            }

            if remove {
                self.dispatches.swap_remove(i);
            } else {
                i += 1;
            }
        }

        // Retail claims (see `RetailClaim`) have no dispatch/truck to time
        // out on the road, so they get their own countdown here: a human
        // that never makes it to the seller (stuck, despawned mid-journey
        // some other way, or otherwise wedged) would otherwise freeze the
        // seller's reservation forever, same failure shape as the dispatch
        // wedges this function already guards against. Expiry releases the
        // reservation but must NOT touch `last_ate` — the buyer goes without
        // and re-queues, hunger keeps rising (never game over).
        self.retail_claims.retain(|_, claim| {
            if claim.ticks_left == 0 {
                if let Some(m) = self.markets.get_mut(&claim.kind) {
                    if let Some(r) = m.reserved.get_mut(&claim.seller) {
                        *r = r.saturating_sub(claim.qty);
                    }
                }
                return false;
            }
            claim.ticks_left -= 1;
            true
        });
        settled
    }
}

/// Test seam (sov-7f7): push a fully-formed dispatch without running a match.
/// The export leg needs it because exports get no dispatch of their own until
/// sov-20g lands the export dispatch loop; the settlement suite drives the
/// `ToDestination` hook through it. Zero behavior impact: compiled out of
/// every non-test build.
#[cfg(test)]
impl Market {
    pub(crate) fn test_push_dispatch(
        &mut self,
        buyer: SoulID,
        seller: SoulID,
        kind: ItemID,
        qty: u32,
        money_delta: Money,
    ) {
        self.dispatches.push(Dispatch {
            buyer,
            seller,
            kind,
            qty,
            money_delta,
            state: DispatchState::ToSource,
            ticks_left: 0,
            truck: None,
            return_route_retries: 0,
            source_wait_ticks: 0,
            sell_order: None,
        });
    }
}

/// The door position of the building a soul owns, if it owns one.
fn door_pos(soul: SoulID, map: &Map, binfos: &BuildingInfos) -> Option<Vec3> {
    let b = find_trade_place(TradeTarget(soul), binfos)?;
    Some(map.buildings.get(b)?.door_pos)
}

fn calculate_prices(price_multiplier: f32) -> BTreeMap<ItemID, Money> {
    let mut item_graph: BTreeMap<ItemID, Vec<GoodsCompanyID>> = BTreeMap::new();
    for company in GoodsCompanyPrototype::iter() {
        let Some(ref recipe) = company.recipe else {
            continue;
        };
        for item in &recipe.production {
            item_graph.entry(item.id).or_default().push(company.id);
        }
    }

    let mut prices = BTreeMap::new();
    fn calculate_price_inner(
        item_graph: &BTreeMap<ItemID, Vec<GoodsCompanyID>>,
        id: ItemID,
        prices: &mut BTreeMap<ItemID, Money>,
        price_multiplier: f32,
    ) {
        if prices.contains_key(&id) {
            return;
        }

        let mut minprice = None;
        for &comp in item_graph.get(&id).unwrap_or(&vec![]) {
            let company = &comp.prototype();
            let mut price_consumption = Money::ZERO;
            let Some(ref recipe) = company.recipe else {
                continue;
            };
            for recipe_item in &recipe.consumption {
                calculate_price_inner(item_graph, recipe_item.id, prices, price_multiplier);
                price_consumption += prices[&recipe_item.id] * recipe_item.amount as i64;
            }
            let qty = recipe
                .production
                .iter()
                .find_map(|x| (x.id == id).then_some(x.amount))
                .unwrap_or(0) as i64;

            // `validate()` (prototypes/src/validation.rs) refuses any recipe
            // amount below 1, so qty is >= 1 for every loaded prototype. Guard
            // anyway: this division is the only arithmetic here that can panic,
            // and a panic on a live path ends the game, which the pillars
            // forbid. Skipping the recipe as a price CANDIDATE is the honest
            // response -- `minprice` already tolerates having no candidate at
            // all (`unwrap_or(Money::ZERO)` below) -- whereas substituting any
            // divisor would publish a silently wrong price instead.
            if qty <= 0 {
                continue;
            }

            let price_workers = recipe.duration.minutes()
                * company.n_workers as f64
                * WORKER_CONSUMPTION_PER_MINUTE;

            let newprice = (price_consumption
                + Money::new_inner((price_workers.inner() as f32 * price_multiplier) as i64))
                / qty;

            minprice = minprice.map(|x: Money| x.min(newprice)).or(Some(newprice));
        }

        prices.insert(id, minprice.unwrap_or(Money::ZERO));
    }

    for item in ItemPrototype::iter() {
        calculate_price_inner(&item_graph, item.id, &mut prices, price_multiplier);
    }

    prices
}

#[cfg(test)]
mod tests {
    use geom::{vec2, Vec2};
    use prototypes::test_prototypes;
    use prototypes::ItemID;

    use crate::economy::WORKER_CONSUMPTION_PER_MINUTE;
    use crate::world::CompanyID;
    use crate::{FreightStationID, SoulID};

    use super::Market;

    fn mk_ent(id: u64) -> CompanyID {
        CompanyID::from(slotmapd::KeyData::from_ffi(id))
    }

    #[test]
    fn test_match_orders() {
        let seller = SoulID::GoodsCompany(mk_ent((1 << 32) | 1));
        let seller_far = SoulID::GoodsCompany(mk_ent((1 << 32) | 2));
        let buyer = SoulID::GoodsCompany(mk_ent((1 << 32) | 3));
        let freight = SoulID::FreightStation(FreightStationID::from(slotmapd::KeyData::from_ffi(
            (1 << 32) | 4,
        )));

        test_prototypes(
            r#"
        data:extend {
          {
            type = "item",
            name = "cereal",
            label = "Cereal"
          },
          {
            type = "item",
            name = "wheat",
            label = "Wheat",
          }
        }
        "#,
        );

        let mut m = Market::default();

        let cereal = ItemID::new("cereal");

        m.produce(seller, cereal, 3);
        m.produce(seller_far, cereal, 3);

        m.buy(buyer, Vec2::ZERO, cereal, 2);
        m.sell(seller, Vec2::X, cereal, 3, 5);
        m.sell(seller_far, vec2(10.0, 10.0), cereal, 3, 5);

        let trades = m.make_trades(|_| Some(freight));

        assert_eq!(trades.len(), 1);
        let t0 = trades[0];
        assert_eq!(t0.seller.0, seller);
        assert_eq!(t0.buyer.0, buyer);
        assert_eq!(t0.qty, 2);
    }

    #[test]
    fn calculate_prices() {
        test_prototypes(
            r#"
        data:extend {
          {
            type = "item",
            name = "cereal",
            label = "Cereal"
          },
          {
            type = "item",
            name = "wheat",
            label = "Wheat",
          }
        }
        
        data:extend {{
            type = "goods-company",
            name = "cereal-farm",
            label = "Cereal farm",
            kind = "factory",
            bgen = "farm",
            recipe = {
                production = {
                    {"cereal", 3}
                },
                consumption = {},
                duration = "3m",
                storage_multiplier = 5,
            },
            n_trucks = 1,
            n_workers = 2,
            size = 0.0,
            asset = "no.jpg",
            price = 0,
        },
        {
            type = "goods-company",
            name = "wheat-factory",
            label = "Wheat factory",
            kind = "factory",
            bgen = "farm",
            recipe = {
                production = {
                    {"wheat", 2}
                },
                consumption = {
                    {"cereal", 2}
                },
                duration = "10m",
                storage_multiplier = 5,
            },
            n_trucks = 1,
            n_workers = 5,
            size = 0.0,
            asset = "no.jpg",
            price = 0,
        }}
        "#,
        );

        let cereal = ItemID::new("cereal");
        let wheat = ItemID::new("wheat");

        let prices = super::calculate_prices(1.0);

        assert_eq!(prices.len(), 2);
        let price_cereal = 2 * WORKER_CONSUMPTION_PER_MINUTE;
        assert_eq!(prices[&cereal], price_cereal);
        assert_eq!(
            prices[&wheat],
            (price_cereal * 2 + 5 * WORKER_CONSUMPTION_PER_MINUTE * 10) / 2
        );
    }

    #[test]
    fn buy_until_negative_capital_orders_sane_qty() {
        test_prototypes(
            r#"
        data:extend {
          {
            type = "item",
            name = "cereal",
            label = "Cereal"
          }
        }
        "#,
        );

        let mut m = Market::default();
        let cereal = ItemID::new("cereal");
        let soul = SoulID::GoodsCompany(mk_ent((1 << 32) | 5));

        // Negative capital is routine (e.g. FreightStation after sov-abs).
        m.produce(soul, cereal, -5);
        assert!(m.capital(soul, cereal) < 0);

        // Must not panic/wrap to ~4.29e9: raw `qty - c as u32` underflows
        // in debug and wraps to qty + 5 in release, both caught below.
        m.buy_until(soul, Vec2::ZERO, cereal, 10);
        let ordered = m
            .m(cereal)
            .buy_order(soul)
            .expect("buy_until places a buy order")
            .qty;
        assert!(
            ordered > 0 && ordered <= 10,
            "sane order quantity, got {ordered}"
        );
    }
    /// sov-sp6: the external-trade branch of `Market::make_trades` moves
    /// money and quantity on every successful leg, sign and magnitude. Eight
    /// mutation shapes once survived here with the suite green (sign flips
    /// on both money legs, magnitude swaps, surplus/guard/reservation/order
    /// corruption); each is pinned below.
    ///
    /// Shape note (post-sov-20g/sov-7f7): the ticket's acceptance text
    /// predates physical exports and delivery settlement, when the match
    /// debited the seller's capital outright. On the final shape the match
    /// only RESERVES (capital moves at the `Loading` arrival, proved by
    /// `sov_20g_export_is_physical_dispatch_to_border`), so (a) asserts the
    /// reservation plus the order decrement plus capital-UNTOUCHED — a
    /// same-tick debit here would be the teleport, and the test fails on it.
    #[test]
    fn sov_sp6_ext_trade_money_and_quantity() {
        test_prototypes(
            r#"
        data:extend {
          {
            type = "item",
            name = "cereal",
            label = "Cereal"
          }
        }
        "#,
        );

        let mut m = Market::default();
        let cereal = ItemID::new("cereal");
        let seller = SoulID::GoodsCompany(mk_ent((1 << 32) | 21));
        let buyer = SoulID::GoodsCompany(mk_ent((1 << 32) | 22));
        let freight = SoulID::FreightStation(FreightStationID::from(slotmapd::KeyData::from_ffi(
            (1 << 32) | 23,
        )));
        let ext_value = m.inner()[&cereal].ext_value;

        m.produce(seller, cereal, 10);
        m.sell(seller, Vec2::X, cereal, 10, 0);
        // 15 exceeds the seller's 10, so no domestic match may steal the
        // buyer: the BUY leg must clear externally, the SELL leg its surplus.
        m.buy(buyer, Vec2::ZERO, cereal, 15);

        // Pass 1: the fresh buy order queues for a domestic pass first
        // (sov-b70), so only the SELL leg trades.
        let pass1: Vec<_> = m
            .make_trades(|_| Some(freight))
            .iter()
            .filter(|t| t.kind == cereal)
            .copied()
            .collect();
        assert_eq!(pass1.len(), 1, "the seller's surplus must export: {pass1:?}");
        let sell = pass1[0];
        assert_eq!(sell.seller.0, seller);
        assert_eq!(sell.buyer.0, freight);
        assert_eq!(sell.qty, 10, "the whole surplus above stock 0 exports");
        // (c) sell Trade.money_delta: sign AND magnitude.
        assert_eq!(
            sell.money_delta,
            ext_value * 10,
            "export money delta must be +(value*qty)"
        );
        // (a) post-20g shape: reservation and order move, capital does not.
        assert_eq!(
            m.reserved(seller, cereal),
            10,
            "matched export stock is reserved, not teleported away"
        );
        assert_eq!(
            m.m(cereal).sell_order(seller).unwrap().qty,
            0,
            "the matched surplus leaves the sell order"
        );
        assert_eq!(
            m.capital(seller, cereal),
            10,
            "the match itself must not debit capital (sov-20g teleport shape)"
        );

        // Pass 2: the aged buy order is border-eligible and imports.
        let pass2: Vec<_> = m
            .make_trades(|_| Some(freight))
            .iter()
            .filter(|t| t.kind == cereal)
            .copied()
            .collect();
        assert_eq!(pass2.len(), 1, "the queued buyer must import: {pass2:?}");
        let buy = pass2[0];
        assert_eq!(buy.buyer.0, buyer);
        assert_eq!(buy.seller.0, freight);
        assert_eq!(buy.qty, 15);
        // (b) buy Trade.money_delta: sign AND magnitude.
        assert_eq!(
            buy.money_delta,
            -(ext_value * 15),
            "import money delta must be -(value*qty)"
        );
    }
}
