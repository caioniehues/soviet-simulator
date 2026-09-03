# Simulation phases

**Kind:** architecture
**Authority:** advisory
**Status:** draft
**Owner:** architecture
**Verified-at:** `266f7b2`
**Last verified:** 2026-09-03

## Current substrate

`Simulation::tick` (`simulation/src/lib.rs`) applies queued `WorldCommand`s, advances time, then
`SeqSchedule::execute` runs eighteen systems in the order they were registered in
`simulation/src/init.rs`, committing each entity type's `ParCommandBuffer` after every system:

```text
electricity_flow_system        UTILITIES      ← runs FIRST
dispatch_system                ALLOCATION/LOGISTICS
update_decision_system         DECISION (humans)
company_system                 PRODUCTION
pedestrian_decision_system     DECISION
transport_grid_synchronize     MOVEMENT
locomotive_system              MOVEMENT
vehicle_decision_system        DECISION/MOVEMENT
vehicle_state_update_system    MOVEMENT
routing_changed_system         ROUTING
routing_update_system          ROUTING
itinerary_update               MOVEMENT
market_update                  ALLOCATION + ACCOUNTING (matching, wages, dispatch advance)
train_reservations_update      ROUTING
freight_station                LOGISTICS
random_vehicles                (test traffic)
update_map                     TOPOLOGY       ← second-last
add_souls_to_empty_buildings   ARRIVAL/SPAWNING (takes &mut Simulation; runs last)
```

There are no phase names, no barriers, no multi-pass structure. `COMMAND` is real — it is
`WorldCommand::apply` before the schedule. Reordering any two systems changes when entities
interact within a tick and therefore changes replay hashes; `test_world_survives_serde` runs the
real `Simulation::schedule()` and compares registered resources plus the bincode-encoded ECS
World (closed `sov-n8v` / `sov-y66`), so it detects the divergence but cannot say which reorder
caused it — there are no per-phase digests yet ([determinism](determinism.md)).

The `COMMAND` step above is the `Simulation::tick` seam only. The native single-player loop applies a fast path first: when every queued command reports `WorldCommand::is_instant`, each is applied directly via `WorldCommand::apply` and the queue is cleared with no schedule pass (`native_app/src/network.rs:51-57`). The four instant variants are `MapBuildHouse`, `MapUpdateIntersectionPolicy`, `UpdateZone`, and `SetGameTime` (`simulation/src/world_command.rs:213-220`). Any other queued command forces one `Simulation::tick` while paused (`native_app/src/network.rs:67-70`).

## Target design

Eleven labelled phases (the design thread's ten plus REPORTING):

```text
COMMAND → TOPOLOGY → ALLOCATION → DECISION → ROUTING → MOVEMENT → ARRIVAL
→ PRODUCTION → UTILITIES → ACCOUNTING → REPORTING
```

A system may be split across phases when its lifecycle needs it. Within a phase, systems that
provably touch disjoint state may run in parallel ([parallelism](parallelism.md)). Between
phases, intents are merged and committed deterministically.

The proposed order **conflicts** with the actual order in two places that matter: electricity
runs first today (ninth in the target) and map update runs second-last (second in the target). Adopting
the target order is a behavioural change, not a relabelling.

## Migration — label, then reorder

1. **Label without reordering.** Add phase markers to `SeqSchedule` (`begin_phase("…")`); group
   the eighteen systems under labels in `init.rs` in their *current* order. Replay hashes are
   unchanged; the schedule can now report time per phase. One day.
2. **Add per-phase digests** so a later divergence is localised to a phase
   ([determinism](determinism.md)).
3. **Reorder within a label** only where the boundary guarantees safety; bump the replay version
   and regenerate `world_replay.json` deliberately.
4. **Adopt the target order** as its own decision, with the electricity-first and map-update moves
   argued explicitly.

Prerequisites: keyed randomness ([randomness](randomness.md)) — otherwise every reorder also
reshuffles the global `RandProvider` draw stream — and the replay-based repeat-run gate
(`test_world_survives_serde`, [determinism](determinism.md)).

## Open decisions

- Is the design thread's order a requirement, or may the current order be relabelled into fewer
  phases that preserve behaviour?
- Must replays stay compatible across versions, or may `world_replay.json` be regenerated after
  each structural change?

## Related

- [Time and events](time-and-events.md) — cadence bands within phases
- [Parallelism](parallelism.md)
- [Determinism](determinism.md)
- [Sim-tick phases proposal](../plan/proposals/sim-tick-phases.md) — the decision-shaped record
- [Current substrate](current-substrate.md)
