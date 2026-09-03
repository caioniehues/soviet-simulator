# Construction

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** construction
**Last verified:** 2026-08-28

| Scope | 1.0 binding |

## What this is

Construction turns a Planner-approved placement into a physical, non-operating building.
It is production over time: materials arrive by truck, work progresses through gates, and the
building activates only after every bill quantity and work gate is satisfied.

The lifecycle is: Ghost → Verdict → Site → gates → ground broken → completion → activation.
Before ground is broken, the Planner can rescind the order. After ground is broken, the
materials are physically committed and rescind is refused.

"Automate execution, not decisions" — the Planner decides what to build and where. The
simulation handles material delivery, work scheduling, and completion. The Planner does not
micro-manage every truck to a construction site.

## 1.0 requirement

`SPEC-CONSTRUCTION-001` — a Ghost MUST show the footprint, full material bill, and refusal
reason before commit.

`SPEC-CONSTRUCTION-002` — an approved proposal creates one non-operating Site, not a
completed building.

`SPEC-CONSTRUCTION-003` — a material bill is a quantity of Resources catalogue identities
and units, never a domestic rouble price.

`SPEC-CONSTRUCTION-004` — rescind before ground is broken MUST cancel the proposal/Site
and release outstanding reservations. After ground is broken, rescind is refused.

`SPEC-CONSTRUCTION-005` — work gates require physical inputs and eligible work. Elapsed
ticks alone MUST NOT complete a gate.

`SPEC-CONSTRUCTION-006` — completion occurs only after every required bill quantity and work
gate is satisfied.

`SPEC-CONSTRUCTION-008` — construction conservation: for every item,
`ΣH_onhand + ΣC_haul + ΣC_embedded = initial + declared_sources - declared_other_sinks`.

## Target design

Capital dilution (PLAUSIBLE, bible §6.13): construction consumes physical resources that
could have gone to production. Housing competes with factory expansion for the same steel
and lumber. This is the physical opportunity cost the Planner must manage.

Construction offices and crews (HYPOTHESIS, future): a construction office dispatches crews
to sites. Without a crew, the site stalls. This adds a labour bottleneck to construction
and is an instrument the Planner controls.

Auto-lots (`MAP-SUB-002`, `SPEC-ZONING-003`): current road construction generates roadside
lots automatically (`simulation/src/map/map.rs:682-720`). This conflicts with the target
placement contract. `SPEC-ZONING-003` forbids spawning from intent — a later explicit
Construction proposal is required for every physical change.

## Current substrate

Building placement is immediate. `MapBuildSpecialBuilding` in
`simulation/src/world_command.rs:284-299` calls `Map::build_special_building`, which
immediately inserts `BuildingInfos`. The scheduler then creates souls for ownerless
buildings (`simulation/src/souls/mod.rs:16-54`). There is no Site, no material bill, no
work gate, no construction phase.

The command path debits `Government.money` (`world_command.rs:223-225`) — a domestic rouble
cost in a supposedly non-price domain. This conflicts with the charter and with
`SPEC-CONSTRUCTION-003`.

Auto-lots are generated on every non-arbitrary road construction (`map/map.rs:682-720`).

## Open questions

- Which building declarations determine bill quantities and work gates?
- Which named work authority supplies eligible labour?
- Which physical prerequisites belong in a Verdict for each building class?

## Related

- [Resources](resources.md)
- [Logistics](logistics.md)
- [Storage](storage.md)
- [Roads](../transport/roads.md)
- [Construction spec](../../reference/specifications/construction.md)
- [Buildings spec](../../reference/specifications/buildings.md)
- [Zoning spec](../../reference/specifications/zoning.md)
