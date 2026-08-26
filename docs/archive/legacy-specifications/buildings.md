# Buildings

**Status:** draft model (grounded in research)
**Phase:** 1
**Primary inspiration:** W&R authored-prefab grammar (kept) + CS1 growable lifecycle (mined for parts)
**Evidence:** see [research/zoning-buildings.md](../research/zoning-buildings.md) for CS1-code and W&R-data sources.

> Player-placed buildings, activated only after a physical construction project completes.

## Purpose

The unified model of a building as a **physical asset**: declared capabilities (dwelling slots, workplaces, storages, service capacity, connection points), a lifecycle (planned → under construction → operating → degrading → demolished), and activation gated on physical completion — never on payment.

## Draft model

### Declaration grammar — adopt W&R's wholesale (**CONFIRMED** source, OURS adoption)

A building is authored flat data (research §E1, from the 488-file corpus). The declaration axes:

- **Function type** — one type token per building (`$TYPE_LIVING` ×54, `$TYPE_FACTORY` ×52, `$TYPE_SHOP`, `$TYPE_UNIVERSITY`, …). The type *is* the "what does this building do" declaration; no separate zone/class layer.
- **Capacities** — storages per transport class (`$STORAGE <class> <n>`), consumption/supply as `$STORAGE_DEMAND_*`; dwelling capacity is a people-bucket ([research/households.md](../research/households.md) §D1).
- **Workforce** — `$WORKERS_NEEDED n`, `$CITIZEN_ABLE_SERVE n`. Staffing is declared; workers are *sourced* at runtime by labour allocation ([spec/citizens.md](citizens.md)).
- **Connections** — explicit plug points with literal coordinates (`$CONNECTION_ROAD_DEAD`, `$CONNECTION_HEATING_*`, station nodes). The building declares where it joins the networks; wiring them up is physical work.
- **Construction cost** — materials + work phases (`$COST_RESOURCE_AUTO`, `$COST_WORK <phase>`), covered in [spec/construction.md](construction.md).

This split is the design principle: **capacity lives in data, policy lives in the plan.** (research §G6)

### The CS1 anti-model — spawning and market leveling (**CONFIRMED**, rejected)

- Growables are engine-spawned (`ZoneBlock.SimulationStep` → `GetRandomBuildingInfo`), construct over a fixed timer, then run. Spawn-by-demand is rejected — placement is a plan act ([spec/zoning.md](zoning.md)).
- CS1 leveling: a growable targets `min(residentEducationLevel, landValueLevel)` and densifies in place via `StartUpgrading` (`ResidentialBuildingAI.cs:658-822`), appending capacity without evicting. The *trigger* (land value market) is rejected; the *mechanic* (same lot, more capacity, never evict on upgrade) is kept — see renovation below.
- CS1 decay: land-value-too-low escalates to MajorProblem → the 64-step abandonment timer ([research/households.md](../research/households.md) §B3). The *pressure* is kept, re-motivated as physical starvation.

### Lifecycle (OURS, grounded in research §G)

1. **Planned** — a project sited on authorised land ([spec/zoning.md](zoning.md)), validated by the physical checklist.
2. **Under construction** — phased, physical; materials hauled, work phases spent ([spec/construction.md](construction.md)).
3. **Operating** — declared capabilities active. Capacity is fixed by the built asset, never by a budget slider.
4. **Degrading** — a building starved of maintenance inputs (heat, repairs, materials) loses quality (W&R `$QUALITY_OF_LIVING` as the residential precedent) and can become uninhabitable — CS1's problem-timer shape, re-motivated as material/service starvation, not land value. (research §G4, §G7)
5. **Renovation/expansion** — in-place project: the plan allocates labour + materials, the building densifies or improves on the same lot, existing occupants stay (CS1's append-units-on-upgrade behaviour). (research §G4)
6. **Demolition** — a planned physical act requiring crews and hauling (W&R `$TYPE_DEMOLITION_OFFICE`); reclaims some materials, frees land. Never click-to-vaporize; CS1's zone-mismatch auto-despawn is rejected. (research §G7)

## Open questions

- ~~Building condition/maintenance as resource sink — precedent or fully OURS?~~ Partly settled: decay pressure has CS1 precedent (problem timers) and W&R a quality scalar, but a *maintenance-input* sink (materials/labour to hold condition) is OURS; W&R declares no upkeep grammar (research §E2 absence).
- ~~Renovation: in-place project or replace-and-rebuild?~~ Settled by research §G4: in-place project, append capacity, never evict.
- One unified Building entity with capability components (ECS) vs per-type classes — Phase 2 question; the W&R grammar (one `$TYPE_*` + orthogonal capacity/connection tokens) argues for components.
- Does degradation ever complete on its own (ruin state) or always stop at uninhabitable-until-renovated?
- Multi-building enterprises (factory + attached hostel + siding) — one asset or a composition?

## Evidence log

| Claim | Evidence level | Source | Notes |
|---|---|---|---|
| W&R building = flat token file: type, storages, workers, connections, construction | CONFIRMED | `buildings_types/*.ini` (488 files), e.g. `shop_grocerystore.ini` | research §E1 |
| W&R has no level/upgrade grammar; one prefab = one form | CONFIRMED (absence) | corpus sweep | research §E2 |
| CS1 growables spawn engine-side, construct on a timer | CONFIRMED | `ZoneBlock.cs:1556-1609`, `PrivateBuildingAI.cs:119-129` | research §B3-B4 |
| CS1 leveling = `min(education, landValue)` → densify in place | CONFIRMED | `ResidentialBuildingAI.cs:658-822` | research §C1 |
| CS1 land-value collapse → MajorProblem → abandonment | CONFIRMED | `ResidentialBuildingAI.cs:756-804` | research §C2 |
| Lifecycle with renovation-as-project, starvation decay, physical demolition | OURS | research §G4, §G6-G7 | this spec's model |

Evidence levels: CONFIRMED · OBSERVED · INFERRED · SPECULATIVE · OURS (see [spec/README](README.md)).

## Related

- [spec/zoning.md](zoning.md) — land authorisation and siting
- [spec/construction.md](construction.md) — how buildings come to exist
- [spec/production.md](production.md) / [spec/households.md](households.md) — what operating buildings do
- Research: [research/zoning-buildings.md](../research/zoning-buildings.md)
