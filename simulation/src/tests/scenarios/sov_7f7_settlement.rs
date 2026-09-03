//! sov-7f7: border money settles at delivery, not at match (ADR-0003 §1).
//!
//! An import's `money_delta` must apply when the truck LOADS at the freight
//! station (`Loading` arrival); an export's when it UNLOADS at the border
//! door (`ToDestination` arrival). The match itself only creates the
//! commitment (orders wiped, reservation held). Before the fix both halves
//! hit `Government.money` inside `market_update`'s match loop, thousands of
//! ticks before — or entirely without — any goods moving.
//!
//! Isolation: neither test builds houses, so no humans exist, so no wages,
//! no retail and no hiring can move `Government.money` under the assertions.
//! Every company in these worlds is a workerless cereal-farm (empty
//! consumption, production gated on workers it never gets), so the only
//! market orders in the city are the ones the tests place by hand.

use super::hoarding::build_company_at;
use super::inflation::remove_default_freight_station;
use super::*;
use crate::economy::{DispatchState, EcoStats, Government, Market};
use crate::map::BuildingKind;
use crate::map_dynamic::BuildingInfos;
use crate::transportation::{spawn_parked_vehicle, VehicleKind};
use crate::world_command::WorldCommand;
use crate::SoulID;
use geom::OBB;
use prototypes::{BuildingGen, FreightStationPrototypeID, GoodsCompanyID, ItemID, Money};

fn cereal() -> ItemID {
    ItemID::new("cereal")
}

fn gov_money(ctx: &TestCtx) -> Money {
    ctx.g.read::<Government>().money
}

/// Sum of one EcoStats leg's money ring for an item, over all bins: the
/// commitment's exact `money_delta` with no cursor bookkeeping, whatever tick
/// the match landed on.
fn leg_money(ctx: &TestCtx, exports: bool, item: ItemID) -> Money {
    let eco = ctx.g.read::<EcoStats>();
    let histories = if exports { &eco.exports } else { &eco.imports };
    let v: Money = histories
        .iter_histories(0)
        .find(|(id, _)| *id == item)
        .map(|(_, lvl)| lvl.past_ring_money.iter().copied().sum())
        .unwrap_or(Money::ZERO);
    v
}

/// Same for the item-quantity ring: proves a border leg matched at all.
fn leg_items(ctx: &TestCtx, exports: bool, item: ItemID) -> i64 {
    let eco = ctx.g.read::<EcoStats>();
    let histories = if exports { &eco.exports } else { &eco.imports };
    let v: i64 = histories
        .iter_histories(0)
        .find(|(id, _)| *id == item)
        .map(|(_, lvl)| lvl.past_ring_items.iter().copied().sum())
        .unwrap_or(0);
    v
}

/// A freight station whose door trucks can actually reach (the sov-abs
/// layout: the seeded default station is road-disconnected by design).
/// Returns the station soul and its door position.
fn reachable_station(ctx: &mut TestCtx) -> (SoulID, Vec3) {
    ctx.build_roads(&[Vec3::new(0.0, 0.0, 0.0), Vec3::new(300.0, 0.0, 0.0)]);
    let station_centre = Vec2::new(150.0, 200.0);
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
    assert_eq!(
        ctx.g.world().freight_stations.len(),
        1,
        "exactly one, reachable, freight station must exist"
    );
    ctx.build_roads(&[
        Vec3::new(250.0, 0.0, 0.0),
        Vec3::new(station_centre.x, station_centre.y - 120.0, 0.0),
    ]);

    let soul = ctx
        .g
        .world()
        .freight_stations
        .iter()
        .next()
        .map(|(id, _)| SoulID::FreightStation(id))
        .expect("freight station soul must exist");
    let door = ctx
        .g
        .map()
        .buildings()
        .iter()
        .find(|(_, b)| matches!(b.kind, BuildingKind::RailFreightStation(_)))
        .map(|(_, b)| b.door_pos)
        .expect("freight station building must exist");
    assert!(
        spawn_parked_vehicle(&mut ctx.g, VehicleKind::Truck, door).is_some(),
        "a truck must be spawnable at the station door"
    );
    (soul, door)
}

