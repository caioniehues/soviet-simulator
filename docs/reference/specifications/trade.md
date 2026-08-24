# Trade specification

**Kind:** specification
**Authority:** binding
**Status:** draft
**Owner:** economy
**Last verified:** 2026-08-24

The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT, RECOMMENDED, NOT
RECOMMENDED, MAY, and OPTIONAL in this document are to be interpreted as described in RFC 2119 and
RFC 8174.

## Purpose

Trade governs the only monetary boundary in the simulation: a single rouble settles an import or
export only when physical goods clear a customs office. Domestic allocation, dispatch, and
production remain price-free.

## Scope and exclusions

1.0 includes multiple customs offices, all sixteen charter resources at fixed per-kind prices, and
one border rouble. Medicine is import-only; Water is never cargo. Dual currency, loans, dynamic
markets, ships, docks, pipelines, cableways, containers, aircraft, vehicle manufacture, and fuel
lifecycle are not 1.0 mechanisms. Archived CS1 and W&R trade material is comparison evidence only.

## Invariants

- `SPEC-TRADE-001` — Domestic matching, allocation, reservation, dispatch, production, and
  consumption MUST NOT debit, credit, rank by, or otherwise clear through roubles.
- `SPEC-TRADE-002` — A trade order has a physical item, quantity, direction, customs office,
  domestic haul, and clearance state. Import stock appears only after inbound physical clearance;
  export stock leaves domestic custody only after outbound physical clearance.
- `SPEC-TRADE-003` — A fixed per-kind rouble amount is settled exactly once at physical clearance,
  paired with the corresponding custody transition. Order placement, market matching, reservation,
  and route assignment do not settle roubles.
- `SPEC-TRADE-004` — A missing customs endpoint, vehicle, route, or compatible stock produces an
  observable pending order or recoverable failure; it MUST NOT teleport imports or destroy exports.
- `SPEC-TRADE-005` — Medicine import requires the same request, allocation, reservation, pickup,
  custody, delivery, and consumption distinctions as any other cargo. Water cannot be a trade
  cargo.

## Model and state

Trade owns order, customs-clearance, and rouble-settlement state. It references Logistics haul and
custody IDs rather than copying domestic transfer state; Logistics remains the sole authority for
domestic allocation, reservation, pickup, custody, and delivery. A trade order records direction,
item, quantity, fixed price, customs office, foreign consignment or domestic logistics job,
clearance event, and single settlement record. Imports follow order → foreign consignment reaches
customs → clearance plus single settlement establishes customs custody → domestic reservation →
pickup → domestic haul/delivery → consumption. Exports follow domestic request/allocation →
pickup/custody → domestic haul arrives at customs → clearance plus single settlement → leaving
domestic custody. A failed order retains its goods or demand with an accountable recovery path.

## Failure behavior

Trade capacity is bounded by real customs offices and domestic movement. If clearance cannot occur,
the order remains pending with its reason. No failure deletes stock or creates roubles; no domestic
shortage is solved by instant foreign stock.

## Observability

The Planner can inspect order direction, item, fixed price, quantity, customs office, custody,
clearance state, settlement record, age, and failure reason. Domestic stock and rouble balances
are visibly separate.

## Acceptance evidence

Tests must prove imports do not credit stock before clearance, exports do not debit stock before
clearance, settlement occurs once, and a failed route preserves stock/order. A mutation that settles
at match time or mixes domestic money must fail. Player-facing proof must show an order waiting at
customs.

## Substrate and decisions

Current imports credit buyer stock immediately and exports can debit seller stock before an external
endpoint exists ([`ECO-SUB-002`](../../research/fact-sheets/wave1-economy.md#eco-sub-002--border-trade-teleports-or-destroys-stock)); this is a violation, not a target feature. The
inherited treasury also prices domestic actions
([`ECO-SUB-004`](../../research/fact-sheets/wave1-economy.md#eco-sub-004--the-inherited-treasury-still-prices-domestic-actions)), which conflicts with the charter.
Freight stations currently retain unitless counters rather than embodied cargo (economy fact-sheet,
Trade). No runtime source cited here proves physical customs clearance.

## Deferred behavior

Dynamic price formation, exchange, credit, loans, bloc currencies, resale, and catalogue-era
systems are outside this 1.0 mechanism.

## Open questions

- What exact fixed rouble price table applies to the sixteen charter resources?
- Does clearance occur at a customs inventory boundary, a vehicle crossing, or both as one atomic
  event?
