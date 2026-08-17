# Phase 1 review — the spec shape + the table

## Verdict: **PASS**

No behaviour changed, no number drifted, no match arm was lost. All 13 rows of
`BUILDINGS` reproduce today's values exactly; I checked every row against the live
function rather than sampling. `BuildingKind::ALL`'s order is identical to `save.rs`'s,
position by position.

The findings below are **test-strength gaps, not regressions**. The table as shipped is
correct — I verified it by hand. What I could not verify is that the tests would *catch*
it being wrong in three specific places. Those matter for Phases 3 and 4, which are the
phases that start reading the fields the weak tests cover. None of them block Phase 1.

---

## Claim 1 — `ALL` order vs `save.rs` discriminants: **MATCH, all 13 positions**

Both lists in full. `catalogue.rs:24-38` (`BuildingKind::ALL`) against `save.rs:235-251`
(`kind_to_u8`) and `save.rs:253-270` (`kind_from_u8`):

| # | `BuildingKind::ALL` | `kind_to_u8` | `kind_from_u8` | |
|---|---|---|---|---|
| 0 | Mine | Mine → 0 | 0 → Mine | ✓ |
| 1 | Quarry | Quarry → 1 | 1 → Quarry | ✓ |
| 2 | PowerPlant | PowerPlant → 2 | 2 → PowerPlant | ✓ |
| 3 | Factory | Factory → 3 | 3 → Factory | ✓ |
| 4 | Dwelling | Dwelling → 4 | 4 → Dwelling | ✓ |
| 5 | Warehouse | Warehouse → 5 | 5 → Warehouse | ✓ |
| 6 | Depot | Depot → 6 | 6 → Depot | ✓ |
| 7 | BusStop | BusStop → 7 | 7 → BusStop | ✓ |
| 8 | ConstructionOffice | ConstructionOffice → 8 | 8 → ConstructionOffice | ✓ |
| 9 | WaterPump | WaterPump → 9 | 9 → WaterPump | ✓ |
| 10 | SewagePlant | SewagePlant → 10 | 10 → SewagePlant | ✓ |
| 11 | HeatPlant | HeatPlant → 11 | 11 → HeatPlant | ✓ |
| 12 | CustomsOffice | CustomsOffice → 12 | 12 → CustomsOffice | ✓ |

All three agree with the enum declaration order at `buildings.rs:18-48`. The enum carries
no explicit discriminants and no `#[repr]`, so `kind as usize` (used by `spec()`,
`catalogue.rs:307`) is declaration order too. `kind_from_u8`'s `_ => return None` is a
total fallback for out-of-range bytes, not a silent default onto a kind — Phase 3's
positional lookup must preserve that `None`, since `ALL.get(v as usize)` gives it for free
but `ALL[v as usize]` would panic.

**Phase 3 can lean on this.** See finding F3 for what it must *not* lean on.

## Claim 2 — every number: **all 13 rows correct, no drift**

Checked against the live source, every row:

**`footprint` vs `BuildingKind::footprint()` (`buildings.rs:51-70`)** — 13/13:
Mine 14×14 ✓ · Quarry 16×12 ✓ · PowerPlant 18×14 ✓ · Factory 20×16 ✓ · Dwelling 12×10 ✓ ·
Warehouse 20×12 ✓ · Depot 22×26 ✓ · BusStop 5×3 ✓ · ConstructionOffice 20×22 ✓ ·
WaterPump 10×8 ✓ · SewagePlant 16×12 ✓ · HeatPlant 18×12 ✓ · CustomsOffice 22×14 ✓

**`inventory_capacity` vs `buildings.rs:71-88`** — 13/13:
Mine 60 ✓ · Quarry 60 ✓ · PowerPlant 40 ✓ · Factory 40 ✓ · Dwelling 10 ✓ · Warehouse 120 ✓ ·
Depot 0 ✓ · BusStop 0 ✓ · ConstructionOffice 0 ✓ · WaterPump 0 ✓ · SewagePlant 0 ✓ ·
HeatPlant 40 ✓ · CustomsOffice 120 ✓

