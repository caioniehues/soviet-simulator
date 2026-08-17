# Phase 1b + Phase 2 report — the recipe contract, and the lookups read the table

## Baseline

`cargo test --lib` before any change: **138 passed; 0 failed; 0 ignored; finished in 0.46s.**
Confirmed myself, matches the lead's stated baseline.

---

# Phase 1b — the recipe test now means something

## What replaced what

`recipe_rates_match_the_named_production_constants` is gone. In its place,
`one_frame_moves_exactly_what_the_recipe_claims` (`catalogue.rs`) covers **all 13 kinds**
by observation: it builds a headless app, places the kind **pre-built**
(`BuildingEdit::PlacePrebuilt`), satisfies every gate the production systems read, ticks
**one** production frame, and asserts the yard's per-resource delta equals
`spec(kind).recipe` — outputs credit, inputs debit.

Test count is unchanged (one test out, one in), so the suite stays at 138.

## The fixture, and why each piece is there

- **Plugins:** `SimPlugin, BuildingSimPlugin, HeatSimPlugin, ConstructionSimPlugin`. The
  first three own every production pass that exists today (`extract_resources`,
  `run_power_plants`, `run_factories`, `run_heat_plants`). Construction is loaded so that
  "placed pre-built" *means* something — a building that genuinely is not a site — rather
  than being true only because no site-attaching observer was registered.
- **Deliberately no customs plugin.** The border sale drains the export yard by its own
  rules; the recipe column does not model it, and loading it would have made CustomsOffice
  fail for a reason that is not a recipe defect. Stated in a comment on the fixture.
- **Gates.** Fuel is a seeded yard; the electricity gate is set by hand (`Powered(true)`,
  generically — "if the entity has the component, satisfy it"); staffing and water gate on
  component *absence*, which is the documented "runs free" fixture path `extract_resources`
  already relies on and which the existing `buildings.rs` tests use. So the observed delta
  is the **unscaled base rate**, which is exactly what the table holds. The labour factor's
  scaling of that base is covered where it lives, `labour::tests::output_scales_with_the_cs1_staffing_curve`.
- **Yard widening.** Five kinds store nothing when finished, so their real yard refuses a
  seed and "consumes nothing" would hold vacuously. The fixture widens `Inventory::capacity`
  to 1000 t and seeds 1 t of each resource before the measured tick, so a rogue draw *or* a
  rogue output is observable on all 13 kinds alike.
- **Timing.** Placement commands flush at the post-Commit barrier
  (`stages.rs`, `auto_insert_apply_deferred: false`), so the placing tick produces nothing
  and the next tick is the building's first production frame. That is why the delta is
  measured across exactly one `update()`.

## Seen to fail, twice, before it was trusted

The test was green on first run, so I broke it deliberately in the two ways the Phase 1
review proved the old test could not catch.

**(1) F1(a) — the fabricated recipe.** Planted the review's own mutation on Warehouse
(`inputs: [(Coal, 99.0)], outputs: [(Goods, 99.0)]`), which previously passed all nine tests:

```
thread 'sim::catalogue::tests::one_frame_moves_exactly_what_the_recipe_claims' panicked at src/sim/catalogue.rs:490:17:
Warehouse / Coal: one frame moved 0, the catalogue claims -99
test result: FAILED. 8 passed; 1 failed; 0 ignored; 0 measured; 129 filtered out
```

**(2) F1(b) — the invisible constant swap.** Pointed the Mine's row at `QUARRY_GRAVEL_RATE`
*and* rebalanced `QUARRY_GRAVEL_RATE` to `0.09` in `buildings.rs` — precisely the R4
scenario the review said would slip through silently:

```
thread 'sim::catalogue::tests::one_frame_moves_exactly_what_the_recipe_claims' panicked at src/sim/catalogue.rs:487:17:
Mine / Coal: one frame moved 0.049999952, the catalogue claims 0.09
test result: FAILED. 8 passed; 1 failed; 0 ignored; 0 measured; 129 filtered out
```

