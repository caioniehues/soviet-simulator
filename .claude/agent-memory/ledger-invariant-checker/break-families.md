---
name: break-families
description: The conservation-break families confirmed in the Market ledger — new bucket vs old reader, teardown that never cancels, map-keyed claims that silently overwrite, and a credited balance with no sink; check all of them every time a bucket is added
metadata:
  type: project
---

Families A-D confirmed at `b3857f5`. Families E-H confirmed 2026-08-26 against
the `sov-dispatch-wedge-ab4` working tree. Each family will recur; check all of
them whenever a bucket or a claim/reservation is added.

**Third pass 2026-08-26 (round-3 implementer tree, HEAD `f89bc3b`): ALL
families A-H are CLOSED. Verdict CONSERVED.** See each family's Status line.

What made each finding findable in the first place, in order of yield:
- **E** — checking the *data layer* (`bread` is not `optout_exttrade`), not the code.
- **F** — asking who else can write a map key that owns a reservation.
- **G** — noticing a `-> bool` that every caller discards.
- **B (buyer half)** — testing the *other side* of a symmetric `retain`.

## Family A — new bucket, old reader
`reserved` added, only one of five capital-readers consulted it.
**Status: CLOSED for the paths that mattered.** Ext-trade surplus loop now
subtracts `reserved` (`market.rs:677-688`); the domestic match's inner guard
`cap_seller - already_reserved < trade.qty` rejects a second buyer for
already-reserved stock.
Residual (latent, not exploitable): `sell_all` still posts `qty = capital`
ignoring `reserved`; `buy_until`'s `qty - c as u32` is unguarded against
negative `c`. Re-verified 3rd pass: `buy_until` is called ONLY on recipe
*consumption* items, and `recipe_should_produce` still gates consumption on
raw `capital >= item.amount` (`goods_company.rs:35`), so an input item's
capital never goes negative. The cast stays unreachable.
**Rule: when a bucket is added, walk [[market-balance-index]] and prove every
reader of the underlying balance either subtracts it or is provably exempt.**

## Family B — teardown never cancels in-flight state
**Status: CLOSED (3rd pass).** Was the worst break of the 2nd pass.
`Market::remove` (`market.rs:250-391`) now takes `map/binfos/world/dispatcher/
tick` and settles the SURVIVING counterparty per `DispatchState`:
- `ToSource` → release `reserved[seller]`, free truck, drop (nothing was debited).
- `Loading`/`ToDestination`/`Returning` → route the truck back to the seller
  and transition to `Returning`; no route or no seller ⇒ declared loss with
  `log::warn!` + truck freed.
- `Unloading` → declared loss (buyer no longer exists to receive).
- Dead *seller* → final `retain(|d| d.seller != soul)` after the buyer loop;
  the seller's whole row is wiped so there is nothing to strand.

Loop skip is `if d.buyer != soul || d.seller == soul { continue }` — the
`d.seller == soul` clause routes self-trades to the seller `retain` instead,
so no dispatch is handled twice.

**`Returning` is now REACHABLE end-to-end.** The 2nd pass called it
structurally unreachable for a demolished GoodsCompany buyer; that is fixed,
because `Market::remove` itself creates the `Returning` dispatch rather than
depending on `advance_dispatches` seeing the demolished building first.
Proven by `audit_end_to_end_demolish_buyer_building`: `map.remove_building`
on a loaded-truck buyer, total 10 → 10.

`Returning` also gained the truck-vanished guard its three siblings had
(`market.rs:930-945`), and `ledger.rs` `total_qty` now counts `Returning` in
the in-flight bucket.

**Mutation-proven, 3rd pass.** Re-inserting the blind drop
(`self.dispatches.retain(|d| d.buyer != soul)` ahead of the settle loop) turns
3 of 5 audit tests red, with `AUDIT C: before=10 after=0` — the exact 2nd-pass
break. Reverted after.
**Rule: every removal path must cancel in-flight operations and release their
reservations — on BOTH counterparties. Test removal of each side separately;
they are different code paths. A `retain` that drops a record holding a
debit/reservation against a *surviving* soul is a destruction, not a cleanup.**

## Family C — a special-cased branch that skips the release path
job-opening reserved but never released. **Status: CLOSED** — `make_trades`
settles job-opening immediately (`*capital -= trade.qty`) instead of reserving.
**Rule: if acquire is unconditional, release must be too.**

