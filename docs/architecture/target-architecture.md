# Target architecture

**Kind:** architecture
**Authority:** advisory — a proposal until a decision accepts it; nothing on this page is implemented unless [current substrate](current-substrate.md) says so
**Status:** draft
**Owner:** architecture
**Last verified:** 2026-08-28

## The centre is not ECS

The design thread and Lane C1 agree: the world is a hand-rolled typed store, not an ECS, and
replacing it with one buys nothing. The architectural centre is:

```text
identity · time · authority · transactions · change propagation · determinism · information boundaries
```

Everything below is a way of making those seven concrete.

## Principles (each has its own page)

| Principle | One line | Page |
|---|---|---|
| One authority per transition | A mutable field has one owning module; others hold IDs, results and intents | [authority boundaries](authority-boundaries.md) |
| Labelled phases | COMMAND · TOPOLOGY · ALLOCATION · DECISION · ROUTING · MOVEMENT · ARRIVAL · PRODUCTION · UTILITIES · ACCOUNTING · REPORTING | [simulation phases](simulation-phases.md) |
| Stable things sleep | Sleep → wake on condition → decide → emit intent → commit → schedule next → sleep; cadence bands | [time and events](time-and-events.md) |
| Two identity families | Append-only dense IDs for citizens and households; generational handles for bodies and vehicles | [entity identity](entity-identity.md) |
| Dense hot state | SoA cores, sparse side stores, fixed resource arrays, bitset cohorts | [state storage](state-storage.md) |
| Compute → merge → commit | Parallel read-only compute into intent buffers; stable sort; serial commit | [parallelism](parallelism.md) |
| Determinism by construction | Keyed randomness, stable ties, idempotent transitions, canonical digests, repeat-run tests | [determinism](determinism.md), [randomness](randomness.md) |
| Propagate what changed | A change journal feeding indexes, observatory, notifications, snapshots | [change journal](change-journal.md) |
| Derived truth is separate | The observatory computes balances and discrepancies; the physical sim owns truth | [observatory](observatory.md) |
| Immutable read views | Planner, Render, Audio, Debug snapshots; the Planner view declares provenance | [snapshots](snapshots.md) |
| Explanation is architecture | Causal facts with parent links and retention classes | [causality](causality.md) |
| Hierarchy for scale | Routing caches keyed by revision and epoch | [routing](routing.md) |
| Shared topology, separate physics | A network kernel; one solver per utility | [network kernel](network-kernel.md) |
| Versioned, migratable saves | Envelope plus a migration seam | [persistence](persistence.md) |
| CPU decides, GPU draws | Authoritative decisions on CPU; validated POD across the boundary | [render boundary](render-boundary.md) |
| Optimise in order | Representation → cadence → locality → incremental → hierarchy → parallelism → SIMD | [performance](performance.md) |

## Module shape (proposal)

Modules inside the `simulation` crate, not new crates:

```text
simulation/
  core/          time · ids · units · scheduler · random · transition · change_journal
  stores/        citizens · households · enterprises · stocks · bodies · vehicles
  physical/      resources · logistics · production · construction · roads · traffic · rail · networks
  society/       households · needs · employment · education · healthcare · demography
  institutions/  enterprises · allocation · reporting · plan · reserves · (work_collectives, unions, representation — Post-1.0)
  observatory/   material_balance · labor_balance · service_balance · discrepancy · causality · indexes
  forecast/      feasibility · shadow_sim · plan_compare
  snapshot/      planner · render · audio · debug
```

Today's shape is `economy/ souls/ map/ map_dynamic/ transportation/ utils/`. The mapping is a
reorganisation, not a rewrite; each move is a mechanical step in the
[migration sequence](migration-sequence.md).

## What the target must respect

- **The lockstep multiplayer crate.** Any parallelism must be bit-identical per frame, or the
  decision to drop `networking/` must be taken explicitly. The design thread never mentioned it.
- **The `exec_ent` closure channel.** `ParCommandBuffer::exec_ent(FnOnce(&mut Simulation))` is the
  main cross-system mutation path. Typed contexts cannot narrow it without first giving deferred
  callbacks a declared resource set.
- **Save continuity.** Nearly every structural proposal changes serialised layout. The migration
  seam comes first, or "one continuous save" is broken by the first refactor.
- **The replay baseline.** Any reorder regenerates `world_replay.json`; a repeat-run digest test
  must exist before reorders, or determinism regressions are invisible.

## What is deliberately not in the target

A general ECS rewrite; an async future per citizen; concurrent hash-map mutation of truth; a
generic rigid-body traffic engine; full numerical engineering solvers where a compact causal
approximation suffices; Salsa or Differential Dataflow as the core (prototype candidates only);
dependency churn without benchmarks.

## Open decisions

Listed with both sides in [migration sequence — open decisions](migration-sequence.md#open-decisions)
and in the [synthesis §6](../research/conversation-mining-2026-08-28/SYNTHESIS.md#6-open-conflicts-both-sides-recorded-decision-left-to-the-planner).

## Related

- [Current substrate](current-substrate.md)
- [Migration sequence](migration-sequence.md)
- [Engineering standards](../engineering/index.md)
- [Lane C2 — architecture vs code](../research/conversation-mining-2026-08-28/C2-architecture-vs-code.md)
- [Rust crates research](../research/engineering/rust-architecture-crates.md)
