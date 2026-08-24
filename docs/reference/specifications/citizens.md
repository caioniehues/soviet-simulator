# Citizens specification

**Kind:** specification
**Authority:** binding
**Status:** draft
**Owner:** settlement
**Last verified:** 2026-08-24

The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT, RECOMMENDED, NOT
RECOMMENDED, MAY, and OPTIONAL in this document are to be interpreted as described in RFC 2119
and RFC 8174.

## Purpose

Citizens provide persistent individual identity for labour, residence, education, health, needs,
and physical trips. A worker is never an anonymous production scalar: assignment identifies a
citizen and a destination, while shortage remains a visible state.

## Scope and exclusions

This specification owns individual identity, lifecycle, individual assignment references, and
itinerary intent. Households owns membership and shared pantry state; Needs owns satisfaction;
Education and Healthcare own their service queues/outcomes; Logistics, Roads, Pathfinding, and
Traffic own physical movement state. Domestic labour allocation is not a market and MUST NOT
price-clear. Crime, vehicle manufacture, vehicle fuel lifecycle, kindergarten, deathcare, and
epidemics are not 1.0 mechanisms here.

## Invariants

- `SPEC-CITIZENS-001` — Every citizen has one persistent Citizen ID through save/load, assignment,
  travel, unmet need, sickness, and housing shortage. A missing job, seat, dwelling, route, or
  service MUST NOT delete the citizen or replace the identity with an aggregate count.
- `SPEC-CITIZENS-002` — Citizen owns its current intended activity and references to home,
  household, work, education, health, and itinerary outcomes. Referenced modules own the
  corresponding assignment, capacity, need, treatment, and route state; Citizen MUST NOT copy or
  mutate them as a parallel authority.
- `SPEC-CITIZENS-003` — A work or study assignment identifies both citizen and destination and
  remains visible until the owning allocator changes it. Assignment requires compatible capacity
  and a route result before attendance; assignment alone neither moves a person nor produces work,
  education, or service completion.
- `SPEC-CITIZENS-004` — Labour allocation is a Planner policy over physical eligibility,
  capacity, and reachability. It MUST NOT debit, credit, rank, or clear through domestic roubles;
  unassigned citizens and vacancies remain observable.
- `SPEC-CITIZENS-005` — A citizen's Food and Meat outcomes reference the household/Needs contract:
  neither a Market match, route assignment, nor itinerary arrival satisfies a need without the
  named authoritative Needs consumption event.
- `SPEC-CITIZENS-006` — Illness, insufficient education, unreachable work, and housing shortage
  produce inspectable reduced participation, waiting, reassignment eligibility, or going without.
  They MUST NOT cause a game-over path or conceal the underlying citizen.
- `SPEC-CITIZENS-007` — Citizens is the sole authority for the death lifecycle transition and its
  immutable `DeathResultID`. Death changes the existing Citizen ID to a deceased lifecycle state
  and preserves that ID, transition reason/time, and audit record; it MUST NOT respawn or replace
  the citizen, silently delete its history, or require a deathcare service. The result authorizes,
  but does not perform, Household membership removal.

## Model and state

Citizens is the sole authority for Citizen ID, individual lifecycle, individual activity intent,
and cross-module references. A citizen records ID, lifecycle state, household reference, current
residence reference, work/study/health assignment references, itinerary reference, and visible
unmet outcomes. Households owns membership/residence allocation; Education owns seats/progress;
Healthcare owns care capacity/treatment; Needs owns consumption/satisfaction; physical route and
movement owners retain all topology, route, vehicle, and congestion state. At death, Citizens
records the deceased lifecycle state and immutable `DeathResultID`; Households consumes that result
as an interface and owns its membership transition. No other module duplicates this lifecycle
transition.

## Failure behavior

