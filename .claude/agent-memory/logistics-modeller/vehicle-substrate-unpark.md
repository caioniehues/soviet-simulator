---
name: vehicle-substrate-unpark
description: Verified mechanics of unpark(), park(), the four VehicleState variants and the transport-grid collider lifecycle — the facts agents keep re-deriving
metadata:
  type: project
---

Verified at source 2026-08-28 in `/home/caio/sov-wave-souls-wt` at commit `5349f34`
(sov-6qx). These are expensive to re-derive; treat as current until the files change.

## unpark / park asymmetry

- `simulation/src/transportation/vehicle.rs:119` — `unpark(sim, vehicle) -> bool`, now
  `#[must_use]`. Returns `false` **without touching anything** when the entity is unknown
  or the state is not `Parked(_)`. Before `5349f34` it warned and proceeded, leaking a
  collider.
- There is still **no `park()` in `transportation/`**. The park path is
  `map_dynamic::router::park(map, vehicle, spot)`, which sets `VehicleState::RoadToPark`,
  and `transportation/road.rs:vehicle_state_update` finishes it: at spline `t >= 1.0` it
  **destroys the collider** and flips to `Parked(spot)`.
- Exactly three production `unpark` callers (grep-confirmed; the code-review-graph returned
  a false zero for `callers_of unpark` while grep found three — settle counts with grep):
  `map_dynamic/router.rs:218`, `economy/market.rs:805`, `world_command.rs:359`.

## Which states move

- `road.rs:vehicle_decision_system` returns early when `v.collider` is `None`.
- `road.rs:54-58` computes a desired speed **only** for `Driving | Panicking(_)`.
- `road.rs:physics` early-returns for `Parked` (snaps `trans` to the spot) and for
  `RoadToPark` (follows the spline).
- Consequence, load-bearing: **a `Parked` vehicle with a live `Itinerary` never moves and
  never will.** `it.has_ended(0.0)` stays false forever. Any state machine that waits on
  arrival while the vehicle is Parked wedges permanently.
- `RoadToPark` keeps its collider until the spline completes, so it is still visible to
  other vehicles while parking.

## The Unpark routing step never blocks

`router.rs:157` and `:171` — `RoutingStep::Unpark(_) => true` for both `cur_step_over` and
`next_step_ready`. The step is popped and considered done on the next evaluation whether or
not `unpark` succeeded. So a refused unpark cannot stall the router; the human proceeds to
`DriveTo`. Step order built at `router.rs:375` is
`WalkTo -> GetInVehicle -> Unpark -> DriveTo -> Park -> GetOutVehicle`, so within one
journey the car is always genuinely `Parked` at the Unpark step. The cross-owner case (a
dispatch truck made the router's vehicle by `WorkKind::Driver`'s `SetVehicle(Some(truck))`,
`souls/desire/work.rs:60`) is the only way to reach a non-Parked Unpark step there.

See [[phantom-collider-congestion]] for what the leaked collider did to routing.
