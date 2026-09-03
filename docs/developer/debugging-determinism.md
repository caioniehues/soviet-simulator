# Debugging determinism

**Kind:** guide
**Authority:** operational
**Status:** active
**Owner:** project lead
**Verified-at:** `4e9e930b2a73`
**Last verified:** 2026-09-03

## What the tests tell you today

- `TestCtx::check_determinism` failing means **serialisation is lossy**: encode → decode changed a
  resource's hash. The message names the resource key and tick. Look for a field that is not
  serialised, a `HashMap` whose order leaks into the hash, or a non-registered resource.
- `test_world_survives_serde` (`tests/test_iso.rs`) failing means **the replay diverged**: two
  replays of `world_replay.json` produced different state. Its `check_size`/`check_start` loop
  bisects to the first divergent tick.

Neither proves repeat-run determinism (same seed and commands → same state). Until the repeat-run
test exists, the procedure below is manual.

## Procedure

1. Reproduce with a fixed seed and a recorded replay (`Replay`, `utils/replay.rs`).
2. Bisect the tick with `test_iso`'s loop or by comparing `Simulation::hashes()` at checkpoints.
3. At the divergent tick, compare per-resource hashes to find the resource.
4. Suspects, in order of frequency:
   - **RNG order.** A new draw from `RandProvider` inserted before an existing one reshuffles every
     later draw. Keyed randomness removes this class ([randomness](../architecture/randomness.md)).
   - **Hash-map iteration** feeding an authoritative choice.
   - **System reorder** in `init.rs`.
   - **Unstable tie-break** (equal distances, equal priorities).
   - **`f32` accumulation order** changed by a refactor.
   - **A `ParCommandBuffer` fed from more than one thread** (should not happen today).
5. Fix, then regenerate `world_replay.json` **only if the change was intended** to alter behaviour,
   and only through the scenario builder
   (`cargo test -p simulation regenerate_fixture_replay -- --ignored --nocapture`), which is the
   sole sanctioned way to re-record it
   ([ADR-0002](../decisions/0002-fixture-world-is-a-materialised-replay.md)); the determinism
   baseline then moves once, deliberately, and the commit message says so.

## When phase digests exist (target)

Compare tick digests, then phase digests, to localise the first divergent phase; inspect the
transition journal around it ([determinism (architecture)](../architecture/determinism.md)).

## Cross-platform divergence

Expected today: `geom/` uses platform `sin`/`cos`/`sqrt`/`atan2`. If two machines disagree in the
last bit and nothing else changed, this is the cause, and the fix is the `libm` decision, not a
bug hunt.

## Related

- [Determinism (architecture)](../architecture/determinism.md)
- [Determinism standard](../engineering/determinism.md)
- [Testing standard](../engineering/testing.md)
