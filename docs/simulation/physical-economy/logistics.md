# Logistics

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** logistics
**Last verified:** 2026-09-03

| Scope | 1.0 — charter row Transport and border |

## What this is

Logistics is the one haul authority. It assigns finite vehicle identities to physical hauls
and preserves accountable quantity and custody from pickup through delivery or release. A haul
has a lifecycle: demand-referenced → allocated → vehicle-reserved → pickup → in-custody →
delivered → released. The design target is that failure at any step creates an observable
stalled or waiting job with a recoverable reason, never deleting demand, stock, or vehicle
identity. The current substrate meets this everywhere except the bounded `Loading`/`Returning`
terminal loss, which deletes cargo with only a warning log (see Current substrate below).

The truck leg — vehicle reservation, physical movement, endpoint debit/credit — is the code's
existing strength. The ledger and retail scenario tests prove conservation across the
dispatch transitions they cover; the bounded-loss terminal state is the known exception.

## 1.0 requirement

`SPEC-LOGISTICS-001` — a haul has one authoritative fulfillment authority.

`SPEC-LOGISTICS-003` — a vehicle is finite and may hold only compatible cargo.

`SPEC-LOGISTICS-004` — missing truck, route, source stock, or destination capacity creates
an observable stalled or waiting job.

`SPEC-LOGISTICS-008` — the vehicle SHALL traverse an ordered compatible itinerary to the
source before pickup, and the same identity SHALL traverse to the destination before delivery.

`SPEC-LOGISTICS-011` — source and destination docks have finite loading and unloading rate
budgets. Missing, occupied, unpowered, or zero-rate docks produce visible wait.

## Target design

Finite loading/unloading and dock rates (PLAUSIBLE, bible §6.5; D §4.3). The current code
has no loading/unloading time — freight station cargo is a counter
(`simulation/src/souls/freight_station.rs:34`, `FreightStation.waiting_cargo: u32`).
The target adds: dock power coupling (a dock without electricity cannot load), rate-limited
transfer (partial progress per tick), and deadhead metrics (empty return trips as a visible
planning cost).

Recovery: the design proposes that a completed dispatch requests vehicle recovery and waits
for Roads to acknowledge a parking-slot reservation (`SPEC-LOGISTICS-007`). The current code
already parks: both terminal branches reserve a nearby spot and call `park` before freeing
the dispatcher reservation (`simulation/src/economy/market.rs:1079-1151`).

## Current substrate

`Market::advance_dispatches` (`simulation/src/economy/market.rs:783`) sequences dispatches
through `ToSource → Loading → ToDestination → Unloading`, plus `Returning` for the
demolished-buyer drive-back (`market.rs:173-184`). A truck is reserved from
`Dispatcher` and physically driven over the road network. The seller's capital is debited when
the truck arrives and enters `Loading` (`market.rs:912-917`); the buyer's capital is credited
when it enters `Unloading` (`market.rs:1060-1062`).

Only the `ToSource` acquisition path retries indefinitely: with no truck available (or no
route found) the dispatch stays in `ToSource` and nothing is debited (`market.rs:889-890`).

**The export-side teleport.** In the committed tree, the external-trade sell block of
`make_trades` debits seller capital immediately at match time (`market.rs:732`,
`*cap -= qty_sell`) and attaches a positive `money_delta` (`market.rs:736-741`). No `Dispatch`
is created for the export. Goods vanish from the seller without physical movement. This is a
live violation of the no-teleport pillar.
Import buy side goes through dispatch via a freight station like any domestic trade
(`market.rs:613-652`): no capital moves at match; the border station is debited at `Loading`
and the buyer credited at `Unloading`, exactly like a domestic trade.

**Dispatch failure.** Route failure out of `Loading` is bounded, not retried forever: after
`MAX_RETURN_ROUTE_RETRIES` (20, `market.rs:145`) failed route attempts the dispatch is
dropped, the truck freed, and the cargo — already debited from the seller — deleted with only
a warning log (`market.rs:943-972`). The `Returning` drive-back is bounded the same way
(`market.rs:996-1030`). There is no loss sink: deleted cargo is neither re-credited nor
accounted anywhere, a live conservation gap. Only `ToSource` waits indefinitely.

**Competing authorities.** Company-owned driver delivery (`goods_company.rs:235-270`)
and market dispatch (`market.rs` `advance_dispatches`) are both live paths for the same
trades (`LOG-SUB-007`). This is substrate debt.

## Open questions

- What return-to-depot and reassignment policy applies after delivery or a stalled route?
- What normalized deficit scale preserves the deficit-first ordering across resource kinds?

## Related

- [Custody](custody.md)
- [Reservation](reservation.md)
- [Vehicles](../transport/vehicles.md)
- [Roads](../transport/roads.md)
- [Logistics spec](../../reference/specifications/logistics.md)
- [Trade spec](../../reference/specifications/trade.md)
