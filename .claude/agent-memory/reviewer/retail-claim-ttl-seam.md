---
name: retail-claim-ttl-seam
description: The store→consumer retail path (RetailClaim, no dispatch, eat-time settlement) and the buy_order+claim double-None signal buyfood uses to detect a resolved wait
metadata:
  type: project
---

Added by sov-dispatch-wedge-ab4 (Option C, kornai-economist ruling). Store→consumer
sales create **no `Dispatch`**: the human's own walk is the physical movement.

- `Market::retail_claims: BTreeMap<SoulID, RetailClaim>` keyed by BUYER (a human only
  ever has one outstanding bread order).
- Created in `make_trades` (market.rs:594-625) on the `SoulID::Human(_)` buyer branch,
  which `continue`s past the `dispatches.push`.
- The good stays in `capital[seller]` and is added to `reserved[seller]` until
  `settle_retail` (market.rs:453) debits capital + releases reserved at eat-time.
  Buyer is credited NOTHING — the loaf is destroyed by being eaten.
- TTL `RETAIL_CLAIM_TTL_TICKS = TICKS_PER_HOUR`, swept at the END of
  `advance_dispatches` (market.rs:1001). Expiry releases `reserved` and must NOT touch
  `last_ate` (never game over).
- Humans are excluded from ext-trade: `buy_orders.extract_if(.., |s,_| !matches!(s,
  SoulID::Human(_)))` (market.rs:648) — money must never clear the retail path.

## The poll-cadence trap (the round-2→3 correction worth remembering)

`BuyFood::apply` is NOT polled every tick — `decision.wait` throttles it to roughly
every 30-80 ticks. So any "did I ever observe a claim" boolean is unsound: a claim can
be created AND expire entirely between two `apply` calls, leaving the flag false
forever and wedging `WaitingForTrade` permanently.

The correct, cadence-independent signal (buyfood.rs:105-114) is **both** of:
`market.buy_order(SoulID::Human(id)).is_none() && market.retail_claim(...).is_none()`.

Why it's sound: `make_trades` removes the buy_order entry (market.rs:546-551) and
inserts the claim in the SAME synchronous call, so there is no window where both are
None while a legitimate wait is in progress.
- never matched → order still live → no reset, stays parked at `score()` 0.0.
- matched then TTL-expired → order gone, claim gone → reset to Empty, re-queue.
- matched and arriving → `bought` drain sets `BoughtAt(b)` first (buyfood.rs:88-92),
  and the guard is gated on `matches!(self.state, WaitingForTrade)`.
- seller's building demolished after the match → `find_trade_place` returns None so the
  state stays `WaitingForTrade`, but the claim is still Some, so no reset until the TTL
  fires an hour later. Bounded and self-healing, not a wedge.

**How to apply:** never "simplify" that double-None into a bool or a single check, and
never assume `apply` sees every tick. `update_decision_system` (init.rs:62) runs before
`market_update` (init.rs:98), so buyfood reads the Market as of the end of the previous
tick.

Related: [[market-remove-dispatch-drop]], [[market-exttrade-seam]].

## Two unbounded-wait shapes that are PRE-FORK, not diff-introduced (adjudicated 2026-08-26)

A cross-vendor reviewer send-back on the sov-dispatch-wedge-ab4 tree raised both;
both CONFIRMED real, both verbatim in `git show HEAD:` (HEAD = f89bc3b), so neither
is a defect of that diff. File as beads, never as send-back on the wedge work.

**(1) `Loading` + live buyer + `Itinerary::route` == None retries forever.**
market.rs:839-850: `else if let Some(buyer_pos) = door_pos(buyer, ...)` computes a
route and, on `None`, falls through with the bare comment "No route found: stay in
Loading (ticks_left at 0) and retry next tick". No counter. The SIBLING branch (the
demolished-buyer return path, market.rs:874) is bounded by
`MAX_RETURN_ROUTE_RETRIES = 20` (market.rs:138) — so the file bounds the return leg
and not the outbound leg. Seller already debited, truck held, `ticks_left` stuck at 0.
Verbatim at `HEAD:562-563`.

**(2) `BuyFoodState::BoughtAt(b)` never resets when building `b` is demolished.**
buyfood.rs:117-147: every reset (`self.state = Empty`) lives INSIDE
`if loc == &Location::Building(b)`. The else arm is only
`GoTo(Destination::Building(b))`. The router refuses to strand a human on a missing
building — router.rs:87-92 does `map.buildings.get(build)` -> `None` ->
`router.cur_dest = router.target_dest; return;` with NO steps pushed — so the human
never walks, never enters, `loc` never equals `Building(b)`, and the arm is a
permanent sink. The retail claim's TTL sweep releases the market-side reservation, so
the LEDGER is fine; the human is the thing that wedges. Verbatim at `HEAD:86-94`.
Note `score()` (buyfood.rs:58-62) returns 1.0 only when already at the building, so a
stranded human falls back to the hunger ramp and re-scores forever without progress.

**How to apply:** when reviewing this diff family, check any new unbounded wait
against BOTH of these; and do not let a cross-vendor reviewer's true-but-pre-existing
finding convert into a send-back — diff `git show HEAD:<file>` first.
