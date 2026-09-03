# Simulation philosophy

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** project lead
**Last verified:** 2026-08-28

Two rules decide what gets simulated and how.

## Cheapest representation that preserves every causal distinction

> **Use the cheapest representation that preserves every causal distinction the game cares about.**

Deep simulation where distinctions change decisions; aggressive aggregation where they do not.
"As much detail as the engine allows" is the wrong upper bound — it produces *Copper Bolt Type 7
Simulator*. The test for any split (of a resource, a lot, a vehicle operation, a production
stage, a quality class, an institutional state) is: does the distinction change routing, storage,
substitution, a bottleneck, allocation priority, quality, timing or a visible consequence? If not,
aggregate. Gosplan balanced roughly 1,943 product categories, not millions of SKUs; the game's
fifteen-resource catalogue lives at about that level ([resources](../simulation/physical-economy/resources.md)).

## Every macro number resolves into physical or institutional state

> **Every important macroeconomic number must eventually resolve into physical or institutional state.**

A steel shortfall resolves into specific missing inputs, labour, power, water, storage, routes or
work. A cold apartment resolves into a heating chain. A worker shortage resolves into people,
qualifications, housing, commute, care obligations, illness, turnover or assignment. This is why
the [material balance](../simulation/planned-economy/material-balance.md) drills to holders and
hauls, and why the [causal inspector](../plan/proposals/causal-inspector.md) is architecture, not
UI polish.

## Model inertia, not generic capacity

Infrastructure is not a set of generic capacity graphs. A truck has mechanical inertia; a road
has queue inertia; a railway has braking-distance and timetable inertia; water has pressure and
tank storage; sewage has gravity and backpressure; heating has transport delay and thermal mass;
electricity balances almost instantly; gas has linepack. Each network's delay signature gives the
player a distinct failure mode and a distinct instrument ([infrastructure](../simulation/infrastructure/index.md),
[phase lag](../simulation/concepts/phase-lag.md)).

## Prefer physical chains to percentage modifiers

Not `kindergarten +5 % workforce`, `bad housing −10 % productivity`, `snow −20 % traffic`. Instead:
a childcare place releases specific care hours, so an adult can attend a shift; snow changes safe
braking distance, so headway rises, so capacity falls. Modifiers hide causes; chains expose them
to the inspector and to the player ([design laws](design-laws.md) §22 of the design thread lists
this as the first anti-pattern).

## The republic must be allowed to work

Historical problems — storming, hoarding, shortage, reporting distortion, turnover — arise from
incentives and physical constraints in this design, never from a hidden rule that central planning
must fail. The baseline is "functional but constrained" (jobs, small flats, monotonous food, basic
healthcare, good schools, adequate transit); crises emerge from plan failures. A game that shows
only dysfunction is as false as one that shows none ([research methodology](../research/methodology.md)).

## The five states of knowledge

This philosophy is a design stance. What 1.0 *requires* is the charter. What the target
architecture *proposes* is the architecture handbook. What the code *implements* is the current
substrate page. What research *suggests* is the research section. The documentation keeps them
apart on purpose ([document authority](../meta/document-authority.md)).

## Related

- [Design laws](design-laws.md)
- [Concepts index](../simulation/concepts/index.md)
- [Target architecture](../architecture/target-architecture.md)
- [Research methodology](../research/methodology.md)
