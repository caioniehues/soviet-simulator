//! sov-ahw: `DispatchState::ToSource` had no tick countdown on the
//! truck-less arm, and a stranded import there killed its enterprise
//! PERMANENTLY -- a "never game over" pillar violation, not merely a wedge.
//!
//! The causal chain, all re-derived from source:
//!   1. `make_trades`' ext-trade block (`market.rs`) `extract_if`s the buy
//!      order out of `buy_orders` at match time, so the order is GONE.
//!   2. `advance_dispatches`' `ToSource`/`truck: None` arm had no counter and
//!      no `else`: if `Dispatcher::query` never offers a truck, the dispatch
//!      sits there forever holding `reserved[seller]`.
//!   3. `buy_until` has exactly two callers (`souls/goods_company.rs`):
//!      `recipe_init`, which runs once at company creation, and `recipe_act`,
//!      which only runs when `recipe_should_produce` is true -- which needs
//!      `capital >= amount` of the very input that never arrived. So the
//!      order is never re-posted and the enterprise is dead forever, and
//!      stays dead even after the player lays the road that would fix it.
//!
//! The fix (`MAX_SOURCE_WAIT_TICKS`, economy/market.rs) bounds the wait and
//! rolls the whole match back onto the market when it expires: reservation
//! released, sell order restored, buy order re-posted via `buy_until`
//! BEFORE the dispatch is removed (the sov-6qx rule: the caller undoes its
//! own bookkeeping). A bare countdown without the re-post is NOT the fix and
//! must not be accepted as one -- the enterprise would stay just as dead.

use super::*;

use super::hoarding::setup_seller_buyer;
use super::inflation::remove_default_freight_station;
use crate::economy::{DispatchState, Government, Market};
use crate::map::{BuildingID, BuildingKind, RoadID};
use crate::map_dynamic::BuildingInfos;
use crate::transportation::{spawn_parked_vehicle, VehicleKind};
use crate::world::HumanID;
use crate::SoulID;
use geom::OBB;
use prototypes::{BuildingGen, FreightStationPrototypeID};
use prototypes::{GoodsCompanyID, ItemID};

/// Same as `inflation::build_company_at`: builds a goods-company building
/// with an explicit `connected_road` for its electricity network. Defined
/// here (rather than importing it) because that helper is private to its
/// module and this ticket owns no shared scenario file.
fn build_company_on_road(
    ctx: &mut TestCtx,
    proto: &prototypes::GoodsCompanyPrototype,
    p: Vec2,
    connected_road: Option<RoadID>,
) -> BuildingID {
    let obb = OBB::new(p, Vec2::X, proto.base.size.w, proto.base.size.h);
    let b = ctx
        .g
        .map_mut()
        .build_special_building(
            &obb,
            BuildingKind::GoodsCompany(proto.id),
            proto.base.bgen,
            None,
            connected_road,
        )
        .unwrap();
    ctx.g.write::<BuildingInfos>().insert(b);
    b
}

