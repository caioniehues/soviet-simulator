---
name: baseline-building-numbers
description: Hand-verified per-kind building numbers, re-confirmed at Phase 2 (2026-08-17) — check later phases' diffs against these instead of re-deriving from git show
metadata:
  type: project
---

# Verified baseline: the 13 building kinds

Every figure below was checked **against the live function**, not against the catalogue,
during the Phase 1 review (2026-08-17). A later phase whose diff disagrees with a number
here is a regression until the plan names it.

**Why:** re-deriving these from `git show HEAD~1:path` every phase is slow and invites
transcription error — the exact failure mode the review exists to catch.

**How to apply:** diff the phase's changed values against this table first. Only fall back
to `git show` if a row here is missing or the source function has moved.

## Canonical order (enum decl == `BuildingKind::ALL` == `save.rs` `kind_to_u8`/`kind_from_u8`)

0 Mine · 1 Quarry · 2 PowerPlant · 3 Factory · 4 Dwelling · 5 Warehouse · 6 Depot ·
7 BusStop · 8 ConstructionOffice · 9 WaterPump · 10 SewagePlant · 11 HeatPlant ·
12 CustomsOffice

All four lists agreed at every position at Phase 1. The enum has no explicit discriminants
and no `#[repr]`, so `kind as usize` is declaration order. `kind_from_u8` returns `None`
for out-of-range — a positional rewrite must preserve that (`ALL.get()`, not `ALL[]`).

## The numbers

| # | Kind | footprint | capacity | workers | default bands |
|---|---|---|---|---|---|
| 0 | Mine | 14×14 | 60 | 6 | Coal 0.0–0.05 |
| 1 | Quarry | 16×12 | 60 | 4 | Gravel 0.0–0.05 |
| 2 | PowerPlant | 18×14 | 40 | 8 | Coal 0.6–1.0 |
| 3 | Factory | 20×16 | 40 | 10 | Goods 0.0–0.1 |
| 4 | Dwelling | 12×10 | 10 | 0 | Goods 0.5–1.0 |
| 5 | Warehouse | 20×12 | 120 | 0 | Coal+Gravel+Goods each 0.2–0.6 (a fold over `ResourceKind::ALL`) |
| 6 | Depot | 22×26 | 0 | 0 | none |
| 7 | BusStop | 5×3 | 0 | 0 | none |
| 8 | ConstructionOffice | 20×22 | 0 | 0 | none |
| 9 | WaterPump | 10×8 | 0 | 0 | none |
| 10 | SewagePlant | 16×12 | 0 | 0 | none |
| 11 | HeatPlant | 18×12 | 40 | 0 | Coal 0.6–1.0 |
| 12 | CustomsOffice | 22×14 | 120 | 0 | none |

Sources at Phase 1: `buildings.rs` `footprint()` / `inventory_capacity()`, `labour.rs`
`workers_needed()`, `storage.rs` `default_policies()` — all four were hand-written matches.

**Re-confirmed unchanged at Phase 2 (2026-08-17)**, after those four matches were deleted and
the functions became one-liners over `catalogue::BUILDINGS`. The bands were confirmed by a
reconstructed-match oracle across 13 kinds × 3 resources, not by eye.

## Which of these numbers currently has a witness

Matters because an unwitnessed number can drift in a later phase with a green suite.
Status re-measured by mutation at **Phase 3 (2026-08-18)** — each row below was actually
corrupted and the suite run.

- **footprint — witnessed, by ONE test only.** `PINNED_COLUMNS` row literals in
  `catalogue.rs`'s test module. Mine 14x14 -> 14x15 fires exactly
  `footprint_column_holds_its_pinned_values` and nothing else (125 passed / 1 failed).
- **capacity — witnessed, by ONE test only.** Mine 60 -> 65 fires exactly
  `inventory_capacity_column_holds_its_pinned_values` (125/1).
- **vacancies — witnessed twice.** Mine 6 -> 7 fires the pin *and*
  `sim::labour::tests::output_scales_with_the_cs1_staffing_curve` (124/2), which hard-codes
  the staffing curve for `present = 3` out of `needed = 6`. The only structural column with a
  behavioural second witness.
- **The five production rates — witnessed behaviourally** by
  `one_frame_moves_exactly_what_the_recipe_claims`. Re-proved at Phase 3: Mine's coal rate
  0.05 -> 0.07 fires it and names the kind (`catalogue.rs:640`).
