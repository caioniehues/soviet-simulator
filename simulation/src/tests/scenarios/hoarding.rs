//! Proves the physical half of ITER-0000's walking skeleton: goods dispatches
//! ride a real truck over real roads, not a blind tick countdown.
//!
//! STORY-0149 AC-4: a matched trade must not move stock on its own (gated by
//! `Loading`/`Unloading`, not by the match itself), and delivery must not
//! happen at all without an actual truck to carry it.

use super::*;

use crate::economy::{DispatchState, Market};
use crate::transportation::{spawn_parked_vehicle, VehicleKind};
use crate::world::CompanyID;
use crate::SoulID;
use prototypes::{GoodsCompanyID, ItemID};

fn mk_soul(id: u64) -> SoulID {
    SoulID::GoodsCompany(CompanyID::from(slotmapd::KeyData::from_ffi(id)))
}

/// Builds a real goods-company building from a loaded prototype (needed for
/// the seller: `economy::market_update`'s trade-application loop unwraps
/// `world.companies` for any GoodsCompany *seller*, so a fabricated SoulID
/// there panics. Mirrors `recipe_provided::build_company_at`.
fn build_company_at(
    ctx: &mut TestCtx,
    proto: &prototypes::GoodsCompanyPrototype,
    p: Vec2,
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
            None,
        )
        .unwrap();
    ctx.g.write::<BuildingInfos>().insert(b);
    b
}

/// Builds a seller/buyer building pair on a real road. The seller is a real
/// "bakery" GoodsCompany (its own recipe never touches "cereal", so it won't
/// interfere with capital we drive manually); the buyer is a fabricated
/// SoulID owning a plain house — `Market` only needs a `SoulID` + door
/// position via `BuildingInfos`, and the buyer side of the trade-application
/// loop never unwraps a missing company.
fn setup_seller_buyer(ctx: &mut TestCtx, buyer_x: f32) -> (SoulID, SoulID, Vec3, Vec3) {
    ctx.build_roads(&[
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(buyer_x + 60.0, 0.0, 0.0),
    ]);
    let bakery = GoodsCompanyID::new("bakery").prototype();
    let seller_b = build_company_at(ctx, bakery, Vec2::new(30.0, 20.0));
    let buyer_b = ctx.build_house_at(Vec2::new(buyer_x, 20.0));

    ctx.tick(); // company_soul spawns on the next tick, like houses

    let seller = ctx.g.read::<BuildingInfos>().owner(seller_b).unwrap();
    let buyer = mk_soul((1 << 32) | 2);
    ctx.g.write::<BuildingInfos>().set_owner(buyer_b, buyer);

    let map = ctx.g.map();
    let seller_pos = map.buildings.get(seller_b).unwrap().door_pos;
    let buyer_pos = map.buildings.get(buyer_b).unwrap().door_pos;
    drop(map);

    (seller, buyer, seller_pos, buyer_pos)
}

/// Ticks until every in-flight dispatch has drained, or `max_ticks` is spent.
/// Returns whether it drained.
fn drain_dispatches(ctx: &mut TestCtx, max_ticks: u32) -> bool {
    let mut spent = 0;
    while spent < max_ticks {
        let chunk = (max_ticks - spent).min(50);
        ctx.advance_ticks(chunk);
        spent += chunk;
        if ctx.g.read::<Market>().dispatches().is_empty() {
            return true;
        }
    }
    false
}

/// SCENARIO-0082: a matched trade must not move stock on its own. Stock only
/// leaves the seller when the truck actually arrives and enters `Loading`,
/// and only reaches the buyer when it arrives and enters `Unloading`. Proves
/// the truck is load-bearing, not decorative: revert the match-time
/// reservation-only change in `make_trades` back to an immediate capital
/// transfer and this test fails at the "match must not move seller/buyer
/// stock" assertions.
#[test]
fn scenario_0082_dispatch_gates_stock_not_match() {
    let mut ctx = TestCtx::new();
    let (seller, buyer, seller_pos, buyer_pos) = setup_seller_buyer(&mut ctx, 120.0);

    spawn_parked_vehicle(&mut ctx.g, VehicleKind::Truck, seller_pos).expect("truck must spawn");

    let cereal = ItemID::new("cereal");
    {
        let mut m = ctx.g.write::<Market>();
        m.produce(seller, cereal, 5);
        m.sell(seller, seller_pos.xy(), cereal, 5, 0);
        m.buy(buyer, buyer_pos.xy(), cereal, 5);
    }

    // The match happens on the next tick's market_update.
    ctx.tick();

    {
        let m = ctx.g.read::<Market>();
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
    }

    assert!(
        drain_dispatches(&mut ctx, 4000),
        "dispatch never completed with a truck available"
    );

    let m = ctx.g.read::<Market>();
    assert_eq!(m.capital(seller, cereal), 0, "seller must end up debited");
    assert_eq!(m.capital(buyer, cereal), 5, "buyer must end up credited");
}

