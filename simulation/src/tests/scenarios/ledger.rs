//! Conservation guard: nothing in the ledger should create or destroy units
//! of an item outside of `produce()` (source: production/extraction, or a
//! negative delta for consumption) and ext-trade (sink: export, source:
//! import). Everything else — a domestic match/dispatch, a soul being
//! removed mid-flight — must be a pure transfer.
//!
//! sov-ledger-invariant-test-8gg. Reproduces three verified double-spend/leak
//! sequences from the ledger audit; each must FAIL before the Stage 2 fixes
//! (sov-ledger-exttrade-cbh, sov-ledger-remove-3v7, sov-reserved-jobs-azr)
//! land, and pass after.

use super::*;

use super::hoarding::{
    build_company_at, drain_dispatches, mk_soul, remove_soul, setup_seller_buyer,
};
use super::inflation::remove_default_freight_station;
use crate::economy::{DispatchState, Market};
use crate::map_dynamic::BuildingInfos;
use crate::transportation::{spawn_parked_vehicle, VehicleKind};
use crate::world::HumanID;
use crate::SoulID;
use prototypes::{GoodsCompanyID, ItemID};

fn mk_human(id: u64) -> SoulID {
    SoulID::Human(HumanID::from(slotmapd::KeyData::from_ffi(id)))
}

/// Σ(capital across all souls) + Σ(qty held by dispatches that have already
/// debited their seller but not yet credited their buyer — `Loading`,
/// `ToDestination` and `Returning` (goods driven back to the seller after a
/// demolished buyer, not yet re-credited); see `Market::advance_dispatches`)
/// for one item. A pure domestic transfer moves value from a soul's capital
/// into this in-flight bucket and back out without changing the total, so it
/// must stay constant across any tick in which no `produce()` and no
/// ext-trade ran.
fn total_qty(m: &Market, item: ItemID) -> i64 {
    let cap: i64 = m
        .inner()
        .get(&item)
        .map(|sm| sm.capital_map().values().map(|&c| c as i64).sum())
        .unwrap_or(0);
    let in_flight: i64 = m
        .dispatches()
        .iter()
        .filter(|d| {
            d.kind == item
                && matches!(
                    d.state,
                    DispatchState::Loading
                        | DispatchState::ToDestination
                        | DispatchState::Returning
                )
        })
        .map(|d| d.qty as i64)
        .sum();
    cap + in_flight
}

#[test]
fn sov_e1q_export_without_external_endpoint() {
    let mut ctx = TestCtx::new();
    let (seller, _, seller_pos, _) = setup_seller_buyer(&mut ctx, 120.0);
    let cereal = ItemID::new("cereal");

    let mut m = Market::default();
    m.produce(seller, cereal, 10);
    m.sell(seller, seller_pos.xy(), cereal, 10, 0);

    let trade_count = m.make_trades(|_| None).len();

    assert_eq!(m.capital(seller, cereal), 10);
    assert_eq!(m.inner()[&cereal].sell_order(seller).unwrap().qty, 10);
    assert_eq!(trade_count, 0);
}

/// sov-dii: the seller-surplus export path must not debit the seller's
/// capital when `find_external` finds nowhere to export to. At the fork
/// commit the deduction ran *before* the endpoint lookup, so in a city with
/// no freight station every sell order at stock 0 silently destroyed its own
/// quantity, tick after tick, with no trade record. Unlike
/// `sov_e1q_export_without_external_endpoint`, which stubs the lookup with
/// `make_trades(|_| None)`, this drives the real `market_update` closure over
/// a world whose only `RailFreightStation` has been demolished.
#[test]
fn sov_dii_no_freight_station_holds_seller_capital() {
    let mut ctx = TestCtx::new();
    remove_default_freight_station(&mut ctx);

    // A synthetic soul, so nothing but the ext-trade path can touch its
    // capital across the ticks below.
    let seller = mk_soul((1 << 32) | 77);
    let cereal = ItemID::new("cereal");

    {
        let mut m = ctx.g.write::<Market>();
        m.produce(seller, cereal, 10);
        m.sell(seller, Vec2::new(50.0, 20.0), cereal, 10, 0);
    }

    ctx.advance_ticks(5);

    let m = ctx.g.read::<Market>();
    assert_eq!(
        m.capital(seller, cereal), 10,
        "no freight station exists, so no export can be recorded; the seller's \
         quantity must survive untouched"
    );
    assert_eq!(
        m.inner()[&cereal].sell_order(seller).unwrap().qty, 10,
        "the sell order must survive too — nothing left the city"
    );
    assert_eq!(total_qty(&m, cereal), 10);
}

