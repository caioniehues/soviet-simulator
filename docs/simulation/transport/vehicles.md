# Vehicles

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** logistics
**Last verified:** 2026-08-28

| Scope | 1.0 binding |

## What this is

A vehicle is a finite persistent identity used by Logistics. Its availability constrains
service. When no truck is available, a haul waits in queue — that queue is a visible
scarcity signal for the Planner.

The design proposes lane-constrained longitudinal physics with mass, power, traction, grade,
and jerk (PLAUSIBLE, D §3.1). A loaded truck behaves differently from an empty truck,
particularly on grades. This gives the Planner a physical reason to care about terrain and
road placement.

## 1.0 requirement

`SPEC-VEHICLES-001` — each operational vehicle has a stable identity and observable state:
available, reserved, travelling, loading, unloading, recovering, or unavailable.

`SPEC-VEHICLES-002` — vehicle reservation is separate from stock allocation and custody.

`SPEC-VEHICLES-005` — each freight vehicle and wagon SHALL have finite compatible cargo
capacity, an accountable owner/depot, physical parking/recovery location, and observable load.

## Target design

The target physics model (PLAUSIBLE, D §3.1):

```text
grade       = terrain_slope_at(lane, s)
F_gravity   = mass * g * sin(grade)
F_traction  = min(power_max / max(v, 0.1), traction_limit)
F_drag      = drag_coeff * v²
F_net       = F_traction - F_drag - F_gravity - F_brake
a           = F_net / mass
v          += clamp(a * dt, -jerk_limit * dt, jerk_limit * dt)
```

Grade data is free: lane points already have Z coordinates (`PolyLine3`). The grade is
`dz/ds` along the polyline. Cost: one `sin()` + one division per vehicle per tick.

IDM parameters (PLAUSIBLE, D §3.2):

| Parameter | Car | Truck | Bus |
|---|---|---|---|
| v0 (desired speed) | speed_limit | speed_limit * 0.8 | speed_limit * 0.8 |
| a_max (max accel) | 3.0 | 2.5 | 2.0 |
| b (comfortable decel) | 3.0 | 2.0 | 2.0 |
| T (safe headway) | 1.0 s | 1.5 s | 1.5 s |
| s0 (jam distance) | 2.0 m | 3.0 m | 3.0 m |

MOBIL lane-changing is Post-1.0 — there is no multi-lane road model in the current
substrate. The charter cuts vehicle manufacture and vehicle fuel lifecycle.

## Current substrate

`Vehicle` struct (`simulation/src/transportation/vehicle.rs:34-45`):
- `ang_velocity: f32`
- `wait_time: f32`
- `max_speed_multiplier: f32` (random 0.95–1.05)
- `state: VehicleState` — Parked / Driving / Panicking / RoadToPark
- `kind: VehicleKind` — Car / Truck / Bus
- `tint: Color`
- `flag: u64` (gridlock detection)

`VehicleKind` provides per-kind constants (`vehicle.rs:60-105`):
- `width()`: Car 4.5, Truck 6.0, Bus 9.0
- `acceleration()`: Car 3.0, Truck 2.5, Bus 2.0 m/s²
- `deceleration()`: all 6.0 m/s²
- `min_turning_radius()`: Car 0.5, Truck 3.0, Bus 4.0
- `speed_factor()`: Car 1.0, Truck/Bus 0.8

Missing from the target model: mass, power, traction, braking (as mass-dependent), cargo,
capacity, owner, depot, fuel, driver, grade response. The physics is purely kinematic:

```rust
speed += clamp(desired_speed - speed, -DELTA * decel, DELTA * accel)
```

No F=ma, no grade, no mass-dependent braking.

## Open questions

- Is loaded-vs-empty-on-grades a 1.0 feature or Post-1.0?
- Is replacing the cone-check with IDM worth the behavioural change risk?
- Which owner/depot sharing policy permits a freight vehicle to serve external hauls?

## Related

- [Roads](roads.md)
- [Pathfinding](pathfinding.md)
- [Traffic](traffic.md)
- [Logistics](../physical-economy/logistics.md)
- [Freight rail](freight-rail.md)
- [Vehicles spec](../../reference/specifications/vehicles.md)
