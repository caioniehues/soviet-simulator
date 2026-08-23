//! Conservation guard: nothing in the ledger should create or destroy units
//! of an item outside of `produce()` (source: production/extraction, or a
//! negative delta for consumption) and ext-trade (sink: export, source:
//! import). Everything else — a domestic match/dispatch, a soul being
//! removed mid-flight — must be a pure transfer.
//!
//! sov-ledger-invariant-test-8gg. Reproduces three verified double-spend/leak
//! sequences from the ledger audit; each must FAIL before the Stage 2 fixes
//! (sov-ledger-exttrade-cbh, sov-ledger-remove-3v7, sov-reserved-jobs-azr)
//! land, and pass after.

use super::*;

use super::hoarding::{build_company_at, drain_dispatches, mk_soul, setup_seller_buyer};
use crate::economy::{DispatchState, Market};
use crate::transportation::{spawn_parked_vehicle, VehicleKind};
use crate::world::HumanID;
use crate::SoulID;
use prototypes::ItemID;

fn mk_human(id: u64) -> SoulID {
    SoulID::Human(HumanID::from(slotmapd::KeyData::from_ffi(id)))
}

/// Σ(capital across all souls) + Σ(qty held by dispatches that have already
/// debited their seller but not yet credited their buyer — `Loading` and
/// `ToDestination`; see `Market::advance_dispatches`) for one item. A pure
/// domestic transfer moves value from a soul's capital into this in-flight
/// bucket and back out without changing the total, so it must stay constant
/// across any tick in which no `produce()` and no ext-trade ran.
fn total_qty(m: &Market, item: ItemID) -> i64 {
    let cap: i64 = m
        .inner()
        .get(&item)
        .map(|sm| sm.capital_map().values().map(|&c| c as i64).sum())
        .unwrap_or(0);
    let in_flight: i64 = m
        .dispatches()
        .iter()
        .filter(|d| {
            d.kind == item
                && matches!(
                    d.state,
                    DispatchState::Loading | DispatchState::ToDestination
                )
        })
        .map(|d| d.qty as i64)
        .sum();
    cap + in_flight
}

/// sov-ledger-exttrade-cbh: a domestic match reserves stock (`reserved`) for
/// a buyer without touching `capital`. Production immediately after calls
/// `sell_all`, which overwrites the sell order with the new full stock and
/// forgets the reservation (`market.rs:191`). The next ext-trade pass then
/// computes its surplus off the un-reserved order and oversells: the
/// domestic delivery plus the export exceed what was ever produced.
///
/// Uses the seller/buyer/"cereal" harness from `hoarding.rs` (proven not to
/// interfere with manually-driven capital) standing in for the audit's
/// flour-factory numbers; the arithmetic is identical (59 capital, 50 stock,
/// a 55-unit domestic match, a 10-unit production, a 19-unit oversold
/// export).
#[test]
fn scenario_ledger_exttrade_double_spend() {
    let mut ctx = TestCtx::new();
    let (seller, buyer, seller_pos, buyer_pos) = setup_seller_buyer(&mut ctx, 120.0);
    spawn_parked_vehicle(&mut ctx.g, VehicleKind::Truck, seller_pos).expect("truck must spawn");

    let cereal = ItemID::new("cereal");
    let ext = mk_soul((1 << 32) | 99); // stands in for `find_external`'s pick

    {
        let mut m = ctx.g.write::<Market>();
        m.produce(seller, cereal, 59);
        m.sell(seller, seller_pos.xy(), cereal, 59, 50);
        m.buy(buyer, buyer_pos.xy(), cereal, 55);

        // Pass 1: domestic match reserves 55 (capital untouched); the
        // remaining-order ext-trade surplus (4 - stock 50) is negative, skipped.
        // (`trades` also carries the harness's own background activity —
        // the house's auto-spawned resident taking the bakery's job opening,
        // the bakery importing its own flour — filter down to cereal.)
        let trades = m.make_trades(|_| Some(ext));
        let cereal_trades: Vec<_> = trades.iter().filter(|t| t.kind == cereal).collect();
        assert_eq!(
            cereal_trades.len(),
            1,
            "only the domestic match should trade cereal this pass"
        );
        assert_eq!(cereal_trades[0].qty, 55);
        assert_eq!(
            total_qty(&m, cereal),
            59,
            "a domestic match alone must not change the total"
        );
    }

    // Production (+10) is a declared source, so the total is allowed to grow
    // here. `sell_all` then overwrites the sell order with the full new
    // stock, forgetting the still-outstanding 55-unit reservation.
    let produced_total = {
        let mut m = ctx.g.write::<Market>();
        let after = m.produce(seller, cereal, 10);
        m.sell_all(seller, seller_pos.xy(), cereal, 50);
        after as i64
    };
    assert_eq!(produced_total, 69);

    // Pass 2: ext-trade surplus, buggy, used to compute off the un-reserved
    // 69 and oversell by 19 (should have been capped at 69 - 55 = 14 free
    // units, which is less than the 50-unit stock threshold, so a correct
    // market should export nothing here at all).
    let exported: i64 = {
        let mut m = ctx.g.write::<Market>();
        let trades = m.make_trades(|_| Some(ext));
        trades
            .iter()
            .filter(|t| t.kind == cereal)
            .map(|t| t.qty as i64)
            .sum()
    };

    assert!(
        drain_dispatches(&mut ctx, 4000),
        "domestic dispatch never completed"
    );

    let m = ctx.g.read::<Market>();
    let delivered = m.capital(buyer, cereal) as i64;
    assert_eq!(delivered, 55, "buyer must receive the full domestic match");
    assert!(
        m.capital(seller, cereal) >= 0,
        "seller capital must never go negative (would mean goods delivered/exported \
         that were never produced): {}",
        m.capital(seller, cereal)
    );
    assert!(
        delivered + exported <= produced_total,
        "conservation violated: {delivered} delivered + {exported} exported > {produced_total} \
         ever produced (seller capital ended at {})",
        m.capital(seller, cereal),
    );
}