/// sov-ahw: a stranded import re-posts its buy order and RESUMES production
/// once a route becomes available.
///
/// The trigger is the reachability mismatch named in the ticket:
/// `market_update`'s `find_external` only tests `nearest_lane(door, Driving,
/// DISPATCH_LANE_CUTOFF)`, while `DispatchOne::query` runs a real backward
/// BFS over the lane graph (`map_dynamic/dispatch.rs`). The freight station
/// below sits on its OWN road island with a driving lane at its door, so it
/// passes the proximity gate and is matched, while the BFS from its lane can
/// never reach the city's only truck. No trickery is needed: this is an
/// ordinary "the player has not connected the station yet" city.
///
/// Mutation: delete the `MAX_SOURCE_WAIT_TICKS` block in
/// `advance_dispatches`' `ToSource`/`None` arm and phase 2 below fails --
/// the buy order never comes back. Delete only the `buy_until` re-post
/// inside it (keeping the countdown) and phase 2 fails at the same
/// assertion -- the dispatch is cleaned up but the enterprise never asks
/// again, which is exactly the "a bare countdown is not the fix" trap.
#[test]
fn sov_ahw_stranded_tosource_import_reposts_and_resumes_production() {
    let mut ctx = TestCtx::new();
    // The START_COMMANDS station is 4km away with no driving lane; this test
    // supplies its own, deliberately unreachable one.
    remove_default_freight_station(&mut ctx);

    // --- road island A: the city -------------------------------------------
    ctx.build_roads(&[Vec3::new(0.0, 0.0, 0.0), Vec3::new(300.0, 0.0, 0.0)]);
    // A side road off the main one. Phase 3's connection attaches at the main
    // road's far end, so this road's id and lanes survive it -- which both the
    // truck (see below) and `connected_road` here depend on: an electricity
    // `connected_road` pointing at a road that a later edit destroyed panics
    // in `map/electricity_cache.rs:129`.
    let (_, road_side) = {
        let mut m = ctx.g.map_mut();
        let a = m.project(Vec3::new(250.0, 0.0, 0.0), 0.0, ProjectFilter::ALL);
        let b = m.project(Vec3::new(250.0, -160.0, 0.0), 0.0, ProjectFilter::ALL);
        m.make_connection(a, b, None, &LanePatternBuilder::default().build())
            .unwrap()
    };

    // --- road island B: the freight station, on its own disconnected road ---
    // Same geometry as `sov_abs_ext_trade_import_is_physical` (station door
    // inside DISPATCH_LANE_CUTOFF of a spur), translated 800 units east and
    // sharing no intersection with island A.
    ctx.build_roads(&[Vec3::new(800.0, 0.0, 0.0), Vec3::new(1100.0, 0.0, 0.0)]);
    let station_centre = Vec2::new(950.0, 200.0);
    ctx.apply(&[WorldCommand::MapBuildSpecialBuilding {
        pos: OBB::new(station_centre, Vec2::X, 160.0, 200.0),
        kind: BuildingKind::RailFreightStation(FreightStationPrototypeID::new("freight-station")),
        gen: BuildingGen::NoWalkway {
            door_pos: Vec2::new(110.0, 0.0),
        },
        zone: None,
        connected_road: None,
    }]);
    ctx.tick();
    ctx.build_roads(&[
        Vec3::new(1050.0, 0.0, 0.0),
        Vec3::new(station_centre.x, station_centre.y - 120.0, 0.0),
    ]);

    let station_b = ctx
        .g
        .map()
        .buildings()
        .iter()
        .find(|(_, b)| matches!(b.kind, BuildingKind::RailFreightStation(_)))
        .map(|(id, _)| id)
        .unwrap();
    let station_door = ctx.g.map().buildings.get(station_b).unwrap().door_pos;
    assert!(
        ctx.g
            .map()
            .nearest_lane(
                station_door,
                crate::map::LaneKind::Driving,
                Some(crate::map_dynamic::DISPATCH_LANE_CUTOFF),
            )
            .is_some(),
        "the station door {:?} must be lane-proximate, or `find_external` \
         refuses it and no import is ever matched at all",
        station_door
    );

    // --- the enterprise: a bakery, on island A, fully able to produce -------
    // `bakery` consumes flour and produces bread (base_mod/companies.lua),
    // 3 workers, 100s per cycle. No domestic flour seller exists here, so the
    // border is its only source. `connected_road` is threaded so the bakery
    // shares an electricity network with the solar panel -- `productivity`
    // returns 0.0 on a blackout and no amount of flour would then produce
    // bread (souls/goods_company.rs).
    let bakery_proto = GoodsCompanyID::new("bakery").prototype();
    let bakery_b = build_company_on_road(
        &mut ctx,
        bakery_proto,
        Vec2::new(150.0, 30.0),
        Some(road_side),
    );
    let solar_proto = GoodsCompanyID::new("solar-panel").prototype();
    build_company_on_road(
        &mut ctx,
        solar_proto,
        Vec2::new(150.0, -200.0),
        Some(road_side),
    );

    // The city's only truck, parked on island A. The bakery is a `store`, so
    // `company_soul` spawns it none of its own (only `CompanyKind::Factory`
    // does), which is what makes the truck pool here exactly one and its
    // island membership unambiguous.
    //
    // It is parked on the SIDE road, not the main one, for a substrate reason
    // found while writing this test and filed separately: `DispatchOne::
    // register` (map_dynamic/dispatch.rs) early-returns for a vehicle that has
    // not moved since it was last seen, so its cached `LaneID` is never
    // refreshed. Joining island A to island B in phase 3 rebuilds the main
    // road's lanes, and a truck parked on them would keep an index entry under
    // a lane that no longer exists -- permanently unqueryable, and nothing to
    // do with sov-ahw. The side road is untouched by that connection, so the
    // truck this test relies on stays honestly indexed.
    spawn_parked_vehicle(
        &mut ctx.g,
        VehicleKind::Truck,
        Vec3::new(250.0, -100.0, 0.0),
    )
    .expect("the city's truck must spawn");

    ctx.tick(); // company souls spawn; recipe_init posts the flour buy order

    let bakery = ctx.g.read::<BuildingInfos>().owner(bakery_b).unwrap();
    let flour = ItemID::new("flour");
    let bread = ItemID::new("bread");

    // Staff the bakery by writing `workers.0` directly, the technique
    // `recipe_provided::scenario_0096_workforce_sourced_live_from_present_population`
    // already establishes: `raw_productivity` reads nothing but that Vec's
    // LENGTH, and the only other reader (`company_system`, goods_company.rs)
    // does `world.humans.get(worker) else { continue }`, so ids that name no
    // entity are inert. Hiring three real humans instead costs three
    // pedestrians and three cars churning through the `transport_grid` for the
    // whole run, and that grid's `flat_spatial` `FnvHashMap` has no
    // serialization-order guarantee -- it diverged the determinism check at
    // tick 11,055 while this test was being written, which is a known harness
    // gap (see inflation.rs) and nothing to do with sov-ahw. Production here
    // still runs through the real `company_system` -> `recipe_act` path.
    let SoulID::GoodsCompany(bakery_id) = bakery else {
        panic!("the bakery must own a GoodsCompany soul");
    };
    ctx.g
        .world_mut_unchecked()
        .companies
        .get_mut(bakery_id)
        .unwrap()
        .workers
        .0 = vec![
        HumanID::from(slotmapd::KeyData::from_ffi((1 << 32) | 1)),
        HumanID::from(slotmapd::KeyData::from_ffi((1 << 32) | 2)),
        HumanID::from(slotmapd::KeyData::from_ffi((1 << 32) | 3)),
    ];

    let buy_order_qty = |ctx: &TestCtx| -> Option<u32> {
        ctx.g
            .read::<Market>()
            .inner()
            .get(&flour)
            .unwrap()
            .buy_order(bakery)
            .map(|o| o.qty)
    };

    // --- phase 1: the import strands in ToSource, buy order consumed --------
    // Since sov-7f7 (ADR-0003 §1) the border commitment settles at DELIVERY
    // (the `Loading` arrival), never at match, so stranding here pays
    // nothing yet. The timeout below must likewise move no money: it hands
    // back no goods that crossed, and refunding a payment that never
    // happened would bleed the treasury once per timeout forever. Nothing
    // else here spends: there are no humans, so
    // `WORKER_CONSUMPTION_PER_MINUTE` never fires, and roads are laid
    // through `Map` directly rather than through a priced `WorldCommand`.
    let money_before = ctx.g.read::<Government>().money;

    let mut ticks = 0;
    // The dispatch's SELLER, kept for the reservation assertion in phase 2.
    // `reserved` is keyed by seller, so asking it about the bakery (the buyer)
    // reads a row that is 0 whatever the rollback does.
    let station_soul = loop {
        ctx.tick();
        ticks += 1;
        let stranded = ctx
            .g
            .read::<Market>()
            .dispatches()
            .iter()
            .find(|d| {
                d.kind == flour
                    && d.buyer == bakery
                    && matches!(d.seller, SoulID::FreightStation(_))
                    && d.state == DispatchState::ToSource
            })
            .map(|d| d.seller);
        if let Some(s) = stranded {
            break s;
        }
        assert!(
            ticks < 400,
            "the border never matched the bakery's flour order"
        );
    };
    assert_eq!(
        buy_order_qty(&ctx),
        None,
        "precondition: the ext-trade match must have consumed the buy order \
         (`extract_if` in `make_trades`), or this test proves nothing about \
         re-posting it"
    );
    assert_eq!(
        ctx.g.read::<Market>().capital(bakery, flour),
        0,
        "nothing may be credited while the import is still in ToSource"
    );

    // --- phase 2: the wait is BOUNDED and the demand comes back ------------
    let mut ticks = 0;
    let returned_qty = loop {
        ctx.tick();
        ticks += 1;
        if let Some(q) = buy_order_qty(&ctx) {
            break q;
        }
        assert!(
            ticks < 4000,
            "an import stranded in ToSource must be given up on and its buy \
             order re-posted; after {} ticks the enterprise still has no \
             standing order and can never ask again (dispatches: {:?})",
            ticks,
            ctx.g.read::<Market>().dispatches()
        );
    };
    assert!(
        returned_qty > 0,
        "the re-posted buy order must actually ask for something"
    );
    assert_eq!(
        ctx.g
            .read::<Market>()
            .dispatches()
            .iter()
            .filter(|d| d.kind == flour && d.buyer == bakery)
            .count(),
        0,
        "the stranded dispatch must be gone, not merely counted down"
    );
    // Note on strength: an ext-trade match never inserts a `reserved` row at
    // all (only the domestic `filter_map` in `make_trades` does), so on THIS
    // path the assertion is a floor, not a trap -- deleting the release does
    // not turn it red. The release is trapped for real by
    // `sov_ahw_domestic_wait_past_timeout_still_delivers` below, whose match
    // is domestic and therefore does reserve.
    assert_eq!(
        ctx.g.read::<Market>().reserved(station_soul, flour),
        0,
        "the ToSource rollback must release the SELLER's reservation ({:?}), \
         not strand it",
        station_soul
    );
    assert_eq!(
        ctx.g.read::<Government>().money,
        money_before,
        "no goods crossed the border, so the treasury must be exactly where it \
         started: the match paid nothing (settlement is at delivery since \
         sov-7f7) and the timeout must move nothing either"
    );

    // --- phase 3: the player lays the road; the route now exists -----------
    ctx.build_roads(&[Vec3::new(300.0, 0.0, 0.0), Vec3::new(800.0, 0.0, 0.0)]);

    let mut ticks = 0;
    while ctx.g.read::<Market>().capital(bakery, flour) <= 0 {
        ctx.advance_ticks(10);
        ticks += 10;
        assert!(
            ticks < 20000,
            "once the islands are joined the truck can reach the station, so \
             the import must physically arrive"
        );
    }

    // --- phase 4: production RESUMES ---------------------------------------
    // Bread only ever comes from `recipe_act`, so observing any bread at all
    // proves the enterprise came back to life. One cycle is 100 game-seconds
    // = 5,000 ticks at productivity 1.0; the budget covers a fully-unstaffed
    // slowdown too.
    let mut produced = false;
    for _ in 0..400 {
        ctx.advance_ticks(50);
        if ctx.g.read::<Market>().capital(bakery, bread) > 0 {
            produced = true;
            break;
        }
    }
    assert!(
        produced,
        "the enterprise must RESUME PRODUCTION once a route becomes \
         available -- cleaning the dispatch up without re-posting the buy \
         order leaves it just as dead (flour capital: {}, buy order: {:?})",
        ctx.g.read::<Market>().capital(bakery, flour),
        buy_order_qty(&ctx)
    );
}

