# Network topology is plain ECS today; the authority/mirror split is the recorded upgrade path

This ADR previously described a `petgraph` topology authority and a packed search mirror as
though they existed. They did not, and reading it as settled fact produced a wrong claim in
ADR 0007 before the code was checked — so it now records what is actually there.

**Today.** Road, wire and transit topology lives in plain ECS: a `RoadNode` owns
`segments: Vec<Entity>`, and nearest-node lookups (`snapped_node`, `endpoint_at`) are linear
scans with a distance filter. The union-find solver that pools power, water and heat is
rebuilt from scratch every solve and holds no persistent state. Two derived caches do exist
and are version-stamped by monotonic id counters plus a modification sum: `DockIndex` and
the `PathService` snapshot. `petgraph` and `kiddo` have been dropped from `Cargo.toml`
rather than sit unused — nothing about this decision depends on them being installed early.

**The upgrade path, unchanged.** The intended shape when scans start costing is still the
authority/mirror split the carried pathfinding doc prescribes: an editable topology
authority separate from a derived, version-stamped search structure — flexible where edits
happen, packed where search happens. Spatial lookup upgrades to hand-rolled uniform grids
(static and moving membership separated, rebuildable from authoritative state) before it
reaches a k-d tree; bounded local queries over mostly-static members favour a grid. None of
that is built, and the trigger is a benchmark showing the linear scans losing, not a date.
