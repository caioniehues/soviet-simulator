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

## Model and state

Each need record has an owner, kind, required quantity or service interval, outcome, age, and the
identities of the allocation request, logistics delivery, and consumption event that concern it.
Food and Meat retain independent records and histories. Allocation owns requested and reserved
quantities; logistics owns custody and receipt; consumption owns consumed quantity. Needs reads
those authorities to determine satisfaction and MUST NOT duplicate them in a parallel fulfillment
record.

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

The Planner can inspect each dwelling's Food and Meat state, age, selected outcome, and the reason
for any wait or going-without state. It links to the allocation authority for requested/reserved
quantity, the logistics authority for received quantity, and the consumption authority for
consumed quantity. Aggregate shortage views may summarize these records but cannot replace the
authoritative per-owner state.

## Acceptance evidence

Evidence must prove that Food cannot satisfy Meat, match-time cannot satisfy either need, an
unfulfilled request survives across an update, and going without remains visible without ending a
plan. A physical-delivery scenario must demonstrate stock transfer before consumption; a mutation
that removes the transfer or persistence guard must fail its test. Player-facing acceptance needs
an inspected view of a dwelling waiting and then recovering.

## Substrate and decisions

`ECO-SUB-001` records that unmatched demand can currently disappear. `ECO-SUB-003` records
price-free domestic matching without durable priority or partial fill, and `ECO-SUB-006` records
conflicting fulfillment timestamps. The current human path buys only bread and can update
`last_ate` on arrival without consuming inventory, which violates this target; see the Needs row
and `ECO-SUB-001` through `ECO-SUB-006` in the [economy fact-sheet](../../research/fact-sheets/wave1-economy.md).
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
