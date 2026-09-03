# Reliability

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** simulation
**Last verified:** 2026-08-28

Scope: **1.0 binding** — the shortage spiral is the game's core mechanic. The charter commits
to the dishonest-enterprise loop, which is the most visible instance of this concept.

## What this is

Unreliable delivery causes defensive buffering. Defensive buffering reduces central
availability. Reduced availability causes further unreliability. The spiral is
self-reinforcing:

```text
unreliable delivery → actors raise buffers and requests
  → central availability falls → others face shortage
    → more buffering → dispatch pressure rises
      → delivery less reliable
```

This is the foundational mechanism of the shortage economy (Kornai 1980, CONFIRMED).

The spiral runs in both directions. The positive spiral is equally real:

```text
reliable delivery → actors lower buffers and requests
  → stock released → emergency dispatch falls
    → congestion falls → delivery more reliable
```

A mature republic is physically calmer. Success is **coordination quality**: smaller emergency
reserves, fewer emergency dispatches, shorter queues, lower plan-period variance, more reliable
deliveries, lower turnover, less overtime, more accurate reports, more household discretionary
time (design bible §1).

The concept applies to every actor in the simulation, not only enterprises:

| Actor | Buffer form | Trigger |
|---|---|---|
| Enterprise | Input stock, labour hoarding, local workshops | Unreliable input delivery |
| Household | Food pantry, social contacts, plot harvest | Unreliable retail supply |
| Hospital | Medicine stockpile, referral delays | Unreliable medicine delivery |
| Wagon fleet | Spare vehicles held in reserve | Unreliable dispatch scheduling |
| Construction site | Material overstock, idle crews | Unreliable material delivery |

Each domain page names its own reliability instance and links here.

## 1.0 requirement

The production specification binds the observable discrepancy that makes the spiral legible:

> An enterprise MAY report a requirement above the recipe's actual consumption. ... The
> Planner SHALL infer suspected deception from inspectable request, receipt, consumption,
> on-hand, surplus, and outstanding-request-age discrepancies.
> — [`SPEC-PRODUCTION-009`](../../reference/specifications/production.md#spec-production-009)

## Target design

The design proposes that reliability drives request inflation adaptively (Lane A §3a,
PLAUSIBLE):

```text
per enterprise:
  reliability_memory: f32   // EMA of fulfillment_rate
  fulfillment_rate: f32     // received_qty / requested_qty over last N cycles
  effective_multiplier: f32 // base_multiplier / max(reliability_memory, floor)
  request_age: u32          // ticks since last fulfilled delivery per input
```

After each recipe cycle, `fulfillment_rate` updates from the ratio of received to requested.
`reliability_memory` blends with exponential decay. Low reliability raises the effective
multiplier; high reliability lowers it. The Planner sees `requested` vs `received` vs
`consumed` per input per cycle, plus `reliability_memory` as a bar. A discrepancy between
`requested` and `consumed` is the Planner's primary hoarding signal.

### The physical floor

The spiral has a physical floor. `recipe_should_produce` refuses to buy above
`amount * (storage_multiplier + 1)` (`simulation/src/souls/goods_company.rs:44-47`,
CONFIRMED in code). An enterprise cannot hoard what it cannot store. Lane G's concern about a
"death spiral with no floor" is bounded by warehouse capacity. Design consequence: storage
construction is a Planner-visible hoarding signal — an enterprise that keeps enlarging its
warehouse while reporting shortage is telling on itself (SYNTHESIS §3.2, lead finding).

### Calmness as maturity

The player fantasy is to turn a fragile, shortage-prone, buffer-hoarding system into a calm,
predictable, sophisticated republic. Calmness is measurable: reserve levels fall, emergency
dispatches decline, queues shorten, reports become more accurate, overtime decreases, and
household discretionary time increases. These are all consequences of the positive reliability
spiral.

## Current substrate

The spiral seed exists. `request_multiplier` is a static `i32` in the `Recipe` prototype
(`prototypes/src/types/recipe.rs:52`), set to 4 for `flour-factory` and 3 for `slaughterhouse`
(`base_mod/companies.lua:40,582`), defaulting to 1 for all others. It is wired end-to-end:
`recipe_init` (`simulation/src/souls/goods_company.rs:22-26`) calls
`market.set_requested(soul, item.id, qty)` where `qty = item.amount * request_multiplier`.

The multiplier is static. No `reliability_memory`, `fulfillment_rate`, or equivalent state
exists in `GoodsCompanyState` (`simulation/src/souls/goods_company.rs:69-78`). The spiral is
seeded but not dynamic.

## Research basis

Kornai (1980) formalised the shortage economy's feedback loop: "Firms hoarded inputs and labour
to buffer against supply uncertainties, exacerbating chronic shortages rather than resolving
them through efficiency gains." Berliner (1957) documents the same practice at the enterprise
level. The positive spiral — reliable delivery reducing buffers — is the less-documented
direction but follows by the same logic.

## Related

- [Reserves](reserves.md) — the physical store that reliability drives.
- [Scarcity](scarcity.md) — reliability determines the distribution of scarcity.
- [Adaptation](adaptation.md) — reliability drives adaptive request inflation.
- [Phase lag](phase-lag.md) — reliability changes propagate with delay.
- [Reliability and buffering](../planned-economy/reliability-and-buffering.md) — the domain
  instance for enterprises and planning credibility.
- [Enterprise behavior](../planned-economy/enterprise-behavior.md) — the dishonest enterprise
  as a reliability response.