When an assignment cannot reach its destination or capacity is absent, Citizen retains its
identity and records waiting or unassigned status with a reason. A cancelled trip follows the
movement owner's recovery contract; it does not imply attendance. Failed care or provision leaves
the relevant need/health outcome visible and may reduce participation, never delete the citizen.
Death is a recorded lifecycle result, not a game-over or silent-deletion path; its household
consequence follows the Household-owned once-only membership rule.

## Observability

The Planner can inspect a citizen's ID, household and residence link, activity intent,
work/study/care references, itinerary state, education and health outcome, Food/Meat need links,
reasoned unassigned or waiting state. For a deceased citizen, the Planner can inspect the same
Citizen ID, death result/reason/time, preserved audit record, and linked Household outcome.
Aggregate employment counts cannot replace individual identity where a causal explanation is needed.

## Acceptance evidence

All guards are **UNIMPLEMENTED** and block ratification. A zero-test command is failure. The
current 26-test suite proves no target below.

| Evidence | Future guard command and observable assertion | Required red mutation | Player-facing proof |
|---|---|---|---|
| `EVID-CITIZENS-001` | `cargo test -p simulation evid_citizens_identity_shortage_persists -- --test-threads=1` — an unassigned citizen survives a housing/work/service shortage with the same ID. | Despawn or replace the citizen when allocation fails. | Inspected citizen shortage view. |
| `EVID-CITIZENS-002` | `cargo test -p simulation evid_citizens_attendance_requires_arrival -- --test-threads=1` — assignment and route result do not create attendance; physical arrival precedes it. | Mark attendance at assignment or route creation. | Inspected commute and attendance capture. |
| `EVID-CITIZENS-003` | `cargo test -p simulation evid_citizens_nonprice_labour_queue -- --test-threads=1` — capacity/reachability-limited allocation retains visible vacancy and citizen queue without rouble ordering. | Rank applicants by roubles or hide the unassigned result. | Inspected assignment rationale and queue. |
| `EVID-CITIZENS-004` | `cargo test -p simulation evid_citizens_food_meat_needs_boundary -- --test-threads=1` — a Market match, route assignment, and itinerary arrival for Food or Meat leave Resources stock and Needs satisfaction unchanged; only the named Needs consumption event after the household physical chain can satisfy. | Mark Food/Meat satisfied or consume stock on match, route assignment, or arrival. | Inspected citizen need link and household consumption capture. |
| `EVID-CITIZENS-005` | `cargo test -p simulation evid_citizens_death_identity_result -- --test-threads=1` — death retains the same Citizen ID in deceased state with one immutable DeathResultID and audit record; it neither respawns/replaces the citizen nor invokes deathcare. | Respawn a new citizen, erase the original identity/audit record, or require a deathcare service to emit the result. | Inspected deceased-citizen and death-result capture. |

## Substrate and decisions

`HumanEnt` is the only current individual identity-facing type and derives save serialization, but
contains only location, itinerary, home, bread desire, optional work, and personal info
(`simulation/src/world.rs:86-104`). Its decision system chooses Home, Work, or Food and begins
the selected desire (`simulation/src/souls/human.rs:127-230`); spawn creates a job-opening Market
request (`simulation/src/souls/human.rs:251-272`). The current human inspector exposes home,
last ate, work, and desires, not household/service/lifecycle contracts
(`native_app/src/gui/inspect/inspect_human.rs:43-124`). These partial facilities do not prove
the target; see the [Wave 2 fact-sheet](../../research/fact-sheets/wave2-substrate.md#2b--settlement-citizens-households-and-services).

## Deferred behavior

Crime, loyalty, legitimacy, vehicle manufacture, fuel lifecycle, kindergarten, deathcare,
epidemics, tourism, and domestic price clearing receive no 1.0 mechanism or evidence here.

## Open questions

- Births and emigration remain unresolved: which, if either, is a 1.0 lifecycle transition beyond
  persistent identity and charter-required death?
- What planner-authored, non-price policy orders equally eligible work assignments?
- What route-feasibility threshold makes an assignment eligible without duplicating Pathfinding or
Traffic ownership?
