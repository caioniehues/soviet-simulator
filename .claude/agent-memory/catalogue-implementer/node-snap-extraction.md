---
name: node-snap-extraction
description: What actually moved to src/sim/node_snap.rs (ADR 0016) and what stayed put on purpose
metadata:
  type: project
---

`src/sim/node_snap.rs` holds only the comparison, not the operation: `nearest_node_within`
(nearest entity within a radius by squared distance) and `nearest_edge_within` +
`point_to_segment_distance` (nearest edge within a radius by point-to-segment distance).
`roads.rs` and `wires.rs` call these but keep everything else local: node/pole creation,
their own id counters (`RoadIds`/`WireIds`), and orphan-despawn rules (roads despawns any
orphaned node; wires despawns orphaned poles but never buildings).

What did **not** get unified, deliberately:
- Radii stayed different: `roads::SNAP_RADIUS = 6.0` vs `wires::POLE_SNAP_RADIUS = 4.0`.
  Both are still named constants passed in by the caller — the shared functions are
  radius-agnostic.
- Wires' building-snap step (`endpoint_at`'s first stage, `wires.rs:95-109`) did **not**
  move — it's a variable-per-candidate threshold (`footprint().length() * 0.5 + margin`),
  not a uniform-radius nearest lookup, so it isn't the same operation as node/pole snapping.
- roads.rs's `snapped_node` (read-only precheck used before any node is created, so the
  duplicate-edge check in `place_segment` never has a side effect) has no wires equivalent.
  wires.rs's `apply_wire_edits` calls the mutating `endpoint_at` directly for both endpoints
  *before* checking for a duplicate span — a pre-existing asymmetry between the two files,
  preserved as-is since fixing it would be a second behaviour change beyond the ticket.

`pay_gravel` in roads.rs also used the point-to-segment formula (yard-to-segment distance
for gravel sourcing, not a snap decision) — routed it through the same
`node_snap::point_to_segment_distance` since it's literally the same arithmetic, one fewer
copy to keep in sync.

See [[sim-plugins-group-shape]] for why this ticket explicitly forbade touching system
registration order in roads.rs/wires.rs's plugins.
