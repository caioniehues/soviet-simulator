---
name: ext-trade-block-eats-unplaceable-orders
description: make_trades' ext-trade block extract_if'd EVERY non-human buy order and dropped the ones find_external could not place; a domestic match masks it from most tests.
metadata:
  type: project
---

`make_trades`' external-trade buy block (`simulation/src/economy/market.rs`)
`extract_if`s every non-human buy order out of `buy_orders` in one pass, then
walks them looking for a border station. Before 2026-09-02 it `continue`d past
the ones `find_external` could not place, destroying the demand. In the default
city the border is closed, so this ate `recipe_init`'s very first order and
every order the ToSource timeout re-posted. Fixed by re-inserting the order.

**Why it hid:** the domestic match runs FIRST in the same pass. If any seller is
offering that item, the order is consumed before it ever reaches the ext block.
Every existing scenario test had a live domestic seller, so removing the fix
left the whole suite green. The guard has to be a buyer with NO possible
domestic match: `sov_ahw_unplaceable_buy_order_survives_the_market_pass` in
`simulation/src/tests/scenarios/hoarding.rs`.

**How to apply:** when judging whether a change to `make_trades` is covered,
ask which tests reach the ext block at all - not which tests touch market code.

See [[market-match-rollback-both-halves]], [[default-city-border-is-closed]].
