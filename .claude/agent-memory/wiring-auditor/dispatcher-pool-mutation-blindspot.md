---
name: dispatcher-pool-mutation-blindspot
description: Why a "every live truck must still be reservable" assertion cannot detect a missing dispatcher.free() when the truck is DEAD — the reserve/free asymmetry in map_dynamic/dispatch.rs
metadata:
  type: project
---

A whole family of scenario tests in `simulation/src/tests/scenarios/` proves "the truck
went back in the pool" by calling `Dispatcher::update` then draining `Dispatcher::query`
and asserting on which trucks came back. **That assertion is only meaningful when the
truck in question is still alive in `world.vehicles`.**

The mechanism, in `simulation/src/map_dynamic/dispatch.rs`:

- `DispatchOne::reserve` (line ~209) **removes** the entity from `positions` *and* from
  `lanes`, and inserts it into `reserved_by`.
- `Dispatcher::free` (line ~116) removes it from `reserved_by` and **does nothing else** —
  it never re-inserts into `positions`.
- Re-insertion happens only in `DispatchOne::register`, driven by `Dispatcher::update`,
  which iterates **live** entities.

So for a truck removed from `world.vehicles`, `free()` has **no observable effect on the
pool at all**: the truck is absent from `positions` either way and `update` can never put
it back. Calling `free()` or not calling it are indistinguishable through `query`.

**Verified by mutation, 2026-08-28** (isolated worktree, branch `fix/sov-wave-market`):
deleting `dispatcher.free(DispatchID::SmallTruck(v));` from the dead-vehicle arm of
`Market::advance_dispatches` (`economy/market.rs:838`) left
`tests::scenarios::retail::scenario_dead_truck_tosource_cancels_without_leak` **green**
— `1 passed; 0 failed` — even though that test's new block exists specifically to guard
that line.

**How to tell a sound pool assertion from a vacuous one:** ask whether the scenario
destroys the vehicle.

- Truck ALIVE → sound. `sov_jcl_outbound_loading_route_failure_is_bounded`
  (`tests/scenarios/ledger.rs`) is the model: it asserts `all_trucks.len() ==
  trucks_before` first ("no truck may be destroyed here, or the free() check below is
  vacuous"), and asserts `handed_out == reachable_trucks`. A missing `free()` drops
  `handed_out` by one and it goes red.
- Truck DEAD → vacuous, no matter how the assertion is phrased. The real claim on that
  path (dispatch removed, `reserved` released, seller capital untouched) has to be carried
  by the market-side assertions instead, which it is.

Related: [[MEMORY]] recurring failure shape 3 — a green line whose subject is not the
feature.
