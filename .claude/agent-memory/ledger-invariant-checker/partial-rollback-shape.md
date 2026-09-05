---
name: partial-rollback-shape
description: Family J — a dispatch rollback that undoes reserved/money/buy-order but NOT the seller's sell_orders[seller].qty decrement; conserved but strands stock off-market and lets the ext block eat the re-posted order (sov-ahw audit 2026-09-02)
metadata:
  type: project
---

Confirmed 2026-09-02 on `/home/caio/sov-ahw-wt` (branch `fix/sov-ahw`, working
tree over `4e9e930`). Verdict for the diff: LEDGER CONSERVED, one major
liveness finding.

**The shape.** `make_trades` mutates FOUR things at a domestic match:
`buy_orders.remove(buyer)`, `sell_orders[seller].qty -= qty` (removing the order at
0), `reserved[seller] += qty`, and (ext-trade only) `gvt.money += money_delta` in
`market_update`. Every rollback path — `Market::remove` ToSource arm, the
dead-truck ToSource arm, and the new `MAX_SOURCE_WAIT_TICKS` timeout — restores
`reserved` (and the timeout also money and the buy order) but NONE restores the
seller's sell order. Quantity stays in `capital`, so the ledger is conserved, but
the stock is invisible to the market until the seller's next `recipe_act` →
`sell_all`.

**Why it bites after sov-ahw.** The timeout re-posts the buyer's order via
`buy_until`, but the seller no longer offers the stock, so no domestic match
happens next tick, and the ext-trade block `extract_if`s every non-human buy
order and `continue`s past any `find_external` cannot place — the re-posted
order is destroyed one tick later. Measured: t=300 buy order Some(5),
t=301 None; a truck spawned afterwards delivers nothing in 2000 ticks. With
the bound raised to u32::MAX-1 the same truck loads the goods (seller 5→0,
in-flight 5). `MAX_SOURCE_WAIT_TICKS` = TICKS_PER_MINUTE = 300 ticks = 6 real
seconds, so "all trucks busy" reaches this on ordinary play.

**CLOSED by sov-ahw v2 (2026-09-03, verified in `/home/caio/sov-ahw-ledger-wt`).**
`Dispatch` now carries `sell_order: Option<(Vec2, u32)>` = the seller's `(pos, stock)`
snapshotted in `make_trades` (`sold_from` map, keyed by seller, qty NOT stored). The
timeout restores per-dispatch `qty` onto the existing order or re-inserts from the
snapshot, clamped `.min(capital)`. The clamp is load-bearing, not cosmetic: with
`sell_all` between match and timeout the unclamped restore gives offer 10 on capital
5 (mutation red), and the potential loop skips any seller whose `qty_sell >
capital_sell` — a liveness wedge. Two buyers on one seller restore to exactly the
sum (5+5 → 10). The ext block's `find_external` None arm now re-inserts the order
(`btaken` is collected first, so no iterator hazard, no double match). Still open,
for sov-5ut: `Market::remove` ToSource arm and the dead-truck arm restore neither
sell order nor `money_delta` — a demolished ext-import buyer leaves the border
payment unrefunded.

**Check every time a rollback is added:** list every map the match mutated and
prove each one is undone, not just the reservation. `sell_orders` is the one
everybody forgets because it is decremented, not inserted.

**Harness note (lesson):** the one flaky observation (re-posted order absent at
t=300 in one run) is almost certainly because the evidence-auditor was mutating
`market.rs` in the SAME worktree at the same time; my "clean" snapshot even
captured one of their live mutations (`if true { // MUT-e`). Two mutators in one
tree produce false reds and contaminated restores. Before any mutation, ask the
lead whether another agent is mutating that tree, and `git diff` the file
against the reviewed diff right before snapshotting.

See [[market-balance-index]], [[break-families]], [[border-soul-ledger]].