/// sov-ledger-exttrade-cbh: a domestic match reserves stock (`reserved`) for
/// a buyer without touching `capital`. Production immediately after calls
/// `sell_all`, which overwrites the sell order with the new full stock and
/// forgets the reservation (`market.rs:191`). The next ext-trade pass then
/// computes its surplus off the un-reserved order and oversells: the
/// domestic delivery plus the export exceed what was ever produced.
///
/// Uses the seller/buyer/"cereal" harness from `hoarding.rs` (proven not to
/// interfere with manually-driven capital) standing in for the audit's
/// flour-factory numbers; the arithmetic is identical (59 capital, 50 stock,
/// a 55-unit domestic match, a 10-unit production, a 19-unit oversold
/// export).
#[test]
fn scenario_ledger_exttrade_double_spend() {
    let mut ctx = TestCtx::new();
    let (seller, buyer, seller_pos, buyer_pos) = setup_seller_buyer(&mut ctx, 120.0);
    spawn_parked_vehicle(&mut ctx.g, VehicleKind::Truck, seller_pos).expect("truck must spawn");

    let cereal = ItemID::new("cereal");
    let ext = mk_soul((1 << 32) | 99); // stands in for `find_external`'s pick

    {
        let mut m = ctx.g.write::<Market>();
        m.produce(seller, cereal, 59);
        m.sell(seller, seller_pos.xy(), cereal, 59, 50);
        m.buy(buyer, buyer_pos.xy(), cereal, 55);

        // Pass 1: domestic match reserves 55 (capital untouched); the
        // remaining-order ext-trade surplus (4 - stock 50) is negative, skipped.
        // (`trades` also carries the harness's own background activity —
        // the house's auto-spawned resident taking the bakery's job opening,
        // the bakery importing its own flour — filter down to cereal.)
        let trades = m.make_trades(|_| Some(ext));
        let cereal_trades: Vec<_> = trades.iter().filter(|t| t.kind == cereal).collect();
        assert_eq!(
            cereal_trades.len(),
            1,
            "only the domestic match should trade cereal this pass"
        );
        assert_eq!(cereal_trades[0].qty, 55);
        assert_eq!(
            total_qty(&m, cereal),
            59,
            "a domestic match alone must not change the total"
        );
    }

    // Production (+10) is a declared source, so the total is allowed to grow
    // here. `sell_all` then overwrites the sell order with the full new
    // stock, forgetting the still-outstanding 55-unit reservation.
    let produced_total = {
        let mut m = ctx.g.write::<Market>();
        let after = m.produce(seller, cereal, 10);
        m.sell_all(seller, seller_pos.xy(), cereal, 50);
        after as i64
    };
    assert_eq!(produced_total, 69);

    // Pass 2: ext-trade surplus, buggy, used to compute off the un-reserved
    // 69 and oversell by 19 (should have been capped at 69 - 55 = 14 free
    // units, which is less than the 50-unit stock threshold, so a correct
    // market should export nothing here at all).
    let exported: i64 = {
        let mut m = ctx.g.write::<Market>();
        let trades = m.make_trades(|_| Some(ext));
        trades
            .iter()
            .filter(|t| t.kind == cereal)
            .map(|t| t.qty as i64)
            .sum()
    };

    assert!(
        drain_dispatches(&mut ctx, 4000),
        "domestic dispatch never completed"
    );

    let m = ctx.g.read::<Market>();
    let delivered = m.capital(buyer, cereal) as i64;
    assert_eq!(delivered, 55, "buyer must receive the full domestic match");
    assert!(
        m.capital(seller, cereal) >= 0,
        "seller capital must never go negative (would mean goods delivered/exported \
         that were never produced): {}",
        m.capital(seller, cereal)
    );
    assert!(
        delivered + exported <= produced_total,
        "conservation violated: {delivered} delivered + {exported} exported > {produced_total} \
         ever produced (seller capital ended at {})",
        m.capital(seller, cereal),
    );
}