/// Import half: match moves nothing; the Loading arrival settles the exact
/// committed delta, once; delivery settles nothing further.
#[test]
fn sov_7f7_import_money_settles_at_loading_not_at_match() {
    let mut ctx = TestCtx::new();
    remove_default_freight_station(&mut ctx);
    let (_station, _station_door) = reachable_station(&mut ctx);

    // Workerless cereal-farm as buyer: empty consumption (posts no orders of
    // its own), production gated on workers it never gets, so the manual
    // order below is the only cereal demand in the city.
    let farm = GoodsCompanyID::new("cereal-farm").prototype();
    let farm_b = build_company_at(&mut ctx, farm, Vec2::new(100.0, 20.0));
    ctx.tick(); // company soul spawns, recipe_init registers (no orders)
    let buyer = ctx.g.read::<BuildingInfos>().owner(farm_b).unwrap();

    ctx.g
        .write::<Market>()
        .buy(buyer, Vec2::new(100.0, 20.0), cereal(), 7);

    // Drive to the match: the import must appear as a Dispatch out of the
    // station (sov-abs), and the match tick must leave Government.money
    // exactly alone.
    let mut ticks = 0;
    loop {
        let before = gov_money(&ctx);
        ctx.tick();
        ticks += 1;
        let matched = ctx.g.read::<Market>().dispatches().iter().any(|d| {
            d.kind == cereal() && d.buyer == buyer && matches!(d.seller, SoulID::FreightStation(_))
        });
        if matched {
            assert_eq!(
                gov_money(&ctx),
                before,
                "sov-7f7: an import match is a commitment only; Government.money \
                 must not move until the truck loads at the station"
            );
            break;
        }
        assert!(
            ticks < 2000,
            "the border never matched the farm's cereal order"
        );
    }
    let import_delta = leg_money(&ctx, false, cereal());
    assert!(
        import_delta < Money::ZERO,
        "the import commitment must carry a negative money_delta, got {:?}",
        import_delta
    );
    let money_at_match = gov_money(&ctx);

    // Drive to the Loading arrival: money sits still while the truck drives
    // to the station, then moves by exactly the committed delta.
    let mut ticks = 0;
    loop {
        let state = ctx
            .g
            .read::<Market>()
            .dispatches()
            .iter()
            .find(|d| d.kind == cereal() && d.buyer == buyer)
            .map(|d| d.state);
        match state {
            None => panic!("the import dispatch vanished before loading"),
            Some(DispatchState::ToSource) => {
                assert_eq!(
                    gov_money(&ctx),
                    money_at_match,
                    "money must not move while the truck is still driving to the station"
                );
            }
            _ => {
                assert_eq!(
                    gov_money(&ctx),
                    money_at_match + import_delta,
                    "the Loading arrival must settle the import delta exactly once \
                     (committed {:?})",
                    import_delta
                );
                break;
            }
        }
        ctx.tick();
        ticks += 1;
        assert!(ticks < 20000, "the import truck never reached the station");
    }
    let money_settled = gov_money(&ctx);

    // Through full delivery: the buyer's-door arrival belongs to the export
    // leg, so it must settle nothing further — and the goods must arrive.
    let mut ticks = 0;
    loop {
        ctx.tick();
        ticks += 1;
        assert_eq!(
            gov_money(&ctx),
            money_settled,
            "money must move exactly once per import; the delivery arrival \
             settled a second time"
        );
        if ctx.g.read::<Market>().capital(buyer, cereal()) > 0 {
            break;
        }
        assert!(
            ticks < 20000,
            "the imported cereal never arrived at the farm"
        );
    }
}

/// Export half (match side): a real surplus export matches, and the match
/// tick leaves Government.money exactly alone. The arrival half lives in
/// `sov_7f7_export_money_settles_at_todestination` below.
#[test]
fn sov_7f7_export_money_not_moved_at_match() {
    let mut ctx = TestCtx::new();
    remove_default_freight_station(&mut ctx);
    let (_station, _station_door) = reachable_station(&mut ctx);

    let farm = GoodsCompanyID::new("cereal-farm").prototype();
    let farm_b = build_company_at(&mut ctx, farm, Vec2::new(60.0, 30.0));
    ctx.tick(); // company soul spawns
    let seller = ctx.g.read::<BuildingInfos>().owner(farm_b).unwrap();
    let seller_pos = ctx.g.map().buildings.get(farm_b).unwrap().door_pos;

    {
        let mut m = ctx.g.write::<Market>();
        m.produce(seller, cereal(), 60);
        m.sell(seller, seller_pos.xy(), cereal(), 60, 0);
    }

    let mut ticks = 0;
    loop {
        let before = gov_money(&ctx);
        ctx.tick();
        ticks += 1;
        let exported = leg_items(&ctx, true, cereal());
        if exported > 0 {
            assert_eq!(
                gov_money(&ctx),
                before,
                "sov-7f7: an export match is a commitment only; Government.money \
                 must not move until the truck unloads at the border door \
                 ({} cereal exported on the match tick)",
                exported
            );
            break;
        }
        assert!(
            ticks < 2000,
            "the border never matched the farm's cereal surplus"
        );
    }
    let export_delta = leg_money(&ctx, true, cereal());
    assert!(
        export_delta > Money::ZERO,
        "the export commitment must carry a positive money_delta, got {:?}",
        export_delta
    );
}

