# Snapshots and the information boundary

**Kind:** architecture
**Authority:** advisory
**Status:** draft
**Owner:** architecture
**Last verified:** 2026-08-28

## Current substrate

`native_app/src/game_loop.rs` holds `sim: Arc<RwLock<Simulation>>`. Every GUI function receives
`&Simulation` and reads what it likes: `sim.read::<GameTime>()`, `sim.read::<Government>().money`,
`sim.map()`, `sim.read::<ElectricityFlow>()`, `sim.read::<Market>()` — at least forty call sites
(Lane C2 §2.4). The renderer reads simulation state directly too. `arc-swap` 1.7 is a dependency
but the snapshot model is not used for the UI. There is **one reality**, and the UI is omniscient.

## Target design

Four immutable read views, published per tick; the simulation writes the next while consumers read
the previous:

```text
PlannerSnapshot   what THE PLANNER may know, with provenance per value
RenderSnapshot    positions, instances, LOD inputs — POD at the GPU boundary
AudioSnapshot     events and positions for sound
DebugSnapshot     physical truth, developer only
```

`ArcSwap` is the publication primitive. UI, render and audio never hold broad simulation locks.

**The Planner information boundary.** `PlannerSnapshot` never exposes hidden physical truth
because it is convenient. If the Planner can know a value, the snapshot says *how*: measured
directly, reported by an enterprise, aggregated statistically, observed via an institution,
estimated or forecast, unknown. This is the code-level form of the
[four realities](../simulation/concepts/information.md) and the reason the dishonest enterprise is
catchable rather than visible. A discrepancy inspector that shows "consumed 91 t" without saying
how the Planner learned it has already broken the model — the
[causal inspector proposal](../plan/proposals/causal-inspector.md) adds the provenance column.

Illustrative shape (not a ratified definition):

```rust
struct PlannerSnapshot { period: PlanPeriod, material_balances: Arc<MaterialBalanceView>,
    institutions: Arc<InstitutionReportView>, shortages: Arc<ShortageView>,
    services: Arc<ServicePressureView>, causal_alerts: Arc<CausalAlertView> }
```

No raw `Simulation` reference is reachable from it.

## Migration (Lane C2 §3.2 — the largest migration surface of any proposal)

1. A `PlannerView` with one field (`money`), populated in `game_loop.rs` after `tick()`; convert
   the menu bar. UI-only refactor; no simulation change.
2. Convert inspectors one at a time; every remaining direct `Simulation` read is either
   converted or explicitly tagged as debug access.
3. `RenderSnapshot` once the render boundary is defined ([render boundary](render-boundary.md)).
4. Remove `Arc<RwLock<Simulation>>` from the UI. Two to four weeks total; can be incremental.

## Open decisions

- Which resources the Planner may *not* see. "Reported institutional reality" is the principle;
  the enumeration does not exist yet. Today the UI reads `Market` capital directly — should the
  Planner see only aggregated trade history plus reports?

## Related

- [Observatory](observatory.md)
- [Render boundary](render-boundary.md)
- [Information (concept)](../simulation/concepts/information.md)
- [Reports and information (design)](../simulation/planned-economy/reports-and-information.md)
- [Observability standard](../engineering/observability.md)
