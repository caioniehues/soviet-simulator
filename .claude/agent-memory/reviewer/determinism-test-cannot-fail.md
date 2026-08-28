---
name: determinism-test-cannot-fail
description: SUPERSEDED 2026-08-28 — test_world_survives_serde now panics on divergence (sov-myg, commit 7fa08e8); it still only sees RESOURCES, never World
metadata:
  type: project
---

**Status change 2026-08-28.** The "cannot fail" defect is FIXED on
`fix/sov-myg-determinism-guard` (commit `7fa08e8`, one file:
`simulation/src/tests/test_iso.rs`). The bisection loop now records
`divergence_tick` in each of the three mismatch branches and panics after the
loop: `panic!("determinism divergence detected at tick {tick}")`. It also
clamps `check_start = (tick - check_size).max(3)`.

**Re-proved by mutation 2026-08-28** (reviewer, in `/home/caio/sov-myg-wt`):
perturbing `sim2`'s `Government::money` at every checkpoint gave
`not equal sim+sim2` then
`panicked at simulation/src/tests/test_iso.rs:316: determinism divergence detected at tick 3`,
`test result: FAILED`. Unmutated: `1 passed` in 7.74s. Full suite `45 passed;
0 failed; 1 ignored`.

**The `.max(3)` clamp is load-bearing — proved by a second mutation.** With the
clamp removed and the same divergence planted, the run dies instead at
`simulation/src/utils/resources.rs:80: called Option::unwrap() on a None value`
— red for an unrelated reason, no tick named. Do not "simplify" the clamp away.

## The hole that is still open

`Simulation::is_equal` (`simulation/src/lib.rs:214-232`) iterates
`saveload_funcs()` only — registered RESOURCES. `World` is not a saveload func;
`hashes()` (`:268-280`) has to encode `self.world` separately, which is the
proof. So an ECS-world-only divergence (entity/component state) still exits
green. The guard closes the resource half of the hole, not all of it.

**How to apply:** the test is now valid evidence for *resource* repeat-run
determinism, and for save/load round-trip. It is still NOT evidence that the
ECS world is deterministic. `docs/reference/architecture/substrate.md:37` still
says "Absent" for repeat-run determinism — that row is now stale for resources
and correct for World; fix it with that distinction, do not just flip it.

Related: [[sim-test-harness-quirks]], [[silent-decode-default-seam]].
