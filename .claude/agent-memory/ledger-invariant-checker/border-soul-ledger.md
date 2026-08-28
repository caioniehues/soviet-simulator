---
name: border-soul-ledger
description: After sov-abs the external border is a SOUL with a negative capital row — imports move physically, exports still teleport out, and negative capital is now routine (a buy_until underflow landmine)
metadata:
  type: project
---

Established at commit `7721cdd` on `fix/sov-wave-market` (audited 2026-08-28,
verdict CONSERVED). Supersedes the old "ext-trade credits capital at match"
shape recorded in [[break-families]].

## The import leg is now an ordinary Dispatch
`make_trades` pushes ext-trade BUY trades **before** the dispatch-creation
loop, so they land in `all_trades[dispatch_start..]` and get a `Dispatch`
whose `seller` is a `SoulID::FreightStation`. No capital moves at match.

- `capital[FreightStation] -= qty` at **ToSource arrival**.
- `capital[buyer] += qty` at **ToDestination arrival**, once. Match credits
  nothing; `Unloading` credits nothing.
- Same `Dispatch::qty` on both sides, so the numbers cannot diverge.

**The border is an ACCOUNTED infinite source.** Its negative `capital` row is
the import ledger, which is why `total_qty` (capital sum + in-flight) stays
conserved across an import. Two consequences:

1. **Negative capital rows are now routine.** `buy_until` (`market.rs:421`)
   computes `qty - c as u32`; with `c < 0` that is a ~4.29e9 wrap and an
   underflow panic in debug. Safe **only** because nothing calls `buy_until`
   or `sell_all` on a `FreightStation` soul. Any future code that does will
   detonate.
2. `Market::remove(FreightStation)` does `capital.remove(&soul)`, deleting the
   accumulated negative row. Any global sum jumps by the whole import volume
   when a station is demolished — a `total_qty` test spanning a station
   demolition reads wrong.

## The export leg still teleports OUT
The seller-surplus block does `*cap -= qty_sell` at match with **no Dispatch**,
and does **not** credit `capital[FreightStation]`. So exports vanish instantly
and the border row drifts monotonically negative. sov-abs fixed only the
import half; "nothing teleports" cuts both ways. Successor-ticket material.

## Money and goods are decoupled on the import leg
`gvt.money += trade.money_delta` fires at match (`economy/mod.rs:104`) while
the goods arrive many ticks later or never. Demolish the buyer mid-flight and
the truck returns the goods to the border — which then holds both the goods
and the money. Money is still conserved into the external counterparty and
never gates a physical flow, so no Kornai violation, but the pairing is gone.
Fix if wanted: apply an import's `money_delta` at ToDestination arrival.

## `buy_until` does not know about in-flight incoming
It reads raw `capital` only. A company that completes a recipe while an
import is in flight re-posts and draws a **second** import from an infinite
source. Conserved (the border pays for both), but over-supply. The same shape
exists for domestic dispatches and predates this diff; what is new is that the
import supply has no ceiling.

## `find_external` reachability filter
`economy/mod.rs` now filters freight stations by
`map.nearest_lane(door_pos, Driving, Some(DISPATCH_LANE_CUTOFF /* 50.0 */))`.
**Probed 2026-08-28:** the hardcoded `START_COMMANDS` station door is at
`(4297.4, 6315.3)` and `nearest_lane` returns `None`. So a fresh game — test
or production — now has **no import border at all** until the player builds a
reachable station. Never reason about ext-trade in a default world without
accounting for this.

## `DispatchState::ToSource` still has no retry bound
sov-jcl bounded the `Loading` leg. `ToSource` with no truck available or no
route simply retries forever. Nothing is debited there, so quantity is
conserved, and it is bounded in practice because a starved factory stops
re-posting buy orders — but imports newly enter this path.
