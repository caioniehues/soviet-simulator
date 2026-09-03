# Education — two 1.0 tiers

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** society
**Last verified:** 2026-09-03

| Scope | Label |
|---|---|
| School and Technical Institute | 1.0 — charter row *Agriculture and services* |
| Qualification → assignment → relocation | Post-1.0 hook |
| Two-shift schools | Post-1.0 |

## What this is

Education produces qualified workers from a physical service chain: seats, staff, buildings,
attendance, and time. A worker who attended a Technical Institute can fill a technician
position; one who did not, cannot. Education is not a radius aura or a price-cleared
entitlement — it is a capacity-limited service that citizens must physically attend.

## 1.0 requirement

The charter commits to "two education tiers." The draft education specification
(`SPEC-EDUCATION-001` through `SPEC-EDUCATION-005`) defines the contract:

- The 1.0 catalogue contains exactly School and Technical Institute
  (`SPEC-EDUCATION-001`). No kindergarten (Post-1.0 charter cut), no university.
- Progress requires attendance at an operating, staffed facility under finite seat and
  throughput capacity (`SPEC-EDUCATION-003`). Enrolment, route assignment, payment, or time
  elapsed without attendance does not grant qualification.
- When no compatible seat, staff, building, or route exists, enrolment remains an observable
  queue with age and reason (`SPEC-EDUCATION-004`).
- Non-price allocation of scarce seats (`SPEC-EDUCATION-005`).

## Target design

### Education → qualification → assignment → relocation — CONFIRMED (B1-28; B2-13)

The Soviet graduate-assignment system (raspredelenie), established in 1933, placed graduates
in positions by commission. Key features:

- Mandatory placement anywhere in the USSR for 3–4 years.
- Young specialists received special status: could not be fired during the assignment period.
- Housing benefits came with the assignment.
- The system created forced migration and household disruption.

In the game, education produces a qualification outcome. Qualification determines which work
assignments a citizen is eligible for. Assignment may require relocation — a graduate sent to
a distant factory must move, disrupting their household. This chain connects
[education](education.md) to [labour](labor.md) to [housing](housing.md) to
[migration](migration.md).

### Two-shift schools as a household-time fact — Post-1.0 (B1-MISSED-09)

Soviet schools commonly operated two shifts due to building shortages:
- 1st shift: 08:30–14:30 (grades 1, 5–10)
- 2nd shift: 15:30–19:30 (grades 2–4)

A child in the second shift requires adult supervision in the morning. Two children in
different shifts create an all-day supervision burden. This is primarily a
[household time](time.md) mechanic, not an education mechanic: it affects care obligations
and the time budget, not qualification outcomes.

### Qualification taxonomy — open question

The draft specification does not name the two qualification outcomes that correspond to School
and Technical Institute (`education.md:97`). The bible (§21.4) lists the qualification
taxonomy as an open question. Possible minimal taxonomy: School produces a basic-worker
qualification; Technical Institute produces a technician qualification. The number of
categories matters only where they change allocation or production.

## Current substrate

No education service type exists. `BuildingKind`
(`simulation/src/map/objects/building.rs:17-24`) has only House, goods company, freight
station, train station, and external trading. Human decisions
(`simulation/src/souls/human.rs:127-230`) enumerate only Home, Work, and Food. `PersonalInfo`
has no qualification field.

Lua leisure data (`base_mod/leisure.lua:1-18`) declares capacity and hours for a leisure
building, but no simulation consumer is cited for it. No school or institute building exists.

## Research basis

- Raspredelenie: established 1933, documented in Wikipedia "Job by distribution"; Belarusian
  continuation documented in Equal Times (2022).
- CIA: "Goals and Attainments of Education in the USSR" (April 1952, ID 7680); "School
  Enrollment in the USSR 1950-75" (September 1966, ID 2136); "Soviet Scientific and
  Engineering Manpower" (September 1972, CIA-RDP08S01350R000602030002-5).
- B2 §3 calibration table: educational attainment ~5 years (1950) → 9+ years (1980).
- B1-MISSED-09 on two-shift schools.

## Open questions

- What two qualification outcomes correspond to School and Technical Institute in 1.0?
  (`education.md:97`.)
- Which staffing inputs does each service require without duplicating Production ownership?
  (`education.md:98`.)
- How is interrupted attendance resumed while preserving an auditable queue and progress
  record? (`education.md:99`.)

## Related

- [Citizens](citizens.md)
- [Labour](labor.md)
- [Migration](migration.md)
- [Time](time.md)
- [Healthcare](healthcare.md)
- [Education specification](../../reference/specifications/education.md)
