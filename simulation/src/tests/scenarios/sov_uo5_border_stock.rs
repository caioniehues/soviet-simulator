//! sov-uo5: Border custody — the freight station holds a bounded stock
//! ledger that import dispatches draw from, replenished by train arrivals.
//!
//! Before the fix the freight-station seller was a bottomless pit: the
//! `Loading` debit in `Market::advance_dispatches` pushed its capital row
//! negative without bound, so an import conjured goods from nothing. Now the
//! station starts full (`MAX_BORDER_STOCK`), each train arrival tops the
//! ledger back up (`on_train_arrived`), and each import draw goes through
//! `try_draw_border_stock`. An empty ledger fails observably as *visible
//! waiting*: the dispatch keeps its arrived truck and stays in `ToSource`
//! (going-without at the border) instead of loading phantom goods.

use super::hoarding::mk_soul;
use super::inflation::remove_default_freight_station;
use super::*;
use crate::economy::{DispatchState, Market};
use crate::map::BuildingKind;
use crate::map_dynamic::BuildingInfos;
use crate::souls::freight_station::{MAX_BORDER_STOCK, TRAIN_RESTOCK_QTY};
use crate::transportation::{spawn_parked_vehicle, VehicleKind};
use crate::SoulID;
use geom::OBB;
use prototypes::{BuildingGen, FreightStationPrototypeID, ItemID};

/// Builds the reachable-station layout from
/// `ledger::sov_abs_ext_trade_import_is_physical` (default
/// `START_COMMANDS` station demolished, station on its own spur off a main
/// road) and returns the station-door position. The door lands within the
/// dispatcher's lane cutoff of the spur, so import trucks can reach it.
fn reachable_station(ctx: &mut TestCtx) -> Vec3 {
    remove_default_freight_station(ctx);
    ctx.build_roads(&[Vec3::new(0.0, 0.0, 0.0), Vec3::new(300.0, 0.0, 0.0)]);

    // Freight-station prototype is 160x200; park it clear of the road with a
    // spur running out to its door (copied from the sov-abs scenario).
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

    let station_door = {
        let map = ctx.g.map();
        let (_, b) = map
            .buildings()
            .iter()
            .find(|(_, b)| matches!(b.kind, BuildingKind::RailFreightStation(_)))
            .unwrap();
        b.door_pos
    };
    assert!(
        station_door.xy().distance(Vec2::new(150.0, 80.0)) < 50.0,
        "station door must be within the dispatcher lane cutoff of the spur: {:?}",
        station_door
    );
    station_door
}

fn only_station_id(ctx: &TestCtx) -> crate::FreightStationID {
    let (id, _) = ctx
        .g
        .world()
        .freight_stations
        .iter()
        .next()
        .expect("exactly one freight station must exist");
    id
}

fn border_stock(ctx: &TestCtx) -> u32 {
    let fid = only_station_id(ctx);
    ctx.g
        .world()
        .freight_stations
        .get(fid)
        .unwrap()
        .f
        .border_stock
}

/// A fabricated company buyer in a roadside house with a manual buy order —
/// no recipe, so nothing ever consumes the import and the ledger math stays
/// exact. Returns the buyer soul.
fn manual_buyer(ctx: &mut TestCtx, kind: ItemID, qty: u32) -> SoulID {
    let buyer_b = ctx.build_house_at(Vec2::new(120.0, 20.0));
    ctx.tick();
    let buyer = mk_soul((1 << 32) | 77);
    ctx.g.write::<BuildingInfos>().set_owner(buyer_b, buyer);
    let buyer_pos = ctx.g.map().buildings.get(buyer_b).unwrap().door_pos;
    ctx.g
        .write::<Market>()
        .buy(buyer, buyer_pos.xy(), kind, qty);
    buyer
}

