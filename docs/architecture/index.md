# Software architecture handbook

**Kind:** index
**Authority:** advisory — target pages are proposals until a decision accepts them; the current-substrate page is observational
**Status:** active
**Owner:** architecture
**Last verified:** 2026-08-28

How the simulation is built today, how the design proposes it should be built, the path from one
to the other, and what is still undecided. The four are kept apart on every page.

## Belongs here

Current architecture (cited to source), target architecture (labelled as proposal), migration
paths with their dependency order, and open architectural decisions.

## Does not belong here

Game mechanics (simulation tree), code rules (engineering standards), task lists (`bd`).

## The four questions

| Question | Page |
|---|---|
| What exists today? | [Current substrate](current-substrate.md) — per subsystem: files, types, authority, scheduler position, persistence, tests, conflicts with target |
| What is proposed? | [Target architecture](target-architecture.md) — the shape, the principles, the module map |
| How do we get there? | [Migration sequence](migration-sequence.md) — architectural milestones, not a backlog |
| What is undecided? | Each page's "Open decisions"; consolidated in [migration sequence](migration-sequence.md#open-decisions) |

## Pages

| Concern | Page | Current | Target |
|---|---|---|---|
| Who owns which state | [authority boundaries](authority-boundaries.md) | `Resources` type-map, `&mut Simulation`, `ParCommandBuffer` closures | one owning module per transition; typed contexts |
| Tick order | [simulation phases](simulation-phases.md) | 18 systems, flat registration order | labelled phases COMMAND…REPORTING |
| Time and wake-up | [time and events](time-and-events.md) | every system every 20 ms | event calendar, cadence bands |
| Identity | [entity identity](entity-identity.md) | generational `slotmapd` keys for everything | dense append-only IDs for citizens/households; slot maps for bodies |
| Storage | [state storage](state-storage.md) | AoS `HopSlotMap` structs; `BTreeMap` stock | SoA cores, sparse side stores, fixed resource arrays, bitsets |
| Parallelism | [parallelism](parallelism.md) | serial; `rayon` used once | compute → intent → sort → commit |
| Determinism | [determinism](determinism.md) | round-trip hash test; `f32` intrinsics; lockstep multiplayer | repeat-run digests, phase digests, `libm` or fixed-point |
| Randomness | [randomness](randomness.md) | one global Xorshift128 + positional hash | keyed `(seed, domain, entity, ordinal)` |
| Change propagation | [change journal](change-journal.md) | none; `rerun.rs` dead | compact events feeding indexes and snapshots |
| Derived views | [observatory](observatory.md) | `EcoStats` ring buffers | material/labour/service balances, discrepancy, forecasts |
| Read views | [snapshots](snapshots.md) | UI reads `Arc<RwLock<Simulation>>` | Planner / Render / Audio / Debug snapshots with provenance |
| Explanation | [causality](causality.md) | none | causal facts with parents and retention classes |
| Routing | [routing](routing.md) | fresh A* per request | hierarchy and caches keyed by revision and epoch |
| Networks | [network kernel](network-kernel.md) | electricity union-find over roads | shared topology kernel, separate solvers |
| Saves | [persistence](persistence.md) | bincode + version string, warn on mismatch | envelope + migration seam |
| Rendering | [render boundary](render-boundary.md) | renderer reads `Simulation` | immutable render snapshot, POD at the GPU boundary |
| Scale | [performance](performance.md) | no benchmark; 250k target with no gate | representation → cadence → locality → incremental → hierarchy → parallel → SIMD |

## Reading path

1. [Current substrate](current-substrate.md) — before anything else, so nothing below reads as implemented.
2. [Target architecture](target-architecture.md).
3. [Simulation phases](simulation-phases.md), [randomness](randomness.md), [determinism](determinism.md) — the dependency chain's first links.
4. [Migration sequence](migration-sequence.md).

## Authoritative documents this section depends on

- [Substrate architecture map](../reference/architecture/substrate.md) and the [fact-sheets](../research/fact-sheets/wave1-substrate.md) — cited code reality at their `Last verified` dates
- [Decisions](../decisions/README.md) — none accepted yet
- [Engineering standards](../engineering/index.md) — what new code must do regardless of which target is chosen

## Related

- [Proposals](../plan/proposals/sim-tick-phases.md) — the decision-shaped drafts awaiting the Planner
- [Rust crates research](../research/engineering/rust-architecture-crates.md)
- [Lane C2](../research/conversation-mining-2026-08-28/C2-architecture-vs-code.md) — the proposal-by-proposal audit against code
