# C1 — Rust Crates and Libraries: Verification Report

**Lane:** C1  
**Source lines:** 522–693 (Rust architecture research), 1102–1116 (Rust implementation model)  
**Date:** 2026-08-28  
**Prior survey (do not duplicate):** `docs/research/awesome-rust-project-fit.md` (2026-08-27)

## 0. Summary (top ten findings)

1. **C1-01** — `egui` tracks git HEAD with NO branch/tag/rev pin. This is the unpinned git dep the release-engineer note warns about. The lockfile pins commit `d4e8966a`, but `cargo update` will silently advance it. Build is not reproducible.
2. **C1-02** — `yakui` also tracks a forked git repo (`Uriopass/yakui`, branch `dev`) with no rev pin. Second unpinned git source. Same risk.
3. **C1-07** — `slotmapd` is the upstream author's own fork of `slotmap`, purpose-built for serialization-cycle determinism. It IS the ECS identity backbone. No alternative ECS crate is in use — the world is hand-rolled `HopSlotMap` containers with typed keys.
4. **C1-10** — `FxHasher` (via `rustc-hash`) is used for ALL hash maps and the pathfinding hash. `FxHasher` is NOT cryptographic and NOT portable-deterministic across platforms/versions. Using it for canonical state hashing (as the conversation proposes) would be wrong.
5. **C1-14** — Cross-platform float determinism is UNADDRESSED. The geom crate calls `.sin()`, `.cos()`, `.sqrt()`, `.atan2()` as platform intrinsics. No `libm` dependency exists. These can produce different results on x86 vs ARM, breaking save/replay determinism.
6. **C1-16** — Salsa is actively maintained (v0.28.2, Aug 2026) and used by rust-analyzer. But its revision/memo overhead is designed for IDE-scale incremental queries (ms latency), not for 60fps game ticks. Using it for the derived Planner layer is PLAUSIBLE but unproven at game scale.
7. **C1-17** — Differential Dataflow (v0.25.1, July 2026) is actively maintained but NO shipped game uses it. Integration cost is high (requires the `timely` runtime). The conversation correctly identified it as experimental.
8. **C1-20** — `rayon` is a dep but is barely used: only `terrain.rs:66` calls `into_par_iter()`, and `pedestrian.rs:72` has a commented-out `par_bridge()`. The conversation's "deterministic parallelism with intent buffers" pattern already exists as `ParCommandBuffer` — but it is used for deferred entity mutations, not parallel computation.
9. **C1-26** — Rerun (v0.36.3, Aug 2026) has a commented-out dep in workspace `Cargo.toml` and a fully commented-out `simulation/src/rerun.rs`. It was an upstream (Egregoria) experiment for spatial data visualization/debugging. Active crate, MIT/Apache-2.0, useful for causal-journal visualization if revived.
10. **C1-30** — The conversation's LP/MILP suggestion maps cleanly to `good_lp` (v1.15.3, Aug 2026, MIT) with `microlp` backend (pure Rust, v0.6.0, July 2026). This is the cheapest path: `good_lp` abstracts solver selection, `microlp` avoids C bindings. `minilp` is abandoned (last release 2020).

## 1. Extracted items

