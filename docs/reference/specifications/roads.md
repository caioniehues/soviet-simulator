# Roads specification

**Kind:** specification
**Authority:** binding
**Status:** draft
**Owner:** map and transport
**Last verified:** 2026-08-24

The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT, RECOMMENDED, NOT
RECOMMENDED, MAY, and OPTIONAL in this document are to be interpreted as described in RFC 2119
and RFC 8174.

## Purpose

Roads provide the Planner-authored physical lane network on which road movement, parking, and
route requests operate. This makes the charter's transport commitment precise without claiming
the current substrate already supplies the full planning contract.

## Scope and exclusions

This specification covers road topology, lane classes, placement, removal, capacity state, and
Planner inspection. Construction material/work mechanics await the construction specification.
Passenger-rail depth, rail signals and electrification, vehicle fuel lifecycle, and road pricing
are Post-1.0 exclusions under the [charter cut line](../../plan/charter-1.0.md#explicit-cuts).

## Invariants

- `SPEC-ROADS-001` A road is an authoritative, Planner-authored physical network object with typed
  lanes and an explicit topology change history. It is not a domestic-money purchase.
- `SPEC-ROADS-002` A route and movement use the compatible typed lanes of the authoritative road
  graph; lane access, direction, speed limit, and parking capacity are represented separately.
- `SPEC-ROADS-003` Placement, alteration, and removal report a verdict and preserve or explicitly
  resolve affected reservations, routes, vehicles, and sites. They MUST NOT silently delete those
  states.
- `SPEC-ROADS-004` Road topology declares static lane/corridor capacity inputs and exposes the
  referenced durable traffic state; dynamic load, queues, stalls, and capacity pressure belong to
  the traffic authority and are not inferred solely from a transient collision.
- `SPEC-ROADS-005` Automatic lot creation is not accepted as the target placement contract. Any
  automatic land-side effect requires a later explicit specification and Planner-visible verdict.
- `SPEC-ROADS-006` Roads alone owns physical parking-space reservations. Vehicles may reference a
  reserved space, Logistics may request recovery, and Traffic may observe blockage, but none may
  mutate the reservation.

## Model and state

A road graph records road identities, geometric connections, typed directed lanes, lane speed and
access class, topology revision, and parking capacity/reservations. Road work is linked to a
physical site when the construction contract is ratified. It references traffic records keyed to
its road, lane, or corridor, but does not own dynamic load, queues, stalls, or capacity pressure.
Vehicles reference parking spaces; Logistics requests recovery; Traffic only observes blockage.
The road-class and lane enum/data contract remains to be ratified; this specification does not
declare its vocabulary.

Topology revision invalidates affected routes explicitly. The road graph does not decide whether a
trip should occur, allocate goods, satisfy a need, or settle any rouble transaction.

## Failure behavior

Invalid or disconnected construction yields a refusal with a physical reason. A removed or
blocked connection leaves affected route users waiting, rerouting, or stalled according to the
pathfinding and traffic contracts. A full parking resource remains reserved or reports capacity
pressure; it must not duplicate a spot or erase a vehicle. Road failure creates visible pressure
and recovery work, never stock teleportation or game over.

## Observability

The Planner can inspect road class, lane directions and access, topology revision, parking
reservations, and affected route count, plus referenced traffic capacity pressure and its reason.
This readout distinguishes enduring pressure from local movement animation without duplicating
traffic's authoritative records.

## Acceptance evidence

All guards below are **UNIMPLEMENTED** and block ratification. A command that executes zero tests
is failure, never green. The current 26-test suite proves no target below.

| Evidence | Future guard command and observable assertion | Negative mutation that must turn it red | Player-facing proof |
|---|---|---|---|
| `EVID-ROADS-001` | `cargo test -p simulation spec_roads_typed_topology_command -- --test-threads=1` — a Planner command creates compatible typed topology. | Make the command create an untyped connection. | Inspected road/lane topology view. |
| `EVID-ROADS-002` | `cargo test -p simulation spec_roads_topology_invalidates_routes -- --test-threads=1` — altered topology invalidates affected routes. | Retain the old route revision after alteration. | Inspected disruption/refusal and route reference. |
| `EVID-ROADS-003` | `cargo test -p simulation spec_roads_parking_exclusive_no_auto_lots -- --test-threads=1` — parking is exclusive and road placement creates no automatic lots. | Permit a second reservation for one spot or create a roadside lot. | Inspected parking and placement verdict view. |
| `EVID-ROADS-004` | `cargo test -p simulation evid_roads_static_capacity_traffic_authority -- --test-threads=1` — `SPEC-ROADS-004`: topology publishes static lane/corridor capacity inputs and durable Traffic state by reference; dynamic load, queue, stall, and pressure are mutated only by Traffic, not inferred from a transient collision. | Mutate queue/load from Roads, derive dynamic pressure solely from a collision, or omit the Traffic-state reference. | Inspected topology capacity declaration alongside Traffic pressure and queue provenance. |

## Substrate and decisions

`MAP-SUB-001` provides typed driving, parking, walking, rail, and other lanes plus authoritative
road commands (`simulation/src/map/objects/lane.rs:11-104`; `simulation/src/map/objects/road.rs:70-226`).
`MAP-SUB-005` provides exclusive reservable parking (`simulation/src/map/objects/road.rs:197-212`;
`simulation/src/map_dynamic/parking.rs:24-90`). `MAP-SUB-002` records that current non-arbitrary
road construction removes and creates lots automatically (`simulation/src/map/map.rs:682-720`),
conflicting with this target. `MAP-SUB-004` records no durable capacity ledger or Planner traffic
readout (`simulation/src/transportation/road.rs:15-78,185-250`). See the [substrate fact-sheet](../../research/fact-sheets/wave1-substrate.md#roads-routing-and-traffic).
External CS1 and Workers & Resources material in archived legacy research is comparison evidence
only, never mechanism authority.

## Deferred behavior

Passenger-rail depth, rail signals/electrification, and vehicle fuel lifecycle are Post-1.0 and
receive no 1.0 mechanism or acceptance evidence here. Road pricing is prohibited by the binding
non-price domestic-clearing model, rather than deferred.

## Open questions

- Which minimum road classes and lane access rules are required for 1.0?
- What topology-change policy preserves valid in-progress trips while a road is altered?
- Is capacity pressure recorded per lane, per corridor, or both?
