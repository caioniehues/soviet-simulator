# Time — a resource

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** society
**Last verified:** 2026-08-28

| Scope | Label |
|---|---|
| Household time budget | 1.0 candidate |
| Social-reproduction balance | architecture hook |
| Gendered time | research |

## What this is

Time is a finite, observable, measurable citizen resource. A shortage costs time even when the
good is eventually obtained. A citizen who queues two hours for bread has not "failed" — they
have spent time. That time is not available for work, childcare, cooking, or rest.

The household time budget is a conservation law: total waking hours per week equal the sum of
all committed activities plus discretionary time. When one activity grows (shopping queues
lengthen), another must shrink (discretionary time, sleep, or a skipped activity). The Planner
can see exactly which activity consumes the time and invest accordingly.

## Target design

### Conservation law — CONFIRMED (B1 §3a; bible §7.3)

The design proposes a conserved weekly account per household (B1 §3a):

```text
total_hours = Σ committed + discretionary

committed categories:
  formal_work        -- from work assignments
  commute            -- from pathfinding distance
  shopping_queue     -- from retail queue wait times
  childcare          -- from household composition and childcare availability
  domestic           -- base rate, reduced by service access (canteen, laundry)
  household_production -- from plot assignment (Post-1.0)
  healthcare         -- from health state and queue times
```

This is a hard constraint, never violated. One `u16` addition per household per cadence tick —
trivial at 100,000 households.

A change to any committed category wakes the household: job assignment, commute change, retail
shortage, child birth or age-out, plot assignment, health event. The household does not
recompute every tick — only when a pressure changes a committed category.

### Social-reproduction balance — PLAUSIBLE (B1 §3f; B1-21)

The same identity summed by district:

```text
TotalAdultHours(district, period)
  = FormalEmployment
  + Commuting
  + HouseholdCare
  + ShoppingQueues
  + HouseholdProduction
  + IllnessAbsence
  + DiscretionaryTime
```

A stacked bar per district shows which sink is eating time. A district with 40 % discretionary
time is functioning; one with 5 % is in time-poverty crisis.

Every social investment is measurable in recovered hours. If the Planner improves retail supply
(more shops, better logistics), shopping time shrinks and discretionary time grows. If the
Planner builds a kindergarten, childcare time shrinks and formal employment and discretionary
time both grow.

### Gendered time — CONFIRMED (Gordon & Klopov 1972; Szalai)

Soviet time-budget studies document a sharp gender disparity:

| Activity | Women | Men |
|---|---|---|
| Domestic work | 28 h/week | 12 h/week |
| Food preparation | 10–12 h/week | 1.5–2 h/week |
| Grocery shopping | ~6 h/week | ~3 h/week |
| Laundry | ~6 h/week | 20–30 min/week |

Whether the game models gender-differentiated time burdens is an open question (B1 §6.6).
The historical data is clear: women bore 2–3× the domestic work load. Modelling this
accurately shows why childcare and canteen investment had outsized returns.

### Pensioners as queue labour — Post-1.0 (B1-MISSED-08)

A household with a pensioner sends them to queue during work hours. The pensioner's "idle"
time converts into shopping time, freeing working-age adults. This is a simple mechanism with
large time-budget impact: a three-generation household (worker, spouse, grandparent) has a
structural advantage over a nuclear family.

### Two-shift schools — Post-1.0 (B1-MISSED-09)

Soviet schools commonly operated two shifts due to building shortages:
- 1st shift: 08:30–14:30 (grades 1, 5–10)
- 2nd shift: 15:30–19:30 (grades 2–4)

A child in the second shift needs adult supervision in the morning. Two children in different
shifts create an all-day supervision burden. School shifts affect the household time budget
directly.

### Time poverty chain — CONFIRMED (B1-22)

The chain linking the economy to society through time:

```text
retail supply failure
  → longer search time
  → longer queues
  → household time burden ↑
  → fatigue
  → late arrivals at work
  → reduced labour performance
  → reduced production
  → retail supply failure (cycle repeats)
```

This is a cross-system causal loop: the economy presses on society through retail; society
presses back on the economy through labour performance. The Planner breaks the loop by
improving any link — better logistics, more shops, shorter commutes, childcare release.

## Current substrate

Citizens have `Work` with a time interval (approximately 08:00–18:00 with random offsets,
`simulation/src/souls/desire/work.rs:32-37`) and `BuyFood`. There is no time budget, no
scheduling, no fatigue, no time tracking, no household-level accounting, no discretionary time.
The human decision system (`simulation/src/souls/human.rs:127-230`) picks the highest-scoring
desire; it does not allocate hours.

## Research basis

- Gordon & Klopov (1972), *Man After Work*: Soviet time-budget studies.
- Szalai (1970s), "Women's Time": gendered time-budget data.
- CIA, "Selected Information on Consumer Welfare in the USSR" (March 1955,
  CIA-RDP79T01149A000400090004-0): 33.65 hours per week to buy a basic food basket (1954
  Moscow).
- CIA, "Consumer Frustrations and the Soviet Regime" (August 1979): 6-hour queue for a hat,
  10-hour queue for a rail ticket.
- CIA, "Soviet Society in the 1980s" (December 1982): shopping/queueing as a "deadening chore."

## Open questions

- Should the game model gender-differentiated time burdens? The data supports it; the design
  implication is significant. (B1 §6.6.)
- At what cadence does the household time budget recompute? Monthly, quarterly, on-event-only?
- Which time categories are 1.0 (work, commute, shopping) and which are Post-1.0 (plot work,
  healthcare, domestic detail)?

## Related

- [Households](households.md)
- [Provisioning](provisioning.md)
- [Labour](labor.md)
- [Workplaces](workplaces.md)
- [Education](education.md)
- [Healthcare](healthcare.md)
- [Social reproduction concept](../concepts/social-reproduction.md)
