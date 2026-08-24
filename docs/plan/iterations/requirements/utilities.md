# Wave 3 Utilities requirements

**Kind:** requirements
**Authority:** operational
**Status:** draft
**Owner:** project lead
**Last verified:** 2026-08-24

These requirements are proposed implementation contracts. Scope comes from the charter; mechanism comes only from the stable SPEC anchors named in each block. Every evidence status is intentionally unimplemented while the specifications remain draft.

## REQ-ELECTRICITY-001 — Finite non-price electricity service

**Kind:** requirement
**Status:** proposed
**Owner:** project lead
**Scope link:** Charter §1.0 — Resources and production; Terrain and environment
**Specification anchors:** SPEC-ELECTRICITY-001, SPEC-ELECTRICITY-002, SPEC-ELECTRICITY-003, SPEC-ELECTRICITY-004, SPEC-ELECTRICITY-005, SPEC-ELECTRICITY-006, SPEC-ELECTRICITY-007
**Evidence intent:** Mutation-proven energy-conservation, shortage, and authority guards plus inspected network capture
**Evidence status:** UNIMPLEMENTED — target guards block specification ratification.

### Acceptance criteria

- Electricity owns explicit topology, bounded generation/storage/service, and a once-only allocation result.
- Shortage sheds declared non-price priority loads with visible served, curtailed, and unmet rates; it neither creates energy nor activates a disconnected building.
- Every offered generation result is bounded by a once-accepted Production result and its declared physical source/input.

## REQ-HEATING-001 — Finite thermal service without electric substitution

**Kind:** requirement
**Status:** proposed
**Owner:** project lead
**Scope link:** Charter §1.0 — Resources and production; Agriculture and services
**Specification anchors:** SPEC-HEATING-001, SPEC-HEATING-002, SPEC-HEATING-003, SPEC-HEATING-004, SPEC-HEATING-005, SPEC-HEATING-006, SPEC-HEATING-007
**Evidence intent:** Mutation-proven heat-flow conservation, shortage, and Weather-prerequisite guards plus inspected thermal capture
**Evidence status:** UNIMPLEMENTED — target guards block specification ratification.

### Acceptance criteria

- Heating owns a rate-bounded thermal graph, buffer, declared loss, and once-only allocation result.
- Thermal shortfall remains visible unmet heat and can make homes colder; Electricity never substitutes for it.
- Variable temperature demand requires a ratified Weather observation; otherwise the result declares static demand.

## REQ-WATER-001 — Metered, finite Water transfer

**Kind:** requirement
**Status:** proposed
**Owner:** project lead
**Scope link:** Charter §1.0 — Resources and production; Transport and border; Terrain and environment
**Specification anchors:** SPEC-WATER-001, SPEC-WATER-002, SPEC-WATER-003, SPEC-WATER-004, SPEC-WATER-005, SPEC-WATER-006, SPEC-TRADE-007, SPEC-TRADE-008
**Evidence intent:** Mutation-proven disconnected, zero-capacity, partial-flow, meter, clearance, and replay guards plus inspected water timeline
**Evidence status:** UNIMPLEMENTED — target guards block specification ratification.

### Acceptance criteria

- Water owns a connected compatible topology, quality, buffers, finite tick capacity, transfer progress, and directional border meter.
- Disconnected, zero-capacity, or partial paths do not deliver or clear; they retain visible unmet transfer without tanker, cargo, early rouble settlement, or created water.
- Trade clears once only after the complete Water-owned metered transfer; each transfer application is idempotent and quantity-conserving.

## REQ-SEWAGE-001 — Finite sewage buffering, treatment, and discharge

**Kind:** requirement
**Status:** proposed
**Owner:** project lead
**Scope link:** Charter §1.0 — Agriculture and services; Terrain and environment
**Specification anchors:** SPEC-SEWAGE-001, SPEC-SEWAGE-002, SPEC-SEWAGE-003, SPEC-SEWAGE-004, SPEC-SEWAGE-005, SPEC-SEWAGE-006
**Evidence intent:** Mutation-proven graph, backpressure, disposition-conservation, and authority guards plus inspected sewage capture
**Evidence status:** UNIMPLEMENTED — target guards block specification ratification.

### Acceptance criteria

- Sewage owns a separate finite graph, buffers, transfer/treatment/discharge records, and non-price priority.
- Blocked capacity retains physical backlog and a declared service restriction; no other system copies or mutates sewage state.
- Treatment/discharge and an optional Water handoff apply once and conserve accepted quantity into named output, residue, and loss.

## REQ-WASTE-001 — Physical waste collection and single disposition

**Kind:** requirement
**Status:** proposed
**Owner:** project lead
**Scope link:** Charter §1.0 — Agriculture and services
**Specification anchors:** SPEC-WASTE-001, SPEC-WASTE-002, SPEC-WASTE-003, SPEC-WASTE-004, SPEC-WASTE-005, SPEC-WASTE-006, SPEC-WASTE-007
**Evidence intent:** Mutation-proven compatible-haul, overflow/recovery, disposition-conservation, and non-price guards plus inspected waste capture
**Evidence status:** UNIMPLEMENTED — target guards block specification ratification.

### Acceptance criteria

- Waste owns typed finite containers, collection requests, overflow, processing disposition, and landfill retention.
- Collection requests exactly one compatible Logistics haul and uses a once-only receipt for container-to-custody pickup.
- Each delivered quantity has one conserving disposition through landfill or a bound Production result; blockage retains waste instead of deleting it or direct-crediting outputs.
