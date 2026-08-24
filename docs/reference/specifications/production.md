# Production specification

**Kind:** specification
**Authority:** binding
**Status:** draft
**Owner:** economy
**Last verified:** 2026-08-24

The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT, RECOMMENDED, NOT
RECOMMENDED, MAY, and OPTIONAL in this document are to be interpreted as described in RFC 2119 and
RFC 8174.

## Purpose

Production transforms physically received inputs into outputs under material, labour, power, water,
capacity, and storage constraints. It makes the charter's resources-and-production commitment
precise without claiming that current integer recipe execution is a complete model.

## Scope and exclusions

This covers the charter's domestic resource tree and twelve new recipe buildings. It excludes
vehicle manufacture and vehicle fuel lifecycle, which are explicitly Post-1.0. Water is a utility
gate, never cargo; Medicine is import-only and has no domestic recipe. Archived CS1 and W&R
examples are comparison evidence only.

## Invariants

- `SPEC-PRODUCTION-001` — A production run MUST consume only inputs already delivered into the
  producer's accountable stock; an allocation, reservation, or planned receipt is insufficient.
- `SPEC-PRODUCTION-002` — Output is bounded by the declared recipe, available input, labour,
  power, water, process capacity, and output storage. The binding constraint and its quantity
  SHALL be recorded for an incomplete run.
- `SPEC-PRODUCTION-003` — Requested, received, consumed, on-hand, reserved, in-custody, and
  surplus quantities are distinct. A reported request is never proof of consumption.
- `SPEC-PRODUCTION-004` — Production MUST NOT debit domestic money or use domestic price clearing
  as a gate. Border rouble settlement belongs only to physical customs clearance.
- `SPEC-PRODUCTION-005` — A blocked producer retains its unmet input request or records its
  substitute/going-without result; it MUST NOT silently delete demand, inputs, or outputs.

## Model and state

Production owns recipe execution and the consumption/production transitions after delivery. A
recipe names inputs, outputs, optional byproducts, and capacity; a production record names the
run, consumed quantities, produced quantities, labour and utility availability, and the binding
constraint. Logistics remains authoritative for allocation through delivery and is referenced by haul
ID; Trade remains authoritative for border clearance and settlement. Input flow is request →
allocation → reservation → pickup → custody → delivery → on-hand → consumption. Output flow begins
at production, remains in producer custody until a separate logistics delivery, and never becomes
another holder's stock at match time.

## Failure behavior

Missing labour, power, water, input, or output space blocks or limits a run with the limiting
reason visible. Failed fulfillment leaves a recoverable queue. A producer may not manufacture
credit, replace unavailable inputs invisibly, or destroy surplus to make accounting balance.

## Observability

The Planner can inspect every recipe's inputs, outputs, received and consumed amounts, storage
state, active binding constraint, outstanding request age, and declared surplus. This makes an
enterprise's reported need distinguishable from its actual consumption.

## Acceptance evidence

All listed guards are **UNIMPLEMENTED** and block ratification. A command that executes zero tests
is failure, never green. The current 26-test suite proves no target below.

| Evidence | Command | Observable assertion | Required red mutation | Player-facing proof |
|---|---|---|---|---|
| `EVID-PRODUCTION-001` | `cargo test -p simulation evid_production_delivered_input_conservation -- --test-threads=1` | Undelivered input blocks output; each completed run conserves declared inputs and outputs. | Consume reserved-but-undelivered input or create output without debiting input. | Inspected binding-constraint and shortage capture. |
| `EVID-PRODUCTION-002` | `cargo test -p simulation evid_production_request_vs_consumption -- --test-threads=1` | Reported request remains distinguishable from received and consumed quantity. | Set consumed equal to requested without a delivery/run. | Inspected dishonest-enterprise quantity capture. |

## Substrate and decisions

Current recipes atomically consume and produce integer capital and gate on inputs, output threshold,
staffing, and some blackout state ([economy fact-sheet, Production](../../research/fact-sheets/wave1-economy.md#domain-rulings)).
The useful atomic transformation seam is partial only: current state does not distinguish receipt,
consumption, or surplus. `set_requested` is test-only and required dishonest-enterprise quantities
are not visible in gameplay ([`ECO-SUB-005`](../../research/fact-sheets/wave1-economy.md#eco-sub-005--dishonest-enterprise-behavior-is-test-only)).

## Deferred behavior

Detailed machinery condition, quality grades, perishability, and utility-network mechanisms are
deferred. They cannot be inferred from archived reference games.

## Open questions

- What recipe schema and acceptance threshold define the twelve new recipe buildings?
- Which utility shortfalls permit partial rate and which require a hard stop?
