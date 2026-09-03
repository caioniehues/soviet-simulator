# Electricity

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** infrastructure
**Last verified:** 2026-08-28

| Scope | 1.0 binding |

## What this is

Electricity is a finite, continuously delivered utility. Generation plus storage discharge
equals served energy plus storage charge plus named loss: `G + D = V + C + L`. Under
shortage, the Planner sets explicit non-price load priorities: hospitals before factories
before houses. The result is brownout, not blackout — partial service with visible
curtailment reasons.

Electricity's inertia is near-instant: a generator trip propagates to loads within seconds.
The player sees immediate consequences and must maintain reserve capacity.

## 1.0 requirement

`SPEC-ELECTRICITY-001` — explicit wire topology. A road, intersection, or building road
link MUST NOT itself be an electrical connection.

`SPEC-ELECTRICITY-002` — `G + D = V + C + L`. Storage updates as
`B_next = B + C - D`, bounded by capacity. One `ElectricityAllocationID` applies once.

`SPEC-ELECTRICITY-003` — continuous, non-price priority load shedding. It MUST NOT replace
the per-building result with one binary network blackout.

`SPEC-ELECTRICITY-007` — generation `G` derives from a Production-owned plant result
accepted once by Electricity as an offer.

## Target design

Priority load shedding on a single island (PLAUSIBLE, D §3.4):
1. Sum generation across all producers in the network
2. Sort demands by declared priority (hospitals > factories > houses)
3. Serve demands in priority order until generation exhausted
4. Remaining demands get `curtailed` status with binding reason

Cost: one sort per network per tick. With roughly 100 buildings per network, this is
negligible.

Future: ramp rates, startup costs, reserve contribution (HYPOTHESIS). No AC physics
in the game — the model is an energy balance, not a power-flow solver.

## Current substrate

`ElectricityCache` (`simulation/src/map/electricity_cache.rs:52-62`) is a union-find over
`NetworkObjectID`, which includes `Building`, `Intersection`, and `Road`. Edges are derived
from building→road and road→intersection adjacency
(`electricity_cache.rs:244-279`). Every building connected to a road is automatically on
the electrical grid. There is no wire object.

`SPEC-ELECTRICITY-001` directly contradicts this: "A road, intersection, or building road
link MUST NOT itself be an electrical connection." Replacing the union-find is a **full
replacement** of the connectivity model, not an incremental improvement.

`electricity_flow_system` (`simulation/src/map_dynamic/electricity.rs:43-93`):
1. For each network, sum consumed/produced power across buildings
2. Houses consume a fixed 100 W
3. Companies consume/produce based on prototype fields and productivity
4. If consumed > produced, set `blackout = true`

This is:
- No explicit wire — connectivity follows road topology
- Binary blackout — no brownout, no priority shedding
- No storage — no battery/capacitor state
- No ramp rates — instant generation
- No load shedding priority — all-or-nothing

Company productivity under blackout is zero (`goods_company.rs:103-108`).

## Open questions

- Which endpoint classes and priority categories are 1.0?
- Is storage required in the first ratified implementation?

## Related

- [Water](water.md)
- [Heating](heating.md)
- [Production](../physical-economy/production.md)
- [Network architecture](network-architecture.md)
- [Electricity spec](../../reference/specifications/electricity.md)
- [Phase lag](../concepts/phase-lag.md)
