# Social reproduction

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** society
**Last verified:** 2026-08-28

Scope: **1.0 binding** — the charter commits to demographics including death, two education
tiers, healthcare, housing, and Food and Meat as separate dwelling needs. The full
social-reproduction loop is a design principle.

## What this is

Workers are physically reproduced. The Plan produces goods; goods sustain households;
households produce labour; labour executes the Plan. The loop is closed:

```text
PLAN
  → production (quotas, allocation, logistics)
    → goods, housing, services, infrastructure
      → household life (food, warmth, health, education, time, commute)
        → labour force (skill, health, availability, motivation, family)
          → PLAN
```

Every link in this chain is physical. A Plan that builds factories without housing creates
labour shortages. A housing programme without shops, clinics, schools, childcare, and heating
creates overcrowded districts where workers are absent, ill, or leaving. A city where the
shopping basket takes 33 hours per week (CIA, 1954 Moscow) has less labour available than a
city where it takes 15.

This is the **social-reproduction balance**: the identity that connects the Plan to the labour
force. Per district, per period:

```text
potential adult hours
  − employment
  − commuting
  − care (childcare, eldercare, healthcare access)
  − shopping and queues
  − household production (cooking, cleaning, maintenance)
  − illness
  = discretionary time
```

A stacked bar per district shows which sink is consuming time. Every social investment is
measurable in recovered hours. A kindergarten that releases 20 hours per week of care labour
from 200 households returns 4,000 hours per week to the labour force — more than hiring 100
new workers.

## 1.0 requirement

The charter commits to demographics including death, two education tiers, healthcare, and Food
and Meat as separate dwelling needs. These are links in the social-reproduction chain. The
household specification requires shared-pantry units
([`SPEC-HOUSEHOLDS-004`](../../reference/specifications/households.md)).

## Target design

The design proposes (Lane B1 §3a) a conservation law for household time:

```text
per household: total_hours = sum(committed) + discretionary
```

This is never violated. One `u16` add per household per cadence tick — trivial at 100,000
households. The social-reproduction balance per district is the same identity summed
(design bible §7.3).

Time is a resource (CONFIRMED — Gordon & Klopov 1972; Szalai; CIA 1955). A shortage costs time
even when the good is eventually obtained. A citizen who spends two hours queuing for bread has
two fewer hours for everything else. The game makes this visible by tracking time allocation per
household and aggregating per district.

The design proposes that workers need to be housed, fed, heated, educated, transported, kept
healthy, and given time (design law 13). Each need is a link in the chain. A break in any link
degrades the labour force — not through a percentage modifier, but through the physical
consequence: no housing → overcrowding → turnover → understaffing → production shortfall.

## Current substrate

The social-reproduction chain does not exist in code. `HumanEnt` is one monolithic struct
(`simulation/src/world.rs:87-105`). `PersonalInfo` is `{name, age, gender}`; age never
increments. One need exists: bread via `BuyFood`. `Home` is a bare `BuildingID`. No household
entity exists (`grep -r Household simulation/` is empty). No time budget, no care obligations,
no healthcare, no education, no demographic events. Citizens are spawned with random age 20–50
and persist unchanged. Everything in this theme is greenfield
(Lane E, E-018…E-035, E-080…E-108).

## Research basis

Social reproduction as a framework for understanding Soviet economic performance is
CONFIRMED (Zaslavskaya 1988; Lane B1). Soviet planners understood the connection: the enterprise
served as a miniature welfare state precisely because housing, childcare, canteens, and clinics
were recognised as labour infrastructure (Filtzer 1994). The CIA's time-budget studies (1955,
1982) documented the household cost of consumer scarcity in quantitative terms.

The game's particular contribution is making this chain playable. The Planner sees the
social-reproduction balance as a dashboard and learns that building a factory without building
a district is building half a factory.

## Related

- [Queues](queues.md) — queue burden is a term in the social-reproduction balance.
- [Reliability](reliability.md) — retail reliability determines shopping time.
- [Adaptation](adaptation.md) — citizens adapt their schedules to social-reproduction pressure.
- [Phase lag](phase-lag.md) — demographic effects propagate with delay.
- [Society pages](../society/) — domain instances for housing, needs, labour, demographics.
- [Design bible §7](../../vision/design-bible.md) — citizens and households.
