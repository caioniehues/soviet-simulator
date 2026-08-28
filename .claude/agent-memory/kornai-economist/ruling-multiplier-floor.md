---
name: ruling-multiplier-floor
description: sov-k3w sign-off — request_multiplier floor of 1 is correct and one-sided; load refusal is not game over; price is decorative domestically
metadata:
  type: project
---

Ruling given 2026-08-28 on **sov-k3w** (uncommitted working tree over f6725f1).
Verdict: **consistent with the model, one caveat** (filed as `sov-mjj`).

## The floor of 1 on `request_multiplier`

`prototypes/src/validation.rs` now refuses `request_multiplier < 1` and any recipe
`consumption`/`production` `amount < 1`.

**Why 1 is the only correct floor.** Under shortage no enterprise rationally
under-requests — Kornai's quantity drive says the rational move is always to inflate.
So the dial is legitimately **one-sided, 1..inf**.

- `request_multiplier = 0` is not a mothballed plant. It is a permanent silent stall:
  `souls/goods_company.rs:23` sets qty 0, `buy_until` never acquires, and
  `recipe_should_produce` (`goods_company.rs:35`) requires `capital >= amount`, never true.
- "Requests nothing" is already expressible correctly as an **empty consumption list**
  (farms, mines). The floor does not touch that case.
- Negative wraps: `-3 as u32` = 4_294_967_293 at `goods_company.rs:23`. Not hoarding —
  an arithmetic accident that starves every other buyer of that item.

**Dynamics this forecloses:** any future AC positing an under-requesting or
zero-requesting enterprise is impossible by construction. That is correct, not a loss.

## Load refusal vs "never game over"

Not a breach. The pillar governs a **running** simulation. `simulation/src/init.rs:44-49`
already panicked on every pre-existing `validate()` error (n_trucks, dangling item ids,
negative power) **before a world exists** — no city, no save, no player state lost. Same
class as a missing asset file. The diff adds members to an existing refusal set; it creates
no new failure mode. Failing to LOAD is a data contract; failing during PLAY is what the
pillar forbids.

## Price is decorative domestically — traced end to end

`calculate_prices` feeds ONLY `SingleMarket::new` -> `ext_value` (`market.rs:216,219`).
`ext_value` is read at exactly three sites:
- `market.rs:651` and `market.rs:740` — external-trade `money_delta` (border clearance)
- `native_app/src/gui/hud/windows/economy.rs:343` — display

Domestic matches hard-code `money_delta: Money::ZERO` (`market.rs:544`). `money_delta` only
accumulates into `gvt.money` (`economy/mod.rs:104`) and ecostats. **Nothing gates on
`Government.money`** — the soft budget constraint working as designed.

`Money::ZERO` is **not a new state**: any item produced by no company with a recipe already
gets `minprice = None -> Money::ZERO` today (`market.rs:1207,1247`). The `qty <= 0` guard
introduces no downstream signal that did not already exist.

## Guard that must NOT be deleted

`if qty <= 0 { continue; }` at **`market.rs:1231`**. `validate()` makes it unreachable from
`base_mod`, so a future cleanup pass will read it as dead code. It is not: it is the
no-panic guard on the only division in `calculate_price_inner`, and pillar 2 requires it
independent of reachability. Same shape as the `market.rs` `Parked` guard a ticket once
proposed deleting.

## Rejected alternatives (do not re-propose)

- **Clamping at consumption instead of refusing at load.** A clamp turns a typo into
  plausible behaviour with no warning — the repo's signature defect. Clamping 0 to 1 would
  silently make a dishonest enterprise honest.
- **Substituting a fallback divisor in `calculate_price_inner`.** Publishes a silently wrong
  border price. Skipping the candidate is right.

## Caveat -> `sov-mjj`

`prototypes/src/types/recipe.rs:63` still reads
`get_lua(&table, "request_multiplier").unwrap_or(1)`, which swallows a **type error** as
well as absence. A malformed value loads as `1` = honest, `validate()` passes, and the
hoarding loop is deleted silently. Correct form exists at
`prototypes/src/prototypes/goods_company.rs:41-42` (`get_lua_opt(..)?.unwrap_or(..)`).

Related: [[ruling-inflation-source]], [[numbers-base-mod]].
