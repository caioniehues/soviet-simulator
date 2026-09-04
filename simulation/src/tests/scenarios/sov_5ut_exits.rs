//! sov-5ut: every dispatch termination that does not deliver runs through
//! one shared exit helper — re-post the buyer's demand (unless the buyer is
//! gone or fulfilled), refund an already-settled border leg, restore the
//! seller's offer where the goods never left. Double-exit is a no-op.
//!
//! The model is the landed sov-ahw `ToSource` timeout block. The TRAP this
//! closes: `Market::remove`'s `ToSource` arm freed the reservation but never
//! restored the seller's sell order, and the `ToSource`/entity-gone arm plus
//! the `sim_drop` seller-half dropped the buyer's dispatch with no re-post
//! and no refund — a never-game-over violation either way.

use super::*;

use super::hoarding::{build_company_at, remove_soul, setup_seller_buyer};
use super::inflation::remove_default_freight_station;
use crate::economy::{DispatchState, Government, Market};
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

/// sov-5ut, dead-buyer `ToSource` exit: the reservation is freed AND the
/// seller's offer is restored (the TRAP), nothing is re-posted for a dead
/// buyer, and no loss is recorded for goods that never moved. Exiting twice
/// is a no-op.
#[test]
fn sov_5ut_remove_buyer_in_tosource_restores_seller_offer_and_noops_twice() {
    let mut ctx = TestCtx::new();
    let (seller, buyer, seller_pos, buyer_pos) = setup_seller_buyer(&mut ctx, 120.0);
    // Deliberately no truck: the dispatch waits in ToSource either way.

    {
        let mut m = ctx.g.write::<Market>();
        m.produce(seller, cereal(), 5);
        m.sell(seller, seller_pos.xy(), cereal(), 5, 0);
        m.buy(buyer, buyer_pos.xy(), cereal(), 5);
    }

    ctx.tick(); // domestic match: reserved 5, dispatch in ToSource

    {
        let m = ctx.g.read::<Market>();
        assert_eq!(m.dispatches().len(), 1);
        assert_eq!(m.dispatches()[0].state, DispatchState::ToSource);
    }

    remove_soul(&mut ctx, buyer);

    let m = ctx.g.read::<Market>();
    assert!(
        m.dispatches().is_empty(),
        "the dead buyer's dispatch must exit, not linger"
    );
    assert_eq!(
        m.reserved(seller, cereal()),
        0,
        "the ToSource exit must free the seller's reservation"
    );
    assert_eq!(
        m.inner()[&cereal()].sell_order(seller).unwrap().qty,
        5,
        "TRAP: the match deleted the sell order at 0; the exit must put the offer back"
    );
    assert!(
        m.inner()[&cereal()].buy_order(buyer).is_none(),
        "no demand may be re-posted for a removed buyer"
    );
    assert!(
        m.lost().is_empty(),
        "goods that never left the seller are not a loss"
    );
    drop(m);

    // Double-exit is a no-op: removing the same soul again changes nothing
    // and panics nowhere.
    remove_soul(&mut ctx, buyer);

    let m = ctx.g.read::<Market>();
    assert!(m.dispatches().is_empty());
    assert_eq!(
        m.inner()[&cereal()].sell_order(seller).unwrap().qty,
        5,
        "a second exit must not duplicate the restored offer"
    );
}

/// sov-5ut, `ToSource`/entity-gone exit: the reserved truck vanishes before
/// arriving, so the match is rolled back in full — reservation, sell order,
/// AND buy order (the live buyer must be able to ask again).
#[test]
fn sov_5ut_vanished_truck_in_tosource_reposts_live_buyer() {
    let mut ctx = TestCtx::new();
    let (seller, buyer, seller_pos, buyer_pos) = setup_seller_buyer(&mut ctx, 120.0);
    spawn_parked_vehicle(&mut ctx.g, VehicleKind::Truck, seller_pos).expect("truck must spawn");

    {
        let mut m = ctx.g.write::<Market>();
        m.produce(seller, cereal(), 5);
        m.sell(seller, seller_pos.xy(), cereal(), 5, 0);
        m.buy(buyer, buyer_pos.xy(), cereal(), 5);
    }

    // Wait for the match AND the truck assignment, while still ToSource.
    let truck = loop {
        ctx.tick();
        let m = ctx.g.read::<Market>();
        assert_eq!(m.dispatches().len(), 1);
        let d = m.dispatches()[0];
        assert_eq!(
            d.state,
            DispatchState::ToSource,
            "the truck must not have loaded yet"
        );
        if let Some(v) = d.truck() {
            break v;
        }
    };

    // The truck vanishes before arriving (e.g. despawned).
    ctx.g.world_mut_unchecked().vehicles.remove(truck);
    ctx.tick();

    let m = ctx.g.read::<Market>();
    assert!(
        m.dispatches().is_empty(),
        "the entity-gone dispatch must exit, not wedge on a dead truck"
    );
    assert_eq!(m.reserved(seller, cereal()), 0);
    assert_eq!(
        m.inner()[&cereal()].sell_order(seller).unwrap().qty,
        5,
        "the seller's offer must be restored"
    );
    assert_eq!(
        m.inner()[&cereal()]
            .buy_order(buyer)
            .map(|o| o.qty),
        Some(5),
        "the LIVE buyer's demand must be re-posted, or its enterprise can never ask again"
    );
}

