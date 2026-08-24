# Dwelling needs specification

**Kind:** specification
**Authority:** binding
**Status:** draft
**Owner:** settlement and economy
**Last verified:** 2026-08-24

The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT, RECOMMENDED, NOT
RECOMMENDED, MAY, and OPTIONAL in this document are to be interpreted as described in RFC 2119
and RFC 8174.

## Purpose

This specification makes the charter's separate Food and Meat dwelling needs, physical
provisioning, and going-without failure rule precise. It is in scope under the charter's
[resources and production commitment](../../plan/charter-1.0.md#10-scope) and its identity
pillars. It does not establish that the current fork provides this mechanism.

## Scope and exclusions

Needs turn household or citizen need state into non-monetary requests and record the outcome of
provisioning. Domestic money and price clearing are prohibited by the charter's binding model.
Tourism is Never; marketing, loyalty, legitimacy, crime, and perishability are Post-1.0 under the
[charter cut line](../../plan/charter-1.0.md#explicit-cuts). Education, healthcare, water,
heating, and household ownership need their own ratified specifications before this file can set
their mechanisms.

## Invariants

- `SPEC-NEEDS-001` Food and Meat are distinct dwelling needs. A record of one never satisfies or
  substitutes for the other unless a future ratified allocation policy explicitly says so.
- `SPEC-NEEDS-002` A need is satisfied only by an authoritative consumption event after the
  required physical stock has reached its permitted point of use. Match, reservation, assignment,
  payment, and route start MUST NOT satisfy it.
- `SPEC-NEEDS-003` Domestic need clearing MUST NOT debit, credit, rank, or reject by roubles or
  another domestic price. The rouble is a border-only currency.
- `SPEC-NEEDS-004` An unmet need persists with age and an explicit outcome: waiting, approved
  substitution where the need policy permits it, or going without. It MUST NOT disappear because
  a seller, vehicle, route, or service is unavailable.
- `SPEC-NEEDS-005` Going without degrades the affected need and is inspectable; it is never a
  game-over transition.
- `SPEC-NEEDS-006` Each dwelling-consumption event has a unique ID, reduces compatible Resources
  on-hand stock exactly once, and applies no more than its quantity across explicitly named need
  records. Reusing an event MUST NOT satisfy a second obligation; partial satisfaction preserves
  the remaining quantity. Satisfaction follows this transaction only.

## Model and state

Needs owns durable need demand, age, outcome, satisfaction, and the dwelling-consumption
transaction. Each need record has an owner, kind, required quantity or service interval, outcome,
age, and the identities of its logistics fulfillment and consumption event. Food and Meat retain
independent records and histories. Logistics alone owns allocation, reservation, pickup,
transport, delivery, and return. Resources alone owns compatible on-hand stock. The need
consumption transaction records its unique event ID, named obligations, applied quantity, and any
remaining quantity. One atomic commit either applies both the Resources stock debit and the bounded
need quantity, or applies neither; Needs MUST NOT duplicate stock or fulfillment state.

Allocation order is a policy question, not a market. A later ratified economy specification must
define priority, partial fill, and any permitted substitution before an implementation selects
among scarce requests.

## Failure behavior

A missing seller, stock, vehicle, route, worker, or service creates a reasoned waiting state; a
cancelled allocation or delivery follows its owning authority's release rule and leaves the need
eligible for recovery.
When policy permits no substitute or recovery has not yet succeeded, the result is going without.
No failure path may consume unavailable inventory, conjure stock, silently remove demand, or end
the plan.

## Observability

The Planner can inspect each dwelling's Food and Meat demand, age, selected outcome,
dwelling-consumption event ID, applied and remaining quantity, and the reason for any wait or
going-without state. It links to Logistics for allocation/reservation/delivery state and to
Resources for on-hand stock; aggregate shortage views may summarize but cannot replace the
authoritative per-owner state.

## Acceptance evidence

All guards below are **UNIMPLEMENTED** and block ratification. A command that executes zero tests
is failure, never green. The current 26-test suite proves no target below.

| Evidence | Future guard command and observable assertion | Negative mutation that must turn it red | Player-facing proof |
|---|---|---|---|
| `EVID-NEEDS-001` | `cargo test -p simulation spec_needs_food_meat_distinct -- --test-threads=1` — consuming Food leaves Meat unmet. | Make Food write the Meat satisfaction state. | Inspected dwelling Food/Meat view. |
| `EVID-NEEDS-002` | `cargo test -p simulation spec_needs_consumption_not_match -- --test-threads=1` — allocation/reservation/route start do not satisfy; named consumption after delivery does. | Mark the need satisfied when allocation matches. | Inspected delivery then consumption session. |
| `EVID-NEEDS-003` | `cargo test -p simulation spec_needs_unmet_persists -- --test-threads=1` — unavailable provision persists with age and visible going without. | Delete the unmet request on failed allocation. | Inspected dwelling wait, going-without, and recovery. |
| `EVID-NEEDS-004` | `cargo test -p simulation spec_needs_consumption_event_idempotent -- --test-threads=1` — one event debits compatible on-hand stock once, cannot satisfy a second named obligation, and preserves partial remainder. | Reuse one event ID or skip its stock reduction. | Inspected event ID, stock debit, and partial remainder view. |

## Substrate and decisions

`ECO-SUB-001` records that unmatched demand can currently disappear
(`simulation/src/economy/market.rs:396-405`). `ECO-SUB-003` records price-free domestic matching
without durable priority or partial fill (`simulation/src/economy/market.rs:274-314`), and
`ECO-SUB-006` records conflicting fulfillment timestamps (`simulation/src/economy/mod.rs:95-104`;
`simulation/src/economy/market.rs:382-393`). The current human path buys only bread and can update
`last_ate` on arrival without consuming inventory (`simulation/src/souls/desire/buyfood.rs:70-90`),
which violates this target; see the Needs row and `ECO-SUB-001` through `ECO-SUB-006` in the
[economy fact-sheet](../../research/fact-sheets/wave1-economy.md).
External CS1 and Workers & Resources material in archived legacy research is comparison evidence
only, never mechanism authority.

## Deferred behavior

Tourism is Never. Loyalty, legitimacy, crime, perishability, and marketing are Post-1.0; this
specification assigns them neither a 1.0 mechanism nor acceptance evidence. Domestic money is
prohibited by the binding model, rather than deferred.

## Open questions

- Which needs are household-owned versus individual-owned, while keeping Food and Meat distinct?
- What non-price allocation policy orders partial fulfillment under scarcity?
- Which substitutions are permitted, and how are their consequences represented without collapsing
  the named needs?
