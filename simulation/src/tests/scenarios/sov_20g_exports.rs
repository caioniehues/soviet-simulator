//! sov-20g: the EXPORT half of external trade must be physical. After
//! sov-abs the import half rides a `Dispatch` out of the freight station,
//! but the export branch (`market.rs`) debited the seller at match time with
//! no dispatch — the dispatch loop iterated `all_trades[dispatch_start..]`
//! BEFORE the export branch pushed, so exports structurally could not get a
//! truck and goods teleported seller -> border in the same tick.
//!
//! The export must mirror the import leg: match reserves (no capital move),
//! a `Dispatch` drives seller -> border door, the seller is debited when the
//! truck loads and the border is credited when it unloads. Money semantics
//! are untouched (sov-7f7's territory): no money assertions here.
//!
//! Setup note: the border soul is a fabricated `SoulID::FreightStation` that
//! owns a plain house on the seller's road (same pattern `hoarding.rs` uses
//! for fabricated buyers), and the match is driven by a direct
//! `make_trades(|_| Some(ext))` call (same style as `ledger.rs`'s ext-trade
//! tests). The default freight station is demolished first, so the real
//! `market_update` closure finds no station and no background ext-trade can
//! ever compete for the single truck while the export drives to the door.

use super::*;

use super::hoarding::{build_company_at, drain_dispatches_of};
use super::inflation::remove_default_freight_station;
use crate::economy::{DispatchState, Market};
use crate::map_dynamic::BuildingInfos;
use crate::transportation::{spawn_parked_vehicle, VehicleKind};
use crate::{FreightStationID, SoulID};
use prototypes::{GoodsCompanyID, ItemID};

/// An export match must create a `Dispatch` to the border door (not teleport),
/// reserve the seller's stock at match time, debit the seller exactly once on
/// loading, and credit the border on arrival.
#[test]
fn sov_20g_export_is_physical_dispatch_to_border() {
    let mut ctx = TestCtx::new();
    remove_default_freight_station(&mut ctx);
    ctx.build_roads(&[Vec3::new(0.0, 0.0, 0.0), Vec3::new(300.0, 0.0, 0.0)]);

    // The seller is a real bakery company: `market_update`'s trade-application
    // loop unwraps `world.companies` for any GoodsCompany seller, so a
    // fabricated seller soul would panic there. Its recipe never touches
    // "cereal", so nothing but this scenario's own orders moves cereal.
    let seller_b = build_company_at(
        &mut ctx,
        GoodsCompanyID::new("bakery").prototype(),
        Vec2::new(30.0, 20.0),
    );
    // The border endpoint: a fabricated freight-station soul owning a house
    // on the same road, so `door_pos` resolves and the export truck has a
    // physical door to drive to.
    let ext_b = ctx.build_house_at(Vec2::new(120.0, 20.0));
    ctx.tick(); // company soul spawns; house resident spawns (owner overwritten next)
    let seller = ctx.g.read::<BuildingInfos>().owner(seller_b).unwrap();
    let ext = SoulID::FreightStation(FreightStationID::from(slotmapd::KeyData::from_ffi(
        (1 << 32) | 60,
    )));
    ctx.g.write::<BuildingInfos>().set_owner(ext_b, ext);
    let seller_pos = ctx.g.map().buildings.get(seller_b).unwrap().door_pos;
    drop(ctx.g.map());

    spawn_parked_vehicle(&mut ctx.g, VehicleKind::Truck, seller_pos).expect("truck must spawn");

    let cereal = ItemID::new("cereal");
    {
        let mut m = ctx.g.write::<Market>();
        m.produce(seller, cereal, 10);
        // qty 10 against stock 0: the whole 10 is surplus for export, and no
        // domestic buyer exists, so the border is the only possible taker.
        m.sell(seller, seller_pos.xy(), cereal, 10, 0);
        let trades: Vec<_> = m
            .make_trades(|_| Some(ext))
            .iter()
            .filter(|t| t.kind == cereal)
            .copied()
            .collect();
        assert_eq!(
            trades.len(),
            1,
            "the seller's surplus must match exactly one export trade"
        );
        assert_eq!(trades[0].seller.0, seller);
        assert_eq!(trades[0].buyer.0, ext);
        assert_eq!(trades[0].qty, 10);
    }

    {
        let m = ctx.g.read::<Market>();
        assert_eq!(
            m.capital(seller, cereal),
            10,
            "an exported good must NOT leave the seller's capital on the tick \
             it was matched -- that is a teleport across the border"
        );
        assert_eq!(
            m.reserved(seller, cereal),
            10,
            "matched export stock must be reserved (still physically at the \
             seller) until a truck loads it"
        );
        assert!(
            m.dispatches().iter().any(|d| {
                d.kind == cereal
                    && d.seller == seller
                    && d.buyer == ext
                    && d.state == DispatchState::ToSource
            }),
            "the export must be carried by a Dispatch from the seller to the \
             border door, like any other physical delivery: {:?}",
            m.dispatches()
        );
    }

    // ...and it must actually arrive, or the fix has merely replaced a
    // teleport with a permanent shortage. Only cereal dispatches are awaited:
    // the bakery's own background orders (flour, other kinds) serialize on
    // the same truck and never touch cereal capital.
    assert!(
        drain_dispatches_of(&mut ctx, 20000, cereal),
        "the export dispatch never completed"
    );

    let m = ctx.g.read::<Market>();
    assert_eq!(
        m.capital(seller, cereal),
        0,
        "the seller must end up debited exactly once (on loading)"
    );
    assert_eq!(
        m.capital(ext, cereal),
        10,
        "the border must be credited on arrival at its door"
    );
}
