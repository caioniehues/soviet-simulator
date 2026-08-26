---
name: seam-hoard-panel-story0107
description: STORY-0107 hoarding-panel seam — Market accessors are public and the building inspector IS reachable, but set_requested has ZERO production callers so no dishonest enterprise can exist in a running game
metadata:
  type: project
---

SEAM: STORY-0107 / sov-hoard-panel-mko — inspection panel showing requested vs consumed
Verified 2026-08-26 against **working tree at HEAD f89bc3b + dirty**.
Volatile: `simulation/src/economy/market.rs` (+475 lines uncommitted),
`simulation/src/souls/desire/buyfood.rs` (+57). sim-impl active during the sweep.

**Re-verified at sweep end: market.rs line numbers SHIFTED ~+11 mid-sweep**
(`set_requested` 428→439, `requested` 433→444, `dispatches` 437→448,
`reserved` 419→430, `retail_claim` 443→454). The accessor SET is unchanged and
the lead finding is unchanged (set_requested still test-only). Line numbers below
are as first observed; add ~11 for market.rs when reading the current file, and
re-grep rather than trusting any market.rs line number in this sheet — it is the
one file guaranteed to keep moving.

## LEAD FINDING — the panel has data to read, but nothing to show

`Market::set_requested` (market.rs:428) has **zero production callers**. The only
callers in the entire repo are `simulation/src/tests/scenarios/hoarding.rs:244-245`.
Grep of `set_requested` across all `.rs`: 4 hits = 1 definition, 1 doc-comment, 2 test lines.

Consequence: in a running game every company falls to
`.requested(soul, item.id).unwrap_or(item.amount as u32)`
(`goods_company.rs:24` in `recipe_init`, `goods_company.rs:57` in `recipe_act`) —
the `unwrap_or` **defaults requested to exactly the recipe amount**. requested ==
consumption for every company, always. The surplus the panel exists to reveal is
identically zero.

