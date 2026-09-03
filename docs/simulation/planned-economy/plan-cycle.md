# Plan cycle

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** economy
**Last verified:** 2026-08-28

Scope: **1.0 binding** — the charter commits to quotas, Plans, and authored plan periods. The
full control loop is the game's thesis.

## What this is

The planned economy is a control system. The Planner sets quotas and policies; enterprises
interpret them, request inputs, produce, report, and adapt; the physical economy delivers or
fails to deliver; the Planner observes outcomes and revises the Plan.

```text
PLAN
  ↓ quotas / priorities / policies
enterprises and institutions adapt
  ↓ requests / buffers / labour decisions / dispatch pressure
physical production and logistics
  ↓ queues / shortages / surplus / delay / quality / congestion
household and workplace experience
  ↓ reports / complaints / institutional information
PLANNER KNOWLEDGE
  ↓
next PLAN
```

The loop is closed. The Plan is not a UI layer on top of logistics. The Plan is one of the
forces that deforms physical systems (design bible §5, CONFIRMED). A taut quota causes
storming; storming causes freight pulses; freight pulses cause congestion; congestion delays
delivery; delayed delivery causes hoarding; hoarding causes scarcity — and the Planner must
diagnose this from imperfect reports.

## 1.0 requirement

The charter commits to three authored Plans on one continuous save. A Plan is a sequence of
quota periods (glossary). Quotas set production, provision, and housing requirements for a plan
period. Quota performance determines the tranche — a period-end allocation of border roubles
(glossary).

The production specification binds the observable discrepancy between request and consumption
([`SPEC-PRODUCTION-009`](../../reference/specifications/production.md#spec-production-009)).
No quota or period mechanism exists in a specification yet.

## Target design

The full control loop has these stages:

1. **Plan** — the Planner sets quotas per enterprise, priorities, allocation policies,
   construction orders, and reserve policies for a plan period.
2. **Quota** — enterprises receive their production targets. Each enterprise interprets the
   quota in light of its experience: can it trust the Plan to deliver inputs?
3. **Enterprise interpretation** — enterprises set their request multipliers (how much more
   than the recipe they will request), storming thresholds, and reserve targets based on
   reliability memory and ratchet history.
4. **Request** — enterprises submit their input requests to the allocation system. These are
   strategic statements, not truth (see [enterprise behavior](enterprise-behavior.md)).
5. **Allocation** — the dispatcher matches supply to demand by target-stock deficit, distance,
   and stable ID. No price participates (see [allocation](allocation.md)).
6. **Physical fulfillment** — trucks and trains carry goods from source to destination. Custody
   is accountable. Loading and unloading take time. Routes congest.
7. **Production** — enterprises consume delivered inputs, produce outputs, and record the
   binding constraint for any incomplete run.
8. **Observed outcome** — physical stock levels, queue lengths, delivery times, unmet requests,
   and surplus accumulation are the ground truth.
9. **Reporting** — enterprises report their fulfilment, requests, and output. Reports may
   diverge from physical truth (see [reports and information](reports-and-information.md)).
10. **Next Plan** — the Planner observes reports and inspections, diagnoses problems, and
    revises the Plan. The ratchet, credibility, and correction cycle all operate at this
    boundary.

### Plan periods

All temporal mechanisms — storming, ratchet, credibility, tranche — depend on a defined plan
period. The open question (Lane A, question 2) is whether plan-period boundaries are
player-defined or system-defined. The charter implies player-authored periods (a Plan is a
sequence of quota periods on one continuous save). A minimal plan-period clock may need to
arrive early in implementation, because every temporal mechanism depends on it (design bible
§5.5, §18; SYNTHESIS §3.14).

## Current substrate

No quota, no plan period, and no reporting cycle exist in the simulation.
`Government` holds only `money: Money` (`simulation/src/economy/government.rs:9-11`).
Production is continuous: `company_system` advances `progress` by `productivity * DELTA /
recipe.duration.seconds()` every tick (`simulation/src/souls/goods_company.rs:204-205`). There
is no period boundary, no deadline, no quota target, no performance tracking.

The allocation system (`make_trades` in `simulation/src/economy/market.rs:551-789`) matches
supply and demand by distance only. No plan priority, no request age, no deficit-first
ordering exists.

`EcoStats` (`simulation/src/economy/ecostats.rs:48-52`) records ring-buffered histories of
exports, imports, and internal trade volume per item. It does not track production, consumption,
or stock levels — only trade matches.

## Open questions

- Are plan periods player-defined (the Planner sets duration) or fixed (e.g. quarterly)?
- Does a minimal plan-period clock arrive before the full Plan/Quota/Tranche loop?
  (Lane A, question 2; SYNTHESIS §3.14)
- Which enterprise behaviours activate at period boundaries vs continuously?

## Related

- [Enterprise behavior](enterprise-behavior.md) — how enterprises interpret quotas.
- [Material balance](material-balance.md) — the accounting identity verified at period end.
- [Reports and information](reports-and-information.md) — what the Planner learns from reports.
- [Storming](storming.md) — a period-end temporal pattern.
- [Reliability and buffering](reliability-and-buffering.md) — the ratchet and credibility.
- [Design bible §5](../../vision/design-bible.md) — the planned economy as a control system.
- [Glossary](../../reference/glossary.md) — Plan, Quota, Tranche definitions.