| ID | Statement | Source line(s) | Verdict |
|---|---|---|---|
| C1-01 | egui tracks git HEAD without rev pin | Cargo.toml:22-24 | **CONFIRMED** — `git = "https://github.com/emilk/egui"` with no `rev`/`tag`/`branch`. Lockfile pins `d4e8966a` but any `cargo update` advances it. |
| C1-02 | yakui tracks forked git repo without rev pin | Cargo.toml:29-33 | **CONFIRMED** — `git = "https://github.com/Uriopass/yakui"`, branch `dev`. Lockfile pins `6c6982ff`. Fork of SecondHalfGames/yakui. |
| C1-03 | Salsa for derived incremental world | conv:652 | **PLAUSIBLE** — v0.28.2, active. See C1-16 for 60fps analysis. |
| C1-04 | Differential Dataflow / timely | conv:654 | **PLAUSIBLE** — both active. See C1-17 for game-integration reality. |
| C1-05 | Slot maps (generational identity) | conv:569-570 | **ALREADY-EXISTS** — `slotmapd` 1.0.11 (HopSlotMap) is the entity store. |
| C1-06 | Dense typed indexes | conv:568 | **PLAUSIBLE** — conversation proposes this for citizens. Current entities use generational slot-map keys (`VehicleID`, `HumanID`, etc. via `new_key_type!`). Dense indexes would be a new pattern. |
| C1-07 | slotmapd fork status | simulation/Cargo.toml:19 | **CONFIRMED** — Uriopass's fork of orlp/slotmap. Serialization-cycle determinism. Zlib license. v1.0.11 (July 2024). Author says "use original if you don't need this invariant." Same author as Egregoria. |
| C1-08 | SoA storage for citizens | conv:573-574 | **PLAUSIBLE** — not implemented. Current citizens are AoS (`HumanEnt` struct). `soa_derive` (v0.14.0, MIT/Apache-2.0) and `soa-rs` (v1.0.1, MIT, with serde) are viable. |
| C1-09 | Timing wheel / monotone event queue | conv:583-584 | **PLAUSIBLE** — `hierarchical_hash_wheel_timer` (v1.4.0, MIT) exists. Current scheduler is `SeqSchedule` — a flat list of systems run every tick. No event-driven scheduling exists. |
| C1-10 | FxHasher determinism | common/src/hash.rs | **WRONG (for canonical hashing)** — `FxHasher` is fast but platform-dependent. The conversation proposes "deterministic canonical hashing" but the codebase uses `FxHasher` everywhere. For canonical state hashing, need a portable hash (xxhash-rust with BSL-1.0, or blake3 with CC0/Apache-2.0). |
| C1-11 | Bitsets (dense/sparse) | conv:624 | **PLAUSIBLE** — `bitflags` 2.4.1 exists for flag sets. For cohort-membership bitsets: `fixedbitset` (v0.5.7, 472M downloads), `roaring` (v0.11.5, sparse), `hi_sparse_bitset` (v0.9.0, hierarchical). None currently in workspace for this purpose. |
| C1-12 | Fixed-point authoritative state | conv:597-598 | **PLAUSIBLE** — `fixed` crate (v1.31.0, MIT/Apache-2.0, no_std) exists. Current economy uses `f32` everywhere. The conversation proposes integer/fixed-point for conserved quantities. |
| C1-13 | Deterministic parallelism with intent buffers | conv:603-610 | **ALREADY-EXISTS (partially)** — `ParCommandBuffer` (`simulation/src/utils/par_command_buffer.rs`) IS an intent buffer. But it's for deferred entity kill/exec, not parallel compute. The "parallel compute → intent → merge → sort → commit" pattern is aspirational. |
| C1-14 | Cross-platform float determinism | conv:680-682 | **UNSUPPORTED** — no `libm` dep, no `-ffast-math` protection, no `core::f32` usage. `geom/src/` calls `.sin()`, `.cos()`, `.sqrt()` as platform intrinsics. Different CPU implementations produce different last-bit results. |
| C1-15 | Keyed randomness (seed+domain+entity+event) | conv:619-620 | **ALREADY-EXISTS (partially)** — `common/src/rand.rs` has `rand(x)`, `rand2(x,y)` etc. using Bob Jenkins' hash. `RandProvider` in simulation uses xorshift128. But keying is ad-hoc: `rand2(pos.x, pos.y)`, `randu(l.dist_from_bottom.to_bits() ^ base_random)`. Not structured seed+domain+entity+event. |
| C1-16 | Salsa viability at 60fps | conv:652 | **PLAUSIBLE with caveats** — Salsa's memo lookup has overhead: hash input, check revision, return cached. rust-analyzer queries take ms, not μs. For a derived Planner layer updated once per game-tick (not per-frame), overhead is acceptable. For per-frame reactive UI, need benchmarks. |
| C1-17 | Differential Dataflow game integration | conv:654 | **UNSUPPORTED (for games)** — No shipped game uses DD. Requires `timely` runtime (v0.31.0). Both are research infrastructure for streaming analytics. Integration cost: entire data pipeline must be expressed as DD operators. |
| C1-18 | Property-based testing | conv:688 | **ALREADY-EXISTS** — `quickcheck` 1.0.3 is a dev-dep. No `proptest`. The prior survey (awesome-rust-project-fit.md) recommends adding domain generators rather than a new framework. |
| C1-19 | Compile-time size assertions | conv:684 | **PLAUSIBLE** — `static_assertions` (v1.1.0, last release 2019, unmaintained but stable — the API surface is complete). `std::mem::size_of` in `const` blocks is the zero-dep alternative since Rust 1.46. Not currently used in this codebase. |
| C1-20 | LP/MILP feasibility solver | conv:669-670 | **PLAUSIBLE** — `good_lp` v1.15.3 (MIT) with backends. See §2 for detailed analysis. |
| C1-21 | Immutable snapshots (arc-swap) | conv:678 | **ALREADY-EXISTS** — `arc-swap` 1.7.1 is a simulation dep. Used for snapshot publishing. v1.9.2 available (minor update). |
| C1-22 | Versioned save format | conv:686 | **UNSUPPORTED** — current saves are raw bincode with no version envelope. `common/src/saveload.rs` has an `Encoder` trait that serializes/deserializes via serde+bincode, compressed with `miniz_oxide`. No format version, no migration path. |
| C1-23 | SIMD for contiguous layouts | conv:680 | **PLAUSIBLE** — `wide` (v1.7.0, Zlib/Apache/MIT) or `std::simd` (nightly). No SIMD anywhere in codebase currently. Premature until profiling identifies a hot path. |
| C1-24 | Typed IDs and unit newtypes | conv:549-564 | **ALREADY-EXISTS (IDs only)** — `VehicleID`, `HumanID`, `TrainID` etc. via `slotmapd::new_key_type!`. Unit newtypes for `Mass`, `Volume`, `Energy` etc. do NOT exist. All quantities are bare `f32`/`i32`. |
| C1-25 | Typed system contexts | conv:615-616 | **PLAUSIBLE** — Current systems take `&mut Simulation`. The `Resources` type-map provides some narrowing. The conversation proposes compile-time capability restriction. |
| C1-26 | Rerun for debugging/visualization | Cargo.toml:36, simulation/src/rerun.rs | **CONFIRMED** — commented-out dep (v0.17.0 in workspace, current is v0.36.3). Full module exists but is dead code. MIT/Apache-2.0. |
| C1-27 | `ordered-float` already a dep | Cargo.toml:26 | **CONFIRMED** — v4.2.1 pinned in lockfile. Used in scheduler timing. |
| C1-28 | `pathfinding` already a dep | simulation/Cargo.toml:23 | **CONFIRMED** — v4.2.1 specified, v4.10.0 in lockfile (compatible update). |
| C1-29 | `rayon` already a dep | simulation/Cargo.toml:21 | **CONFIRMED** — v1.10.0 in lockfile. Barely used (see §2). |
| C1-30 | LP solver path | conv:669-670 | **PLAUSIBLE** — best path is `good_lp` + `microlp`. See §2. |

