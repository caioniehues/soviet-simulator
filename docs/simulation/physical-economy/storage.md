# Storage

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** economy
**Last verified:** 2026-08-28

| Scope | 1.0 binding |

## What this is

Storage is the physical space where goods sit between delivery and consumption. Each unit of
stock at a building is in one of five accounting states: on hand, reserved (allocated but not
yet picked up), in custody (in transit), embedded (consumed into a construction site), or
consumed (used in production or need satisfaction). These states are mutually exclusive for a
given unit at a given time.

Storage capacity is a physical limit. When output storage is full, production halts. This
is the hoarding floor: an enterprise cannot accumulate more than its storage holds. The
Planner can infer hoarding from an enterprise that builds more warehouse capacity while
reporting shortage — the storage construction itself is a signal (SYNTHESIS §3.2).

## 1.0 requirement

`SPEC-PRODUCTION-002` — output is bounded by output storage. The binding constraint and its
quantity SHALL be recorded for an incomplete run.

`SPEC-PRODUCTION-003` — on-hand, reserved, in-custody, and surplus are distinct quantities.

`SPEC-RESOURCES-002` — stock is an owned physical quantity. The five accounting states are
distinct records.

## Target design

The design proposes `storage_multiplier` as a Planner-visible capacity parameter
(CONFIRMED, code). An enterprise whose `capital - reserved >= amount * (storage_multiplier + 1)`
stops buying. This is the physical floor on the hoarding spiral — warehouse capacity, not
a policy flag, limits accumulation (SYNTHESIS §3.2).

## Current substrate

`recipe_should_produce` in `simulation/src/souls/goods_company.rs:43-46` implements the
storage cap:

```rust
recipe.production.iter().all(move |item| {
    (market.capital(soul, item.id) - market.reserved(soul, item.id) as i32)
        < item.amount * (recipe.storage_multiplier + 1)
})
```

Production halts when unreserved on-hand stock reaches the storage threshold. This is proven
by `scenario_0095_full_output_storage_halts_production` in
`simulation/src/tests/scenarios/recipe_provided.rs`.

`SingleMarket.capital` is `BTreeMap<SoulID, i32>` — a simple integer counter per soul per
item. There is no separate field for in-custody, embedded, or consumed quantities. The five
accounting states are a target, not the current structure.

## Open questions

- How does storage capacity interact with handling classes (bulk vs unit vs heavy)?
- Does a construction site's embedded stock count against city-wide resource totals?

## Related

- [Custody](custody.md)
- [Production](production.md)
- [Resources](resources.md)
- [Reliability](../concepts/reliability.md)
- [Production spec](../../reference/specifications/production.md#spec-production-002)
