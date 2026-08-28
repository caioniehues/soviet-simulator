---
name: determinism-guard-reach
description: test_world_survives_serde runs an EMPTY schedule and is_equal ignores the ECS World — the two-run determinism check reaches far less than its name suggests
metadata:
  type: project
---

`simulation/src/tests/test_iso.rs::test_world_survives_serde` is the repo's only two-run
determinism check, and its reach is much narrower than it looks. Two independent holes,
both verified 2026-08-28 on branch `fix/sov-myg-v2` (base f6725f1):

1. **It builds `SeqSchedule::default()` (test_iso.rs:247) — an EMPTY schedule.**
   `Simulation::schedule()` (`simulation/src/lib.rs:138-145`) is the populated one that adds
   every registered gsystem. So the test executes **zero simulation systems**. It exercises
   only replay `WorldCommand::apply`, the `GameTime` increment (`lib.rs:251-254`), and
   `Replay::last_tick_recorded`.
   *Proof:* a process-global `AtomicU64` perturbation inserted into `economy::market_update`
   (`simulation/src/economy/mod.rs:51`) compiled and the test still passed GREEN. The same
   perturbation moved into `Simulation::tick` failed immediately.
   Inherited from upstream Egregoria at fork commit `68fe28c`, not introduced here.

2. **`Simulation::is_equal` (`lib.rs:214-233`) compares only the resources** via
   `saveload_funcs()`. It never touches `self.world`. `Simulation::hashes()`
   (`lib.rs:268-276`) *does* hash the serialized `World`, so `TestCtx::check_determinism`
   covers the ECS — but the two-run replay check does not.

Filed as **sov-n8v** (P2, open).

**How to apply:** never cite `test_world_survives_serde` as proof that a *system* is
deterministic. To force a divergence this test can actually see, perturb a registered
saveload resource from inside `Simulation::tick` itself — e.g. `Government.money`
(registered `init.rs:131`) — not from a system.

Related mechanics worth keeping:
- `SimulationReplayLoader` is created with `speed: 1` (`lib.rs:172`), so
  `advance_tick` runs exactly one `sim.tick` per call. The two runs in `test_iso` therefore
  interleave strictly, which is what makes a process-global counter a reliable divergence
  injector.
- `is_equal` writes `<resource_name>_a.json` / `_b.json` into the crate dir on mismatch;
  `save_to_disk("world")` writes `simulation/world/world.zip` + `world_replay.json`. Clean
  all of these up after a mutation run.
- In the narrowing loop, `check_start = (tick - check_size).max(3)` — the `.max(3)` is
  load-bearing, not cosmetic. Without it `check_start` reaches 0 and the bisection walks into
  ticks below the fixture's safe start, panicking in `Resources::read`
  (`simulation/src/utils/resources.rs:80`) before the terminal assertion can fire.

See [[sim-test-harness]] in the user memory index for the TestCtx side.
