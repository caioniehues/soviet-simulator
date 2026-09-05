---
name: tosource-timeout-ruling
description: sov-ahw ruling (2026-09-02) — ToSource/None bounded at MAX_SOURCE_WAIT_TICKS = 300 ticks; why churn is accepted, what was rejected, which sibling exits still kill an enterprise
metadata:
  type: project
---

Ruled 2026-09-02 in `/home/caio/sov-ahw-wt` on `4e9e930` + working tree (sov-ahw Phase-4 movement sign-off).

## The numbers (re-derived far too often)

- `TICKS_PER_SECOND = 50 / SECONDS_PER_REALTIME_SECOND(10) = 5`; `TICKS_PER_MINUTE = 300` ticks
  = **6 realtime seconds** at 1x (`prototypes/src/types/time.rs:10,17-18`).
- Truck `max_speed = 22 m/s` (`base_mod/roadvehicles.lua:18`). 300 ticks covers ~132 m of driving, so
  any delivery job beyond ~66 m one-way outlasts the ToSource/None bound; a 1 km border trip is
  ~4,500 ticks, ~15 timeouts.
- `MAX_RETURN_ROUTE_RETRIES = 20` ticks = 0.4 s (bounds a route-search failure, not a wait).

## The ruling

SIGN-OFF. The bound tears down a truck-less `ToSource` dispatch, releases the seller's
reservation, refunds the ext-trade `money_delta`, and re-posts the buyer's order via `buy_until`.
Churn in a truck-poor city is real (warn per timeout, treasury debit/refund sawtooth for imports)
but costs no progress: `make_trades` runs before `advance_dispatches` in `market_update`, so a
re-posted order re-matches the next tick, and the tear-down moves no truck and no capital.

**Rejected:** bare countdown (kills the enterprise — mutation-proved); longer bound (only prolongs
the reservation lock); Dispatcher-BFS reachability inside `find_external` (right long-term fix,
would make this bound a rare backstop, out of scope — see sov-2uv).

**Fairness:** there is no age order anywhere. `Dispatcher::query` runs per dispatch in vec index
order, `swap_remove` scrambles it, domestic matching ranks by distance only, ext-trade matches every
non-human order every tick. A re-post goes to the back of a queue that does not exist.

## Still open after sov-ahw (same enterprise-death shape)

Every dispatch exit that does not credit a live buyer must re-post the demand and refund
`money_delta`. Not done at: `Market::remove` seller-half (demolished freight station — named in
the ticket description, NOT covered), `ToSource/Some` entity-gone, both Loading exhaustion arms.
Required a P2 ticket; natural home is sov-otw's shared helper.

**Observability:** no in-game surface at all; the player sees only a non-producing enterprise.
Mitigation accepted: count timeouts per buyer at the timeout branch, surface "supplier unreachable".

## Process hazard

Two gate agents mutating the same shared worktree collided: my restore put back another agent's
live mutation. Always restore by exact-line sed from the known original, never trust a backup cp
taken in a shared tree.

See [[dispatch-tosource-wedge-surface]], [[dispatcher-pool-and-reachability]].
