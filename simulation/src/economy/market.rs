use std::collections::btree_map::Entry;
use std::collections::BTreeMap;

use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};

use geom::{Vec2, Vec3};
use prototypes::{
    prototypes_iter, GoodsCompanyID, GoodsCompanyPrototype, ItemPrototype, Money, Tick,
    TICKS_PER_HOUR,
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
/// route back to the seller (see `DispatchState::Returning`) before giving
/// up and treating the goods as lost. A severed road can make the route
/// search fail forever; without a bound this reintroduces the exact wedge
/// shape this ticket exists to close.
const MAX_RETURN_ROUTE_RETRIES: u32 = 20;

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
    pub state: DispatchState,
    ticks_left: u32,
    /// The truck carrying this dispatch. `None` while waiting for the
    /// `Dispatcher` to find one available.
    truck: Option<crate::world::VehicleID>,
    /// Failed `Itinerary::route` attempts while trying to route the truck
    /// back to the seller after the buyer's building was demolished (see
    /// `DispatchState::Returning`). A severed road can make this fail
    /// forever, so it's bounded (see `MAX_RETURN_ROUTE_RETRIES`) rather than
    /// retried indefinitely — that would just reintroduce the wedge shape.
    return_route_retries: u32,
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
        // reference. A dead SELLER is handled by the blanket `reserved`/
        // `capital` removal above (the seller's own row is gone, so there's
        // nothing left to credit or reserve); such dispatches are simply
        // dropped, same as before.
        //
        // A dead BUYER with a surviving seller is different: the seller's
        // row is still live, so silently dropping the dispatch either
        // strands `reserved[seller]` forever (`ToSource`, nothing debited
        // yet) or destroys goods already debited from the seller with no
        // sink (`Loading`/`ToDestination`/`Returning`) -- see
        // sov-dispatch-wedge-ab4. Route each such dispatch through the same
        // fate a live buyer-demolition takes in `advance_dispatches`.
        let mut i = 0;
        while i < self.dispatches.len() {
            let d = &self.dispatches[i];
            if d.buyer != soul || d.seller == soul {
                i += 1;
                continue;
            }
            let (seller, kind, qty, state, truck) = (d.seller, d.kind, d.qty, d.state, d.truck);
            match state {
                DispatchState::ToSource => {
                    // Nothing was ever debited: freeing the reservation is
                    // the whole fix, the goods never physically left.
                    if let Some(r) = self.m(kind).reserved.get_mut(&seller) {
                        *r = r.saturating_sub(qty);
                    }
                    if let Some(v) = truck {
                        dispatcher.free(DispatchID::SmallTruck(v));
                    }
                    self.dispatches.swap_remove(i);
                    continue;
                }
                DispatchState::Loading
                | DispatchState::ToDestination
                | DispatchState::Returning => {
                    // Seller already debited (or is mid-return): drive the
                    // goods physically back instead of teleport-refunding or
                    // destroying them.
                    if let Some(seller_pos) = door_pos(seller, map, binfos) {
                        let start = truck
                            .and_then(|v| world.vehicles.get(v))
                            .map(|ve| ve.trans.pos);
                        let route = start.and_then(|start| {
                            Itinerary::route(tick, start, seller_pos, map, PathKind::Vehicle)
                        });
                        if let Some(route) = route {
                            if let Some(v) = truck {
                                if let Some(ve) = world.vehicles.get_mut(v) {
                                    ve.it = route;
                                }
                            }
                            self.dispatches[i].state = DispatchState::Returning;
                        } else {
                            // No route back to the seller: an honest
                            // physical loss (the goods are already debited
                            // from the seller and never credited to anyone),
                            // same shape as the sibling loss paths above.
                            log::warn!(
                                "dispatch {:?} {:?} ({:?} -> dead buyer): no route back to \
                                 seller, treating as lost",
                                qty,
                                kind,
                                seller
                            );
                            if let Some(v) = truck {
                                dispatcher.free(DispatchID::SmallTruck(v));
                            }
                            self.dispatches.swap_remove(i);
                            continue;
                        }
                    } else {
                        // Seller is also gone: nothing left to return the
                        // goods to.
                        log::warn!(
                            "dispatch {:?} {:?} lost: buyer and seller are both gone",
                            qty,
                            kind
                        );
                        if let Some(v) = truck {
                            dispatcher.free(DispatchID::SmallTruck(v));
                        }
                        self.dispatches.swap_remove(i);
                        continue;
                    }
                }
                DispatchState::Unloading => {
                    // The buyer was about to be credited; it no longer
                    // exists to receive the goods, so nothing is credited.
                    // The seller was already debited when the truck loaded
                    // and does not get the goods back (mirrors Unloading's
                    // own honest-loss shape: once loaded, goods that never
                    // reach a buyer are gone, not refunded).
                    log::warn!(
                        "dispatch {:?} {:?} lost: buyer removed while unloading",
                        qty,
                        kind
                    );
                    if let Some(v) = truck {
                        dispatcher.free(DispatchID::SmallTruck(v));
                    }
                    self.dispatches.swap_remove(i);
                    continue;
                }
            }
            i += 1;
        }
        // A dead SELLER's rows were wiped above, so there is nothing left to
        // credit or reserve and the dispatch is simply dropped -- but the
        // truck it holds must go back to the pool first. `Dispatcher::query`
        // skips anything still in `reserved_by` and only `free` clears it, so
        // dropping the dispatch without freeing removes that truck from the
        // city permanently (sov-dispatch-wedge-ab4 round 4).
        for d in self.dispatches.iter().filter(|d| d.seller == soul) {
            if let Some(v) = d.truck {
                dispatcher.free(DispatchID::SmallTruck(v));
            }
        }
        self.dispatches.retain(|d| d.seller != soul);
    }

    /// Called when an agent tells the world it wants to buy something
    /// If an order is already placed, it will be updated.
    pub fn buy(&mut self, soul: SoulID, near: Vec2, kind: ItemID, qty: u32) {
        log::debug!("{:?} buy {:?} {:?} near {:?}", soul, qty, kind, near);

        self.m(kind)
            .buy_orders
            .insert(soul, BuyOrder { pos: near, qty });
    }

    pub fn buy_until(&mut self, soul: SoulID, near: Vec2, kind: ItemID, qty: u32) {
        let c = self.capital(soul, kind);
        if c >= qty as i32 {
            return;
        }
        self.buy(soul, near, kind, qty - c as u32);
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

    pub fn dispatches(&self) -> &[Dispatch] {
        &self.dispatches
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
        self.all_trades.clear();

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
                            // `RetailClaim`s are only ever created for the
                            // market's own `kind` (this loop iterates one
                            // kind at a time), so the old claim's reservation
                            // always lives in this same `reserved` map.
                            debug_assert_eq!(old.kind, kind);
                            if let Some(r) = reserved.get_mut(&old.seller) {
                                *r = r.saturating_sub(old.qty);
                            }
                        }
                        continue;
                    }
                    self.dispatches.push(Dispatch {
                        buyer: trade.buyer.0,
                        seller: trade.seller.0,
                        kind,
                        qty: trade.qty as u32,
                        state: DispatchState::ToSource,
                        ticks_left: 0,
                        truck: None,
                        return_route_retries: 0,
                    });
                }
            }

            // External trading
            if !*optout_exttrade {
                // Humans never clear through the external market: retail
                // clears by queue and going-without only (never by money —
                // an ext-trade buy attaches a money_delta and credits
                // capital directly, both forbidden on the human path, see
                // `RetailClaim`). Their unmatched buy orders must survive
                // this pass untouched so they're still there for next
                // tick's domestic match, not silently dropped.
                let btaken: BTreeMap<_, _> = buy_orders
                    .extract_if(.., |s, _| !matches!(s, SoulID::Human(_)))
                    .collect();
                // All remaining (non-human) buyers can fulfil since they can buy externally
                self.all_trades.reserve(btaken.len());
                for (buyer, order) in btaken {
                    let qty_buy = order.qty as i32;

                    let Some(ext) = find_external(order.pos) else {
                        continue;
                    };

                    *capital.entry(buyer).or_default() += qty_buy;

                    self.all_trades.push(Trade {
                        buyer: TradeTarget(buyer),
                        seller: TradeTarget(ext),
                        qty: qty_buy,
                        kind,
                        money_delta: -(*ext_value * qty_buy as i64), // we buy from external so we pay
                    });
                }

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

                    let Some(ext) = find_external(order.pos) else {
                        continue;
                    };

                    *cap -= qty_sell;
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
        }

        &self.all_trades
    }

    pub fn inner(&self) -> &BTreeMap<ItemID, SingleMarket> {
        &self.markets
    }

    /// Advances every in-flight dispatch by one tick, sequencing it through
    /// ToSource -> Loading -> ToDestination -> Unloading, driven by a real truck
    /// reserved from the `Dispatcher` and physically driven over the road network.
    /// The seller's capital is debited exactly when the truck arrives and enters
    /// `Loading`, and the buyer's capital is credited exactly when it arrives and
    /// enters `Unloading`; must be called once per tick for dispatches to ever
    /// complete. With no truck available, a dispatch simply waits in `ToSource` —
    /// no capital moves.
    /// Rolls back the truck reservation `advance_dispatches` recorded for a
    /// `ToSource` dispatch, putting it back to "waiting for a truck" so the
    /// next tick retries. Returns whether a dispatch was actually rolled back.
    ///
    /// Needed because the truck is assigned here but `unpark` runs deferred
    /// through `ParCommandBuffer<VehicleEnt>`: if the truck stopped being
    /// `Parked` in between, `unpark` refuses (sov-6qx) and the truck never
    /// moves. `ToSource` has no tick countdown, so a dispatch left holding a
    /// motionless truck waits forever and freezes the seller's reservation
    /// with it. Identified by the truck rather than by index because
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

    pub fn advance_dispatches(
        &mut self,
        world: &mut World,
        map: &Map,
        binfos: &BuildingInfos,
        dispatcher: &mut Dispatcher,
        cbuf_vehicle: &ParCommandBuffer<VehicleEnt>,
        parking: &mut ParkingManagement,
        tick: Tick,
    ) {
        let mut i = 0;
        while i < self.dispatches.len() {
            let (seller, buyer, kind, qty, state, ticks_left, truck, return_route_retries) = {
                let d = &self.dispatches[i];
                (
                    d.seller,
                    d.buyer,
                    d.kind,
                    d.qty,
                    d.state,
                    d.ticks_left,
                    d.truck,
                    d.return_route_retries,
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
                                        // ends. `ToSource` has no timeout, so
                                        // logging the refusal and keeping
                                        // `truck = Some(v)` would freeze this
                                        // dispatch, the truck and the
                                        // seller's reserved quantity forever.
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
                        // No truck available (or no route found): stay in
                        // ToSource and retry next tick. Nothing is debited.
                    }
                    Some(v) => {
                        if world.vehicles.get(v).is_none() {
                            // Wedge (b): the reserved vehicle entity is gone
                            // (e.g. despawned) before it ever arrived.
                            // Nothing was ever debited from the seller, so
                            // freeing the reservation is the whole fix — the
                            // goods never physically left.
                            dispatcher.free(DispatchID::SmallTruck(v));
                            if let Some(r) = self.m(kind).reserved.get_mut(&seller) {
                                *r = r.saturating_sub(qty);
                            }
                            remove = true;
                        } else {
                            let arrived = world
                                .vehicles
                                .get(v)
                                .map(|ve| ve.it.has_ended(0.0))
                                .unwrap_or(false);
                            if arrived {
                                let m = self.m(kind);
                                *m.capital.entry(seller).or_default() -= qty as i32;
                                if let Some(r) = m.reserved.get_mut(&seller) {
                                    *r = r.saturating_sub(qty);
                                }
                                let d = &mut self.dispatches[i];
                                d.state = DispatchState::Loading;
                                d.ticks_left = DISPATCH_DWELL_TICKS;
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
                                // a teleport, so nothing is re-credited.
                                log::warn!(
                                    "dispatch lost {:?} {:?} ({:?} -> {:?}): truck vanished while loading",
                                    qty, kind, seller, buyer
                                );
                                dispatcher.free(DispatchID::SmallTruck(v));
                                remove = true;
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
                                }
                                // No route found: stay in Loading (ticks_left at 0)
                                // and retry next tick.
                            } else {
                                // Wedge (a): buyer's building was demolished.
                                // The goods are already debited from the
                                // seller and physically on the truck; drive
                                // them back instead of teleport-refunding.
                                if let Some(seller_pos) = door_pos(seller, map, binfos) {
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
                                        self.dispatches[i].state = DispatchState::Returning;
                                    } else if return_route_retries + 1 >= MAX_RETURN_ROUTE_RETRIES {
                                        // No route back after repeated tries
                                        // (e.g. the road home was severed):
                                        // treat as an honest physical loss
                                        // rather than retry forever.
                                        log::warn!(
                                            "dispatch dropped {:?} {:?}: no route back to \
                                             seller after {} attempts",
                                            qty,
                                            kind,
                                            MAX_RETURN_ROUTE_RETRIES
                                        );
                                        dispatcher.free(DispatchID::SmallTruck(v));
                                        remove = true;
                                    } else {
                                        self.dispatches[i].return_route_retries =
                                            return_route_retries + 1;
                                    }
                                } else {
                                    // Seller is also gone: free the truck and
                                    // drop the goods (already debited from a
                                    // seller that no longer exists — nothing
                                    // left to return them to).
                                    log::warn!(
                                        "dispatch dropped {:?} {:?}: both buyer and seller \
                                         buildings are gone",
                                        qty,
                                        kind
                                    );
                                    dispatcher.free(DispatchID::SmallTruck(v));
                                    remove = true;
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
                        // the Loading case above.
                        log::warn!(
                            "dispatch lost {:?} {:?} ({:?} -> {:?}): truck vanished in transit",
                            qty,
                            kind,
                            seller,
                            buyer
                        );
                        if let Some(v) = truck {
                            dispatcher.free(DispatchID::SmallTruck(v));
                        }
                        remove = true;
                    } else {
                        let arrived = truck
                            .and_then(|v| world.vehicles.get(v))
                            .map(|ve| ve.it.has_ended(0.0))
                            .unwrap_or(false);
                        if arrived {
                            let m = self.m(kind);
                            *m.capital.entry(buyer).or_default() += qty as i32;
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
                        // debited from the seller and never re-credited.
                        log::warn!(
                            "dispatch lost {:?} {:?} ({:?} -> {:?}): truck vanished while returning",
                            qty,
                            kind,
                            seller,
                            buyer
                        );
                        if let Some(v) = truck {
                            dispatcher.free(DispatchID::SmallTruck(v));
                        }
                        remove = true;
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

            let price_workers = recipe.duration.minutes()
                * company.n_workers as f64
                * WORKER_CONSUMPTION_PER_MINUTE;

            dbg!(price_consumption, price_workers, qty);

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
}