Note the Quarry did **not** fail there — its row and its system both moved to 0.09
together, which is correct. Only the row naming the wrong constant broke, by name.

Both mutations reverted; `git diff` on `buildings.rs` is empty against HEAD and the suite is
green. `0.049999952` also confirms the float headroom: the observed noise is ~5e-8 against
a `RATE_EPSILON` of 1e-5.

## What this test now proves, stated as the Phase 4 contract

For every one of the 13 kinds, with the production plugins running and every gate
satisfied, **one tick moves through the yard exactly what `spec(kind).recipe` says and
nothing else** — including that the eight recipe-less kinds move nothing at all. Phase 4's
generic pass has to satisfy this unchanged. Two things it deliberately does not pin, so
Phase 4 knows where it is free:

- **The labour factor.** Every producer scales its rate by `f` except `run_heat_plants`,
  which burns flat (noted in the Phase 1 review too). The fixture runs at `f == 1.0`, so
  the test is blind to that asymmetry — it must be preserved on the review's word, not this
  test's.
- **The MW and heat outputs.** `PLANT_OUTPUT_MW` and `HEAT_PLANT_OUTPUT` are not in the
  table (Phase 1 declared this), so nothing here asserts them. They are flows, not yard
  stock, and are invisible to an inventory delta by construction.

No kind needed scaffolding I could not justify; nothing was weakened to make it pass.

---

# Phase 2 — the four lookups read the table

## Verified before deleting

`findReferences` on each function first — **54 references total**, every one a call site or
an import, none of which changes because no signature changes:

| Function | refs | files |
|---|---|---|
| `BuildingKind::footprint` | 11 | 7 (`game/buildings.rs`, `game/hud.rs`, `game/juice.rs`, `sim/construction.rs`, `sim/wires.rs`, + self, + catalogue test) |
| `BuildingKind::inventory_capacity` | 13 | 11 (incl. 6 `src/bin/` capture and bench binaries) |
| `BuildingKind::workers_needed` | 9 | 4 (`game/hud.rs`, `sim/labour.rs` ×3, `bin/bench_citizens.rs` ×3) |
| `storage::default_policies` | 21 | 10 (incl. 6 `src/bin/`) |

Then I checked all 13 arms of each match against the table row by row myself, rather than
trusting Phase 1's transcription. **All four columns agree on all 13 kinds**, including the
collapsed `|`-chains (`Mine | Quarry => 60.0`, `WaterPump | SewagePlant => 0.0`, the
8-kind zero-vacancy chain, the 5-kind no-band chain) and the Warehouse's
`ResourceKind::ALL` fold, which the table unrolls into three identical triples. No
disagreement of any kind to report.

## What was deleted, and what replaced it

- `buildings.rs` — both matches gone; `footprint()` and `inventory_capacity()` are
  `super::catalogue::spec(self).<field>`. −34/+5 lines.
- `labour.rs` — the vacancy match gone; `workers_needed()` is one line. −18/+5.
- `storage.rs` — the policy match gone; `default_policies()` is the fold the Phase 1 report
  specified, since `StoragePolicies::with` is not `const`. −22/+10.

**Comments migrated, not dropped.** The deleted arms carried five *why* notes that existed
nowhere else. They moved to the catalogue row that now owns the fact: the Depot's "shed
plus the two-row parking apron", the ConstructionOffice's "office hut plus the machine
apron", the CustomsOffice's "gatehouse plus inspection yard" and "the export yard: goods
wait there for the border sale", and the Dwelling's "goods land here before pantry pickup".
The two group-level notes (loading labour arriving with M3.4; a kind that stores no cargo
has nothing to band) are statements about a *column*, not a row, so they stayed on the
function doc comments.

## The trap this phase sets, and what I did about it

