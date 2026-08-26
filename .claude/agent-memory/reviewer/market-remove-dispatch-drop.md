---
name: market-remove-dispatch-drop
description: Market::remove both halves now fixed (sov-dispatch-wedge-ab4 r3 buyer, r4 seller truck-free); SimDrop::sim_drop takes &mut World; dead-seller drop is truck-only, never a ledger event
metadata:
  type: project
---

`simulation/src/economy/market.rs::Market::remove` used to end with a single blind
`self.dispatches.retain(|d| d.buyer != soul && d.seller != soul);`.

**Fixed 2026-08-26 (sov-dispatch-wedge-ab4 round 3) for the BUYER half only.**
`remove` now takes `(soul, map, binfos, world, dispatcher, tick)` and routes a dead
buyer's dispatches through the real fates: `ToSource` → release `reserved[seller]` +
free truck; `Loading`/`ToDestination`/`Returning` → re-route the truck to the seller
and set `Returning` (or honest logged loss if no route/no seller); `Unloading` →
honest logged loss.

**FIXED 2026-08-26 (round 4).** A 5-line loop immediately before the final
`retain(|d| d.seller != soul)` frees any `d.truck` for seller-matched dispatches.
Verified by review: `Dispatcher::free` is `reserved_by.remove` (a BTreeSet remove,
dispatch.rs:116) so it is idempotent and a no-op on an already-freed truck. No
double-free with the buyer half: the buyer loop's guard is
`d.buyer != soul || d.seller == soul`, and every buyer arm that calls `free`
also `swap_remove`s the dispatch, so no dispatch is reachable by both loops.
A self-dealing dispatch (buyer == seller == soul) skips the buyer loop entirely
and is freed exactly once by the seller loop.

**The dead-seller drop is NOT a conservation surface.** The round-4 block mutates
only `Dispatcher::reserved_by` — no `capital`, no `reserved`, no `dispatches`
membership beyond the pre-existing retain. Quantity behaviour on that path is
byte-identical to round 3 (whose ledger signoff still covers it).

**Historical (round 3 state):** The function ends with
`self.dispatches.retain(|d| d.seller != soul);` (market.rs:390): unconditional, and it
never calls `dispatcher.free(DispatchID::SmallTruck(v))` for an assigned truck. Trucks
are assigned in `ToSource` (`self.dispatches[i].truck = Some(v)`, market.rs:772), so
demolishing a *seller* mid-dispatch permanently removes that truck from the Dispatcher
pool. The old `ponytail:` comment that admitted this leak was deleted in the same diff,
so nothing in the file flags it any more. Ledger is fine (the seller's capital/reserved
rows are wiped wholesale); it's a resource leak, not a conservation break.

**`SimDrop::sim_drop` widened to `(self, id, world: &mut World, res: &mut Resources)`**
(`utils/par_command_buffer.rs:7`) so `Market::remove` can reach `world.vehicles`. Safe
by construction: `apply` does `E::storage_mut(&mut sim.world).remove(entity)` BEFORE
calling `sim_drop` (par_command_buffer.rs:63-66), so the dying entity is already out of
its storage — no aliasing, no re-entrancy. `world` and `resources` are separate
`Simulation` fields, so the split borrow compiles. The 3 no-op impls
(Vehicle/Train/Wagon) take `_world`.

**Reachability is now genuinely fixed.** `company_system` (goods_company.rs:197-198)
queues `cbuf.kill(me)` when the building is gone; `scheduler.rs:46-51` flushes after
every system → `CompanyEnt::sim_drop` → the new `Market::remove`. So `Returning` fires
on the ordinary demolish path. NOTE the tests do NOT prove this chain: `remove_soul`
(hoarding.rs:77) calls `Market::remove` directly, and
`scenario_demolished_buyer_returns_goods_physically` still uses the synthetic
`mk_soul((1<<32)|2)` buyer from `setup_seller_buyer`. The real-entity helper is
`setup_real_seller_buyer` (ledger.rs:251, uses `build_company_at`) — prefer it.

`tests/scenarios/ledger.rs::total_qty` in-flight filter now includes `Returning`
(ledger.rs:45-53). It sums `capital_map()` only, NOT `reserved` — correct, because a
retail claim leaves the good in the seller's capital until `settle_retail` debits it.

**How to apply:** any diff adding a `DispatchState`, a reservation, or a cancellation
branch must be read against BOTH halves of `Market::remove` and against `total_qty`.
The seller half is the one still cutting corners.

**Regression guards (both mutation-proven 2026-08-26 by re-running the mutation):**
`ledger.rs:331 scenario_dead_seller_frees_its_truck` — deleting the 5-line free loop
gives `left: 1 right: 2` at ledger.rs:410. It queries the Dispatcher to exhaustion and
compares against the true truck count, because a leaked reservation is invisible to a
query for a *different* truck (new candidate = new id) and every `kind="factory"`
company spawns its own `n_trucks`, so "one truck in the city" is not constructible.
`ledger.rs:430 scenario_demolish_buyer_building_end_to_end_conserves` — the only test
driving the REAL chain (`map_mut().remove_building` -> `company_system` cbuf.kill ->
`CompanyEnt::sim_drop` -> `Market::remove`); re-inserting the round-2 blind
`retain(|d| d.buyer != soul)` gives `left: 0 right: 10` at ledger.rs:499. Every other
dead-buyer test calls `Market::remove` directly via `remove_soul` (hoarding.rs:77).

Related: [[market-exttrade-seam]], [[sim-test-harness-quirks]], [[retail-claim-ttl-seam]].
