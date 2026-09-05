---
name: exttrade-block-eats-buy-orders
description: make_trades ext-trade block used to extract_if + continue every unplaceable non-human buy order; FIXED in sov-ahw v2 (re-insert), plus the ToSource timeout now restores the seller's SellOrder from a make_trades snapshot
metadata:
  type: project
---

Status 2026-09-03: FIXED on branch fix/sov-ahw (v2, approve-with-fixes). The ext block
now does `buy_orders.insert(buyer, order); continue` for orders `find_external` cannot
place, pinned by `hoarding::sov_ahw_unplaceable_buy_order_survives_the_market_pass`.
The ToSource timeout arm restores reservation, sell order, gvt money and buy order;
`Dispatch.sell_order: Option<(Vec2,u32)>` is a `(pos, stock)` snapshot taken in
`make_trades`' filter_map (correct for one seller / many buyers because both fields
are invariant across a pass; qty comes from the dispatch). Restore is clamped to
seller capital; the clamp cannot strand because only reserved qty is ever debited.

**Why:** the first-pass send-back (2026-09-02) found the timeout re-post was eaten by
the ext block in the default (rail-only border) city, killing buyers after 300 ticks.

**How to apply:** when reviewing anything on the dispatch rollback seam, the probe
shape that catches regressions is: default city, seller 5 cereal, buyer, no truck
400 ticks, spawn truck, drain; buyer capital must reach 5. Two-buyer variant proves
the snapshot. Still open: `Market::remove` ToSource arm and the dead-truck arm
restore neither sell order nor buy order nor money_delta (sov-5ut); truck=Some arm
unbounded; Dispatch has 4 positional bincode fields added since the fork with no
VERSION bump (VERSION 0.6.1 unchanged, lib.rs gate is warn-only).
See [[market-exttrade-seam]], [[market-remove-dispatch-drop]].
