# Phase lag

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** simulation
**Last verified:** 2026-08-28

Scope: **architecture hook** — the design proposes trajectory management as a Planner skill;
no specification commits to specific lag models.

## What this is

Physical systems carry momentum. A disruption does not produce an instant failure; reserves,
buffers, thermal mass, tank volume, linepack, rail slack, and household pantries absorb the
shock and delay its arrival. Recovery is equally slow: a restored coal supply does not
instantly warm a district; the hot water must travel through the pipe network, arrive at
buildings, and raise interior temperatures through thermal mass.

Both disruption and repair propagate with delay. The delay is the mechanic. A taut system with
two days of reserve can realise less than a resilient system with twelve, because one disruption
cascades before the Plan can respond. Slack is physical resilience, not inefficiency
(design bible §5.9, CONFIRMED).

## 1.0 requirement

No specification commits to specific lag times or trajectory models. The charter requires that
failure persists as a visible state (never instant recovery), which implies lag. The utility
specifications (heating, water, electricity) each define their own inertia characteristics
in draft form.

## Target design

The design proposes that the Planner manages **trajectories**, not red icons. The relevant
trajectory variables are:

- **Current stock** — what exists right now.
- **Consumption rate** — how fast it is being used.
- **Incoming supply** — deliveries in transit, production in progress.
- **ETA** — when the next supply arrives.
- **Time to depletion** — at the current rate, when does the reserve reach zero?

A Planner who sees "coal bunker: 18 hours at current burn; next train: 22 hours away" can act
before the failure materialises. A Planner who sees only a green/red icon acts after it is too
late.

Each network exhibits a different form of inertia (design bible §10):

| Network | Inertia character |
|---|---|
| Electricity | Near-instant balance; curtailment is immediate. |
| Water | Pressure and tank storage; tank drains before service fails. |
| Sewage | Gravity and backpressure; full downstream buffer restricts upstream. |
| Heating | Transport delay and thermal mass; cold flats arrive hours after coal stops. |
| Gas | Linepack; supply shortfall hides while linepack drains, then collapses. |
| Logistics | Vehicle fleet and route time; rerouting takes hours. |
| Society | Household pantries and adaptations; queues lengthen gradually. |

"Different systems respond at different speeds — that delay is the mechanic"
(design bible §11).

## Current substrate

No explicit lag or trajectory model exists in the simulation. Electricity blackout is
immediate and binary: consumed > produced → blackout
(`simulation/src/map_dynamic/electricity.rs:43-93`). No other utility network exists. No
reserve tracking, no depletion forecast, no time-to-failure computation is present in
`simulation/src/`.

## Research basis

Phase lag is a first-principles consequence of physical conservation. Every stored quantity
(thermal mass, tank volume, coal bunker, linepack, rail wagon slack) delays the effect of a
supply change. Soviet economic planning operated under these physical lags — a coal train
arriving late at a power station did not produce an immediate blackout because boiler reserves
absorbed the delay. The cascade failure came when reserves depleted faster than resupply. This
is the subject of the design bible's cross-system causal loops (§11).

## Related

- [Reserves](reserves.md) — reserves determine how long the system withstands disruption.
- [Reliability](reliability.md) — the reliability spiral operates over lag timescales.
- [Queues](queues.md) — queue length responds with delay to supply changes.
- [Storming](../planned-economy/storming.md) — storming creates demand pulses that propagate
  with lag through the freight system.
- [Design bible §10–§11](../../vision/design-bible.md) — network inertia and cross-system loops.
