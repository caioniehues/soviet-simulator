---
name: catalogue-layout
description: Where each BuildingKind per-kind value actually lived before the catalogue table existed, cited file:line, for tracing a value back to its source system
metadata:
  type: project
---

Before Phase 1 (`src/sim/catalogue.rs`), a `BuildingKind`'s facts were scattered across six
files, each with its own hand-written `match self { BuildingKind::... }`. Recording where,
because chasing one of these across the tree cost real time and the next phase (2, 4, 6)
will need to find and delete each arm one at a time.

**Status: the first four are gone.** Phase 2 deleted the `footprint`, `inventory_capacity`,
`workers_needed` and `default_policies` matches — those four functions still live at the
same addresses and keep the same signatures, but each is now one line over
`catalogue::spec()`. Everything below them in this list is still a live hand-written match.
See [[catalogue-test-traps]] before deleting the next one.

- `footprint()` — `buildings.rs:51` (`impl BuildingKind` block).
- `inventory_capacity()` — `buildings.rs:71`, same impl block.
- `workers_needed()` — `labour.rs:28`, a *second* `impl BuildingKind` block in a different
  file. Bare literals (6, 4, 8, 10, 0), no named constants — unlike the rate constants
  (`MINE_COAL_RATE` etc.), nothing to reuse here; the catalogue table's literals are the
  first named home these numbers have.
- `default_policies()` — `storage.rs:73`, a free function (not an inherent method), returns
  a built `StoragePolicies`. `StoragePolicies::with` (`storage.rs:41`) is not `const fn` — it
  mutates a private array through a builder — so nothing that needs `default_policies` inside
  a `const` table can call it. `StoragePolicies` also has no `PartialEq` derive; comparing it
  for equality means going band-by-band via `.band(resource)`, not a whole-struct `==`.
- Production rate constants live in two places by kind: `buildings.rs:110-119` for
  Mine/Quarry/PowerPlant/Factory/Dwelling (`MINE_COAL_RATE`, `QUARRY_GRAVEL_RATE`,
  `PLANT_COAL_BURN`, `PLANT_OUTPUT_MW`, `FACTORY_GOODS_RATE`, `FACTORY_DEMAND_MW`,
  `DWELLING_DEMAND_MW`); `heat.rs:24-28` for HeatPlant/Dwelling-heat
  (`DWELLING_HEAT_MAX`, `HEAT_PLANT_OUTPUT`, `HEAT_PLANT_COAL_BURN`); `water.rs:19-23` for
  WaterPump/SewagePlant/Dwelling/Factory water (`PUMP_SUPPLY`, `SEWAGE_CAPACITY`,
  `DWELLING_WATER`, `FACTORY_WATER`).
- `solve_power`'s per-kind demand match — `wires.rs:219-226`, inline inside the system, not a
  standalone function. Only matches consumers (Factory, Dwelling); the plant's own output is
  read generically off any entity with a `PowerOutput` component, never kind-matched here.
- `attach_watered` — `water.rs:44-58` (component gate, Factory|Dwelling). `solve_water`'s
  per-kind match — `water.rs:63-97`, matches consumers *and* type-filters `WaterPump`/
  `SewagePlant` directly inside its query filter closures (`.filter(|(_, b)| b.kind == ...)`),
  unlike power where the producer side is generic.
- `attach_heat_components` — `heat.rs:84-101` (component gate: Dwelling→`Heated`,
  HeatPlant→`HeatOutput`). The dwelling's actual heat *rate* is not here — it's
  `heat::dwelling_heat_demand(climate.temperature)` (`heat.rs:54`), computed fresh every
  `solve_heat` tick from the shared `Climate` resource, not a per-kind constant. There is no
  fixed number to put in a spec table for this without duplicating the formula.
  **Status (2026-08-19): converted.** Now `match spec(building.kind).heat { Some(Consumer)
  => Heated, Some(Producer) => HeatOutput, _ => {} }` — one column drives both arms, no need
  to reach for `flow_output` as a second discriminant for the Producer case (the brief
  suggested checking `flow_output`, but `HeatDemand::Producer` already exists in the same
  column and is a cleaner, narrower read). This converted `catalogue.rs`'s
  `heat_demand_matches_attach_heat_components_s_per_kind_match` (`catalogue.rs:718-728`) into
  the tautology [[catalogue-test-traps]] warned about — it's now comparing a second
  hand-transcription of the same table against itself. Flagged for the orchestrator to
  delete; not deleted here (out of `heat.rs`'s file scope).
- Save discriminant — `save.rs:235-267`, `kind_to_u8`/`kind_from_u8`, a hand-paired match in
  each direction. Confirmed (2026-08-17) this order is identical to the enum's declaration
  order in `buildings.rs` — i.e. identical to what `BuildingKind::ALL` now lists in
  `catalogue.rs`. Useful for [[catalogue-field-design]]'s Phase 3 note: no reordering step
  needed before Phase 3 can switch to position-based discriminants.
- Component-attach side effects tied to kind, worth knowing exist even though Phase 1 didn't
  need to touch them: `apply_building_edits` (`buildings.rs:216-224`) attaches `PowerOutput`
  to PowerPlant and `Powered` to Factory|Dwelling. Cross-checked against `solve_power`'s own
  demand set (Factory, Dwelling) and found them in agreement — not a second source of truth
  that could drift, just a second place the same fact is currently spelled out.
