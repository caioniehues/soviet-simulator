---
name: plugin-group-dedup-trap
description: Deleting a duplicate resource registration (Treasury/AllocationFeedback/StatePlan/ZoningFeedback) breaks ~15 in-module unit-test app() builders that never pulled in the owning plugin
metadata:
  type: project
---

Ticket #118 (R0.7 Groundwork) named four duplicate resource registrations to delete
in favour of one owner each: `Treasury`/`AllocationFeedback`/`StatePlan` on
`PlanSimPlugin` (`src/sim/plan.rs`), `ZoningFeedback` on `BuildingSimPlugin`
(`src/sim/buildings.rs:133`, not `ZoningSimPlugin` — `BuildingSimPlugin` needs it to
gate siting even headless without zoning, so it stays the owner and `zoning.rs` drops
its copy, not the other way round).

The trap: every duplicate existed *only* to satisfy `#[cfg(test)] mod tests { fn app()
-> App }` builders scattered across `sim/*.rs` that assemble a hand-picked plugin
subset and never included `PlanSimPlugin`. Deleting the duplicate from
`vehicles.rs`/`households.rs`/`save.rs` silently breaks every such test at *runtime*
(missing-resource panic), not compile time — `cargo build --lib` stays green, only
`cargo test --lib` catches it. Found and fixed 9 call sites this way: `vehicles.rs`,
`households.rs` (two — the shared `app()` and the ad hoc `reallocation_prefers_the_flat_near_the_household_jobs`
test), `save.rs`, `commute.rs` (two — `app()` and `transit_app()`), `needs.rs`,
`labour.rs`, `dispatch.rs`, `construction.rs` (two — `dispatcher_hauls_the_phase_bill_to_the_site`
and `machine_app()`), `transit.rs` (`bus_app()`). The fix is always the same: add
`PlanSimPlugin` to the subset, not re-add the resource.

Bin-level equivalent: bins that `insert_resource(Treasury { roubles: f32::INFINITY })`
*before* `add_plugins` are safe regardless — `init_resource` is a no-op if the resource
already exists, so `PlanSimPlugin` (now included by the `SimPlugins` group, since
disabling it would also drop `AllocationFeedback`) never clobbers the override. Never
disable `PlanSimPlugin` in a bench's `SimPlugins.build().disable::<...>()` chain just
because the bench predates the rouble economy — it's still the sole source of
`AllocationFeedback`, which `VehicleSimPlugin`'s `apply_vehicle_edits` reads
unconditionally.

See [[sim-plugins-group-shape]] for the group itself and per-bench disable lists.
