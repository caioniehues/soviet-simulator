# Education specification

**Kind:** specification
**Authority:** binding
**Status:** draft
**Owner:** settlement
**Last verified:** 2026-08-24

The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT, RECOMMENDED, NOT
RECOMMENDED, MAY, and OPTIONAL in this document are to be interpreted as described in RFC 2119
and RFC 8174.

## Purpose

Education supplies a physically attended, capacity-limited qualification path. 1.0 has exactly
two education service kinds: School and Technical Institute. Education is neither a radius aura
nor a price-cleared entitlement.

## Scope and exclusions

Education owns enrolment, seat/throughput queue, attendance outcome, and qualification progress.
Citizens owns identity and itinerary intent; Buildings will own service existence; Logistics and
movement owners prove physical arrival; Production owns staffing/outputs. Kindergarten is an
explicit Post-1.0 cut and MUST NOT be introduced as a 1.0 service, evidence row, or implied
prerequisite. Universities, ambient education fields, tuition, and domestic price clearing are
also outside this mechanism.

## Invariants

- `SPEC-EDUCATION-001` — The 1.0 education catalogue contains exactly School and Technical
  Institute. No kindergarten, university, or unbounded generic education building satisfies this
  specification.
- `SPEC-EDUCATION-002` — Education owns each enrolment ID, seat/throughput reservation, queue,
  attendance record, and qualification-progress outcome. It references the Citizen and route
  result rather than copying identity, itinerary, or transport state.
- `SPEC-EDUCATION-003` — Progress requires attendance at an operating, staffed School or
  Technical Institute under finite declared seat and throughput capacity. Enrolment, route
  assignment, payment, or time elapsed without attendance MUST NOT grant qualification.
- `SPEC-EDUCATION-004` — When no compatible seat, staff, building, or route exists, enrolment
  remains an observable queue with age and reason. It MUST NOT grant a credential, delete the
  applicant, or become game over.
- `SPEC-EDUCATION-005` — Planner policy allocates scarce seats by explicit non-price criteria.
  Domestic roubles MUST NOT rank, debit, credit, or clear enrolment.

## Model and state

Education is the sole authority for education service kind, enrolment, capacity reservation,
queue, attendance result, and qualification progress. An enrolment records citizen ID, School or
Technical Institute ID, requested qualification, queue/seat state, attendance history, progress,
and shortage reason. Citizens owns the citizen and activity intent; Buildings owns placed service
existence; movement owners own actual route/travel; no record duplicates those authoritative
states.

## Failure behavior

Absent capacity, staff, route, or building leaves the applicant queued and the qualification
unearned. Interrupted attendance preserves completed progress and records the reason according to
the future policy; it never creates a diploma from a timer. A failed service does not remove the
citizen, fabricate staff, or terminate the plan.

## Observability

The Planner can inspect School/Technical Institute capacity, staffed throughput, enrolment queue,
individual attendance/progress, qualification outcome, and the reason for each wait. It can link
an enrolment to the named citizen and physical itinerary without confusing a reservation with
attendance.

## Acceptance evidence

All guards are **UNIMPLEMENTED** and block ratification. A zero-test command is failure. The
current 26-test suite proves no target below.

| Evidence | Future guard command and observable assertion | Required red mutation | Player-facing proof |
|---|---|---|---|
| `EVID-EDUCATION-001` | `cargo test -p simulation evid_education_two_tiers_only -- --test-threads=1` — only School and Technical Institute register as 1.0 education services. | Register kindergarten or a third 1.0 education service. | Inspected education catalogue capture. |
| `EVID-EDUCATION-002` | `cargo test -p simulation evid_education_attendance_capacity -- --test-threads=1` — finite capacity plus physical attendance advances progress; enrolment/timer alone does not. | Grant progress at enrolment or exceed capacity. | Inspected queue, arrival, and progress capture. |
| `EVID-EDUCATION-003` | `cargo test -p simulation evid_education_shortage_nonprice_queue -- --test-threads=1` — absent staff/route retains an aged, non-price queue without credential. | Delete queued student or rank it by roubles. | Inspected shortage and allocation rationale. |
| `EVID-EDUCATION-004` | `cargo test -p simulation evid_education_references_not_copies -- --test-threads=1` — an enrolment resolves its Citizen ID and route result from Citizens and movement authorities; no independently mutable Citizen identity, itinerary, or route record exists in Education. | Add an Education-owned copy and mutate its Citizen ID, itinerary, or route independently. | Inspected enrolment links and authoritative itinerary capture. |

## Substrate and decisions

Education has no current service type, decision, or capacity owner: the live `BuildingKind`
enumeration includes only House, goods company, freight station, train station, and external
trading (`simulation/src/map/objects/building.rs:17-24`), while human decisions enumerate only
Home, Work, and Food (`simulation/src/souls/human.rs:127-230`). Lua leisure declares capacity and
hours but this fact-sheet finds no cited simulation consumer (`base_mod/leisure.lua:1-18`). These
are absences, not evidence for the target; see the [Wave 2 fact-sheet](../../research/fact-sheets/wave2-substrate.md#2b--settlement-citizens-households-and-services).

## Deferred behavior

Kindergarten is Post-1.0 by the charter and receives no 1.0 mechanism or acceptance evidence.
University, specialisation, adult re-education, tuition, and ambient/radius qualification are
deferred design questions, not active target behavior.

## Open questions

- What two qualification outcomes correspond to School and Technical Institute in 1.0?
- Which staffing inputs are required for each service without duplicating Production ownership?
- How is interrupted attendance resumed while preserving an auditable queue and progress record?
