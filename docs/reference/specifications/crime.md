# Crime direction

**Kind:** specification
**Authority:** binding
**Status:** draft
**Owner:** settlement
**Last verified:** 2026-08-24

The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT, RECOMMENDED, NOT
RECOMMENDED, MAY, and OPTIONAL in this document are to be interpreted as described in RFC 2119
and RFC 8174.

## Purpose

Crime is recorded only as Post-1.0 direction. The charter explicitly excludes crime from 1.0, so
this document prevents it being smuggled into settlement, healthcare, housing, or labour contracts.

## Scope and exclusions

Crime has no 1.0 state owner, simulation transition, building type, queue, policy, UI claim, or
acceptance criterion. It MUST NOT be inferred from citizen wellbeing, household shortage,
healthcare failure, police-like buildings, leisure declarations, or legacy documents. Crime does
not authorize domestic monetary penalties or price clearing.

## Direction

- `SPEC-CRIME-001` — A future charter revision MAY bring crime into scope only with a separately
  ratified specification that names an authoritative module, observable causes/outcomes, physical
  constraints, queue/shortage recovery, and executable evidence.
- `SPEC-CRIME-002` — Any future crime mechanism MUST preserve persistent citizen identity and
  never resolve scarcity, housing, healthcare, or labour failure by deleting people, erasing
  queues, teleporting goods, or ending the plan.
- `SPEC-CRIME-003` — A future crime policy MUST NOT create domestic price clearing or grant a
  domestic-money authority to housing, settlement, or services.

## Current substrate

Current substrate supplies no crime mechanism: the live building kinds contain no service/crime
variant (`simulation/src/map/objects/building.rs:17-24`), and human decisions are limited to home,
work, and food (`simulation/src/souls/human.rs:127-230`). The inspector exposes location, home,
food, work, and desires—not crime state (`native_app/src/gui/inspect/inspect_human.rs:43-124`).
The Wave 2 fact-sheet classifies crime and service capacity as absent
([settlement section](../../research/fact-sheets/wave2-substrate.md#2b--settlement-citizens-households-and-services)).

## Acceptance evidence

There are **no 1.0 EVID rows**. Crime is Post-1.0 direction only; no command, mutation, or
player-facing proof may be used to imply 1.0 implementation or ratification.

## Deferred behavior

Crime is explicitly Post-1.0 under the [charter cut line](../../plan/charter-1.0.md#explicit-cuts).
Policing, punishment, criminal service buildings, legitimacy effects, and related policies remain
deferred until a future charter revision authorizes them.

## Open questions

- If a future charter admits crime, what physical and social signals are observable without a
  hidden moral-status flag?
- Which future subsystem would own events, queues, and outcomes without duplicating Citizen or
  Household state?
