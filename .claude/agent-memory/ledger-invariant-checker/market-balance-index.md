---
name: market-balance-index
description: Complete reader/writer index for the Market ledger buckets (capital, reserved, requested, dispatches) as of b3857f5 — use it to find the distant reader that does not know about a new writer
metadata:
  type: project
---

`SingleMarket` holds four per-soul buckets. Index of every site that touches them,
so a new writer can be cross-checked against every old reader.

## `capital: BTreeMap<SoulID, i32>` (can now go NEGATIVE — see [[break-families]])
Writers:
- `Market::produce` (`+= delta`) — declared source/sink, also used with negative delta by `recipe_act` consumption.
- `Market::register` (`entry().or_default()`) — insert-only.
- `make_trades` ext-trade buy block: `*capital.entry(buyer).or_default() += qty_buy`.
- `make_trades` ext-trade sell-surplus block: `*cap -= qty_sell` (guard: `*cap < qty_sell` only — **ignores `reserved`**).
- `advance_dispatches` ToSource-arrival: `*m.capital.entry(seller).or_default() -= qty` (**no guard at all**).
- `advance_dispatches` ToDestination-arrival: `*m.capital.entry(buyer).or_default() += qty`.
- `Market::remove`: `capital.remove(&soul)`.

Readers:
- `Market::capital` → `sell_all` (`c as u32`, guarded `c <= 0`), `buy_until` (`qty - c as u32`, **unguarded against c < 0**).
- `recipe_should_produce` (both the "has inputs" and the "has storage" test — the storage test does NOT subtract `reserved`).
- domestic match loop: pre-filter `qty_sell > capital_sell`, then `cap_seller - already_reserved < trade.qty`.
- `capital_map()` / `SingleMarket::capital()` for UI/inspect.

## `reserved: BTreeMap<SoulID, u32>` (added by 6ea4553)
Writers: domestic match filter_map `+= trade.qty`; `advance_dispatches` Loading transition `saturating_sub(qty)`.
Readers: **only** the domestic match filter_map. Not `sell_all`, not `buy_until`, not the
ext-trade block, not `recipe_should_produce`, not `Market::remove`.

## `requested: BTreeMap<SoulID, u32>`
Writers: `set_requested` (production callers: none yet — only `tests/scenarios/hoarding.rs`).
Readers: `recipe_init` / `recipe_act` via `Market::requested`. Not cleared by `Market::remove`.

## `dispatches: Vec<Dispatch>`
Writers: `make_trades` (push, skipped for `job-opening`); `advance_dispatches` (mutate, `swap_remove`).
Not touched by `Market::remove`. No cap, no timeout, no cancellation path.

## Data-layer facts that decide which branch real items take
- `base_mod/items.lua`: 21 items, `optout_exttrade = true` on **`job-opening` only**; 20 physical goods take the ext-trade path.
- `base_mod/companies.lua`: every recipe has `storage_multiplier = 5`; production `amount` is 1 for almost everything, **10 for `flour-factory`**. `sell_all` stock arg = `amount * storage_multiplier`.
- `binfos.rs` `set_owner` is called for GoodsCompany, FreightStation, **and Human** (`human.rs:272`, their house) — so `door_pos` resolves for human buyers. The comment at `market.rs:358` claiming "a human buyer owns no building" is wrong.
