# Freight rail

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** transport
**Last verified:** 2026-09-03

| Scope | 1.0 — charter row Transport and border |

## What this is

The charter commits to minimal freight rail: three buildings (freight station, train station,
and external train station), one locomotive type, and one wagon type. Rail is the most
developed transport subsystem in the codebase: it has consist mass, length, locomotive power,
intersection reservation, and look-ahead braking. What it lacks is cargo capacity, wagon
compatibility, signalling, yards, and empty repositioning.

The default city has a "border closed until road reaches the station" ruling
(see `docs/reference/architecture/substrate.md`).

## 1.0 requirement

`SPEC-VEHICLES-004` — the 1.0 freight-rail consist uses the same no-teleport custody rules
as road freight.

`SPEC-VEHICLES-005` — each wagon SHALL have finite compatible cargo capacity and observable
load.

## Target design

Missing wagon capacity and cargo (PLAUSIBLE, D §4.4): `RailWagon` has `kind` and
`rolling_stock` but no cargo type, capacity, or compatibility field. The hook the bible
asks for (§9.11: "preserve fields for wagon capacity, cargo custody") is absent.

Signalling as capacity (PLAUSIBLE, D §4.2): the current model uses intersection
reservation (`TrainReservations`), not signal blocks. Two trains can occupy the same lane
segment simultaneously. Real rail capacity is constrained by signal block length + braking
distance. Signals are a charter Post-1.0 cut.

Empty repositioning (PLAUSIBLE, D-13): trains return empty to the external station after
unloading. The return trip is real traffic, but there is no explicit empty-wagon logistics
model.

Yards (PLAUSIBLE, D §4.4): no yard model exists for consist assembly, wagon sorting, or
loading queue management.

## Current substrate

`Locomotive` struct (`simulation/src/transportation/train.rs:25-34`):
- `max_speed: f32` (m/s)
- `acc_force: f32` (m/s², force/mass)
- `dec_force: f32` (m/s², force/mass)
- `length: f32` (m)

`calculate_locomotive` (`train.rs:57-77`) computes consist properties from wagon prototypes:
- `max_speed`: minimum across all rolling stock
- `acc_force`: sum of forces / total mass
- `dec_force`: sum of forces / total mass
- `length`: sum of lengths + 10.0 m buffer

`TrainReservations` (`train.rs:19-22`): tracks intersection occupancy per train and lane
localisations. Intersection reservation prevents junction conflicts.

Look-ahead braking (`train.rs:388-475`): the train scans upcoming track for occupied
intersections and computes braking distance to stop before them.

60-second stuck-train creep (`train.rs:379-383`): when a train has waited > 60 seconds,
it creeps forward at `0.1 * DELTA` per tick. This is a deadlock-breaker hack that can
phase a train through a blocked junction.

Prototype data (`base_mod/rollingstock.lua`):

| Rolling stock | Length | Mass | Max speed | Acc force | Dec force |
|---|---|---|---|---|---|
| Locomotive | 16.75 m | 60 t | 200.0 m/s | 2000.0 kN | 360.0 kN |
| Passenger wagon | 16.75 m | 40 t | 200.0 m/s | 0 | 240.0 kN |
| Freight wagon | 16.75 m | 80 t | 160.0 m/s | 0 | 480.0 kN |
| Passenger EMU front | 28.0 m | 60 t | 360.0 m/s | 240.0 kN | 360.0 kN |

**Placeholder speeds:** Locomotive max_speed of 200.0 m/s = 720 km/h is unrealistic. A
Soviet freight locomotive would be roughly 30 m/s (108 km/h). The EMU at 360.0 m/s
(1296 km/h) is even more unrealistic. These values need correction.

Freight station cargo is a counter (`simulation/src/souls/freight_station.rs:34`,
`FreightStation.waiting_cargo: u32`, `wanted_cargo: u32`). No embodied cargo per wagon.

Known gap: when a freight train finishes loading, `freight_station_system` unwraps the
first external station (`freight_station.rs:109`, `.first().unwrap()`), but
`Map::remove_building` allows removing an `ExternalTrading` building
(`simulation/src/map/map.rs:128-135`). If the last external station is removed while a
train is loading, the unwrap panics instead of leaving an observable waiting state.

## Open questions

- What wagon capacity and compatibility data is required by the 1.0 freight model?
- Should the placeholder rolling-stock speeds be corrected now?
- What is the 1.0 scope for consist assembly?

## Related

- [Vehicles](vehicles.md)
- [Logistics](../physical-economy/logistics.md)
- [Custody](../physical-economy/custody.md)
- [Roads](roads.md)
- [Public transport (future)](public-transport-future.md)
- [Vehicles spec](../../reference/specifications/vehicles.md)
