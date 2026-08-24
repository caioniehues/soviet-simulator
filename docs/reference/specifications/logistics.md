# Logistics specification

**Kind:** specification
**Authority:** binding
**Status:** draft
**Owner:** logistics
**Last verified:** 2026-08-24

The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT, RECOMMENDED, NOT
RECOMMENDED, MAY, and OPTIONAL in this document are to be interpreted as described in RFC 2119 and
RFC 8174.

## Purpose

Logistics assigns finite vehicle identities to physical hauls and preserves accountable quantity
and custody from pickup through delivery or release. It implements the charter's no-teleport pillar
and failure-as-queue rule.

## Scope and exclusions

This specification covers domestic freight and the domestic legs of border trade. Water is never
cargo. Finite compatible cargo capacity and physical parking/depot recovery are 1.0 target
mechanics, though they are not currently provided. Driver assignment, vehicle fuel lifecycle, and
vehicle manufacture are outside this mechanism; fuel lifecycle and manufacture are Post-1.0. CS1
and W&R material is comparison evidence only.

## Invariants

- `SPEC-LOGISTICS-001` — A haul has one authoritative fulfillment authority. It references the
  requesting subsystem's durable demand and the consuming subsystem's consumption ID, and records
  allocation, reservation, pickup, in-transit custody, delivery, physical return, or release;
  it does not own consumed state or quantity.
- `SPEC-LOGISTICS-002` — Allocation and vehicle reservation do not transfer stock. Pickup changes
  custody from source to a named in-transit record; delivery changes custody to destination; only
  a consuming process may consume delivered stock.
- `SPEC-LOGISTICS-003` — A vehicle identity is finite and may hold only cargo compatible with its
  declared capacity. Logistics SHALL own cargo identity, quantity, and custody; Vehicle fleet state
  SHALL own capacity, owner/depot assignment, recovery state, and a reference to Roads-owned
  parking. A freight job cannot exceed the referenced vehicle or wagon capacity.
- `SPEC-LOGISTICS-004` — Missing truck, route, source stock, or destination capacity MUST create
  an observable stalled or waiting job with a recoverable reason. It MUST NOT delete demand,
  stock, reservation, or vehicle identity.
- `SPEC-LOGISTICS-005` — Domestic dispatch and fulfillment use no money or price priority. Border
  roubles settle only at the separate physical clearance event defined by trade.
- `SPEC-LOGISTICS-006` — Reservation is a non-additive encumbrance. For pickup quantity `x`, the
  atomic transition is `H_source -= x`, `R_source -= x`, `C_haul += x`; pre-pickup cancellation
  changes only `R_source`, while post-pickup cancellation preserves `C_haul` until physical return,
  reassignment, or delivery.
- `SPEC-LOGISTICS-007` — Roads solely owns parking-slot reservations. Vehicles references a slot;
  Logistics requests vehicle recovery and waits for Roads acknowledgement, never instant parking
  or teleport.
- `SPEC-LOGISTICS-008` — After reservation, the referenced vehicle identity SHALL traverse an
  ordered compatible itinerary to the source before pickup. The same identity, retaining its haul
  reference and custody, SHALL traverse a compatible itinerary to the destination before delivery.
  Elapsed ticks, reservation, or route creation alone MUST NOT advance pickup or delivery.
- `SPEC-LOGISTICS-009` — Cancellation while vehicle-reserved and before pickup SHALL release only
  the source-goods reservation, cancel outstanding itinerary references, release the vehicle
  assignment, and mark the haul cancelled. The physical vehicle remains at its actual location;
  reassignment or recovery starts there and MUST NOT teleport it to source, destination, or depot.
- `SPEC-LOGISTICS-010` — Planner target-stock minima and maxima per storage bucket create demand
  below minimum and supply above maximum. Compatible demands are ordered by greatest normalized
  target deficit first, then meaningful route distance and a stable identity tie-break; money and
  price never participate.
- `SPEC-LOGISTICS-011` — Source and destination docks have finite loading and unloading rate
  budgets. Pickup or delivery per tick is bounded by compatible vehicle capacity, available stock
  or space, `rate * elapsed_time`, and available dock power. Missing, occupied, unpowered, or
  zero-rate docks produce visible wait, and partial progress changes custody only by the physically
  transferred quantity.

## Model and state

A haul is the sole authority for domestic allocation, reservation, pickup, in-transit custody,
delivery, physical return, and release. It references the requesting subsystem's demand ID and the
consuming subsystem's consumption ID, Vehicle fleet state, and Pathfinding route state by ID rather
than copying their authority. A haul contains source/destination, item, quantity, capacity
requirement, vehicle reference, pickup custody, delivery state, attempts, age, and recovery reason.
Its main state is demand-referenced → allocated → vehicle-reserved → pickup → in-custody →
delivered → released; a post-pickup cancellation branches through physical return or reassigned
delivery before release. Consumption is a separate Production or Needs transition. Completion
requests Vehicle recovery, then waits for Roads to acknowledge a parking-slot reservation.
Pre-pickup cancellation releases only reservation; post-pickup cancellation retains custody until
physical return, reassignment, or delivery. If a vehicle was already reserved, pre-pickup
cancellation also cancels its itinerary and releases it at its current physical location. Storage
policy derives deficit/surplus from Resources balances, and dispatch records source and destination
dock progress under their finite transfer rates.

## Failure behavior

