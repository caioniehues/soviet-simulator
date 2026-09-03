# Enterprise behavior

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** economy
**Last verified:** 2026-08-28

Scope: **1.0 binding** — the dishonest enterprise is the game's core loop. The charter commits
to inspectable discrepancies and persistent observable state.

## What this is

An enterprise's request is a strategic statement, not a truthful report of need. An enterprise
consuming 100 tonnes of copper may report a requirement of 135 tonnes. The seven reasons for
inflating a request are all CONFIRMED (Kornai 1980; Berliner 1957):

1. **Reliability buffer** — past deliveries have been incomplete; request more to get enough.
2. **Expected allocation cuts** — the Plan typically allocates less than requested; inflate to
   compensate.
3. **Plan-risk buffer** — the quota may rise next period; hold stock against future obligation.
4. **Safety stock** — maintain a minimum buffer against any disruption.
5. **Maintenance reserve** — parts and materials for equipment repair.
6. **Expected storming** — end-of-period demand spikes require extra input.
7. **Fear of shortage** — even without a specific reason, experience of scarcity drives
   hoarding.

The dishonest enterprise is the core gameplay loop. The enterprise requests more than it
consumes, hoards the surplus, and the player — acting as THE PLANNER — catches it from
observable state. There is no hidden verdict, but there is rich hidden state.

## 1.0 requirement

The production specification binds the observable discrepancy:

> For each input and accounting period, an enterprise MAY report a requirement above the
> recipe's actual consumption. Logistics allocates by its ordinary shortage rules, not an
> honesty label; receipts become physical on-hand stock through Resources and excess after
> consumption remains accountable surplus, so an honest competitor MAY wait.
> — [`SPEC-PRODUCTION-009`](../../reference/specifications/production.md#spec-production-009)

> The Planner SHALL infer suspected deception from inspectable request, receipt, consumption,
> on-hand, surplus, and outstanding-request-age discrepancies; no authoritative `dishonest`
> flag may replace those observations.
> — [`SPEC-PRODUCTION-009`](../../reference/specifications/production.md#spec-production-009)

The eight-state chain that makes the discrepancy visible:

```text
technical forecast → reported requirement → allocated → reserved →
received → consumed → surplus/on-hand → outstanding request age
```

## Target design

The design proposes rich hidden *state* with no hidden *verdict*. An enterprise that inflates
requests has reasons — remembered reliability, reserve targets, bargaining history — and those
live in the simulation, hidden from the Planner's view, not absent. The Planner sees the
discrepancy, never the reason (SYNTHESIS §6; Lane G-09 resolution).

### Enterprise intent model (Lane A §3a)

```text
per enterprise:
  reliability_memory: f32      // EMA of fulfillment_rate
  fulfillment_rate: f32        // received / requested over last N cycles
  effective_multiplier: f32    // base_multiplier / max(reliability_memory, floor)
  request_age: u32             // ticks since last fulfilled delivery per input
```

After each recipe cycle, `fulfillment_rate` updates from the ratio of received to requested.
`reliability_memory` blends with exponential decay. Low reliability raises the effective
multiplier. This is one instance of the general
[reliability → defensive buffering](../concepts/reliability.md) concept.

### Missed behaviours (Lane A §4) — all CONFIRMED historically, all ABSENT

The dishonest enterprise has forms beyond simple request inflation:

- **Tolkachi / expediters** (Berliner, from 1937): a worker leaves production to physically
  chase inputs through personal contacts. A second allocation topology. A visible cost
  (labour diverted from production). Post-1.0.
- **Ministries as inflating aggregators**: real planning was Gosplan → ministry → enterprise.
  A "dishonest ministry" aggregates and is harder to catch. Post-1.0.
- **Investment hunger** (Kornai): enterprises request buildings they cannot staff. A distinct
  dishonest pattern — capital dilution. Post-1.0.
- **Assortment evasion**: aggregate quota met with the easy mix; harder items neglected. The
  Planner must inspect mix, not only volume. Post-1.0.
- **Plan-fulfilment falsification** (Harrison 2011; CIA grain overstatement up to 53 %):
  reported output decoupled from physical stock change. The inspector compares reported
  production against physical stock. See [reports and information](reports-and-information.md).
- **OTK quality attestation**: a quality gate on output makes storming costly through rework
  that consumes more inputs. Post-1.0.

## Current substrate

The seed exists and is wired end-to-end. `request_multiplier` is a static `i32` on the `Recipe`
prototype (`prototypes/src/types/recipe.rs:52`), set to 4 for `flour-factory` and 3 for
`slaughterhouse` (`base_mod/companies.lua:40,582`), defaulting to 1 for all others.

`recipe_init` (`simulation/src/souls/goods_company.rs:22-26`) calls
`market.set_requested(soul, item.id, qty)` where `qty = item.amount * request_multiplier`.
This is proven by `SCENARIO-0151` and `sov-lpj`.

The multiplier is static: no `reliability_memory`, `fulfillment_rate`, or equivalent state
exists in `GoodsCompanyState` (`simulation/src/souls/goods_company.rs:69-78`). The enterprise
cannot learn or adapt.

**The Planner cannot see it.** `Market::requested()` (`market.rs:77-78`) is a public accessor.
No code in `native_app/` calls it. The building inspector (`inspect_building.rs:244-267`)
shows `capital` per item only — not `requested`, `consumed`, `reserved`, or `surplus`.

Wiring `Market::requested()` into `inspect_building.rs` is approximately 30 lines and is the
single cheapest high-value change in the project (Lane E §3). It would give the Planner the
ability to catch the first dishonest enterprise *today*.

## Open questions

- Adaptive multiplier or Planner-set request limits? (Lane A, question 1)
- How many reserve classes? (Lane A, question 4)
- Ministry layer ever? (Lane A, question 5)

## Related

- [Reliability and buffering](reliability-and-buffering.md) — the spiral that drives
  enterprise inflation.
- [Reports and information](reports-and-information.md) — falsification as a distinct behaviour.
- [Reserves](reserves.md) — the hidden surplus that the Planner must detect.
- [Storming](storming.md) — storming as an enterprise temporal response.
- [Plan cycle](plan-cycle.md) — where enterprises fit in the control loop.
- [Reliability](../concepts/reliability.md) — the general concept.
- [Production specification](../../reference/specifications/production.md) — SPEC-PRODUCTION-009.
- [Design bible §5.1](../../vision/design-bible.md) — requests as strategic statements.
