# Demography — death in 1.0

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** society
**Last verified:** 2026-09-03

| Scope | Label |
|---|---|
| Death | 1.0 — charter row *Agriculture and services* |
| Births | Post-1.0 (open question) |
| Fertility echo | Post-1.0 hook |

## What this is

The 1.0 target population model includes death; current code does not yet change population
through lifecycle, births, or migration (see Current substrate below). Death is a 1.0 charter
commitment. Births remain an open question. The long-wave consequence of demographic change —
fewer workers a generation later — is the fertility echo.

## 1.0 requirement

The charter commits to "demographics including death." The draft citizens specification
(`SPEC-CITIZENS-007`) defines the death contract:

- Death changes the existing Citizen ID to a deceased lifecycle state.
- It preserves the ID, transition reason/time, and audit record.
- It does not respawn or replace the citizen, silently delete its history, or require a
  deathcare service.
- The immutable `DeathResultID` authorizes, but does not perform, Household membership
  removal.

The households specification (`SPEC-HOUSEHOLDS-008`) consumes the `DeathResultID` once to
remove the named deceased member. When the final member dies, the household enters
`EmptyAfterDeath` state rather than silently disappearing.

Deathcare (funeral services) is neither required nor implied. The charter explicitly cuts
deathcare to Post-1.0.

## Target design

### Death as lifecycle result

Death is a recorded result, not a deletion. The dead citizen retains its identity so the
Planner can inspect what happened: who died, why, when, where they lived, where they worked.
Aggregate mortality statistics derive from individual death records.

The Planner sees the labour-force consequence: death reduces the workforce. If deaths
concentrate in a specific enterprise, district, or age group, the pattern is inspectable.

### Births — open question

The citizens specification (line 117) says: "Births and emigration remain unresolved: which,
if either, is a 1.0 lifecycle transition beyond persistent identity and charter-required
death?"

The charter does not explicitly commit to births. Population can shrink through death. Whether
it can grow through births in 1.0 is an open design question.

### Fertility echo — CONFIRMED (B1-24; B2-07)

Housing crowding and care burden affect family formation. A generation that grew up in housing
shortage has low housing expectations and tolerates crowding — they may have more children
despite poor conditions. A generation that grew up with adequate housing expects better and
defers family formation when conditions worsen.

Shortage today echoes in the labour force decades later. A housing crisis that suppresses
births produces a labour-force contraction one generation later, which constrains production,
which constrains housing construction. The echo is a long-wave causal loop.

This mechanism depends on the [expectations and cohorts](citizens.md) system (bible §7.10,
PLAUSIBLE).

### CIA demographic record — DOCUMENTED (B2-19)

CIA tracked Soviet demographic change as an economic threat:

- European USSR working-age population was declining through 1995 (CIA, "Strains in the
  Soviet Labor Force", March 1987, CIA-RDP90T00114R000800010001-9).
- Labour supply growth came primarily from Central Asian republics.
- Labour demand was concentrated in western industrialised regions, Siberia, and the Far East.
- Geographic mismatch was "a critical challenge to the Gorbachev regime."
- CIA tracked fertility decline from 1968 (ID 6944, "Soviet Concern Over Falling Birth Rate").

The 1982 society report (CIA-RDP83T00853R000200180002-4) documents that housing shortages,
consumer scarcity, and the "deadening chore" of searching and standing in line were "obstacles
to stable family life and having children."

## Current substrate

`PersonalInfo` (`simulation/src/souls/human.rs:42`) has `age: u8` randomised between 20 and
50 at spawn. Age never increments. There is no death, no birth, no lifecycle beyond the
initial spawn. Citizens persist unchanged. No `DeathResultID`, no deceased state, no lifecycle
transitions.

## Open questions

- Are births a 1.0 lifecycle transition? (`citizens.md:117`.)
- If births exist, what determines fertility — is it a policy, a household decision, or
  an emergent consequence of housing and service conditions?
- Should the fertility echo be an explicit mechanic, or does it emerge from the interaction
  of cohort expectations and housing conditions?
- Which causes of death are modelled? Age, illness, workplace accidents, or only an abstract
  mortality rate?

## Related

- [Citizens](citizens.md)
- [Households](households.md)
- [Healthcare](healthcare.md)
- [Migration](migration.md)
- [Time](time.md)
- [Citizens specification](../../reference/specifications/citizens.md)
- [Households specification](../../reference/specifications/households.md)