**Three of the four Phase 1 equivalence tests became tautologies the moment the matches were
deleted** — `footprint_matches_the_hand_written_lookup` and friends were asserting
`spec(kind).footprint == kind.footprint()`, and `kind.footprint()` is now
`spec(kind).footprint`. This is the exact defect that sent Phase 1 back (F1: a test that
compares a value with itself), reintroduced by the deletion rather than by the table.

I proved it rather than assuming it. With the Mine row corrupted in all three columns at
once (`14×14 → 14×15`, `60 → 61`, `6 → 7`) the full suite reported:

```
test sim::catalogue::tests::footprint_matches_the_hand_written_lookup ... ok
test sim::catalogue::tests::inventory_capacity_matches_the_hand_written_lookup ... ok
test sim::catalogue::tests::workers_needed_matches_the_hand_written_lookup ... ok
test sim::labour::tests::output_scales_with_the_cs1_staffing_curve ... FAILED
test result: FAILED. 137 passed; 1 failed
```

All three "equivalence" tests passed on a corrupt row. Only the vacancy change was caught,
incidentally, by a labour test that happens to staff a mine. **The footprint and capacity
columns were caught by nothing at all, on any kind.**

So the three tests are now pinned against `PINNED_COLUMNS`, a second transcription of
(footprint, capacity, vacancies) for all 13 kinds in `ALL`'s order, holding exactly what the
deleted matches held. Each test also asserts its pinned row is still aligned with `ALL`, so
a reorder cannot quietly re-pair the values. Re-running the same mutation:

```
test sim::catalogue::tests::footprint_column_holds_its_pinned_values ... FAILED
test sim::catalogue::tests::inventory_capacity_column_holds_its_pinned_values ... FAILED
test sim::catalogue::tests::workers_needed_column_holds_its_pinned_values ... FAILED

assertion `left == right` failed: Mine
  left: Vec2(14.0, 15.0)
 right: Vec2(14.0, 14.0)
```

**This is the one judgement call in the phase and the lead should confirm it.** The
alternative was deleting the three tests outright and accepting that a pure per-kind lookup
has no second witness once the table is its only definition. I chose to keep the guard
because (a) a refactor billed as behaviour-preserving should not quietly cost the suite
three real assertions, and (b) it is the same shape the Phase 1 review *demanded* for Phase
3 ("assert against hard-coded integers, never against `ALL`'s own positions"). The cost is
one extra line per new kind in R2/R4, and a deliberate rebalance edits both sides — which I
think is the right friction for a footprint, and would be the wrong friction for a rate.

**Ruling (lead, 2026-08-17): approved, kept.** The deciding fact was that `PINNED_COLUMNS`
lives inside `mod tests` — a golden record, not a second production source of truth, so the
per-kind cost lands in a fixture where it belongs. The principle is now in `task_plan.md`
under "How a column keeps a witness": pin structural constants (footprint, capacity,
vacancies), ground balance numbers (rates, utility demand) in behaviour. `PINNED_COLUMNS`
carries a doc comment saying what it does *not* prove — that it catches drift rather than
validating correctness, that a rebalance edits both sides, and that rates are deliberately
absent because `one_frame_moves_exactly_what_the_recipe_claims` guards them better — so the
next reader neither deletes it as a duplicate nor adds rate pins "for consistency".
`cargo test --lib` re-run after that comment: **138 passed; 0 failed**, fmt clean.

`default_policies_matches_the_hand_written_lookup` was **not** a tautology and was kept,
renamed to `default_policies_folds_every_band_the_row_lists` to say what it actually proves
now: that the fold delivers every band the row lists, drops none, and that
`StorageBand::new`'s clamp leaves the values alone. Its *values* are single-sourced in the
table — see "not acted on" below.

---

## Gates

```
$ cargo test --lib
test result: ok. 138 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.45s
```

Save round-trip specifically, the strongest guard in the tree:

