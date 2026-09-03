//! sov-bub: the bounded Loading/Returning route-failure sink is named and
//! visible. A severed-road scenario must record the deleted goods in the
//! `Lost` ledger (item + qty) AND still delete them (deletion behavior,
//! bounds and retry counts unchanged — the retail bounded-loss tests stay
//! green alongside this one).

use super::hoarding::{drain_dispatches, setup_seller_buyer};
use super::*;

use crate::economy::{DispatchState, Market};
use crate::map_dynamic::BuildingInfos;
use crate::transportation::{spawn_parked_vehicle, VehicleKind};
use prototypes::ItemID;

/// Sever the road home after the buyer's building is demolished: the goods
/// are honestly lost (already debited, never returned), and that loss must
/// land in `Lost` instead of only a log warning.
#[test]
fn scenario_severed_road_loss_is_recorded_and_goods_stay_gone() {
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

    // Drive until the dispatch has debited the seller and entered Loading.
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

    // Demolish the buyer's building, then sever every road so the truck
    // can never route back to the seller.
    let buyer_building = ctx
        .g
        .read::<BuildingInfos>()
        .building_owned_by(buyer)
        .unwrap();
    ctx.g.map_mut().remove_building(buyer_building);
    let road_ids: Vec<_> = ctx.g.map().roads().keys().collect();
    for r in road_ids {
        ctx.g.map_mut().remove_road(r);
    }

    assert!(
        drain_dispatches(&mut ctx, 6000),
        "a Returning dispatch with no route home must still terminate, not wedge forever"
    );

    // Deletion preserved: the goods are gone from both sides, not refunded.
    let m = ctx.g.read::<Market>();
    assert_eq!(
        m.capital(seller, cereal),
        0,
        "no route home means no re-credit -- an honest loss, not a teleport"
    );
    assert_eq!(m.capital(buyer, cereal), 0);

    // ... and the loss is now named: exactly the deleted qty sits in Lost.
    let lost_qty: u32 = m
        .lost()
        .iter()
        .filter(|e| e.kind == cereal)
        .map(|e| e.qty)
        .sum();
    assert_eq!(
        lost_qty, 5,
        "the severed-road deletion must record the full deleted qty in Lost"
    );
}
