# Wave 1 economy substrate fact-sheet

**Kind:** reference
**Authority:** reference
**Status:** active
**Owner:** economy
**Last verified:** 2026-08-24
**Commit:** `186e08179b5ad9415dc4cd2d42d77a49303e35d6`

This fact-sheet constrains the rewrite of needs, resources, production, logistics, and trade. Code
reality is descriptive; the charter rulings are normative.

## Domain rulings

| Surface | Current reality | Binding rewrite constraint | Ruling |
|---|---|---|---|
| Needs | Humans buy only bread. A match publishes `Bought`; reaching the seller updates `last_ate` without consuming inventory (`simulation/src/economy/mod.rs:95-104`, `souls/desire/buyfood.rs:78-90`). | Food and Meat are separate dwelling needs. Satisfaction consumes authoritative physical stock; failure persists as waiting or going without. | **VIOLATION** |
| Resources | Lua declares 21 items. Metadata is identity plus `optout_exttrade`; no unit, mass, volume, storage class, transport class, or capacity exists (`base_mod/items.lua:1-108`, `prototypes/src/prototypes/item.rs:6-25`). | The 1.0 catalogue is the charter's 15 physical resources plus import-only Medicine. Water is a network utility, never cargo. | **CURRENT SUBSTRATE ONLY** |
| Production | Recipes atomically consume/produce integer inventory and gate on inputs, output threshold, staffing, and some blackout state (`simulation/src/souls/goods_company.rs:21-64,77-110`). | Request, receipt, true consumption, surplus, and binding constraint are distinct state; inputs, capacity, labour, power, and water are physical gates. | **PARTIAL** |
| Logistics | Domestic matching reserves seller stock; a finite truck gates seller debit and buyer credit at physical endpoints (`simulation/src/economy/market.rs:327-393,462-610`). | Preserve the useful transfer seam, but define one custody ledger and one delivery authority with cancellation and recovery. | **CONSISTENT CORE, UNSAFE INTERFACES** |
| Trade | Imports credit buyers directly; exports debit before a border endpoint is confirmed. Freight stations hold unitless counters (`market.rs:396-451`, `souls/freight_station.rs:30-37`). | One border-only rouble, fixed per-kind prices, physical customs clearance, and no domestic money. | **VIOLATION** |

## Conflict register

### ECO-SUB-001 — Unmatched demand is not a durable queue

The external fallback removes unmatched buy orders with `mem::take`; without a freight station the
order disappears (`simulation/src/economy/market.rs:399-405`). Starved companies normally repost
only after a successful cycle, and waiting citizens do not repost (`souls/goods_company.rs:21-27`,
`souls/desire/buyfood.rs:40-49`).

Classification: **VIOLATION**. Scarcity can erase demand instead of producing waiting time,
substitution, or going without.

### ECO-SUB-002 — Border trade teleports or destroys stock

Imports credit buyer capital immediately (`market.rs:399-416`). Exports debit seller capital before
`find_external`; without a station, goods vanish without a trade or rouble credit
(`market.rs:425-450`). Even with a station, settlement precedes physical freight.

Classification: **VIOLATION** of physical causality and border-only settlement.

### ECO-SUB-003 — Domestic matching is price-free but not queue-clearing

Domestic `money_delta` is zero, which is consistent with no internal money. Matching sorts by
distance and requires one seller to cover the buyer's full quantity; it has no partial multi-seller
fill, request age, or plan priority (`market.rs:274-314`).

Classification: **PARTIAL**. Absence of price does not itself implement shortage allocation.

### ECO-SUB-004 — The inherited treasury still prices domestic actions

Workers, roads, zones, buildings, houses, and trains debit `Government.money`
(`simulation/src/economy/mod.rs:53-55`, `economy/government.rs:22-75`,
`world_command.rs:223-225`). Commands can drive it negative, so it is not a hard gate.

Classification: **CONFLICTING** with the rouble's border-only meaning.

### ECO-SUB-005 — Dishonest-enterprise behavior is test-only

Production reads `Market.requested`, but `set_requested` has no non-test caller. The scenario
manually configures request inflation (`economy/market.rs:240-249`,
`tests/scenarios/hoarding.rs:224-246`). No UI exposes requested, received, consumed, reserved,
in-transit, or surplus state (`native_app/src/gui/inspect/inspect_building.rs:244-299`).

Classification: **UNREACHABLE AND UNOBSERVABLE** in gameplay.

### ECO-SUB-006 — Fulfillment has competing timestamps and authorities

`EcoStats`, `Sold`, and `Bought` record match-time promises. Company drivers react to `Sold`, while
the new market dispatch separately drives a global truck and transfers stock at endpoints
(`economy/mod.rs:64-104`, `souls/goods_company.rs:235-270`, `economy/market.rs:382-393`).

Classification: **CONFLICTING**. Allocation, delivery, consumption, and reporting are not one
coherent contract.

## Rewrite constraints

- Persist unsatisfied requests with age, partial fulfillment, substitution, and going-without evidence.
- Separate reported request, received, consumed, on-hand, reserved, and in-transit quantities.
- Keep production treasury-independent; do not call absence of enterprise finance a soft budget constraint.
- Establish one dispatch/custody authority and make every stalled or cancelled transfer recoverable.
- Settle border stock and roubles at an explicit physical clearance event, never at match time.

## Verification boundary

All cited Rust, Lua, prototype, UI, charter, and scenario locations were reopened. Production
reachability searches confirmed no non-test `set_requested` caller. The simulation test command was
not executed because this thread had cached a prompt-required policy while running with approvals
disabled. No gameplay, save/load, mutation, profiler, or reference-game run was performed.
