# Causality

**Kind:** architecture
**Authority:** advisory
**Status:** draft
**Owner:** architecture
**Last verified:** 2026-08-28

Causal explanation is not cosmetic UI. Every important simulation problem should eventually answer
STATUS / CAUSE / TREND / POLICY / PHYSICAL CHAIN, and that requires the simulation to record
causes as it commits, not to reconstruct them afterwards.

## Current substrate

Nothing records a cause. The inspectors show state (workers, productivity, power, progress,
storage; a human's location, destination, house, last meal, work) and no chain
([current substrate](current-substrate.md)).

## Target design

**Causal facts.** Important transitions emit:

```text
FactId · tick · subject · kind · causes[]   (a small list of parent FactIds)
```

Emitted from authoritative commits alongside [change journal](change-journal.md) events; consumed
by the inspector, notifications and the chronicle.

**Retention classes** — never full event-sourcing forever:

- active and recent failures: detailed;
- routine old history: compacted to aggregates;
- major lifecycle and plan events (birth, qualification, employment change, household formation,
  moves, death; plan period results): permanent — these are also the citizen biography.

**Worked example** (design thread §18.2):

```text
Apartment 41 — indoor 15.8 °C
served heat 61 %
→ district supply temperature low
→ Heat Plant 3 constrained
→ coal bunker shortage
→ coal delivery 3 h 41 late
→ rail corridor congestion
→ strategic freight surge
```

Each arrow is a parent link; each node is a fact whose provenance is known; the chain crosses
heating, production, logistics and rail authorities without any of them copying another's state.

**Notifications** derive from causal state, never from arbitrary events: request inflation rising;
repeated period-end storming; rail peak/mean divergence worsening; queue burden crossing a
threshold; a tank on a depletion trajectory; a work collective reporting the same unresolved
issue again.

**Drill-down cost.** "Every aggregate clickable down to real trains" requires an index whose cost
scales with entities × statistics (Lane G-23). Aggregates should link to representative physical
examples first; full drill-down needs a budget.

## Migration

1. `Fact` struct and a `FactStore` with the three retention classes.
2. Emit facts from one seam: dispatch state changes (the tests already narrate this chain).
3. One inspector panel that walks parents for one object.
4. Notifications from two fact kinds.

## Related

- [Change journal](change-journal.md)
- [Observatory](observatory.md)
- [Causal inspector proposal](../plan/proposals/causal-inspector.md)
- [Causal loops (design)](../simulation/causal-loops.md)
- [Game modes — chronicle](../product/game-modes.md)
