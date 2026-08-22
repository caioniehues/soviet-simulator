//! Behavior scenarios: journeys and scenario cards from the requirements corpus,
//! each as a `#[test]` fn whose name carries its stable corpus ID
//! (`scenario_0082_...`, `journey_0001_...`).
//!
//! Sentinel set: a fixed group of six corpus IDs re-run for regression
//! detection across iterations (JOURNEY-0001, SCENARIO-0009, SCENARIO-0015,
//! SCENARIO-0090, SCENARIO-0115, SCENARIO-0118). Each sentinel test's fn name
//! must contain `sentinel` so the whole set can be run with:
//!
//!   cargo test -p simulation sentinel
//!
//! e.g. `fn sentinel_journey_0001_...()`. Individual scenarios/journeys not in
//! the sentinel set omit that tag.

use super::*;

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
