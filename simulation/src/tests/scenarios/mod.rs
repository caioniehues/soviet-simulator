//! Behavior scenarios: journeys and scenario cards from the requirements corpus,
//! each as a `#[test]` fn whose name carries its stable corpus ID
//! (`scenario_0082_...`, `journey_0001_...`).
//!
//! Sentinel set: `sentinel_*` tests re-run every iteration as the regression
//! corpus. The substrate sentinels (`scenario_0082/0083/0151`, promoted in
//! `docs/plan/iterations/evidence/build_evidence.py`) plus the cross-domain
//! target journeys (`sentinel_journey_*`, same promotion record) all run the
//! standing pillar assertions in `sentinel_pillars`. Runnable corpus command:
//! `cargo test -p simulation sentinel -- --test-threads=1`.
//! Scenarios not in the sentinel set omit the `sentinel_` prefix.

use super::*;

mod hoarding;
mod inflation;
mod ledger;
mod recipe_provided;
mod retail;
mod sentinel_journey;
mod sentinel_pillars;
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
mod sov_2uv_station_vehicles;
