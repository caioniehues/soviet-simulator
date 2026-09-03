# Network kernel

**Kind:** architecture
**Authority:** advisory
**Status:** draft
**Owner:** architecture
**Verified-at:** `266f7b2`
**Last verified:** 2026-09-03

> Shared topology concepts ≠ shared physical solver.

## Current substrate

The only network is electricity: `ElectricityCache` (`map/electricity_cache.rs`) is a `BTreeMap`
graph over `NetworkObjectID` (buildings, roads, intersections) — `graph: BTreeMap<NetworkObjectID,
Vec<NetworkObjectID>>` (`simulation/src/map/electricity_cache.rs:62`) with BFS reachability via
`pathfinding::directed::bfs::bfs_reach` in `path_exists` (`simulation/src/map/electricity_cache.rs:179-186`).
Two buildings share a network iff a road path joins them. There is
no wire object; `SPEC-ELECTRICITY-001` forbids exactly this. Water, sewage, heating and gas have
no topology at all. The road graph itself is the only shared infrastructure.

## Target design

A shared kernel that owns the shape of a network and nothing about its physics:

```text
NodeId · EdgeId · attachments (building ↔ node) · connected components · topology revision · compact CSR adjacency
```

Each domain keeps separate solver state and separate rules:

| Domain | Solver (cheapest adequate, Lane D §3) | Inertia |
|---|---|---|
| Electricity | per-island sum; priority-ordered service; per-building served/curtailed with reason | near-instant |
| Water | tree-based static head from sources; pressure per floor as a lookup; tank state | pressure, tank storage |
| Sewage | gravity DAG with per-pipe capacity and junction buffers; backpressure | gravity, buffer |
| Heating | pipe FIFO delay line + first-order building thermal ODE | transport delay, thermal mass |
| Gas (Post-1.0) | one linepack integrator per segment | linepack |
| Reservoir/hydro | mass balance; `P = ρ g Q H η` | stored head |

Cross-domain coupling happens through **explicit service results** (a pump reads the electricity
allocation result it references), never by one module mutating another's state — the utility
specs already require this (`SPEC-WATER-001`, `SPEC-HEATING-001`, `SPEC-ELECTRICITY-*`).

**Weather** is a shared stressor with an explicit interface (heating demand, hydrology, crop
cycles, road surface, utility demand). No weather spec exists; heating's temperature-responsive
demand is blocked on it.

**Topology revision** is the cache key for every derived structure — routes, components,
solver matrices. Never "probably still valid".

## Migration

1. Kernel types and a builder from the map; electricity re-expressed over the kernel with a wire
   object (this is a **replacement** of the connectivity model, not an increment — Lane D §4.6).
2. Water as the second domain (tree head), reusing the kernel unchanged.
3. Each further domain proves the kernel is sufficient or extends it deliberately.

## Open decisions

- Does pressure/head belong in the first ratified water implementation?
- Gas in scope for design work at all (charter cuts pipelines)?
- A unified weather authority in 1.0, or static heating demand first (SPEC-HEATING-003)?

## Related

- [Infrastructure (design)](../simulation/infrastructure/index.md)
- [Network architecture (design)](../simulation/infrastructure/network-architecture.md)
- [Authority boundaries](authority-boundaries.md)
- [Lane D §3](../research/conversation-mining-2026-08-28/D-vehicles-traffic-utilities.md)
