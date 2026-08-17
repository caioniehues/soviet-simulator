# Phase 1 report — the spec shape + the table

## Baseline

`cargo test --lib` before any change: **129 passed; 0 failed; 0 ignored; finished in 0.46s.**
Matches `findings.md`'s recorded baseline exactly.

## What was built

- `src/sim/catalogue.rs` (new file), registered in `src/sim/mod.rs` (`pub mod catalogue;`,
  alongside the other sim modules — one-line addition, no other line in `mod.rs` touched).
- `impl BuildingKind { pub const COUNT: usize = 13; pub const ALL: [BuildingKind; COUNT] = [...]; }`
  — a second inherent-impl block for `BuildingKind`, living in `catalogue.rs` rather than
  `buildings.rs`, in the exact declaration order `buildings.rs`'s enum already uses (which is
  also `save.rs`'s existing `kind_to_u8`/`kind_from_u8` order — Phase 3 can lean on that).
- `pub struct Recipe { inputs: &'static [(ResourceKind, f32)], outputs: &'static [(ResourceKind, f32)] }`,
  exactly the shape asked for.
- `pub struct BuildingSpec` with fields `kind`, `footprint`, `inventory_capacity`,
  `workers_needed`, `default_policies`, `recipe`, `power`, `water`, `heat` — see "Field
  design notes" below for the three utility-demand fields, which needed small supporting
  types not named in the brief.
- `pub const BUILDINGS: [BuildingSpec; BuildingKind::COUNT]`, one row per kind, in `ALL`'s
  order, and `pub fn spec(kind: BuildingKind) -> &'static BuildingSpec { &BUILDINGS[kind as usize] }`.
- Nine tests in `catalogue.rs`'s own `#[cfg(test)] mod tests`, one per field-group, each
  looping `BuildingKind::ALL` and asserting the table agrees with the function/match it will
  replace. No call site outside `catalogue.rs` changed — every `match self { BuildingKind::… }`
  arm in `buildings.rs`, `labour.rs`, `storage.rs`, `wires.rs`, `water.rs`, `heat.rs` is
  untouched, byte-for-byte.

## Field design notes — three fields the brief named but didn't fully shape

The brief asked for "the utility demands a kind has today (power MW, water, heat)". Three
different systems shape that differently, so three small types carry it, each named after
the function whose match it mirrors:

- `power: Option<UtilityDemand>` — `UtilityDemand { rate: f32, priority: PriorityClass }`
  (`PriorityClass` reused from `network.rs`, not reinvented). Mirrors `solve_power`'s
  `demands` construction exactly: `Some` only for `Factory` (`FACTORY_DEMAND_MW`,
  `PriorityClass::Industry`) and `Dwelling` (`DWELLING_DEMAND_MW`, `PriorityClass::Housing`).
  `PowerPlant` gets `None` — `solve_power` never kind-matches the plant itself, it just reads
  every entity carrying a `PowerOutput` component generically, so there is nothing per-kind
  to capture there.
- `water: Option<WaterDemand>` — `WaterDemand::Draws(UtilityDemand)` for the two consumers
  (`Dwelling`/`DWELLING_WATER`/`Housing`, `Factory`/`FACTORY_WATER`/`Industry`, matching both
  `attach_watered`'s component gate and `solve_water`'s demand list, which agree on the same
  two kinds), `WaterDemand::Supplies(f32)` for `WaterPump` (`PUMP_SUPPLY`), `WaterDemand::Drains(f32)`
  for `SewagePlant` (`SEWAGE_CAPACITY`) — `solve_water` type-matches those two kinds directly
  to build its supply/drainage iterators, so unlike power, the producer side genuinely needs
  representing here for Phase 6 to have something to read.
- `heat: Option<HeatDemand>` — `HeatDemand::Consumer` for `Dwelling`, `HeatDemand::Producer`
  for `HeatPlant`, mirroring `attach_heat_components`'s match exactly. **No rate field**: a
  dwelling's per-tick heat draw is `heat::dwelling_heat_demand(climate.temperature)`, a
  function of the shared climate resource, not a per-kind constant — there is no fixed number
  to put in the table without either duplicating the formula (drift risk) or fabricating a
  number that isn't what the game actually uses. `HEAT_PLANT_OUTPUT` and `HEAT_PLANT_COAL_BURN`
  live in `run_heat_plants`, not `attach_heat_components`, and the brief's field list named
  only the latter — so the plant's fuel burn is captured in `recipe.inputs` (a real, constant,
  per-kind number) and its heat output rate is deliberately left uncaptured. See "Left out"
  below.

