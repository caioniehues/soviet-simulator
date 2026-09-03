# Roads

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** transport
**Last verified:** 2026-09-03

| Scope | 1.0 — charter row Transport and border |

## What this is

Roads are the Planner-authored physical lane network. The Planner builds roads; simulation
uses them for movement, parking, and route requests. Roads are not a domestic-money purchase.
They are physical infrastructure that constrains everything downstream: vehicle routing,
freight delivery, and citizen commutes.

Parking is Roads-owned reservable capacity. Each parking lane creates exclusive spots that
vehicles can reserve and occupy. Parking scarcity creates visible pressure; the Planner can
respond by building more roads or lots.

## 1.0 requirement

`SPEC-ROADS-001` — a road is an authoritative, Planner-authored physical network object with
typed lanes and an explicit topology change history.

`SPEC-ROADS-004` — road topology declares static lane/corridor capacity inputs and exposes
the referenced durable traffic state.

`SPEC-ROADS-005` — automatic lot creation is not accepted as the target placement contract.

`SPEC-ROADS-006` — Roads alone owns physical parking-space reservations.

## Target design

The design proposes junction deadlock resolution beyond the current random-wait hack
(PLAUSIBLE, D §4.1). The current `Panicking` state waits a random duration
(`wait_time = fract(pos.x * 1000) * 0.5`) and retries. Two vehicles facing each other at a
junction with no room to pass can deadlock permanently.

Winter road state (HYPOTHESIS, future): snow and ice reduce safe braking distance and road
capacity. Snow clearing is real vehicle logistics — snow plows are vehicles that consume fuel
and need routes.

## Current substrate

`MAP-SUB-001`: typed driving, parking, walking, rail, and other lanes are provided.
Road objects store geometry, direction, speed limits, and controls
(`simulation/src/map/objects/lane.rs:11-104`, `simulation/src/map/objects/road.rs:70-226`).

`MAP-SUB-005`: parking lanes create exclusive reservable spots
(`simulation/src/map/objects/road.rs:197-212`, `simulation/src/map_dynamic/parking.rs:24-90`).

`MAP-SUB-002` (conflict): current non-arbitrary road construction removes intersecting lots
and generates new roadside lots automatically (`simulation/src/map/map.rs:682-720`). This
conflicts with `SPEC-ROADS-005` and `SPEC-ZONING-003`.

`MAP-SUB-004`: no durable capacity ledger or Planner traffic readout exists. Collision
avoidance is local and spatial-grid based; no aggregate congestion metric is exposed.

## Open questions

- Which minimum road classes and lane access rules are required for 1.0?
- What topology-change policy preserves valid in-progress trips while a road is altered?
- Is capacity pressure recorded per lane, per corridor, or both?

## Related

- [Pathfinding](pathfinding.md)
- [Traffic](traffic.md)
- [Vehicles](vehicles.md)
- [Construction](../physical-economy/construction.md)
- [Roads spec](../../reference/specifications/roads.md)
- [Zoning spec](../../reference/specifications/zoning.md)
