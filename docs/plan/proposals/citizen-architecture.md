# Proposal — citizen and household representation for 250k identities

**Kind:** decision (draft)
**Authority:** advisory — binds nothing until accepted as a numbered decision
**Status:** proposed
**Owner:** project lead
**Date:** 2026-08-28
**Feeds:** the charter's 250,000-identity target; `SPEC-CITIZENS-001`; `SPEC-HOUSEHOLDS-004`; the save-migration seam

## Context

`HumanEnt` is one monolithic struct with physics, routing, desires and a boxed `PersonalInfo`
(`name: String`, age, gender); there is no household, no lifecycle, no need but bread. The design
thread proposes a record/body split, SoA cores, dense IDs, an event calendar, cadence bands and
bitset cohorts; Lane G computed ~320 bytes per citizen (80 MB at 250k) and noted that no source
states the active fraction per tick; Lane C2 found every one of these changes alters the save
layout and no migration mechanism exists.

## Decision proposed

1. **Save envelope and migration seam before any of the below** ([persistence](../../architecture/persistence.md)).
2. **`CitizenRecord` with an append-only dense `CitizenId`** extracted from `HumanEnt`
   (identity, household, workplace, qualification, residence, biography); `HumanEnt` keeps hot
   movement and decision state and holds its `CitizenId`. A dead Citizen #N stays #N.
3. **`Household` as a first-class entity** (membership range, residence or queue position, Food and
   Meat pantry holders, next review) — the first greenfield entity, and the gate for housing,
   provisioning and time mechanics.
4. **Citizens as scheduled actors**: a `next_event` on the record; an event calendar with stable
   ties; wake → decide → intent → commit → reschedule. Sleeping citizens refresh beliefs from the
   change journal on wake.
5. **Storage decided by benchmark**: hand-written SoA versus a crate; `EnumMap` for per-holder
   stock; byte budgets asserted once measured.
6. **Bitset cohorts** over dense IDs for labour, education, healthcare, shopping and migration
   selection — filter cheaply, think expensively.
7. **State the active-fraction target** and measure it in the headless 250k benchmark before
   layering rich citizen mechanics.

## Alternatives

- Keep `HumanEnt` monolithic and add fields. Rejected: 250k × full `HumanEnt` with per-citizen
  strings and vectors cannot meet the target; every citizen would be a body.
- Adopt an ECS. Rejected: a rewrite of world, systems and saves for no gain over dense stores.

## Consequences

Save format changes (hence 1); `spawn_human`, `update_decision`, every inspector and save/load
are touched (one to two weeks per Lane C2); the renderer must draw bodies from a snapshot, not
records ([render boundary](../../architecture/render-boundary.md)).

## Validation

Round-trip and repeat-run determinism green; a migration from a pre-split save loads; a scenario
where a citizen dies and its `CitizenId` is never reused; the headless benchmark reports the
active fraction and per-tick cost at 250k.

## Open for the Planner

"New save required" acceptable pre-1.0? Household composition at spawn? Births in 1.0?

## Related

- [Entity identity](../../architecture/entity-identity.md) · [State storage](../../architecture/state-storage.md) · [Time and events](../../architecture/time-and-events.md) · [Citizens (design)](../../simulation/society/citizens.md) · [Households (design)](../../simulation/society/households.md) · [Lane B1 §3](../../research/conversation-mining-2026-08-28/B1-society-households.md)
