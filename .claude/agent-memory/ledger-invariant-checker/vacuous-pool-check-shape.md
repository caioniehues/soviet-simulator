---
name: vacuous-pool-check-shape
description: The dispatcher-pool "no leaked truck" assertion is vacuous whenever the scenario destroys the vehicle — mutation-proven 2026-08-28; the correct form asserts handed_out == reachable_trucks with two live trucks
metadata:
  type: project
---

A recurring bad shape in this repo's dispatcher tests. Record it because it
reads exactly like a real guard.

## The vacuous form
`retail::scenario_dead_truck_tosource_cancels_without_leak` (commit `7721cdd`)
does `world.vehicles.remove(dead_truck)`, then collects `all_trucks` from
`world.vehicles` and asserts every one of them comes back from
`dispatcher.query`.

That cannot fail. The dead truck is not in `all_trucks`, and its leaked
`reserved_by` entry names a non-existent entity, so it blocks no live truck.

**Mutation proof (2026-08-28, worktree `/home/caio/sov-wave-market-wt`,
reverted afterwards):** deleting
`dispatcher.free(DispatchID::SmallTruck(v));` from the `ToSource` wedge-(b)
arm of `Market::advance_dispatches` leaves the test green —
`test result: ok. 1 passed; 0 failed`.

The rest of that test (dispatch cancelled, `reserved == 0`, `capital == 10`)
is real. Only the appended pool check is not.

## The correct form
`ledger::sov_jcl_outbound_loading_route_failure_is_bounded` gets it right:

- assert **no vehicle was destroyed** in the scenario, so the check cannot be
  satisfied by the sibling truck-vanished branch;
- count `reachable_trucks` (`Dispatcher::query` walks lanes outward from the
  target, so it only ever reaches trucks on the target's road network) and
  assert `reachable_trucks >= 2`, "or a leak of exactly one is
  indistinguishable from an empty pool";
- drain the pool and assert `handed_out == reachable_trucks`.

## The stale-id caveat both tests must handle
`Dispatcher::update` never purges an entity removed from `world.vehicles`, so
a drain-until-`None` loop can be handed a **stale dead id**. Assert *which*
live trucks came back, or bound the loop — never assert on the raw count
alone in a scenario that destroyed a vehicle.