The dispatcher queues jobs under finite fleet, compatible capacity, parking, and route scarcity.
It can retry, reassign, cancel, or return custody according to a recorded policy; each path leaves
a Planner-visible reason. Cancellation is never silent deletion. Shortage remains in the request
queue until a permitted substitution or going-without decision is recorded.

## Observability

The Planner can inspect a haul's source, destination, item, quantity, vehicle identity, cargo
capacity, compatibility, owner/depot, parking, custody, age, reservation, route status, and
recovery reason, plus the queue it blocks.

## Acceptance evidence

All listed guards are **UNIMPLEMENTED** and block ratification. A command that executes zero tests
is failure, never green. The current 26-test suite proves no target below.

| Evidence | Command | Observable assertion | Required red mutation | Player-facing proof |
|---|---|---|---|---|
| `EVID-LOGISTICS-001` | `cargo test -p simulation evid_logistics_pickup_delivery_cancel_conservation -- --test-threads=1` | Pickup atomically applies `H_source -= x`, `R_source -= x`, `C_haul += x`; delivery/physical return conserve quantity and consumption remains external to the haul. | Credit destination at pickup, consume through the haul, or omit custody return after post-pickup cancellation. | Inspected haul/custody inspector capture. |
| `EVID-LOGISTICS-002` | `cargo test -p simulation evid_logistics_no_truck_no_route_recovery -- --test-threads=1` | Missing truck or route yields a visible recoverable stalled job without loss. | Remove the stalled state or discard request/reservation after retry. | Inspected stalled-haul session. |
| `EVID-LOGISTICS-003` | `cargo test -p simulation evid_logistics_same_vehicle_arrival_gates_transfer -- --test-threads=1` | One assigned vehicle reaches source before pickup and the same identity reaches destination before delivery over compatible routes. | Complete a leg after a fixed timer, substitute another vehicle, or permit an incompatible leg. | Inspected vehicle, route-progress, pickup, and delivery timeline. |
| `EVID-LOGISTICS-004` | `cargo test -p simulation evid_logistics_pre_pickup_cancel_releases_reservations_without_teleport -- --test-threads=1` | Cancellation releases goods reservation and vehicle assignment, cancels itinerary, leaves stock/custody unchanged, and preserves actual vehicle location. | Retain an itinerary/reservation, debit stock, or reset the vehicle to its depot. | Inspected cancellation and physical-recovery session. |
| `EVID-LOGISTICS-005` | `cargo test -p simulation evid_logistics_target_deficit_distance_priority -- --test-threads=1` | Target policy derives demand/supply; larger deficit wins at equal distance, and nearer demand wins at equal deficit. | Rank equal-distance jobs without deficit or ignore distance for equal deficits. | Inspected policy, candidate inputs, and dispatch order. |
| `EVID-LOGISTICS-006` | `cargo test -p simulation evid_logistics_rate_limited_dock_transfer -- --test-threads=1` | Compatible powered docks cap loading/unloading; zero rate or no power makes no progress and partial transfer conserves custody. | Transfer all cargo in one tick, bypass a zero-rate or unpowered dock, or credit more than the partial debit. | Inspected dock occupancy, power, rate budget, and partial custody capture. |

## Substrate and decisions

Current market dispatch proves a truck can route to source and destination, with source debit and
destination credit at the endpoints (`simulation/src/economy/market.rs:462-599`; [`LOG-SUB-002`](../../research/fact-sheets/wave1-logistics.md#log-sub-002--market-dispatch-drives-a-truck-over-routed-itineraries)).
It does not embody cargo or capacity in `Vehicle` (`simulation/src/transportation/vehicle.rs:34-45`; [`LOG-SUB-005`](../../research/fact-sheets/wave1-logistics.md#log-sub-005--cargo-is-not-embodied-by-the-vehicle)).
Company-held truck IDs are ignored by global dispatch ([`LOG-SUB-006`](../../research/fact-sheets/wave1-logistics.md#log-sub-006--company-ownership-does-not-constrain-global-dispatch)),
and company-driver delivery and market dispatch are competing current authorities
(`simulation/src/souls/goods_company.rs:127-148,235-270`; `simulation/src/map_dynamic/dispatch.rs:95-104`; `simulation/src/economy/market.rs:382-393`; [`LOG-SUB-006`](../../research/fact-sheets/wave1-logistics.md#log-sub-006--company-ownership-does-not-constrain-global-dispatch), [`LOG-SUB-007`](../../research/fact-sheets/wave1-logistics.md#log-sub-007--old-company-delivery-and-new-market-freight-both-remain-live)); both are substrate debt, not target architecture.
Completion has no return-to-depot recovery (`simulation/src/economy/market.rs:583-599`; [`LOG-SUB-008`](../../research/fact-sheets/wave1-logistics.md#log-sub-008--completion-releases-a-truck-without-parking-it)) and failure retries without terminal recovery
(`simulation/src/economy/market.rs:494-527,562-609`; [`LOG-SUB-009`](../../research/fact-sheets/wave1-logistics.md#log-sub-009--dispatch-failure-has-no-terminal-recovery-policy)).

## Deferred behavior

Fuel, wear, manufacture, and detailed driver labour are excluded from 1.0. The in-scope freight
rail mechanism must embody cargo and capacity under the same custody contract; its detailed
station and consist design remains an open implementation question.

## Open questions

- What normalized deficit scale preserves the required deficit-first ordering across resource kinds?
- What return-to-depot and reassignment policy applies after delivery or a stalled route?
