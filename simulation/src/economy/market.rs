use std::collections::btree_map::Entry;
use std::collections::BTreeMap;

use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};

use geom::Vec2;
use prototypes::{prototypes_iter, GoodsCompanyID, GoodsCompanyPrototype, ItemPrototype, Money};

use crate::economy::{ItemID, WORKER_CONSUMPTION_PER_MINUTE};
use crate::map::BuildingID;
use crate::map_dynamic::BuildingInfos;
use crate::SoulID;

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

/// How many ticks a dispatch spends travelling to the source, and again travelling
/// to the destination. ponytail: fixed proxy for travel time, not real vehicle
/// pathing/speed; upgrade to real trip duration once goods dispatches ride actual
/// vehicles.
const DISPATCH_TRAVEL_TICKS: u32 = 3;

/// The stage of a goods dispatch. A trade match doesn't move stock immediately:
/// the quantity is only debited from the seller when the dispatch transitions into
/// `Loading`, and only credited to the buyer when it transitions into `Unloading`.
/// Between those two transitions, the quantity is held by the dispatch itself and
/// counted in neither soul's capital.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DispatchState {
    ToSource,
    Loading,
    ToDestination,
    Unloading,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Dispatch {
    pub buyer: SoulID,
    pub seller: SoulID,
    pub kind: ItemID,
    pub qty: u32,
    pub state: DispatchState,
    ticks_left: u32,
}

impl Default for Market {
    fn default() -> Self {
        let prices = calculate_prices(1.25);
        Self {
            markets: prototypes_iter::<ItemPrototype>()
                .map(|v| (v.id, SingleMarket::new(prices[&v.id], v.optout_exttrade)))
                .collect(),
            dispatches: Default::default(),
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
    pub fn remove(&mut self, soul: SoulID) {
        for market in self.markets.values_mut() {
            market.sell_orders.remove(&soul);
            market.buy_orders.remove(&soul);
            market.capital.remove(&soul);
        }
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

                    // The goods stay physically at the seller (neither capital bucket
                    // moves yet) until a dispatch actually loads/unloads them; only
                    // reserve them so they can't be sold again.
                    *reserved.entry(trade.seller.0).or_default() += trade.qty as u32;

                    Some(trade)
                }));

            for &trade in &self.all_trades[dispatch_start..] {
                self.dispatches.push(Dispatch {
                    buyer: trade.buyer.0,
                    seller: trade.seller.0,
                    kind,
                    qty: trade.qty as u32,
                    state: DispatchState::ToSource,
                    ticks_left: DISPATCH_TRAVEL_TICKS,
                });
            }

