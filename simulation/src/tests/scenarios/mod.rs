//! Behavior scenarios: journeys and scenario cards from the requirements corpus,
//! each as a `#[test]` fn whose name carries its stable corpus ID
//! (`scenario_0082_...`, `journey_0001_...`).
//!
//! Sentinel set (planned, not yet implemented): six corpus IDs are nominated
//! for regression re-runs across iterations (JOURNEY-0001, SCENARIO-0009,
//! SCENARIO-0015, SCENARIO-0090, SCENARIO-0115, SCENARIO-0118), but no active
//! test fn name contains `sentinel` today (verified 2026-09-03), so
//! `cargo test -p simulation sentinel` matches zero tests. Until sentinel
//! tests land, run `cargo test -p simulation scenario_`
//! (verified 2026-09-04: 31 tests) for the scenario set. Scenarios not in
//! the sentinel set omit that tag.

use super::*;

mod hoarding;
mod inflation;
mod ledger;
mod recipe_provided;
mod retail;
mod validation;

/// Harness smoke test: proves the scenario harness works end to end
/// (roads, lot-independent building placement, multi-tick advance, and the
/// periodic determinism check all fire). Not a corpus-numbered scenario.
#[test]
fn scenario_harness_smoke() {
    let mut ctx = TestCtx::new();

    ctx.build_roads(&[Vec3::new(0.0, 0.0, 0.0), Vec3::new(100.0, 0.0, 0.0)]);
    let b = ctx.build_house_at(Vec2::new(50.0, 20.0));

    ctx.advance_ticks(5);

    assert!(ctx.g.map().buildings().contains_key(b));
}

mod sov_13h_retries;
mod sov_20g_exports;
mod sov_5ut_exits;
mod sov_7f7_settlement;
mod sov_91e_park;
mod sov_ahw;
mod sov_b70_border_age;
mod sov_bub_loss_sink;
mod sov_eix_throughput;
mod sov_nun_export_lane;
mod sov_otw_return;
mod sov_q5p_inflight;
mod sov_uo5_border_stock;