/// sov-ahw, round 2: the ORDINARY domestic path, not the exotic one. Every
/// truck in the city is busy (here: there is no truck at all yet) and the
/// delivery waits longer than `MAX_SOURCE_WAIT_TICKS`. Once a truck appears
/// the goods must still be delivered.
///
/// The first attempt at this ticket failed exactly here. Its timeout rolled
/// back the reservation, the money and the buy order, but not the seller's
/// `SellOrder`, which `make_trades` had deleted when the match drove its `qty`
/// to 0. The buyer's demand was therefore re-posted into a market with no
/// cereal on offer, and the ext-trade block then `extract_if`d and dropped
/// that order too, since the default city's border is closed. A truck arriving
/// afterwards had nothing to carry and the buyer stayed at 0 forever -- a
/// never-game-over violation moved from "the player has not connected the
/// station" onto "the trucks are busy for six seconds".
#[test]
fn sov_ahw_domestic_wait_past_timeout_still_delivers() {
    let mut ctx = TestCtx::new();
    let (seller, buyer, seller_pos, buyer_pos) = setup_seller_buyer(&mut ctx, 120.0);

    let cereal = ItemID::new("cereal");
    {
        let mut m = ctx.g.write::<Market>();
        m.produce(seller, cereal, 5);
        m.sell(seller, seller_pos.xy(), cereal, 5, 0);
        m.buy(buyer, buyer_pos.xy(), cereal, 5);
    }

    // Well past the 300-tick bound, so the match times out and rolls back at
    // least once before any truck exists.
    ctx.advance_ticks(400);
    assert_eq!(
        ctx.g.read::<Market>().capital(buyer, cereal),
        0,
        "nothing may reach the buyer while the city has no truck"
    );

    spawn_parked_vehicle(&mut ctx.g, VehicleKind::Truck, seller_pos).expect("truck must spawn");

    let mut ticks = 0;
    while ctx.g.read::<Market>().capital(buyer, cereal) < 5 {
        ctx.advance_ticks(50);
        ticks += 50;
        assert!(
            ticks < 4000,
            "a delivery that outwaited MAX_SOURCE_WAIT_TICKS must still happen \
             once a truck exists: the timeout has to put the seller's SELL \
             order back on the market, not only the buyer's buy order \
             (seller capital {}, buyer capital {}, dispatches {:?})",
            ctx.g.read::<Market>().capital(seller, cereal),
            ctx.g.read::<Market>().capital(buyer, cereal),
            ctx.g.read::<Market>().dispatches()
        );
    }

    let m = ctx.g.read::<Market>();
    assert_eq!(
        m.capital(seller, cereal),
        0,
        "the seller must be debited exactly once, not once per rollback cycle"
    );
    assert_eq!(
        m.capital(buyer, cereal),
        5,
        "the buyer must be credited once"
    );
    // The real trap for the reservation release: every timeout rolls the
    // reservation back, and the final delivery releases it again. If the
    // timeout leaked its reservation, each rollback cycle would stack another
    // 5 behind it and this row could never read 0 after delivery.
    assert_eq!(
        m.reserved(seller, cereal),
        0,
        "no rollback cycle may strand the seller's reservation (sov-ahw)"
    );
}