- **Storage bands — WITNESSED as of Phase 3.** F1 closed. `PINNED_COLUMNS` grew a 5th
  element and `default_policies_column_holds_its_pinned_bands` compares it per resource.
  Verified **bidirectional**: a value change (Quarry 0.05->0.5, Dwelling 0.5->0.15,
  HeatPlant 0.6->0.9), a band **added** to a kind that never had one (Depot gains Coal:
  `left: Some((0.3,0.7)) right: None`) and a band **deleted** from a kind that has one
  (Mine loses Coal: `left: None right: Some((0.0,0.05))`) all fire it. The deletion case is
  the one the implementer did not test; it works.
- **`PLANT_OUTPUT_MW`, `HEAT_PLANT_OUTPUT`, utility demands** — not in the recipe column by
  design; guarded only by the transcribed-match utility tests, which go circular at Phase 6.

### Provenance of the pin (closes the circularity objection)

The pin is the *sole* live definition of footprint, capacity and bands — nothing ties
`BUILDINGS` to pre-refactor code any more. But its literals do have provenance:
**all four columns hand-compared at Phase 3 against `git show d019ca8:` (the last commit
that still held the hand-written matches).** `d019ca8:src/sim/buildings.rs` `footprint()`
and `inventory_capacity()`, `d019ca8:src/sim/labour.rs` `workers_needed()`, and
`d019ca8:src/sim/storage.rs` `default_policies()` — 13 arms each, every arm maps to a row,
zero wildcard arms in any of the four. The band column added at Phase 3 was checked this way
specifically because a pin transcribed from the new table would be circular.
Warehouse's fold (`ResourceKind::ALL.fold(p, |a,r| a.with(r, 0.2, 0.6))`) expands to exactly
Coal/Gravel/Goods each `(0.2, 0.6)` — `ResourceKind::ALL` is `[Coal, Gravel, Goods]`,
`COUNT == 3`.

Also verified at Phase 3: the pin's first four elements are **byte-identical** across the
Phase 3 diff (parsed both sides of `git diff` and compared; silent).

## Constants

`MINE_COAL_RATE` 0.05 · `QUARRY_GRAVEL_RATE` 0.05 · `PLANT_COAL_BURN` 0.02 ·
`PLANT_OUTPUT_MW` 10.0 · `FACTORY_GOODS_RATE` 0.03 · `FACTORY_DEMAND_MW` 4.0 ·
`DWELLING_DEMAND_MW` 1.0 (`buildings.rs:110-119` at Phase 1, **shifted to `:81-90` at Phase 2**
when the two matches were deleted — values re-verified identical, only the lines moved. Do not
treat a line-number change in these constants as a finding; diff the values.)

`HEAT_PLANT_OUTPUT` 60.0 · `HEAT_PLANT_COAL_BURN` 0.02 (`heat.rs:26-28`)

`PUMP_SUPPLY` 20.0 · `SEWAGE_CAPACITY` 20.0 · `DWELLING_WATER` 1.0 · `FACTORY_WATER` 2.0
(`water.rs:19-23`)

**Four collide in pairs:** `MINE_COAL_RATE == QUARRY_GRAVEL_RATE == 0.05`,
`PLANT_COAL_BURN == HEAT_PLANT_COAL_BURN == 0.02`, and
`PUMP_SUPPLY == SEWAGE_CAPACITY == 20.0`. Swapping either member of a pair is
numerically invisible today — see [[vacuous-checks-data-driven-buildings]].

## Utility roles

- power (`solve_power`, `wires.rs:219-226`): Factory `(Industry, 4.0)`, Dwelling
  `(Housing, 1.0)`, everything else none. **REGRESSED at Phase 6 (e4c7fba): the catalogue's
  Dwelling row says `Industry` — the baseline value is Housing. Blocked; check it got fixed.**
- water (`attach_watered` `water.rs:44-58` + `solve_water` `water.rs:63-97`): Dwelling
  draws `(Housing, 1.0)`, Factory draws `(Industry, 2.0)`, WaterPump supplies 20.0,
  SewagePlant drains 20.0.
- heat (`attach_heat_components`, `heat.rs:82-99`): Dwelling consumer, HeatPlant producer.
  Consumer draw is climate-driven (`dwelling_heat_demand`), **not** a per-kind constant.

## Labour scaling asymmetry (matters at Phase 4)

`extract_resources`, `run_power_plants` and `run_factories` all scale their rate by the
labour factor `f` (`RATE * f`). **`run_heat_plants` does not** — it burns the flat
constant. A generic recipe pass that normalises this changes heat-plant behaviour.

