# Adding a simulation system

**Kind:** guide
**Authority:** operational
**Status:** active
**Owner:** project lead
**Verified-at:** `4e9e930b2a73`
**Last verified:** 2026-08-28

## Before you write code

1. Read the specification for the subsystem and its `## Current substrate` section.
2. Name the state your system owns in the [authority index](../reference/authority-index.md). If
   another module already owns it, you are extending that module, not adding a system.
3. Dispatch the `substrate-cartographer` on the seam if no fact-sheet covers it (Phase 0 of the
   [development cycle](../process/development-cycle.md)). No brief until the ground is mapped.
4. Claim the `bd` issue.

## Registering a system

Systems are registered in `simulation/src/init.rs` and run in registration order:

```rust
register_system(&mut registry, "my_system", my_system);            // fn(&mut World, &mut Resources)
register_system_sim(&mut registry, "my_sim_system", my_sim_system); // fn(&mut Simulation) — avoid for new code
```

Position matters: a system placed after `market_update` sees this tick's matches; one placed
before `update_map` sees last tick's topology. Say in the `bd` issue and the commit why the
position was chosen. There are no phase labels yet ([simulation phases](../architecture/simulation-phases.md));
when there are, register under the right label.

## Owning state

Register a resource so it is saved and hashed:

```rust
register_resource::<MyState, Bincode>(&mut registry, "mystate", MyState::default);
register_resource_noserialize::<MyTransient>(&mut registry);   // not persisted, not hashed
```

A serialised resource changes the save layout — see the [serialization standard](../engineering/serialization.md).

## Mutating other entities

Deferred changes go through the entity's `ParCommandBuffer` (`exec_ent`, `kill`); they apply after
your system. Prefer emitting a typed intent that the owning module applies over a closure that
reaches into another module's state ([authority boundaries](../architecture/authority-boundaries.md)).

## Randomness

Do not draw from `RandProvider` for a new outcome if a keyed derivation from
`common::rand` works; keyed randomness is the target and the sequential stream is legacy
([randomness](../architecture/randomness.md)).

## Prove it

- A scenario test in `simulation/src/tests/scenarios/` — declare the module in `scenarios/mod.rs`
  (the shared file ownership tables forget).
- Watch it fail: break the behaviour, run, paste the red output, revert ([testing standard](../engineering/testing.md)).
- `cargo test -p simulation` green; the round-trip determinism check inside `TestCtx::tick` still
  passes.
- If the system changes scheduling or ordering, regenerate `world_replay.json` deliberately and say so.

## Document it

Update [current substrate](../architecture/current-substrate.md) (files, provides, does not
provide, scheduler position, tests). Run `python3 scripts/check_docs.py`.

## Gate

Phase 3 `evidence-auditor`; Phase 4 `wiring-auditor` (is it reachable from the running game?),
`ledger-invariant-checker` if the economy is touched, `reviewer`, domain advisor.

## Related

- [Current substrate](../architecture/current-substrate.md)
- [Writing evidence tests](writing-evidence-tests.md)
- [Rust standard](../engineering/rust.md)
- [Development cycle](../process/development-cycle.md)
