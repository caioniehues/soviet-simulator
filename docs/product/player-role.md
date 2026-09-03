# The Planner

**Kind:** concept
**Authority:** advisory (the role itself is a charter pillar)
**Status:** draft
**Owner:** project lead
**Last verified:** 2026-08-28

The player is **THE PLANNER** — the glossary's word, and the only one. Not mayor, not
entrepreneur, not market participant.

## What the Planner does

- Authors the **Plan**: a sequence of quota periods on one continuous save (glossary).
- Sets **quotas** — state-set production, provision or housing requirements per period.
- Sets **priorities** and allocation policy, knowing that priority decides *where* scarcity
  appears, never whether it exists ([priorities](../simulation/planned-economy/priorities.md)).
- Places roads, buildings and construction Sites; every placement gets a **ghost**, a **verdict**
  and, if refused, a physical **refusal** reason (glossary; SPEC-CONSTRUCTION).
- Holds and releases **reserves**, and decides whether to confiscate an enterprise's hidden
  surplus at the cost of its trust ([reserves](../simulation/planned-economy/reserves.md)).
- Reads reports, dashboards, inspections and measurements — and learns which are credible
  ([reports and information](../simulation/planned-economy/reports-and-information.md)).
- Catches the **dishonest enterprise** from observable discrepancies, never from a flag
  ([enterprise behaviour](../simulation/planned-economy/enterprise-behavior.md)).

## What the Planner does not do

- Buy, sell or price domestic goods. Domestic clearing is non-price ([scarcity](../simulation/concepts/scarcity.md)).
- Drive trucks, assign every worker, or click every delivery. **Automate execution, not decisions.**
- See physical truth directly. The Planner sees the Planner World, one of
  [four realities](../simulation/concepts/information.md).
- Lose. There is no game over. The Planner's failures are queues, shortages, colder homes and
  going without — visible, persistent, and recoverable with delay ([phase lag](../simulation/concepts/phase-lag.md)).

## The Planner's instruments (design)

The design thread proposes these as the Planner's surfaces; the [causal inspector
proposal](../plan/proposals/causal-inspector.md) and [observatory](../architecture/observatory.md)
carry the detail:

- the **material balance** per resource, every line drillable to holders, hauls, consumers or reports;
- the **causal inspector** — STATUS / CAUSE / TREND / POLICY / PHYSICAL CHAIN for any object, with
  a provenance label on every value;
- **pressure maps** — freight queue, road spillback, rail utilisation, housing queue, service
  burden, water pressure, heating reserve, curtailment, social time loss;
- **reserves in natural units** — "coal bunker: 18 h at current burn";
- **notifications from causal state** — request inflation rising, repeated period-end storming,
  a tank on a depletion trajectory.

None of these exist today. The building inspector shows workers, productivity, power, progress
and storage; nothing causal ([current substrate](../architecture/current-substrate.md)).

## What the Planner experiences over time

Early: a fragile local economy where one late coal train cascades into cold flats. Middle: an
integrated republic whose enterprises still hoard because delivery has been unreliable. Late: a
calm republic — the Planner's reward for reliability is that the whole system needs less slack,
and national projects become possible without tearing society apart ([vision](vision.md)).

## The tutorial problem

The charter requires that the First Plan alone teach a new player to play for two hours without
outside help. That is a design problem about this role: the dishonest-enterprise loop must be
learned through play, from observable discrepancies pointed at by the HUD strip, not from
exposition. Recorded as an open design item in [game modes](game-modes.md).

## Related

- [Vision](vision.md)
- [Design laws](design-laws.md)
- [Glossary](../reference/glossary.md) — Planner, Plan, Quota, Tranche, Ghost, Verdict, Refusal
- [Plan cycle](../simulation/planned-economy/plan-cycle.md)
- [Information](../simulation/concepts/information.md)