Also: `run_heat_plants` (`heat.rs:108`) and `solve_water` (`water.rs:63`) lack the
`Without<ConstructionSite>` filter the other three carry. That is the *one* intentional
behaviour change, scheduled for Phase 4.

## The art columns (verified against 7c3b4ce at Phase 5, 2026-08-17)

Hand-compared row by row against the three deleted matches in
`git show 7c3b4ce:src/game/buildings.rs` (`kind_material`, `roof_material`, `kind_height`),
the deleted match in `git show 7c3b4ce:src/game/vehicles.rs` (`shipped_resource`) and the
hand-written BUILD list in `git show 7c3b4ce:src/game/toolbar.rs`. All 13 rows × 5 columns
identical. Pristine `src/game/art.rs` md5 at that point: `05d101aef2f709af6c0a8a593116b258`.

| # | Kind | label | wall (role, shade, metallic) | roof | height | shipped |
|---|---|---|---|---|---|---|
| 0 | Mine | MINE | SootBrick 1.0 0.0 | RUST | 6.0 | Coal |
| 1 | Quarry | QUARRY | Concrete 0.8 0.0 | CIVIC | 3.0 | Gravel |
| 2 | PowerPlant | POWER PLANT | Concrete 0.72 0.0 | RUST | 12.0 | Coal |
| 3 | Factory | FACTORY | Concrete 0.75 0.0 | RUST | 9.0 | Goods |
| 4 | Dwelling | DWELLING | Concrete 0.82 0.0 | TARRED | 11.0 | Goods |
| 5 | Warehouse | WAREHOUSE | WornEarth 0.9 0.0 | RUST | 7.0 | Goods |
| 6 | Depot | DEPOT | Timber 1.15 0.0 | CIVIC | 6.0 | Goods |
| 7 | BusStop | BUS STOP | Concrete 0.85 0.0 | TARRED | 3.0 | Goods |
| 8 | ConstructionOffice | CONSTR. OFFICE | MachineOchre 0.85 0.0 | CIVIC | 5.0 | Goods |
| 9 | WaterPump | WATER PUMP | Concrete 0.95 0.0 | RUST | 4.0 | Goods |
| 10 | SewagePlant | SEWAGE WORKS | Concrete 0.65 0.0 | RUST | 4.0 | Goods |
| 11 | HeatPlant | HEAT PLANT | SootBrick 1.2 0.0 | RUST | 11.0 | Coal |
| 12 | CustomsOffice | CUSTOMS | Concrete 0.7 0.0 | TARRED | 5.0 | Goods |

`RUST = (RustedSteel, 0.75, 0.3)` · `CIVIC = (Concrete, 0.55, 0.0)` ·
`TARRED = (Asphalt, 1.25, 0.0)`.

**7c3b4ce's `roof_material` had a wildcard arm** (`_ => RustedSteel .shade(0.75).metallic(0.3)`)
covering exactly the seven RUST kinds: Mine, PowerPlant, Factory, Warehouse, WaterPump,
SewagePlant, HeatPlant. The two named arms were Dwelling|BusStop|CustomsOffice → TARRED and
Quarry|Depot|ConstructionOffice → CIVIC. The wildcard→table expansion is faithful. This is
**the only wildcard arm the refactor has collapsed so far** — the four sim matches at Phase 2
were all exhaustive.

**`Mat` default metallic is 0.0** (`palette.rs` `Mat::new`), and `.metallic()` is plain
assignment, so `Surface::mat()`'s unconditional `.metallic(m)` reproduces 7c3b4ce's wall
chain (`Mat::new(role).shade(s)`, no metallic call) exactly. Nothing else in the old art code
touched roughness / emissive / polished / alpha, so no `Mat` default is newly relied on and
none newly overridden. **Toolbar labels do not appear anywhere else in the tree** — HUD names
kinds by `{:?}`, so there is no second label list to drift.

**Independently re-verified mechanically at the same gate (2026-08-18)**, by the mutation
reviewer: a parser extracted `BUILDING_ART` and `PINNED_ART` *separately* and compared each
against 7c3b4ce's five sources — 0 mismatches on all 65 values, twice. So the table is right
**and** the pin is an independent transcription of the same truth, not a copy of the table.
That second half is the part a hand-compare cannot establish, and it is what makes the pin
non-circular. Do the same two-sided parse at every later phase that adds a pin.
