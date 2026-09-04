//! Standing pillar assertions for the sentinel corpus (sov-pillar-assertions-ugb).
//!
//! Both design pillars were violated in shipped code with no test noticing
//! (the ext-trade buy block granting free goods, found only by a manual
//! conservation trace). These helpers are the cheap runtime form of each
//! pillar, called by every `sentinel_*` journey test so the sentinel corpus
//! re-runs them every iteration:
//!
//! * **Nothing teleports** (`PillarLedger`): no capital increases without a
//!   declared production/import source, no decreases without a declared
//!   consumption/export sink. Generalizes `ledger::total_qty` beyond one
//!   hand-built item: totals run per item over *every* item in the market,
//!   summing capital across *all* souls plus the in-flight dispatch bucket
//!   (debited-not-yet-credited: `Loading`, `ToDestination`, `Returning`).
//! * **Never game over** (`assert_run_survives`,
//!   `assert_demand_visible_or_met`): the run must keep advancing ticks after
//!   the adverse event, and unmet need must stay visible (open order,
//!   recorded request, or live dispatch) or be met by delivery — never
//!   silently deleted, never a terminated run.
//! * **Clearing by queue, never by price** (`assert_no_domestic_money`):
//!   domestic dispatch legs carry `Money::ZERO`; only a completed physical
//!   customs clearance settles border money (ADR-0003).

use super::*;
use crate::economy::{DispatchState, Market};
use crate::SoulID;
use prototypes::{ItemID, Money};
use std::collections::BTreeMap;

pub(super) fn pillar_market_totals(m: &Market) -> BTreeMap<ItemID, i64> {
    let mut totals = BTreeMap::new();
    for (item, sm) in m.iter() {
        let cap: i64 = sm.capital_map().values().map(|&c| c as i64).sum();
        totals.insert(*item, cap);
    }
    for d in m.dispatches() {
        if matches!(
            d.state,
            DispatchState::Loading | DispatchState::ToDestination | DispatchState::Returning
        ) {
            *totals.entry(d.kind).or_insert(0) += d.qty as i64;
        }
    }
    totals
}

/// Nothing-teleports guard: snapshot the conserved totals for the watched
/// items, declare every legitimate source/sink delta as the scenario drives
/// it, and assert the books balance at each phase boundary.
pub(super) struct PillarLedger {
    base: BTreeMap<ItemID, i64>,
    declared: BTreeMap<ItemID, i64>,
}

impl PillarLedger {
    /// Snapshot the conserved totals for `items`. Production/consumption that
    /// already ran stays in the base; only deltas driven *after* the snapshot
    /// need `declare`.
    pub(super) fn watch(m: &Market, items: &[ItemID]) -> Self {
        let all = pillar_market_totals(m);
        let base = items
            .iter()
            .map(|item| (*item, all.get(item).copied().unwrap_or(0)))
            .collect();
        Self {
            base,
            declared: BTreeMap::new(),
        }
    }

    /// Record a declared source (`produce`/import, positive) or sink
    /// (consumption/export, negative) driven after the snapshot.
    pub(super) fn declare(&mut self, item: ItemID, delta: i32) {
        *self.declared.entry(item).or_insert(0) += delta as i64;
    }

    /// The standing nothing-teleports assertion: current totals must equal
    /// snapshot plus declared sources/sinks, for every watched item.
    pub(super) fn assert_no_teleport(&self, m: &Market, phase: &str) {
        let now = pillar_market_totals(m);
        for item in self.base.keys().chain(self.declared.keys()) {
            let expected =
                self.base.get(item).copied().unwrap_or(0) + self.declared.get(item).copied().unwrap_or(0);
            let got = now.get(item).copied().unwrap_or(0);
            assert_eq!(
                got, expected,
                "pillar nothing-teleports violated at {phase}: {item:?} totals {got}, expected {expected}"
            );
        }
    }
}

/// Never-game-over assertion, run half: the simulation must still advance.
/// A code path that terminates the run (or hangs it) fails here by the tick
/// counter not moving exactly `ticks`.
pub(super) fn assert_run_survives(ctx: &mut TestCtx, ticks: u32, phase: &str) {
    let before = ctx.g.get_tick();
    ctx.advance_ticks(ticks);
    assert_eq!(
        ctx.g.get_tick(),
        before + ticks as u64,
        "pillar never-game-over violated at {phase}: run stopped advancing"
    );
}

/// Never-game-over assertion, degrade half: unmet need must stay visible or
/// be met — an open buy order, a recorded request, or a live dispatch for
/// the buyer, or capital at/above the wanted quantity. Need that is neither
/// visible nor met was silently deleted: going without, not gone.
pub(super) fn assert_demand_visible_or_met(
    m: &Market,
    buyer: SoulID,
    item: ItemID,
    want: i32,
    phase: &str,
) {
    let met = m.capital(buyer, item) >= want;
    let order_open = m
        .inner()
        .get(&item)
        .and_then(|sm| sm.buy_order(buyer))
        .is_some();
    let visible = order_open
        || m.requested(buyer, item).is_some()
        || m.dispatches().iter().any(|d| d.buyer == buyer && d.kind == item);
    assert!(
        met || visible,
        "pillar never-game-over violated at {phase}: unmet need for {buyer:?}/{item:?} is neither visible nor met"
    );
}

/// Clearing-by-queue assertion: no domestic dispatch leg moves money. Border
/// money settles only at completed physical customs clearance (ADR-0003), so
/// every dispatch in these border-free journeys must carry `Money::ZERO`.
pub(super) fn assert_no_domestic_money(m: &Market, phase: &str) {
    for d in m.dispatches() {
        assert_eq!(
            d.money_delta, Money::ZERO,
            "clearing-by-queue violated at {phase}: dispatch {d:?} moves money before customs clearance"
        );
    }
}

/// In-flight bucket membership shared with `ledger::total_qty`: only legs
/// that already debited the seller and have not yet credited the buyer.
trait InFlight {
    fn kind_counted_in_flight(&self) -> bool;
}

impl InFlight for crate::economy::Dispatch {
    fn kind_counted_in_flight(&self) -> bool {
        matches!(
            self.state,
            DispatchState::Loading | DispatchState::ToDestination | DispatchState::Returning
        )
    }
}