/// sov-ledger-remove-3v7: `Market::remove` clears a soul's `sell_orders`,
/// `buy_orders` and `capital`, but not `reserved`/`requested`/`dispatches`.
/// A dispatch still naming the removed soul as seller survives; when its
/// truck eventually arrives, `Loading`'s debit (`market.rs:503`) resurrects
/// a negative capital row for a soul that no longer exists, and the buyer
/// still gets credited for goods nobody delivered.
#[test]
fn scenario_ledger_remove_leak() {
    let mut ctx = TestCtx::new();
    let (seller, buyer, seller_pos, buyer_pos) = setup_seller_buyer(&mut ctx, 120.0);
    spawn_parked_vehicle(&mut ctx.g, VehicleKind::Truck, seller_pos).expect("truck must spawn");

    let cereal = ItemID::new("cereal");
    {
        let mut m = ctx.g.write::<Market>();
        m.produce(seller, cereal, 10);
        m.sell(seller, seller_pos.xy(), cereal, 10, 0);
        m.buy(buyer, buyer_pos.xy(), cereal, 10);
    }

    ctx.tick(); // domestic match: reserved[seller] = 10, dispatch in ToSource

    {
        let m = ctx.g.read::<Market>();
        assert_eq!(m.dispatches().len(), 1);
        assert_eq!(m.dispatches()[0].state, DispatchState::ToSource);
    }

    remove_soul(&mut ctx, seller); // seller "demolished" mid-flight

    assert!(drain_dispatches(&mut ctx, 4000), "dispatch never completed");

    let m = ctx.g.read::<Market>();
    assert!(
        !m.inner()[&cereal].capital_map().contains_key(&seller),
        "a removed soul must never reappear in the ledger, phantom row: {:?}",
        m.inner()[&cereal].capital_map().get(&seller)
    );
    assert_eq!(
        m.capital(buyer, cereal),
        0,
        "buyer must not be credited for goods nobody delivered"
    );
}

/// sov-reserved-jobs-azr: every domestic match reserves stock
/// (`market.rs:334-336`), but `job-opening` matches never get a dispatch
/// (`market.rs:363`) — the dispatch is the only thing that would later
/// debit capital or release the reservation. So a company's job-opening
/// capital never falls as its openings get filled.
#[test]
fn scenario_ledger_job_opening_reserve_leak() {
    let mut ctx = TestCtx::new();
    let bakery = prototypes::GoodsCompanyID::new("bakery").prototype();
    let b = build_company_at(&mut ctx, bakery, geom::Vec2::new(30.0, 20.0));
    ctx.tick(); // company_soul spawns on the next tick

    let seller = ctx
        .g
        .read::<crate::map_dynamic::BuildingInfos>()
        .owner(b)
        .unwrap();
    let job_opening = ItemID::new("job-opening");
    let pos = ctx.g.map().buildings.get(b).unwrap().door_pos.xy();

    let baseline = ctx.g.read::<Market>().capital(seller, job_opening);
    assert_eq!(
        baseline, 3,
        "bakery must post one job opening per worker slot (n_workers = 3)"
    );

    {
        let mut m = ctx.g.write::<Market>();
        for i in 0..baseline as u64 {
            m.buy(mk_human(i), pos, job_opening, 1);
        }
        let trades = m.make_trades(|_| None);
        assert_eq!(
            trades.len(),
            baseline as usize,
            "every opening should match"
        );
    }

    let m = ctx.g.read::<Market>();
    assert_eq!(
        total_qty(&m, job_opening),
        0,
        "job-opening capital must fall to 0 once every slot is filled, not stay reserved forever"
    );
}

/// Builds a real cereal-farm seller and a real flour-factory buyer (both
/// actual `GoodsCompany` entities, not `hoarding::mk_soul` stubs) on a road,
/// so the buyer can be killed through the same `SimDrop`/`Market::remove`
/// path a demolished building actually takes. See sov-dispatch-wedge-ab4:
/// the earlier gate pass only ever removed a fabricated buyer SoulID, which
/// is why `Market::remove`'s blind `retain` on a dead BUYER went unnoticed.
fn setup_real_seller_buyer(ctx: &mut TestCtx) -> (SoulID, SoulID, Vec3, Vec3) {
    ctx.build_roads(&[Vec3::new(0.0, 0.0, 0.0), Vec3::new(200.0, 0.0, 0.0)]);
    let seller_proto = GoodsCompanyID::new("cereal-farm").prototype();
    let buyer_proto = GoodsCompanyID::new("flour-factory").prototype();
    let seller_b = build_company_at(ctx, seller_proto, Vec2::new(30.0, 20.0));
    let buyer_b = build_company_at(ctx, buyer_proto, Vec2::new(150.0, 20.0));
    ctx.tick(); // company souls spawn on the next tick

    let seller = ctx.g.read::<BuildingInfos>().owner(seller_b).unwrap();
    let buyer = ctx.g.read::<BuildingInfos>().owner(buyer_b).unwrap();
    let seller_pos = ctx.g.map().buildings.get(seller_b).unwrap().door_pos;
    let buyer_pos = ctx.g.map().buildings.get(buyer_b).unwrap().door_pos;
    (seller, buyer, seller_pos, buyer_pos)
}

