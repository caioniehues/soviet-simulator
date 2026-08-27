---
name: seam-data-layer-2026-08-27
description: Data-layer seam (prototypes crate + base_mod Lua) mapped 2026-08-27 at 8531d3c+dirty; proven divide-by-zero panic from one Lua digit, four dead prototype kinds, silent-default parse traps
metadata:
  type: project
---

# SEAM: data layer — prototypes/ + base_mod/*.lua

**Verified at commit `8531d3c` + dirty working tree, 2026-08-27.**
Dirty state relevant to this seam: `base_mod/companies.lua` carries two uncommitted
`request_multiplier` lines (flour-factory = 4, meat-facility = 3).

**Tooling note:** LSP was DISABLED for this session (`ToolSearch("select:LSP")` loaded a
schema, but calling it returned "No such tool available: LSP. LSP is disabled for this
session, in subagents as well as here."). All reachability claims below come from `grep`
plus the guard's documented relent path (3 blocked Reads per file → allowed). Disclosed
per the guard's own instruction.

## The load-bearing fact

**One digit in a Lua file panics the entire simulation at startup, and no validation
catches it.** `market.rs:1127` divides by `qty`, where `qty` is a recipe production
`amount` read straight from Lua. `Money`'s `Div<i64>` (`prototypes/src/types/money.rs:193`)
is unguarded integer division. `prototypes/src/validation.rs` never checks `amount`.

PROVEN, not inferred. Probe run and reverted:
- Edited `base_mod/companies.lua` cereal-farm `production = {{"cereal", 1}}` → `{{"cereal", 0}}`.
- `cargo test -p prototypes` → **5 passed**. Load-time validation accepts it silently.
- `cargo test -p simulation` → **20+ scenarios FAILED**, panic text:
  `panicked at 'attempt to divide by zero': prototypes/src/types/money.rs:193`
- Reverted; `cargo test -p simulation` → **44 passed; 0 failed**.

Note the shape: the crate that owns validation passes, the crate that consumes the data
dies. Validation is in the wrong place relative to the invariant it must protect.

## Silent defaults — the parse interface swallows typos

Four fields accept a missing/malformed value without any warning:

| Field | Site | Default on failure |
|---|---|---|
| `optout_exttrade` | `prototypes/src/prototypes/item.rs:24` | `unwrap_or(false)` |
| `request_multiplier` | `prototypes/src/types/recipe.rs:63` | `unwrap_or(1)` = honest |
| `zone` | `prototypes/src/prototypes/goods_company.rs:43` | `get_lua(...).ok()` → None |
| `order` | `prototypes/src/prototypes/base.rs:17` | `unwrap_or(String::new())` |

`request_multiplier` defaulting to 1 means a typo silently makes a dishonest enterprise
honest — it deletes the core game loop with no error. This corroborates
[[seam-perimeter-native-app]], which proved the same default by probe.

### The swallow is a TYPE-error swallow, not just an absence default

Credit: proven by lens-perimeter's probe (see [[seam-perimeter-native-app]] lines 25-49):
`request_multiplier = "not-a-number"` parses as **1** with no error; `0` and `-3` pass
`validate()` untouched.

`get_lua(..).unwrap_or(d)` discards the `Err` arm, so a wrong *type* is indistinguishable
from a *missing* field. The correct pattern already exists two files over:
`get_lua_opt(table, "n_trucks")?.unwrap_or(0)` (`goods_company.rs:41-42`) — the `?`
propagates a type error and defaults only on genuine absence.

**Full enumeration of the crate, re-grepped 2026-08-27 — SIX swallow sites, not five.**
lens-perimeter's sheet lists five; it omits `goods_company.rs:43`, which I found
independently:

| Site | Field | Default |
|---|---|---|
| `prototypes/src/prototypes/base.rs:17` | `order` | `String::new()` |
| `prototypes/src/prototypes/item.rs:24` | `optout_exttrade` | `false` |
| `prototypes/src/prototypes/goods_company.rs:43` | `zone` | `.ok()` → None |
| `prototypes/src/types/recipe.rs:63` | `request_multiplier` | `1` |
| `prototypes/src/types/zone.rs:20` | `price_per_area` | `Money::new_bucks(100)` |
| `prototypes/src/types/zone.rs:21` | `randomize_filler` | `false` |

Correct `get_lua_opt` form: only **2** sites (`goods_company.rs:41,42`). So the crate is
6-to-2 against itself — the safe pattern is the minority.

### The wrap reaches the market at runtime — PROVEN, and it does NOT panic

lens-perimeter inferred `-3i32 as u32` → 4294967293 from the source at
`goods_company.rs:23`. Confirmed end to end by probe (set flour-factory
`request_multiplier = 4` → `-3`, ran, reverted; 44/44 green after):

```
left: Some(4294967293)
 right: Some(4)  -- simulation/src/tests/scenarios/inflation.rs:95
```

Only the 2 tests asserting the literal 4 failed. **No panic, no crash.** A negative
multiplier becomes a ~4.3-billion-unit standing request that the market accepts as
ordinary demand — the worst failure mode, because nothing reports it.

**Correction to lens-perimeter's open question.** That sheet (line 51-53) suspected
`request_multiplier = 0` might panic at the unconditional
`market.requested(soul, item.id).unwrap()` (`goods_company.rs:55`). **It does not.**
`set_requested` (`market.rs:441-443`) inserts unconditionally, so qty 0 stores `Some(0)`
and the unwrap succeeds. `0` means "request nothing while still consuming" — a silent
permanent stall, not a crash. Verified by reading `market.rs:441-448`.

**Unknown FIELDS are never reported at all.** `macros.rs:114-118` warns on an unknown
prototype *type* only. Proof: `companies.lua:79` declares `max_power = "1kW"` on
solar-panel; `grep -rn "max_power" --include=*.rs .` returns **zero hits**. The field is
parsed by nothing, warns nothing, and has sat there as pure decoration.

## PRESENT-BUT-DEAD — declarations with no consumer

- **Leisure**: `LeisurePrototype` (`prototypes/src/prototypes/leisure.rs`), 1 declaration
  (`base_mod/leisure.lua`, cinema). `opening_hours` / `entry_fee` / `capacity` have **zero**
  consumers outside the prototypes crate. Loader logs "loaded 1 leisure" every run.
- **Road vehicles**: `RoadVehiclePrototype`, 2 declarations (simple_car, simple_truck).
  `grep RoadVehiclePrototype` / `RoadVehicleID` across `simulation/src` + `native_app/src`
  → **zero hits**. `max_speed` / `acceleration` / `deceleration` reach nothing.
- **Solar subtype**: `SolarPanelPrototype` has no fields of its own beyond `id`; only
  non-test mention is a doc comment. It works only because it *derefs* to
  `GoodsCompanyPrototype`. The subtype adds nothing.
- **assets_gui prototype editor**: `assets_gui/src/yakui_gui.rs` lines **264-397** are inside
  `/* */`, and the sole call site `self.properties()` is commented at **line 56**. The dead
  block references `CompanyKind::Factory { n_trucks: 1 }` and `CompanyKind::Network` —
  **neither exists**; the real enum (`goods_company.rs:11-16`) is a plain two-variant
  `Store | Factory`. `cargo check -p assets_gui` passes *because* it is commented out.
  This is the exact "commented-out reads as present" trap that has burned this repo before.

## PROVIDED — reachable, with the chain

- Production entry: `simulation/src/init.rs:44` → `prototypes::load_prototypes(base)`
  → `load.rs:18` → `parse_prototypes_str` → `Box::leak` into `PROTOTYPES` OnceLock
  (`lib.rs:105`).
- `optout_exttrade`: `item.rs:24` → `market.rs:205` (`SingleMarket::new`) → `market.rs:653`
  (`if !*optout_exttrade`). Exactly **1 of 21 items** sets it: `job-opening`
  (`items.lua:6`). Every other good is externally tradable by default.
- `request_multiplier`: `recipe.rs:63` → `souls/goods_company.rs:23`
  (`item.amount * recipe.request_multiplier`).
- `storage_multiplier`: → `goods_company.rs:46` and `:64` (halt threshold).
- `n_trucks`: → `goods_company.rs:133`, gated by `validation.rs:26-39`.
- `power_consumption`/`power_production`: → `map_dynamic/electricity.rs:75-76`.

## Adding a resource IS data-only for the market — with one caveat

`calculate_prices` (`market.rs:1081-1140`) derives every price from the recipe graph by
walking `GoodsCompanyPrototype::iter()` and `ItemPrototype::iter()`. **No hardcoded price
table.** So the charter's fifteen-resource tree and twelve recipe buildings are a pure Lua
change for pricing.

The caveat is name literals. Hardcoded `ItemID::new("...")` in non-test production code:
**13 sites**, over only **two** names — `"job-opening"` (7 sites) and `"bread"` (4 sites,
all in `souls/desire/buyfood.rs`). `GoodsCompanyID::new` in production: **zero**. So the
per-new-resource code-edit count is **0**; the debt is that two existing goods are welded
into logic. Adding Medicine costs no code edits; changing what humans eat costs 4.

Also live in production: `dbg!(price_consumption, price_workers, qty)` at
`market.rs:1124`, firing on every price computation.

## Test-fixture prototypes: a REAL second adapter (two, not one)

`load.rs:8 test_prototypes(lua: &str)` sets a **thread-local** `TEST_PROTOTYPES`
(`lib.rs:110-112`); `try_prototypes()` (`lib.rs:126-129`) checks thread-local first, then
the global OnceLock. Used at `market.rs:1168` and `market.rs:1207`. So the registry seam
has two adapters — real base_mod and ad-hoc per-test Lua — and the global is NOT a barrier
to alternative prototype sets. This is the residue of the `static mut` race fixed 2026-08-26.

**Trap:** the two `calculate_prices`/`test_match_orders` unit tests run against fixtures,
so they did NOT catch the divide-by-zero. Only the `tests/scenarios/*` suite, which loads
real base_mod via `load_prototypes("../")`, caught it. A change to base_mod is not covered
by the market unit tests.

## Counts (base_mod, this commit)

21 items · 27 companies (26 `goods-company` + 1 `solar-panel`) · 28 buildings ·
6 rolling stock · 2 road vehicles (dead) · 1 leisure (dead) · 1 freight station.
Numbers confirmed by loader output in test stdout.

**Duplicate `order` keys in companies.lua:** `h-1` ×2, `i-1` ×2, `k-1` ×2. Ordering
(`macros.rs:87-90`) sorts by `(order, id)`, so ties fall through to a **hash**, making the
toolbox listing order arbitrary but stable.

## Freight cargo — the lost invariant

`souls/freight_station.rs:35-36`: `waiting_cargo: u32`, `wanted_cargo: u32`. Bare counters
with **no `ItemID`**. Incremented at `goods_company.rs:230` and `human.rs:116`, decremented
by a magic `saturating_sub(100)` at `freight_station.rs:102-103`, displayed at
`inspect_building.rs:127-128`. A crate of bread and a crate of iron ore are the same
number. Charter's one-locomotive/one-wagon commitment lands directly here.

## Charter gap — Medicine

Charter commits Medicine as import-only. The only trade-facing prototype field is
`optout_exttrade: bool`, which is symmetric — it opts out of external trade in **both**
directions. There is no import-only representation. ABSENT; nearest existing thing is the
boolean.

Related: [[seam-economy-logistics-2026-08-27]] (market internals),
[[seam-perimeter-native-app]] (request_multiplier probe), [[seam-simwide-structure-2026-08-27]].
