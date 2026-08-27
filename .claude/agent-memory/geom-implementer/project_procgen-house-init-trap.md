---
name: procgen-house-init-trap
description: Calling gen_exterior_house outside TestCtx SIGSEGVs in release unless crate::init::init() ran; test builds also load prototypes from '../' so cwd must be simulation/
metadata:
  type: project
---

To exercise `simulation::map::procgen::building::gen_exterior_house` from a test that
does not go through `TestCtx`, two setup steps are mandatory.

**Why:** `gen_exterior_house` reads `crate::colors()`, which reads the process-wide
prototype set. `prototypes::prototypes()` ends in `try_prototypes().unwrap_unchecked()`
(`prototypes/src/lib.rs:118-122`). With prototypes unloaded that is a `debug_assert`
panic in dev and a **bare SIGSEGV with no message in release** — a gdb backtrace shows
only `gen_exterior_house`, which reads exactly like a geometry memory bug and is not
one. Cost three turns during sov-bo3.

**How to apply:**

- Run `crate::init::init()` once (guard it with a `std::sync::Once`) before the first
  call.
- `simulation/src/init.rs:39-42` picks the prototype base path with `#[cfg(test)]`:
  `"../"` under test, `"./"` otherwise. So a simulation **test binary must be run with
  cwd = `simulation/`**, not the repo root, or it panics with
  `loading data.lua: No such file or directory`. `cargo test` gets this right; running
  `target/release/deps/simulation-*` by hand does not.
- `gen_exterior_house(size, seed)` is fully deterministic in `seed` (`common::rand::rand2`),
  so a seed sweep is a stable reproducible corpus, not a fuzz run.

Related: [[skeleton-lav-traps]], [[memory-capped-runs]]
