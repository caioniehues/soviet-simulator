---
name: tests-passing-on-the-teleport
description: Before sov-abs, ext-trade credited buyer capital instantly, and at least two scenario tests were silently proving their claim through that teleport rather than the mechanism they named
metadata:
  type: project
---

Found 2026-08-28 fixing sov-abs (commit `7721cdd`).

Until that commit, `Market::make_trades` satisfied any unmatched **non-human** buy
order with `*capital.entry(buyer) += qty_buy` and no `Dispatch`. Every `TestCtx`
city had a freight station, so **every enterprise buy order was filled for free,
in the same tick, from nowhere**. That is a silent success path underneath a whole
class of scenario assertions.

Two tests were provably resting on it, both verified by re-running them on a clean
tree:

- `retail::scenario_dead_truck_tosource_cancels_without_leak` asserted "the second
  delivery must complete cleanly" (`capital(buyer) == 5`). At that point the
  seller's sell order was `None` — no domestic match was even possible. The 5 units
  were the border credit. **The delivery it claimed to prove never happened.**
- `ledger::scenario_ledger_exttrade_double_spend` drives `make_trades(|_| Some(ext))`
  where `ext = mk_soul(..)` owns no building. Once imports create dispatches, that
  background flour import can never leave `ToSource` (`door_pos` is `None`), so
  `drain_dispatches` hangs. It now drains on its own item via
  `hoarding::drain_dispatches_of`.

**How to apply.** When a market-side change turns 9 of 48 tests red at once, the
first hypothesis should be "these were green for the wrong reason", not "I broke
the sim". Confirm the way it was confirmed here: revert the production diff, keep
the probe, and read the pre-existing state the assertion depended on. A test that
passes identically with and without the mechanism in its own doc comment is
vacuous, and saying so is worth more than the ticket.

Still open, found while doing this and NOT fixed: with a real sell order re-posted,
that retail scenario's freshly spawned truck reaches `ToSource` with a truck
assigned and then sits at `VehicleState::Driving` at a fixed position for 6000
ticks without moving — a vehicle-substrate stall around `unpark`/the `Transporter`
collider, not a market defect.

Related: [[testctx-always-has-freight-station]], [[dispatch-reachability-50-units]],
[[feedback-stale-brief-check]].
