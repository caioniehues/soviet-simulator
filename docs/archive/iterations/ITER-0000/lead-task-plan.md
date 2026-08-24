# ITER-0000 — the walking skeleton

**Intent:** One mill hoards coal delivered by one truck over one road, and the planner catches it
from observable state.

**Goal (user, 2026-08-22):** ITER-0000 end to end — harness, 9 stories, JOURNEY-0001 passing,
opus gate, 15–20s video.

Baseline verified before any edit: `cargo check -p simulation --tests` exit 0; citation check
`OK: all 149 cited stories exist`. Scope cut applied (130 scheduled + 19 deferred).

## Lead decisions (made, not delegable)

1. **Scenario runner lives INSIDE the crate.** `TestCtx` is `pub(crate)` under `#![cfg(test)]`.
   Do NOT change its visibility and do NOT build an external integration-test crate — the roadmap
   already concedes JOURNEY-0001 step 5 is proven as *sim-observable plus eyeballed panel*.
   A scenario is a plain `#[test] fn journey_0001()` carrying its ID in the name. No DSL, no
   YAML, no registry framework beyond what the sentinel runner literally needs.
2. **The mill is placed pre-built.** `TestCtx::build_house_near` selects from `map().lots()`,
   which only works because `Lot::generate_along_road` runs live from `Map::connect`
   (`simulation/src/map/map.rs:719`). ITER-0000 adds a lot-independent placement helper; that is
   the migration `sov-harness-lots-k54` asks for, done early rather than at ITER-0005.
3. **Both teleport paths get fenced, and I verified both in source, not from the brief:**
   - `market.rs:~287` — `*capital.entry(buyer).or_default() += qty_buy;` executes BEFORE
     `find_external(order.pos)` is consulted, so a buyer is credited even when no external partner
     exists. Fence: coal and steel authored with `optout_exttrade = true`.
   - `market.rs:277` — `*cap_buyer += trade.qty;` moves stock at MATCH time. Without an AC binding
     transfer to load/unload, the dispatch state machine passes every AC while the truck is
     decorative. STORY-0106 AC-3 exists precisely for this.
4. **Hoarding insertion point** is `recipe_init` (`goods_company.rs:23`) and `recipe_act`
   (`goods_company.rs:47`), both calling `market.buy_until(soul, near, item.id, item.amount as u32)`.
   Verified against source; matches STORY-0105 AC-1's citation exactly.

## File ownership — no two agents share a file

| Task | Stories | Owns | DependsOn |
|---|---|---|---|
| T0 harness | — | `simulation/src/tests/mod.rs`, `simulation/src/tests/scenarios/**` | — | ✅ `2fcfd94` |
| T1 physical delivery + hoarding | 0105, 0106, 0149 | `simulation/src/economy/market.rs`, `simulation/src/souls/goods_company.rs`, `simulation/src/map_dynamic/dispatch.rs`, `simulation/src/tests/scenarios/hoarding.rs` | T0 |
| T3 provided-substrate proofs | 0093–0097 | `simulation/src/tests/scenarios/recipe_provided.rs` | T0 |
| T4 inspection panel | 0107 | `native_app/src/**` | T1 |
| T5 journey assembly | JOURNEY-0001 | `simulation/src/tests/scenarios/journey_0001.rs` | T1 |

**Why T1 and T2 merged (correction to the original plan):** STORY-0149 AC-3 ("seller stock
decrements only on transition into `loading`, buyer stock increments only on transition into
`unloading`") and STORY-0106 AC-3 ("surplus becomes non-zero only as the result of a truck
completing an unload") are the SAME change to `market.rs:277-279`. Splitting them across two
agents puts both on the same lines. One agent owns the whole vertical.

T3 only reads production code and adds one test file — genuinely disjoint from T1.

## Sequence

- Wave 1: T0 alone. ✅
- Wave 2: T1 (delivery vertical) and T3 (proofs) in parallel.
- Wave 3: T4, T5.
- Gate: opus reviewer re-derives from source, not from worker summaries.
- Close: 15–20s video per CLAUDE.md; a prior attempt captured the wrong monitor, so verify the
  capture shows the game window before calling it done.

## Status

- Scope cut: DONE, `1bc80ca`. Opus gate FAILed with 5 CONFIRMED findings; all fixed, re-verified.
- T0 harness: DONE, `2fcfd94`. 15 tests green.
- T1 + T3: DONE, `6ea4553`. 22 tests green. STORY-0105, 0106, 0093–0097 complete.
- T2b (real truck, STORY-0149 AC-4): RUNNING. Replaces the synthetic
  `DISPATCH_TRAVEL_TICKS = 3` countdown with actual vehicle arrival events.
- T5b (migrate hoarding tests into the scenario corpus with SCENARIO-IDs): BLOCKED on T2b —
  both write `market.rs`. User confirmed corpus traceability matters: unit tests in `market.rs`
  cannot be run as a sentinel set, and the sentinel set is how every later iteration detects
  regressions.
- T4 (inspection panel, STORY-0107) and T5 (JOURNEY-0001): queued behind T2b.

## Ownership gap found the hard way

`simulation/src/tests/scenarios/mod.rs` needs a `mod X;` line from EVERY agent adding a scenario
file. My ownership table gave each agent its own scenario file but forgot they all must edit that
one shared declaration file. It did not bite this wave only because T3 finished before T1 reached
its scenario file. Any future parallel wave adding scenario files must either serialize on
`scenarios/mod.rs` or have the lead add all declarations up front.
