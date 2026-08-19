---
name: policy-edit-queue
description: PolicyEditQueue (ticket #118 item 2, ADR 0003 amendment) — shape, why it lives in storage.rs, and the two traps that would otherwise break green tests
metadata:
  type: project
---

`PolicyEditQueue` (`src/sim/storage.rs`) carries both edits ADR 0003's amendment names
as policy: `PolicyEdit::SetBand { building, resource, min_pct, max_pct }` (absolute
values — the HUD reads the current band and computes the shifted min/max before
pushing, same as it always did, just no longer writing the component directly) and
`PolicyEdit::AdjustRecruitmentTarget { delta: i32 }`. Applied by `apply_policy_edits`,
registered on `StorageSimPlugin` in `SimStage::ApplyCommands.after(ApplyCommandsFlush)`
— the same barrier idiom as `WireEditQueue`/`BuildingEditQueue`.

**Trap 1 — cross-module resource, not cross-module ownership.** The recruitment half
touches `RecruitmentPlan`, which `HouseholdSimPlugin` owns, not `StorageSimPlugin`. At
least five in-module test `app()` builders add `StorageSimPlugin` without
`HouseholdSimPlugin` (`construction.rs`, `customs.rs`, `dispatch.rs`, `vehicles.rs`,
`save.rs` — see [[plugin-group-dedup-trap]] for why that pattern is common here). A
plain `ResMut<RecruitmentPlan>` on the applier would panic every tick in all of them,
green at `cargo build` and red only at `cargo test --lib`, exactly the earlier
dedup trap's failure shape. Fixed with `Option<ResMut<RecruitmentPlan>>` — mirrors
`hud.rs`'s own precedent for optional-plugin resources (`climate: Option<Res<Climate>>`
in `update_population_readout`). Storage sits before Household in the `SimPlugins`
group order ([[sim-plugins-group-shape]]), so this isn't a load-order fix, just an
existence guard.

**Trap 2 — `save::snapshot`/`save::restore` want the (almost) full stack.**
`snapshot()` unconditionally calls `world.resource::<RecruitmentPlan>()`,
`resource::<DispatchQueue>()`, `resource::<StatePlan>()`/`Treasury`, `RoadIds`,
`WireIds`, `VehicleIds`, etc. — not optional. A save-round-trip test for the policy
queue needs the same plugin list `save.rs`'s own test `app()` uses (`SimPlugin,
RoadSimPlugin, BuildingSimPlugin, PlanSimPlugin, HouseholdSimPlugin, LabourSimPlugin,
CommuteSimPlugin, NeedsSimPlugin, StorageSimPlugin, VehicleSimPlugin, DispatchSimPlugin,
WireSimPlugin` — `SaveSimPlugin` itself isn't needed since `snapshot`/`restore` are free
functions, not systems). The module's existing minimal `app()` (three plugins) panics
immediately if reused for a snapshot test.

**Latency semantics change, expected and covered.** Both writes moved from immediate
(same-frame direct component/resource mutation from the HUD `Update` system) to
barrier-applied (next `SimTick`'s `ApplyCommands`). One consequence worth remembering:
two `SetBand` pushes for the same building/resource in the same unapplied window both
compute their delta off the same stale read, so the second doesn't compound — the
queue is last-write-wins per drain, not a delta accumulator. Covered by
`set_band_applies_at_the_next_barrier_not_immediately` in `storage.rs`.

See [[sim-plugins-group-shape]] and [[plugin-group-dedup-trap]] for the plugin-group
context this trap sits inside.
