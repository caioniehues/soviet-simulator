# Observability standard

**Kind:** standard
**Authority:** operational; rules marked *(target)* depend on the snapshot decision
**Status:** active
**Owner:** project lead
**Last verified:** 2026-08-28

Two things are meant by observability here: what the **Planner** may see (a game-design
constraint), and what a **developer** can see (a debugging constraint). They must not be the same
surface.

## Planner-facing rules

1. **Must** *(target)*: player-facing UI reads a `PlannerSnapshot`, never `&Simulation`. New UI
   code should not add direct `sim.read::<T>()` calls; existing ones are migration debt
   ([snapshots](../architecture/snapshots.md)).
2. **Must:** every Planner-visible value carries its provenance — measured, reported, aggregated,
   observed via institution, estimated, unknown. A panel that shows an enterprise's true
   consumption without saying how the Planner learned it breaks the four-realities model.
3. **Must:** the Planner never sees a hidden verdict (`dishonest`, `loyal`, `corrupt`). The Planner
   sees discrepancies and history.
4. **Should:** every significant object can eventually answer STATUS / CAUSE / TREND / POLICY /
   PHYSICAL CHAIN, from recorded causal facts, not reconstruction
   ([causality](../architecture/causality.md)).
5. **Should:** notifications derive from causal state (a trajectory, a repeated pattern, a threshold
   on a recorded metric), never from arbitrary events.
6. **Should:** reserves are shown in natural units ("18 h at current burn"), not one generic percentage.

## Developer-facing rules

7. **Must:** every authoritative transition is inspectable after the fact — by ID, with its inputs
   and its result — so a ledger question can be answered without re-running.
8. **Should** *(target)*: authoritative commits emit change-journal events; the debug snapshot
   exposes physical truth to developers only.
9. **Must:** a scenario test that fails narrates the causal chain in its message (the `sov-ahw`
   test's doc-comment is the model), so the failure explains itself.
10. **Must not:** let a debug read path become a game read path. Debug access to `Simulation` is
    tagged as such where it survives.

## Related

- [Snapshots (architecture)](../architecture/snapshots.md)
- [Causality (architecture)](../architecture/causality.md)
- [Causal inspector proposal](../plan/proposals/causal-inspector.md)
- [Information (concept)](../simulation/concepts/information.md)
