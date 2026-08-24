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
- `SPEC-PRODUCTION-006` — Atomic consumption asks Resources to debit compatible delivered on-hand
  stock exactly once and records the consumption ID in the same commit; reservation, custody, or
  delivery alone cannot consume stock.
- `SPEC-PRODUCTION-007` — One idempotent recipe-run transaction keyed by run ID SHALL atomically
  ask Resources to debit every declared input and credit every declared output/byproduct. If any
  gate or stock mutation fails, none commit; a retry of the same run ID MUST NOT reapply.
- `SPEC-PRODUCTION-008` — An enterprise whose reported plan fulfillment or physical production is
  below plan MUST remain an observable operating institution and eligible for ordinary physical
  allocation. Shortfall MUST NOT liquidate or deactivate it, waive an ordinary Logistics rule,
  conjure stock, or conceal its unmet request, binding constraint, or shortage.
- `SPEC-PRODUCTION-009` — For each input and accounting period, an enterprise MAY report a
  requirement above the recipe's actual consumption. Logistics allocates by its ordinary shortage
  rules, not an honesty label; receipts become physical on-hand stock through Resources and excess
  after consumption remains accountable surplus, so an honest competitor MAY wait. The Planner
  SHALL infer suspected deception from inspectable request, receipt, consumption, on-hand,
  surplus, and outstanding-request-age discrepancies; no authoritative `dishonest` flag may
  replace those observations.

## Model and state

Production owns recipe execution plus the consumption and production transitions after delivery.
Resources owns on-hand balances; Production invokes its stock-debit interface in the same atomic
recipe-run transaction rather than copying balances. The transaction is keyed by run ID and debits
every declared input while crediting every declared output/byproduct, or commits none. When it
requests input, Production owns that durable demand. A recipe names inputs, outputs, optional
byproducts, and capacity; a production record names the run, consumption ID, consumed quantities,
produced quantities, labour and utility availability, and the binding constraint. Logistics remains
authoritative for allocation through delivery and is referenced by haul ID; Trade remains
authoritative for border clearance and settlement. Input flow is request →
allocation → reservation → pickup → custody → delivery → on-hand → consumption. Output flow begins
at production, remains in producer custody until a separate logistics delivery, and never becomes
another holder's stock at match time.

## Failure behavior

Missing labour, power, water, input, or output space blocks or limits a run with the limiting
reason visible. Failed fulfillment leaves a recoverable queue. A producer may not manufacture
credit, replace unavailable inputs invisibly, or destroy surplus to make accounting balance.
Plan shortfall leaves the enterprise operating, physically constrained, allocation-eligible, and
inspectable; it neither dissolves the institution nor bails it out with stock.

## Observability

The Planner can inspect every recipe's inputs, outputs, received and consumed amounts, storage
state, active binding constraint, outstanding request age, and declared surplus. This makes an
enterprise's reported need distinguishable from its actual consumption.

## Acceptance evidence

All listed guards are **UNIMPLEMENTED** and block ratification. A command that executes zero tests
is failure, never green. The current 26-test suite proves no target below.

| Evidence | Command | Observable assertion | Required red mutation | Player-facing proof |
|---|---|---|---|---|
| `EVID-PRODUCTION-001` | `cargo test -p simulation evid_production_delivered_input_conservation -- --test-threads=1` | Undelivered input blocks output; one consumption ID debits compatible on-hand input exactly once and each completed run conserves declared inputs and outputs. | Consume reserved-but-undelivered input, consume the same ID twice, or create output without debiting input. | Inspected binding-constraint and shortage capture. |
| `EVID-PRODUCTION-002` | `cargo test -p simulation evid_production_request_vs_consumption -- --test-threads=1` | Reported request remains distinguishable from received and consumed quantity. | Set consumed equal to requested without a delivery/run. | Inspected dishonest-enterprise quantity capture. |
| `EVID-PRODUCTION-003` | `cargo test -p simulation evid_production_run_id_atomicity -- --test-threads=1` | An interrupted/failed run leaves every input, output, and byproduct balance unchanged; retry of one run ID applies once. | Split input debit from output credit or reapply the same run ID. | Inspected recipe-run and binding-constraint capture. |
| `EVID-PRODUCTION-004` | `cargo test -p simulation evid_production_soft_budget_shortfall -- --test-threads=1` | A physically input-starved, below-plan enterprise remains registered, inspectable, and eligible for normal allocation; shortfall grants neither stock nor an allocation bypass. | Deactivate/liquidate it on shortfall, or credit/reserve input solely because of shortfall. | Inspected shortfall, queue, and eligibility capture. |
| `EVID-PRODUCTION-005` | `cargo test -p simulation evid_production_dishonest_enterprise_inference -- --test-threads=1` | Under constrained supply, an inflated request can receive and retain surplus after consumption while an honest competitor waits or partially fills; inspection exposes same-period discrepancies without a dishonest flag. | Clamp request to recipe amount, discard excess, fully supply both from one quantity, or replace comparison with an honesty flag. | Inspected two-enterprise discrepancy capture. |

## Substrate and decisions

Current recipes atomically consume and produce integer capital and gate on inputs, output threshold,
staffing, and some blackout state (`simulation/src/souls/goods_company.rs:21-64,77-110`;
[economy fact-sheet, Production](../../research/fact-sheets/wave1-economy.md#domain-rulings)).
The useful atomic transformation seam is partial only: current state does not distinguish receipt,
consumption, or surplus. `set_requested` is test-only and required dishonest-enterprise quantities
are not visible in gameplay (`simulation/src/economy/market.rs:240-249`;
`native_app/src/gui/inspect/inspect_building.rs:244-299`; [`ECO-SUB-005`](../../research/fact-sheets/wave1-economy.md#eco-sub-005--dishonest-enterprise-behavior-is-test-only)).

## Deferred behavior

Detailed machinery condition, quality grades, perishability, and utility-network mechanisms are
deferred. They cannot be inferred from archived reference games.

## Open questions

- What recipe schema and acceptance threshold define the twelve new recipe buildings?
- Which utility shortfalls permit partial rate and which require a hard stop?
