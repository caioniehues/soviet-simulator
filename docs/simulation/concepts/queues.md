# Queues

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** simulation
**Last verified:** 2026-08-28

Scope: **1.0 — charter row Households and citizens** — the charter commits to failure that
substitution, colder homes, and going without" (charter §Identity).

## What this is

A queue is a first-class scarcity object. When demand exceeds supply, demand waits. The wait is
observable, measurable, and consequential. Queues operate at different time scales across the
simulation:

| Domain | Time scale | Example |
|---|---|---|
| Retail | Minutes to hours | Citizens walk to a store, wait for stock, leave with or without goods. |
| Healthcare | Days to weeks | A clinic visit queues behind other patients; a hospital bed may wait days. |
| Housing | Years to decades | A family waits on the municipal list for a separate flat; 10+ years in Moscow was typical. |
| Dispatch | Minutes to hours | A freight job waits for an available truck. |
| Construction | Months | A site waits for material deliveries and labour. |
| Education | Months to years | A qualification requires school or institute enrolment and attendance. |

Each queue has age. Queue age is a measure of scarcity: a queue that is six months old
contains six months of unmet demand. Queue burden is the human time the queue consumes — a
citizen spending two hours at a bread store is two hours of life lost to scarcity, regardless of
whether they obtain the bread.

## 1.0 requirement

The charter requires that failure persists as a queue or going-without state, never as game
termination. The resources specification requires that unmet demand remains observable:

> A quantity cannot be silently deleted or created during a failed request, reservation,
> transfer, or consumption. Unmet demand remains an observable queue, substitutes, or a
> going-without outcome.
> — [`SPEC-RESOURCES-005`](../../reference/specifications/resources.md#spec-resources-005)

The logistics specification requires that a missing truck or route produces a visible stalled
job, not silent deletion
([`SPEC-LOGISTICS-004`](../../reference/specifications/logistics.md#spec-logistics-004)).

## Target design

The design proposes that queues exist as explicit data structures, not as implicit side effects
of unmatched orders. Each queue type records membership, entry tick, priority (where the domain
defines priority classes), and reason. Queue membership is observable by the Planner.

The housing queue is the most consequential for the player: it determines which households gain
housing and which continue in overcrowded or communal conditions. The design proposes
(Lane B1 §3b) a `BTreeMap` keyed by (channel, priority, registration tick), with three
channels matching historical practice: enterprise allocation (~2 years), municipal list
(10+ years), and cooperative (PLAUSIBLE — Lane B1).

Queue burden is measured in human time: the total hours citizens spend waiting per district per
period. A stacked bar per district shows which sink is consuming household time
(design bible §7.3).

## Current substrate

`BuyFood` has a `WaitingForTrade` state that functions as a retail queue:
citizens walk to a matched seller and wait. But there is no explicit queue data structure;
matching is global by distance. No clinic, housing, education, or dispatch queue exists as a
named data structure in `simulation/src/`. Unmatched external buy orders can be erased by
`mem::take` when no freight station exists
([`ECO-SUB-001`](../../research/fact-sheets/wave1-economy.md#eco-sub-001--unmatched-demand-is-not-a-durable-queue)).

## Research basis

Queuing for consumer goods is among the most documented features of Soviet daily life. The CIA
(1955) estimated 33.65 hours per week for a 1954 Moscow food basket (Lane B2 calibration
table). Housing queues of 10+ years are documented by Andrusz (1984) and Morton (1980), with
12–36% of families on waiting lists. The queue is not a failure state — it is the normal
allocation mechanism under scarcity.

## Related

- [Scarcity](scarcity.md) — queues are the primary clearing mechanism.
- [Reserves](reserves.md) — reserves reduce queue pressure.
- [Reliability](reliability.md) — unreliable delivery lengthens queues.
- [Phase lag](phase-lag.md) — queue length responds with delay to supply changes.
- [Social reproduction](social-reproduction.md) — queue burden consumes household time.
- [Allocation](../planned-economy/allocation.md) — dispatch queues under deficit-first policy.
