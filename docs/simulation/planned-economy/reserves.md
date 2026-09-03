# Reserves (planned economy)

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** economy
**Last verified:** 2026-09-03

| Scope | Label |
|---|---|
| Physical stock and inspectable surplus | 1.0 — charter Resources and production row |
| Five-class custody-state model | Post-1.0 hook |

The charter requires physical stock and inspectable surplus. The five-class decomposition is a
PLAUSIBLE design proposal (Lane A, section 3e), deferred as a hook by
[ADR-0001](../../decisions/0001-households-and-utilities-are-1.0-scope.md).

## What this is

In the planned-economy domain, reserves are custody states that sum to physical stock. The
five-purpose taxonomy proposed by the design thread decomposes a single stock number into
distinct buckets with different access rules:

| Class | Access rule | Planner visibility |
|---|---|---|
| **Operating** | Consumed automatically by recipe_act | Full |
| **Safety** | Drawn when operating is depleted (recorded event) | Full |
| **Enterprise** | Never drawn automatically — the hoard | Inferrable from inspection |
| **State** | Moved only by Planner action (confiscation, reallocation) | Full |
| **Project** | Moved only by a national-project system | Full |

The conservation invariant:
`operating + safety + enterprise_reserve + state_reserve + project_reserve == physical_stock`

None can go negative. Transfer between classes is an explicit action.

This is the domain instance of [reserves](../concepts/reserves.md). The concept page explains
the general idea and the network-reserves table; this page applies it to the planned economy.

## 1.0 requirement

The production specification requires that surplus remains visible
([`SPEC-PRODUCTION-003`](../../reference/specifications/production.md#spec-production-003))
and that the Planner can infer deception from stock discrepancies
([`SPEC-PRODUCTION-009`](../../reference/specifications/production.md#spec-production-009)).
No specification commits to the five-class breakdown.

## Target design

The design proposes (Lane A, section 3e) that `recipe_act` consumes from operating first. When
operating is depleted, the enterprise draws from safety — with a credibility penalty, because
drawing safety stock signals that the supply chain failed. The enterprise reserve is never drawn
automatically; it is the enterprise's hidden surplus, the hoarding behaviour the Planner must
detect.

The Planner can observe the enterprise reserve if they inspect closely: physical stock minus
operating, safety, state, and project equals the hidden surplus. The enterprise's own report
omits it. This is SPEC-PRODUCTION-009 made concrete.

### Confiscation

Confiscation is a Planner act with a credibility cost. Seizing enterprise reserves degrades
planning credibility (see [reliability and buffering](reliability-and-buffering.md)), which
increases future hoarding. The Planner must weigh the immediate benefit (redistributed stock)
against the long-term cost (reduced trust, increased inflation of future requests).

## Current substrate

`SingleMarket` tracks `capital` (on-hand as `i32`), `reserved` (matched but not yet picked up
as `u32`), and `requested` (declared need as `u32`)
(`simulation/src/economy/market.rs:39-53`). There is no distinction between types of on-hand
stock. A single `i32` represents all stock at an enterprise.

The storage-capacity floor on hoarding is CONFIRMED in code. `recipe_should_produce`
(`simulation/src/souls/goods_company.rs:44-47`) refuses to buy when
`capital - reserved >= amount * (storage_multiplier + 1)`. An enterprise cannot hoard what it
cannot store.

## Open questions

- Five reserve classes or three? The essential class is the hidden enterprise reserve. Two
  (operating + enterprise) or three (operating + safety + enterprise) may carry the core loop.
- How does confiscation interact with the storage cap? Confiscated stock must go somewhere
  physical.

## Related

- [Reserves](../concepts/reserves.md) — the general concept and network-reserves table.
- [Enterprise behavior](enterprise-behavior.md) — hoarding as a strategic act.
- [Reliability and buffering](reliability-and-buffering.md) — confiscation and credibility.
- [Material balance](material-balance.md) — stock terms decompose into reserve classes.
- [Physical causality](../concepts/physical-causality.md) — conservation invariant.
- [Production specification](../../reference/specifications/production.md).
