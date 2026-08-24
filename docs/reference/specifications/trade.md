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
one border rouble. Medicine is import-only; Water is never cargo and crosses only through a
physical metered border utility connection. Dual currency, loans, dynamic
markets, ships, docks, pipelines, cableways, containers, aircraft, vehicle manufacture, and fuel
lifecycle are not 1.0 mechanisms. Archived CS1 and W&R trade material is comparison evidence only.

## Invariants

- `SPEC-TRADE-001` — Domestic matching, allocation, reservation, dispatch, production, and
  consumption MUST NOT debit, credit, rank by, or otherwise clear through roubles.
- `SPEC-TRADE-002` — A trade order has a physical item, quantity, direction, customs office,
  tagged transport reference, and clearance state. A non-Water order references a domestic haul;
  a Water order references its utility transfer. Import stock appears only after inbound physical clearance;
  an export source holder MAY debit stock to accountable in-transit domestic custody at pickup, but
  total domestic custody leaves only after outbound physical clearance.
- `SPEC-TRADE-003` — A fixed per-kind rouble amount is settled exactly once at physical clearance,
  paired with the corresponding custody transition. Order placement, market matching, reservation,
  and route assignment do not settle roubles.
- `SPEC-TRADE-004` — A missing customs endpoint, vehicle, route, or compatible stock produces an
  observable pending order or recoverable failure; it MUST NOT teleport imports or destroy exports.
- `SPEC-TRADE-005` — Medicine import requires the same request, allocation, reservation, pickup,
  custody, delivery, and consumption distinctions as any other cargo. Water is never cargo, but
  MAY clear only through a physical metered border utility connection; Trade owns clearance and
  settlement while the future Water specification owns utility transport and meter implementation.
- `SPEC-TRADE-006` — Clearance is idempotent by order ID. At import clearance of quantity `q` at
  fixed price `p`: `foreign_stock -= q`, `customs_stock += q`, `Planner_roubles -= p*q`, and
  `foreign_counterparty_roubles += p*q`; export applies the exact inverse. The foreign
  stock/counterparty
  pair may be an external-ledger abstraction when the foreign world is not simulated. A failed or
  retried transaction changes none of these balances; insufficient roubles leaves import pending,
  never negative.
- `SPEC-TRADE-007` — Transport is tagged: a non-Water order references one Logistics haul, while
  a Water order references one Water-owned metered utility-transfer ID and MUST NOT have a freight
  haul. Water's completed transfer atomically changes foreign-network and domestic-network
  quantities by opposite signed `q` and increments the directional cumulative meter reading, which
  is not stock. Trade consumes that completed result to clear and apply the `SPEC-TRADE-006` signed
  rouble legs once under the order ID. Failed or retried transfer changes no balance. The future
  Water specification owns network/meter implementation; Trade owns clearance and settlement.
- `SPEC-TRADE-008` — A Water order SHALL clear only after its referenced Water-owned transfer
  reports completed quantity `q` through connected border utility infrastructure under a finite
  transfer-rate budget. Water owns connectivity, per-tick physical progress, capacity, meter
  mutation, and completion. Trade MUST NOT create network quantity, increment a meter, clear, or
  settle for a disconnected, zero-capacity, failed, or partial transfer; it consumes one completed
  transfer result exactly once.

## Model and state

Trade owns order, customs-clearance, and rouble-settlement state. A non-Water order references a
Logistics haul and custody IDs rather than copying domestic transfer state; Logistics remains the
sole authority for domestic allocation, reservation, pickup, custody, and delivery. A Water order
instead references a Water-owned metered utility-transfer ID and cannot reference a freight haul.
A trade order records direction, item, quantity, fixed price, customs office, tagged transport
reference, clearance event, single settlement record, and order-ID idempotency key. Non-Water
imports follow order → foreign consignment reaches customs → clearance plus single settlement
establishes customs custody → domestic reservation → pickup → domestic haul/delivery →
consumption. Non-Water exports follow domestic request/allocation → pickup/custody → domestic haul
arrives at customs → clearance plus single settlement → leaving domestic custody. Water import or
export first progresses under Water authority across connected infrastructure and finite capacity.
After Water reports completed opposite-signed network mutation and directional meter increment for
the full `q`, Trade clears and applies the signed rouble legs once under the order ID. A partial
transfer remains uncleared. A failed order retains its goods or demand with an accountable recovery
path.

