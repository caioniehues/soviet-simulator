# Parallelism

**Kind:** architecture
**Authority:** advisory
**Status:** draft
**Owner:** architecture
**Last verified:** 2026-08-28

## Current substrate

Systems run serially (`SeqSchedule::execute`). `rayon` is a dependency of `simulation`, `engine`
and `native_app` but is used once in the simulation, for terrain generation (`map/terrain.rs`);
a `par_bridge()` in `transportation/pedestrian.rs` is commented out. `ParCommandBuffer`
(`utils/par_command_buffer.rs`) is an intent buffer — it collects kill/exec commands into a
`Mutex<Vec<…>>` and applies them in insertion order after each system — but nothing feeds it from
more than one thread. If it ever did, insertion order would depend on thread scheduling and the
result would be non-deterministic (Lane C1 §3.3).

`Resources` uses interior mutability with runtime borrow checks; two systems reading the same
resource from different threads would panic or need per-resource locks.

The multiplayer crate is **lockstep**: server-merged inputs per `Frame`, `assert_eq!(frame, tick+1)`
on client and headless. Any parallelism that is not bit-identical across machines breaks it.

## Target design

```text
parallel read-only compute
→ thread-local intent buffers
→ concatenate
→ stable deterministic sort (immutable key: entity ID, then declared ordering)
→ serial or partitioned authoritative commit
```

Authoritative correctness never depends on `DashMap`, lock acquisition order or Rayon scheduling.
Intents carry their source key so the sort is total. The commit step is the owning module's
([authority boundaries](authority-boundaries.md)).

## Prerequisites — in order (Lane C2 §3.1)

1. **Keyed randomness** ([randomness](randomness.md)) — otherwise parallel workers drawing from
   one RNG stream are non-deterministic by construction.
2. **Typed contexts** — so the compiler knows which systems touch disjoint state.
3. **Labelled phases** — so "within a phase" has a meaning.
4. **Repeat-run determinism test with per-phase digests** — so a regression is visible and
   localisable.
5. Then: two provably disjoint systems in one phase on a Rayon pool; verify digests and
   multiplayer frames.

Lane C2's estimate for the full programme is months; it recommends nothing parallel until the
prerequisites land. The [performance hierarchy](performance.md) puts parallelism sixth of seven.

## Open decisions

- Keep lockstep multiplayer (then every step above must be bit-identical) or drop `networking/`
  (then determinism is a single-machine property). The design thread never raised this.
- Intent enums versus closures for the deferred path (see authority boundaries).

## Related

- [Simulation phases](simulation-phases.md)
- [Determinism](determinism.md)
- [Randomness](randomness.md)
- [Determinism standard](../engineering/determinism.md)
- [Lane C1 §3.3](../research/conversation-mining-2026-08-28/C1-rust-crates.md)
