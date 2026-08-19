---
name: catalogue-field-design
description: Why BuildingSpec's utility-demand fields split into three small types instead of one Demands struct, and which per-kind numbers still have no home in the table
metadata:
  type: project
---

`BuildingSpec` (`src/sim/catalogue.rs`) ended up with three separate `Option<...>` fields —
`power`, `water`, `heat` — instead of one `Demands { power, water, heat }` bundle, because the
three utility systems don't actually share a shape. Worth remembering before reaching for a
single unifying struct in a later phase — it would paper over a real asymmetry:

- **Power** has only a consumer side worth capturing. `solve_power` (`wires.rs`) never
  kind-matches the producer (`PowerPlant`) — it reads `PowerOutput` generically off whatever
  entity has one. So `power: Option<UtilityDemand>` (`rate: f32, priority: PriorityClass`,
  reusing `network::PriorityClass` rather than inventing a new rank type) is `Some` only for
  Factory and Dwelling.
- **Water** has both sides matched by kind inside `solve_water` itself (it directly filters
  `b.kind == BuildingKind::WaterPump` / `SewagePlant` to build its supply/drainage
  iterators), so `water: Option<WaterDemand>` needs three variants: `Draws(UtilityDemand)`
  for the two consumers, `Supplies(f32)` for the pump, `Drains(f32)` for the treatment
  works. This one is the field Phase 6 will actually need to fully retire `solve_water`'s
  per-kind matches — power's producer side has nothing to retire because it was never
  matched there.
- **Heat** has no representable rate at all for its consumer. `attach_heat_components`
  (`heat.rs:82`) only gates *membership* (which component a kind gets); the actual per-tick
  draw a dwelling faces is `dwelling_heat_demand(climate.temperature)` — a function of shared
  climate state, evaluated fresh every tick in `solve_heat`, never a per-kind constant. So
  `heat: Option<HeatDemand>` is a bare two-variant enum (`Consumer`, `Producer`) with no rate
  field — resist the urge to "complete the symmetry" with power/water by inventing one; there
  is no fixed number the running game actually uses that would belong there.

**Deliberately left out of the table, flagged for whoever scopes Phase 4:** `PLANT_OUTPUT_MW`
(`buildings.rs:113`) and `HEAT_PLANT_OUTPUT` (`heat.rs:26`) are genuine per-kind constants —
the fuelled output rate of the power plant and heat plant — but they live inside
`run_power_plants`/`run_heat_plants`, not inside any of the three functions Phase 1's brief
named (`solve_power`, `attach_watered`/`solve_water`, `attach_heat_components`). Phase 4
("one generic production pass driven by spec.recipe") will need these numbers somewhere —
either as a fourth `Demands` field or folded into `Recipe` once `ResourceKind` grows a
non-stored "flow" variant (MW/heat aren't stored commodities today, see `resources.rs`'s own
doc comment) — but adding them to `BuildingSpec` in Phase 1 would have been scope creep past
what the brief's field list and its equivalence tests could actually prove.

`default_policies: &'static [(ResourceKind, f32, f32)]` (not a built `StoragePolicies`) is
the pattern to reuse anywhere else a `const` table needs to hold something a builder method
constructs at runtime: keep the raw data shape, and give the *consuming* code (not the table)
the one-liner that folds it into the real type when Phase 2 actually reads it.

**Power's utilities phase (2026-08-19) also retired the `PowerOutput` spawn-gate match** in
`buildings.rs:180` (`apply_building_edits`) — this wasn't in the field-design note above
because it lives in `buildings.rs`, not one of the three systems Phase 1 named. `flow_output`
does *not* uniquely mark PowerPlant on its own (HeatPlant is also `Some(flow_output)`, just
`FlowOutput::Heat`) — the working read is `matches!(spec(kind).flow_output,
Some(FlowOutput::Power(_)))`, matching the variant, not just presence. `Powered` insertion
became `spec(kind).power.is_some()` directly — that one *was* a clean presence check, no
variant-matching needed, since the `power` field carries no producer variant.
