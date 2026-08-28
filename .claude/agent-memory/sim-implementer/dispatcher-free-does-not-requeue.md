---
name: dispatcher-free-does-not-requeue
description: DispatchOne::reserve removes the entity from the position cache and Dispatcher::free only clears reserved_by — a freed truck is queryable again only after the next Dispatcher::update
metadata:
  type: project
---

`DispatchOne::reserve` (`simulation/src/map_dynamic/dispatch.rs:201-208`) does two things:
inserts into `reserved_by` **and removes the entity from `positions` and `lanes`**.
`Dispatcher::free` (`dispatch.rs:110-117`) only removes it from `reserved_by`; it never
restores the position. The position comes back from `Dispatcher::update`, which
`dispatch_system` runs once per tick.

Consequence for tests: an assertion shaped `dispatcher.free(x); assert!(dispatcher.query(..) == Some(x))`
**fails on correct code**. The honest check is `free`, then `Dispatcher::update(&map, world)`
(what `dispatch_system` does), then `query`. Without the free, `query` still skips the entity
via `reserved_by`, so the assertion stays discriminating.

`query` also skips an entity on the target lane whose `dist_along` is past the target
(`dispatch.rs:275-277`) — so a query can legitimately return `None` for a registered,
unreserved truck depending on where it sits.

**Why:** cost about five probe cycles on sov-6qx, 2026-08-28, chasing a `None` that looked
like a broken `free`. `Dispatcher::free`'s own doc comment says it ("It should be re-added to
the cache at the next update iteration") — read it before asserting on dispatcher state.

**How to apply:** when a test needs to prove a truck was released back to the pool, drive
`Dispatcher::update` yourself rather than ticking — a full tick also runs `advance_dispatches`
and will re-grab the truck, closing whatever window you were trying to observe.

Related: [[dispatcher-truck-pool]], [[refusal-signals-need-caller-rollback]].