**`workers_needed` vs `labour.rs:28-46`** — 13/13:
Mine 6 ✓ · Quarry 4 ✓ · PowerPlant 8 ✓ · Factory 10 ✓ · Dwelling 0 ✓ · the eight-kind
`|`-chain (Warehouse, Depot, BusStop, ConstructionOffice, WaterPump, SewagePlant,
HeatPlant, CustomsOffice) all 0 ✓ — every arm of that chain is present as its own row.

**`default_policies` vs `storage.rs:73-95`** — 13/13:
Mine Coal 0.0–0.05 ✓ · Quarry Gravel 0.0–0.05 ✓ · PowerPlant Coal 0.6–1.0 ✓ ·
Factory Goods 0.0–0.1 ✓ · Dwelling Goods 0.5–1.0 ✓ · Warehouse Coal/Gravel/Goods each
0.2–0.6 ✓ (see F4) · Depot, BusStop, ConstructionOffice, WaterPump, SewagePlant all empty ✓
· HeatPlant Coal 0.6–1.0 ✓ · CustomsOffice empty ✓

**Production constants** (`buildings.rs:110-119`, `heat.rs:26-28`) — all values confirmed
at their definition *and* at their use site:
`MINE_COAL_RATE` 0.05 → `extract_resources` (`buildings.rs:255`) ✓ ·
`QUARRY_GRAVEL_RATE` 0.05 → `buildings.rs:258` ✓ ·
`PLANT_COAL_BURN` 0.02 → `run_power_plants` (`buildings.rs:288`) ✓ ·
`FACTORY_GOODS_RATE` 0.03 → `run_factories` (`buildings.rs:324`) ✓, inputs empty ✓ (declared) ·
`HEAT_PLANT_COAL_BURN` 0.02 → `run_heat_plants` (`heat.rs:116`) ✓

Note for Phase 4: every one of those systems scales the constant by the labour factor `f`
(`MINE_COAL_RATE * f` etc.), except `run_heat_plants`, which burns the flat constant with
no `f`. The table stores the unscaled base, which is right — but the generic pass must
reproduce that asymmetry, not normalise it.

**`solve_power` (`wires.rs:219-226`)** — Factory → `(Industry, FACTORY_DEMAND_MW=4.0)` ✓ ·
Dwelling → `(Housing, DWELLING_DEMAND_MW=1.0)` ✓ · `_ => None` ✓ (all other 11 rows `None`).

**`attach_watered` (`water.rs:44-58`) + `solve_water` (`water.rs:63-97`)** —
Dwelling → `Draws(Housing, DWELLING_WATER=1.0)` ✓ · Factory → `Draws(Industry, FACTORY_WATER=2.0)` ✓ ·
WaterPump → `Supplies(PUMP_SUPPLY=20.0)` ✓ · SewagePlant → `Drains(SEWAGE_CAPACITY=20.0)` ✓ ·
all others `None` ✓. `attach_watered`'s `Factory | Dwelling` gate is exactly the `Draws(_)`
set — the implementer's cross-check holds.

**`attach_heat_components` (`heat.rs:82-99`)** — Dwelling → `Consumer` ✓ · HeatPlant →
`Producer` ✓ · `_ => {}` ✓. The `if !has_heated` / `if !has_output` guards are idempotence
checks, not per-kind data; correctly not modelled.

## Claim 3 — test strength: **6 of 9 proven load-bearing by mutation, 1 partly vacuous**

I mutated the table row by row and re-ran. Six mutations produced six distinct failures,
each naming the exact offending kind:

| Mutation | Test that caught it |
|---|---|
| Mine footprint 14×14 → 14×15 | `footprint_matches…` — "Mine" |
| Quarry capacity 60 → 61 | `inventory_capacity_matches…` — "Quarry" |
| PowerPlant workers 8 → 7 | `workers_needed_matches…` — "PowerPlant" |
| Warehouse: drop the Gravel band | `default_policies_matches…` — "Warehouse / Gravel" |
| Factory power priority Industry → Housing | `power_demand_matches…` — "Factory" |
| Dwelling heat Consumer → Producer | `heat_demand_matches…` — "Dwelling" |

Tree reverted afterwards and confirmed byte-identical (md5 `0aae79be…` before and after);
full suite re-run green.

`every_row_sits_at_its_own_kind_s_position` is also genuine — it ties `BUILDINGS[i]` to
`ALL[i]` *and* to `kind as usize`, which is what makes `spec()` sound.