/// sov-ledger-remove-3v7: `Market::remove` clears a soul's `sell_orders`,
/// `buy_orders` and `capital`, but not `reserved`/`requested`/`dispatches`.
/// A dispatch still naming the removed soul as seller survives; when its
/// truck eventually arrives, `Loading`'s debit (`market.rs:503`) resurrects
/// a negative capital row for a soul that no longer exists, and the buyer
/// still gets credited for goods nobody delivered.
#[test]
fn scenario_ledger_remove_leak() {
    let mut ctx = TestCtx::new();
    let (seller, buyer, seller_pos, buyer_pos) = setup_seller_buyer(&mut ctx, 120.0);
    spawn_parked_vehicle(&mut ctx.g, VehicleKind::Truck, seller_pos).expect("truck must spawn");

    let cereal = ItemID::new("cereal");
    {
        let mut m = ctx.g.write::<Market>();
        m.produce(seller, cereal, 10);
        m.sell(seller, seller_pos.xy(), cereal, 10, 0);
        m.buy(buyer, buyer_pos.xy(), cereal, 10);
    }

    ctx.tick(); // domestic match: reserved[seller] = 10, dispatch in ToSource

    {
        let m = ctx.g.read::<Market>();
        assert_eq!(m.dispatches().len(), 1);
        assert_eq!(m.dispatches()[0].state, DispatchState::ToSource);
    }

    ctx.g.write::<Market>().remove(seller); // seller "demolished" mid-flight

    assert!(drain_dispatches(&mut ctx, 4000), "dispatch never completed");

    let m = ctx.g.read::<Market>();
    assert!(
        !m.inner()[&cereal].capital_map().contains_key(&seller),
        "a removed soul must never reappear in the ledger, phantom row: {:?}",
        m.inner()[&cereal].capital_map().get(&seller)
    );
    assert_eq!(
        m.capital(buyer, cereal),
        0,
        "buyer must not be credited for goods nobody delivered"
    );
}

/// sov-reserved-jobs-azr: every domestic match reserves stock
/// (`market.rs:334-336`), but `job-opening` matches never get a dispatch
/// (`market.rs:363`) — the dispatch is the only thing that would later
/// debit capital or release the reservation. So a company's job-opening
/// capital never falls as its openings get filled.
#[test]
fn scenario_ledger_job_opening_reserve_leak() {
    let mut ctx = TestCtx::new();
    let bakery = prototypes::GoodsCompanyID::new("bakery").prototype();
    let b = build_company_at(&mut ctx, bakery, geom::Vec2::new(30.0, 20.0));
    ctx.tick(); // company_soul spawns on the next tick

    let seller = ctx
        .g
        .read::<crate::map_dynamic::BuildingInfos>()
        .owner(b)
        .unwrap();
    let job_opening = ItemID::new("job-opening");
    let pos = ctx.g.map().buildings.get(b).unwrap().door_pos.xy();

    let baseline = ctx.g.read::<Market>().capital(seller, job_opening);
    assert_eq!(
        baseline, 3,
        "bakery must post one job opening per worker slot (n_workers = 3)"
    );

    {
        let mut m = ctx.g.write::<Market>();
        for i in 0..baseline as u64 {
            m.buy(mk_human(i), pos, job_opening, 1);
        }
        let trades = m.make_trades(|_| None);
        assert_eq!(
            trades.len(),
            baseline as usize,
            "every opening should match"
        );
    }

    let m = ctx.g.read::<Market>();
    assert_eq!(
        total_qty(&m, job_opening),
        0,
        "job-opening capital must fall to 0 once every slot is filled, not stay reserved forever"
    );
}