## Family D — source credited before its paired sink is known to exist
**Status: CLOSED.** `market.rs:655-660` now does `find_external` first, credit
second.
**Rule: never credit before the paired debit is proven reachable.**

## Family E — a credited balance with no sink anywhere in the codebase
`make_trades`' ext-trade buy block credited `capital[buyer] += qty` for ANY
buyer including `SoulID::Human`, and nothing ever debits a human's
`capital[bread]`.
**Status: CLOSED.** `market.rs:648-650` uses `buy_orders.extract_if(.., |s, _|
!matches!(s, SoulID::Human(_)))` so human orders never enter `btaken` — no
credit, no `money_delta`, order survives for next tick's domestic match.
**Rule: for every `+=` on a balance, name the code that will later decrement it.
If you cannot name a reader, the balance is a leak, not a ledger.**

## Family F — a map-keyed claim that silently overwrites
`retail_claims.insert(buyer, ...)` on a map keyed by buyer; a second match
bumped `reserved` again and orphaned the first reservation.
**Status: CLOSED.** The displaced claim's reservation is released on the OLD
seller's row before the new claim stands.
Residual latent assumption: the release uses the per-kind loop's local
`reserved` map guarded by `debug_assert_eq!(old.kind, kind)`, a no-op in
release. Safe only because `buyfood.rs:82` is the ONLY human buy-order issuer
and hardcodes `bread`. **A second human-purchasable item would silently
decrement the wrong market's row in release builds** — this is the single
highest-value thing to re-check when retail grows past bread.
**Rule: `map.insert(k, v)` where `v` owns a reservation is an unpaired acquire
whenever `k` can repeat.**

## Family G — settlement return value ignored, effect applied anyway
**Status: CLOSED.** `buyfood.rs` `BoughtAt` checks
`market.retail_claim(...).is_some()` BEFORE settling and only advances
`last_ate` inside that branch.
**Rule: when a settlement can fail, the physical effect it pays for must be
conditional on its success.**

## Family H — an unbounded retry that hides an unpayable debt
**Status: CLOSED.** Bounded by `MAX_RETURN_ROUTE_RETRIES = 20`; after the
bound the dispatch is dropped as a declared physical loss with NO re-credit.
**Rule: an unbounded retry on a path where the debit already happened is a
silent destruction with no timestamp. Bound it and declare the loss.**
**Test trap:** proving "truck was freed" by removing every road strands every
truck for unrelated reasons — the reuse assertion fails as a test artifact.

## Release-path idempotence (3rd pass, verified)
Every reservation-release site removes the owning record in the SAME
operation, which makes double-release structurally impossible:
- `settle_retail` — `retail_claims.remove()` first, returns `false` if absent.
- `Market::remove` — `retail_claims.remove(&soul)` / `retain`.
- TTL sweep — `retain` returning `false`.
- Claim overwrite — releases the displaced value.
Dispatcher `free`/`unregister` are `BTreeSet`/map removes: idempotent.
**Rule: make release consume the record. A release that only decrements a
counter can fire twice; one that removes its own record cannot.**

## The buyfood reset signal (3rd pass, verified)
`buyfood.rs:105-113` resets `WaitingForTrade -> Empty` when
`buy_order(human).is_none() && retail_claim(human).is_none()`.
Both conditions are independent of `apply()`'s poll cadence — which matters
because `apply` runs only every ~30-80 ticks, so a claim can be created AND
expire between two calls (this killed the reviewer's suggested `claimed: bool`
approach). Never-matched (order still live) stays parked at score 0.0;
matched-then-resolved resets. The reset requires `retail_claim` to be None, so
a new claim can only ever follow a released one — no double-reserve.

## Money (3rd pass, verified)
Non-zero `money_delta` appears at exactly two sites, both ext-trade with a
foreign counterparty (`market.rs:667, 701`). The domestic match sets
`Money::ZERO` (`:517`) and humans are carved out of ext-trade entirely.
**No physical flow is gated by money.** Clearing stays by queue, substitution
and going without.

## Numeric consequences to re-check after any of the above
- `capital` is `i32` and can be negative; `reserved` is `u32`.
  `recipe_should_produce` does `capital - reserved as i32` — with negative
  capital this only makes the `<` more true (produce), so it is safe.
- `Returning`'s credit `+= qty as i32` (`:954`) exactly mirrors ToSource's
  debit `-= qty as i32` (`:801`); `qty` is `u32` and never re-derived, so the
  round trip is exact.
- `Market::remove`'s new code introduces NO casts — only `saturating_sub` on
  `u32` reserved.
