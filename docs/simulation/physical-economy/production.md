# Production

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** economy
**Last verified:** 2026-08-28

| Scope | 1.0 binding |

## What this is

A production run transforms physically delivered inputs into outputs. The run is bounded by
the declared recipe, available input, labour, power, water, process capacity, and output
storage. The binding constraint — the scarcest gate that limited the run — is recorded.
This lets the Planner see why a factory underproduces: not enough iron, not enough workers,
no electricity, or output storage full.

Surplus stays visible. An enterprise that produces more than it ships accumulates on-hand
stock. That stock is observable. The Planner can compare requested inputs, received inputs,
consumed inputs, and on-hand output to detect the dishonest enterprise.

## 1.0 requirement

`SPEC-PRODUCTION-001` — a run MUST consume only inputs already delivered into the producer's
accountable stock.

`SPEC-PRODUCTION-002` — output is bounded by recipe, input, labour, power, water, process
capacity, and output storage. The binding constraint SHALL be recorded.

`SPEC-PRODUCTION-004` — production MUST NOT debit domestic money or use domestic price
clearing as a gate.

`SPEC-PRODUCTION-007` — one idempotent recipe-run transaction keyed by `ProductionRunID`
SHALL atomically debit inputs and credit outputs. If any gate fails, none commit.

## Target design

`run = min(recipe, delivered_input, labour, power, water, process, output_space)`

The binding constraint is the term that produces the minimum. This is the Liebig-bottleneck
model applied to production: one constraint at a time limits output.

`ProductionRunID` atomicity (HYPOTHESIS, bible §6.10) makes each run a single transaction.
A partial run that debits inputs without crediting outputs would violate conservation. The
run ID ensures idempotency: retry of the same ID is a no-op.

An electricity blackout stops production entirely. The current code sets productivity to
zero under blackout (`goods_company.rs:103-108`).

## Current substrate

`recipe_should_produce` (`simulation/src/souls/goods_company.rs:31-49`) gates on:
- Input availability: `capital(soul, item.id) >= item.amount` for every consumed item
- Output storage cap: `(capital - reserved) < amount * (storage_multiplier + 1)`
- Non-empty recipe

`recipe_act` (`goods_company.rs:51-66`) atomically debits inputs and credits outputs in one
pass through the recipe:

```rust
for item in &recipe.consumption {
    market.produce(soul, item.id, -item.amount);
    // ...
}
for item in &recipe.production {
    market.produce(soul, item.id, item.amount);
    // ...
}
```

Productivity scales with workforce ratio: `workers.len() / n_workers` (`goods_company.rs:83-85`).
If the company consumes power, a blackout sets productivity to zero (`goods_company.rs:103-108`).
Zone area further scales productivity.

What is missing: the binding constraint is not recorded. There is no `ProductionRunID`.
Water is not a production gate. The production progress counter is `f32` (`progress` field
in `GoodsCompanyState`), advanced by `productivity * DELTA` each tick.

## Research basis

Kornai (1980) describes the socialist enterprise as constrained by input availability and
plan targets, not by demand or price. The binding constraint is the physical expression of
that: the enterprise does what it can with what it has (CONFIRMED, Lane A).

## Open questions

- Which utility shortfalls permit partial rate and which require a hard stop?
- What recipe schema and acceptance threshold define the twelve new recipe buildings?

## Related

- [Storage](storage.md)
- [Resources](resources.md)
- [Requests](requests.md)
- [Logistics](logistics.md)
- [Electricity](../infrastructure/electricity.md)
- [Production spec](../../reference/specifications/production.md)
