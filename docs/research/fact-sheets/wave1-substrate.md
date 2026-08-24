# Wave 1 foundation and movement substrate fact-sheet

**Kind:** reference
**Authority:** reference
**Status:** active
**Owner:** architecture
**Last verified:** 2026-08-24
**Commit:** `186e08179b5ad9415dc4cd2d42d77a49303e35d6`

This fact-sheet constrains the rewrite of foundations, architecture, roads, pathfinding, traffic,
and prototype authority.

## Foundation contract

| Surface | Ruling | Current production reality |
|---|---|---|
| Tick | **PARTIAL** | Nominal time is 50 ticks/second. Commands apply first, time increments, then a serial schedule executes (`simulation/src/lib.rs:244-270`, `prototypes/src/types/time.rs:10`). Four instant commands bypass ticking; other commands force a tick even while paused (`native_app/src/network.rs:31-78`, `simulation/src/world_command.rs:211-235`). |
| Schedule | **PROVIDED** | Registration order becomes a sequential system vector; command buffers commit after every system, so later systems see earlier structural changes (`simulation/src/init.rs:52-109`, `utils/scheduler.rs:27-56`). |
| Initialization | **CONFLICTING** | Registries are unsynchronized `static mut` vectors, preventing safe parallel test initialization (`simulation/src/init.rs:111-130`). |
| Save/load | **PARTIAL/CONFLICTING** | Snapshots store World and separately encoded resources. Version mismatch only warns; resource decode failure logs and leaves fresh defaults, permitting a loaded-world/default-resource hybrid (`simulation/src/lib.rs:359-448`, `init.rs:173-191`). VERSION remains `0.6.1` despite serialized Market changes. |
| Determinism | **PARTIAL** | The test helper serializes and reloads one current state and compares hashes. This proves current-schema round-trip stability, not repeat-run determinism (`simulation/src/tests/mod.rs:107-121`). |
| Presentation | **PROVIDED** | Tools read immutable Simulation, mutate UI state, and queue WorldCommands; renderers and inspectors consume simulation state directly (`native_app/src/gui/mod.rs:40-88`, `game_loop.rs:122-180`). |

The strongest inherited seam is therefore:

```text
WorldCommand → command-first serial schedule → authoritative Simulation → presentation consumers
```

The rewrite must explicitly decide whether instant mutations remain legal and must not claim all
commands enter through a fixed tick.

## Roads, routing, and traffic

### MAP-SUB-001 — Typed lane graph and physical movement are live

Roads create typed driving, biking, bus, parking, walking, and rail lanes with geometry, direction,
controls, and speed limits (`simulation/src/map/objects/lane.rs:11-104`,
`map/objects/road.rs:70-226`). Player road tools queue authoritative connection commands
(`native_app/src/gui/tools/roadbuild.rs:267-311`, `simulation/src/world_command.rs:244-253`).

Classification: **PROVIDED**.

### MAP-SUB-002 — Road construction still creates lots automatically

Every non-arbitrary road removes intersecting lots and generates new roadside lots
(`simulation/src/map/map.rs:682-720`).

Classification: **CONFLICTING** with any brief that assumes player-planned placement only.

### MAP-SUB-003 — Pathfinding is static, not congestion-aware

Vehicle A* uses lane length/speed plus deterministic noise. It has no live congestion, capacity,
queue, closure, freight restriction, or vehicle-class cost (`simulation/src/map/pathfinding.rs:189-268`).
Missing topology retries periodically without terminal failure or a player-visible stalled-route
queue (`map_dynamic/itinerary.rs:171-198`).

Classification: **PARTIAL**.

### MAP-SUB-004 — Traffic is microscopic movement only

Vehicles follow, collide, stop at signals, and may enter `Panicking` during gridlock
(`simulation/src/transportation/road.rs:15-78,185-250`, `map/traffic_control.rs:38-92`). There is no
durable congestion ledger, queue age, road load, capacity state, or Planner-facing traffic signal.

Classification: **PARTIAL**. Collision avoidance must not be described as the planned congestion
model.

### MAP-SUB-005 — Finite parking is provided

Parking lanes create exclusive reservable spots; human routing can walk, unpark, drive, park, and
walk across graph levels (`simulation/src/map/objects/road.rs:197-226`,
`map_dynamic/parking.rs:24-90`, `map_dynamic/router.rs:348-438`).

Classification: **PROVIDED**.

## Prototype authority

| Declaration | Ruling | Reachable consumer |
|---|---|---|
| Items | **PROVIDED** | All item prototypes initialize Market (`base_mod/items.lua`, `simulation/src/economy/market.rs:150-161`). |
| Goods companies | **PROVIDED** | Toolbox placement, company spawning, truck counts, workers, and recipes consume the declarations (`native_app/src/gui/hud/toolbox/building.rs:41-72`, `simulation/src/souls/goods_company.rs:113-177`). |
| Solar subtype | **PARTIAL** | Parent company fields work, but subtype `max_power` is parsed by no production field (`base_mod/companies.lua:74-88`, `prototypes/src/prototypes/solar.rs:10-22`). |
| Road vehicles | **UNREACHABLE** | Lua fields parse, but physics and rendering use hard-coded `VehicleKind` values and assets (`prototypes/src/prototypes/road_vehicle.rs:20-42`, `simulation/src/transportation/vehicle.rs:60-105`). |
| Leisure | **UNREACHABLE/PARTIAL** | Parent building/icon data can exist; no production system consumes capacity, hours, or fee (`prototypes/src/prototypes/leisure.rs:7-25`, `simulation/src/map/objects/building.rs:17-38`). |
| Rolling stock | **PROVIDED** | Mass, force, speed, length, and assets drive train physics/rendering (`simulation/src/transportation/train.rs:58-102`, `native_app/src/rendering/entity_render.rs:24-42`). |
| Freight station | **PARTIAL** | Placeable live soul, but cargo is only unitless waiting/wanted counters (`prototypes/src/prototypes/freightstation.rs:7-24`, `simulation/src/souls/freight_station.rs:30-48`). |

Rule for every rewrite brief: a Lua declaration is authoritative only after proving the exact
parsed field has a reachable production consumer.

## Verification boundary

Every cited location and negative reachability search was independently reopened. No build, test,
gameplay, performance, save-migration, corrupted-save, or mutation run was performed because this
thread cached the old command policy. Reference-game observations establish data vocabulary only,
not proprietary scheduling or routing algorithms.
