---
name: dead-entity-leaks-need-reserved-by
description: A dispatcher reservation leak on a DESPAWNED entity is invisible to Dispatcher::query - assert on reserved_by, never on which live trucks come back
metadata:
  type: project
---

Any test phrased as "every live truck must still be reservable" **cannot detect a leaked
reservation held by a dead entity**. Verified by mutation 2026-08-28.

**Why:** `DispatchOne::reserve` (`simulation/src/map_dynamic/dispatch.rs:209-216`) REMOVES
the entity from `positions`. `Dispatcher::free` only clears `reserved_by` — it never
re-inserts the position. Re-insertion happens solely in `register`, driven by
`Dispatcher::update`, which iterates **live** `world.vehicles`. So an entity removed from
`world.vehicles` is never handed back by `query` whether or not it was freed, and the
assertion is about a different truck either way.

**How to apply:** to observe a leak for a dead entity, assert on its own `reserved_by`
entry. `Dispatcher::is_reserved(impl Into<DispatchID>) -> bool` exists for this
(`#[cfg(test)] pub(crate)`, added in sov-ie6, sits next to `free`). Also assert the
reservation was TRUE before the kill, or the guard passes for the wrong reason.

Do NOT copy `ledger::sov_jcl_outbound_loading_route_failure_is_bounded`'s
`handed_out == reachable_trucks` form for a dead-entity case — it works there only because
its truck stays alive.

Related: [[dispatcher-free-does-not-requeue]], [[graph-zero-callers-is-a-lie]].
