//! sov-91e: the three Loading exhaustion sites park-then-free (sov-2c4
//! convention), never bare-free. A freed-but-Driving truck keeps a live
//! TransportGrid collider stopped in a lane: it re-enters the pool
//! permanently unusable (every later `ToSource` query hits the Parked guard
//! and frees it again) and permanently blocking the lane.
//!
//! The three sites (all in `Market::advance_dispatches`' Loading arm, all
//! now routed through the shared exit helper): no route to a live buyer,
//! no route back to a demolished buyer's seller, both buildings gone.

use super::*;

use super::hoarding::build_company_at;
use super::inflation::remove_default_freight_station;
use crate::economy::{DispatchState, Market};
use crate::map_dynamic::BuildingInfos;
use crate::transportation::{spawn_parked_vehicle, VehicleKind, VehicleState};
use prototypes::{GoodsCompanyID, ItemID};

fn cereal() -> ItemID {
    ItemID::new("cereal")
}

/// sov-91e: a dispatch driven to Loading route-failure exhaustion frees its
/// truck Parked with no collider — and that same truck then serves a second
/// dispatch, proving it is usable, not a lane-blocking zombie.
///
/// Mutation: revert the helper's park-then-free to a bare `free()` and the
/// Parked assertion below goes red (the truck stays Driving with a live
/// collider at the seller's door).
#[test]
fn sov_91e_loading_exhaustion_parks_truck_and_truck_serves_again() {
    let mut ctx = TestCtx::new();
    remove_default_freight_station(&mut ctx);

    // Two road islands, never connected: the seller and truck on A, the
    // buyer on B. Domestic matching ignores connectivity, but the truck can
    // never drive A -> B, so Loading exhausts its route retries.
    ctx.build_roads(&[Vec3::new(0.0, 0.0, 0.0), Vec3::new(300.0, 0.0, 0.0)]);
    ctx.build_roads(&[Vec3::new(600.0, 0.0, 0.0), Vec3::new(900.0, 0.0, 0.0)]);
    let seller_b = build_company_at(
        &mut ctx,
        GoodsCompanyID::new("bakery").prototype(),
        Vec2::new(30.0, 20.0),
    );
    let buyer1_b = ctx.build_house_at(Vec2::new(750.0, 20.0));
    ctx.tick(); // company soul spawns; house resident spawns (owner overwritten next)

    let seller = ctx.g.read::<BuildingInfos>().owner(seller_b).unwrap();
    let buyer1 = crate::SoulID::GoodsCompany(crate::world::CompanyID::from(
        slotmapd::KeyData::from_ffi((1 << 32) | 31),
    ));
    ctx.g.write::<BuildingInfos>().set_owner(buyer1_b, buyer1);
    let seller_pos = ctx.g.map().buildings.get(seller_b).unwrap().door_pos;
    let buyer1_pos = ctx.g.map().buildings.get(buyer1_b).unwrap().door_pos;

    spawn_parked_vehicle(&mut ctx.g, VehicleKind::Truck, seller_pos).expect("truck must spawn");

    {
        let mut m = ctx.g.write::<Market>();
        m.produce(seller, cereal(), 5);
        m.sell(seller, seller_pos.xy(), cereal(), 5, 0);
        m.buy(buyer1, buyer1_pos.xy(), cereal(), 5);
    }

    // Drive until the truck loads at the seller: Loading, retries counting.
    let mut ticks = 0;
    loop {
        ctx.tick();
        ticks += 1;
        let m = ctx.g.read::<Market>();
        if !m.dispatches().is_empty() && m.dispatches()[0].state == DispatchState::Loading {
            break;
        }
        assert!(ticks < 4000, "dispatch never reached Loading");
    }
    let truck = ctx.g.read::<Market>().dispatches()[0]
        .truck()
        .expect("a Loading dispatch holds its truck");

    // Past the retry bound: the dispatch must terminate (honest loss), and
    // the freed truck must be parked, not abandoned Driving.
    ctx.advance_ticks(600);
    {
        let m = ctx.g.read::<Market>();
        assert!(
            m.dispatches().is_empty(),
            "an unroutable Loading dispatch must still terminate"
        );
        let lost_qty: u32 = m
            .lost()
            .iter()
            .filter(|e| e.kind == cereal())
            .map(|e| e.qty)
            .sum();
        assert_eq!(lost_qty, 5, "the debited goods are an honest loss");
    }

    // Parking completes when the truck physically reaches its spot (the
    // park itinerary drives it there; arrival flips it to Parked and frees
    // the collider, sov-2c4). Poll for it — then pin both halves.
    let mut ticks = 0;
    loop {
        let parked = ctx
            .g
            .world()
            .vehicles
            .get(truck)
            .is_some_and(|ve| matches!(ve.vehicle.state, VehicleState::Parked(_)));
        if parked {
            break;
        }
        ctx.advance_ticks(50);
        ticks += 50;
        assert!(ticks < 3000, "the freed truck never parked");
    }
    let ve = ctx.g.world().vehicles.get(truck).expect("truck must exist");
    assert!(
        ve.collider.is_none(),
        "a parked truck must hold no TransportGrid collider"
    );

    // Second dispatch, same island as the seller: it must reserve and use
    // THAT truck — proving the freed truck is back in the pool AND usable.
    let buyer2_b = ctx.build_house_at(Vec2::new(150.0, 20.0));
    ctx.tick();
    let buyer2 = crate::SoulID::GoodsCompany(crate::world::CompanyID::from(
        slotmapd::KeyData::from_ffi((1 << 32) | 32),
    ));
    ctx.g.write::<BuildingInfos>().set_owner(buyer2_b, buyer2);
    let buyer2_pos = ctx.g.map().buildings.get(buyer2_b).unwrap().door_pos;
    {
        let mut m = ctx.g.write::<Market>();
        m.produce(seller, cereal(), 5);
        m.sell(seller, seller_pos.xy(), cereal(), 5, 0);
        m.buy(buyer2, buyer2_pos.xy(), cereal(), 5);
    }
    let mut ticks = 0;
    loop {
        ctx.tick();
        ticks += 1;
        let m = ctx.g.read::<Market>();
        if !m.dispatches().is_empty()
            && m.dispatches()[0].state == DispatchState::Loading
            && m.dispatches()[0].truck().is_some()
        {
            break;
        }
        assert!(ticks < 4000, "the second dispatch never loaded");
    }
    assert_eq!(
        ctx.g.read::<Market>().dispatches()[0].truck(),
        Some(truck),
        "the second dispatch must reuse the freed truck"
    );
}