            // External trading
            if !*optout_exttrade {
                // All buyers can fullfil since they can buy externally
                let btaken = std::mem::take(buy_orders);
                self.all_trades.reserve(btaken.len());
                for (buyer, order) in btaken {
                    let qty_buy = order.qty as i32;
                    *capital.entry(buyer).or_default() += qty_buy;

                    let Some(ext) = find_external(order.pos) else {
                        continue;
                    };

                    self.all_trades.push(Trade {
                        buyer: TradeTarget(buyer),
                        seller: TradeTarget(ext),
                        qty: qty_buy,
                        kind,
                        money_delta: -(*ext_value * qty_buy as i64), // we buy from external so we pay
                    });
                }

                // Seller surplus goes to external trading
                for (&seller, order) in sell_orders.iter_mut() {
                    let qty_sell = order.qty as i32 - order.stock as i32;
                    if qty_sell <= 0 {
                        continue;
                    }
                    let cap = capital.entry(seller).or_default();
                    if *cap < qty_sell {
                        log::warn!("{:?} is selling more than it has: {:?}", &seller, qty_sell);
                        continue;
                    }
                    *cap -= qty_sell;
                    order.qty -= qty_sell as u32;

                    let Some(ext) = find_external(order.pos) else {
                        continue;
                    };

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
    /// ToSource -> Loading -> ToDestination -> Unloading. The seller's capital is
    /// debited exactly on the transition into `Loading`, and the buyer's capital is
    /// credited exactly on the transition into `Unloading`; must be called once per
    /// tick for dispatches to ever complete.
    pub fn advance_dispatches(&mut self) {
        let mut i = 0;
        while i < self.dispatches.len() {
            let (seller, buyer, kind, qty, state, ticks_left) = {
                let d = &self.dispatches[i];
                (d.seller, d.buyer, d.kind, d.qty, d.state, d.ticks_left)
            };

            let mut remove = false;
            match state {
                DispatchState::ToSource => {
                    if ticks_left == 0 {
                        self.dispatches[i].state = DispatchState::Loading;
                    } else {
                        self.dispatches[i].ticks_left = ticks_left - 1;
                    }
                }
                DispatchState::Loading => {
                    let m = self.m(kind);
                    *m.capital.entry(seller).or_default() -= qty as i32;
                    if let Some(r) = m.reserved.get_mut(&seller) {
                        *r = r.saturating_sub(qty);
                    }
                    let d = &mut self.dispatches[i];
                    d.state = DispatchState::ToDestination;
                    d.ticks_left = DISPATCH_TRAVEL_TICKS;
                }
                DispatchState::ToDestination => {
                    if ticks_left == 0 {
                        self.dispatches[i].state = DispatchState::Unloading;
                    } else {
                        self.dispatches[i].ticks_left = ticks_left - 1;
                    }
                }
                DispatchState::Unloading => {
                    let m = self.m(kind);
                    *m.capital.entry(buyer).or_default() += qty as i32;
                    remove = true;
                }
            }

            if remove {
                self.dispatches.swap_remove(i);
            } else {
                i += 1;
            }
        }
    }
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

    use super::{DispatchState, Market, DISPATCH_TRAVEL_TICKS};

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

    /// SCENARIO: a matched trade must not move stock on its own. Stock only
    /// leaves the seller when the dispatch enters `Loading`, and only reaches the
    /// buyer when it enters `Unloading`. Proves the truck is load-bearing, not
    /// decorative: revert the match-time reservation-only change in `make_trades`
    /// back to an immediate capital transfer and this test fails at the
    /// "match must not move seller/buyer stock" assertions.
    #[test]
    fn test_dispatch_gates_stock_not_match() {
        let seller = SoulID::GoodsCompany(mk_ent((1 << 32) | 10));
        let buyer = SoulID::GoodsCompany(mk_ent((1 << 32) | 11));
        let freight = SoulID::FreightStation(FreightStationID::from(slotmapd::KeyData::from_ffi(
            (1 << 32) | 12,
        )));

        test_prototypes(
            r#"
        data:extend {
          { type = "item", name = "cereal", label = "Cereal" }
        }
        "#,
        );

        let mut m = Market::default();
        let cereal = ItemID::new("cereal");

        m.produce(seller, cereal, 5);
        m.sell(seller, Vec2::X, cereal, 5, 0);
        m.buy(buyer, Vec2::ZERO, cereal, 5);

        let trades = m.make_trades(|_| Some(freight));
        assert_eq!(trades.len(), 1);

        // The match happened (trade recorded, order book updated) but stock must
        // still be sitting with the seller, held by the dispatch.
        assert_eq!(
            m.capital(seller, cereal),
            5,
            "match must not move seller stock"
        );
        assert_eq!(
            m.capital(buyer, cereal),
            0,
            "match must not move buyer stock"
        );
        assert_eq!(m.dispatches().len(), 1);
        assert_eq!(m.dispatches()[0].state, DispatchState::ToSource);

        // Burn down ToSource, then the tick that transitions into Loading.
        let calls_to_debit = DISPATCH_TRAVEL_TICKS + 2;
        for _ in 0..calls_to_debit - 1 {
            m.advance_dispatches();
            assert_eq!(m.capital(seller, cereal), 5, "seller debited too early");
            assert_eq!(m.capital(buyer, cereal), 0, "buyer credited too early");
        }
        m.advance_dispatches();
        assert_eq!(
            m.capital(seller, cereal),
            0,
            "seller must be debited exactly on entering Loading"
        );
        assert_eq!(m.capital(buyer, cereal), 0, "buyer still must not have it");

        // Burn down ToDestination, then the tick that transitions into Unloading.
        let calls_to_credit = 2 * (DISPATCH_TRAVEL_TICKS + 2);
        for _ in calls_to_debit..calls_to_credit - 1 {
            m.advance_dispatches();
            assert_eq!(m.capital(buyer, cereal), 0, "buyer credited too early");
        }
        m.advance_dispatches();
        assert_eq!(
            m.capital(buyer, cereal),
            5,
            "buyer must be credited exactly on entering Unloading"
        );
        assert!(m.dispatches().is_empty());
    }

    /// SCENARIO: two otherwise-identical companies buy the same item, one honest
    /// (requests exactly what its recipe consumes) and one inflated (requests more
    /// than it consumes). Only the inflated one should accumulate surplus stock
    /// over several cycles.
    #[test]
    fn test_inflated_request_hoards_honest_does_not() {
        let seller = SoulID::GoodsCompany(mk_ent((1 << 32) | 20));
        let honest = SoulID::GoodsCompany(mk_ent((1 << 32) | 21));
        let inflated = SoulID::GoodsCompany(mk_ent((1 << 32) | 22));
        let freight = SoulID::FreightStation(FreightStationID::from(slotmapd::KeyData::from_ffi(
            (1 << 32) | 23,
        )));

        test_prototypes(
            r#"
        data:extend {
          { type = "item", name = "cereal", label = "Cereal" }
        }
        "#,
        );

        let mut m = Market::default();
        let cereal = ItemID::new("cereal");

        const TRUE_CONSUMPTION: u32 = 2;
        const INFLATED_REQUEST: u32 = 5;

        m.produce(seller, cereal, 1000);
        m.sell(seller, Vec2::X, cereal, 1000, 0);

        m.set_requested(honest, cereal, TRUE_CONSUMPTION);
        m.set_requested(inflated, cereal, INFLATED_REQUEST);

        let calls_to_credit = 2 * (DISPATCH_TRAVEL_TICKS + 2);

        for _ in 0..3 {
            // Both consume exactly what their recipe needs, same as `recipe_act`.
            let honest_consumed = m.capital(honest, cereal).min(TRUE_CONSUMPTION as i32);
            m.produce(honest, cereal, -honest_consumed);
            let inflated_consumed = m.capital(inflated, cereal).min(TRUE_CONSUMPTION as i32);
            m.produce(inflated, cereal, -inflated_consumed);

            // Both re-request up to their reported (possibly inflated) quantity,
            // same as `recipe_init`/`recipe_act`.
            let hq = m.requested(honest, cereal).unwrap();
            m.buy_until(honest, Vec2::ZERO, cereal, hq);
            let iq = m.requested(inflated, cereal).unwrap();
            m.buy_until(inflated, vec2(0.0, 1.0), cereal, iq);

            m.make_trades(|_| Some(freight));
            for _ in 0..calls_to_credit {
                m.advance_dispatches();
            }
        }

        assert!(
            m.capital(honest, cereal) <= TRUE_CONSUMPTION as i32,
            "honest company must not hoard beyond what it truly consumes: {}",
            m.capital(honest, cereal)
        );
        assert!(
            m.capital(inflated, cereal) > TRUE_CONSUMPTION as i32,
            "inflated company must accumulate surplus above true consumption: {}",
            m.capital(inflated, cereal)
        );
    }
}
