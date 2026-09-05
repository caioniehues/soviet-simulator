# Mechanics index

**Kind:** reference
**Authority:** operational — a navigation table, not a specification; the "Current" column is observational at the verified commit
**Status:** active
**Owner:** project lead
**Verified-at:** `4e9e930b2a73`
**Last verified:** 2026-09-05

One row per mechanic. Scope uses ADR-0001 vocabulary (`docs/decisions/0001-households-and-utilities-are-1.0-scope.md`):
**1.0 — charter row \<name\>** (charter-committed), **Post-1.0** (deferred),
**Post-1.0 hook** (deferred; avoid an architectural dead end, build nothing),
**Never in scope** (charter permanent exclusion).
Current: **EXISTS**, **PARTIAL**, **ABSENT**, **CONTRADICTED** (the code does the opposite of the
pillar).

| Mechanic | Domain | Scope | Design page | Specification | Current | Research |
|---|---|---|---|---|---|---|
| Request vs allocation vs reservation vs receipt vs consumption | economy | 1.0 — charter row Resources and production | [requests](../simulation/physical-economy/requests.md) | production §003, logistics | PARTIAL (`capital`, `reserved`, `requested`) | E-005 |
| Enterprise over-requesting (dishonest enterprise) | economy | 1.0 — charter row Resources and production | [enterprise behaviour](../simulation/planned-economy/enterprise-behavior.md) | production §009 | PARTIAL — wired, not observable | A-02/03, E-007 |
| Storage-capacity floor on hoarding | economy | 1.0 — charter row Resources and production | [reserves (concept)](../simulation/concepts/reserves.md) | production | EXISTS (`storage_multiplier`) | synthesis §3.2 |
| Adaptive request inflation from reliability | economy | Post-1.0 hook | [reliability and buffering](../simulation/planned-economy/reliability-and-buffering.md) | — | ABSENT | A §3a |
| Planning credibility record | economy | Post-1.0 hook | [reliability and buffering](../simulation/planned-economy/reliability-and-buffering.md) | — | ABSENT | A §3d |
| Ratchet | economy | Post-1.0 hook | [reliability and buffering](../simulation/planned-economy/reliability-and-buffering.md) | — | ABSENT | A-12, G-32 |
| Plan periods, quotas | economy | 1.0 — charter row Plans and onboarding | [plan cycle](../simulation/planned-economy/plan-cycle.md) | **none** | ABSENT | A, H-14 |
| Storming | economy | Post-1.0 | [storming](../simulation/planned-economy/storming.md) | — | ABSENT | A-09, B2-15 |
| Reserve classes and confiscation | economy | Post-1.0 hook | [reserves](../simulation/planned-economy/reserves.md) | — | ABSENT | A §3e |
| Priority classes, priority inflation | economy | Post-1.0 | [priorities](../simulation/planned-economy/priorities.md) | logistics §005 (deficit → distance → ID) | ABSENT | A-05 |
| Material balance | economy | 1.0 — charter row Resources and production | [material balance](../simulation/planned-economy/material-balance.md) | — | ABSENT (`EcoStats` volumes only) | A-10 |
| Tolkachi, ministries, assortment, investment hunger, OTK, falsification | economy | Post-1.0 | [enterprise behaviour](../simulation/planned-economy/enterprise-behavior.md) | — | ABSENT | A §4 |
| Non-price domestic matching | economy | 1.0 — charter row Transport and border | [allocation](../simulation/physical-economy/allocation.md) | logistics §005 | EXISTS (`money_delta = 0`, distance sort) | E-113 |
| Domestic treasury debit (must retire) | economy | Never in scope | [construction](../simulation/physical-economy/construction.md) | trade | **CONTRADICTED** (treasury costs computed in `simulation/src/economy/government.rs:22`) | E-114/122 |
| Export-side physical clearance | trade | 1.0 — charter row Transport and border | [logistics](../simulation/physical-economy/logistics.md) | trade | EXISTS (physical export dispatch to the border door, money at delivery: the sell-side branch pushes before the dispatch loop, `simulation/src/economy/market.rs:1155-1230`; export money settles on the `ToDestination` arrival, `market.rs:1799-1801`, applied to `Government.money` via the `advance_dispatches` return in `simulation/src/economy/mod.rs:153-161`; every exit rolls back through the shared `terminate_dispatch` helper, `market.rs:455`) | E-121 |
| Import as physical truck | trade | 1.0 — charter row Transport and border | [logistics](../simulation/physical-economy/logistics.md) | trade, logistics | EXISTS (physical import truck drawing on a bounded border ledger, money at delivery: Border custody on the freight station, `simulation/src/souls/freight_station.rs:49-53`, `MAX_BORDER_STOCK` at `:37`, train restock at `:60-67`, `try_draw_border_stock` at `:71-77`; empty stock waits visibly in `ToSource`, `simulation/src/economy/market.rs:1549-1593`; import money settles on the `ToSource`→`Loading` arrival, `market.rs:1607-1609`; buyer stock credited at `ToDestination`, `:1795`) | E-128 |
| Truck dispatch lifecycle | logistics | 1.0 — charter row Transport and border | [logistics](../simulation/physical-economy/logistics.md) | logistics | EXISTS (ToSource→Loading→ToDestination→Unloading) | E-128 |
| Custody conservation on cancel/return | logistics | 1.0 — charter row Transport and border | [custody](../simulation/physical-economy/custody.md) | logistics | PARTIAL (bounded Loading/Returning route failure still deletes already-debited cargo, but every deletion site now records it in the named `Lost` sink — `sov-bub`, `Market::terminate_dispatch` at `simulation/src/economy/market.rs:449-461` with the border-money reversal at `:521-525`, `record_lost` at `:887-889`, `lost()` at `:881`, freight-station inspector rows at `native_app/src/gui/inspect/inspect_building.rs:153-156`; deletion behavior, bounds and retry counts unchanged per ADR-0003 §4) | E-136 |
| Cargo/capacity on the vehicle | logistics | 1.0 — charter row Transport and border | [custody](../simulation/physical-economy/custody.md) | vehicles, logistics | ABSENT | LOG-SUB-005 |
| Finite loading/unloading, dock rates | logistics | 1.0 — charter row Transport and border | [logistics](../simulation/physical-economy/logistics.md) | logistics §011 | ABSENT | D §4.3 |
| Handling classes | logistics | Post-1.0 hook | [resources](../simulation/physical-economy/resources.md) | — | ABSENT | bible §6.8 |
| Durable unmet demand (queue with age) | economy | 1.0 — charter row Households and citizens | [queues (concept)](../simulation/concepts/queues.md) | needs, production | PARTIAL (unmatched orders persist: humans never route externally at `simulation/src/economy/market.rs:1061-1077`, unserved non-human orders are re-inserted at `:1081-1117`, timed-out dispatches re-post via `terminate_dispatch` at `:455`; but there is no age-based shortage queue or Planner readout — matches `ECO-SUB-001` as narrowed in the substrate map) | ECO-SUB-001 |
| Production run bounded by inputs, storage, workforce | economy | 1.0 — charter row Resources and production | [production](../simulation/physical-economy/production.md) | production | EXISTS (`recipe_should_produce`) | E-002 |
| Binding-constraint record | economy | 1.0 — charter row Resources and production | [production](../simulation/physical-economy/production.md) | production | ABSENT | bible §6.9 |
| Construction Site: ghost, verdict, gates, ground broken | construction | 1.0 — charter row Planner interaction | [construction](../simulation/physical-economy/construction.md) | construction, buildings | ABSENT (instant placement) | E-015 |
| Auto-generated lots | zoning | Never in scope | [roads](../simulation/transport/roads.md) | zoning §003 | **CONTRADICTED** | MAP-SUB-002 |
| Citizen persistent identity | society | 1.0 — charter row Households and citizens | [citizens](../simulation/society/citizens.md) | citizens §001 | PARTIAL (`HumanEnt`, no lifecycle) | E-030 |
| Household entity, shared pantry | society | 1.0 — charter row Households and citizens | [households](../simulation/society/households.md) | households §004 | ABSENT | E-041 |
| Housing queue | society | 1.0 — charter row Households and citizens | [housing](../simulation/society/housing.md) | households | ABSENT | B1 §3b |
| Food and Meat as separate needs | society | 1.0 — charter row Resources and production | [provisioning](../simulation/society/provisioning.md) | needs | PARTIAL (bread only) | E-125 |
| Going without | society | 1.0 — charter row Households and citizens | [scarcity (concept)](../simulation/concepts/scarcity.md) | needs §004 | PARTIAL (food) | E-115 |
| Time budget, social-reproduction balance | society | Post-1.0 | [time](../simulation/society/time.md) | — | ABSENT | B1 §3a/3f |
| Citizen knowledge and search | society | Post-1.0 | [provisioning](../simulation/society/provisioning.md) | — | ABSENT (perfect knowledge) | B1 §3d |
| Blat / informal allocation | society | Post-1.0 | [social networks](../simulation/society/social-networks.md) | — | ABSENT | B1 §3c |
| Propiska, limitchiki, migration | society | Post-1.0 | [migration](../simulation/society/migration.md) | — | ABSENT | B1 §4 |
| Labour differentiation, tenure ramp, labour hoarding | society | Post-1.0 | [labor](../simulation/society/labor.md) | — | ABSENT (`workers / n_workers`) | B1 §3e |
| Education two tiers | society | 1.0 — charter row Agriculture and services | [education](../simulation/society/education.md) | education | ABSENT | E-099 |
| Healthcare, Medicine chain | society | 1.0 — charter row Agriculture and services | [healthcare](../simulation/society/healthcare.md) | healthcare | ABSENT | E-103 |
| Death | society | 1.0 — charter row Agriculture and services | [demography](../simulation/society/demography.md) | citizens | ABSENT | E-072 |
| Births, cohort expectations | society | Post-1.0 | [demography](../simulation/society/demography.md) | citizens (open Q) | ABSENT | B1 §3g |
| Work collectives, unions, local Soviets | institutions | Post-1.0 | [institutions](../simulation/society/institutions/index.md) | — | ABSENT | synthesis §3.5 |
| Lane-constrained vehicle motion | transport | 1.0 — charter row Transport and border | [vehicles](../simulation/transport/vehicles.md) | vehicles | EXISTS (kinematic) | D-02 |
| Mass/grade/traction physics | transport | Post-1.0 hook | [vehicles](../simulation/transport/vehicles.md) | vehicles | ABSENT | D §3.1 |
| Collision avoidance | transport | 1.0 — charter row Transport and border | [traffic](../simulation/transport/traffic.md) | traffic §004 | EXISTS (cone check, not IDM) | D-06 |
| EWMA/BPR/Gawron route cost | transport | 1.0 — charter row Transport and border | [traffic](../simulation/transport/traffic.md) | traffic §007/008 | ABSENT | D-08 |
| Spillback, meso traffic | transport | Post-1.0 | [traffic](../simulation/transport/traffic.md) | traffic §005 | ABSENT | D §3.8 |
| Junction deadlock resolution | transport | 1.0 — charter row Transport and border | [roads](../simulation/transport/roads.md) | roads | PARTIAL (random wait) | D §4.1 |
| Parking reservation | transport | 1.0 — charter row Transport and border | [roads](../simulation/transport/roads.md) | roads | EXISTS | MAP-SUB-005 |
| A* routing | transport | 1.0 — charter row Transport and border | [pathfinding](../simulation/transport/pathfinding.md) | pathfinding | EXISTS (flat, no load) | C2-10 |
| Freight rail (minimal) | transport | 1.0 — charter row Transport and border | [freight rail](../simulation/transport/freight-rail.md) | vehicles | PARTIAL (consist physics, reservations; no cargo/capacity) | D-03 |
| Signalling, yards, empty repositioning | transport | Post-1.0 | [freight rail](../simulation/transport/freight-rail.md) | — | ABSENT | D §4.2 |
| Public transport | transport | Post-1.0 | [public transport](../simulation/transport/public-transport-future.md) | — | ABSENT | D-12 |
| Electricity: wire, storage, priority shedding | infra | 1.0 — charter row Utilities | [electricity](../simulation/infrastructure/electricity.md) | electricity §001–003 | **CONTRADICTED** (road union-find, binary blackout) | D-10 |
| Water transfer, quality, border meter | infra | 1.0 — charter row Utilities | [water](../simulation/infrastructure/water.md) | water §001–006 | ABSENT | D-14 |
| Sewage | infra | Post-1.0 | [sewage](../simulation/infrastructure/sewage.md) | sewage | ABSENT | D-15 |
| Heating, no electric fallback | infra | 1.0 — charter row Utilities | [heating](../simulation/infrastructure/heating.md) | heating §001 | ABSENT | D-16 |
| Waste | infra | 1.0 — charter row Utilities | [waste](../simulation/infrastructure/waste.md) | waste | ABSENT | — |
| Reservoir, hydro | infra | 1.0 — charter row Terrain and environment | [hydrology](../simulation/infrastructure/hydrology.md) | **none** | ABSENT | D-19 |
| Weather | infra | 1.0 — charter row Presentation and audio | [network architecture](../simulation/infrastructure/network-architecture.md) | **none** | ABSENT | D §4.9 |
| Gas linepack | infra | Post-1.0 | [network architecture](../simulation/infrastructure/network-architecture.md) | — | ABSENT | D-18 |
| Network reserves view | UI | 1.0 — charter row Planner interaction | [reserves (concept)](../simulation/concepts/reserves.md) | — | ABSENT | D-21 |
| Causal inspector STATUS/CAUSE/TREND/POLICY/CHAIN | UI | 1.0 — charter row Planner interaction | [causality (arch)](../architecture/causality.md) | **none** | ABSENT | E-105 |
| Planner snapshot / information boundary | arch | 1.0 — charter row Planner interaction | [snapshots (arch)](../architecture/snapshots.md) | — | ABSENT (UI omniscient) | C2-06 |
| Game modes, national projects | product | Post-1.0 | [game modes](../product/game-modes.md) | — | ABSENT | H |
| Lockstep multiplayer | net | Post-1.0 | [parallelism (arch)](../architecture/parallelism.md) | — | EXISTS (`networking/`) | G-28, C2-14 |
| Crime | society | Post-1.0 | — (no simulation design page; see specification) | [crime](specifications/crime.md) | ABSENT (no mechanism; `simulation/src/map/objects/building.rs:17-22`) | — |
| Resources catalogue and stock (incl. import-only Medicine) | economy | 1.0 — charter row Resources and production | [resources](../simulation/physical-economy/resources.md) | — | PARTIAL (21-name catalogue with no `medicine`, `base_mod/items.lua`; `job-opening` declared as an item, `base_mod/items.lua:1-7`) | — |

## Related

- [Authority index](authority-index.md)
- [Invariants index](invariants.md)
- [Current substrate](../architecture/current-substrate.md)
- [Lane E code-gap matrix](../research/conversation-mining-2026-08-28/E-code-gap-matrix.md) — 136 rows, the source of most "Current" verdicts
