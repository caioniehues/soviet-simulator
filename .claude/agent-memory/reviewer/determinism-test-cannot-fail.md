---
name: determinism-test-cannot-fail
description: FIXED (branch-local) — test_world_survives_serde panics on divergence; the .max(3) clamp is load-bearing; the guard still sees only RESOURCES, never World or any system
metadata:
  type: project
---

**Status 2026-08-28.** The "cannot fail" defect is FIXED on
`fix/sov-myg-v2` @ `a48ea1d` (identical content to the earlier
`fix/sov-myg-determinism-guard` @ `7fa08e8` — `git diff 7fa08e8 a48ea1d --
simulation/src/tests/test_iso.rs` is empty). One file. **Not on main** as of
2026-08-28; a closed sov-myg citing a branch sha is the exact trap that got it
reopened once already.

The bisection loop records `divergence_tick` in each of the three mismatch
branches and panics after the loop (`test_iso.rs:311-313`), and clamps
`check_start = (tick - check_size).max(3)`.

## Mutation protocol that works on this test

Perturb **production code**, not the harness: an `AtomicU64` in
`Simulation::tick` (`lib.rs:256`, before `game_schedule.execute`) subtracting
`n` cents from `Government`. `SimulationReplayLoader { speed: 1 }`
(`lib.rs:172`) makes `advance_tick` run exactly one tick per call
(`replay.rs:32-38`), and the test alternates `loader`/`loader2`, so run A takes
even counter values and run B odd ones — the sums can never coincide.
`Government` is saveload-registered (`init.rs:131`), so `is_equal` sees it.
Result: 11 bisection rounds, then
`panicked at test_iso.rs:312: determinism divergence detected at tick 3`.

**The `.max(3)` clamp is load-bearing — proved twice.** Remove it with the same
divergence planted and the run reaches tick 2 and dies at
`simulation/src/utils/resources.rs:80: called Option::unwrap() on a None value`
— red for an unrelated reason, no tick named. It is a FLOOR fix, not an
underflow fix: `get_tick` is `u64` and `tick % check_size == 0` with
`tick >= 3` makes `tick >= check_size` always, so the subtraction cannot wrap.
The floor 3 equals `check_start`'s initial value, so it adds no blind spot.

## The hole that is still open (sov-n8v, P1)

Two, actually:
- `test_iso.rs:247` builds `SeqSchedule::default()` — **empty**. The populated
  one is `Simulation::schedule()` (`lib.rs:138-145`). The test runs ZERO
  simulation systems; a perturbation in `economy::market_update` stays green.
- `is_equal` (`lib.rs:214-233`) iterates `saveload_funcs()` only and never reads
  `self.world`. `hashes()` (`:268-279`) encodes `self.world` separately — that
  asymmetry is the proof.

So the armed panic only sees a divergence in a serialized RESOURCE produced by
command application or `GameTime`. Never say "the determinism guard is armed"
without that caveat.

## Repo quirk confirmed while reviewing this

`cargo fmt --all -- --check` **exits 1 on a clean checkout** of this branch:
`simulation/src/transportation/vehicle.rs:125` violates rustfmt at HEAD,
pre-existing. Any worker claiming "fmt exits 0" measured it with a format
hook's rewrite still on disk. Verify fmt claims against a `git checkout --`-ed
tree.

Related: [[sim-test-harness-quirks]], [[silent-decode-default-seam]].
