# ADR-0003: Border settlement — money at delivery, physical exports, accountable border stock

**Kind:** decision
**Authority:** binding
**Status:** accepted
**Owner:** project lead
**Last verified:** 2026-09-04
**Date:** 2026-09-04
**Decision makers:** project lead (caioniehues), interviewed by the lead agent

## Context and problem

External trade was half-physical. After `sov-abs` the import half moves goods by truck, but
five beads recorded the gaps (all confirmed against code 2026-09-04, verdicts on the beads):

- `sov-7f7`: `gvt.money += trade.money_delta` fires at MATCH time (`economy/mod.rs:104`) for
  both border halves, while goods move over thousands of ticks. The pairing is gone.
- `sov-20g`: exports debit stock at match (`market.rs:730`) with no dispatch — the dispatch
  loop (`:658`) runs before the export branch pushes (`:706-741`), so exports structurally
  cannot get a truck.
- `sov-uo5`: the freight-station seller holds train counters only
  (`freight_station.rs:31-36`); import trucks load from nothing. No external money balance
  exists anywhere — the sole `Money` is `Government.money`.
- `sov-bub`: bounded Loading/Returning failures delete cargo after the seller was debited.
  Investigation corrected the frame: an intentional honest-loss sink EXISTS (7+ deletion
  sites, all `log::warn`, re-credit only by physical return) — it is unnamed and invisible
  to the Planner, not absent.
- `sov-5ut`: consumed orders are wiped at match and never re-posted, money never reversed,
  on all four dispatch-exit shapes; `ToSource` retries forever (no timeout exists anywhere).

Domestic, retail and job-opening deltas are ZERO by construction (`market.rs:539-545`); only
border legs move money. EcoStats already routes the legs separately.

## Decision drivers

- "Roubles clear only at the border" is the game's identity; the mechanics-index calls the
  current match-time clearing a violation on both halves.
- The import half already proves the physical pattern works — the fixture trades across the
  border today.
- Bounded-retries-then-honest-loss is settled, tested behavior (the retail test asserts it);
  the defect is observability, not mechanics.
- Four exit sites with the same three omissions is a missing abstraction, not four bugs.

## Considered options

Per question, with the rejected alternative in parentheses:

1. Money timing: move to delivery (keep match-time as "commitment", document the gap).
2. Exports: real dispatch to the border door (keep teleport as a documented approximation).
3. Border stock: bounded accountable stock fed by train arrivals (keep infinite-supply counters).
4. Loss sink: name and surface it (change behavior to return/re-credit instead).
5. Exits: one shared helper (four per-site fixes).

## Decision outcome

1. **Money moves at delivery.** Import `money_delta` applies on Loading arrival (`:912`),
   export on ToDestination arrival (`:1060`). Match creates the commitment (orders wiped,
   reservation held); crossing creates the money movement.
2. **Exports get a real dispatch.** Fix the loop order so the export branch pushes before the
   dispatch loop iterates; export trucks drive seller → border door like imports drive
   border → buyer.
3. **Bounded border stock (Border custody).** The freight station gains a stock ledger the
   import seller draws from, decremented on dispatch, replenished by train arrivals (the
   current counter-consumption at `:101-103` becomes a real restock event). Bounded, not a
   full economy. Glossary term: **Border custody**.
4. **Name and surface the loss sink; do not change deletion behavior.** A `Lost` account in
   the model, visible in the inspector; the seven deletion sites record into it instead of
   only warning. Glossary term: **Lost**.
5. **One shared exit helper** used by every dispatch termination: re-post the buyer order,
   refund `money_delta` where nonzero (i.e. border legs dead between match and delivery),
   restore the seller order where appropriate. Unblocks the `ToSource`-forever hang by
   giving it the same bounded-exit shape (bound value is implementation detail, not decided
   here).

## Consequences

- `sov-7f7`, `sov-20g`, `sov-uo5` close structurally; `sov-bub` closes by naming +
  surfacing (deletion behavior unchanged — the retail test asserting it stays green);
  `sov-5ut` closes with the helper.
- The mechanics-index import/export rows flip from VIOLATION to EXISTS once all five land;
  they must land together or not at all — a moved money leg with teleport exports (or vice
  versa) is worse than the current consistent violation.
- The fixture replay exercises border trade both directions, so the determinism gate and
  the EcoStats L0 window cover the new paths; extend the census only if a new entity kind
  appears (none is planned — stock is a ledger, not an entity).
- Cost: the export dispatch path and the stock ledger are new simulation surface with no
  substrate behind them; the money move touches the most-read line in the economy
  (`mod.rs:104`).

## Confirmation

- A materialised fixture shows import money moving on Loading arrival and export money on
  ToDestination arrival (EcoStats import/export legs, or probe output).
- An export dispatch exists in `market.dispatches()` with a border-door target; the loop
  order guarantees every pushed trade is iterated.
- Import dispatch fails when border stock is empty (observable going-without at the border).
- The inspector shows `Lost` entries for a severed-road scenario; the retail bounded-loss
  test still asserts deletion.
- Every dispatch exit in the exit table (recorded on `sov-5ut`) re-posts or fulfills —
  none silently drops.

## More information

- The five beads carry the code-truth comments and the full exit table (2026-09-04).
- [ADR-0002](0002-fixture-world-is-a-materialised-replay.md): the fixture that covers the
  new paths.
- Glossary: **Border custody**, **Lost**, see also **Custody**, **Dispatch**.
