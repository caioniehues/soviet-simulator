---
name: truck-pool-reservation-lifetime
description: The truck pool is a conserved resource with 15 free sites and ONE acquire; free does not re-add to the position cache, so a truck freed during advance_dispatches is invisible until the next tick's dispatch_system
metadata:
  type: project
---

Audited 2026-08-27, HEAD `8531d3c`. The truck pool obeys the same conservation
rules as goods: a truck is acquired once and must be released exactly once, and
"released but invisible" is a *temporary* destruction of pool capacity.

## The asymmetry
`simulation/src/map_dynamic/dispatch.rs`:
- **acquire** — `Dispatcher::query` (`:130-140`) is the ONLY acquire. It calls
  `disp.query(...)` then `disp.reserve(best_ent)`.
- `DispatchOne::reserve` (`:203-210`) does THREE things:
  `reserved_by.insert(id)`, `positions.remove(&id)`, and
  `lanes[pos.lane].retain(|e| *e != id)`.
- **release** — `Dispatcher::free` (`:110-117`) does ONE thing:
  `disp.reserved_by.remove(&ent)`. It does **not** re-insert into `positions`
  or `lanes`. Its own doc comment says so: *"It should be re-added to the cache
  at the next update iteration"* (`:109`).
- Re-registration only happens in `Dispatcher::update` (`:85-105`), which walks
  `world.vehicles` and calls `register` per truck.

`DispatchOne::query` returns `None` when `self.positions.is_empty()` (`:233`)
and otherwise searches `positions`/`lanes` — so a freed-but-not-re-registered
truck is **unfindable**, even though it is no longer reserved.

## Why the same-tick miss is real
`simulation/src/init.rs`: `dispatch_system` is registered at `:59`,
`market_update` at `:98`. `Dispatcher::update` runs inside `dispatch_system`,
i.e. **before** `market_update` in the same tick. `advance_dispatches` contains
both `free` sites and the single `query` site, so any truck freed during that
pass cannot be re-acquired until the *next* tick's `dispatch_system`.

Free sites counted in `market.rs` (15, verified): 313, 351, 365, 384, 400
(all in `Market::remove`), then 787, 806, 821, 860, 908, 925, 946, 976, 999,
1036 in `advance_dispatches`.

**Consequence is latency, not loss** — one wasted tick of pool capacity. It is
self-healing because `update` re-registers unconditionally from `world.vehicles`
every tick. It only becomes a leak if a truck is freed and *also* removed from
`world.vehicles`, in which case `unregister` (`:119-125`, the full removal that
clears all three structures) is the correct call, not `free`.

## The contrast worth copying
`ParkingManagement` gets this right by type: `SpotReservation(ParkingSpotID)`
(`parking.rs:10`) is consumed **by value** in `free(&mut self, spot:
SpotReservation)` and ends with `std::mem::forget(spot)` (`:27-32`). You cannot
double-free a spot because you no longer own the token. `DispatchID` is `Copy`,
so `dispatcher.free(id)` can be called any number of times with no complaint —
15 call sites is exactly the situation where a consuming token pays for itself.

**Rule: a release that does not restore the resource to the acquirable pool is
only half a release. Check what `acquire` removed, and confirm `release`
restores every one of those structures — or that something else provably will,
and when.**

Related: [[market-balance-index]] (the seller-side truck free at
`market.rs:398-403` was added precisely because a missing `free` leaked trucks
permanently), [[break-families]] Family B.