STORY-0107 AC-2 ("two identical buildings, one honest and one inflated, and the
inflated one visibly accumulates surplus") is **not satisfiable on this tree by UI
work alone**. Inflation exists only as a test fixture. Something must set requested
above recipe amount in the sim — a Lua field, a prototype field, or a spawn-time
rule — before any panel can render a non-zero delta.

This is a `simulation/` dependency that lands BEFORE the UI brief.

## PROVIDED — exists and reachable

**Market accessors (all `pub`, re-exported `pub use market::*` at economy/mod.rs:30):**
- `Market::requested(soul, kind) -> Option<u32>` — market.rs:433. Present at HEAD (HEAD:247).
- `Market::set_requested(soul, kind, qty)` — market.rs:428. Present at HEAD (HEAD:242).
- `Market::capital(soul, kind) -> i32` — market.rs:412. On-hand stock.
- `Market::reserved(soul, kind) -> u32` — market.rs:419. **UNCOMMITTED ONLY** — absent at HEAD.
- `Market::retail_claim(buyer) -> Option<&RetailClaim>` — market.rs:443. **UNCOMMITTED ONLY**.
- `Market::settle_retail(buyer) -> bool` — market.rs:453. **UNCOMMITTED ONLY** (mutating).
- `Market::iter() -> impl Iterator<Item=(&ItemID,&SingleMarket)>` — market.rs:218.
- `Market::inner() -> &BTreeMap<ItemID,SingleMarket>` — market.rs:710.
- `SingleMarket::{capital,buy_order,sell_order,requested,reserved,capital_map}` — market.rs:66-82.

**The inspector host is live.** `inspect_building()` at
`native_app/src/gui/inspect/inspect_building.rs:24`, called from
`native_app/src/gui/inspect/mod.rs:23`, driven by the `InspectedBuilding` resource
(`gui/mod.rs:92`, registered `init.rs:55`), written by click in
`gui/tools/selectable.rs:23`. Full path: click → selectable → InspectedBuilding →
inspect_building → `render_goodscompany` (line 150). Reachable, no new plumbing needed.

`render_goodscompany` **already reads Market** at line 163 (`sim.read::<Market>()`)
and already renders per-item on-hand stock at lines 257-266 via `m.capital(c_id.into())`.
It already renders the recipe's true consumption via `render_recipe` (line 269),
called at line 199. **Both halves of AC-1 are already on screen in the same panel** —
they are simply not differenced or labelled as request-vs-consumption.

Other panel hosts (one line, per brief): `gui/hud/windows/economy.rs` (436L),
`gui/hud/toolbox/building.rs`, `debug_gui/debug_window.rs` (688L),
`debug_gui/debug_inspect.rs`, plus siblings `inspect_human/train/vehicle.rs`.

## PRESENT-BUT-DEAD

**`Market::dispatches()` — market.rs:437. ANSWER: NO reachable observation path
from the running game. The prior gate's finding STILL HOLDS on this tree.**

- LSP `findReferences` on market.rs:437 → "No references found."
- Grep: every caller is `simulation/src/tests/scenarios/ledger.rs`
  (lines 43, 177, 178, 288, 289, 296, 360, 377, 457, 537, 539, 558). Test-only.
- Zero references anywhere in `native_app/`.
- The method is `pub` and the type `Dispatch` is `pub` (market.rs:180), so it is
  *callable* from native_app — nothing blocks a panel binding to it. It is dead by
  disuse, not by visibility. "No observation path" = nothing observes it today,
  NOT that observation is impossible.
- Trap: `Dispatch`'s fields split. `buyer/seller/kind/qty/state` are `pub`
  (market.rs:181-185) but `ticks_left`, `truck`, `return_route_retries` are
  **private** (market.rs:186-195) with no accessors. A panel wanting ETA or the
  carrying truck needs new accessors in simulation/.

## ABSENT

- No per-company "consumed this cycle" counter. Consumption is applied in-place at
  `goods_company.rs:55` (`market.produce(soul, item.id, -item.amount)`) and never
  recorded. The panel must derive true consumption from
  `proto.recipe.consumption[i].amount`, not from an observed tally.
- No delivered/received-to-date accumulator. Delivery lands as a `capital` delta;
  nothing tracks cumulative receipts per company.
- No surplus/discrepancy/age field anywhere, despite REQ-PRODUCTION-001 naming
  "request, receipt, consumption, surplus, and age discrepancies".
- No honesty flag — correctly so; REQ-PRODUCTION-001 requires inference "without an
  honesty flag".

## CONTRADICTS

1. **The lead's cited path was wrong.**
   `docs/plan/iterations/requirements/story-migration.md` **does not exist**. The real
   file is `docs/plan/traceability/story-migration.md`. The requirements directory holds
   six domain files (built-world, economy, movement, settlement, utilities + README +
   build_requirements.py), no migration ledger.

2. **The bd issue's CEILING presupposes an accessor that is only half-there.**
   sov-hoard-panel-mko says "Assert the accessor the panel binds to is public API on
   the simulation crate." The *reading* accessors (`requested`, `capital`) are public
   API and pass that assert. But the assert is satisfiable while the story remains
   unbuildable, because nothing in the sim ever sets requested ≠ consumption. Passing
   the stated ceiling check does NOT mean AC-2 is reachable.

3. **The bd issue's legacy acceptance criteria are explicitly not authority.**
   story-migration.md:119 marks STORY-0107 `rewritten`, with rationale "legacy
   acceptance detail is not authority", mapping to **REQ-PRODUCTION-001**. The bd
   issue text still carries the legacy AC-1/AC-2 wording. Where they differ, the REQ
   card governs.

## Q5 ANSWER — the live REQ card

STORY-0107 → **REQ-PRODUCTION-001** "Input-bounded production and observable
dishonest enterprises" (`docs/plan/iterations/requirements/economy.md:27`).
Anchors SPEC-PRODUCTION-001..009. Status **proposed**;
**Evidence status: "UNIMPLEMENTED — target guards block specification ratification."**

Its binding sentence (economy.md:41): "request, receipt, consumption, surplus, and
age discrepancies let the Planner infer hoarding without an honesty flag."

**The card carries NO numeric constants** — no quantities, thresholds, rates or
$CONSTANT citations. Nothing to ground-truth against the W&R corpus. Reported as a
finding per the lead's instruction, not as "none". The card is qualitative; the only
numbers in this seam are the test's own `TRUE_CONSUMPTION=2` / `INFLATED_REQUEST=5`
(hoarding.rs:237-238), which are fixture values, not spec.

## LUA — base_mod/, 947 lines total

- `items.lua`: **21 items**, of which **exactly 1** sets `optout_exttrade = true`
  — `job-opening` (items.lua:6). The known trap, re-confirmed on this tree.
  `job-opening` is also special-cased out of the inspector's storage list
  (inspect_building.rs:256, 261-263) when its value is 0.
- `companies.lua`: **26 goods-companies, all 26 declaring a recipe.**
  Kinds: 21 `factory`, 6 `store` (27 = one company declares kind twice or a nested
  kind; treat the 21/6 split as indicative, the 26 count as exact).
- **No Lua field declares requested, hoarding, inflation or dishonesty.** Grep for
  `requested|hoard|inflat|optout` across all seven .lua files returns exactly one
  hit: the `optout_exttrade` line above.
- Recipe grammar is `consumption = {{"flour",1}}`, `production = {{"bread",1}}`,
  `duration`, `storage_multiplier` (companies.lua:12-17). The panel's "true
  per-cycle consumption" comes from this declaration.
- **Consequence for the panel:** all 26 companies would appear, every one with
  requested == consumption. Adding a per-company inflation knob is a Lua schema
  change (new prototype field) plus a prototypes/ parse change, not a UI change.

## REFERENCE — W&R, data not UI (labelled per lead's Q4 ruling)

Corpus verified present 2026-08-26: 1,472 files at
`~/.local/share/Steam/steamapps/common/SovietRepublic/media_soviet/buildings_types/`.

**W&R ships no UI source.** The `.ini` corpus declares *data*; the inspection window
is compiled C++. "How W&R surfaces requested vs consumed" is NOT answerable from the
readable substrate. Answering the answerable version — which fields a panel could surface:

Verbatim from the asphalt plant (`$NAME 6169`):
```ini
$TYPE_FACTORY
$WORKERS_NEEDED 5
$PRODUCTION asphalt 29
$CONSUMPTION gravel 25
$CONSUMPTION bitumen 4
$CONSUMPTION eletric 3
$STORAGE_IMPORT RESOURCE_TRANSPORT_OIL 15
$STORAGE_IMPORT RESOURCE_TRANSPORT_GRAVEL 30
$STORAGE_EXPORT RESOURCE_TRANSPORT_GRAVEL 1
$ELETRIC_WITHOUT_WORKING_FACTOR 0.4
```

Structural note: W&R separates `$CONSUMPTION` (the recipe rate) from
`$STORAGE_IMPORT` (the inbound buffer cap, per transport class). That is a
declared *rate* vs a declared *buffer* — the same two quantities our panel must
distinguish. Our `storage_multiplier` (companies.lua:16) is the nearest equivalent
to `$STORAGE_IMPORT`, and `recipe.consumption` to `$CONSUMPTION`.
**W&R has no field expressing "requests more than it consumes"** — the dishonest
enterprise is our invention, not inherited. No W&R precedent to copy.

## TRAPS

1. **Do not brief a UI-only ticket.** Panel work alone cannot satisfy AC-2. The
   inflation source must exist in simulation/ first.
2. **`reserved()` / `retail_claim()` / `settle_retail()` are uncommitted.** A panel
   binding to them will not compile against HEAD f89bc3b. Confirm sim-impl's work
   lands before any brief cites them.
3. **`requested()` returns `Option<u32>`, and the None case is the normal case.**
   `None` does not mean zero — both call sites treat it as "use the recipe amount"
   (`goods_company.rs:24,57`). A panel rendering `unwrap_or(0)` would show every
   company requesting nothing.
4. **True consumption is `item.amount`, an `i32` on the recipe; requested is `u32`.**
   The comparison needs a cast. `market.capital()` is also `i32` and CAN go negative.
5. **`reserved` is not surplus.** Per market.rs:417-418 it is stock matched but not
   yet collected — physically at the seller. Subtracting it from capital gives
   sellable stock, not hoard.
6. **`render_goodscompany` early-returns twice** (lines 153-155, 156-158) when the
   building has no GoodsCompany soul. A panel addition placed after those returns is
   invisible for any building mid-spawn.
7. **`Dispatch` private fields** (`ticks_left`, `truck`, `return_route_retries`) have
   no accessors — ETA/truck display needs new simulation/ API.
8. **26 companies all render.** Any per-item row added to the storage loop
   (inspect_building.rs:257) runs once per item per company.

## Where the primary sources live
- Market: `simulation/src/economy/market.rs` (accessors 412-465, Dispatch 180-196).
- Request/consume: `simulation/src/souls/goods_company.rs:21-68`
  (`recipe_init`, `recipe_should_produce`, `recipe_act`).
- Panel host: `native_app/src/gui/inspect/inspect_building.rs:150-299`.
- Click path: `native_app/src/gui/tools/selectable.rs:22-23` → `gui/mod.rs:92`.
- Hoarding fixture: `simulation/src/tests/scenarios/hoarding.rs:237-245`.
- REQ card: `docs/plan/iterations/requirements/economy.md:27-41`.
- Migration ledger: `docs/plan/traceability/story-migration.md:119`.
- W&R corpus: `~/.local/share/Steam/.../media_soviet/buildings_types/` (1,472 files).
