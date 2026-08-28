---
name: leak-blind-dead-truck-test
description: retail::scenario_dead_truck_tosource_cancels_without_leak cannot detect the dispatcher leak it is named for — proven by mutation 2026-08-28
metadata:
  type: project
---

`simulation/src/tests/scenarios/retail.rs::scenario_dead_truck_tosource_cancels_without_leak`
is **blind to the leak in its own name**.

**Proof (mutation, 2026-08-28, branch `fix/sov-wave-market`):** delete
`dispatcher.free(DispatchID::SmallTruck(v))` from the `ToSource` /
"vehicle entity is gone" branch of `Market::advance_dispatches`
(`economy/market.rs:~837`) → the test still passes (`1 passed`).

**Why:** the leaked `reserved_by` entry belongs to the **dead** truck, which was
removed from `world.vehicles` and therefore is not in the test's `all_trucks`
list. The assertion only requires every **live** truck to still be reservable.
A stale reservation on a dead entity is simply never handed out and never
observed. `Dispatcher::update` does not purge removed entities.

Its pre-rewrite form was blind too, for a different reason: its final assertion
(`capital(buyer) == 5`) was satisfied by the ext-trade teleport, not by a
delivery — the seller's sell order had already been consumed by the first match.

**How to apply:** do not cite this test as coverage for `dispatcher.free` on the
ToSource cancellation path. A real guard must assert on the dead truck's own
`reserved_by` entry (or on `Dispatcher` internals), not on live-truck
queryability. Related: [[dispatch-truck-park-seam]], [[market-exttrade-seam]].