The two I could not break are covered in F1 and F2.

## Claim 4 — nothing reads the table: **confirmed, by LSP not grep**

`findReferences` on `BUILDINGS` (`catalogue.rs:110`) returns **3 references, all inside
`catalogue.rs`**: its own definition, `spec()` at :307, and the position test at :318.
`spec()` itself has zero external references. Grep could not have proven this; LSP can, and
I health-checked the server first — `findReferences` on `BuildingKind::footprint`
(`buildings.rs:51`) returns 11 references across 7 files, so the index is live and the empty
result for `spec` is a real absence, not a cold index.

`footprint()` retaining 10 non-catalogue call sites also confirms the hand-written lookups
are still the live path, untouched.

`git status` shows **no modification to `buildings.rs`, `labour.rs`, `storage.rs`,
`wires.rs`, `water.rs` or `heat.rs`** — they are not in the diff at all, so "untouched
byte-for-byte" is proven trivially rather than by inspection.

`src/sim/mod.rs` gained exactly one line, `pub mod catalogue;`, in alphabetical position.
`catalogue.rs` contains no `Plugin` impl, no `add_systems`, no `add_observer` — grepped and
confirmed. No system was registered; no schedule changed.

## Claim 5 — omissions declared, not silent: **confirmed**

`PLANT_OUTPUT_MW` (10.0) and `HEAT_PLANT_OUTPUT` (60.0) are genuinely absent from
`catalogue.rs` — grepped, zero hits. Not half-present: no field exists for them, so nothing
can read a stale value. The report declares both. `Powered`/`Watered` attachment is likewise
genuinely absent as a field, and I confirmed the implied sets agree (see Claim 2).

One derivation-vs-literal conversion exists and is declared: Warehouse. See F4.

## Scope

`src/` diff is exactly two files: `src/sim/mod.rs` (+1) and new `src/sim/catalogue.rs`.
Nothing in `src/game/`, nothing in `src/bin/`. Clean.

`docs/adr/0017-a-building-is-its-product.md` appeared as untracked during this review and
was not in the session-opening snapshot. **Not a phase-1 scope violation** — `findings.md:30`
cites it as the research track's decision record, so it belongs to the researcher, not the
catalogue implementer. Noting it only because it crossed the line mid-review; the lead may
want to confirm the attribution.

## Gates

- `cargo test --lib` — **138 passed; 0 failed; 0 ignored**, 0.47s. Run by me, twice (once
  before mutation, once after revert). Matches the claimed 129 + 9.
- `cargo build --lib` — clean, finished, no warnings.
- `cargo clippy --lib` — **4 warnings, the same four**: `game/juice.rs:125`,
  `game/vehicles.rs:436`, `sim/dispatch.rs:179`, `sim/households.rs:129`. Unchanged.
- `cargo fmt --check` — clean, exit 0.
- Save round-trip: `round_trip_preserves_the_sim_state_hash` green — but see F3 for why that
  is weaker evidence than it looks.

Minor inaccuracy in the report: it says `cargo clippy --lib --tests` shows "the four
pre-existing warnings". `--tests` actually surfaces five more (`wires.rs:348`, `water.rs:175`,
`network.rs:116/117/132`). All five are pre-existing test-profile lints and **none** is in
`catalogue.rs`, so the substantive claim — nothing new from this change — holds.

---

## Findings, most severe first

### F1 — `recipe` is unguarded for 8 of 13 kinds, and constant-swaps are invisible (Medium; blocks nothing now, matters at Phase 4)

`catalogue.rs:369-399`, `recipe_rates_match_the_named_production_constants`.

Two independent weaknesses, both demonstrated:

**(a) It checks 5 kinds and never asserts the other 8 are `NO_RECIPE`.** I planted a
fabricated recipe on Warehouse — `inputs: [(Coal, 99.0)], outputs: [(Goods, 99.0)]` — and
**all nine tests passed**. Warehouse, Dwelling, Depot, BusStop, ConstructionOffice,
WaterPump, SewagePlant and CustomsOffice have no assertion that their recipe is empty.

