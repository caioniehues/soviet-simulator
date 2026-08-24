# Pathfinding specification

**Kind:** specification
**Authority:** binding
**Status:** draft
**Owner:** map and transport
**Last verified:** 2026-08-24

The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT, RECOMMENDED, NOT
RECOMMENDED, MAY, and OPTIONAL in this document are to be interpreted as described in RFC 2119
and RFC 8174.

## Purpose

Pathfinding translates an already-authorized trip into a route over the authoritative typed lane
graph. It keeps destination choice, goods allocation, need satisfaction, and traffic observation
outside the route solver.

## Scope and exclusions

This specification covers route request, compatibility, invalidation, failure, and recovery.
It does not define domestic allocation, road pricing, passenger-rail depth, rail signals and
electrification, or fuel lifecycle. It consumes the in-scope congestion signal owned by Traffic;
the other mechanisms are owned elsewhere or excluded Post-1.0 by the
[charter](../../plan/charter-1.0.md#explicit-cuts).

## Invariants

- `SPEC-PATHFINDING-001` A route is derived from a stated origin, destination, compatible lane
  access, and a recorded topology revision of the authoritative road graph.
- `SPEC-PATHFINDING-002` Vehicle route cost starts from compatible typed lanes, recorded topology,
  and declared lane length and speed, then multiplies free-flow time by Traffic's damped BPR cost
  for each lane. Pedestrian routes omit congestion cost. Road price is never a cost.
- `SPEC-PATHFINDING-003` A topology-invalid route is explicitly invalidated and its trip enters a
  visible reroute, waiting, or stalled state. It MUST NOT be silently discarded.
- `SPEC-PATHFINDING-004` Route failure records a reason and remains recoverable. It never creates
  a substitute route by teleporting an actor, stock, or need fulfillment.
- `SPEC-PATHFINDING-005` A route authorizes movement only; it neither transfers custody nor
  satisfies a dwelling need.
- `SPEC-PATHFINDING-006` A lane Traffic marks blocked is excluded from new routes. Topology
  invalidation or Traffic's stall threshold may trigger rerouting; ambient congestion changes
  alone MUST NOT continuously replan an en-route vehicle. Equal-cost choices use a deterministic
  tie-breaker.

## Model and state

A route request records requester identity, origin, destination, allowed lane classes, graph
revision, result, and failure reason. A resulting route records an ordered lane traversal and the
revision it was calculated against. The traveler owns its movement progress; logistics owns cargo
custody and needs own satisfaction state. Deterministic tie-breaking is a target: its completion
requires repeat-run evidence from identical inputs and initial state. Current save round-trip
evidence does not prove determinism.

## Failure behavior

No compatible path, a topology change, or repeated inability to progress produces a reasoned
waiting/reroute/stall state. Reroute can occur only over a compatible route and never silently
changes the trip's destination or its allocation. A later connected topology can recover the
same request. Stalled freight remains accounted for by logistics; stalled residents retain their
unmet need rather than disappearing.

## Observability

The Planner can inspect origin, destination, permitted lane classes, graph revision, route state,
failure reason, retry age, and affected traveler/job identity. Aggregate route failures remain
traceable to those individual records.

## Acceptance evidence

All guards below are **UNIMPLEMENTED** and block ratification. A command that executes zero tests
is failure, never green. The current 26-test suite proves no target below.

| Evidence | Future guard command and observable assertion | Negative mutation that must turn it red | Player-facing proof |
|---|---|---|---|
| `EVID-PATHFINDING-001` | `cargo test -p simulation evid_pathfinding_compatible_bpr_blocked_route -- --test-threads=1` — vehicle routes use compatible lanes and Traffic's damped BPR cost, exclude blocked lanes, and pedestrian routes omit congestion. | Read raw load, admit a blocked lane, or apply congestion cost to a pedestrian route. | Inspected route inputs, exclusions, and result view. |
| `EVID-PATHFINDING-002` | `cargo test -p simulation spec_pathfinding_invalidation_recovery_persists -- --test-threads=1` — topology invalidation preserves a visible reroute/wait/stall request. | Delete the request after invalidation. | Inspected route reason and recovery session. |
| `EVID-PATHFINDING-003` | `cargo test -p simulation spec_pathfinding_repeat_run_determinism -- --test-threads=1` — identical initial state and inputs yield identical route result. | Randomize an equal-cost tie break. | Inspected repeat-run route comparison; serde round-trip is not proof. |

## Substrate and decisions

`MAP-SUB-001` provides the typed lane graph and physical movement
(`simulation/src/map/objects/lane.rs:11-104`; `simulation/src/map/objects/road.rs:70-226`).
`MAP-SUB-003` records the live vehicle A* cost as lane length/speed plus deterministic noise, with
periodic retry for missing topology (`simulation/src/map/pathfinding.rs:189-268`;
`simulation/src/map_dynamic/itinerary.rs:171-198`) and no congestion, capacity, queue, closure,
freight restriction, or vehicle-class cost. It is therefore static and retry-only, not
congestion-aware. `MAP-SUB-004` confirms traffic has no durable congestion input for the solver
(`simulation/src/transportation/road.rs:15-78,185-250`). See the
[substrate fact-sheet](../../research/fact-sheets/wave1-substrate.md#roads-routing-and-traffic).
External CS1 and Workers & Resources material in archived legacy research is comparison evidence
only, never mechanism authority.

## Deferred behavior

Passenger-rail depth, rail signals/electrification, and vehicle fuel lifecycle are Post-1.0 and
receive no 1.0 mechanism or acceptance evidence here. Road pricing is prohibited by the binding
non-price domestic-clearing model, rather than deferred.

## Open questions

- Which routing restrictions are necessary for 1.0 before a future congestion model is ratified?
- What retry cadence and terminal stalled-state threshold preserve liveness without hiding failure?
- What repeat-run test will establish any determinism claim?
