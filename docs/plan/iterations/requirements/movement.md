# Wave 3 Movement requirements

**Kind:** requirements
**Authority:** operational
**Status:** draft
**Owner:** project lead
**Last verified:** 2026-08-24

These requirements are proposed implementation contracts. Scope comes from the charter; mechanism comes only from the stable SPEC anchors named in each block. Every evidence status is intentionally unimplemented while the specifications remain draft.

## REQ-LOGISTICS-001 — Physical, finite, cancellable freight fulfillment

**Kind:** requirement
**Status:** proposed
**Owner:** project lead
**Scope link:** Charter §1.0 — Transport and border; Resources and production
**Specification anchors:** SPEC-LOGISTICS-001, SPEC-LOGISTICS-002, SPEC-LOGISTICS-003, SPEC-LOGISTICS-004, SPEC-LOGISTICS-005, SPEC-LOGISTICS-006, SPEC-LOGISTICS-007, SPEC-LOGISTICS-008, SPEC-LOGISTICS-009, SPEC-LOGISTICS-010, SPEC-LOGISTICS-011, SPEC-VEHICLES-001, SPEC-VEHICLES-002, SPEC-VEHICLES-003, SPEC-VEHICLES-005, SPEC-VEHICLES-006
**Evidence intent:** Mutation-proven same-vehicle traversal, timer-delivery, cancellation/recovery, deficit-ordering, and dock-rate guards plus inspected haul timeline
**Evidence status:** UNIMPLEMENTED — target guards block specification ratification.

### Acceptance criteria

- A finite compatible vehicle traverses one ordered compatible itinerary to source before pickup and to destination before delivery; elapsed time or route creation alone proves neither transition.
- Allocation and reservation do not transfer stock. Pickup, custody, delivery, cancellation, and recovery preserve one accountable quantity and vehicle identity without teleporting it.
- Target-stock demand uses declared non-price deficit, route distance, stable tie-break, and bounded docks; unavailable capacity waits with a visible reason.

## REQ-ROADS-001 — Planner-authored roads and physical parking

**Kind:** requirement
**Status:** proposed
**Owner:** project lead
**Scope link:** Charter §1.0 — Transport and border; Planner interaction
**Specification anchors:** SPEC-ROADS-001, SPEC-ROADS-002, SPEC-ROADS-003, SPEC-ROADS-004, SPEC-ROADS-005, SPEC-ROADS-006
**Evidence intent:** Mutation-proven topology, invalidation, and parking-authority guards plus inspected road verdict capture
**Evidence status:** UNIMPLEMENTED — target guards block specification ratification.

### Acceptance criteria

- Road topology is an authoritative Planner-authored typed physical network with declared capacity inputs and refusal feedback.
- Road placement or alteration preserves or explicitly invalidates affected route and parking references.
- Roads alone reserves physical parking; no consumer or dispatcher instant-parks a vehicle.

## REQ-PATHFINDING-001 — Compatible route derivation

**Kind:** requirement
**Status:** proposed
**Owner:** project lead
**Scope link:** Charter §1.0 — Transport and border
**Specification anchors:** SPEC-PATHFINDING-001, SPEC-PATHFINDING-002, SPEC-PATHFINDING-003, SPEC-PATHFINDING-004, SPEC-PATHFINDING-005, SPEC-PATHFINDING-006, SPEC-TRAFFIC-007, SPEC-TRAFFIC-008
**Evidence intent:** Mutation-proven route compatibility, blocked-lane, and no-transfer guards plus inspected route/reason capture
**Evidence status:** UNIMPLEMENTED — target guards block specification ratification.

### Acceptance criteria

- A route records origin, destination, compatible lane types, topology revision, and Traffic-derived damped cost.
- Invalid, blocked, or absent paths leave a recoverable reason and never transfer custody or satisfy a request.
- New routes exclude Traffic-blocked lanes and consume Traffic's published cost rather than copying congestion state.

## REQ-TRAFFIC-001 — Observable congestion and physical recovery

**Kind:** requirement
**Status:** proposed
**Owner:** project lead
**Scope link:** Charter §1.0 — Transport and border
**Specification anchors:** SPEC-TRAFFIC-001, SPEC-TRAFFIC-002, SPEC-TRAFFIC-003, SPEC-TRAFFIC-004, SPEC-TRAFFIC-005, SPEC-TRAFFIC-006, SPEC-TRAFFIC-007, SPEC-TRAFFIC-008
**Evidence intent:** Mutation-proven stall, no-deletion, EMA/BPR/Gawron, and authority guards plus inspected congestion capture
**Evidence status:** UNIMPLEMENTED — target guards block specification ratification.

### Acceptance criteria

- Moving vehicles remain on compatible physical lanes while queue, pressure, and stall age remain durable state.
- Stall recovery reroutes through Pathfinding or exposes a Planner-visible bottleneck; it never deletes a vehicle or clears a domestic request.
- Traffic publishes EMA load, BPR cost, Gawron damping, and blockage while retaining authority over the dynamic inputs.

## REQ-VEHICLES-001 — Finite freight vehicles and fixed rail consists

**Kind:** requirement
**Status:** proposed
**Owner:** project lead
**Scope link:** Charter §1.0 — Transport and border
**Specification anchors:** SPEC-VEHICLES-001, SPEC-VEHICLES-002, SPEC-VEHICLES-003, SPEC-VEHICLES-004, SPEC-VEHICLES-005, SPEC-VEHICLES-006, SPEC-LOGISTICS-003, SPEC-LOGISTICS-007
**Evidence intent:** Mutation-proven vehicle identity, capacity, recovery, parking-authority, and fixed-consist guards plus inspected fleet capture
**Evidence status:** UNIMPLEMENTED — target guards block specification ratification.

### Acceptance criteria

- Every operational freight vehicle has durable identity, state, compatible finite capacity, depot/recovery reference, and a Roads-owned parking reference.
- A missing vehicle or failed route preserves the haul and physical recovery state rather than spawning or deleting a substitute.
- The 1.0 rail catalogue is one locomotive type and one wagon type in a fixed compatible consist; passenger rail is not a requirement.
