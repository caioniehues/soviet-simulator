---
name: finding-exttrade-teleport
description: Ext-trade status 2026-08-26 — the free-goods leak IS fixed, but ext-trade still credits enterprise capital with no Dispatch whenever a freight station exists (bd sov-abs)
metadata:
  type: project
---

Verified against the working tree **2026-08-26** during the sov-lpj consult.
Supersedes the standing brief's claim that "scarcity is currently switched off".

## FIXED — the free-goods leak

The old violation (`sov-ledger-exttrade-cbh`) said market.rs credited a buyer
their full requested quantity *before* checking `find_external`. **No longer
true.** At `market.rs:667-671` the `let Some(ext) = find_external(order.pos)
else { continue; }` now runs **before** `*capital.entry(buyer) += qty_buy`.
With no freight station (the normal early game) unmatched enterprise buy
orders are correctly denied. Early-game shortage is real.

Humans were additionally carved out entirely (market.rs:651-661): their
unmatched buy orders survive the ext-trade pass untouched, so retail clears by
queue and going-without only.

## STILL LIVE — the teleport (bd **sov-abs**, P2, filed 2026-08-26)

Once **any** freight station exists, every unmatched non-human enterprise buy
order is satisfied *instantly*: capital credited at `market.rs:671` with **no
Dispatch**. The dispatch loop at `market.rs:604` iterates
`all_trades[dispatch_start..]` and has already run by the time ext trades are
pushed at `market.rs:673`. Goods appear in the larder having moved nowhere.

Breaks **nothing teleports** (charter-1.0.md:29) and defeats enterprise-side
shortage entirely wherever it is live.

**Why it matters beyond itself:** a hoarding enterprise's withdrawal has no
victim, because everyone downstream is backfilled free at the border. Any
demonstration of the dishonest-enterprise loop must therefore run in a city
with **no freight station**, or it proves nothing about shortage. That is a
binding acceptance constraint on [[ruling-inflation-source]].

**Lesson for future audits:** re-verify the standing "known violations" list
against the tree before citing it. Half of this one had been fixed.
