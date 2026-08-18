---
name: catalogue-test-traps
description: How to drive a building for one production frame in a headless test, and the tautology every catalogue phase creates when it deletes a per-kind match
metadata:
  type: project
---

Learned building the Phase 1b behavioural recipe test and doing the Phase 2 deletions.

## Every phase that deletes a per-kind match turns its own equivalence test into a tautology

Phase 1 wrote `spec(kind).X == kind.X()` tests. The moment Phase 2 made `kind.X()` read
`spec(self).X`, three of them compared a value with itself. **Proved by mutation, not
assumed:** corrupting the Mine row in all three columns at once left
`footprint_matches…`/`inventory_capacity_matches…`/`workers_needed_matches…` all green;
only `labour::tests::output_scales_with_the_cs1_staffing_curve` noticed, and only the
vacancy change. Footprint and capacity were caught by *nothing* in the tree.

The fix in place now is `PINNED_COLUMNS` (`catalogue.rs`, test module): a second
transcription of (footprint, capacity, vacancies) × 13 in `ALL`'s order, plus a per-test
assert that the pinned row still lines up with `ALL[i]`. The lead settled the pin-vs-ground
question as a standing rule — see task_plan.md, "How a column keeps a witness"; don't
re-litigate it per phase.

**Phase 6 will hit this exact trap** when it deletes `solve_power`'s / `solve_water`'s /
`attach_heat_components`'s matches — `power_demand_matches…`, `water_demand_matches…` and
`heat_demand_matches…` are safe *only* because those matches still exist. Check the test
before deleting the match, not after.

A pure per-kind lookup has no independent behavioural witness once the table is its only
definition: `Inventory::new(kind.inventory_capacity())` and
`ConstructionSite::for_kind(kind)` both go through the same function, so "ground it in
behaviour" is not available for these the way it was for recipes.

## Driving one production frame headless

- Placement costs a tick. `apply_building_edits` is `.after(ApplyCommandsFlush)` and
  `stages.rs` sets `auto_insert_apply_deferred: false`, so the spawn lands at the
  post-Commit barrier — tick 1 places, tick 2 is the first production frame. The existing
  `mine_accumulates_coal_until_yard_is_full` encodes the same fact as "101 ticks".
- `BuildingEdit::PlacePrebuilt` + `ConstructionSimPlugin` is how you get a *finished*
  building in a world where sites exist; `attach_sites` (`construction.rs:325`) early-returns
  on `Prebuilt` and therefore also leaves the yard at `kind.inventory_capacity()` instead of
  resizing it to the material bill.
- Gates satisfied by *absence*: no labour plugin ⇒ no `Staffing` ⇒ `f == 1.0`; no water
  plugin ⇒ no `Watered` ⇒ `is_none_or` passes. Only `Powered` needs setting by hand. So a
  headless one-frame delta measures the **unscaled base rate**.
- `Inventory::add` is bounded by *shared* yard capacity, not per-resource, and five kinds
  have capacity 0 — seed a zero-capacity yard and the assertion goes vacuously green.
  Widen `Inventory::capacity` (a `pub` field) first.
- Do **not** load `CustomsSimPlugin` in a recipe fixture: the border sale drains the export
  yard by its own rules, which the `recipe` column does not model.
- f32 noise on a 0.05 delta off a 1.0 base measures ~5e-8 (observed `0.049999952`), so 1e-5
  is a safe epsilon; the file's older tests use a much looser 1e-3.

## The two collided constants, and how to demonstrate the collision

`MINE_COAL_RATE == QUARRY_GRAVEL_RATE == 0.05` and
`PLANT_COAL_BURN == HEAT_PLANT_COAL_BURN == 0.02`. To show a test actually catches a
wrong-constant reference you must *also* rebalance the constant — point the Mine row at
`QUARRY_GRAVEL_RATE` **and** set `QUARRY_GRAVEL_RATE = 0.09`. The Quarry stays green (row
and system move together); only the Mine breaks. Swapping the reference alone proves
nothing, which is what made the Phase 1 test look adequate.
