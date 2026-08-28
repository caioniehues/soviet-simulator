---
name: export-gate-blocks-on-teleport
description: The freight-station driving-lane gate is load-bearing for exports for the WRONG reason - ungating them (sov-nun) drains sellers to the border and breaks the hoarding core-loop test
metadata:
  type: project
---

`find_external` in `simulation/src/economy/mod.rs` is ONE closure consumed by both the
import block (`market.rs:~637`) and the seller-surplus export block (`market.rs:~727`).
It filters freight stations to those within `DISPATCH_LANE_CUTOFF` of a `LaneKind::Driving`
lane.

**Ungating the export half cannot land until exports become physical (sov-20g).**
Measured 2026-08-28 on `fix/sov-wave-market`, A/B on
`tests::scenarios::hoarding::scenario_0151_inflated_request_hoards_honest_does_not`,
identical apart from the export gate:

| export gate | seller capital | honest | inflated | result |
|---|---|---|---|---|
| gated | 985 | 2 | 5 | pass |
| ungated | 0 | 0 | 1 | fail |

**Why:** the export block computes `qty_sell = (order.qty - reserved) - order.stock` and,
on a match, debits capital and pushes a `Trade` with a `money_delta` — **no `Dispatch`, no
truck, immediate**. A seller posting `sell(qty 1000, stock 0)` therefore loses all 1000
units to the border in one pass, and every domestic buyer starves from the next tick.
That is sov-20g (export half still teleports) plus sov-b70 (border out-competes domestic).

**How to apply:** do not "fix" sov-nun on its own; it needs `bd dep sov-20g --blocks sov-nun`.
If a brief asks you to ungate exports, run that hoarding scenario before believing it is a
small change. Related: [[tests-passing-on-the-teleport]].

**Also verified while there:** `MAX_RETURN_ROUTE_RETRIES = 20` fires on the **20th** failed
attempt, not the 19th — `return_route_retries` starts at 0, increments after each failure,
and the arm is `retries + 1 >= MAX`. A review claim of "19 attempts" was wrong.