/// sov-5ut, settled-import exit with the buyer gone and no route home: the
/// treasury gets the Loading-arrival settlement back (refund), the loss is
/// named, and no demand is resurrected for the dead buyer.
///
/// Isolation (as in sov-7f7): no houses, so no humans, no wages, no retail —
/// the only money mover is the import leg under test.
#[test]
fn sov_5ut_remove_buyer_of_settled_import_refunds_treasury() {
    let mut ctx = TestCtx::new();
    remove_default_freight_station(&mut ctx);

    // Reachable station (sov-7f7 layout): road, station on a spur, truck.
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
    ctx.build_roads(&[
        Vec3::new(250.0, 0.0, 0.0),
        Vec3::new(station_centre.x, station_centre.y - 120.0, 0.0),
    ]);
    let _station = ctx
        .g
        .world()
        .freight_stations
        .iter()
        .next()
        .map(|(id, _)| SoulID::FreightStation(id))
        .expect("station soul must exist");
    let station_door = ctx
        .g
        .map()
        .buildings()
        .iter()
        .find(|(_, b)| matches!(b.kind, BuildingKind::RailFreightStation(_)))
        .map(|(_, b)| b.door_pos)
        .expect("station building must exist");
    spawn_parked_vehicle(&mut ctx.g, VehicleKind::Truck, station_door).expect("truck must spawn");

    // Workerless cereal-farm buyer: empty consumption, production gated on
    // workers it never gets — the manual order below is the only demand.
    let farm_b = build_company_at(
        &mut ctx,
        GoodsCompanyID::new("cereal-farm").prototype(),
        Vec2::new(100.0, 20.0),
    );
    ctx.tick();
    let buyer = ctx.g.read::<BuildingInfos>().owner(farm_b).unwrap();
    let buyer_pos = ctx.g.map().buildings.get(farm_b).unwrap().door_pos;

    let money_before: Money = ctx.g.read::<Government>().money;
    {
        let mut m = ctx.g.write::<Market>();
        m.buy(buyer, buyer_pos.xy(), cereal(), 5);
    }

    // Drive until the truck loads at the station: the import leg settles.
    let mut ticks = 0;
    loop {
        ctx.tick();
        ticks += 1;
        let m = ctx.g.read::<Market>();
        if m
            .dispatches()
            .iter()
            .any(|d| d.kind == cereal() && d.state == DispatchState::Loading)
        {
            break;
        }
        assert!(ticks < 4000, "import never reached Loading");
    }
    let money_settled: Money = ctx.g.read::<Government>().money;
    assert!(
        money_settled < money_before,
        "the Loading arrival must have settled the import payment"
    );

    // Demolish every road, then remove the buyer: no route can carry the
    // goods back to the station, so the exit is an honest loss + refund.
    let road_ids: Vec<_> = ctx.g.map().roads().keys().collect();
    for r in road_ids {
        ctx.g.map_mut().remove_road(r);
    }
    remove_soul(&mut ctx, buyer);

    {
        let m = ctx.g.read::<Market>();
        assert!(
            m.dispatches().is_empty(),
            "the dead buyer's dispatch must exit via the shared helper"
        );
        assert!(
            m.inner()[&cereal()].buy_order(buyer).is_none(),
            "no demand may be re-posted for a removed buyer"
        );
        let lost_qty: u32 = m
            .lost()
            .iter()
            .filter(|e| e.kind == cereal())
            .map(|e| e.qty)
            .sum();
        assert_eq!(lost_qty, 5, "settled-but-undelivered goods are a named loss");
    }

    // The refund parks one pass (remove runs outside market_update): a tick
    // drains it back into the treasury, to the exact pre-match level.
    ctx.tick();
    assert_eq!(
        ctx.g.read::<Government>().money,
        money_before,
        "a border leg dead between match and delivery must be refunded"
    );
}