## Failure behavior

Trade capacity is bounded by real customs offices and domestic movement. If clearance cannot occur,
the order remains pending with its reason. No failure deletes stock or creates roubles; no domestic
shortage is solved by instant foreign stock.

## Observability

The Planner can inspect order direction, item, fixed price, quantity, customs office, custody,
clearance state, settlement record, age, and failure reason. Domestic stock and rouble balances
are visibly separate.

## Acceptance evidence

All listed guards are **UNIMPLEMENTED** and block ratification. A command that executes zero tests
is failure, never green. The current 26-test suite proves no target below.

| Evidence | Command | Observable assertion | Required red mutation | Player-facing proof |
|---|---|---|---|---|
| `EVID-TRADE-001` | `cargo test -p simulation evid_trade_clearance_single_settlement -- --test-threads=1` | Import stock appears after clearance; export remains in total domestic custody until clearance; each order settles once. | Credit import or settle at order match, or let export leave domestic custody at pickup. | Inspected customs inspector capture. |
| `EVID-TRADE-002` | `cargo test -p simulation evid_trade_failure_preserves_order -- --test-threads=1` | Missing route, customs endpoint, or insufficient roubles preserves order and balances; insufficient roubles remains pending. | Debit export without accountable in-transit custody, discard pending order, or permit negative Planner roubles. | Inspected pending-customs session. |
| `EVID-TRADE-003` | `cargo test -p simulation evid_trade_idempotent_clearance_ledger -- --test-threads=1` | One order-ID-keyed clearance applies the signed `q`/`p*q` ledger equation exactly once; retry applies no second change. | Apply clearance twice on retry or reverse one export ledger sign. | Inspected customs ledger capture. |
| `EVID-TRADE-004` | `cargo test -p simulation evid_trade_water_metered_border_clearance -- --test-threads=1` | Water clearance references its utility-transfer ID, atomically changes opposite-signed foreign/domestic network quantities, increments only its directional cumulative meter, and has no freight haul. | Require a freight haul, or credit a domestic network without a meter increment and foreign-network debit. | Inspected border-utility meter capture. |
| `EVID-TRADE-005` | `cargo test -p simulation evid_trade_water_clearance_requires_completed_connected_transfer -- --test-threads=1` | Disconnected and zero-capacity transfers make no progress; finite rate permits conserved partial flow that remains uncleared; Water-owned completion of full quantity precedes one Trade clearance and settlement. | Clear on order age, advance a disconnected/zero-capacity path, exceed the rate, or settle before Water completion. | Inspected connection, partial-flow, meter, and clearance capture. |

## Substrate and decisions

Current imports credit buyer stock immediately and exports can debit seller stock before an external
endpoint exists (`simulation/src/economy/market.rs:396-451`;
[`ECO-SUB-002`](../../research/fact-sheets/wave1-economy.md#eco-sub-002--border-trade-teleports-or-destroys-stock));
this is a violation, not a target feature. The inherited treasury also prices domestic actions
(`simulation/src/economy/mod.rs:53-55`, `simulation/src/economy/government.rs:22-75`,
`simulation/src/world_command.rs:223-225`;
[`ECO-SUB-004`](../../research/fact-sheets/wave1-economy.md#eco-sub-004--the-inherited-treasury-still-prices-domestic-actions)),
which conflicts with the charter. Freight stations currently retain unitless counters rather than
embodied cargo (`simulation/src/souls/freight_station.rs:30-37`; [economy fact-sheet,
Trade](../../research/fact-sheets/wave1-economy.md#domain-rulings)). No runtime source cited here
proves physical customs clearance.

## Deferred behavior

Dynamic price formation, exchange, credit, loans, bloc currencies, resale, and catalogue-era
systems are outside this 1.0 mechanism.

## Open questions

- What exact fixed rouble price table applies to the sixteen charter resources?
- Does clearance occur at a customs inventory boundary, a vehicle crossing, or both as one atomic
  event?
