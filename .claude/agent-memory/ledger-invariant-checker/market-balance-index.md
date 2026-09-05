---
name: market-balance-index
description: Complete reader/writer index for the Market ledger buckets (capital, reserved, requested, dispatches, retail_claims) — use it to find the distant reader that does not know about a new writer
metadata:
  type: project
---

`SingleMarket` holds four per-soul buckets; `Market` holds two global ones
(`dispatches`, `retail_claims`). Index of every site that touches them, so a new
writer can be cross-checked against every old reader.

Last re-derived: 2026-08-26 against the `sov-dispatch-wedge-ab4` working tree
over HEAD `f89bc3b`, **after the implementer's ROUND-3 fix pass** (3rd audit
pass, verdict CONSERVED). Line numbers are from that tree and shifted ~150
lines vs the round-2 audit — `Market::remove` grew from a 5-line retain to a
140-line per-DispatchState settlement loop.

## `capital: BTreeMap<SoulID, i32>` (can go NEGATIVE)
Writers:
- `Market::produce` (`+= delta`) — declared source/sink, also negative delta from `recipe_act`.
- `Market::register` (`entry().or_default()`) — insert-only.
- `make_trades` job-opening branch: `*capital.entry(seller).or_default() -= trade.qty` (immediate settle, added by the wedge diff — closes old Family C).
- `make_trades` ext-trade buy block (`:576`): `*capital.entry(buyer).or_default() += qty_buy`. **Humans are now carved out at `:556-566`** — their keys are removed from `btaken` and reinserted into `buy_orders`, so retail never clears by money. See [[break-families]] Family E (CLOSED).
- `make_trades` ext-trade sell-surplus block (`:546`): `*cap -= qty_sell`, guard now `*cap - already_reserved < qty_sell`. **Fixed — subtracts `reserved` at `:535-542`.**
- `advance_dispatches` ToSource-arrival (`:657`): `-= qty`.
- `advance_dispatches` ToDestination-arrival (`:763`): `+= qty`.
- `advance_dispatches` Returning-arrival (`:778`): `+= qty` (re-credit). Fires at most once — dispatch is `swap_remove`d in the same iteration.
- `Market::settle_retail` (`:339`): `*capital.entry(seller).or_default() -= claim.qty`. Buyer credited nothing, by design.
- `Market::remove` (`:258`): `capital.remove(&soul)`.

Readers:
- `Market::capital` → `sell_all` (`c as u32`, guarded `c <= 0`), `buy_until` (`qty - c as u32`, **still unguarded against c < 0**).
- `recipe_should_produce` — consumption test on raw capital; **storage test now `capital - reserved`** (goods_company.rs:46).
- domestic match pre-filter (`:375`) `qty_sell > capital_sell` on RAW capital, then inner filter_map `cap_seller - already_reserved < trade.qty` (`:423`) — the inner guard is what actually protects; verified sufficient.
- `capital_map()` / `SingleMarket::capital()` for UI/inspect.
- **Nothing anywhere reads or debits a HUMAN's `capital[bread]`.** Grep `capital` under `simulation/src/souls/` returns only goods_company.rs. This is the sink that does not exist.

## `reserved: BTreeMap<SoulID, u32>`
Writers:
- domestic match filter_map (`:463`) `+= trade.qty` — every non-job-opening match, retail included.
- `advance_dispatches` ToSource-arrival (`:658`) `saturating_sub(qty)`.
- `advance_dispatches` dead-truck ToSource (`:645`) `saturating_sub(qty)`.
- `Market::settle_retail` (`:340`) `saturating_sub(qty)`.
- `Market::remove` buyer-claim release (`:244`) `saturating_sub(qty)`.
- `Market::remove` (`:259`) `reserved.remove(&soul)`.
- TTL sweep in `advance_dispatches` (`:827`) `saturating_sub(qty)`.
Readers: domestic match filter_map; ext-trade surplus loop (`:535`); `recipe_should_produce` (`:46`); `SingleMarket::reserved` / `Market::reserved`.
**Still NOT read by `sell_all` or `buy_until`.** `sell_all` posts `qty = capital` including reserved; safe today only because the inner match guard catches it.

