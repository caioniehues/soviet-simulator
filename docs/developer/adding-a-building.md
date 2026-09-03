# Adding a building or company

**Kind:** guide
**Authority:** operational
**Status:** active
**Owner:** project lead
**Verified-at:** `4e9e930b2a73`
**Last verified:** 2026-08-28

## Declare a goods company

In `base_mod/companies.lua`:

```lua
{
    type = "goods-company",
    order = "a-9",
    name = "wire-plant",
    label = "Wire Plant",
    bgen = { kind = "centered_door", vertical_factor = 0.6 },   -- or "farm"
    kind = "factory",            -- "factory": goods are delivered by truck; "store": buyers walk in
    n_trucks = 1,
    recipe = {
        consumption = {{"copper", 1}},
        production  = {{"wire", 4}},
        duration = "200s",
        storage_multiplier = 5,  -- output storage cap = amount × multiplier; halts production when full
        request_multiplier = 1,  -- >1 makes this enterprise dishonest: it requests amount × multiplier
    },
    n_workers = 10,
    size = 80.0,
    asset = "wire_plant.glb",
    price = 1000,                -- rouble cost on placement — an inherited pillar violation, see below
    power_consumption = "10kW",
},
```

Parsed by `prototypes/src/prototypes/goods_company.rs` (`GoodsCompanyPrototype { base, kind,
recipe, n_trucks, n_workers, zone }`) and `prototypes/src/types/recipe.rs` (`Recipe`). Validation
(`sov-k3w`) refuses amounts, durations or multipliers below 1.

## What the fields do in the sim

- `recipe` drives `recipe_init` (posts requests at `amount × request_multiplier`),
  `recipe_should_produce` (gates on inputs, output storage, workforce) and `recipe_act`
  (`simulation/src/souls/goods_company.rs`).
- `kind = "factory"` means the market dispatches a truck to deliver to it; `"store"` means citizens
  walk to it (the retail two-leg model).
- `n_workers` sets productivity as `present / n_workers`.
- `price` is debited from `Government.money` on placement (`world_command.rs`) — this contradicts
  the no-domestic-money pillar and is listed for retirement; do not add new money gates.
- Buildings are placed instantly; there is no Site, ghost or material bill yet
  ([construction](../simulation/physical-economy/construction.md)).

## The dishonest enterprise switch

`request_multiplier = 4` on `flour-factory` and `3` on `slaughterhouse` are the two dishonest
enterprises in the walking skeleton. If you add one, add a scenario that asserts it hoards and an
honest twin does not (`scenario_0151_inflated_request_hoards_honest_does_not` is the model), and
remember the Planner cannot yet see the discrepancy in the UI.

## Assets and presentation

Place the `.glb` under the assets tree (Git LFS). Presentation changes are judged by the
`soviet-authenticity` advisor against the art direction; paid asset generation needs the user's
confirmation first (`CLAUDE.md`).

## Prove and document

`cargo test -p prototypes`; a scenario test; an inspected frame if the building is visible. Update
[current substrate — prototypes](../architecture/current-substrate.md#prototypes) if you add a
field or a company kind.

## Related

- [Adding a resource](adding-a-resource.md)
- [Production (design)](../simulation/physical-economy/production.md)
- [Enterprise behaviour (design)](../simulation/planned-economy/enterprise-behavior.md)
- [Art direction](../reference/art-direction.md)