/// SCENARIO-0083: the proof that the truck is real, not a state machine that
/// completes on its own. With zero trucks available, the dispatch must sit in
/// `ToSource` forever: no debit, no credit, goods never move. Deleting the
/// truck spawn from `scenario_0082` above turns it into exactly this test.
#[test]
fn scenario_0083_zero_trucks_blocks_delivery() {
    let mut ctx = TestCtx::new();
    let (seller, buyer, seller_pos, buyer_pos) = setup_seller_buyer(&mut ctx, 120.0);

    // Deliberately no truck spawned.

    let cereal = ItemID::new("cereal");
    {
        let mut m = ctx.g.write::<Market>();
        m.produce(seller, cereal, 5);
        m.sell(seller, seller_pos.xy(), cereal, 5, 0);
        m.buy(buyer, buyer_pos.xy(), cereal, 5);
    }

    ctx.tick();

    {
        let m = ctx.g.read::<Market>();
        assert_eq!(m.dispatches().len(), 1);
        assert_eq!(m.dispatches()[0].state, DispatchState::ToSource);
    }

    ctx.advance_ticks(1000);

    let m = ctx.g.read::<Market>();
    assert_eq!(
        m.dispatches().len(),
        1,
        "dispatch must still be waiting for a truck"
    );
    assert_eq!(
        m.dispatches()[0].state,
        DispatchState::ToSource,
        "dispatch must not progress without a truck"
    );
    assert_eq!(m.capital(seller, cereal), 5, "seller must never be debited");
    assert_eq!(m.capital(buyer, cereal), 0, "buyer must never be credited");
}

/// SCENARIO-0151: two otherwise-identical companies buy the same item, one
/// honest (requests exactly what its recipe consumes) and one inflated
/// (requests more than it consumes). Only the inflated one should accumulate
/// surplus stock over several delivery cycles. Both buyers share a single
/// truck, so deliveries serialize through the `Dispatcher`.
#[test]
fn scenario_0151_inflated_request_hoards_honest_does_not() {
    let mut ctx = TestCtx::new();

    ctx.build_roads(&[Vec3::new(0.0, 0.0, 0.0), Vec3::new(270.0, 0.0, 0.0)]);
    let bakery = GoodsCompanyID::new("bakery").prototype();
    let seller_b = build_company_at(&mut ctx, bakery, Vec2::new(30.0, 20.0));
    let honest_b = ctx.build_house_at(Vec2::new(120.0, 20.0));
    let inflated_b = ctx.build_house_at(Vec2::new(210.0, 20.0));

    ctx.tick(); // company_soul spawns on the next tick, like houses

    let seller = ctx.g.read::<BuildingInfos>().owner(seller_b).unwrap();
    let honest = mk_soul((1 << 32) | 11);
    let inflated = mk_soul((1 << 32) | 12);

    {
        let mut binfos = ctx.g.write::<BuildingInfos>();
        binfos.set_owner(honest_b, honest);
        binfos.set_owner(inflated_b, inflated);
    }

    let map = ctx.g.map();
    let seller_pos = map.buildings.get(seller_b).unwrap().door_pos;
    let honest_pos = map.buildings.get(honest_b).unwrap().door_pos;
    let inflated_pos = map.buildings.get(inflated_b).unwrap().door_pos;
    drop(map);

    spawn_parked_vehicle(&mut ctx.g, VehicleKind::Truck, seller_pos).expect("truck must spawn");

    let cereal = ItemID::new("cereal");

    const TRUE_CONSUMPTION: u32 = 2;
    const INFLATED_REQUEST: u32 = 5;

    {
        let mut m = ctx.g.write::<Market>();
        m.produce(seller, cereal, 1000);
        m.sell(seller, seller_pos.xy(), cereal, 1000, 0);
        m.set_requested(honest, cereal, TRUE_CONSUMPTION);
        m.set_requested(inflated, cereal, INFLATED_REQUEST);
    }

    for _ in 0..3 {
        {
            let mut m = ctx.g.write::<Market>();
            // Both consume exactly what their recipe needs, same as `recipe_act`.
            let honest_consumed = m.capital(honest, cereal).min(TRUE_CONSUMPTION as i32);
            m.produce(honest, cereal, -honest_consumed);
            let inflated_consumed = m.capital(inflated, cereal).min(TRUE_CONSUMPTION as i32);
            m.produce(inflated, cereal, -inflated_consumed);

            // Both re-request up to their reported (possibly inflated)
            // quantity, same as `recipe_init`/`recipe_act`.
            let hq = m.requested(honest, cereal).unwrap();
            m.buy_until(honest, honest_pos.xy(), cereal, hq);
            let iq = m.requested(inflated, cereal).unwrap();
            m.buy_until(inflated, inflated_pos.xy(), cereal, iq);
        }

        // Two dispatches may be created (one per buyer); only one truck
        // exists, so they serialize through the Dispatcher.
        assert!(
            drain_dispatches(&mut ctx, 8000),
            "deliveries never drained within budget"
        );
    }

    let m = ctx.g.read::<Market>();
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