## 2. Validation detail

### C1-01 / C1-02 — Unpinned git dependencies

The workspace `Cargo.toml` declares:
```toml
egui = { git = "https://github.com/emilk/egui" }
yakui = { git = "https://github.com/Uriopass/yakui", branch = "dev" }
```

Neither has `rev = "..."` or `tag = "..."`. The lockfile captures specific commits:
- egui: `d4e8966aac9347965f8d02310ecf2c9f23bb9bbc` (egui 0.27.2)
- yakui: `6c6982ff196850dc67de80ee7983ececd15966a8` (yakui 0.2.0)

13 total packages in the lockfile come from git sources. All 13 are from these two repositories (egui family: `ecolor`, `egui`, `egui_extras`, `egui_plot`, `egui-winit`, `egui-wgpu`, `emath`, `epaint`; yakui family: `yakui`, `yakui-core`, `yakui-widgets`, `yakui-wgpu`, `yakui-winit`).

**Risk:** `cargo update` (or a fresh clone without `Cargo.lock`) pulls whatever HEAD is. egui 0.29+ has breaking API changes from 0.27. yakui upstream (SecondHalfGames) has diverged from the Uriopass fork.

**Fix:** Add `rev = "<current-hash>"` to every git dep in workspace `Cargo.toml`. This makes the manifest self-documenting and prevents accidental updates. The lockfile then agrees with intent.

### C1-07 — slotmapd: the ECS backbone

`slotmapd` v1.0.11 (Zlib, July 2024). Author: Uriopass (same person who wrote Egregoria).

The fork's reason to exist: in upstream `slotmap`, a serialize-then-deserialize cycle can change key generation order and iteration order. `slotmapd` patches this so round-trip serialization produces identical observable behavior. This is critical for save/load determinism in a game that uses slotmap keys as persistent entity identities.

`world.rs` defines the actual "ECS":
```rust
new_key_type! {
    pub struct VehicleID;
    pub struct TrainID;
    pub struct HumanID;
    // ...
}
```
Each entity type gets its own `HopSlotMap<ID, Ent>` in the `World` struct. This is NOT a general ECS (no archetype queries, no component iteration). It is a typed collection store with generational identity.

The conversation's suggestion of `hecs`/`legion`/`bevy_ecs` as alternatives (conv:~570) is **WRONG for this codebase**. The entity model is deeply coupled: each entity type has its own struct with fixed fields, its own `ParCommandBuffer`, and its own `SimDrop` cleanup. Switching to a component ECS would require rewriting world, all entity types, all systems, and the save format.

**Maintenance risk:** slotmapd has 0 GitHub stars and the author says "use original if you don't need this invariant." If Uriopass stops maintaining it, the codebase must either (a) take on the fork, (b) patch upstream slotmap, or (c) accept non-deterministic save cycles.

### C1-10 — Hashing: FxHasher is not canonical

`common/src/hash.rs` wraps `rustc_hash::FxHasher` as the universal hash:
```rust
pub type FastMap<K, V> = rustc_hash::FxHashMap<K, V>;
pub type FastSet<V> = rustc_hash::FxHashSet<V>;
pub fn hash_u64<T: Hash>(obj: T) -> u64 { /* FxHasher */ }
```

`FxHasher` is designed for speed in hash maps. It is NOT:
- Cryptographic
- Portable across platforms (byte order matters)
- Stable across `rustc-hash` versions

For the conversation's "canonical deterministic hashing" (conv:688), the right tools are:
- **`xxhash-rust`** (v0.8.18, BSL-1.0, no_std): XXH3 is fast and portable. Already 96M downloads. Best balance of speed and determinism for state hashing.
- **`blake3`** (v1.8.7, CC0/Apache-2.0, SIMD-accelerated): overkill for state checksums but maximally portable. Use only if hash must survive adversarial inputs.
- **`ahash`** (v0.8.11): already a transitive dep via egui. Faster than `FxHasher` on modern CPUs (uses AES-NI). BUT it has random per-process seeds by default — NOT deterministic across runs unless you explicitly construct with a fixed seed.

**Recommendation:** Keep `FxHasher` for hash maps (speed). Add `xxhash-rust` for canonical state hashing and deterministic replay comparison. Do not use `ahash` for canonical purposes without fixing seeds.

### C1-14 — Float determinism

