---
name: recipe-numeric-fields-are-consumed-unsigned
description: Recipe amounts and request_multiplier are i32 in Lua/Rust but consumed as u32 and as a divisor — below 1 inverts their meaning; plus the real base_mod distribution of both
metadata:
  type: project
---

`Recipe.request_multiplier` and `RecipeItem.amount` are both `i32`
(`prototypes/src/types/recipe.rs:8,52`), but every consumer treats them as
positive:

- `simulation/src/souls/goods_company.rs:23` —
  `item.amount as u32 * recipe.request_multiplier as u32`. `-3i32 as u32` is
  4_294_967_293: a standing market request larger than the whole economy, with
  no panic and no log. `0` requests nothing, so the enterprise never produces —
  a silent permanent stall.
- `simulation/src/economy/market.rs` `calculate_price_inner` — divides the
  recipe's cost by the production `amount`. `0` panics
  `attempt to divide by zero` at **`prototypes/src/types/money.rs:193`**
  (`impl Div<i64> for Money`). Note: there is no `simulation/src/economy/money.rs`;
  the economy dir is `ecostats.rs`, `government.rs`, `market.rs`, `mod.rs`.
  Several tickets cite the wrong path for this panic.

**Why this is the lane's archetype:** the danger is not the explicit values, it
is that the parse default and the sign convention are invisible in the Lua.
`request_multiplier` is parsed with `.unwrap_or(1)` and 1 means *honest* — so
any value that fails to be a positive number silently deletes or inverts the
dishonest-enterprise loop, the core of the game.

**base_mod distribution, verified 2026-08-28 at f6725f1** (26 recipes in
`base_mod/companies.lua`, all using the positional `{"id", n}` form; the keyed
`amount =` form is unused in base_mod):

- `request_multiplier`: set on **2 of 26** — `flour-factory` = 4
  (companies.lua:40), `meat-factory` = 3 (:582). The other **24 omit it** and
  take the parse default `1`, i.e. they route through the honest branch.
- recipe amounts: min **1**, max **10** (`flour-factory` production
  `{"flour", 10}`, :37); the only other non-1 value is `{"vegetable", 2}`
  (:600). 14 recipes have an empty `consumption = {}` or `production = {}`.
- `storage_multiplier`: present on all 26.

Since sov-k3w, `validate()` refuses `request_multiplier < 1` and any amount
`< 1`, and `calculate_price_inner` skips a recipe as a price candidate when
`qty <= 0` rather than dividing.

Related: [[prototype-load-and-validate-path]]
