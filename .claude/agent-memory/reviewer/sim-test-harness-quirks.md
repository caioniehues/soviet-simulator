---
name: sim-test-harness-quirks
description: Traps in simulation/src/tests — vehicles.rs is fully commented out, check_determinism is a round-trip not a desync check, sentinel command is vacuous; the static-mut race is FIXED (2026-08-26)
metadata:
  type: project
---

- `simulation/src/tests/mod.rs` is `#![cfg(test)] #![allow(dead_code)]`, so unused
  harness helpers never warn — a helper can be added and never called.
- **CORRECTED 2026-08-26** (sov-test-race-initfuncs-qt6, reviewed + verified):
  the `static mut` race is GONE. `init.rs` now uses one `OnceLock<Registry>`;
  `prototypes` uses `OnceLock<&'static Prototypes>` plus a `thread_local!`
  TEST_PROTOTYPES override that `try_prototypes()` checks first. Zero `static mut`
  remains in simulation/ or prototypes/. Parallel runs are now trustworthy:
  I measured 5/5 clean `cargo test -p simulation` (26 passed) and 2/2 serial.
  `tests/test_iso.rs:243` still calls `init()` outside the `Once`, but that is now
  a benign no-op (`let _ = REGISTRY.set(..)`), not UB.
  CLAUDE.md was updated to say "parallel runs are trustworthy" — the old
  `--test-threads=1` instruction is gone.
- The thread-local prototype override is safe because `SeqSchedule::execute`
  (`utils/scheduler.rs:41`) is a sequential for-loop — the sim never runs systems on
  worker threads. The only rayon site is `map/terrain.rs:66` (chunk gen), which is
  pure noise math and never calls `prototypes()`. Re-check this if either fact changes:
  a system moved onto a worker thread would silently read the GLOBAL set in tests.
- `simulation/src/tests/vehicles.rs` is entirely inside a `/* */` block. Its
  `TestCtx::init()` calls are dead text, not a compile error. Don't report them.
- `TestCtx::check_determinism` encodes the sim, decodes it, and compares
  `hashes()` of both. `hashes()` hashes the *same* saveload encoders. So it
  proves serialize/deserialize round-trips; it can NOT detect simulation desync,
  and it is structurally blind to state omitted from a `Serialize` derive.
- `tests/scenarios/mod.rs` documents a sentinel set (JOURNEY-0001, SCENARIO-0009/
  0015/0090/0115/0118) run via `cargo test -p simulation sentinel`. No test fn
  contains `sentinel` — that command passes vacuously.

**Why:** each of these makes a green check mean less than it looks like.
**How to apply:** before citing a simulation test as evidence, confirm the fn
exists and is not inside the commented block.