The geom crate (`geom/src/`) calls `f32::sin()`, `f32::cos()`, `f32::sqrt()`, `f32::atan2()` directly. These compile to platform-specific instructions:
- x86: SSE `sqrtss`, x87 `fsin`/`fcos` (or libm)
- ARM: NEON or software libm
- Wasm: different libm entirely

IEEE 754 mandates exact results only for `+`, `-`, `*`, `/`, `sqrt`. Transcendentals (`sin`, `cos`, `atan2`, `exp`, `ln`) are implementation-defined in the last bit.

For cross-platform deterministic replay (the conversation's thesis at conv:680-688), these must be replaced with `libm` (the Rust `libm` crate, v0.2.11, MIT/Apache-2.0, no_std, pure-Rust software implementations). This is a prerequisite for any deterministic save/replay system.

The `fast-float` dep in `common/Cargo.toml` is a DIFFERENT crate — it's for fast float-to-string parsing, not float arithmetic.

No `-ffast-math` or equivalent Rust flag (`#[cfg(target_feature = "fast-math")]`) is set anywhere. This is correct — fast-math breaks IEEE conformance.

### C1-15 — Keyed randomness: partially exists

Two separate RNG systems exist:

1. **`common/src/rand.rs`** — stateless hash-based PRNG. Uses Bob Jenkins' one-at-a-time hash. Functions like `rand(x: f32) -> f32`, `rand2(x, y)`. Used for positional randomness (building generation, light timing, pathfinding perturbation). This IS keyed randomness — but the keys are ad-hoc (position coordinates, entity IDs XORed with ticks).

2. **`simulation/src/utils/rand_provider.rs`** — stateful xorshift128 PRNG. Serialized with saves. Used for runtime random decisions (vehicle colors, spawn decisions).

The conversation proposes `seed + domain + entity + event_index` structured keys (conv:619-620). The current code uses positional keys (spatial coordinates) or sequential state (xorshift). The structured-key pattern is a design improvement, not a new dependency. It would use the existing hash infrastructure.

### C1-16 — Salsa at 60fps: feasibility analysis

Salsa v0.28.2 (MIT/Apache-2.0, MSRV 1.85) is used by rust-analyzer for incremental compilation queries. Its overhead model:

1. **Input change** → bump global revision counter.
2. **Query access** → check memo: hash inputs, compare revisions. If inputs unchanged, return cached. Otherwise, recompute.
3. **Memo storage** → one `DashMap` entry per (query-function, input-key) pair.

For a game's Planner-derived layer:
- The physical sim changes ~thousands of values per tick.
- The Planner layer queries aggregates (material balances, queue stats, shortage indices).
- If Salsa queries run once per tick (16ms budget at 60fps), memo overhead is negligible — the expensive part is the recomputation, which Salsa reduces.
- If Salsa queries run per-frame for reactive UI, the memo check overhead (DashMap lookup per query) could matter with thousands of queries.

**Verdict:** Salsa is viable for a per-tick derived layer that feeds aggregated stats to the UI. It is NOT suitable as the physical simulation's inner loop. The conversation correctly identified it as "for the derived world, not the physical simulation core" (conv:652).

**Alternative:** A hand-rolled dirty-flag + recompute-on-read pattern (simpler, no dep, lower overhead) may be sufficient for the first iteration. Salsa's value comes only when the query dependency graph is complex enough that manual invalidation becomes error-prone.

### C1-17 — Differential Dataflow: integration reality

Differential Dataflow v0.25.1 (MIT, MSRV 1.86) with Timely Dataflow v0.31.0 (MIT).

Both are research infrastructure by Frank McSherry. They provide incrementally-maintained relational operators (join, group, iterate) over changing datasets.

**Game integration cost:**
1. Must express ALL incrementally-tracked data as DD `Collection<G, D, R>` types.
2. Must run the `timely` worker runtime alongside the game loop.
3. Must convert game state changes to DD `(data, time, diff)` triples.
4. DD's "time" is a logical partial order, not game ticks — mapping between them adds complexity.
5. DD allocates freely and has its own scheduling — fights with game-frame budgets.

**No game has shipped on DD.** It is used in production at Materialize (streaming SQL database) and in academic streaming-data research.

**Verdict:** The conversation correctly labeled this "experimental" (conv:654). Integration cost is too high for any near-term iteration. A change journal with hand-rolled incremental aggregates is cheaper and more controllable.

### C1-20 — LP/MILP solver: recommendation

The conversation proposes LP/MILP for analyzing Plan feasibility (conv:669-670). Options:

| Crate | Version | License | Type | Pure Rust? | Status |
|---|---|---|---|---|---|
| `good_lp` | 1.15.3 | MIT | API facade | Yes | Active (Aug 2026) |
| `microlp` | 0.6.0 | Apache-2.0 | LP+IP solver | Yes | Active (July 2026) |
| `minilp` | 0.2.2 | Apache-2.0 | LP solver | Yes | **Abandoned** (May 2020) |
| `highs` | 2.4.0 | MIT | C++ binding | No | Active (July 2026) |
| `clarabel` | 0.11.1 | Apache-2.0 | Conic solver | Yes | Active (June 2025) |
| `russcip` | — | — | C binding | No | — |

**Recommended path:** `good_lp` + `microlp` feature. Pure Rust, no C/C++ build dependencies, no system library requirements. `good_lp` provides a clean modeling API; `microlp` provides the solver. If LP proves insufficient and MILP is needed, `microlp` v0.6.0 now supports integer and boolean variables. If performance becomes an issue, switch backend to `highs` (requires HiGHS C++ library but is much faster on large problems).

`minilp` is NOT viable — last release 2020, no maintenance. The `microlp` crate is its maintained successor (by a different author, forked and actively developed).

### C1-22 — Save format: current state

`common/src/saveload.rs` defines an `Encoder` trait. The actual encoding is bincode 1.3.3 (MIT/Unlicense) with `miniz_oxide` compression.

There is NO:
- Format version tag
- Schema migration
- Compatibility check
- Forward/backward compatibility story

The conversation proposes "stable versioned release saves" (conv:686). Save format alternatives:

| Crate | Version | Speed | Size | Schema evolution | Serde? |
|---|---|---|---|---|---|
| bincode 1.x (current) | 1.3.3 | Fast | Compact | None | Yes |
| bincode 2.x | 2.0.1 | Faster | Compact | None built-in | Yes (adapter) |
| postcard | 1.1.3 | Fast | Very compact | None built-in | Yes |
| rkyv | 0.8.18 | Zero-copy | Compact | Migration via versioned types | Own derive |

The prior survey (`awesome-rust-project-fit.md`) correctly concluded: "Save risk is versioning and bounds, not encoding speed or syntax." The first need is a version envelope around the existing bincode payload. A format switch is premature.

### C1-29 — rayon usage: minimal

`rayon` 1.10.0 is a dep in simulation, engine, and native_app. Actual parallel iterator usage in `simulation/`:

1. `simulation/src/map/terrain.rs:66` — `into_par_iter()` for terrain generation.
2. `simulation/src/transportation/pedestrian.rs:72` — `par_bridge()` commented out.

That's it. The conversation's "deterministic parallelism" model (conv:603-610) describes a pattern where parallel computation writes to intent buffers, which are then deterministically merged. The existing `ParCommandBuffer` is close — it collects entity kill/exec commands from potentially-parallel code, then applies them sequentially in `apply()`. But the actual systems run sequentially in `SeqSchedule::execute()`. There is no parallel system dispatch.

## 3. Deeper mechanics

### 3.1 — Cheapest implementation path per architectural technique

| Technique | Cheapest path | Line-count estimate | Risk |
|---|---|---|---|
| **Salsa-derived Planner layer** | Hand-roll dirty-flag aggregates first. Salsa only if dependency graph exceeds ~20 query types. | 200-400 lines (dirty-flag); 800+ (Salsa integration) | Salsa overhead unknown at game scale. Start hand-rolled, measure. |
| **SoA citizen storage** | `soa-rs` v1.0.1 (has serde, no_std). Wrap `CitizenCore` fields. | 100-200 lines + migration of accessors | AoS→SoA changes every field access pattern. Profile first. |
| **Dense typed indexes** | `newtype_derive` pattern over `u32` with a generation counter in a parallel array. ~50 lines per type. | 50-100 lines | Must handle deletion/reuse. `slotmapd` already does this for entities. |
| **Timing wheel** | `hierarchical_hash_wheel_timer` (v1.4.0, MIT). | 50 lines integration | Must replace current "every system every tick" scheduler model. Large architectural change. |
| **Bitset cohort queries** | `fixedbitset` for dense (all citizens), `roaring` for sparse (citizens with property X). | 100-200 lines per use site | Requires citizen IDs to be dense indexes (currently generational slot-map keys). |
| **Fixed-point quantities** | `fixed` crate or hand-rolled: `struct Mass(i64)` with a scale constant. | 20-50 lines per quantity type | Must convert all f32 arithmetic at seam boundaries. Economy code uses f32 pervasively. |
| **Intent-buffer parallelism** | Extend `ParCommandBuffer` with typed output channels. Use `rayon` `par_iter` in systems that are provably independent. | 200-400 lines | Determinism requires stable iteration order. `rayon` does NOT guarantee order. Must collect+sort. |
| **Keyed randomness** | Restructure `common::rand` functions to take `(seed, domain: u32, entity_id: u32, event_idx: u32)`. | 30-50 lines refactor | Must audit every call site. ~20 call sites in simulation/. |
| **Canonical state hash** | Add `xxhash-rust`, hash world state after each tick, compare across runs. | 100-200 lines | Must ensure all state is hashable. `slotmapd` keys and `f32` values need careful handling. |
| **LP feasibility** | `good_lp` + `microlp` feature. Model Plan constraints as LP. | 200-500 lines for initial model | LP relaxation may not catch integer infeasibility. Start with LP, add MILP if needed. |
| **Compile-time size assertions** | `const { assert!(std::mem::size_of::<HumanEnt>() == N) }` in a test. Zero deps since Rust 1.79. | 1 line per struct | Must update N when struct changes. Use as regression guard, not absolute truth. |
| **Versioned saves** | Add a 4-byte magic + 4-byte version header before bincode payload. Check on load. | 30-50 lines | Must handle old saves (no header = version 0). |
| **Cross-platform float determinism** | Add `libm` dep, replace all `.sin()`/`.cos()`/`.sqrt()` in geom with `libm::sinf()` etc. | 50-100 lines of substitution | Performance regression possible (software vs hardware sqrt). Profile. |

### 3.2 — Salsa: per-tick derived layer design

If Salsa were used for the Planner's "Observatory" (conv:639-651), the architecture would be:

```
Physical sim tick
  → emit ChangeJournal entries (input mutations)
  → Salsa input writes: set_stock(entity, resource, qty), set_production(entity, resource, rate)
  → Planner queries: material_balance(resource), shortage_index(region), queue_length(store)
  → Salsa memoizes: only recomputes if upstream inputs changed
  → UI reads query results from cached Salsa DB
```

**Overhead per tick:** N input writes (hash+store, ~100ns each) + M query reads (memo check, ~50-200ns each if cached). For N=1000 inputs and M=100 queries, overhead is ~200μs — acceptable in a 16ms frame.

**The problem:** Salsa's current API (`#[salsa::tracked]`, `#[salsa::input]`) requires defining a Salsa database struct and using it as the root context. This does NOT compose easily with the existing `Simulation` struct + `Resources` type-map. Integration requires a new Salsa database as a parallel data store, with explicit sync at tick boundaries.

### 3.3 — rayon + intent buffers: determinism analysis

The conversation's pattern (conv:603-610):
```
parallel compute → intent buffers → deterministic merge → stable sort → authoritative commit
```

The existing code's pattern:
```
sequential system runs → ParCommandBuffer collects kills/execs → apply() drains in insertion order
```

**rayon's determinism:** `par_iter().for_each()` processes items in arbitrary order. `par_iter().map().collect()` preserves order of results but not execution order. Side effects (writes to shared state via Mutex, like `ParCommandBuffer`) are non-deterministic in arrival order.

**Current `ParCommandBuffer`:** Uses `Mutex<Vec<...>>` to collect commands. Insertion order depends on thread scheduling. The `apply()` method processes in insertion order — which is NONDETERMINISTIC if commands came from parallel code.

This is currently safe because systems run sequentially (no actual parallel command insertion). If parallel systems are introduced, commands must be:
1. Collected with their source-entity ID
2. Sorted by entity ID (or another stable key) before application
3. Applied in that deterministic order

### 3.4 — Deterministic float: the `libm` path

Files requiring `libm` substitution in `geom/src/`:
- `angle.rs`: `.cos()`, `.sin()` (4 calls)
- `polyline.rs`: `.sqrt()` (2 calls)
- `polyline3.rs`: `.cos()`, `.sqrt()` (3 calls)
- `segment.rs`: `.sqrt()` (1 call)
- `spline.rs`: `.sqrt()` (3 calls)
- `v3.rs`: `.sqrt()`, `.cos()`, `.sin()` (3 calls)

Plus scattered `.sqrt()` / `.sin()` / `.cos()` / `.atan2()` in `simulation/src/` (not yet audited exhaustively).

The `libm` crate (v0.2.11, MIT/Apache-2.0) provides `libm::sinf(x)`, `libm::cosf(x)`, `libm::sqrtf(x)` — pure Rust, bit-identical across platforms.

**Performance impact:** `libm::sqrtf` is software — ~5x slower than SSE `sqrtss`. For hot inner loops (pathfinding, collision), this matters. Mitigation: use `libm` only for authoritative state computation; allow platform-native for render-only math.

## 4. Missed / not apparent

### 4.1 — `bytemuck` (already a transitive dep, not mentioned)

`bytemuck` v1.25.2 (Zlib/Apache/MIT) is already in the lockfile, used by `engine/Cargo.toml` (v1.7.2 specified) and `native_app/Cargo.toml`. It's a direct dep of engine, not simulation.

For SoA storage, `bytemuck` is essential — it provides `Pod`/`Zeroable` traits for safe zero-copy casting of typed arrays. Any SoA citizen store should derive `bytemuck::Pod` on field types.

### 4.2 — `enum-map` (already a transitive dep via egui_extras)

`enum-map` v3.1.0 (MIT/Apache-2.0) is in the lockfile as a transitive dependency of `egui_extras`. It maps C-like enums to arrays — exactly what the conversation's "fixed resource arrays" (conv:593-594) needs.

With a small fixed resource catalogue (`enum Resource { Coal, Iron, Steel, ... }`), `EnumMap<Resource, Qty>` replaces per-holder `HashMap<Resource, f32>` with a dense array. Zero allocation, cache-friendly, O(1) access. This is possibly the single highest-impact data structure change for the economy system.

### 4.3 — `tracing` (already a transitive dep)

`tracing` is in the lockfile as a transitive dep (via wgpu's dependencies). The prior survey deferred replacing `log` with `tracing`. The conversation's "causal journal" concept (conv:656-661) aligns with `tracing`'s structured spans and events. But `tracing` is a logging framework, not a causal DAG — the causal history needs purpose-built data structures.

### 4.4 — Deterministic float: `ordered-float` is NOT the answer

`ordered-float` (already a dep) provides `OrderedFloat<f32>` with total ordering. This solves NaN comparison but does NOT solve cross-platform arithmetic determinism. Different CPUs can produce different `OrderedFloat` values from the same computation.

### 4.5 — `petgraph` is NOT in the workspace

The conversation implies graph infrastructure for networks (conv:673-674). `petgraph` (v0.8.3, MIT/Apache-2.0, 484M downloads, serde support) is NOT in the lockfile. The codebase uses custom graph structures (road network, electricity cache via union-find). `petgraph` would add ~40KB compiled but provides Dijkstra, DFS, BFS, topological sort, connected components, etc. — many of which are hand-rolled or use the `pathfinding` crate.

Note: `fixedbitset` is part of the petgraph project (same GitHub org), but `petgraph` itself is not a dep. `pathfinding` (which IS a dep) uses `fixedbitset` internally.

### 4.6 — `strum` not present, would help exhaustive enum handling

`strum` (enum stringify/iterate) is NOT in the lockfile. For the proposed resource catalogue where every holder must track every resource, `strum::EnumIter` + `enum-map` provides compile-time-complete resource coverage. Small convenience but prevents silent omission.

### 4.7 — `fast-float` in common: misleading name

`common/Cargo.toml` depends on `fast-float` v0.2.0. This is NOT a fast-math library. It's Lemire's fast float-parsing algorithm (string-to-float). Used in Lua data loading. Irrelevant to the conversation's float-determinism discussion.

### 4.8 — `beul` in engine: unexplained dep

`engine/Cargo.toml` depends on `beul` v1.0.0. This is a tiny crate for "build-time embedding of files as &[u8] constants" — a compile-time include-bytes wrapper. Innocuous but undocumented in any project docs.

### 4.9 — `flat_spatial` (Uriopass's own crate)

`flat_spatial` v0.6.1 is a workspace dep used in simulation and geom. Author: Uriopass (same as Egregoria). It provides spatial hashing (grid-based spatial index). The conversation does not mention it, but it's the spatial query backbone — the "spatial grids" in the heterogeneous data structures list (conv:630) already exist through this crate.

### 4.10 — wgpu snapshot publishing and arc-swap

The conversation proposes separate immutable snapshots (conv:678). `arc-swap` (already a dep) enables atomic pointer swaps for publishing snapshots from sim thread to render thread. The existing code uses `arc-swap` but the snapshot model is not fully implemented — the renderer still reads from `Simulation` directly in many places (the substrate map documents this as a known coupling).

### 4.11 — No `serde-version` or equivalent exists

The conversation implies versioned serialization (conv:686). There is no crate named `serde-versioning` on crates.io. Alternatives:
- `serde_with` (attribute macros for custom serialization)
- `serde-version` (exists but low downloads, not recommended)
- Manual version enum: `enum SaveV1 { ... }`, `enum SaveV2 { ... }` with `From` impls
- `rkyv`'s Archive trait with versioned schemas

The cheapest path is a magic+version header before the existing bincode payload, with explicit migration functions per version bump.

## 5. Cross-lane hooks

| Hook | Lane | What they need to know |
|---|---|---|
| FxHasher is not canonical | **Economy (A)** | If the economy implements conserved-quantity checksums, must NOT use `hash_u64()` from common — need xxhash-rust or similar portable hash. |
| Float determinism unaddressed | **Economy (A), Society (B1)** | Any cross-platform determinism claim (replay, networking, save-comparison) is false until `libm` replaces platform intrinsics. |
| `enum-map` for resource arrays | **Economy (A)** | Highest-impact data structure change for fixed resource catalogues. Already a transitive dep. |
| LP/MILP path exists | **Economy (A)** | `good_lp` + `microlp` is the cheapest path for Plan feasibility analysis. Pure Rust, no system deps. |
| No event-driven scheduler | **Society (B1, B2)** | The conversation proposes wake-on-demand citizen scheduling. Current scheduler runs all systems every tick. This is an architectural change, not a crate swap. |
| Bitset cohort queries need dense IDs | **Society (B1)** | Citizens currently use generational slot-map keys. Bitset membership queries (fixedbitset/roaring) require mapping to dense indexes or using roaring's sparse representation. |
| `ParCommandBuffer` is sequential | **All** | Intent-buffer pattern exists but does not enable actual parallel system execution. Don't assume systems can run in parallel. |
| Save format has no version envelope | **All** | Any change to serialized types breaks old saves silently. |

## 6. Open questions for the user

1. **Float determinism priority:** Is cross-platform deterministic replay a 1.0 requirement? If yes, the `libm` substitution (~50-100 edits) should happen early. If not, defer it.
2. **Salsa vs hand-rolled:** The Planner's derived layer could start as dirty-flag aggregates (simple) or as Salsa queries (more powerful but harder to integrate). Which iteration should prototype which?
3. **LP solver use case:** Should the LP feasibility check be a player tool (Gosplan computer shows "your plan is infeasible because X"), an AI/automation tool (enterprise optimizers), or a debug tool (developer validates economy balance)?
4. **Rerun revival:** The commented-out Rerun integration could become the causal journal's visualization layer. Is this worth reviving, or is Tracy + log sufficient?
5. **slotmapd maintenance:** If Uriopass stops maintaining the fork, should the project vendor the crate or upstream the serialization-determinism fix to orlp/slotmap?

## 7. Sources

### Crates.io API (accessed 2026-08-28)
- salsa 0.28.2: https://crates.io/api/v1/crates/salsa
- slotmapd 1.0.11: https://crates.io/api/v1/crates/slotmapd
- slotmap 1.1.1: https://crates.io/api/v1/crates/slotmap
- differential-dataflow 0.25.1: https://crates.io/api/v1/crates/differential-dataflow
- timely 0.31.0: https://crates.io/api/v1/crates/timely
- thunderdome 0.6.1: https://crates.io/api/v1/crates/thunderdome
- soa_derive 0.14.0: https://crates.io/api/v1/crates/soa_derive
- soa-rs 1.0.1: https://crates.io/api/v1/crates/soa-rs
- fixedbitset 0.5.7: https://crates.io/api/v1/crates/fixedbitset
- roaring 0.11.5: https://crates.io/api/v1/crates/roaring
- bitvec 1.1.1: https://crates.io/api/v1/crates/bitvec
- hi_sparse_bitset 0.9.0: https://crates.io/api/v1/crates/hi_sparse_bitset
- fixed 1.31.0: https://crates.io/api/v1/crates/fixed
- simba 0.10.2: https://crates.io/api/v1/crates/simba
- good_lp 1.15.3: https://crates.io/api/v1/crates/good_lp
- microlp 0.6.0: https://crates.io/api/v1/crates/microlp
- minilp 0.2.2: https://crates.io/api/v1/crates/minilp
- highs 2.4.0: https://crates.io/api/v1/crates/highs
- clarabel 0.11.1: https://crates.io/api/v1/crates/clarabel
- proptest 1.11.0: https://crates.io/api/v1/crates/proptest
- static_assertions 1.1.0: https://crates.io/api/v1/crates/static_assertions
- arc-swap 1.9.2: https://crates.io/api/v1/crates/arc-swap
- postcard 1.1.3: https://crates.io/api/v1/crates/postcard
- rkyv 0.8.18: https://crates.io/api/v1/crates/rkyv
- xxhash-rust 0.8.18: https://crates.io/api/v1/crates/xxhash-rust
- blake3 1.8.7: https://crates.io/api/v1/crates/blake3
- bytemuck 1.25.2: https://crates.io/api/v1/crates/bytemuck
- wide 1.7.0: https://crates.io/api/v1/crates/wide
- hierarchical_hash_wheel_timer 1.4.0: https://crates.io/api/v1/crates/hierarchical_hash_wheel_timer
- keyed_priority_queue 0.4.2: https://crates.io/api/v1/crates/keyed_priority_queue
- enum-map 3.1.0: https://crates.io/api/v1/crates/enum-map
- tracing 0.1.44: https://crates.io/api/v1/crates/tracing
- rerun 0.36.3: https://crates.io/api/v1/crates/rerun
- petgraph 0.8.3: https://crates.io/api/v1/crates/petgraph

### GitHub repositories (accessed 2026-08-28)
- https://github.com/salsa-rs/salsa (70 open issues, 2356 commits)
- https://github.com/Uriopass/slotmapd (179 commits, 0 stars, explicitly low-maintenance)
- https://github.com/Uriopass/yakui (fork of SecondHalfGames/yakui)
- https://github.com/emilk/egui (egui workspace, commit d4e8966a pinned in lockfile)

### Local files (tree as of 2026-08-28)
- `/home/caio/soviet-simulator/Cargo.toml` — workspace manifest, git deps
- `/home/caio/soviet-simulator/Cargo.lock` — 13 git-sourced packages, 2 unique git sources
- `/home/caio/soviet-simulator/simulation/Cargo.toml` — simulation deps
- `/home/caio/soviet-simulator/simulation/src/world.rs` — entity model (slotmapd HopSlotMap)
- `/home/caio/soviet-simulator/simulation/src/utils/scheduler.rs` — SeqSchedule
- `/home/caio/soviet-simulator/simulation/src/utils/par_command_buffer.rs` — ParCommandBuffer
- `/home/caio/soviet-simulator/simulation/src/utils/rand_provider.rs` — xorshift128 RNG
- `/home/caio/soviet-simulator/simulation/src/rerun.rs` — commented-out Rerun integration
- `/home/caio/soviet-simulator/common/src/hash.rs` — FxHasher wrappers
- `/home/caio/soviet-simulator/common/src/rand.rs` — stateless hash-based PRNG
- `/home/caio/soviet-simulator/common/src/saveload.rs` — bincode+miniz_oxide save codec
- `/home/caio/soviet-simulator/common/Cargo.toml` — common deps (bincode, rustc-hash, fast-float)
- `/home/caio/soviet-simulator/engine/Cargo.toml` — engine deps (wgpu 0.20.1, bytemuck, beul)
- `/home/caio/soviet-simulator/native_app/Cargo.toml` — native_app deps
- `/home/caio/soviet-simulator/geom/Cargo.toml` — geom deps (no libm)
- `/home/caio/soviet-simulator/geom/src/angle.rs`, `v3.rs`, `spline.rs`, etc. — platform float intrinsics
- `/home/caio/soviet-simulator/docs/research/awesome-rust-project-fit.md` — prior crate survey
