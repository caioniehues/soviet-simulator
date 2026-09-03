# Labour — differentiated capacity

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** society
**Last verified:** 2026-09-03

| Scope | Label |
|---|---|
| Differentiated labour | Post-1.0 hook |
| Tenure ramp | Post-1.0 hook |
| Labour hoarding | Post-1.0 hook |
| Labour shortage as the socialist condition | Post-1.0 |

## What this is

Labour is the scarce factor in a socialist economy. The system does not face unemployment — it
faces worker shortage. An enterprise cannot buy labour; it must attract workers through
housing, services, and assignment. Labour is differentiated: twenty operators and one technician
are not interchangeable. A missing technician halves output even when twenty operators stand
idle.

## 1.0 requirement

The charter commits to "two education tiers" and "demographics including death," which imply
that workers carry qualifications and that the labour force shrinks through mortality. The
draft citizens specification (`SPEC-CITIZENS-003`) says a work assignment identifies both
citizen and destination and requires compatible capacity. `SPEC-CITIZENS-004` says labour
allocation is non-price.

No separate labour specification exists. Labour mechanics cross the citizens, education, and
production specifications.

## Target design

### Differentiated labour — PLAUSIBLE (bible §8.1)

The design proposes role-differentiated staffing: an enterprise recipe declares positions by
qualification category (e.g., 20 operators, 2 technicians, 1 inspector). Output is gated by
the scarcest role — a Liebig bottleneck. Qualification categories matter only where they
change allocation or production; the design does not propose personality or individual skill
curves.

The qualification taxonomy is an open question (bible §21.4).

### Tenure ramp — CONFIRMED (Feshbach & Rapawy 1973; ~17 %/yr turnover)

A new worker does not contribute at full productivity from the first tick. The design proposes
(B1 §3e):

```text
effectiveness = min(1.0, workplace_experience / ramp_complete_at)
```

A linear ramp is cheapest; logarithmic is more realistic but harder to test. The inspector
shows average worker effectiveness per enterprise — a high-turnover enterprise has persistently
low average effectiveness, and the Planner can diagnose why production is below quota without
a hidden "turnover" statistic.

Replacement workers in Soviet industry needed months to years to reach full productivity,
depending on skill level. Soviet studies documented turnover rates of 20–30 % annually in
some industries.

### Labour hoarding — CONFIRMED (Kornai ch. 11; 10–20 % surplus)

Enterprises with soft budget constraints hoard workers the way they hoard materials: as
insurance against absence, storming, and recruitment risk. An enterprise carrying 10–20 % more
workers than production technically requires is behaving rationally under unreliable supply.

The mechanism mirrors material over-requesting. The Planner sees the same discrepancy:
enterprise reports needing 120 workers but only 100 contribute to production at any time.
The observable is worker-to-output ratio, not a hidden "hoarding" flag.

### Turnover / tekuchest' — CONFIRMED (Feshbach; Cambridge Core)

Housing was the primary driver of voluntary turnover. Enterprises without housing could not
retain workers. The feedback loop: poor housing → worker departure → lost experience → reduced
output → inability to build housing.

CIA data (1987, "Strains in the Soviet Labor Force"): ~17 % of the workforce changed jobs
annually in the 1960s–70s. Enterprises with housing allocations had measurably lower turnover.

### Labour shortage as the socialist condition — CONFIRMED (B1-12; Kornai)

The socialist economy faces a structural shortage of workers, not jobs. The solutions the
Planner has are all physical: build housing to attract workers, build schools to create
qualified workers, improve transit to extend commute range, build childcare to release adult
hours, invest in mechanisation to reduce labour demand.

Each solution has a cost in other scarce resources. Building childcare requires construction
materials that compete with factory construction. Building housing requires the same. Every
labour solution is a physical trade-off.

## Current substrate

`raw_productivity` (`simulation/src/souls/goods_company.rs:82-85`) computes
`workers.len() / n_workers` — one undifferentiated headcount. Every worker contributes equally
from tick 1. There is no tenure ramp, no qualification requirement, no recruitment, no
migration, no turnover mechanic. `max_workers` is a per-prototype constant.

Workers are matched to enterprises through Market buy/sell of a `job-opening` item. Housing
has no effect on recruitment or retention.

## Research basis

- Kornai (1980), *Economics of Shortage*, ch. 11: labour hoarding under soft budget constraints.
- Feshbach & Rapawy (1973), "Labor constraints in the five-year plan," JEC: housing as the
  primary cause of turnover; replacement productivity ramps.
- Filtzer (1994), *Soviet Workers and De-Stalinization*: enterprise social obligations.
- CIA, "Strains in the Soviet Labor Force" (March 1987,
  CIA-RDP90T00114R000800010001-9): European USSR working-age population declining.
- B1 §3e designed the productivity ramp; B2 §3 calibration table provides the turnover rate.

## Open questions

- Which qualification categories does 1.0 require, and how do they map to the two education
  tiers? (Bible §21.4.)
- Should enterprises adapt their request_multiplier for labour hoarding, or is it a static
  prototype setting? (Same question as material hoarding.)
- At what rate does a new worker ramp? Linear, logarithmic, or stepped?

## Related

- [Workplaces](workplaces.md)
- [Education](education.md)
- [Housing](housing.md)
- [Citizens](citizens.md)
- [Migration](migration.md)
- [Time](time.md)
- [Scarcity concept](../concepts/scarcity.md)
