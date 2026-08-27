# Wave 3 evidence coverage

**Kind:** generated evidence coverage
**Authority:** reporting only
**Status:** draft
**Owner:** project lead
**Last verified:** 2026-08-24
**Generator:** `python3 docs/plan/iterations/evidence/build_evidence.py --extract docs/plan/iterations/extract/requirements.json --specifications docs/reference/specifications --bindings docs/plan/iterations/evidence/evid-spec-bindings.json --output-dir docs/generated/evidence`

All rows are planned target evidence. `UNIMPLEMENTED` means no target guard has been implemented or mutation-proven; current regressions are listed separately and are not target proof.

| Requirement | Planned EVID anchors | Implemented | Status |
| --- | ---: | ---: | --- |
| `REQ-BUILDINGS-001` | 4 | 0 | UNIMPLEMENTED |
| `REQ-CITIZENS-001` | 6 | 0 | UNIMPLEMENTED |
| `REQ-CONSTRUCTION-001` | 9 | 0 | UNIMPLEMENTED |
| `REQ-EDUCATION-001` | 4 | 0 | UNIMPLEMENTED |
| `REQ-ELECTRICITY-001` | 6 | 0 | UNIMPLEMENTED |
| `REQ-HEALTHCARE-001` | 4 | 0 | UNIMPLEMENTED |
| `REQ-HEATING-001` | 6 | 0 | UNIMPLEMENTED |
| `REQ-HOUSEHOLDS-001` | 4 | 0 | UNIMPLEMENTED |
| `REQ-LOGISTICS-001` | 10 | 0 | UNIMPLEMENTED |
| `REQ-NEEDS-001` | 4 | 0 | UNIMPLEMENTED |
| `REQ-PATHFINDING-001` | 6 | 0 | UNIMPLEMENTED |
| `REQ-PRODUCTION-001` | 6 | 0 | UNIMPLEMENTED |
| `REQ-RESOURCES-001` | 3 | 0 | UNIMPLEMENTED |
| `REQ-ROADS-001` | 4 | 0 | UNIMPLEMENTED |
| `REQ-SEWAGE-001` | 5 | 0 | UNIMPLEMENTED |
| `REQ-TRADE-001` | 9 | 0 | UNIMPLEMENTED |
| `REQ-TRAFFIC-001` | 7 | 0 | UNIMPLEMENTED |
| `REQ-VEHICLES-001` | 4 | 0 | UNIMPLEMENTED |
| `REQ-WASTE-001` | 5 | 0 | UNIMPLEMENTED |
| `REQ-WATER-001` | 9 | 0 | UNIMPLEMENTED |
| `REQ-ZONING-001` | 5 | 0 | UNIMPLEMENTED |

| Current EVID anchors | Planned target scenarios | Implemented target scenarios | Uncovered EVID anchors |
| ---: | ---: | ---: | ---: |
| 107 | 107 | 0 | 0 |

Every current EVID anchor is represented by exactly one `TARGET-EVID-*` scenario; every target binds one or more re-derived `REQ-*` and `SPEC-*` identifiers.
