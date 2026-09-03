# Citizens — persistent individual identity

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** society
**Last verified:** 2026-08-28

| Scope | Label |
|---|---|
| Persistent identity | 1.0 binding |
| L0–L4 materialisation levels | architecture hook |
| Biographies | architecture hook |

## What this is

A citizen is a person in the simulation. Every person has one persistent identity through
employment, housing shortage, sickness, travel, and death. A citizen is never an anonymous
headcount or an aggregate.

The Planner diagnoses shortages by following individual citizens: why this worker is absent,
where that family queues, how long a commute takes. Persistent identity is what makes the
dishonest-enterprise loop legible at the human scale.

## 1.0 requirement

The charter requires "persistent individual identities and observable state" and 250,000 citizen
identities at 60 fps. The draft citizens specification (`SPEC-CITIZENS-001` through
`SPEC-CITIZENS-007`) defines the target contract. Key commitments:

- One Citizen ID persists through save/load, assignment, travel, unmet need, sickness, and
  housing shortage (`SPEC-CITIZENS-001`).
- A citizen owns activity intent and references to home, household, work, education, health,
  and itinerary — but does not copy or mutate the state of referenced modules
  (`SPEC-CITIZENS-002`).
- Labour allocation is non-price (`SPEC-CITIZENS-004`).
- Death retains the Citizen ID in a deceased state with an immutable `DeathResultID`
  (`SPEC-CITIZENS-007`).
- Births and emigration remain open questions in the specification.

## Target design

### L0–L4 materialisation levels — HYPOTHESIS

The design proposes five layers for scaling to 250,000 identities (bible §7.1; citizen
architecture proposal):

| Level | What lives there | When active |
|---|---|---|
| L0 — `CitizenRecord` | Biography, household membership, core state | Always |
| L1 — scheduled social agent | Next event in the deterministic calendar | When a pressure wakes it |
| L2 — active activity/trip | Current task, resource references | During a task |
| L3 — `CitizenBody` | Physical position, movement, collision | During travel |
| L4 — render instance | Visible sprite or mesh | Only for bounded visible citizens |

A citizen moves between layers without losing identity. Most citizens rest at L0–L1 on any
given tick. The active fraction per tick determines whether 250,000 is feasible within the
frame budget (G-06: ~320 bytes minimum per citizen, 80 MB total; a naive full scan costs
~2.7 ms sequential).

See [entity identity](../../architecture/entity-identity.md) and
[state storage](../../architecture/state-storage.md) for the architectural detail the lead
writes.

### Biographies — HYPOTHESIS

A citizen accumulates a permanent record: birth, education milestones, qualification, major
employment changes, household formation, children, major relocations, death. The design
proposes retention classes (bible §7.14): recent events keep full detail for diagnostic
inspection; old routine history compresses to milestones; lifetime transitions (birth,
qualification, death) are never discarded.

The Planner can follow any citizen's life story to understand why they are where they are.

## Current substrate

`HumanEnt` (`simulation/src/world.rs:87-105`) is the sole citizen type. It is a monolithic
struct holding physics, routing, food-buying, and personal info in one entity. Every citizen
carries `Transform`, `Speed`, `Location`, `Pedestrian`, `Collider`, `Router`, `Itinerary`,
`HumanDecision`, `Home`, `BuyFood`, `Bought`, `Work`, and `PersonalInfo`.

`PersonalInfo` (`simulation/src/souls/human.rs:42`) stores three fields: `name: String`,
`age: u8`, `gender: Gender`. Age is randomised between 20 and 50 at spawn and never
increments. There is no household reference, no qualification, no biography, no lifecycle,
no expectations.

`spawn_human` (`simulation/src/souls/human.rs:237-278`) creates one human per empty house and
immediately assigns ownership. There is no household, no queue, no eligibility.

The human decision system (`simulation/src/souls/human.rs:127-230`) chooses the highest-scoring
desire among Home, Work, and BuyFood. There is no schedule, no time budget, no adaptation.

No `CitizenRecord`/`CitizenBody` split exists. No SoA layout. No wake/sleep mechanism. No
change journal. The L0–L4 model is entirely greenfield.

## Research basis

Soviet citizens held persistent documentary identities: internal passport (from age 16),
propiska (residence registration), work record book (trudovaya knizhka), military ID, education
diplomas. Life events were tracked by the enterprise, the housing office, and the district
Soviet.

The game's persistent identity mirrors this documentary reality. A citizen is never a
statistical placeholder — the Planner should be able to trace a worker's career the way a
factory personnel office would.

## Open questions

- What is the steady-state active fraction per tick at 250,000 citizens? (G open question 5.)
  This number determines whether the architecture is feasible.
- Births and emigration: which lifecycle transitions beyond death does 1.0 include?
  (`citizens.md:117`.)
- What non-price policy orders equally eligible work assignments?
  (`citizens.md:119`.)
- How does a sleeping citizen refresh stale memory on wake? The design says the change journal
  (G-15 tension), but the change journal does not exist.

## Related

- [Households](households.md)
- [Citizens specification](../../reference/specifications/citizens.md)
- [Citizen architecture proposal](../../plan/proposals/citizen-architecture.md)
- [Labour](labor.md)
- [Time](time.md)
- [Demography](demography.md)
- [Glossary](../../reference/glossary.md)
