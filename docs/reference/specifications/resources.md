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

Water is a network utility, never vehicle cargo. Medicine is import-only: no 1.0 domestic recipe
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
  or a trade haul. Network utility specifications own its production and distribution.
- `SPEC-RESOURCES-004` — Medicine MAY enter domestic stock only after physical import clearance;
  it MUST NOT be a domestic production output in 1.0.
- `SPEC-RESOURCES-005` — A quantity cannot be silently deleted or created during a failed request,
  reservation, transfer, or consumption. Unmet demand remains an observable queue, substitutes,
  or a going-without outcome.

## Model and state

Resources owns catalogue membership, compatibility, and stock invariants. Each catalogue entry
requires an identity, unit, permitted storage/handling classes, and whether it is domestic or
import-only; a stock record requires quantity and accountable holder. Logistics is the sole
authority for domestic transfer transitions and custody, while Trade is the sole authority for
customs clearance and settlement. Resource records reference their haul or trade-order IDs rather
than duplicate their state. These are target records, not a claim that the current substrate stores
them.

## Failure behavior

An incompatible store or carrier refuses the transfer with its reason intact. Insufficient stock
creates a partially fulfilled or waiting request; it does not erase demand. A cancelled or stalled
transfer releases reservation and custody back to an accountable holder, preserving both goods and
the request for recovery.

## Observability

The Planner can inspect item identity, on-hand, reserved, in-custody, delivered, and consumed
quantities, plus the age and reason of an unmet request. This is required to make shortage and the
dishonest-enterprise loop legible.

## Acceptance evidence

Evidence must exercise a compatible transfer and reject an incompatible one; prove that a partial
allocation preserves the remainder; and prove Water cannot be made cargo and Medicine cannot be
produced domestically. A conservation mutation must fail when an item is duplicated or deleted.

## Substrate and decisions

Current substrate differs materially: Lua has 21 item identities but no unit, mass, volume,
storage class, transport class, or capacity metadata ([economy fact-sheet, Resources](../../research/fact-sheets/wave1-economy.md#domain-rulings)).
Current Market inventory is an integer capital counter, and its unmatched demand can disappear
([`ECO-SUB-001`](../../research/fact-sheets/wave1-economy.md#eco-sub-001--unmatched-demand-is-not-a-durable-queue)).
This specification does not promote either behavior.

## Deferred behavior

Perishability, refrigerated transport, containers, and fuel lifecycle are not 1.0 mechanisms.
Detailed utility flows are deferred to their utility specifications.

## Open questions

- Which named fifteen resources satisfy the charter catalogue, and what are their units and
  compatible handling classes?
- Which substitutions are legal for each dwelling need and production input?
