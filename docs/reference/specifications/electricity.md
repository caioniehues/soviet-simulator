# Electricity specification

**Kind:** specification
**Authority:** binding
**Status:** draft
**Owner:** utilities
**Last verified:** 2026-08-24

The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT, RECOMMENDED, NOT RECOMMENDED, MAY, and OPTIONAL in this document are to be interpreted as described in RFC 2119 and RFC 8174.

## Purpose

Electricity is a finite, continuously delivered utility. This draft replaces the current road-derived binary blackout with an explicit wire-topology target. Voltage tiers, transformers, combined heat and power (CHP), and electric-heating fallback are charter cuts.

## Invariants

- `SPEC-ELECTRICITY-001` — Electricity SHALL be sole authority for explicit wire topology, endpoint attachment, network membership, generation/load rates, storage, and service result. A road, intersection, or building road link MUST NOT itself be an electrical connection.
- `SPEC-ELECTRICITY-002` — For each network allocation, generated energy `G` plus storage discharge `D` SHALL equal served energy `V` plus storage charge `C` plus named loss `L`: `G + D = V + C + L`. Storage updates exactly as `B_next = B + C - D`, with `0 <= B_next <= B_capacity`; generation, discharge, charge, service, and loss are each bounded by their declared per-tick rate/capacity. Neither blackout nor retry creates or deletes energy.
- `SPEC-ELECTRICITY-003` — Under shortage, Electricity SHALL apply declared continuous, non-price priority load shedding and record served, curtailed, and unmet rate. It MUST NOT replace that record with one binary network result.
- `SPEC-ELECTRICITY-004` — A disconnected endpoint, zero generation, depleted storage, or wire-capacity limit leaves a visible curtailed/queued demand with its binding reason. It MUST NOT activate a building, debit domestic money, create power, or end the plan.
- `SPEC-ELECTRICITY-005` — Production, Buildings, Water, Sewage, Heating, and Waste consume a referenced service result only. They MUST NOT copy or mutate Electricity topology, rate, or storage.
- `SPEC-ELECTRICITY-006` — Electricity SHALL apply at most one immutable `ElectricityAllocationID` for one network and tick. A replay is a no-op; the same generated or discharged unit MUST NOT be both served and charged, or allocated twice.
- `SPEC-ELECTRICITY-007` — Generation `G` SHALL derive from one immutable, input-bounded Production-owned plant `ProductionRunID`/generation result accepted once by Electricity as an offer. A fuelled plant result names its compatible Resources input debit; a non-fuel producer such as solar still names its declared physical/environmental source and bounded prototype capacity. Electricity MUST NOT mint `G`, replay the result, or create an offer without that result/source.

## Model and state

Electricity owns `WireID`, endpoint attachment, network membership, wire capacity, the accepted generation offer reference, load request, storage charge, tick allocation, curtailment reason, and `ElectricityAllocationID`. Production owns each plant `ProductionRunID`/generation result and its input/source record; Electricity accepts that result once as `G`. Buildings expose referenced endpoints; Production receives only a served-rate result and remains responsible for physical inputs/outputs. The Planner can set explicit non-price load priorities.

## Failure behavior and observability

Finite capacity, missing generation, and depleted storage create inspectable curtailment and downstream physical shortage. Recovery is a later connected, rate-bounded allocation, never a timer or retroactive stock change. The Planner can inspect endpoint, wire path/capacity, generation, requested/served rate, storage, priority, curtailment age/reason, and downstream result; a no-power icon alone is insufficient.

## Acceptance evidence

All guards are **UNIMPLEMENTED** and block ratification. A zero-test command is failure; the current serial suite proves no target below.

| Evidence | Future guard command and observable assertion | Required red mutation | Player-facing proof |
|---|---|---|---|
| `EVID-ELECTRICITY-001` | `cargo test -p simulation evid_electricity_wire_topology_not_roads -- --test-threads=1` — road-connected unwired endpoints are disconnected; a wire path serves them. | Treat a road/intersection as a wire edge. | Inspected topology/endpoint capture. |
| `EVID-ELECTRICITY-002` | `cargo test -p simulation evid_electricity_rate_storage_conservation -- --test-threads=1` — every allocation proves `G + D = V + C + L`, `B_next = B + C - D`, and all rate/capacity bounds; one `ElectricityAllocationID` applies once. | Charge and serve the same generated unit, replay one allocation ID, exceed a rate, or change storage without signed flow. | Inspected allocation ID, rate/energy/storage timeline, and named-loss capture. |
| `EVID-ELECTRICITY-003` | `cargo test -p simulation evid_electricity_priority_brownout_continuous -- --test-threads=1` — shortage partly serves higher declared priority, curtails lower priority, records each rate, and produces the same order despite reversed rouble balances. | Rank/debit by roubles, set one network blackout flag, or over-serve. | Inspected priority, rouble-independent order, and recovery capture. |
| `EVID-ELECTRICITY-004` | `cargo test -p simulation evid_electricity_reasoned_shortage_no_activation_or_money -- --test-threads=1` — zero generation, depleted storage, and wire limit retain a reasoned unmet record; no endpoint activates and no domestic-money balance changes. | Complete a timer-based activation, debit/credit roubles, or omit the binding reason. | Inspected shortage reason, inactive endpoint, and unchanged-money capture. |
| `EVID-ELECTRICITY-005` | `cargo test -p simulation evid_electricity_authority_references_not_copies -- --test-threads=1` — consumers cannot mutate Electricity state. | Add a consumer-owned electrical balance. | Inspected authority links. |
| `EVID-ELECTRICITY-006` | `cargo test -p simulation evid_electricity_generation_requires_production_result -- --test-threads=1` — each accepted `G` references one once-applied Production plant result: fuelled generation includes compatible Resources debit; solar/non-fuel generation includes declared physical/environmental source and bounded prototype capacity. | Set `G` without a Production result, omit fuel input debit or non-fuel declared source/capacity, or replay a generation result. | Inspected ProductionRunID, source/input, capacity, accepted offer, and replay-no-op capture. |

## Substrate and decisions

The current cache models buildings, roads, and intersections (`simulation/src/map/electricity_cache.rs:6-63`), rebuilds edges from building-road and road/intersection adjacency (`simulation/src/map/electricity_cache.rs:203-279`), and inserts building-road edges on creation (`simulation/src/map/objects/building.rs:132-153`). Flow sums aggregate power then records binary blackout (`simulation/src/map_dynamic/electricity.rs:40-92`); company productivity becomes zero under that flag (`simulation/src/souls/goods_company.rs:91-110`). It is scheduled/saved (`simulation/src/init.rs:52-80`) but the cache rebuilds on load (`simulation/src/map/serializing.rs:35-51`); Lua declares power fields (`base_mod/companies.lua:1-115`) and the HUD shows only a marker (`native_app/src/gui/hud.rs:61-105`). These are target conflicts, not proof. See the [Wave 2 fact-sheet](../../research/fact-sheets/wave2-substrate.md#2c--utilities-electricity-water-sewage-heating-and-waste).

## Deferred behavior

Voltage tiers, transformers, grid depth, CHP, and electric-heating fallback have no 1.0 acceptance criteria here.

## Open questions

- Which endpoint classes and priority categories are 1.0?
- Is storage required in the first ratified implementation?
