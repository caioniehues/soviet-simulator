//! Cross-domain target journeys, sentinel-promoted (sov-journey-sentinels-rxa).
//!
//! Each test below is a journey across market, logistics, and production
//! consumption in one run, binding canonical `REQ`/`SPEC`/`EVID` identities
//! (never archived numeric corpus IDs). Both tests run the standing pillar
//! assertions from `sentinel_pillars` at every phase, so the sentinel corpus
//! re-proves both pillars every iteration:
//!
//! * `sentinel_journey_produce_haul_deliver_conserves`
//!   binds `REQ-LOGISTICS-001` + `REQ-PRODUCTION-001` via
//!   `EVID-LOGISTICS-001` (`SPEC-LOGISTICS-002`, `SPEC-LOGISTICS-006`) and
//!   `EVID-PRODUCTION-002` (`SPEC-PRODUCTION-003`).
//! * `sentinel_journey_stall_recovers_without_loss`
//!   binds `REQ-LOGISTICS-001` + `REQ-PRODUCTION-001` via
//!   `EVID-LOGISTICS-002` (`SPEC-LOGISTICS-004`) and `EVID-PRODUCTION-004`
//!   (`SPEC-PRODUCTION-005`, `SPEC-PRODUCTION-008`).
//!
//! Promotion record: `SENTINEL-JOURNEY-HAUL-CONSERVES` and
//! `SENTINEL-JOURNEY-STALL-RECOVERS` in
//! `docs/plan/iterations/evidence/build_evidence.py`.

use super::*;
use super::hoarding::{drain_dispatches, setup_seller_buyer};
use super::inflation::remove_default_freight_station;
use super::sentinel_pillars::{
    assert_demand_visible_or_met, assert_no_domestic_money,
    assert_run_survives, PillarLedger,
};
use crate::economy::{DispatchState, Market};
use crate::transportation::{spawn_parked_vehicle, VehicleKind};
use prototypes::ItemID;

/// Cross-domain journey: produce → match → haul → deliver → consume.
///
/// `EVID-LOGISTICS-001`: the match moves nothing; pickup debits only the
/// seller; delivery credits only the buyer; the conserved total never moves.
/// `EVID-PRODUCTION-002`: the buyer's reported request stays distinguishable
/// from received and consumed quantity throughout.
#[test]
fn sentinel_journey_produce_haul_deliver_conserves() {
    let mut ctx = TestCtx::new();
    // No border for this domestic journey: without a freight station no
    // ext-trade runs, so the conserved total must hold exactly.
    remove_default_freight_station(&mut ctx);
    let (seller, buyer, seller_pos, buyer_pos) = setup_seller_buyer(&mut ctx, 120.0);
    spawn_parked_vehicle(&mut ctx.g, VehicleKind::Truck, seller_pos).expect("truck must spawn");

    let cereal = ItemID::new("cereal");
    {
        let mut m = ctx.g.write::<Market>();
        // Declared production source, predating the ledger snapshot.
        m.produce(seller, cereal, 10);
        m.set_requested(buyer, cereal, 10);
        m.sell(seller, seller_pos.xy(), cereal, 10, 0);
        m.buy(buyer, buyer_pos.xy(), cereal, 10);
    }
    let mut ledger = PillarLedger::watch(&ctx.g.read::<Market>(), &[cereal]);

    // The match happens on the next tick's market_update.
    ctx.tick();
    {
        let m = ctx.g.read::<Market>();
        assert_eq!(m.dispatches().len(), 1, "match must create one haul");
        assert_eq!(m.capital(seller, cereal), 10, "match must not move seller stock");
        assert_eq!(m.capital(buyer, cereal), 0, "match must not move buyer stock");
        assert_eq!(
            m.requested(buyer, cereal),
            Some(10),
            "reported request stays distinguishable from receipt"
        );
        ledger.assert_no_teleport(&m, "match");
        assert_no_domestic_money(&m, "match");
        assert_demand_visible_or_met(&m, buyer, cereal, 10, "match");
    }

    // Drive until the truck has loaded (seller debited, goods on the truck).
    let mut ticks = 0;
    loop {
        ctx.tick();
        ticks += 1;
        let m = ctx.g.read::<Market>();
        if !m.dispatches().is_empty()
            && matches!(
                m.dispatches()[0].state,
                DispatchState::Loading | DispatchState::ToDestination
            )
        {
            break;
        }
        assert!(ticks < 4000, "dispatch never reached Loading");
    }
    {
        let m = ctx.g.read::<Market>();
        assert_eq!(m.capital(seller, cereal), 0, "pickup debits the seller");
        assert_eq!(m.capital(buyer, cereal), 0, "delivery has not credited the buyer yet");
        ledger.assert_no_teleport(&m, "pickup");
        assert_no_domestic_money(&m, "pickup");
    }

    assert!(
        drain_dispatches(&mut ctx, 4000),
        "dispatch never completed with a truck available"
    );
    {
        let m = ctx.g.read::<Market>();
        assert_eq!(m.capital(seller, cereal), 0, "seller stays debited");
        assert_eq!(m.capital(buyer, cereal), 10, "delivery credits the buyer");
        ledger.assert_no_teleport(&m, "delivery");
        assert_no_domestic_money(&m, "delivery");
    }

    // Consumption is external to the haul: a declared sink, not a teleport.
    {
        let mut m = ctx.g.write::<Market>();
        m.produce(buyer, cereal, -4);
        ledger.declare(cereal, -4);
    }
    {
        let m = ctx.g.read::<Market>();
        assert_eq!(m.capital(buyer, cereal), 6, "consumption debits on-hand stock");
        assert_eq!(
            m.requested(buyer, cereal),
            Some(10),
            "consumed quantity stays distinguishable from the reported request"
        );
        ledger.assert_no_teleport(&m, "consumption");
    }

    assert_run_survives(&mut ctx, 50, "aftermath");
    ledger.assert_no_teleport(&ctx.g.read::<Market>(), "aftermath");
}

