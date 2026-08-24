# Resources specification

**Kind:** specification
**Authority:** binding
**Status:** draft
**Owner:** economy
**Last verified:** 2026-08-24

The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT, RECOMMENDED, NOT
RECOMMENDED, MAY, and OPTIONAL in this document are to be interpreted as described in RFC 2119 and
RFC 8174.

## Purpose

This specification makes the charter's physical resource catalogue actionable without treating the
current Lua item list as that catalogue. It applies to the fifteen-resource domestic tree, the
separate Food and Meat dwelling needs, and import-only Medicine. The binding scope is
[the 1.0 charter](../../plan/charter-1.0.md#10-scope) (Resources and production; Transport and
border).

## Scope and exclusions

Water is a network utility, never vehicle cargo; in 1.0 it trades only through a physical metered
border utility connection. Trade owns its clearance and settlement, while the future Water
specification owns utility transport and meter implementation. Medicine is import-only: no 1.0 domestic recipe
may produce it. Resource metadata and handling must support physical stock, but vehicle
manufacture and vehicle fuel lifecycle are Post-1.0 exclusions; this specification supplies no
acceptance criteria for either.

CS1 and Workers & Resources material retained in the archive is comparison evidence only. It
cannot define this catalogue, its quantities, transport classes, or runtime behavior.

## Invariants

- `SPEC-RESOURCES-001` — The ratified catalogue SHALL contain exactly the charter's fifteen
  domestic resources plus Medicine, and every resource identity SHALL have a declared unit and
  handling/storage compatibility before a recipe, storage, or haul can use it.
- `SPEC-RESOURCES-002` — Stock is an owned physical quantity. A request, allocation,
  reservation, pickup, custody, delivery, and consumption are distinct records; none is evidence
  that another occurred.
- `SPEC-RESOURCES-003` — Water MUST NOT enter cargo stock, vehicle custody, a freight station,
  or a trade haul. It MAY cross the border only through the physical metered utility connection;
  Trade owns clearance and settlement, and the future Water specification owns its transport and meter.
- `SPEC-RESOURCES-004` — Medicine MAY enter domestic stock only after physical import clearance;
  it MUST NOT be a domestic production output in 1.0.
- `SPEC-RESOURCES-005` — A quantity cannot be silently deleted or created during a failed request,
  reservation, transfer, or consumption. Unmet demand remains an observable queue, substitutes,
  or a going-without outcome.
- `SPEC-RESOURCES-006` — Pre-pickup cancellation releases only the non-additive reservation;
  post-pickup cancellation retains accountable in-transit custody until physical return,
  reassignment, or delivery.

## Model and state

Resources owns catalogue membership, compatibility, and on-hand stock balances. Each catalogue
entry requires an identity, unit, permitted storage/handling classes, and whether it is domestic or
import-only; a stock record requires quantity and accountable holder. The requesting subsystem owns
durable demand. Logistics is the sole authority for domestic fulfillment transitions and custody,
while Trade is the sole authority for customs clearance and settlement; Production or Needs owns
consumption state and atomically coordinates a Resources-owned on-hand balance mutation. Only
Resources mutates on-hand balance/debit records. Resource records reference haul, trade-order, and
consumption IDs rather than duplicate their state. These are target records, not a claim that the
current substrate stores them.

## Failure behavior

An incompatible store or carrier refuses the transfer with its reason intact. Insufficient stock
creates a partially fulfilled or waiting request; it does not erase demand. Pre-pickup cancellation
releases only its reservation. Post-pickup cancellation retains in-transit custody until a physical
return, reassignment, or delivery, preserving goods and demand for recovery.

## Observability

The Planner can inspect item identity, on-hand, reserved, in-custody, delivered, and consumed
quantities, plus the age and reason of an unmet request. This is required to make shortage and the
dishonest-enterprise loop legible.

## Acceptance evidence

All listed guards are **UNIMPLEMENTED** and block ratification. A command that executes zero tests
is failure, never green. The current 26-test suite proves no target below.

| Evidence | Command | Observable assertion | Required red mutation | Player-facing proof |
|---|---|---|---|---|
| `EVID-RESOURCES-001` | `cargo test -p simulation evid_resources_conservation_partial_cancel -- --test-threads=1` | Partial allocation preserves the remainder; pre-pickup cancellation releases only reservation and post-pickup cancellation preserves custody pending physical return. | Release in-transit custody at post-pickup cancellation or credit the destination twice. | Inspected shortage/custody inspector capture. |
| `EVID-RESOURCES-002` | `cargo test -p simulation evid_resources_water_medicine_restrictions -- --test-threads=1` | Water is rejected as cargo; a domestic Medicine recipe is rejected. | Permit Water in a haul or register Medicine as a domestic output. | Inspected rejection and catalogue capture. |
| `EVID-RESOURCES-003` | `cargo test -p simulation evid_resources_water_metered_border_only -- --test-threads=1` | Water crosses only through a physical metered border utility connection, never freight custody. | Settle Water without a meter reading or route it through a freight station. | Inspected border-utility meter capture. |

## Substrate and decisions

Current substrate differs materially: Lua has 21 item identities but no unit, mass, volume,
storage class, transport class, or capacity metadata (`base_mod/items.lua:1-108`,
`prototypes/src/prototypes/item.rs:6-25`; [economy fact-sheet,
Resources](../../research/fact-sheets/wave1-economy.md#domain-rulings)). Current Market inventory is
an integer capital counter (`simulation/src/economy/market.rs:35-49`), and its unmatched demand can
disappear (`simulation/src/economy/market.rs:399-405`;
[`ECO-SUB-001`](../../research/fact-sheets/wave1-economy.md#eco-sub-001--unmatched-demand-is-not-a-durable-queue)).
This specification does not promote either behavior.

## Deferred behavior

Perishability, refrigerated transport, containers, and fuel lifecycle are not 1.0 mechanisms.
Detailed utility flows are deferred to their utility specifications.

## Open questions

- Which named fifteen resources satisfy the charter catalogue, and what are their units and
  compatible handling classes?
- Which substitutions are legal for each dwelling need and production input?
