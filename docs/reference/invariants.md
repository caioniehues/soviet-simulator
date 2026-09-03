# Invariants index

**Kind:** reference
**Authority:** operational — an index; each invariant binds only through the charter pillar or specification that states it
**Status:** active
**Owner:** project lead
**Last verified:** 2026-08-28

The cross-system rules that every subsystem instantiates. Not a copy of any specification; an index
from rule → the specs that state it → the tests that guard it → the code that violates it today.

| Invariant | Statement | Binding source | Specifications that instantiate it | Guarded by (today) | Violated today |
|---|---|---|---|---|---|
| **No teleportation** | Stock changes hands only at a physical endpoint by a physical carrier; never at match, payment or reservation | charter pillar 1 | logistics, trade, water (§004), production | `scenario_0082_dispatch_gates_stock_not_match`, `sov_abs_ext_trade_import_is_physical`, ledger tests | export-side ext-trade block of `make_trades` |
| **Conservation** | source + destination + custody + embedded + declared sinks = initial + declared sources, across request → reserve → pickup → cancel → return → deliver → consume | design law 4; spec conservation rows | logistics, resources, production (§003), construction, water (§003), electricity (§002), heating | `scenario_demolish_buyer_building_end_to_end_conserves`, `scenario_ledger_*`, `scenario_dead_*` | none known after `sov-abs`/`sov-6qx`; `sov-ahw` in progress |
| **Single authority** | One module writes each authoritative field; others hold IDs, results, intents | design law 18; spec register table | every spec's "authoritative module" | review (`wiring-auditor`, `reviewer`) | `Government.money` written by three modules; `Market` fuses six responsibilities |
| **Idempotent transitions** | An immutable transition ID applies once; replay is a no-op | design law 19 | water §006, electricity §002, logistics, production (`ProductionRunId`, target) | none — no `evid_*` test exists | — (mechanism absent) |
| **Stable identity** | A citizen, household or institution keeps its ID for its whole history; a dead Citizen #N stays #N | charter pillar 5; citizens §001 | citizens, households | `slotmapd` round-trip in `TestCtx::check_determinism` (for slot keys) | citizens are generational slot keys, reusable after death |
| **Finite capacity** | Storage, docks, vehicles, pipes, wires, classrooms, clinics are finite; over-capacity is a visible queue or refusal, never silent overflow | charter pillar 2 | logistics §011, production, education, healthcare, utilities | `scenario_0095_full_output_storage_halts_production`, `scenario_0083_zero_trucks_blocks_delivery` | vehicle cargo has no capacity; docks have no rate |
| **Failure persistence** | Unmet demand, stalled hauls, waiting citizens persist with age and reason; nothing is deleted to tidy a report; the game never ends | charter pillar 2; design law 3–4 | needs §004, logistics, production | `scenario_never_matched_waiting_is_not_reset`, `sov_ahw_stranded_tosource_import_reposts_and_resumes_production` | unmatched orders removed with `mem::take` (`ECO-SUB-001`) |
| **Non-price domestic clearing** | No domestic price participates in matching, dispatch or allocation; the rouble is border-only | charter pillar 3–4 | logistics §005, trade, production, needs | `scenario_0097_production_never_checks_treasury`, `evid_*_non_price_*` (planned) | treasury debits for buildings, roads, wages (`ECO-SUB-004`) |
| **Determinism** | Same initial state and commands → same authoritative state; no wall clock, no OS randomness, stable ties | design law 17; multiplayer lockstep | — (cross-cutting) | round-trip hash in every `TestCtx::tick`; `test_world_survives_serde` | no repeat-run test; sequential RNG; platform float intrinsics |
| **Observable discrepancy, no hidden verdict** | Strategic behaviour is inferred from request/receipt/consumption/on-hand/age; no authoritative `dishonest` flag | production §009 | production | `scenario_0151_inflated_request_hoards_honest_does_not` (sim side) | the Planner cannot observe `requested` in the UI |
| **Physical opportunity cost** | Priority relocates scarcity; what one use receives, another loses | design law 6 | logistics §005 (deficit priority) | none | no priority system |
| **Provenance on Planner values** *(target)* | Every Planner-visible value declares how it is known | design law 7–9 | none yet | none | UI reads physical truth directly |

## How to use this table

- Writing a spec: name which invariants your `SPEC-*` claims instantiate; add a row link.
- Writing a test: cite the invariant in the doc-comment; add the test name here.
- Reviewing an economy diff: the `ledger-invariant-checker` asks the conservation row's question.

## Related

- [Design laws](../product/design-laws.md)
- [Simulation transitions standard](../engineering/simulation-transitions.md)
- [Testing standard](../engineering/testing.md)
- [Mechanics index](mechanics-index.md)
