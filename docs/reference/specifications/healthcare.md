# Healthcare specification

**Kind:** specification
**Authority:** binding
**Status:** draft
**Owner:** settlement
**Last verified:** 2026-08-24

The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT, RECOMMENDED, NOT
RECOMMENDED, MAY, and OPTIONAL in this document are to be interpreted as described in RFC 2119
and RFC 8174.

## Purpose

Healthcare makes illness and care a physical, finite service chain: a persistent citizen waits
for capacity, reaches care, and receives treatment only when compatible Medicine has physically
entered domestic stock through the border chain. It does not claim that current code has a health
service.

## Scope and exclusions

Healthcare owns care request, triage queue, finite service capacity, treatment outcome, and
healthcare-specific observability. Citizens owns identity/lifecycle; Needs owns need outcomes;
Resources owns stock balances; Trade owns Medicine clearance; Logistics owns Medicine fulfillment
and cargo custody; movement owners own a citizen's route. Medicine is import-only in 1.0. Ambulance
fuel lifecycle, vehicle manufacture, epidemics, deathcare, insurance, fees, and domestic price
clearing are excluded.

## Invariants

- `SPEC-HEALTHCARE-001` — A care request has an ID, citizen, reason, age, priority, queue state,
  and named treatment outcome. The request persists through missing capacity, Medicine, staff, or
  route; it MUST NOT be silently deleted or turn into game over.
- `SPEC-HEALTHCARE-002` — Healthcare is the sole authority for triage, service-capacity
  reservation, treatment completion, and health-care outcome. It references Citizen, Resources,
  Trade, Logistics, and movement results without duplicating their state.
- `SPEC-HEALTHCARE-003` — Treatment requiring Medicine consumes only compatible on-hand Medicine
  after its physical import clearance and delivery. Resources alone performs the on-hand mutation;
  request, order, clearance intent, reservation, or route start MUST NOT cure a citizen or consume
  Medicine.
- `SPEC-HEALTHCARE-004` — Care requires finite declared staffed capacity and physical arrival or
  an explicitly ratified service-delivery interface. No timer, coverage radius, or payment may
  stand in for capacity and treatment.
- `SPEC-HEALTHCARE-005` — Scarce care is ordered by explicit health/triage policy, never domestic
  roubles. Partial capacity leaves the remaining queue and its age/reason visible.
- `SPEC-HEALTHCARE-006` — Each Medicine treatment is keyed to one `CareRequestID` and accepts one
  `ConsumptionID` at most once. A `ConsumptionID` binds to no more than that one treatment;
  Resources debits compatible Medicine exactly once for its stated quantity and retries are
  no-ops. One Medicine consumption MUST NOT cure a second request or apply a second treatment.

## Model and state

Healthcare owns care request and triage records, service capacity/queue, arrival acceptance,
treatment event, and health outcome. A treatment event references a citizen, facility, capacity
slot, and when needed the Resources-owned Medicine consumption event plus its Trade/Logistics
provenance. Its immutable key is `CareRequestID`; its `ConsumptionID`, when required, is accepted
once and is not reusable by another treatment. Resources is the sole stock-balance mutator; Trade
and Logistics retain customs and physical transfer state; Citizens retains identity; no health
record copies their transitions.

## Failure behavior

No staff, service capacity, physical route, or Medicine leaves the request queued or waiting with
the binding reason. An interrupted delivery or cancelled reservation follows the owning module's
physical recovery rules and does not cure the citizen. Failure worsens observable health or
participation under future policy; it never fabricates Medicine, deletes a citizen, or ends the
plan.

## Observability

The Planner can inspect care request ID, citizen, triage reason/age, capacity queue, arrival,
treatment outcome, Medicine consumption event, and the physical import/delivery links where
Medicine was required. Aggregate service coverage cannot replace the causal per-request record.

## Acceptance evidence

All guards are **UNIMPLEMENTED** and block ratification. A zero-test command is failure. The
current 26-test suite proves no target below.

| Evidence | Future guard command and observable assertion | Required red mutation | Player-facing proof |
|---|---|---|---|
| `EVID-HEALTHCARE-001` | `cargo test -p simulation evid_healthcare_finite_queue_persists -- --test-threads=1` — capacity exhaustion retains an aged care queue and citizen identity. | Delete the request/citizen or complete treatment over capacity. | Inspected care queue capture. |
| `EVID-HEALTHCARE-002` | `cargo test -p simulation evid_healthcare_medicine_physical_chain -- --test-threads=1` — Medicine treatment follows import clearance and delivery; one `ConsumptionID` binds once to one `CareRequestID`/treatment and causes one compatible Resources debit. Retry is a no-op; no ConsumptionID can cure two requests. | Cure at import order/reservation, omit or repeat the Resources debit, reuse one ConsumptionID for a second CareRequestID, or reapply treatment on retry. | Inspected CareRequestID, ConsumptionID, import/delivery provenance, one debit, and treatment capture. |
| `EVID-HEALTHCARE-003` | `cargo test -p simulation evid_healthcare_nonprice_arrival_required -- --test-threads=1` — triage is non-price and treatment requires capacity plus arrival. | Rank by roubles or complete on timer/coverage alone. | Inspected arrival, triage, and treatment capture. |
| `EVID-HEALTHCARE-004` | `cargo test -p simulation evid_healthcare_references_not_copies -- --test-threads=1` — a care request/treatment holds references to the authoritative Citizen, Resources Medicine event, Trade clearance, Logistics fulfillment, and movement result; Healthcare has no independently mutable copy of any of those transitions. | Add a Healthcare-owned copy and mutate Citizen identity, Medicine balance/consumption, clearance, fulfillment, or route independently. | Inspected treatment provenance and authoritative-state links. |

## Substrate and decisions

No current healthcare type, capacity queue, or health decision is present: `BuildingKind` has no
healthcare variant (`simulation/src/map/objects/building.rs:17-24`) and human decision selection
only covers home, work, and food (`simulation/src/souls/human.rs:127-230`). Medicine is a target
import-only resource, not current service evidence; the existing human food flow requests bread
and updates `last_ate` at arrival without an authoritative inventory consumption
(`simulation/src/souls/desire/buyfood.rs:70-90`). This violation cannot be reused for medicine.
See the [Wave 2 fact-sheet](../../research/fact-sheets/wave2-substrate.md#2b--settlement-citizens-households-and-services).

## Deferred behavior

Ambulance fuel lifecycle and vehicle manufacture, epidemics, deathcare, insurance, fees,
private-price access, and coverage-radius cure systems have no 1.0 mechanism or acceptance
evidence here. The charter's vehicle and epidemic cuts remain binding.

## Open questions

- Which healthcare facility types and treatment capacities are in 1.0?
- Which health outcomes affect citizen participation without giving Healthcare ownership of
Citizen lifecycle?
- Which care cases require Medicine, and what compatible quantity is consumed per treatment?