/// Export half (arrival side): a border-bound commitment settles exactly its
/// delta on the `ToDestination` arrival — unmoved while driving to the farm
/// and while loading, moved once at the border door, never again after.
///
/// The dispatch is pushed through `Market::test_push_dispatch`, not matched:
/// exports get no dispatch of their own until sov-20g lands the export
/// dispatch loop, so no gameplay path can drive one here yet — and pushing it
/// directly keeps this world free of any other border activity, so the money
/// assertions are exact. The match side (a real surplus match moves nothing)
/// is covered by `sov_7f7_export_money_not_moved_at_match` above; this covers
/// the hook the same commitment will hit once sov-20g's dispatch exists.
#[test]
fn sov_7f7_export_money_settles_at_todestination() {
    let mut ctx = TestCtx::new();
    remove_default_freight_station(&mut ctx);
    let (station, _station_door) = reachable_station(&mut ctx);

    let farm = GoodsCompanyID::new("cereal-farm").prototype();
    let farm_b = build_company_at(&mut ctx, farm, Vec2::new(60.0, 30.0));
    ctx.tick(); // company soul spawns
    let seller = ctx.g.read::<BuildingInfos>().owner(farm_b).unwrap();
    let seller_pos = ctx.g.map().buildings.get(farm_b).unwrap().door_pos;
    assert!(
        spawn_parked_vehicle(&mut ctx.g, VehicleKind::Truck, seller_pos).is_some(),
        "a truck must be spawnable at the farm door"
    );

    // Goods on hand so the Loading debit is clean, but deliberately NO sell
    // order: no real export match may exist in this world (its post-sov-20g
    // dispatch would settle too and break the exact assertions below).
    // Distinctive fixed delta: the hook applies whatever the commitment
    // carries — the match-side test ties real quotes to the same path.
    let export_delta = Money::new_cents(12345);
    {
        let mut m = ctx.g.write::<Market>();
        m.produce(seller, cereal(), 6);
        m.test_push_dispatch(station, seller, cereal(), 6, export_delta);
    }
    let money_before = gov_money(&ctx);

    let mut settled_seen = false;
    let mut ticks = 0;
    loop {
        let state = ctx
            .g
            .read::<Market>()
            .dispatches()
            .iter()
            .find(|d| d.kind == cereal() && d.seller == seller)
            .map(|d| d.state);
        match state {
            None => {
                assert!(
                    settled_seen,
                    "the export dispatch vanished before settling at the border door"
                );
                assert_eq!(
                    gov_money(&ctx),
                    money_before + export_delta,
                    "nothing may settle after the border-door arrival"
                );
                break;
            }
            Some(DispatchState::ToSource)
            | Some(DispatchState::Loading)
            | Some(DispatchState::ToDestination) => {
                assert_eq!(
                    gov_money(&ctx),
                    money_before,
                    "export money must not move before the border-door arrival \
                     (still {:?})",
                    state.unwrap()
                );
            }
            Some(DispatchState::Unloading) => {
                assert_eq!(
                    gov_money(&ctx),
                    money_before + export_delta,
                    "the ToDestination arrival must settle the export delta \
                     exactly once (committed {:?})",
                    export_delta
                );
                settled_seen = true;
            }
            Some(unexpected) => {
                panic!(
                    "the export dispatch took an unexpected path ({:?}); both \
                     doors are live, so only the forward journey may happen",
                    unexpected
                );
            }
        }
        ctx.tick();
        ticks += 1;
        assert!(
            ticks < 20000,
            "the export truck never reached the border door"
        );
    }
}
