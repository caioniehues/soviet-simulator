//! sov-13h: `MAX_RETURN_ROUTE_RETRIES` survives transient road failure.
//!
//! The bound was 20 ticks — 0.4 seconds of wall clock at 50 ticks/s —
//! comfortably inside a single player road-drag, so bulldozing one segment
//! silently deleted every city-wide load mid-`Loading`. It is now 300 ticks
//! (6 seconds at 50 ticks/s, one game-minute): transient failures ride
//! through, while the PERMANENT trigger (a demolished building) still
//! terminates — and the old `retries + 1 >= MAX` fired after 19 attempts
//! rather than 20, corrected alongside.
//!
//! This test drives a Loading dispatch into route failure lasting longer
//! than the OLD bound and asserts it is still alive — then keeps the failure
//! permanent and asserts the bound still ends it (finite, not infinite).

use super::*;

use super::hoarding::build_company_at;
use super::inflation::remove_default_freight_station;
use crate::economy::{DispatchState, Market};
use crate::map_dynamic::BuildingInfos;
use crate::transportation::{spawn_parked_vehicle, VehicleKind};
use crate::world::CompanyID;
use prototypes::{GoodsCompanyID, ItemID};

fn cereal() -> ItemID {
    ItemID::new("cereal")
}

/// sov-13h: a dispatch survives a route failure past the old 20-tick bound,
/// then still terminates on a permanently unroutable road.
#[test]
fn sov_13h_dispatch_survives_past_old_bound_then_terminates() {
    let mut ctx = TestCtx::new();
    remove_default_freight_station(&mut ctx);

    // Two road islands, never connected (as in sov-91e): the seller and
    // truck on A, the buyer on B. The match ignores connectivity; the road
    // never heals, so the failure outlasts any transient drag.
    ctx.build_roads(&[Vec3::new(0.0, 0.0, 0.0), Vec3::new(300.0, 0.0, 0.0)]);
    ctx.build_roads(&[Vec3::new(600.0, 0.0, 0.0), Vec3::new(900.0, 0.0, 0.0)]);
    let seller_b = build_company_at(
        &mut ctx,
        GoodsCompanyID::new("bakery").prototype(),
        Vec2::new(30.0, 20.0),
    );
    let buyer_b = ctx.build_house_at(Vec2::new(750.0, 20.0));
    ctx.tick(); // company soul spawns; house resident spawns (owner overwritten next)

    let seller = ctx.g.read::<BuildingInfos>().owner(seller_b).unwrap();
    let buyer = crate::SoulID::GoodsCompany(CompanyID::from(slotmapd::KeyData::from_ffi(
        (1 << 32) | 51,
    )));
    ctx.g.write::<BuildingInfos>().set_owner(buyer_b, buyer);
    let seller_pos = ctx.g.map().buildings.get(seller_b).unwrap().door_pos;
    let buyer_pos = ctx.g.map().buildings.get(buyer_b).unwrap().door_pos;

    spawn_parked_vehicle(&mut ctx.g, VehicleKind::Truck, seller_pos).expect("truck must spawn");

    {
        let mut m = ctx.g.write::<Market>();
        m.produce(seller, cereal(), 5);
        m.sell(seller, seller_pos.xy(), cereal(), 5, 0);
        m.buy(buyer, buyer_pos.xy(), cereal(), 5);
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

    // Three times the old bound of route failure: the dispatch must still
    // be Loading with its truck, not an honest loss.
    ctx.advance_ticks(60);
    {
        let m = ctx.g.read::<Market>();
        assert_eq!(
            m.dispatches().len(),
            1,
            "a 60-tick outage must not kill the dispatch (old bound was 20)"
        );
        assert_eq!(
            m.dispatches()[0].state,
            DispatchState::Loading,
            "the dispatch must still be working the route, not lost"
        );
        assert!(
            m.lost().is_empty(),
            "nothing may be deleted while retries remain"
        );
    }

    // Permanent failure still terminates: the bound is generous, not
    // infinite — the dispatch exits as an honest loss and frees its truck.
    ctx.advance_ticks(600);
    {
        let m = ctx.g.read::<Market>();
        assert!(
            m.dispatches().is_empty(),
            "a permanently unroutable dispatch must still terminate"
        );
        let lost_qty: u32 = m
            .lost()
            .iter()
            .filter(|e| e.kind == cereal())
            .map(|e| e.qty)
            .sum();
        assert_eq!(lost_qty, 5, "the terminated goods are honestly lost");
    }
}