**(b) Both sides of the assertion name the same constant, and four constants collide.**
`MINE_COAL_RATE == QUARRY_GRAVEL_RATE == 0.05` and
`PLANT_COAL_BURN == HEAT_PLANT_COAL_BURN == 0.02`. I swapped the Mine's rate to
`QUARRY_GRAVEL_RATE` and the PowerPlant's to `HEAT_PLANT_COAL_BURN` in the table — **tests
passed**, because the test compares the table's constant against the same constant, and the
values are numerically equal anyway.

The table is *correct today* — I verified all five recipe rows by hand against their use
sites. The risk is forward-looking and concrete: when R4 rebalances `QUARRY_GRAVEL_RATE`,
a Mine row that names the wrong constant silently follows it, and nothing fails.

*Input that would show it:* set `QUARRY_GRAVEL_RATE = 0.09` in a tree where the Mine row
names it; the mine's coal output changes and no test complains.

*Suggested guard for Phase 4 (not for me to write):* loop `BuildingKind::ALL` and assert
the 8 non-producers have empty inputs and outputs, and assert the producers' rates against
a literal (`0.05`, `0.02`, `0.03`) rather than against the constant the table itself uses.

### F2 — three tests assert against a re-transcribed copy of the match, not the live match (Low)

`catalogue.rs:402-448`. `power_demand_matches…`, `water_demand_matches…` and
`heat_demand_matches…` each rebuild the expected match **inside the test body** rather than
calling the live function. They caught my table mutations (proven above), so they are not
worthless — but they cannot catch a *mis-reading* of the live solve that the implementer
then reproduced in both the table and the test copy.

This is unavoidable at Phase 1 — `solve_power` is a Bevy system taking `Query` params and
cannot be called from a unit test without a world. I closed the gap manually instead: all
three matches are verified line by line against `wires.rs:219-226`, `water.rs:44-97` and
`heat.rs:82-99` in Claim 2, and they are correct. Recording it so the next reviewer does not
mistake these three for the same strength as the `footprint`/`workers_needed` tests.

### F3 — the save round-trip hash cannot catch a symmetric reorder (Low; a Phase 3 instruction)

`save.rs:1261-1278`. The test snapshots, restores into a fresh world and compares
`state_hash`. Encode (`kind_to_u8`) and decode (`kind_from_u8`) both run **in the same
binary**, so a reorder applied consistently to both round-trips perfectly while every
save file already on disk decodes to the wrong kinds. The lead's worry is correct: this
test would not catch it.

Phase 1 is safe because the order is right — hand-verified above, all 13 positions. But
**Phase 3 must not treat the green hash test as evidence**. The discriminant-stability test
the plan already names (pinning `Mine == 0 … CustomsOffice == 12` against literals) is the
only thing that would actually guard this, and it needs to assert against hard-coded
integers, not against `ALL`'s own positions — otherwise it is vacuous in the same way as F1(b).

### F4 — Warehouse is the one row where a fold became literals (Info)

`storage.rs:81-83` folds `ResourceKind::ALL` into a 0.2–0.6 band for every resource;
`catalogue.rs:206-210` unrolls that into three literals. This is the only place the table
represents code structure differently rather than mirroring it, and it is declared in a
comment.

It is currently safe, and safe for a good reason: `default_policies_matches…` loops
`ResourceKind::ALL` and compares against the live function, so adding a fourth resource
makes the live fold produce a band the table lacks and **the test fails loudly**. Verified —
that is exactly the mutation that produced "Warehouse / Gravel". Good design; noted only so
a later phase does not "simplify" the test into iterating the table's own triples, which
would destroy the guard.

### F5 — ADR 0017 appeared mid-review (Info, resolved)

Attributed to the research track via `findings.md:30`. No action.

---

## What I checked and found clean

- All 13 rows × 8 fields against live source (Claim 2) — no value differs by any amount.
- `ALL` order vs both save discriminant functions, position by position (Claim 1).
- Every arm of every collapsed match enumerated: `workers_needed`'s 8-kind `|`-chain,
  `default_policies`'s 5-kind `|`-chain, and the three `_ =>` wildcards in the utility
  solves all map to explicit rows. No kind dropped, no default flipped.
- Zero external readers of `BUILDINGS`/`spec` (LSP, health-checked).
- Six of nine tests proven load-bearing by mutation; tree restored and re-verified.
- 138/138 green, build clean, 4 clippy warnings unchanged, fmt clean.
- Scope: two `src/` files, both expected.