## `retail_claims: BTreeMap<SoulID /* buyer */, RetailClaim>` (added by the wedge diff)
Writers: `make_trades` human-buyer branch (`:503-531`) — `insert` whose displaced value's reservation is **released on the OLD seller's row** before the new claim stands (Family F, CLOSED); `settle_retail` `remove`; `Market::remove` `remove(&soul)` + `retain(seller != soul)`; TTL sweep `retain`.
Readers: `Market::retail_claim` → `buyfood.rs:100` (`WaitingForTrade` reset) and `buyfood.rs:117` (`BoughtAt` eat-or-go-without gate, Family G).
Keyed by buyer, so one human = at most one live claim. The release uses the per-kind loop's local `reserved` map, guarded only by `debug_assert_eq!(old.kind, kind)` — sound today because `buyfood.rs:82` is the sole human buy-order issuer and hardcodes `bread`.

## `Government::money` written from inside `advance_dispatches` (sov-ahw, 2026-09-02)
`gvt.money -= d.money_delta` on the ToSource/no-truck timeout (`MAX_SOURCE_WAIT_TICKS`,
300 ticks). `Dispatch` now carries `money_delta` (copied from the import Trade) and
`source_wait_ticks` (never reset). Refund mutation-proven exactly-once (remove → -0.50$,
double → +0.49$ vs baseline). No other dispatch drop path (dead truck, `Market::remove`,
Loading loss) refunds an import — pre-existing asymmetry. See [[partial-rollback-shape]].

## `requested: BTreeMap<SoulID, u32>`
Writers: `set_requested`; `Market::remove` (`:260`) now removes it.
Readers: `recipe_init` / `recipe_act`.

## `Dispatch::return_route_retries: u32` (added by the fix pass)
Bounded by `MAX_RETURN_ROUTE_RETRIES = 20` (`:138`). Incremented at `:794` when
`Loading` cannot route back to the seller; at the bound the dispatch is dropped
as a declared physical loss with no re-credit (`:779-796`). See Family H.

## `dispatches: Vec<Dispatch>`
Writers: `make_trades` push (`:626`, skipped for job-opening AND for
`SoulID::Human` buyers); `advance_dispatches` mutate/`swap_remove`;
`Market::remove` (`:295-390`).
**`Market::remove` is now a per-`DispatchState` settlement loop, not a blind
retain** (round-3 fix, Family B CLOSED). Skip condition `if d.buyer != soul ||
d.seller == soul { continue }` at `:298`; dead-seller dispatches fall through
to the final `retain(|d| d.seller != soul)` at `:390`. Arms:
- `ToSource` (`:304`) → `reserved[seller] -= qty`, free truck, drop.
- `Loading`/`ToDestination`/`Returning` (`:316`) → route truck home, set
  `Returning`; else declared loss + warn + free truck.
- `Unloading` (`:369`) → declared loss + warn + free truck.
`DispatchState::Returning` in `advance_dispatches` (`:963`) HAS the
truck-vanished guard as of round 3; arrival credits `capital[seller] += qty`
at `:987` and `swap_remove`s in the same iteration (credits exactly once —
verified by 1500 extra ticks in `audit_returning_credits_seller_exactly_once`).

**CORRECTED 2026-08-27 (HEAD `8531d3c`): the seller-side truck leak is FIXED.**
An earlier version of this file said the truck is "still not freed on the
seller-side `retain`". That is no longer true — `market.rs:398-403` now does
`for d in self.dispatches.iter().filter(|d| d.seller == soul) { if let Some(v)
= d.truck { dispatcher.free(...) } }` *before* the `retain` at `:403`, with a
"round 4" comment at `:392-397` explaining that `Dispatcher::query` skips
anything in `reserved_by` and only `free` clears it, so dropping without
freeing removed the truck from the city permanently.