/// Σ(buyer capital) + border ledger + Σ(qty held by dispatches that already
/// drew from the border but have not credited the buyer yet). A pure import
/// transfer moves value between these buckets without changing the total.
fn import_total(ctx: &TestCtx, buyer: SoulID, kind: ItemID) -> i64 {
    let m = ctx.g.read::<Market>();
    let cap = m.capital(buyer, kind) as i64;
    let in_flight: i64 = m
        .dispatches()
        .iter()
        .filter(|d| {
            d.kind == kind
                && matches!(
                    d.state,
                    DispatchState::Loading
                        | DispatchState::ToDestination
                        | DispatchState::Returning
                )
        })
        .map(|d| d.qty as i64)
        .sum();
    drop(m);
    cap + border_stock(ctx) as i64 + in_flight
}

/// sov-uo5, empty-stock observable: with an empty Border custody ledger the
/// import dispatch does NOT load — it keeps its arrived truck and waits
/// visibly in `ToSource` (going-without at the border), and the buyer is
/// never credited.
#[test]
fn sov_uo5_empty_border_stock_import_waits_in_tosource() {
    let mut ctx = TestCtx::new();
    let station_door = reachable_station(&mut ctx);
    spawn_parked_vehicle(&mut ctx.g, VehicleKind::Truck, station_door)
        .expect("truck must spawn near the station door");

    let cereal = ItemID::new("cereal");
    let buyer = manual_buyer(&mut ctx, cereal, 7);

    // Drain the ledger: the border has nothing to sell.
    {
        let fid = only_station_id(&ctx);
        ctx.g
            .world_mut_unchecked()
            .freight_stations
            .get_mut(fid)
            .unwrap()
            .f
            .border_stock = 0;
    }

    // The match/dispatch flow itself is untouched: an import dispatch out of
    // the freight station is still created.
    let mut ticks = 0;
    loop {
        ctx.tick();
        ticks += 1;
        if ctx
            .g
            .read::<Market>()
            .dispatches()
            .iter()
            .any(|d| d.kind == cereal && matches!(d.seller, SoulID::FreightStation(_)))
        {
            break;
        }
        assert!(ticks < 1000, "the border never offered the import");
    }

    // Wait until the truck has physically reached the station door, then give
    // it a further window in which a bottomless border would have loaded.
    let mut ticks = 0;
    loop {
        ctx.tick();
        ticks += 1;
        let arrived = {
            let m = ctx.g.read::<Market>();
            let d = m
                .dispatches()
                .iter()
                .find(|d| d.kind == cereal && matches!(d.seller, SoulID::FreightStation(_)))
                .expect("import dispatch must still exist");
            d.truck().is_some_and(|v| {
                ctx.g
                    .world()
                    .vehicles
                    .get(v)
                    .is_some_and(|ve| ve.it.has_ended(0.0))
            })
        };
        if arrived {
            break;
        }
        assert!(
            ticks < 2000,
            "the import truck never reached the station door"
        );
    }
    ctx.advance_ticks(100);

    let m = ctx.g.read::<Market>();
    let imports: Vec<_> = m
        .dispatches()
        .iter()
        .filter(|d| d.kind == cereal && matches!(d.seller, SoulID::FreightStation(_)))
        .collect();
    assert!(
        !imports.is_empty(),
        "the waiting import dispatch must still exist"
    );
    for d in &imports {
        assert_eq!(
            d.state,
            DispatchState::ToSource,
            "an import with an empty border ledger must wait in ToSource, \
             never loading phantom goods: {:?}",
            d
        );
    }
    assert_eq!(
        m.capital(buyer, cereal),
        0,
        "the buyer must never be credited from an empty border"
    );
    drop(m);
    assert_eq!(
        border_stock(&ctx),
        0,
        "waiting must not consume border stock"
    );
}

