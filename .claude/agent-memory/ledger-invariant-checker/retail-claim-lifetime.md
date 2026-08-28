---
name: retail-claim-lifetime
description: The full lifetime of a RetailClaim — who can end it, in what order, and why abandoning a BoughtAt customer conserves; includes the ordering fact that update_decision_system runs BEFORE company_system
metadata:
  type: project
---

Re-derived 2026-08-28 at branch `fix/sov-wave-souls` HEAD `5349f34`.

A `RetailClaim` (`market.rs:99-102`, `:155`) is the human retail leg's only
in-flight state. Keyed by **buyer**, so one human holds at most one.

**Nothing is debited while a claim stands.** The match only does
`reserved[seller] += qty` (`market.rs:594`); `capital[seller] -= qty` happens
solely in `settle_retail` (`:466-477`) at eat-time, and the buyer is credited
nothing (the loaf is destroyed by being eaten). **No money moves at all:** the
domestic match sets `money_delta: Money::ZERO` (`:530`); only ext-trade
attaches money (`:680`, `:715`) and humans are carved out of ext-trade. So an
abandoned claim can never strand a coin — there is none.

Four, and only four, ways a claim ends:
1. `settle_retail` — the human ate.
2. TTL sweep at the end of `advance_dispatches` (`market.rs:1068-1078`),
   `RETAIL_CLAIM_TTL_TICKS = TICKS_PER_HOUR`; releases `reserved`.
3. `Market::remove(buyer)` (`:263-269`) — releases `reserved` on the
   **seller's** row.
4. `Market::remove(seller)` (`:274`) `retain(|_, c| c.seller != soul)`, then
   `:275-282` wipes the seller's whole `capital`/`reserved` row.

Plus displacement: `make_trades` (`:614-637`) releases a displaced claim's
reservation on the OLD seller's row before overwriting. That is what makes it
safe for a customer to abandon a claim and immediately re-queue.

## Ordering fact that a code comment gets wrong
`init.rs` registers `update_decision_system` (`:62`) **BEFORE** `company_system`
(`:65`), and `company_system` is what kills a company whose building is gone
(`goods_company.rs:195`). So on the tick a store is demolished, `BuyFood::apply`
sees the dead building while **the seller soul is still alive and its
reservation still stands**. The comment at `buyfood.rs:127-129` claims
`Market::remove` has already released it. It has not; removal (or the TTL)
follows later, which is why it still conserves.

Only `goods_company.rs` posts sell orders in production (`sell_all` at `:60`
and `:170`), so a retail seller is always a `GoodsCompany` and always dies with
its building on the same tick. No long-lived orphan claim on a live seller.
