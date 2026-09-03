# Migration sequence

**Kind:** plan
**Authority:** advisory — architectural milestones, not a backlog; `bd` holds actionable work
**Status:** draft
**Owner:** architecture
**Verified-at:** `266f7b2`
**Last verified:** 2026-09-03

The order in which the target architecture can be reached without breaking determinism, saves or
the pillars. Reconciled from the design thread's Phases 0–10, Lane C2's dependency DAG, Lane E's
cheapest moves and Lane H's cheapest modes ([synthesis §3.14](../research/conversation-mining-2026-08-28/SYNTHESIS.md#314-implementation-sequence--bible-20-versus-the-lanes)).

## Dependency graph (Lane C2 §3.1)

```text
keyed randomness ─────────┐
typed system contexts ────┼─→ labelled phases ─→ deterministic parallelism
cadence bands ────────────┘

citizen record/body ─→ SoA cores ─→ bitset cohorts
                   └─→ event-driven citizens ─→ event calendar

change journal ─→ observatory ─→ causal inspector
planner snapshot ─→ four snapshots ─→ shadow simulation

save envelope + migration seam ─→ (every structural change above)
market decomposition, hierarchical routing, network kernel ─→ independent
```

## Milestones

### 0 · Foundation
Pin `egui`/`yakui` to revisions. Save envelope and migration seam. Keyed randomness. Repeat-run
determinism test and a portable digest. Inert phase metadata/labels without reorder (no access
semantics, no barriers — replay hashes unchanged; the schedule can then report time per phase).
Typed phase contexts with access barriers arrive later, as a prerequisite to deterministic
parallelism — not in this milestone. Ratify the missing 1.0
specifications (agriculture, terrain/geology, weather, hydrology, pollution, Plan/Quota/Tranche,
authored plans, notifications, shell/save/crash, presentation/audio); settle resource units and
handling classes.

### 1 · Scale proof
Dense `CitizenRecord` with append-only IDs; event calendar; cadence field; snapshots skeleton;
finite 250k simulation benchmark — a fixed-seed, fixed-tick CPU runner that replaces the cancelled
`sov-1ae` contract, not a run of the `headless` lockstep server binary; state the active-fraction target. Before any rich citizen mechanic.

### 2 · One physical chain
The truck leg exists and is tested. Make exports physical (the `make_trades` ext-trade block);
add loading/unloading time and vehicle custody; retire the five contradicted paths — export
teleport, the two domestic-money debits, auto-lots, the static multiplier's role as the only
inflation source. Extract `DispatchManager`.

### 3 · Dishonest-enterprise loop and observatory
Requested vs consumed on the building inspector (~30 lines; the cheapest high-value change in the
project). Change journal with one event. Material balance. Discrepancy view with provenance. This
proves the game's thesis earliest and is the minimum viable loop.

### 4 · Construction
One building from Ghost through Site material and work gates to activation.

### 5 · Households and lived scarcity
Household identity; residence and housing queue; Food and Meat pantry; consumption; explicit
going without; minimal household scheduling.

### 6 · Transport at scale
Traffic EMA/BPR/Gawron; hierarchical routing; spillback where feasible; junction deadlock
resolution; render culling and LOD.

### 7 · Utilities
Network kernel; electricity re-expressed over wire with priority shedding; water; sewage;
heating; waste; reservoir/hydro. Phase 7 is part of the 1.0 migration sequence, not Post-1.0 —
only gas is Post-1.0 (see [network kernel](network-kernel.md)).

### 8 · Social systems
Employment and qualification; education; healthcare; death and demography; richer time effects if
budget allows.

### 9 · Plan loop
Plan / Quota / Tranche. **Caveat:** a minimal plan-period clock may need to arrive earlier —
storming, ratchet, credibility and the Taut Plan all hang from it.

### 10 · Polish and authored Plans
The three Plans stress the same simulation; the First Plan teaches the loop through play. Polish
is interleaved throughout, never postponed.

## Open decisions

| Decision | Options | Page |
|---|---|---|
| Cross-platform determinism a 1.0 goal? | `libm` + fixed-point now / defer | [determinism](determinism.md) |
| Digest hash | XXH3 / BLAKE3 | [determinism](determinism.md) |
| Keep lockstep multiplayer? | keep (all parallelism bit-identical) / drop `networking/` | [parallelism](parallelism.md) |
| Save migration before structural refactors? | seam first / accept "new save" pre-1.0 | [persistence](persistence.md) |
| Save codec | bincode + envelope / postcard + zstd | [persistence](persistence.md) |
| Phase order | relabel current / adopt target order | [simulation phases](simulation-phases.md) |
| Replay compatibility across versions | maintain / regenerate | [determinism](determinism.md) |
| Deferred callbacks | narrowed closures / intent enums | [authority boundaries](authority-boundaries.md) |
| Derived layer | hand-rolled dirty flags / Salsa | [observatory](observatory.md) |
| LP backend | `microlp` / HiGHS | [observatory](observatory.md) |
| Property testing | `quickcheck` (present) / `proptest` | [testing standard](../engineering/testing.md) |
| SoA | hand-written / crate | [state storage](state-storage.md) |
| Planner-hidden resources | enumerate | [snapshots](snapshots.md) |
| `slotmapd` maintenance | vendor / upstream | [entity identity](entity-identity.md) |
| Active fraction at 250k | measure and decide | [performance](performance.md) |

Both sides of each are recorded in the pages and in [synthesis §6](../research/conversation-mining-2026-08-28/SYNTHESIS.md#6-open-conflicts-both-sides-recorded-decision-left-to-the-planner).
None is accepted. The three decision-shaped drafts are in `docs/plan/proposals/`.

## Related

- [Target architecture](target-architecture.md)
- [Current substrate](current-substrate.md)
- [1.0 scope](../product/scope-1.0.md)
- [Proposals](../plan/proposals/sim-tick-phases.md)
- `bd ready` — the actionable work