/// Cross-domain journey: stall without a truck, then recover.
///
/// `EVID-LOGISTICS-002`: a missing truck yields a visible recoverable stalled
/// job — still `ToSource`, still holding its truck-less reservation, losing
/// nothing — and the job completes once a truck exists.
/// `EVID-PRODUCTION-004`: the starved buyer stays registered with its demand
/// visible and is granted neither stock nor a bypass while short.
#[test]
fn sentinel_journey_stall_recovers_without_loss() {
    let mut ctx = TestCtx::new();
    remove_default_freight_station(&mut ctx);
    let (seller, buyer, seller_pos, buyer_pos) = setup_seller_buyer(&mut ctx, 120.0);
    // Deliberately no truck: the haul must stall visibly, not vanish.

    let cereal = ItemID::new("cereal");
    {
        let mut m = ctx.g.write::<Market>();
        m.produce(seller, cereal, 5);
        m.set_requested(buyer, cereal, 5);
        m.sell(seller, seller_pos.xy(), cereal, 5, 0);
        m.buy(buyer, buyer_pos.xy(), cereal, 5);
    }
    let ledger = PillarLedger::watch(&ctx.g.read::<Market>(), &[cereal]);

    ctx.tick();
    {
        let m = ctx.g.read::<Market>();
        assert_eq!(m.dispatches().len(), 1, "match must create one haul");
        assert_eq!(m.dispatches()[0].state, DispatchState::ToSource);
    }

    // Stall: the run keeps advancing with no truck, losing nothing.
    assert_run_survives(&mut ctx, 500, "stall");
    {
        let m = ctx.g.read::<Market>();
        assert_eq!(m.dispatches().len(), 1, "stalled haul must not be deleted");
        assert_eq!(
            m.dispatches()[0].state,
            DispatchState::ToSource,
            "haul must not progress without a truck"
        );
        assert!(
            m.dispatches()[0].truck().is_none(),
            "stalled haul visibly holds no truck"
        );
        assert_eq!(m.capital(seller, cereal), 5, "seller must never be debited");
        assert_eq!(m.capital(buyer, cereal), 0, "shortfall grants no stock");
        assert_demand_visible_or_met(&m, buyer, cereal, 5, "stall");
        ledger.assert_no_teleport(&m, "stall");
        assert_no_domestic_money(&m, "stall");
    }

    // Recovery: a truck arrives, the same haul completes.
    spawn_parked_vehicle(&mut ctx.g, VehicleKind::Truck, seller_pos).expect("truck must spawn");
    assert!(
        drain_dispatches(&mut ctx, 4000),
        "stalled haul must recover once a truck exists"
    );
    {
        let m = ctx.g.read::<Market>();
        assert_eq!(m.capital(seller, cereal), 0, "seller ends up debited");
        assert_eq!(m.capital(buyer, cereal), 5, "buyer ends up credited");
        ledger.assert_no_teleport(&m, "recovery");
        assert_no_domestic_money(&m, "recovery");
        assert_demand_visible_or_met(&m, buyer, cereal, 5, "recovery");
    }

    assert_run_survives(&mut ctx, 50, "aftermath");
}
