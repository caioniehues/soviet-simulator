---
name: sim-test-harness-quirks
description: Traps in simulation/src/tests — the Once guard is incomplete, vehicles.rs is fully commented out, check_determinism is a round-trip not a desync check
metadata:
  type: project
---

- `simulation/src/tests/mod.rs` is `#![cfg(test)] #![allow(dead_code)]`, so unused
  harness helpers never warn — a helper can be added and never called.
- `TestCtx::new()` guards `crate::init::init()` with a `Once`, but
  `tests/test_iso.rs:243` (`test_world_survives_serde`) still calls `init()`
  directly. The guard is incomplete; `init()` pushes into unsynchronized
  `static mut` INIT_FUNCS/GSYSTEMS.
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
