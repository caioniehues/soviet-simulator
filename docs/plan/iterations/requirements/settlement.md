# Wave 3 Settlement requirements

**Kind:** requirements
**Authority:** operational
**Status:** draft
**Owner:** project lead
**Last verified:** 2026-08-24

These requirements are proposed implementation contracts. Scope comes from the charter; mechanism comes only from the stable SPEC anchors named in each block. Every evidence status is intentionally unimplemented while the specifications remain draft.

## REQ-CITIZENS-001 — Persistent citizens under shortage and death

**Kind:** requirement
**Status:** proposed
**Owner:** project lead
**Scope link:** Charter §1.0 — Agriculture and services; Presentation and audio
**Specification anchors:** SPEC-CITIZENS-001, SPEC-CITIZENS-002, SPEC-CITIZENS-003, SPEC-CITIZENS-004, SPEC-CITIZENS-005, SPEC-CITIZENS-006, SPEC-CITIZENS-007
**Evidence intent:** Mutation-proven identity, allocation, shortage, and death-once guards plus inspected citizen capture
**Evidence status:** UNIMPLEMENTED — target guards block specification ratification.

### Acceptance criteria

- Each citizen retains one persistent identity through save/load, assignment, unmet need, and death lifecycle.
- Planner policy allocates eligible labour and study work by declared non-price criteria; unreachable work or shortage remains an observable outcome.
- Citizens alone publishes the death transition consumed once by dependent systems.

## REQ-NEEDS-001 — Distinct dwelling needs and going without

**Kind:** requirement
**Status:** proposed
**Owner:** project lead
**Scope link:** Charter §1.0 — Resources and production; Agriculture and services
**Specification anchors:** SPEC-NEEDS-001, SPEC-NEEDS-002, SPEC-NEEDS-003, SPEC-NEEDS-004, SPEC-NEEDS-005, SPEC-NEEDS-006
**Evidence intent:** Mutation-proven distinct-need, non-price, persistent-shortage, and once-consumption guards plus inspected need capture
**Evidence status:** UNIMPLEMENTED — target guards block specification ratification.

### Acceptance criteria

- Food and Meat remain distinct dwelling needs and satisfaction follows one authoritative compatible consumption event after physical availability.
- Domestic need clearing uses no roubles or prices; unsatisfied need persists as waiting, substitution, or inspectable going without.
- Each consumption ID changes compatible Resources stock once and cannot be replayed.

## REQ-HOUSEHOLDS-001 — Households, housing, and shared pantries

**Kind:** requirement
**Status:** proposed
**Owner:** project lead
**Scope link:** Charter §1.0 — Agriculture and services
**Specification anchors:** SPEC-HOUSEHOLDS-001, SPEC-HOUSEHOLDS-002, SPEC-HOUSEHOLDS-003, SPEC-HOUSEHOLDS-004, SPEC-HOUSEHOLDS-005, SPEC-HOUSEHOLDS-006, SPEC-HOUSEHOLDS-007, SPEC-HOUSEHOLDS-008
**Evidence intent:** Mutation-proven capacity, pantry, fulfillment-once, and death-consumption guards plus inspected household capture
**Evidence status:** UNIMPLEMENTED — target guards block specification ratification.

### Acceptance criteria

- A household has a persistent member set, a residence queue, and a capacity-bounded residence assignment.
- Food and Meat pantries are distinct shared physical records and cannot be credited by request, allocation, or a duplicated fulfillment.
- Housing priority is observable Planner policy and a Citizens death result removes membership once without deleting household history.

## REQ-EDUCATION-001 — Capacity-limited school and technical education

**Kind:** requirement
**Status:** proposed
**Owner:** project lead
**Scope link:** Charter §1.0 — Agriculture and services
**Specification anchors:** SPEC-EDUCATION-001, SPEC-EDUCATION-002, SPEC-EDUCATION-003, SPEC-EDUCATION-004, SPEC-EDUCATION-005
**Evidence intent:** Mutation-proven capacity, attendance, shortage, and non-price-priority guards plus inspected enrolment capture
**Evidence status:** UNIMPLEMENTED — target guards block specification ratification.

### Acceptance criteria

- The 1.0 catalogue contains exactly School and Technical education, with persistent enrolment, seat reservation, queue, and progress records.
- Progress requires attendance at a staffed operating compatible facility; absence of seat, staff, building, or route remains a visible queue or going-without result.
- Planner policy orders scarce seats by explicit non-price criteria.

## REQ-HEALTHCARE-001 — Finite healthcare with physical Medicine

**Kind:** requirement
**Status:** proposed
**Owner:** project lead
**Scope link:** Charter §1.0 — Agriculture and services
**Specification anchors:** SPEC-HEALTHCARE-001, SPEC-HEALTHCARE-002, SPEC-HEALTHCARE-003, SPEC-HEALTHCARE-004, SPEC-HEALTHCARE-005, SPEC-HEALTHCARE-006
**Evidence intent:** Mutation-proven capacity, Medicine-once, arrival, and non-price guards plus inspected care capture
**Evidence status:** UNIMPLEMENTED — target guards block specification ratification.

### Acceptance criteria

- Care requests retain citizen, reason, queue, priority, and outcome under Healthcare authority.
- Treatment requires finite staffed capacity, physical arrival or declared remote care, and compatible on-hand Medicine consumed once.
- Scarcity uses declared health priority, leaves waiting or worsening outcomes visible, and never clears by domestic price.
