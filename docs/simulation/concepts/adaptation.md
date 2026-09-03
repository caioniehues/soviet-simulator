# Adaptation

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** simulation
**Last verified:** 2026-08-28

Scope: **1.0 candidate** — the charter commits to persistent citizen identities with observable
state; adaptive enterprise behaviour drives the dishonest-enterprise loop. The computational
pattern (stable things sleep) is a HYPOTHESIS.

## What this is

Every actor in the simulation adapts to its environment. Enterprises inflate requests when
delivery is unreliable. Citizens search different stores when their usual one is empty.
Workers change jobs when housing or commute conditions are poor. Managers hoard labour against
storming. Networks adjust routing under congestion. The state revises plans when reports arrive.

Adaptation is what makes the planned economy a game. If actors did not adapt, the Planner would
set parameters once and the system would run forever. Because actors adapt — strategically,
defensively, and imperfectly — the Planner must continuously observe, diagnose, and correct.

The actors who adapt and the forms their adaptation takes:

| Actor | Adaptation form | Driven by |
|---|---|---|
| Enterprise | Request inflation, labour hoarding, local workshops, report manipulation | Reliability memory, storming pressure, ratchet experience |
| Citizen | Store search, queue choice, job change, relocation, social contacts | Retail reliability, commute, housing quality, shop knowledge |
| Household | Pantry buffering, plot cultivation, schedule adjustment, informal exchange | Supply reliability, time pressure, neighbourhood knowledge |
| Worker | Tenure ramp, absenteeism, overtime acceptance, productivity | Workplace conditions, commute, fatigue, household needs |
| Manager | Capacity concealment, storming timing, expediter deployment | Quota pressure, ratchet history, planning credibility |
| Network | Route adjustment, congestion equilibrium | Volume-delay feedback (BPR/Gawron for roads) |
| State (Planner) | Plan revision, priority reassignment, reserve policy, confiscation | Reports, inspections, observed outcomes |

## 1.0 requirement

The charter commits to "persistent individual identities and observable state" that let the
Planner understand and correct the dishonest-enterprise loop (charter §Identity). The
production specification commits to inspectable request-vs-consumption discrepancies
([`SPEC-PRODUCTION-009`](../../reference/specifications/production.md#spec-production-009)).
These are the observable face of enterprise adaptation.

## Target design

The design proposes (design law 16, HYPOTHESIS) a computational principle:

> **Stable things sleep; pressure wakes them.**

Not every citizen, every enterprise, or every vehicle needs evaluation every tick. A citizen
with a stable job, adequate housing, and reliable food supply can sleep until something
changes: a store restocks, a factory closes, a housing offer arrives. Only pressure — a
threshold crossing, a failed search, an institutional change — wakes the actor for
re-evaluation.

The cost savings make 250,000 citizen identities feasible. If 10% are awake per tick, the
active set is 25,000 — tight but plausible (Lane G, G-06; the active fraction is an open
question).

A sleeping actor's memory goes stale. The design resolves this (design bible §7.4, answering
Lane G-15) through the change journal: on wake, the actor refreshes its knowledge from the
journal of changes since its last evaluation, not by scanning the world. This requires the
change-journal infrastructure proposed in `docs/plan/proposals/citizen-architecture.md`.

### Adaptation is not personality

Adaptation derives from physical and institutional conditions, not from hidden personality
traits. An enterprise inflates requests because its delivery history is poor, not because it
has a "dishonesty" stat. A citizen changes stores because the old one was empty last visit, not
because they have a "search behaviour" flag. The observable effect is the same; the cause is
traceable to a physical condition the Planner can act on (design bible §19, anti-patterns).

## Current substrate

Enterprise adaptation does not exist dynamically. `request_multiplier` is a static per-prototype
`i32` (`prototypes/src/types/recipe.rs:52`). No `reliability_memory` or equivalent state exists
in `GoodsCompanyState` (`simulation/src/souls/goods_company.rs:69-78`).

Citizen adaptation is minimal. `HumanDecision` uses a max-score function over `{Home, Work,
BuyFood}` (`simulation/src/souls/desire/`). The `BuyFood` desire places a global market order
matched by distance — no search, no memory, no learning. Citizens have no store knowledge, no
social contacts, no substitute behaviour.

The sleep/wake pattern does not exist. Every human is evaluated every `decision.wait` ticks
(30–80 ticks, `simulation/src/souls/human.rs`). Every system runs every 20 ms tick. No
event-driven wake-up, no cadence bands, no change journal.

## Open questions

- What fraction of citizens should be awake per tick at 250,000? This number determines
  whether the architecture is feasible (Lane G, open question 5).
- Should enterprise adaptation be driven by a rule-based heuristic or by a more general
  learning model?

## Related

- [Reliability](reliability.md) — the primary driver of enterprise adaptation.
- [Information](information.md) — adaptation changes what actors report, not just what they do.
- [Queues](queues.md) — adaptation shifts queue membership.
- [Social reproduction](social-reproduction.md) — citizen adaptation consumes household time.
- [Enterprise behavior](../planned-economy/enterprise-behavior.md) — enterprise adaptation
  as the core loop.
- [Design bible §7.4, §13](../../vision/design-bible.md) — scheduling and change journal.
