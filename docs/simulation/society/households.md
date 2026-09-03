# Households — the shared-pantry unit

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** society
**Last verified:** 2026-08-28

| Scope | Label |
|---|---|
| Household as first-class actor | 1.0 binding |
| Shared Food/Meat pantry | 1.0 binding |
| Household reviews and adaptation | architecture hook |

## What this is

A household is the smallest economic actor in the simulation. It owns shared Food and Meat
pantries, a residence or housing-queue position, and the care obligations that connect
employment to daily life. Consumption decisions belong to the household, not the individual.

The household is the unit where shortages become lived experience. When retail supply fails,
the household sends members to search and queue. When housing is overcrowded, the household
waits. When a parent works the second shift, the household rearranges childcare. The Planner
sees aggregated households by district — average queue time, overcrowding rate, discretionary
time — and can trace any aggregate back to the named household that produced it.

## 1.0 requirement

The charter requires persistent individual identities and observable state. The draft
households specification (`SPEC-HOUSEHOLDS-001` through `SPEC-HOUSEHOLDS-008`) defines the
target contract:

- A Household has a persistent ID and an explicit member set of Citizen IDs
  (`SPEC-HOUSEHOLDS-001`).
- The household owns residence assignment and housing-queue entry. Shortage retains the
  household, never deletes people or dissolves demand (`SPEC-HOUSEHOLDS-002`).
- Each household has distinct shared Food and Meat pantry records at a permitted dwelling point
  of use (`SPEC-HOUSEHOLDS-004`).
- A pantry is credited only by completed Logistics delivery → Resources receipt → Needs
  consumption; never by request, reservation, or arrival alone (`SPEC-HOUSEHOLDS-005`).
- Housing priority is a non-price Planner policy (`SPEC-HOUSEHOLDS-006`).
- Death removes one named member once via an immutable `DeathResultID`; the last-member
  household becomes `EmptyAfterDeath` (`SPEC-HOUSEHOLDS-008`).

## Target design

### Membership and composition

A household contains one or more citizens. A citizen belongs to at most one household. Members
include working adults, dependants (children, elderly), and potentially pensioners with
distinct scheduling roles (see [time](time.md)).

Historical Soviet households averaged 3.5–4.0 persons: two adults, one to two children, and
sometimes a grandparent. Whether households spawn with realistic composition or form over time
is an open question (B1 §6.1).

### Residence and queue position

The household either occupies a dwelling or holds a position in the [housing queue](housing.md).
A queued household retains its members, its queue age, and its reason. Displacement
(building demolition, enterprise closure) re-enters the household with a priority bonus.

### Shared pantry

Food and Meat are household resources, not individual ones. The shared-pantry model means
one delivery satisfies the household, and one shortage affects every member.

Household pantry buffering is one domain instance of
[reliability → defensive buffering](../concepts/reliability.md). A household that has
experienced shortage maintains a small reserve; a household that has experienced plenty lets
its pantry run lower.

### Care obligations

The household tracks which members require care (children, sick), which members provide it,
and the time cost. A childcare facility releases specific adult hours — the household
recalculates its time budget when care coverage changes.

### Adaptation state

The household adapts to its circumstances: it shifts shopping responsibility between members,
adjusts the acceptable commute distance for a job change, chooses which queue to join, and
decides when to apply for relocation. These adaptations are slow, driven by household reviews
that fire on threshold crossings or lifecycle events — not every tick.

### Household reviews

A household review is triggered by: pantry below threshold, shopping failure, childcare gap,
housing-queue movement, lifecycle event (birth, death, qualification change), or relocation
opportunity. Each review updates one domain of household state: shopping assignment, childcare
coverage, work-commute threshold, housing application, or migration consideration.

## Current substrate

No household entity exists. `grep -r Household simulation/` returns zero results. `Home`
(`simulation/src/souls/desire/home.rs:8-11`) is `{ house: BuildingID, last_score: f32 }` — a
bare building reference with a constant score of 0.2. `spawn_human`
(`simulation/src/souls/human.rs:237-278`) creates one human per empty house and assigns
building ownership. The inspector renders one owner and current occupants rather than a
household (`native_app/src/gui/inspect/inspect_building.rs:96-117`). Multiple humans can share
a house building but have no shared pantry, no membership, no family structure.

The households specification documents this gap: "Current code has no household ID, membership,
shared inventory, or housing queue" (`households.md:113-119`).

## Research basis

Soviet households were the unit of daily survival. The housing office (ZhEK) tracked
households by apartment. Ration cards (during wartime and certain shortages) were issued per
household. Housing allocation queues were household-keyed: the applicant was the family, not
the individual.

The design's shared-pantry model reflects Soviet reality: one household shopped, cooked, and
consumed together. A working mother's shopping trip fed the family; a pensioner's queue time
freed the mother for work.

## Open questions

- Household composition at spawn: historical mean (3.5–4.0 persons) or single adults that
  form households over time? (B1 §6.1.)
- Which lifecycle events (marriage, divorce, birth, children leaving) create, merge, split, or
  retire a household? (`households.md:131-133`.)
- Which dwelling-capacity states permit temporary overcrowding, and what need outcome do they
  produce? (`households.md:129-130`.)
- Which non-price queue attributes and tie-breaks are required for 1.0?
  (`households.md:131`.)

## Related

- [Citizens](citizens.md)
- [Housing](housing.md)
- [Time](time.md)
- [Provisioning](provisioning.md)
- [Households specification](../../reference/specifications/households.md)
- [Needs specification](../../reference/specifications/needs.md)
- [Reliability concept](../concepts/reliability.md)
