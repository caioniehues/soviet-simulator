# Rust crates and architecture techniques — verified findings

**Kind:** research
**Authority:** research
**Status:** active
**Owner:** project lead
**Verified-at:** `4e9e930b2a73`; crates.io and GitHub as of 2026-08-28
**Last verified:** 2026-08-28
**Sources:** Lane C1 (`../conversation-mining-2026-08-28/C1-rust-crates.md`), Lane C2 (`C2-architecture-vs-code.md`), Lane G, the design bible §13–§16, lead checks this session (crates.io API records; `good_lp` README via Context7 `/rust-or/good_lp`).

## What the workspace actually depends on (relevant to the target)

| Crate | Status here | Note |
|---|---|---|
| `slotmapd` 1.0.11 | direct; **the entity store** | Uriopass's fork of `slotmap` for serialisation-cycle determinism; 0 stars; author says use upstream if the invariant is not needed |
| `rustc-hash` (`FxHasher`) | direct; all maps and `hash_u64` | fast; **not portable-deterministic** — wrong for a canonical digest |
| `arc-swap` 1.7.1 | direct | snapshot publication primitive; unused for the UI today; 1.9.2 available |
| `rayon` 1.10 | direct | used once (`map/terrain.rs`); a commented-out `par_bridge` |
| `pathfinding` 4.x | direct | flat A* |
| `flat_spatial` 0.6 | direct (Uriopass) | the spatial grid |
| `ordered-float` 4.2 | direct | NaN ordering for costs; **not** a determinism fix |
| `quickcheck` 1.0.3 | dev | property testing already present |
| `bincode` 1.3 + `miniz_oxide` | direct (`common`) | save codec; no envelope |
| `fast-float` | direct (`common`) | float *parsing*; irrelevant to determinism |
| `enum-map` 3.1 | transitive (via `egui_extras`) | ideal for fixed resource arrays |
| `bytemuck` 1.25 | transitive / direct in `engine` | POD for SoA and GPU boundary |
| `tracing` | transitive (via wgpu) | not a causal DAG |
| `egui` (git, **no rev**), `yakui` (git fork `dev`, **no rev**) | direct | 13 lockfile packages; build not reproducible |
| `rerun` | commented out; `simulation/src/rerun.rs` dead | active at v0.36; possible journal visualiser |

## Techniques the design proposes — cheapest path and verdict

| Technique | Cheapest path | Verdict |
|---|---|---|
| Keyed randomness | `fn keyed_rand(seed, domain, entity, ordinal)` over `common::rand`; ~50 call sites | no dependency; MUST-DO-FIRST for parallelism |
| Canonical digest | `xxhash-rust` (XXH3, BSL-1.0, portable) beside `hashes()` | open conflict with BLAKE3 (CC0/Apache; heavier; adversarial-safe) |
| Cross-platform floats | `libm` 0.2 for `sin/cos/sqrt/atan2` in `geom/` authoritative paths (~50–100 edits); software `sqrt` ~5× slower | only if cross-platform replay is a goal |
| Fixed resource arrays | `enum-map` (already transitive) | highest-impact economy data-structure change |
| SoA citizens | hand-written first; `soa-rs` 1.0 / `soa_derive` 0.14 as comparison | benchmark before adopting |
| Dense typed indexes | newtype over `u32` + generation array; `typed-index-collections` 3.5.0 (Jan 2026, 5.0 M downloads) as candidate | prototype |
| Bitset cohorts | `fixedbitset` 0.5 (dense; part of the `petgraph` project), `roaring` 0.11 (sparse) | needs dense citizen IDs |
| Fixed-point | `fixed` 1.31 or `struct Mass(i64)` with a scale | for conserved quantities |
| Timing wheel | hand-rolled `BTreeMap<(SimTime, Domain, Key), Event>` first; `hierarchical_hash_wheel_timer` 1.4 as candidate | large scheduler change |
| Intent-buffer parallelism | extend `ParCommandBuffer` with source keys + sort; `rayon` inside provably disjoint systems | deep change; `Resources` interior mutability is the obstacle |
| Derived layer | dirty-flag aggregates first; Salsa 0.28 (~200 µs per tick at 1,000 inputs + 100 queries) past ~20 query types | Salsa's database model does not compose with `Resources` |
| Differential Dataflow 0.25 + `timely` 0.31 | — | no shipped game; research only |
| LP feasibility | `good_lp` 1.15 with `microlp` 0.6 (pure Rust, MILP) — **or** `highs` 2.4 (C++, needs a C compiler; no extra libs on Linux per the `good_lp` README) | open conflict; `minilp` is abandoned (2020) |
| Save envelope | 4-byte magic + version before the bincode payload; `SaveMigration` trait | codec switch (`postcard` 1.1 + `zstd`) is an open conflict; `rkyv` 0.8 for internal snapshots only |
| Property tests | `quickcheck` already present; `proptest` 1.11 duplicates | open conflict |
| Concurrency testing | `shuttle` 0.9.3 (awslabs; Aug 2026) — note Context7 resolves "shuttle" to the unrelated shuttle.dev | for infrastructure primitives if parallelism lands |
| Instruction benchmarks | `iai-callgrind` 0.16.1 (Jul 2025; needs valgrind); Criterion for wall clock | after whole-world bottlenecks are known |
| Hierarchical routing | `fast_paths` or a hand-built contraction hierarchy behind a flag | validate against A* |
| Graphs | `petgraph` 0.8 is **not** a dependency; networks are custom (union-find) | consider for the network kernel |
| Size assertions | `const _: () = assert!(size_of::<T>() <= N)` — zero deps | `static_assertions` unnecessary |
| Snapshots | `arc-swap` (present) | — |
| Rerun revival | dep and module exist commented out | decide with the journal |

## What the design thread got wrong or omitted (C1, C2, G)

- Suggesting `hecs`/`legion`/`bevy_ecs` alternatives: WRONG for this codebase — the world is a
  typed store with per-type buffers and drop hooks; an ECS swap is a rewrite of world, entities,
  systems and save format.
- Calling `FxHasher` a canonical hash.
- Omitting `libm` from the float-determinism discussion; omitting `networking/` from the
  parallelism discussion; omitting the `exec_ent` closure channel from typed contexts; omitting
  the save-migration seam from the envelope.
- Naming Salsa and Differential Dataflow without a fit analysis.

## Open conflicts (recorded, undecided)

Digest: XXH3 vs BLAKE3. LP: `microlp` vs HiGHS. Property tests: `quickcheck` vs `proptest`. Save
codec: bincode + envelope vs `postcard` + `zstd`. Floats: fixed-point + `libm` now vs defer.
Derived layer: hand-rolled vs Salsa. Each is listed with both sides in the
[migration sequence](../../architecture/migration-sequence.md#open-decisions).

## Related

- [Dependency standard](../../engineering/dependencies.md)
- [Target architecture](../../architecture/target-architecture.md)
- [Project-fit survey (2026-08-27)](../awesome-rust-project-fit.md)
- [Lane C1](../conversation-mining-2026-08-28/C1-rust-crates.md)
