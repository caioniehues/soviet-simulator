# Allocation

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** economy
**Last verified:** 2026-09-03

Scope: 1.0 — charter row Transport and border — the logistics specification commits to
non-price dispatch ordering
([`SPEC-LOGISTICS-005`](../../reference/specifications/logistics.md#spec-logistics-005)).

## What this is

Allocation is how goods reach their destination without price. In a market economy, price
clears supply and demand. In this planned economy, allocation clears by policy, deficit,
distance, and queue. The Planner's allocation instruments are:

- **Target-stock policy** — per storage bucket, the Planner sets a minimum and maximum stock
  level. Demand below minimum generates a request; supply above maximum generates an offer.
- **Deficit-first dispatch** — the dispatcher orders compatible demands by greatest normalised
  target deficit first, then meaningful route distance, then stable identity tie-break. No
  domestic money or price participates.
- **Substitution chains** — when the preferred input is unavailable, an approved substitute is
  used. Substitution is an explicit action with a penalty and a traceable chain. A missing →
  substitute B → B's allottee substitutes C. The chain propagates (Lane A, M-07, CONFIRMED).
- **Rationing regimes** — Post-1.0. When scarcity is severe, the Planner may impose rationing
  rules (per-household quotas, coupon systems). These are institutional instruments, not
  price instruments.

The soft-budget-constraint physical analogue (Lane A, M-02, CONFIRMED — Kornai): never-game-over
means the Planner rescues failing enterprises by taking from performing ones. The reliable
are punished. The design proposes making this tension explicit: the Planner's rescue of failing
enterprises degrades the reliable ones, and the reliable ones respond by hoarding more.

## 1.0 requirement

The logistics specification commits:

> Planner target-stock minima and maxima per storage bucket create demand below minimum and
> supply above maximum. Compatible demands are ordered by greatest normalized target deficit
> first, then meaningful route distance and a stable identity tie-break; money and price
> never participate.
> — [`SPEC-LOGISTICS-005`](../../reference/specifications/logistics.md#spec-logistics-005)

The production specification commits:

> Production MUST NOT debit domestic money or use domestic price clearing as a gate.
> — [`SPEC-PRODUCTION-004`](../../reference/specifications/production.md#spec-production-004)

## Target design

The design proposes that the allocation system operates in three stages:

1. **Demand computation** — for each storage bucket, compare current stock against target-stock
   policy. Below minimum generates a request with deficit = (minimum - current) / minimum as
   the normalised urgency.

2. **Matching** — sort all compatible demands by (deficit descending, distance ascending,
   stable ID ascending). Match to available supply. No money, no price, no auction.

3. **Dispatch** — create a physical haul for each match. The truck or train carries the goods
   from source to destination under the custody contract defined by the logistics specification.

Target-stock policy gives the Planner a lever without giving them direct allocation power. The
Planner sets policy; the dispatcher executes it. This is "automate execution, not decisions"
applied to allocation.

## Current substrate

`make_trades` (`simulation/src/economy/market.rs:511-551`) matches supply and demand by
distance only. The scoring function is `sorder.pos.distance2(border.pos)` (`market.rs:537`) —
squared Euclidean distance between seller and buyer positions. Potential trades are sorted by
this score using `sort_unstable_by_key` with `OrderedFloat` (`market.rs:550-551`).

No target-stock policy, no deficit-first ordering, no request age, no plan priority, and no
substitution exists. Matching has no partial multi-seller fill: one seller must cover the
buyer's full quantity.

The domestic `money_delta` is `Money::ZERO` (`market.rs:544`), which is consistent with
non-price clearing. But `Government.money` debits for construction and wages elsewhere,
conflicting with SPEC-PRODUCTION-004.

## Open questions

- What normalised deficit scale preserves the required deficit-first ordering across resource
  kinds with different units? (logistics specification, open question 1)
- Substitution chains: which substitutions are legal for each resource? (resources
  specification, open question 2)

## Related

- [Scarcity](../concepts/scarcity.md) — allocation is the mechanism that clears scarcity.
- [Priorities](priorities.md) — priority classes can modify the deficit ordering.
- [Physical causality](../concepts/physical-causality.md) — allocation does not teleport stock.
- [Enterprise behavior](enterprise-behavior.md) — enterprises adapt their requests to
  allocation outcomes.
- [Plan cycle](plan-cycle.md) — allocation policy is a Plan instrument.
- [Logistics specification](../../reference/specifications/logistics.md).
- [Design bible section 5.8](../../vision/design-bible.md).
