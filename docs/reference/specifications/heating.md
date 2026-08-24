# Heating specification

**Kind:** specification
**Authority:** binding
**Status:** draft
**Owner:** utilities
**Last verified:** 2026-08-24

The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT, RECOMMENDED, NOT RECOMMENDED, MAY, and OPTIONAL in this document are to be interpreted as described in RFC 2119 and RFC 8174.

## Purpose

Heating is a finite thermal-flow utility. It transfers produced heat through its own connected infrastructure; shortfall becomes observable warmth shortage. Temperature-responsive demand requires a ratified Weather interface. Electricity cannot substitute for physical heat.

## Invariants

- `SPEC-HEATING-001` — Heating SHALL be sole authority for thermal topology, endpoint attachment, generation offer, buffer, pipe/pump capacity, declared loss, transfer progress, and served/unmet thermal rate. Electricity MUST NOT satisfy heat shortfall.
- `SPEC-HEATING-002` — For each thermal allocation, generated heat `G` plus buffer discharge `D` SHALL equal served heat `V` plus buffer charge `C` plus named loss `L`: `G + D = V + C + L`. Buffer updates exactly as `B_next = B + C - D`, with `0 <= B_next <= B_capacity`; generation, discharge, charge, service, and loss are bounded by declared tick rate/capacity. Loss has a named physical/environmental destination.
- `SPEC-HEATING-003` — Variable temperature demand MAY be evaluated only after ratified Weather supplies the referenced observation. Before then Heating uses declared static demand and MUST NOT claim weather-driven behaviour.
- `SPEC-HEATING-004` — Disconnection, insufficient generation, finite capacity, or depleted buffer leaves served/unmet heat with age/reason. Needs may translate it to colder homes/going without; no module may create heat or end the plan.
- `SPEC-HEATING-005` — Priority is explicit and non-price-based. Production, Buildings, Needs, and Weather reference Heating results and MUST NOT duplicate flow, storage, or topology.
- `SPEC-HEATING-006` — Heating SHALL apply at most one immutable `HeatAllocationID` for one network and tick. A replay is a no-op; no generated or discharged unit may be served, charged, or lost more than once.
- `SPEC-HEATING-007` — Generation `G` SHALL derive from one immutable, input/source/capacity-bounded Production-owned plant `ProductionRunID`/heat-generation result accepted once by Heating as an offer. A fuelled plant result names its compatible Resources input debit; any non-fuel producer names its declared physical/environmental source and bounded prototype capacity. Heating MUST NOT mint `G`, replay the result, or create an offer without that result/source.

## Model and state

Heating owns `HeatNodeID`, pipe/pump graph, endpoint attachment, accepted generation-offer reference, demand, buffer, capacity, loss, `HeatAllocationID`, and `HeatServiceResultID`. Production owns each plant `ProductionRunID`/heat-generation result and its input/source record; Heating accepts that result once as `G`. Needs receives only served/unmet heat. Weather, once ratified, publishes immutable observation; Heating does not own weather state.

## Failure behavior and observability

Shortfall is finite unmet demand and may make homes/workplaces colder under Needs/Buildings rules. Recovery is a later rate-bounded flow. There is no electric fallback, CHP bypass, price priority, or game over. The Planner can inspect pipe/path, generation, demand, buffer, capacity, loss, service result, priority, age/reason, and Weather reference or static-demand label.

## Acceptance evidence

All guards are **UNIMPLEMENTED** and block ratification. A zero-test command is failure; the current serial suite proves no target below.

| Evidence | Future guard command and observable assertion | Required red mutation | Player-facing proof |
|---|---|---|---|
| `EVID-HEATING-001` | `cargo test -p simulation evid_heating_finite_flow_storage_loss -- --test-threads=1` — every allocation proves `G + D = V + C + L`, `B_next = B + C - D`, capacity/rate bounds, and one `HeatAllocationID` application. | Replay an allocation ID, serve plus lose more than source, over-allocate a tick, or change buffer without signed flow. | Inspected allocation ID, thermal-flow timeline, and named-loss capture. |
| `EVID-HEATING-002` | `cargo test -p simulation evid_heating_shortage_no_electric_fallback -- --test-threads=1` — shortfall remains unmet and adds no electric/CHP service. | Satisfy from Electricity or generate power+heat together. | Inspected shortage capture. |
| `EVID-HEATING-003` | `cargo test -p simulation evid_heating_weather_prerequisite -- --test-threads=1` — variable demand rejects missing Weather and accepts ratified reference. | Derive temperature from timer/unowned field. | Inspected Weather/demand capture. |
| `EVID-HEATING-004` | `cargo test -p simulation evid_heating_nonprice_priority -- --test-threads=1` — declared heating priority orders equal-capacity demand identically despite reversed rouble balances. | Rank or debit/credit by roubles. | Inspected allocation order and unchanged-money capture. |
| `EVID-HEATING-005` | `cargo test -p simulation evid_heating_authority_references_not_copies -- --test-threads=1` — consumers cannot mutate Heating state. | Add consumer thermal balance. | Inspected authority links. |
| `EVID-HEATING-006` | `cargo test -p simulation evid_heating_generation_requires_production_result -- --test-threads=1` — each accepted `G` references one once-applied Production plant result: fuelled heat includes compatible Resources debit; any non-fuel heat includes declared physical/environmental source and bounded prototype capacity. | Set `G` without a Production result, omit fuel input debit or non-fuel declared source/capacity, or replay a generation result. | Inspected ProductionRunID, source/input, capacity, accepted offer, and replay-no-op capture. |

## Substrate and decisions

No heating kind or registered heating/weather system exists (`simulation/src/map/objects/building.rs:17-37`; `simulation/src/init.rs:52-70`). The only flow is aggregate binary electricity blackout (`simulation/src/map_dynamic/electricity.rs:40-92`), which cannot prove thermal behaviour. The [Wave 2 fact-sheet](../../research/fact-sheets/wave2-substrate.md#2c--utilities-electricity-water-sewage-heating-and-waste) records no heating save/UI/test surface. Legacy `spec/heating.md` is rewrite input only.

## Deferred behavior

CHP, electric-heating fallback, voltage tiers, fuel lifecycle, and unratified Weather mechanics have no 1.0 acceptance criteria.

## Open questions

- Which heat sources and endpoint classes are 1.0?
- Which Weather observation contract is required before variable demand?