/// sov-ahw, round 2: an unmatched buy order must SURVIVE the market pass.
///
/// `make_trades`' external-trade block `extract_if`s every non-human buy order
/// out of `buy_orders` in one go, then walks them looking for a border station
/// `find_external` will accept. Orders it cannot place used to be `continue`d
/// past and silently destroyed, so in a city whose border is closed -- the
/// DEFAULT city, whose only freight station is 4 km out with no driving lane
/// at its door -- an enterprise's demand vanished the first tick no domestic
/// seller happened to be offering. That eats `recipe_init`'s very first order,
/// and it ate every order the `MAX_SOURCE_WAIT_TICKS` rollback re-posts.
///
/// No cereal is produced here, so nothing can match domestically and the order
/// is guaranteed to reach the ext block.
#[test]
fn sov_ahw_unplaceable_buy_order_survives_the_market_pass() {
    let mut ctx = TestCtx::new();
    let (_seller, buyer, _seller_pos, buyer_pos) = setup_seller_buyer(&mut ctx, 120.0);

    let cereal = ItemID::new("cereal");
    ctx.g
        .write::<Market>()
        .buy(buyer, buyer_pos.xy(), cereal, 5);

    ctx.tick();

    let m = ctx.g.read::<Market>();
    assert_eq!(
        m.inner()
            .get(&cereal)
            .unwrap()
            .buy_order(buyer)
            .map(|o| o.qty),
        Some(5),
        "a buy order the border cannot place must be left on the market, not \
         dropped: nothing here can satisfy it yet, and destroying it is how an \
         enterprise stops asking forever (sov-ahw)"
    );
    assert!(
        m.dispatches().iter().all(|d| d.kind != cereal),
        "no cereal could be matched, so no cereal dispatch may exist: {:?}",
        m.dispatches()
    );
}
