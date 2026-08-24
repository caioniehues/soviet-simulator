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

- `SPEC-LOGISTICS-001` — A haul has one authoritative fulfillment authority. It records request,
  allocation, reservation, pickup, custody, delivery, and consumption/release in that order.
- `SPEC-LOGISTICS-002` — Allocation and vehicle reservation do not transfer stock. Pickup changes
  custody from source to a named in-transit record; delivery changes custody to destination; only
  a consuming process may consume delivered stock.
- `SPEC-LOGISTICS-003` — A vehicle identity is finite and may hold only cargo compatible with its
  declared capacity. Logistics SHALL own cargo identity, quantity, and custody; Vehicle fleet state
  SHALL own capacity, owner/depot assignment, parking, and recovery. A freight job cannot exceed the
  referenced vehicle or wagon capacity.
- `SPEC-LOGISTICS-004` — Missing truck, route, source stock, or destination capacity MUST create
  an observable stalled or waiting job with a recoverable reason. It MUST NOT delete demand,
  stock, reservation, or vehicle identity.
- `SPEC-LOGISTICS-005` — Domestic dispatch and fulfillment use no money or price priority. Border
  roubles settle only at the separate physical clearance event defined by trade.

## Model and state

A haul is the sole authority for domestic allocation, reservation, pickup, custody, delivery, and
release. It references the requesting/consuming system, Vehicle fleet state, and Pathfinding route
state by ID rather than copying their authority. A haul contains source/destination, item, quantity,
capacity requirement, vehicle reference, pickup custody, delivery state, attempts, age, and recovery
reason. Its
state is requested → allocated → vehicle-reserved → pickup → in-custody → delivered → consumed or
released. Completion recovers the vehicle to an accountable physical parking/depot location;
cancellation returns any unconsumed quantity to an accountable holder, frees its vehicle
reservation, and recovers its parking/depot state.

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

Evidence must show source debit only at pickup, destination credit only at delivery, one active
authority, conservation over cancellation, and recoverable no-truck/no-route failures. A mutation
that duplicates, loses, or delivers without pickup must fail.

## Substrate and decisions

Current market dispatch proves a truck can route to source and destination, with source debit and
destination credit at the endpoints ([`LOG-SUB-002`](../../research/fact-sheets/wave1-logistics.md#log-sub-002--market-dispatch-drives-a-truck-over-routed-itineraries)).
It does not embody cargo or capacity in `Vehicle` ([`LOG-SUB-005`](../../research/fact-sheets/wave1-logistics.md#log-sub-005--cargo-is-not-embodied-by-the-vehicle)).
Company-held truck IDs are ignored by global dispatch ([`LOG-SUB-006`](../../research/fact-sheets/wave1-logistics.md#log-sub-006--company-ownership-does-not-constrain-global-dispatch)),
and company-driver delivery and market dispatch are competing current authorities
([`LOG-SUB-007`](../../research/fact-sheets/wave1-logistics.md#log-sub-007--old-company-delivery-and-new-market-freight-both-remain-live)); both are substrate debt, not target architecture.
Completion has no return-to-depot recovery ([`LOG-SUB-008`](../../research/fact-sheets/wave1-logistics.md#log-sub-008--completion-releases-a-truck-without-parking-it)) and failure retries without terminal recovery
([`LOG-SUB-009`](../../research/fact-sheets/wave1-logistics.md#log-sub-009--dispatch-failure-has-no-terminal-recovery-policy)).

## Deferred behavior

Fuel, wear, manufacture, and detailed driver labour are excluded from 1.0. The in-scope freight
rail mechanism must embody cargo and capacity under the same custody contract; its detailed
station and consist design remains an open implementation question.

## Open questions

- What priority rule orders competing domestic requests without prices?
- What return-to-depot and reassignment policy applies after delivery or a stalled route?
