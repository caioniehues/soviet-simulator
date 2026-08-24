# Wave 3 Economy requirements

**Kind:** requirements
**Authority:** operational
**Status:** draft
**Owner:** project lead
**Last verified:** 2026-08-24

These requirements are proposed implementation contracts. Scope comes from the charter; mechanism comes only from the stable SPEC anchors named in each block. Every evidence status is intentionally unimplemented while the specifications remain draft.

## REQ-TRADE-001 — Physical border clearance and the single rouble

**Kind:** requirement
**Status:** proposed
**Owner:** project lead
**Scope link:** Charter §1.0 — Transport and border
**Specification anchors:** SPEC-TRADE-001, SPEC-TRADE-002, SPEC-TRADE-003, SPEC-TRADE-004, SPEC-TRADE-005, SPEC-TRADE-006, SPEC-TRADE-007, SPEC-TRADE-008, SPEC-RESOURCES-003, SPEC-RESOURCES-004
**Evidence intent:** Mutation-proven border-clearance, one-settlement, Medicine, and tagged-transport guards plus inspected customs capture
**Evidence status:** UNIMPLEMENTED — target guards block specification ratification.

### Acceptance criteria

- Domestic matching, allocation, reservation, dispatch, production, and needs use no money or price gate.
- A fixed per-kind rouble amount settles exactly once only after physical customs clearance of the declared order.
- Non-Water orders use a completed Logistics haul; Water clears only after a completed Water-owned metered transfer and never enters freight custody.

## REQ-PRODUCTION-001 — Input-bounded production and observable dishonest enterprises

**Kind:** requirement
**Status:** proposed
**Owner:** project lead
**Scope link:** Charter §1.0 — Resources and production
**Specification anchors:** SPEC-PRODUCTION-001, SPEC-PRODUCTION-002, SPEC-PRODUCTION-003, SPEC-PRODUCTION-004, SPEC-PRODUCTION-005, SPEC-PRODUCTION-006, SPEC-PRODUCTION-007, SPEC-PRODUCTION-008, SPEC-PRODUCTION-009
**Evidence intent:** Mutation-proven delivered-input, atomic-run, soft-budget, and dishonest-enterprise guards plus inspected discrepancy capture
**Evidence status:** UNIMPLEMENTED — target guards block specification ratification.

### Acceptance criteria

- A run consumes only delivered compatible input and remains bounded by declared recipe, labour, utilities, capacity, and storage, with its binding constraint visible.
- Run IDs apply input/output/byproduct changes atomically and once through Resources; domestic money never gates production.
- Underperformance retains an observable, allocation-eligible enterprise without conjured stock; request, receipt, consumption, surplus, and age discrepancies let the Planner infer hoarding without an honesty flag.

## REQ-RESOURCES-001 — Physical resource catalogue and accountable stock

**Kind:** requirement
**Status:** proposed
**Owner:** project lead
**Scope link:** Charter §1.0 — Resources and production; Transport and border
**Specification anchors:** SPEC-RESOURCES-001, SPEC-RESOURCES-002, SPEC-RESOURCES-003, SPEC-RESOURCES-004, SPEC-RESOURCES-005, SPEC-RESOURCES-006
**Evidence intent:** Mutation-proven conservation, cancellation, Water restriction, and Medicine restriction guards plus inspected stock capture
**Evidence status:** UNIMPLEMENTED — target guards block specification ratification.

### Acceptance criteria

- The catalogue declares the charter resource identities, units, handling compatibility, and import-only Medicine before use.
- Resources alone mutates on-hand balances; request, allocation, reservation, custody, delivery, and consumption remain separate accountable records.
- Failure and cancellation preserve quantity, Water never becomes cargo, and Medicine enters domestic stock only after physical import clearance.