`default_policies` is `&'static [(ResourceKind, f32, f32)]`, not a built `StoragePolicies`:
`StoragePolicies::with` is not a `const fn` (it mutates a private array field through a
public builder method), so a `BuildingSpec` inside a `const` table cannot hold an actual
`StoragePolicies` value. The triples are the same (resource, min_pct, max_pct) shape
`storage::default_policies` already builds from; Phase 2's one-liner will be
`self.default_policies.iter().fold(StoragePolicies::default(), |p, &(r, lo, hi)| p.with(r, lo, hi))`
(not written this phase — nothing reads the table yet).

## Fields left out, and why

- **Producer rates for power and heat** (`PLANT_OUTPUT_MW`, `HEAT_PLANT_OUTPUT`). Both are
  genuine per-kind constants, but neither `solve_power` nor `attach_heat_components` (the
  functions the brief named) matches on kind to read them — `run_power_plants` and
  `run_heat_plants` do, and those weren't named. I left them out rather than silently
  widening scope. They're easy to add in whichever phase (4, most likely, when the four
  hand-written production systems collapse into one recipe-driven pass) actually needs them —
  worth flagging to whoever plans Phase 4's field list.
- **`Powered`/`Watered` component attachment as its own field.** `apply_building_edits`
  (`buildings.rs:216-224`) attaches `Powered` to `Factory | Dwelling`, and `attach_watered`
  (`water.rs:49-57`) attaches `Watered` to the same two kinds. Both sets are already implied
  by `power.is_some()` / `water` being a `Draws(_)` variant respectively — cross-checked by
  hand, they agree with `solve_power`'s and `solve_water`'s own demand sets today, so adding
  a redundant boolean field would just be a second place the same fact could drift out of
  sync. Not a bug, just noted as a fact I verified rather than assumed.

## Test output

```
$ cargo test --lib
test result: ok. 138 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.46s
```

129 baseline + 9 new (`every_row_sits_at_its_own_kind_s_position`,
`footprint_matches_the_hand_written_lookup`, `inventory_capacity_matches_the_hand_written_lookup`,
`workers_needed_matches_the_hand_written_lookup`, `default_policies_matches_the_hand_written_lookup`,
`recipe_rates_match_the_named_production_constants`, `power_demand_matches_solve_power_s_per_kind_match`,
`water_demand_matches_attach_watered_and_solve_water_s_per_kind_match`,
`heat_demand_matches_attach_heat_components_s_per_kind_match`). Same wall time as baseline —
138 tests still under 0.46s, no measurable overhead from the new const table.

`cargo build --lib`: clean, no warnings. `cargo clippy --lib --tests`: the four pre-existing
warnings (`game/juice.rs`, `game/vehicles.rs`, `sim/dispatch.rs`, `sim/households.rs`) are
unchanged; nothing in `catalogue.rs` triggers a new one. `cargo fmt --check`: clean after one
`cargo fmt` pass (a single multi-line `assert_eq!` collapsed to one line — no logic change).

## Things found on the way, not acted on

- `BuildingKind::ALL`'s order matches `save.rs`'s `kind_to_u8`/`kind_from_u8` order exactly
  (both are enum-declaration order, unverified as *intentional* alignment but true today) —
  good news for Phase 3, which can lean on `ALL`'s position instead of hand-pairing two
  matches, without a reordering step first.
- Confirmed by direct read (not just findings.md's word for it): `run_heat_plants`
  (`heat.rs:108`) and `solve_water` (`water.rs:63`) really do lack the `Without<ConstructionSite>`
  filter that `extract_resources`, `run_power_plants`, `run_factories` and `plan_labour` all
  carry. Left untouched per the brief — Phase 4's named exception.
- No second definition of any per-kind value disagreed with another. Specifically checked:
  `apply_building_edits`'s `Powered`/`PowerOutput` attachment set against `solve_power`'s
  demand set, and `attach_watered`'s gate set against `solve_water`'s demand set — both pairs
  agree today. Nothing to report as a divergence bug.
