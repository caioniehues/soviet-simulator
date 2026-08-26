---
name: dispatcher-truck-pool
description: How trucks leak out of the Dispatcher pool, why the leak is invisible to most tests, and the only reliable way to assert on it
metadata:
  type: project
---

`Dispatcher::free` is the ONLY thing that clears `DispatchOne::reserved_by`
(`dispatch.rs:116`). `DispatchOne::query` skips any id present in `reserved_by`
(`dispatch.rs:271`). Therefore any code path that drops a `Dispatch` holding
`truck: Some(v)` without calling `dispatcher.free(DispatchID::SmallTruck(v))`
removes that truck from the city permanently.

**Why it does not self-heal:** `reserve()` does `positions.remove(&id)`, and
`register()` only re-inserts on `Entry::Vacant`. So `Dispatcher::update` DOES
re-register a leaked truck's position each tick — it looks alive — but `query`
still skips it forever because `reserved_by` was never cleared.

**How to observe it in a test (verified 2026-08-26).** Two approaches that do
NOT work:
- Driving a second delivery and expecting failure: every `GoodsCompany` with
  `kind = "factory"` spawns its own `n_trucks` (`goods_company.rs:129-137`), so
  you cannot construct a one-truck city out of real companies.
- Querying for a *different* truck: a leaked reservation is invisible to that,
  because a new candidate is a new id. Already documented at `retail.rs:353-362`.

What works: call `Dispatcher::query` in a loop until it returns `None`, count
how many it handed out, and compare against the real truck count from
`world.vehicles`. A leak shows up as exactly one fewer. Requires
`dispatcher.update(&map, world)` first, and a `ctx.tick()` before that so
positions are current.

`Dispatch::truck` is PRIVATE and the `truck()` accessor was deliberately
dropped in round 3 of sov-dispatch-wedge-ab4 — do not re-add it. To prove a
truck was assigned, wait for `DispatchState::Loading`, which is only reachable
after a truck was reserved and physically drove to the seller.

Related: [[sim-test-setup-traps]].
