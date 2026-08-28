---
name: dispatch-tosource-wedge-surface
description: DispatchState::ToSource with truck=Some(v) has no timeout — the only exit is the vehicle entity vanishing; and why the market.rs Parked guard must never be removed
metadata:
  type: project
---

Ruling made 2026-08-28 during the sov-6qx Phase-4 movement sign-off. Read at source in
`simulation/src/economy/market.rs` (`Market::advance_dispatches`).

## The market.rs Parked guard is NOT dead code

`market.rs:783-786` checks `matches!(ve.vehicle.state, VehicleState::Parked(_))` before
grabbing a dispatcher truck, and calls `dispatcher.free(...)` when it is not parked. The
sov-6qx ticket suggested this guard might become dead once `unpark` itself refuses
non-Parked vehicles. **It does not, and it must stay.** `unpark` returning `false` cannot
free the dispatcher reservation — it has no access to the `Dispatcher`. Without the guard:

1. `dispatcher.query` reserves a truck that is mid-`RoadToPark`.
2. `ve.it` is set to the route; the deferred `unpark` refuses.
3. `self.dispatches[i].truck = Some(v)` is set anyway.
4. The truck finishes parking, its collider is destroyed, state becomes `Parked` — and a
   `Parked` vehicle never moves, so `it.has_ended(0.0)` is false forever.
5. `ToSource` with `truck = Some(v)` **has no timeout**. The only exit is
   `world.vehicles.get(v).is_none()`. So the dispatch, the truck reservation and the
   seller's `reserved` quantity are held permanently.

That is the sov-jcl / sov-dispatch-wedge-ab4 failure shape, one state earlier.

## Standing hazard, independent of sov-6qx

`ToSource`/`Some(v)` having no tick countdown is a wedge surface in its own right —
`Loading` and the return path have retry counters, `ToSource` does not. Any future change
that can leave a reserved truck immobile re-opens it. Prefer a bounded retry that frees the
reservation and drops back to `ToSource`/`None`, matching how `Loading` and `retail_claims`
already bound their waits.

## The rule this generalises to

Refusal is the right semantic for `unpark`, but **a refusal signal is only safe where the
caller can undo its own bookkeeping.** Any caller that has already taken a reservation, set
an itinerary, or recorded ownership must roll that back on `false`, not merely log it.

See [[vehicle-substrate-unpark]] and [[phantom-collider-congestion]].
