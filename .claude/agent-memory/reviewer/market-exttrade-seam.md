---
name: market-exttrade-seam
description: The Egregoria market's external-trade block teleports goods and bypasses any seller-side reservation; check it whenever reviewing Market/dispatch/stock changes
metadata:
  type: project
---

`simulation/src/economy/market.rs::make_trades` has TWO capital paths per item,
and reviewers keep only looking at the first:

1. the domestic potential-trade match loop (the one everybody edits), and
2. the `if !*optout_exttrade` block right after it, which
   - credits every remaining buy order instantly (`*capital.entry(buyer) += qty_buy`) — goods from nothing, and
   - debits every leftover sell order (`*cap -= qty_sell`) with only a `*cap < qty_sell` guard.

Path 2 knows nothing about `reserved` or in-flight `Dispatch`es.

**Why:** `base_mod/items.lua` sets `optout_exttrade = true` on **job-opening only**;
all 20 physical goods take path 2. So any "nothing teleports" / deferred-transfer
work that only touches the domestic loop is unenforced for every real good, and
any seller-side reservation added to the domestic loop can be double-spent by
path 2, driving `capital` negative.

**How to apply:** when a diff touches `Market`, stock movement, reservations, or
the "nothing teleports" pillar, read the ext-trade block as the paired consumer
side and check the optout table in `base_mod/items.lua` before believing any
claim about capital. Related: [[sim-test-harness-quirks]].