/// A dead BUYER at `ToSource` (nothing debited yet): the fix must release
/// `reserved[seller]`, not strand it forever -- which, combined with
/// `recipe_should_produce` reading capital-reserved, would throttle the
/// seller's production forever too.
#[test]
fn scenario_dead_buyer_tosource_releases_reservation() {
    let mut ctx = TestCtx::new();
    let (seller, buyer, seller_pos, buyer_pos) = setup_real_seller_buyer(&mut ctx);
    spawn_parked_vehicle(&mut ctx.g, VehicleKind::Truck, seller_pos).expect("truck must spawn");

    let cereal = ItemID::new("cereal");
    {
        let mut m = ctx.g.write::<Market>();
        m.produce(seller, cereal, 10);
        m.sell(seller, seller_pos.xy(), cereal, 10, 0);
        m.buy(buyer, buyer_pos.xy(), cereal, 10);
    }

    ctx.tick(); // domestic match: reserved[seller] = 10, dispatch in ToSource

    {
        let m = ctx.g.read::<Market>();
        assert_eq!(m.dispatches().len(), 1);
        assert_eq!(m.dispatches()[0].state, DispatchState::ToSource);
    }

    remove_soul(&mut ctx, buyer); // buyer "demolished" mid-flight, seller survives

    let m = ctx.g.read::<Market>();
    assert!(
        m.dispatches().is_empty(),
        "the dead buyer's dispatch must not linger"
    );
    assert_eq!(
        m.reserved(seller, cereal),
        0,
        "reserved[seller] must be released, not stranded forever on a live seller"
    );
    assert_eq!(
        m.capital(seller, cereal),
        10,
        "nothing was ever debited (truck never arrived), goods stay with the seller"
    );
}

