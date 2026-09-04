//! sov-nun: the export side gets its own lane-UNFILTERED border lookup.
//!
//! `find_external` was one closure used by BOTH the import block and the
//! seller-surplus export block, with a `DISPATCH_LANE_CUTOFF` reachability
//! filter. Imports need a driving lane (a truck must run from the station
//! door); exports ride a dispatch the SELLER's truck drives to the border
//! door and need no lane at the station end. Gating them on one was
//! over-application: on a fresh map (whose only station has no driving lane
//! in range) a new city could not export at all until the player laid road
//! near the station.
//!
//! One test asserting both halves on the same map: the export succeeds while
//! the import correctly refuses.

use super::*;

use super::hoarding::build_company_at;
use crate::economy::{DispatchState, Market};
use crate::map_dynamic::BuildingInfos;
use crate::world::CompanyID;
use prototypes::{GoodsCompanyID, ItemID};

fn cereal() -> ItemID {
    ItemID::new("cereal")
}

/// sov-nun: on a map whose only freight station has no driving lane in
/// range, an export still matches (unfiltered lookup) while an import on the
/// same map still refuses (lane filter kept).
#[test]
fn sov_nun_export_without_lane_import_refuses() {
    // Default city: the START_COMMANDS station sits ~4km out with no driving
    // lane at its door — exactly the failing shape. No roads are built on
    // purpose: the seller below is real but its surplus can only go abroad.
    let mut ctx = TestCtx::new();

    // Real bakery seller (the trade-application loop unwraps GoodsCompany
    // sellers): produce + surplus by hand, since it is unstaffed.
    let seller_b = build_company_at(
        &mut ctx,
        GoodsCompanyID::new("bakery").prototype(),
        Vec2::new(30.0, 20.0),
    );
    // Fabricated buyer owning a house (hoarding.rs pattern): Market only
    // needs a SoulID + door position for it.
    let buyer_b = ctx.build_house_at(Vec2::new(200.0, 20.0));
    ctx.tick(); // company soul spawns; house resident spawns (owner overwritten next)
    let seller = ctx.g.read::<BuildingInfos>().owner(seller_b).unwrap();
    let buyer = crate::SoulID::GoodsCompany(CompanyID::from(slotmapd::KeyData::from_ffi(
        (1 << 32) | 41,
    )));
    ctx.g.write::<BuildingInfos>().set_owner(buyer_b, buyer);
    let seller_pos = ctx.g.map().buildings.get(seller_b).unwrap().door_pos;
    let buyer_pos = ctx.g.map().buildings.get(buyer_b).unwrap().door_pos;

    {
        let mut m = ctx.g.write::<Market>();
        m.produce(seller, cereal(), 10);
        m.sell(seller, seller_pos.xy(), cereal(), 10, 0);
        // 15 exceeds the seller's 10: no domestic match may steal either
        // half — the export must clear externally, the import must refuse.
        m.buy(buyer, buyer_pos.xy(), cereal(), 15);
    }

    ctx.advance_ticks(3);
    let m = ctx.g.read::<Market>();
    assert!(
        m.dispatches().iter().any(|d| {
            d.kind == cereal()
                && d.seller == seller
                && matches!(d.buyer, crate::SoulID::FreightStation(_))
        }),
        "the export must succeed with no driving lane at the station: {:?}",
        m.dispatches()
    );
    // Import half: same map, still correctly refused.
    assert!(
        !m.dispatches().iter().any(|d| {
            d.kind == cereal()
                && d.buyer == buyer
                && matches!(d.seller, crate::SoulID::FreightStation(_))
        }),
        "the import must still refuse with no driving lane at the station"
    );
    assert_eq!(
        m.inner()[&cereal()]
            .buy_order(buyer)
            .map(|o| o.qty),
        Some(15),
        "the refused import demand must survive for the next domestic match"
    );
    assert!(
        m.dispatches()
            .iter()
            .filter(|d| d.kind == cereal())
            .all(|d| d.state == DispatchState::ToSource),
        "with no truck spawned nothing may have loaded"
    );
}