## `SimDrop::sim_drop` (widened round 3)
Signature `fn sim_drop(self, id, world: &mut World, res: &mut Resources)` —
`par_command_buffer.rs:7`, applied at `:67`. Widened purely so
`Market::remove` can reach `world.vehicles` to re-route a truck.
6 impls in `world.rs`: Vehicle/Train/Wagon take `_world` (no-ops); Human
(`:108`), FreightStation (`:165`) and Company (`:194`) pass it into
`Market::remove`. **No impl mutates state another impl settles.**
Ordering fact: `apply()` removes the entity from its storage BEFORE calling
`sim_drop`, and `scheduler.rs:46-51` flushes the six buffers in the order
Vehicle, Human, Train, Wagon, FreightStation, Company after EVERY system. So a
truck killed in the same batch as its buyer is already out of `world.vehicles`
when `Market::remove` runs — `remove` handles this via `start`/`route` being
`None`, taking the declared-loss branch. Verified, conserves.

## Data-layer facts that decide which branch real items take
- `base_mod/items.lua`: `optout_exttrade = true` on **`job-opening` only** (line 6). **`bread` (line 20) does NOT opt out** — so human bread buy orders reach the ext-trade block.
- `base_mod/companies.lua`: `bakery` is `kind = "store"` (line 11), recipe `flour 1 -> bread 1`, `storage_multiplier = 5`. Stores get no trucks (`goods_company.rs:135` spawns only for `CompanyKind::Factory`).
- `binfos.rs` `set_owner` is called for GoodsCompany, FreightStation and Human — so `door_pos` and `find_trade_place` resolve for all three, including a FreightStation named as an ext-trade `seller`.

## System order within a tick (`init.rs`)
`update_decision_system` (`:62`) runs **before** `company_system` (`:65`), which
runs **before** `market_update` (`:98`, runs `make_trades` then
`advance_dispatches`). So a buy order issued this tick is matched later in the
same tick, and the resulting `Trade` lands in `h.bought` for the *next* tick's
`BuyFood::apply` to drain.

**Critical: `scheduler.rs:41-52` flushes EVERY `ParCommandBuffer` after EVERY
system, not once per tick.** Two consequences that decide reachability:
- `settle_retail`, queued from `update_decision_system`, applies before
  `market_update`'s TTL sweep → the claim is already gone, **no double-release.**
- `company_system`'s `cbuf.kill(me)` (goods_company.rs:197-198, fired when its
  building was demolished) applies before `advance_dispatches` → `Market::remove`
  runs first. **As of round 3 this is no longer a reachability problem:**
  `Market::remove` itself creates the `Returning` dispatch, so the headline
  feature fires on the ordinary demolish-a-building path. Proven end-to-end
  with `map.remove_building` on a loaded-truck buyer (total 10 → 10), and
  mutation-proven load-bearing (blind-drop mutation gives 10 → 0).

## Where `Market::remove`'s two halves are each covered by a test
Round 2 missed the buyer half because only seller removal was tested. Current
coverage, all in `tests/scenarios/ledger.rs`:
- seller removed → `scenario_ledger_remove_leak`
- buyer removed at `ToSource` → `scenario_dead_buyer_tosource_releases_reservation`
- buyer removed at `Loading` → `scenario_dead_buyer_loading_goods_returned`
- buyer then seller, mid-`Returning` → *audit-only, not committed*
- seller first, then buyer → *audit-only, not committed*
- end-to-end `map.remove_building` → *audit-only, not committed*
**The three audit-only cases were written and passed in the 3rd pass but
reverted (audit does not edit the repo). If this area is touched again, they
are worth re-adding as permanent tests** — especially the end-to-end one,
since it is the only test that exercises the real gameplay path rather than a
direct `Market::remove` call.
