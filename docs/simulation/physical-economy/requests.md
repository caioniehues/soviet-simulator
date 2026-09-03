# Requests

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** economy
**Last verified:** 2026-08-28

| Scope | 1.0 binding |

## What this is

A request is a stated demand for a physical resource. It is distinct from allocation,
reservation, receipt, consumption, and stock on hand. When an enterprise needs cereal,
it places a request. That request persists — with age — until it is allocated, substituted,
or becomes a going-without outcome. The request is the first state in the
[physical sequence](index.md).

The dishonest enterprise exploits the gap between request and consumption: a plant that
truly needs 100 t of cereal requests 400 t. The surplus accumulates as visible on-hand
stock. The Planner catches it by comparing requested, received, consumed, and on-hand
quantities — not by reading a hidden flag.

## 1.0 requirement

`SPEC-PRODUCTION-003` — requested, received, consumed, on-hand, reserved, in-custody,
and surplus quantities are distinct. A reported request is never proof of consumption.

`SPEC-PRODUCTION-005` — a blocked producer retains its unmet input request or records
its substitute/going-without result; it MUST NOT silently delete demand, inputs, or
outputs.

`SPEC-PRODUCTION-009` — an enterprise MAY report a requirement above the recipe's actual
consumption. Logistics allocates by its ordinary shortage rules, not an honesty label.

`SPEC-RESOURCES-005` — unmet demand remains an observable queue, substitutes, or a
going-without outcome.

## Target design

Durable unmet demand with age and partial fulfillment. The request persists across ticks
until satisfied, substituted, or recorded as going-without. Request age is a planning
signal: long-standing unmet demand points to structural shortage (PLAUSIBLE, bible §5).

## Current substrate

`SingleMarket.buy_orders` (`simulation/src/economy/market.rs`) stores buy orders as
`BTreeMap<SoulID, BuyOrder>`. An unmatched buy order is removed by `make_trades` when
the external fallback takes it (`ECO-SUB-001`): without a freight station, the order
disappears. This is a live violation of `SPEC-RESOURCES-005`.

`SingleMarket.requested` stores the per-soul requested quantity.
`Market::set_requested` is called from `recipe_init` in
`simulation/src/souls/goods_company.rs:23-24`, so the dishonest-enterprise over-request
is wired in the running game. But no UI reads `Market::requested()` — the Planner
cannot observe the discrepancy (`ECO-SUB-005`, stale since `0caee71` for the sim side;
the observability half still stands).

## Open questions

- How does request age interact with the allocation priority rule
  (`SPEC-LOGISTICS-005`: deficit → distance → stable ID)?
- Does a retail (citizen) request carry the same age semantics as an enterprise request?

## Related

- [Allocation](allocation.md)
- [Resources](resources.md)
- [Storage](storage.md)
- [Enterprise behavior](../planned-economy/enterprise-behavior.md)
- [Production spec](../../reference/specifications/production.md)
- [Resources spec](../../reference/specifications/resources.md)
- [Glossary — Request](../../reference/glossary.md)
