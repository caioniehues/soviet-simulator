---
name: refusal-signals-need-caller-rollback
description: A function that refuses and returns false is only safe where the caller can undo its own bookkeeping — a caller that took a reservation must roll it back, never just log
metadata:
  type: feedback
---

When hardening a function to refuse work and signal it (`-> bool`, `Option`, `Result`),
auditing the callers is not "does each caller handle the value". It is **"did this caller
already record something that the refusal invalidates?"** A caller that reserved a resource
or recorded ownership before the call must roll that back on refusal. Logging is not
handling.

**Why:** ruled by logistics-modeller on sov-6qx, 2026-08-28. `unpark` was hardened to refuse
non-`Parked` vehicles. `economy/market.rs` `DispatchState::ToSource` reserves a truck from
the `Dispatcher` and records `truck = Some(v)` **before** the deferred `unpark` runs
(`cbuf_vehicle.exec_ent`). My first version logged the refusal and kept `truck = Some(v)`.
A `Parked` vehicle has no collider, `vehicle_decision_system` skips colliderless vehicles,
so its itinerary never ends — and `ToSource` has **no tick countdown**, unlike `Loading`,
the return path and `retail_claims`. The dispatch, the truck and the seller's reserved
quantity would have been held permanently. The fix is `Market::release_tosource_truck(v)`
plus `Dispatcher::free`.

**How to apply:** for every call site of a newly-refusable function, ask what state the
caller wrote between "decided to call" and "the call actually ran" — especially where the
call is deferred through a `ParCommandBuffer`, because the check and the effect then observe
different worlds. Callers that only *read* before calling (`router.rs` `RoutingStep::Unpark`
pops its step regardless; `world_command.rs` `SpawnRandomCars` just spawned the car) need
nothing. `#[must_use]` makes "did I visit every caller" a build error rather than a search
problem — prefer it to enumerating by grep.

Related: [[dispatcher-truck-pool]], [[graph-zero-callers-is-a-lie]].
