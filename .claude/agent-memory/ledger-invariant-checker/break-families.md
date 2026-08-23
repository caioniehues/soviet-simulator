---
name: break-families
description: The conservation-break families confirmed in the Market ledger — a new bucket meeting an old reader, and teardown paths that never cancel in-flight state; check both every time a bucket is added
metadata:
  type: project
---

Four breaks confirmed at `b3857f5`. Each belongs to a family that will recur.

## Family A — new bucket, old reader
`reserved` was added by 6ea4553 and only ONE of five capital-readers consults it.
Concrete: `sell_all` re-posts `qty = capital` (ignoring `reserved`), and the
ext-trade surplus block then debits `capital` down to `stock`; the later dispatch
`Loading` debit takes it negative whenever `reserved > stock`.
**Rule: when a bucket is added, walk [[market-balance-index]] and prove every
reader of the underlying balance either subtracts it or is provably exempt.**

## Family B — teardown never cancels in-flight state
`Market::remove` clears `sell_orders`/`buy_orders`/`capital` and nothing else.
`reserved`, `requested` and `dispatches` survive the soul. `advance_dispatches`
then uses `capital.entry(soul).or_default()`, which RESURRECTS a ledger entry for
a dead soul at `-qty`, and credits a live buyer.
**Rule: every removal path must cancel in-flight operations and release their
reservations. `entry().or_default()` on a teardown-reachable map is a red flag —
it silently converts "this soul is gone" into "this soul now owes 4 coal".**

## Family C — a special-cased branch that skips the release path
`make_trades` reserves for EVERY match, then skips dispatch creation for
`job-opening`. The dispatch is the only thing that ever releases `reserved` or
debits `capital`, so job-opening reserves leak permanently.
**Rule: if acquire is unconditional, release must be too. An `if kind != X` that
sits between acquire and release is a leak.**

## Family D — source credited before its paired sink is known to exist
Ext-trade buy block credits `capital` first, then `let Some(ext) = find_external(...) else { continue }`.
No freight station reachable → free goods, zero money delta. Pre-dates the fork.
**Rule: never credit before the paired debit is proven reachable.**

## Numeric consequence to re-check after any of the above
`capital` was non-negative by construction before deferred debiting. It is not now.
That makes `buy_until`'s `qty - c as u32` (`market.rs:220`) an underflow panic in
debug the moment a consumption-item balance goes negative.
