# Logistics

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** logistics
**Last verified:** 2026-08-28

| Scope | 1.0 binding |

## What this is

Logistics is the one haul authority. It assigns finite vehicle identities to physical hauls
and preserves accountable quantity and custody from pickup through delivery or release. A haul
has a lifecycle: demand-referenced → allocated → vehicle-reserved → pickup → in-custody →
delivered → released. Failure at any step creates an observable stalled or waiting job with
a recoverable reason; it never deletes demand, stock, or vehicle identity.

The truck leg — vehicle reservation, physical movement, endpoint debit/credit — is the code's
existing strength. Twelve ledger tests and fourteen retail tests prove conservation across
dispatch transitions.

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
releases the truck at its final position without parking it (`LOG-SUB-008`).

## Current substrate

`Market::advance_dispatches` (`simulation/src/economy/market.rs`) sequences dispatches
through `ToSource → Loading → ToDestination → Unloading`. A truck is reserved from
`Dispatcher`, physically driven over the road network. The seller's capital is debited when
the truck arrives and enters `Loading`; the buyer's capital is credited when it enters
`Unloading`.

The in-flight change `sov-ahw` (uncommitted) adds a `ToSource` timeout; the committed
behaviour is that a truck without a route waits indefinitely in `ToSource`.

**The export-side teleport.** In the committed tree, the external-trade sell block of
`make_trades` debits seller capital immediately at match time (`market.rs:774`,
`*cap -= qty_sell`). No `Dispatch` is created for the export. Goods vanish from the seller
without physical movement. This is a live violation of the no-teleport pillar.
Import buy side was fixed by `sov-abs`: imports now go through dispatch via a freight
station like any domestic trade.

**Dispatch failure.** No terminal recovery policy exists. No route or no truck causes
indefinite retry. There is no timeout, reassignment, cancellation, or player-visible
stalled-job state (`LOG-SUB-009`).

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
