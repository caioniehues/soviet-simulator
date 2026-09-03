# Mechanics index

**Kind:** reference
**Authority:** operational — a navigation table, not a specification; the "Current" column is observational at the verified commit
**Status:** active
**Owner:** project lead
**Verified-at:** `4e9e930b2a73`
**Last verified:** 2026-08-28

One row per mechanic. Scope: **1.0** (charter + draft spec), **cand.** (in charter scope, no spec),
**hook** (design the data now, mechanic later), **P1.0** (Post-1.0), **res.** (research only).
Current: **EXISTS**, **PARTIAL**, **ABSENT**, **CONTRADICTED** (the code does the opposite of the
pillar).

| Mechanic | Domain | Scope | Design page | Specification | Current | Research |
|---|---|---|---|---|---|---|
| Request vs allocation vs reservation vs receipt vs consumption | economy | 1.0 | [requests](../simulation/physical-economy/requests.md) | production §003, logistics | PARTIAL (`capital`, `reserved`, `requested`) | E-005 |
| Enterprise over-requesting (dishonest enterprise) | economy | 1.0 | [enterprise behaviour](../simulation/planned-economy/enterprise-behavior.md) | production §009 | PARTIAL — wired, not observable | A-02/03, E-007 |
| Storage-capacity floor on hoarding | economy | 1.0 | [reserves (concept)](../simulation/concepts/reserves.md) | production | EXISTS (`storage_multiplier`) | synthesis §3.2 |
| Adaptive request inflation from reliability | economy | P1.0 | [reliability and buffering](../simulation/planned-economy/reliability-and-buffering.md) | — | ABSENT | A §3a |
| Planning credibility record | economy | P1.0 | [reliability and buffering](../simulation/planned-economy/reliability-and-buffering.md) | — | ABSENT | A §3d |
| Ratchet | economy | P1.0 | [reliability and buffering](../simulation/planned-economy/reliability-and-buffering.md) | — | ABSENT | A-12, G-32 |
| Plan periods, quotas | economy | cand. (charter: three Plans) | [plan cycle](../simulation/planned-economy/plan-cycle.md) | **none** | ABSENT | A, H-14 |
| Storming | economy | P1.0 | [storming](../simulation/planned-economy/storming.md) | — | ABSENT | A-09, B2-15 |
| Reserve classes and confiscation | economy | P1.0 | [reserves](../simulation/planned-economy/reserves.md) | — | ABSENT | A §3e |
| Priority classes, priority inflation | economy | P1.0 | [priorities](../simulation/planned-economy/priorities.md) | logistics §005 (deficit → distance → ID) | ABSENT | A-05 |
| Material balance | economy | cand. | [material balance](../simulation/planned-economy/material-balance.md) | — | ABSENT (`EcoStats` volumes only) | A-10 |
| Tolkachi, ministries, assortment, investment hunger, OTK, falsification | economy | P1.0 | [enterprise behaviour](../simulation/planned-economy/enterprise-behavior.md) | — | ABSENT | A §4 |
| Non-price domestic matching | economy | 1.0 | [allocation](../simulation/physical-economy/allocation.md) | logistics §005 | EXISTS (`money_delta = 0`, distance sort) | E-113 |
| Domestic money gate (buildings, roads, wages) | economy | must retire | [construction](../simulation/physical-economy/construction.md) | trade | **CONTRADICTED** | E-114/122 |
| Export-side physical clearance | trade | 1.0 | [logistics](../simulation/physical-economy/logistics.md) | trade | **CONTRADICTED** (teleport at match) | E-121 |
| Import as physical truck | trade | 1.0 | [logistics](../simulation/physical-economy/logistics.md) | trade, logistics | EXISTS (`sov-abs`) | E-128 |
| Truck dispatch lifecycle | logistics | 1.0 | [logistics](../simulation/physical-economy/logistics.md) | logistics | EXISTS (ToSource→Loading→ToDestination→Unloading) | E-128 |
| Custody conservation on cancel/return | logistics | 1.0 | [custody](../simulation/physical-economy/custody.md) | logistics | EXISTS (ledger tests) | E-136 |
| Cargo/capacity on the vehicle | logistics | 1.0 | [custody](../simulation/physical-economy/custody.md) | vehicles, logistics | ABSENT | LOG-SUB-005 |
| Finite loading/unloading, dock rates | logistics | 1.0 | [logistics](../simulation/physical-economy/logistics.md) | logistics §011 | ABSENT | D §4.3 |
| Handling classes | logistics | hook | [resources](../simulation/physical-economy/resources.md) | — | ABSENT | bible §6.8 |
| Durable unmet demand (queue with age) | economy | 1.0 | [queues (concept)](../simulation/concepts/queues.md) | needs, production | **CONTRADICTED** (`mem::take`) | ECO-SUB-001 |
| Production run bounded by inputs, storage, workforce | economy | 1.0 | [production](../simulation/physical-economy/production.md) | production | EXISTS (`recipe_should_produce`) | E-002 |
| Binding-constraint record | economy | 1.0 | [production](../simulation/physical-economy/production.md) | production | ABSENT | bible §6.9 |
| Construction Site: ghost, verdict, gates, ground broken | construction | 1.0 | [construction](../simulation/physical-economy/construction.md) | construction, buildings | ABSENT (instant placement) | E-015 |
| Auto-generated lots | zoning | must retire | [roads](../simulation/transport/roads.md) | zoning §003 | **CONTRADICTED** | MAP-SUB-002 |
| Citizen persistent identity | society | 1.0 | [citizens](../simulation/society/citizens.md) | citizens §001 | PARTIAL (`HumanEnt`, no lifecycle) | E-030 |
| Household entity, shared pantry | society | 1.0 | [households](../simulation/society/households.md) | households §004 | ABSENT | E-041 |
| Housing queue | society | 1.0 | [housing](../simulation/society/housing.md) | households | ABSENT | B1 §3b |
| Food and Meat as separate needs | society | 1.0 | [provisioning](../simulation/society/provisioning.md) | needs | PARTIAL (bread only) | E-125 |
| Going without | society | 1.0 | [scarcity (concept)](../simulation/concepts/scarcity.md) | needs §004 | PARTIAL (food) | E-115 |
| Time budget, social-reproduction balance | society | P1.0 | [time](../simulation/society/time.md) | — | ABSENT | B1 §3a/3f |
| Citizen knowledge and search | society | P1.0 | [provisioning](../simulation/society/provisioning.md) | — | ABSENT (perfect knowledge) | B1 §3d |
| Blat / informal allocation | society | P1.0 | [social networks](../simulation/society/social-networks.md) | — | ABSENT | B1 §3c |
| Propiska, limitchiki, migration | society | P1.0 | [migration](../simulation/society/migration.md) | — | ABSENT | B1 §4 |
| Labour differentiation, tenure ramp, labour hoarding | society | P1.0 | [labor](../simulation/society/labor.md) | — | ABSENT (`workers / n_workers`) | B1 §3e |
| Education two tiers | society | 1.0 | [education](../simulation/society/education.md) | education | ABSENT | E-099 |
| Healthcare, Medicine chain | society | 1.0 | [healthcare](../simulation/society/healthcare.md) | healthcare | ABSENT | E-103 |
| Death | society | 1.0 | [demography](../simulation/society/demography.md) | citizens | ABSENT | E-072 |
| Births, cohort expectations | society | open / P1.0 | [demography](../simulation/society/demography.md) | citizens (open Q) | ABSENT | B1 §3g |
| Work collectives, unions, local Soviets | institutions | P1.0 | [institutions](../simulation/society/institutions/index.md) | — | ABSENT | synthesis §3.5 |
| Lane-constrained vehicle motion | transport | 1.0 | [vehicles](../simulation/transport/vehicles.md) | vehicles | EXISTS (kinematic) | D-02 |
| Mass/grade/traction physics | transport | hook | [vehicles](../simulation/transport/vehicles.md) | vehicles | ABSENT | D §3.1 |
| Collision avoidance | transport | 1.0 | [traffic](../simulation/transport/traffic.md) | traffic §004 | EXISTS (cone check, not IDM) | D-06 |
| EWMA/BPR/Gawron route cost | transport | 1.0 | [traffic](../simulation/transport/traffic.md) | traffic §007/008 | ABSENT | D-08 |
| Spillback, meso traffic | transport | P1.0 | [traffic](../simulation/transport/traffic.md) | traffic §005 | ABSENT | D §3.8 |
| Junction deadlock resolution | transport | cand. | [roads](../simulation/transport/roads.md) | roads | PARTIAL (random wait) | D §4.1 |
| Parking reservation | transport | 1.0 | [roads](../simulation/transport/roads.md) | roads | EXISTS | MAP-SUB-005 |
| A* routing | transport | 1.0 | [pathfinding](../simulation/transport/pathfinding.md) | pathfinding | EXISTS (flat, no load) | C2-10 |
| Freight rail (minimal) | transport | 1.0 | [freight rail](../simulation/transport/freight-rail.md) | vehicles | PARTIAL (consist physics, reservations; no cargo/capacity) | D-03 |
| Signalling, yards, empty repositioning | transport | P1.0 | [freight rail](../simulation/transport/freight-rail.md) | — | ABSENT | D §4.2 |
| Public transport | transport | P1.0 | [public transport](../simulation/transport/public-transport-future.md) | — | ABSENT | D-12 |
| Electricity: wire, storage, priority shedding | infra | 1.0 | [electricity](../simulation/infrastructure/electricity.md) | electricity §001–003 | **CONTRADICTED** (road union-find, binary blackout) | D-10 |
| Water transfer, quality, border meter | infra | 1.0 | [water](../simulation/infrastructure/water.md) | water §001–006 | ABSENT | D-14 |
| Sewage | infra | 1.0 | [sewage](../simulation/infrastructure/sewage.md) | sewage | ABSENT | D-15 |
| Heating, no electric fallback | infra | 1.0 | [heating](../simulation/infrastructure/heating.md) | heating §001 | ABSENT | D-16 |
| Waste | infra | 1.0 | [waste](../simulation/infrastructure/waste.md) | waste | ABSENT | — |
| Reservoir, hydro | infra | 1.0 | [hydrology](../simulation/infrastructure/hydrology.md) | **none** | ABSENT | D-19 |
| Weather | infra | 1.0 (seasons) | [network architecture](../simulation/infrastructure/network-architecture.md) | **none** | ABSENT | D §4.9 |
| Gas linepack | infra | P1.0 | [network architecture](../simulation/infrastructure/network-architecture.md) | — | ABSENT | D-18 |
| Network reserves view | UI | cand. | [reserves (concept)](../simulation/concepts/reserves.md) | — | ABSENT | D-21 |
| Causal inspector STATUS/CAUSE/TREND/POLICY/CHAIN | UI | cand. | [causality (arch)](../architecture/causality.md) | **none** | ABSENT | E-105 |
| Planner snapshot / information boundary | arch | cand. | [snapshots (arch)](../architecture/snapshots.md) | — | ABSENT (UI omniscient) | C2-06 |
| Game modes, national projects | product | P1.0 | [game modes](../product/game-modes.md) | — | ABSENT | H |
| Lockstep multiplayer | net | unresolved | [parallelism (arch)](../architecture/parallelism.md) | — | EXISTS (`networking/`) | G-28, C2-14 |

## Related

- [Authority index](authority-index.md)
- [Invariants index](invariants.md)
- [Current substrate](../architecture/current-substrate.md)
- [Lane E code-gap matrix](../research/conversation-mining-2026-08-28/E-code-gap-matrix.md) — 136 rows, the source of most "Current" verdicts
