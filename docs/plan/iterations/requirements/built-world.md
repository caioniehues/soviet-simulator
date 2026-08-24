# Wave 3 Built World requirements

**Kind:** requirements
**Authority:** operational
**Status:** draft
**Owner:** project lead
**Last verified:** 2026-08-24

These requirements are proposed implementation contracts. Scope comes from the charter; mechanism comes only from the stable SPEC anchors named in each block. Every evidence status is intentionally unimplemented while the specifications remain draft.

## REQ-CONSTRUCTION-001 — Physical construction and activation

**Kind:** requirement
**Status:** proposed
**Owner:** project lead
**Scope link:** Charter §1.0 — Planner interaction; Resources and production
**Specification anchors:** SPEC-CONSTRUCTION-001, SPEC-CONSTRUCTION-002, SPEC-CONSTRUCTION-003, SPEC-CONSTRUCTION-004, SPEC-CONSTRUCTION-005, SPEC-CONSTRUCTION-006, SPEC-CONSTRUCTION-007, SPEC-CONSTRUCTION-008, SPEC-BUILDINGS-002, SPEC-BUILDINGS-003
**Evidence intent:** Mutation-proven executable construction conservation guard plus inspected Ghost/Site capture
**Evidence status:** UNIMPLEMENTED — target guards block specification ratification.

### Acceptance criteria

- A proposal records its footprint, material bill, verdict, and refusal reason before it creates one non-operating Site.
- Only received physical material and recorded work make a Site ground broken or complete; completion publishes one Buildings result and never activates an asset early.
- Partial delivery, interruption, rescind, and refusal conserve the material bill and remain inspectable.

## REQ-BUILDINGS-001 — Declared buildings and observable activation

**Kind:** requirement
**Status:** proposed
**Owner:** project lead
**Scope link:** Charter §1.0 — Planner interaction; Agriculture and services
**Specification anchors:** SPEC-BUILDINGS-001, SPEC-BUILDINGS-002, SPEC-BUILDINGS-003, SPEC-BUILDINGS-004, SPEC-BUILDINGS-005, SPEC-BUILDINGS-006
**Evidence intent:** Mutation-proven activation-once guard plus inspected building-state capture
**Evidence status:** UNIMPLEMENTED — target guards block specification ratification.

### Acceptance criteria

- A building declaration owns its identity, declared capability, connection declarations, and operating-state prerequisites.
- A completed Site activates exactly once through Buildings and remains observable when a declared prerequisite blocks operation.
- Planner inspection shows the declaration, Site, activation state, and every blocking prerequisite.

## REQ-ZONING-001 — Planner land-use intent and siting feedback

**Kind:** requirement
**Status:** proposed
**Owner:** project lead
**Scope link:** Charter §1.0 — Planner interaction
**Specification anchors:** SPEC-ZONING-001, SPEC-ZONING-002, SPEC-ZONING-003, SPEC-ZONING-004, SPEC-ZONING-005, SPEC-ZONING-006, SPEC-ROADS-005
**Evidence intent:** Mutation-proven non-spawn and siting-verdict guards plus inspected zoning capture
**Evidence status:** UNIMPLEMENTED — target guards block specification ratification.

### Acceptance criteria

- Planner land-use intent is an inspectable boundary record that Construction consults for Ghost verdicts.
- Changing intent never spawns, activates, demolishes, or deletes Sites or buildings; automatic lot spawning is not accepted as target placement.
- Shortage indicators are decision support only and expose their inputs without becoming an autonomous placement loop.