/// A dead SELLER mid-dispatch must give its truck back to the `Dispatcher`.
///
/// sov-dispatch-wedge-ab4 round 4. `Market::remove` ends with an
/// unconditional `dispatches.retain(|d| d.seller != soul)`; unlike the
/// dead-BUYER half directly above it, that path never calls
/// `dispatcher.free(DispatchID::SmallTruck(v))`, so a truck assigned to the
/// dropped dispatch stays in `DispatchOne::reserved_by` forever.
///
/// Why the assertion is shaped this way: a leaked reservation is invisible
/// to a query for a *different* truck (a new candidate is a new id, see
/// `retail::scenario_dead_truck_tosource_cancels_without_leak`), and it does
/// not break conservation either -- the dead seller's capital/reserved rows
/// are wiped wholesale, so `total_qty` is unaffected and the ledger gate
/// cannot see it. It is only observable as the leaked truck itself never
/// being reservable again: `DispatchOne::query` skips any id in
/// `reserved_by` (dispatch.rs:271) and only `free` clears it. So this asks
/// the `Dispatcher` directly, which is the narrowest true statement of the
/// defect -- after the seller dies, every truck in the city must be
/// reservable again.
#[test]
fn scenario_dead_seller_frees_its_truck() {
    let mut ctx = TestCtx::new();
    let (seller, buyer, seller_pos, buyer_pos) = setup_real_seller_buyer(&mut ctx);

    let cereal = ItemID::new("cereal");
    {
        let mut m = ctx.g.write::<Market>();
        m.produce(seller, cereal, 10);
        m.sell(seller, seller_pos.xy(), cereal, 10, 0);
        m.buy(buyer, buyer_pos.xy(), cereal, 10);
    }

    // Drive until the truck has physically arrived and loaded. `Dispatch`'s
    // `truck` field is private (the accessor was deliberately dropped), but
    // reaching `Loading` is only possible once a truck was reserved AND drove
    // to the seller, so it proves a reservation exists to be leaked --
    // without it the test would pass vacuously.
    let mut ticks = 0;
    loop {
        ctx.tick();
        ticks += 1;
        let loaded = ctx
            .g
            .read::<Market>()
            .dispatches()
            .first()
            .is_some_and(|d| {
                matches!(
                    d.state,
                    DispatchState::Loading | DispatchState::ToDestination
                )
            });
        if loaded {
            break;
        }
        assert!(ticks < 4000, "dispatch never reached Loading");
    }

    // Every truck that exists, and how many the dispatcher can hand out while
    // one is reserved by the live dispatch.
    let all_trucks: Vec<_> = ctx
        .g
        .world()
        .vehicles
        .iter()
        .filter(|(_, v)| matches!(v.vehicle.kind, VehicleKind::Truck))
        .map(|(id, _)| id)
        .collect();
    assert!(!all_trucks.is_empty(), "the city must have trucks");

    remove_soul(&mut ctx, seller); // seller's building demolished mid-flight

    assert!(
        ctx.g.read::<Market>().dispatches().is_empty(),
        "the dead seller's dispatch must not linger"
    );

    // With the dispatch gone, nothing holds a reservation any more, so the
    // dispatcher must be able to hand out every truck. A leaked `reserved_by`
    // entry makes exactly one of them permanently unqueryable.
    ctx.tick(); // let dispatch_system re-register positions
    let (world, res) = ctx.g.world_res();
    let map = res.read::<crate::map::Map>();
    let mut dispatcher = res.write::<crate::map_dynamic::Dispatcher>();
    dispatcher.update(&map, world);

    let mut handed_out = 0;
    while dispatcher
        .query(
            &map,
            crate::map_dynamic::DispatchKind::SmallTruck,
            crate::map_dynamic::DispatchQueryTarget::Pos(seller_pos),
        )
        .is_some()
    {
        handed_out += 1;
        assert!(handed_out <= all_trucks.len(), "query returned duplicates");
    }

    assert_eq!(
        handed_out,
        all_trucks.len(),
        "every truck must be reservable once the dead seller's dispatch is \
         dropped; a truck still in `reserved_by` is leaked out of the pool \
         permanently, and no later dispatch can ever use it"
    );
}

/// The real demolition chain, end to end: `map.remove_building` ->
/// `goods_company` sees its building gone -> `cbuf.kill(me)` ->
/// `CompanyEnt::sim_drop` -> `Market::remove`.
///
/// sov-dispatch-wedge-ab4 round 4, asked for by both gates: every other
/// dead-buyer test calls `Market::remove` directly via `remove_soul`, so the
/// wiring that makes it fire on the ordinary gameplay path was confirmed by
/// reading `init.rs`/`scheduler.rs` but never by execution. This drives the
/// player-visible action (demolish the buyer's building) and asserts the
/// shipment is conserved.
#[test]
fn scenario_demolish_buyer_building_end_to_end_conserves() {
    let mut ctx = TestCtx::new();
    ctx.build_roads(&[Vec3::new(0.0, 0.0, 0.0), Vec3::new(200.0, 0.0, 0.0)]);
    let seller_proto = GoodsCompanyID::new("cereal-farm").prototype();
    let buyer_proto = GoodsCompanyID::new("flour-factory").prototype();
    let seller_b = build_company_at(&mut ctx, seller_proto, Vec2::new(30.0, 20.0));
    let buyer_b = build_company_at(&mut ctx, buyer_proto, Vec2::new(150.0, 20.0));
    ctx.tick();

    let seller = ctx.g.read::<BuildingInfos>().owner(seller_b).unwrap();
    let buyer = ctx.g.read::<BuildingInfos>().owner(buyer_b).unwrap();
    let seller_pos = ctx.g.map().buildings.get(seller_b).unwrap().door_pos;
    let buyer_pos = ctx.g.map().buildings.get(buyer_b).unwrap().door_pos;
    spawn_parked_vehicle(&mut ctx.g, VehicleKind::Truck, seller_pos).expect("truck must spawn");

    let cereal = ItemID::new("cereal");
    {
        let mut m = ctx.g.write::<Market>();
        m.produce(seller, cereal, 10);
        m.sell(seller, seller_pos.xy(), cereal, 10, 0);
        m.buy(buyer, buyer_pos.xy(), cereal, 10);
    }

    // Drive until the goods are physically on the truck (seller debited).
    let mut ticks = 0;
    loop {
        ctx.tick();
        ticks += 1;
        let loaded = ctx
            .g
            .read::<Market>()
            .dispatches()
            .first()
            .is_some_and(|d| {
                matches!(
                    d.state,
                    DispatchState::Loading | DispatchState::ToDestination
                )
            });
        if loaded {
            break;
        }
        assert!(ticks < 4000, "dispatch never reached Loading");
    }
    let before = total_qty(&ctx.g.read::<Market>(), cereal);
    assert_eq!(before, 10, "10 units exist, in flight");
    assert_eq!(
        ctx.g.read::<Market>().capital(seller, cereal),
        0,
        "seller is already debited once the truck has loaded"
    );

    // The player demolishes the buyer's building. Nothing else: the kill and
    // the Market::remove must follow from this alone.
    ctx.g.map_mut().remove_building(buyer_b);
    ctx.tick(); // company_system sees the missing building -> cbuf.kill -> sim_drop

    assert!(
        ctx.g.read::<BuildingInfos>().owner(buyer_b).is_none()
            || ctx.g.read::<Market>().capital(buyer, cereal) == 0,
        "the demolished buyer must not be holding goods"
    );

    assert!(
        drain_dispatches(&mut ctx, 8000),
        "the dispatch never resolved after the buyer's building was demolished"
    );

    let m = ctx.g.read::<Market>();
    assert_eq!(
        total_qty(&m, cereal),
        10,
        "demolishing the buyer's building must not create or destroy units -- \
         the shipment is driven back to the seller, never silently dropped"
    );
    assert_eq!(
        m.capital(seller, cereal),
        10,
        "goods already debited must be physically returned and re-credited"
    );
    assert_eq!(
        m.capital(buyer, cereal),
        0,
        "a demolished buyer must never be credited"
    );
}

