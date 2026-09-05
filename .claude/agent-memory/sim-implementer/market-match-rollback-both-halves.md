---
name: market-match-rollback-both-halves
description: A Market match consumes FOUR things; any rollback that undoes fewer than four leaves the trade dead. The sell order is the one everyone forgets.
metadata:
  type: project
---

`Market::make_trades` mutates four things when a domestic trade matches
(`simulation/src/economy/market.rs`): it removes the buyer's `BuyOrder`,
decrements `sell_orders[seller].qty` and REMOVES the order outright at 0,
adds to `reserved[seller]`, and (ext-trade only) moves `Government::money`.
Any code path that tears a dispatch down before the goods move must undo all
four. Restoring only the reservation, the money and the buy order leaves the
stock physically present at the seller but unoffered, so the re-posted demand
can never be served again.

**Why:** sov-ahw's first attempt did exactly that and two gates independently
proved it killed ordinary domestic enterprises in the default city. The
`SellOrder` is the hard one because `stock` and `pos` are not derivable at
rollback time - capture them on the `Dispatch` at match time.

**How to apply:** clamp any restored sell-order `qty` to the seller's capital.
`sell_all` overwrites the order with the seller's FULL (still-undebited)
capital, and `make_trades` skips any seller whose `sorder.qty` exceeds
`capital(seller)` outright - an inflated offer silently blocks that seller
from every domestic match, which reads as a wedge, not as an arithmetic bug.

Sites still carrying the same omission (as of 2026-09-02, ticket sov-5ut):
`Market::remove`'s ToSource arm and `advance_dispatches`' dead-truck arm.

See [[dispatcher-truck-pool]], [[refusal-signals-need-caller-rollback]].
