# Scarcity

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** economy
**Last verified:** 2026-08-28

Scope: **1.0 — charter row Households and citizens** — non-price domestic clearing is a charter
(charter §Identity, lines 31–33).

## What this is

The domestic economy clears without price. When demand exceeds supply, the simulation resolves
the gap through queue, allocation policy, substitution, and going without. No domestic money
gates access. Priority relocates scarcity from one user to another; it does not create supply.

The rouble is a single foreign currency used only at the border. It settles imports and exports
at fixed per-kind prices through physical customs clearance. Inside the economy, it has no
clearing function.

This is the economic law that makes the game a planned-economy simulator rather than a market
simulator. The Planner's tools are quotas, priorities, allocation policies, construction
programmes, reserves, and institutional rules. The Planner does not set prices.

## 1.0 requirement

The charter states:

> Domestic clearing uses queue, allocation, substitution, and going without, never price.

> The rouble is a single foreign currency used only at the border.

The trade specification commits:

> Domestic matching, allocation, reservation, dispatch, production, and consumption MUST NOT
> debit, credit, rank by, or otherwise clear through roubles.
> — [`SPEC-TRADE-001`](../../reference/specifications/trade.md#spec-trade-001)

The production specification commits:

> Production MUST NOT debit domestic money or use domestic price clearing as a gate.
> — [`SPEC-PRODUCTION-004`](../../reference/specifications/production.md#spec-production-004)

## Target design

The design proposes four clearing mechanisms that operate without price:

1. **Queue** — demand waits in a durable ordered queue until supply arrives. Queue membership
   is observable; queue age is a measure of scarcity.
2. **Allocation policy** — the Planner sets target-stock minima and maxima, dispatch priorities,
   and reserve policies. Dispatch orders by largest normalised deficit first, then meaningful
   route distance, then stable ID tie-break
   ([`SPEC-LOGISTICS-005`](../../reference/specifications/logistics.md#spec-logistics-005)).
3. **Substitution** — when the preferred input is unavailable, an approved substitute is used.
   Substitution is an explicit action with a penalty and a traceable chain. Forced-substitution
   chains propagate: A missing → substitute B → B's allottee substitutes C (Lane A, M-07).
4. **Going without** — an explicit unmet-need outcome under scarcity. It is a simulation state,
   not game termination. (glossary)

Priority decides **where** scarcity appears, not whether it exists. Copper to the Space
Programme is copper not in radios, machine tools, or construction. The Planner sees the
displaced use. Priority inflation — when everyone labels a request critical — makes priority
meaningless; the design proposes constraining who assigns priority classes and exposing the
share of activity under emergency status (design bible §5.8, HYPOTHESIS).

## Current substrate

Domestic `money_delta` is `Money::ZERO` for internal trades
(`simulation/src/economy/market.rs:584`), which is consistent with non-price clearing.
Matching sorts by distance (`market.rs:577–591`) with no partial multi-seller fill, no request
age, and no plan priority
([`ECO-SUB-003`](../../research/fact-sheets/wave1-economy.md#eco-sub-003--domestic-matching-is-price-free-but-not-queue-clearing)).

However, `Government.money` debits for roads, buildings, trains, and worker wages
(`government.rs:22-75`, `economy/mod.rs:54`, `world_command.rs:225`). Money can go negative
(not a hard gate), but it is a price-like cost in a non-price domain
([`ECO-SUB-004`](../../research/fact-sheets/wave1-economy.md#eco-sub-004--the-inherited-treasury-still-prices-domestic-actions)).

## Research basis

The shortage economy is Kornai's (1980) central framework. Scarcity resolves through
administrative allocation, queuing, rationing, connections, and going without — never through
price equilibrium in the domestic sphere. The rouble existed inside Soviet enterprises but did
not clear goods the way prices do in a market economy; it served as a unit of account for
planning aggregation. This game simplifies to one foreign-only rouble, consistent with the
charter.

## Related

- [Physical causality](physical-causality.md) — rule 5: no domestic price clearing.
- [Queues](queues.md) — the primary clearing mechanism under scarcity.
- [Reserves](reserves.md) — reserve policy as a Planner tool against scarcity.
- [Priorities](../planned-economy/priorities.md) — priority relocates scarcity.
- [Allocation](../planned-economy/allocation.md) — how allocation clears without price.
- [Trade specification](../../reference/specifications/trade.md) — border settlement.
