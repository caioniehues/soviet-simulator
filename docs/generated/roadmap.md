# Wave 3 controlled-documentation roadmap

**Kind:** generated roadmap
**Authority:** reporting only; requirements and specifications remain authoritative
**Status:** draft — no target implementation is claimed
**Owner:** project lead
**Last verified:** 2026-08-24
**Generator:** `python3 docs/plan/iterations/build_roadmap.py --requirements-dir docs/plan/iterations/requirements --extract docs/plan/iterations/extract/requirements.json --evidence docs/generated/evidence/target-scenarios.json --output docs/generated/roadmap.md`

This roadmap reports the current re-derived Wave 3 contract. It does not import legacy completion, scenario IDs, or status claims. Current substrate regressions are intentionally reported outside the target-evidence totals.

| Re-derived requirements | Planned target scenarios | Implemented target scenarios | Current status |
| ---: | ---: | ---: | --- |
| 21 | 107 | 0 | draft / target evidence unimplemented |

## Requirement schedule

| Requirement | Contract | Planned EVID scenarios | Implemented | Status |
| --- | --- | ---: | ---: | --- |
| `REQ-BUILDINGS-001` | Declared buildings and observable activation | 4 | 0 | UNIMPLEMENTED |
| `REQ-CITIZENS-001` | Persistent citizens under shortage and death | 6 | 0 | UNIMPLEMENTED |
| `REQ-CONSTRUCTION-001` | Physical construction and activation | 9 | 0 | UNIMPLEMENTED |
| `REQ-EDUCATION-001` | Capacity-limited school and technical education | 4 | 0 | UNIMPLEMENTED |
| `REQ-ELECTRICITY-001` | Finite non-price electricity service | 6 | 0 | UNIMPLEMENTED |
| `REQ-HEALTHCARE-001` | Finite healthcare with physical Medicine | 4 | 0 | UNIMPLEMENTED |
| `REQ-HEATING-001` | Finite thermal service without electric substitution | 6 | 0 | UNIMPLEMENTED |
| `REQ-HOUSEHOLDS-001` | Households, housing, and shared pantries | 4 | 0 | UNIMPLEMENTED |
| `REQ-LOGISTICS-001` | Physical, finite, cancellable freight fulfillment | 10 | 0 | UNIMPLEMENTED |
| `REQ-NEEDS-001` | Distinct dwelling needs and going without | 4 | 0 | UNIMPLEMENTED |
| `REQ-PATHFINDING-001` | Compatible route derivation | 6 | 0 | UNIMPLEMENTED |
| `REQ-PRODUCTION-001` | Input-bounded production and observable dishonest enterprises | 6 | 0 | UNIMPLEMENTED |
| `REQ-RESOURCES-001` | Physical resource catalogue and accountable stock | 3 | 0 | UNIMPLEMENTED |
| `REQ-ROADS-001` | Planner-authored roads and physical parking | 4 | 0 | UNIMPLEMENTED |
| `REQ-SEWAGE-001` | Finite sewage buffering, treatment, and discharge | 5 | 0 | UNIMPLEMENTED |
| `REQ-TRADE-001` | Physical border clearance and the single rouble | 9 | 0 | UNIMPLEMENTED |
| `REQ-TRAFFIC-001` | Observable congestion and physical recovery | 7 | 0 | UNIMPLEMENTED |
| `REQ-VEHICLES-001` | Finite freight vehicles and fixed rail consists | 4 | 0 | UNIMPLEMENTED |
| `REQ-WASTE-001` | Physical waste collection and single disposition | 5 | 0 | UNIMPLEMENTED |
| `REQ-WATER-001` | Metered, finite Water transfer | 9 | 0 | UNIMPLEMENTED |
| `REQ-ZONING-001` | Planner land-use intent and siting feedback | 5 | 0 | UNIMPLEMENTED |

## Evidence boundary

Each planned row binds a rewritten `REQ-*` identifier, one or more stable `SPEC-*` anchors, and one current `EVID-*` anchor. A scenario can become implemented only when its exact guard exists, executes at least one test, and has mutation evidence. The separately generated current-regression inventory is not target proof.
