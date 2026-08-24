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
electrification, fuel lifecycle, or a congestion-aware routing algorithm; those are either owned
elsewhere or excluded Post-1.0 by the [charter](../../plan/charter-1.0.md#explicit-cuts).

## Invariants

- `SPEC-PATHFINDING-001` A route is derived from a stated origin, destination, compatible lane
  access, and a recorded topology revision of the authoritative road graph.
- `SPEC-PATHFINDING-002` The 1.0 route cost uses only compatible typed lanes, recorded topology,
  and declared lane length and speed inputs. It has no congestion, queue, capacity, closure,
  freight-restriction, or road-price cost term unless a later active specification changes this
  contract.
- `SPEC-PATHFINDING-003` A topology-invalid route is explicitly invalidated and its trip enters a
  visible reroute, waiting, or stalled state. It MUST NOT be silently discarded.
- `SPEC-PATHFINDING-004` Route failure records a reason and remains recoverable. It never creates
  a substitute route by teleporting an actor, stock, or need fulfillment.
- `SPEC-PATHFINDING-005` A route authorizes movement only; it neither transfers custody nor
  satisfies a dwelling need.

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

Evidence must show a route over typed compatible lanes, a rejected incompatible request, and a
topology change that invalidates then visibly recovers or stalls a request. A mutation that leaves
an invalid route executable or deletes a failed request must fail its guard. Player-facing proof
must show a route's stated state and reason.

## Substrate and decisions

`MAP-SUB-001` provides the typed lane graph and physical movement. `MAP-SUB-003` records the live
vehicle A* cost as lane length/speed plus deterministic noise, with periodic retry for missing
topology and no congestion, capacity, queue, closure, freight restriction, or vehicle-class cost.
It is therefore static and retry-only, not congestion-aware. `MAP-SUB-004` confirms traffic has
no durable congestion input for the solver. See the [substrate fact-sheet](../../research/fact-sheets/wave1-substrate.md#roads-routing-and-traffic).
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
