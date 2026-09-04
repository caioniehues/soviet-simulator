//! sov-otw: `try_return_to_seller` is extracted and shared by
//! `Market::remove`'s dead-buyer arm and `advance_dispatches`' Loading
//! buyer-gone branch (a pure refactor of a conservation-critical path).
//!
//! The retry asymmetry is preserved deliberately: the ADVANCE path counts a
//! failed return against `MAX_RETURN_ROUTE_RETRIES` (bounded retries, then
//! honest loss), while the REMOVE path gets exactly one attempt and exits on
//! failure. This test pins the remove path THROUGH the helper: a Loading
//! dispatch whose buyer is removed with roads intact must become
//! `Returning` (not exit), drive home, and re-credit the seller.

use super::*;

use super::hoarding::{drain_dispatches, remove_soul, setup_seller_buyer};
use crate::economy::{DispatchState, Market};
use crate::transportation::{spawn_parked_vehicle, VehicleKind};
use prototypes::ItemID;

fn cereal() -> ItemID {
    ItemID::new("cereal")
}

/// sov-otw: removing the buyer of a Loading dispatch routes the goods home
/// through the shared helper — `Returning`, then re-credited on arrival.
#[test]
fn sov_otw_remove_dead_buyer_returns_to_seller() {
    let mut ctx = TestCtx::new();
    let (seller, buyer, seller_pos, buyer_pos) = setup_seller_buyer(&mut ctx, 120.0);
    spawn_parked_vehicle(&mut ctx.g, VehicleKind::Truck, seller_pos).expect("truck must spawn");

    {
        let mut m = ctx.g.write::<Market>();
        m.produce(seller, cereal(), 5);
        m.sell(seller, seller_pos.xy(), cereal(), 5, 0);
        m.buy(buyer, buyer_pos.xy(), cereal(), 5);
    }

    // Drive until the seller is debited and the dispatch is Loading.
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
    assert_eq!(
        ctx.g.read::<Market>().capital(seller, cereal()),
        0,
        "the seller must be debited before the return leg means anything"
    );

    // Remove the buyer with roads intact: one shared return attempt must
    // succeed, so the dispatch goes Returning instead of exiting.
    remove_soul(&mut ctx, buyer);

    {
        let m = ctx.g.read::<Market>();
        assert_eq!(m.dispatches().len(), 1);
        assert_eq!(
            m.dispatches()[0].state,
            DispatchState::Returning,
            "the remove path must share the advance path's return-to-seller fate"
        );
    }

    // The truck drives home and the seller is re-credited: conservation.
    assert!(
        drain_dispatches(&mut ctx, 8000),
        "the Returning dispatch never completed"
    );
    let m = ctx.g.read::<Market>();
    assert_eq!(
        m.capital(seller, cereal()),
        5,
        "goods driven home must be re-credited exactly once"
    );
    assert!(m.lost().is_empty(), "a completed return is not a loss");
}