/// A dead BUYER after the truck has already loaded (seller debited, goods
/// physically on the truck): the fix must hand the goods to the same
/// physical return-to-seller path a live buyer-demolition takes, not
/// destroy them. Conservation must hold at every tick along the way.
#[test]
fn scenario_dead_buyer_loading_goods_returned() {
    let mut ctx = TestCtx::new();
    let (seller, buyer, seller_pos, buyer_pos) = setup_real_seller_buyer(&mut ctx);
    spawn_parked_vehicle(&mut ctx.g, VehicleKind::Truck, seller_pos).expect("truck must spawn");

    let cereal = ItemID::new("cereal");
    {
        let mut m = ctx.g.write::<Market>();
        m.produce(seller, cereal, 10);
        m.sell(seller, seller_pos.xy(), cereal, 10, 0);
        m.buy(buyer, buyer_pos.xy(), cereal, 10);
    }

    // Drive until the truck has loaded (seller debited, goods on the truck).
    let mut ticks = 0;
    loop {
        ctx.tick();
        ticks += 1;
        let m = ctx.g.read::<Market>();
        if !m.dispatches().is_empty()
            && matches!(
                m.dispatches()[0].state,
                DispatchState::Loading | DispatchState::ToDestination
            )
        {
            break;
        }
        assert!(ticks < 4000, "dispatch never reached Loading");
    }
    assert_eq!(
        ctx.g.read::<Market>().capital(seller, cereal),
        0,
        "seller must already be debited once the truck has loaded"
    );

    remove_soul(&mut ctx, buyer); // buyer "demolished" mid-flight, seller survives

    {
        let m = ctx.g.read::<Market>();
        assert_eq!(
            m.dispatches()[0].state,
            DispatchState::Returning,
            "the dead buyer's dispatch must be handed to the physical return path"
        );
        assert_eq!(
            total_qty(&m, cereal),
            10,
            "goods driven back on a Returning dispatch are still in flight, not yet \
             back in the seller's capital -- the conservation accountant must count \
             them or it under-reports the total while they're on the road home"
        );
    }

    assert!(
        drain_dispatches(&mut ctx, 6000),
        "the returning dispatch never resolved"
    );

    let m = ctx.g.read::<Market>();
    assert_eq!(
        m.capital(seller, cereal),
        10,
        "goods already debited from the seller must be physically driven back and \
         re-credited, never silently destroyed"
    );
    assert_eq!(
        m.capital(buyer, cereal),
        0,
        "a removed buyer must never be credited"
    );
}