/// sov-uo5, conservation: domestic (buyer) + border ledger + in-transit stays
/// constant from match, through loading, to physical arrival.
#[test]
fn sov_uo5_border_custody_conserved_across_import() {
    let mut ctx = TestCtx::new();
    let station_door = reachable_station(&mut ctx);
    spawn_parked_vehicle(&mut ctx.g, VehicleKind::Truck, station_door)
        .expect("truck must spawn near the station door");

    let cereal = ItemID::new("cereal");
    const STOCK: u32 = 500;
    const QTY: u32 = 7;
    {
        let fid = only_station_id(&ctx);
        ctx.g
            .world_mut_unchecked()
            .freight_stations
            .get_mut(fid)
            .unwrap()
            .f
            .border_stock = STOCK;
    }
    let buyer = manual_buyer(&mut ctx, cereal, QTY);

    let total_before = import_total(&ctx, buyer, cereal);
    assert_eq!(total_before, STOCK as i64);

    // Phase 1: the truck loads at the station — the ledger drops by exactly
    // the shipment and the goods move into the in-flight bucket.
    let mut ticks = 0;
    loop {
        ctx.tick();
        ticks += 1;
        let loading = ctx.g.read::<Market>().dispatches().iter().any(|d| {
            d.kind == cereal
                && matches!(d.seller, SoulID::FreightStation(_))
                && !matches!(d.state, DispatchState::ToSource)
        });
        if loading {
            break;
        }
        assert!(ticks < 2000, "the stocked import never loaded");
    }
    assert_eq!(
        border_stock(&ctx),
        STOCK - QTY,
        "loading must draw exactly the shipment from the border ledger"
    );
    assert_eq!(
        import_total(&ctx, buyer, cereal),
        total_before,
        "drawing from the border must conserve the total"
    );

    // Phase 2: the truck physically arrives — the buyer is credited and the
    // in-flight bucket drains, total unchanged.
    let mut ticks = 0;
    while ctx.g.read::<Market>().capital(buyer, cereal) == 0 {
        ctx.tick();
        ticks += 1;
        assert!(
            ticks < 5000,
            "the imported cereal never physically arrived at the buyer"
        );
    }
    assert_eq!(ctx.g.read::<Market>().capital(buyer, cereal), QTY as i32);
    assert_eq!(
        import_total(&ctx, buyer, cereal),
        total_before,
        "delivery must conserve the total"
    );
}

/// sov-uo5, restock: one train arrival consumes the waiting/wanted counters
/// (today's behavior) AND replenishes the Border custody ledger, bounded
/// above by `MAX_BORDER_STOCK`; drawing more than the ledger holds fails
/// without touching it.
#[test]
fn sov_uo5_train_arrival_restocks_border_stock_bounded() {
    let mut ctx = TestCtx::new();
    reachable_station(&mut ctx);
    let fid = only_station_id(&ctx);
    let world = ctx.g.world_mut_unchecked();
    let s = &mut world.freight_stations.get_mut(fid).unwrap().f;

    // Empty ledger: draws fail observably and consume nothing.
    s.border_stock = 0;
    assert!(
        !s.try_draw_border_stock(1),
        "drawing from an empty ledger must fail"
    );
    assert_eq!(s.border_stock, 0);

    // One arrival restocks exactly the train quantity on top of the consumed
    // counters.
    s.waiting_cargo = 100;
    s.wanted_cargo = 100;
    s.on_train_arrived();
    assert_eq!(s.waiting_cargo, 0);
    assert_eq!(s.wanted_cargo, 0);
    assert_eq!(s.border_stock, TRAIN_RESTOCK_QTY);

    // A fitting draw succeeds exactly.
    assert!(s.try_draw_border_stock(TRAIN_RESTOCK_QTY));
    assert_eq!(s.border_stock, 0);

    // Restock saturates at the bound instead of growing without limit.
    s.border_stock = MAX_BORDER_STOCK - 1;
    s.on_train_arrived();
    assert_eq!(
        s.border_stock, MAX_BORDER_STOCK,
        "the custody ledger must stay bounded"
    );
    s.on_train_arrived();
    assert_eq!(s.border_stock, MAX_BORDER_STOCK);
}