```
test sim::save::tests::round_trip_preserves_the_sim_state_hash ... ok
test sim::save::tests::restore_into_the_same_world_is_identical ... ok
test sim::save::tests::file_round_trip_and_version_gate ... ok
test sim::save::tests::loaded_world_resumes_production_and_commutes ... ok
test sim::save::tests::mid_trip_save_keeps_the_cargo_and_the_order ... ok
test sim::save::tests::mid_commute_save_normalizes_travellers_home ... ok
test result: ok. 6 passed; 0 failed
```

`cargo clippy --lib --tests` — **nine warnings, the same nine**, none in a file this phase
touched:
`game/juice.rs:125`, `game/vehicles.rs:436`, `sim/dispatch.rs:179`, `sim/households.rs:129`,
`sim/network.rs:116`, `:117`, `:132`, `sim/water.rs:175`, `sim/wires.rs:348`.

`cargo fmt --check` clean. `cargo check --all-targets` clean — the 6 `src/bin/` binaries
that call `inventory_capacity`/`default_policies` still compile untouched.

All **seven** bench gates re-run and passing (the lookups now sit behind a table index on
hot paths, so this was worth checking rather than assuming):

| gate | mean | budget |
|---|---|---|
| `bench_sites` | 0.0941 ms | 2 ms |
| `bench_chain` | 0.0752 ms | 0.33 ms |
| `bench_citizens` | 0.5563 ms | 2 ms |
| `bench_dispatch` | 0.3531 ms | 2 ms |
| `bench_networks` | 0.0456 ms | 1 ms |
| `bench_traffic` | 1.0871 ms | 16 ms |
| `bench_transit` | 0.2780 ms | 2 ms |

## Scope

`src/sim/` only, four files: `catalogue.rs`, `buildings.rs`, `labour.rs`, `storage.rs`.
Nothing in `src/game/`, `src/bin/` or `docs/adr/`. Zero intentional behaviour changes; the
`Without<ConstructionSite>` gap in `run_heat_plants` and `solve_water` is untouched and
still Phase 4's named exception.

---

## Found, not acted on

- **The band *values* are now single-sourced with no second witness.** I pinned footprint,
  capacity and vacancies but not the storage bands: the band column is variable-length and
  a literal copy of it would be a verbatim duplicate of the table's most-tuned column. The
  fold is guarded; the numbers are not. If the reviewer wants symmetry, extending
  `PINNED_COLUMNS` with a fourth element is mechanical. Flagging rather than deciding.
- **The three utility tests (F2) are unchanged in strength** and remain the transcribed-match
  kind, correctly — `solve_power`, `solve_water` and `attach_heat_components` still carry
  their own matches, so those tests are not yet circular. **Phase 6 will make them circular
  in exactly the way Phase 2 made the lookup tests circular.** Whoever takes Phase 6 should
  read the tautology section above *before* deleting those matches, not after.
- **`heat.rs`'s `run_heat_plants` burns its coal flat, with no labour factor**, unlike the
  other three producers. The 1b test cannot see this (it runs at `f == 1.0`). Phase 4 must
  reproduce the asymmetry deliberately; it is only recorded in the Phase 1 review and now
  here.
- **`Inventory::add` is bounded by *shared* capacity, not per-resource.** A producer whose
  yard is full silently produces nothing — which is intended ("a full yard halts
  extraction") but means any future recipe test that seeds a nearly-full yard will measure
  a clamp rather than a rate. The fixture avoids it with a wide capacity; a Phase 4 test
  that forgets to will get a confusing green.
- **`spec()` indexes `BUILDINGS[kind as usize]` and will panic, not `None`, on a bad
  index.** That is sound today (the review verified declaration order at all 13 positions,
  and `every_row_sits_at_its_own_kind_s_position` pins it), but Phase 3's `kind_from_u8`
  replacement needs `ALL.get(v as usize)`, not `ALL[v as usize]`, to keep today's total
  `None` fallback for an out-of-range byte.
