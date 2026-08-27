---
name: test-exact-name-module-path
description: A brief that mandates `cargo test -p simulation <name> -- --exact` forces an integration test file; a unit test in a src module can never match that filter
metadata:
  type: feedback
---

When a brief hands you an acceptance command of the form
`cargo test -p simulation <bare_test_name> -- --exact`, the test CANNOT live in a
`#[cfg(test)] mod tests` inside `simulation/src/**`.

**Why:** libtest's `--exact` matches the *full* test path. A test in
`simulation/src/transportation/lane_queue_spike.rs` is reported as
`transportation::lane_queue_spike::tests::sov_dda_lane_queue_convoy`, which is not equal to
`sov_dda_lane_queue_convoy`, so the command runs 0 tests and exits 0 — a silently vacuous
"pass". Only two placements give a bare name: `simulation/src/lib.rs`'s root module, or an
integration-test file under `simulation/tests/<file>.rs`, whose root module is the file.

**How to apply:** put the implementation in your owned `src/` path and add a thin
integration-test file under `simulation/tests/`. That directory did not exist before
2026-08-27 (verified at commit `0aa5c35`); creating it adds a new test target that links the
whole lib. Report the extra file as a deviation — a brief that lists only "the spike file
plus one mod line" as owned paths has not accounted for it.

Integration tests see only `simulation`'s **public** API plus its dev-dependencies. `common`,
`prototypes` and `slotmapd` are normal dependencies and are NOT reachable from there — so
re-export anything the test needs (e.g. `DELTA`, a `Bincode` round-trip helper, slotmap key
constructors) from your own module. `VehicleID` is public as `simulation::VehicleID` via
`pub use world::*;` (`lib.rs:54`), even though `mod world;` itself is private.

Related: [[sim-test-setup-traps]], [[feedback-stale-brief-check]].
