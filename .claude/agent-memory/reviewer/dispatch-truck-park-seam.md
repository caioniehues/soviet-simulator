---
name: dispatch-truck-park-seam
description: unpark() clobbers the collider of a non-Parked vehicle without freeing the stale transport-grid entry; market.rs's grab site is guarded but router.rs:217 is not
metadata:
  type: project
---

`unpark` (`simulation/src/transportation/vehicle.rs:107-125`) warns
"Trying to unpark X that wasn't parked" on a non-`Parked` vehicle and then
**overwrites `v.collider` anyway** without removing the stale
`TransportGrid` entry — leaving a phantom blocker (`speed=0`,
`is_vehicle=true`) that silently queues every later vehicle behind it. Filed as
**sov-6qx**, deliberately unfixed as of 2026-08-27.

Two production call sites can hit it with a dispatcher-pool truck:

- `simulation/src/economy/market.rs:783-786` (`DispatchState::ToSource` grab) —
  **guarded** since sov-7pg: it checks `matches!(state, VehicleState::Parked(_))`
  and calls `dispatcher.free` instead of grabbing a truck still mid-`RoadToPark`.
- `simulation/src/map_dynamic/router.rs:217` (`RoutingStep::Unpark`) —
  **unguarded**, and reachable for a company truck because `WorkKind::Driver`
  pushes `SetVehicle(Some(truck))` (`simulation/src/souls/desire/work.rs:60`),
  making a dispatcher-managed truck the router's vehicle.

Freeing a truck back to the dispatcher is safe and does not livelock:
`Dispatcher::free` only clears `reserved_by` (`map_dynamic/dispatch.rs:110-117`),
while `Dispatcher::update` re-registers every truck each tick
(`dispatch.rs:99-104`), and `dispatch_system` (`init.rs:59`) runs before
`market_update` (`init.rs:98`). `RoadToPark` is bounded by
`TIME_TO_PARK = 4.0` (`transportation/vehicle.rs:13`).

`SpotReservation` has **no `Drop` impl** and `ParkingManagement::free` uses
`mem::forget`, so a reservation dropped on a path that never reaches `park()`
leaks the spot permanently. `VehicleEnt::sim_drop` (`world.rs:74-78`) correctly
frees both `Parked` and `RoadToPark`, so a killed truck is fine.

**Why:** the phantom collider produces no error and no "gridlock!" line — the
symptom is dispatches that stall forever in `ToDestination`, which reads as a
market/recipe bug rather than a traffic one. It cost two tickets to find.

**How to apply:** when reviewing anything that grabs, parks, or unparks a
dispatcher truck, check the `Parked` guard at the call site, and remember the
root cause is still live in `unpark` itself — a call-site guard fixes one path
only. Verify parking claims by probing for trucks that are `Driving` with an
ended itinerary; the healthy count is 0. Related: [[market-remove-dispatch-drop]],
[[zero-workers-zero-production]].
