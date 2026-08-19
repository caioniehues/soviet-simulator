---
name: applier-total-order
description: ADR 0013's ApplierOrder SystemSet — where it lives, what it does and doesn't prove, and the save.rs fixture question it does not answer
metadata:
  type: project
---

Built 2026-08-19 (ticket #118 groundwork tail). `ApplierOrder` is a new `SystemSet` enum in
`src/sim/stages.rs`, configured `.chain().in_set(SimStage::ApplyCommands).after(ApplyCommandsFlush)`
with variants `Zones, Roads, Buildings, Policy, Wires, Households, Vehicles, Transit`. Each
plugin's edit-applier system(s) join exactly one variant instead of hand-rolled
`.in_set(SimStage::ApplyCommands).after(ApplyCommandsFlush)` (+ ad-hoc `.before`/`.after` against
siblings) — grep any applier registration for the old spelling if you find one that slipped
through. Because `ApplierOrder::X` is itself configured `.in_set(SimStage::ApplyCommands)`, a
system only needs `.in_set(ApplierOrder::X)`; it does not also need the outer set or the flush.

`Policy` (the `PolicyEditQueue` applier from [[policy-edit-queue]]) sits right after `Buildings`
— a storage-policy edit may set bands on a building placed the same tick.

`CustomsSimPlugin`'s three ProductionAndUtilities systems are `.chain()`ed too, but that set is
untouched by `ApplierOrder` (it's a different stage, `ProductionAndUtilities`, not
`ApplyCommands`) — sell_exports → buy_imports (real dependency: sell's treasury credit funds
buy's spend the same tick) → release_border_arrivals (no data dependency, placed last purely to
keep it out of the trade pair reading).

**The `full_town` fixture's `ticks(app, 3)` before pushing `VehicleEdit::BuyTruck`/`CreateShuttle`
in save.rs is not a workaround for the missing buildings→vehicles edge, and the total order does
not let it shrink.** Those two edits carry live `Entity` handles (`depot`, `mine`, `plant`) that
the test code obtains by querying the world after the building queue has actually been drained
and the entities spawned — you cannot construct `VehicleEdit::BuyTruck { depot: <entity that
doesn't exist yet> }` no matter how same-tick-ordered the appliers are declared to be. This is a
test-harness-level constraint (need a concrete `Entity` id in Rust source), orthogonal to the ECS
schedule ordering ADR 0013 is about. Checked by reading the fixture and the edit enum shapes, not
touched.

Test proving the order is real: `sim::stages::tests::a_road_placed_this_tick_is_visible_to_the_
buildings_applier_the_same_tick` — a probe system in `ApplierOrder::Buildings` counts
`RoadSegment`s. Confirmed by mutation: swapping `Roads`/`Buildings` in the chain makes it fail
(temporarily verified, then reverted before landing).
