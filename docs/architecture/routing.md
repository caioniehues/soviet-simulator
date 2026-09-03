# Routing

**Kind:** architecture
**Authority:** advisory
**Status:** draft
**Owner:** architecture
**Last verified:** 2026-08-28

The authority split matters: **Pathfinding** owns route request and result; **Traffic** owns load,
queue, pressure and stall; **Roads** own topology and parking; **Vehicles** own the vehicle;
**Logistics** owns the haul and cargo custody (spec register). A route authorises movement; it is
not movement.

## Current substrate

`simulation/src/map/pathfinding.rs` runs `pathfinding::directed::astar::astar` for pedestrian,
vehicle and rail paths on every `Itinerary::route` call. Cost is lane length divided by speed limit
plus deterministic noise (`common::rand::randu`); pedestrians use a 1.3× heuristic. No hierarchy,
no cache, no load term, no closures. `routing_changed_system` and `routing_update_system`
(`map_dynamic/router.rs`) re-route on topology change or stall — retry-only (`MAP-SUB-003`).

## Target design

- **Route cost from Traffic** (SPEC-TRAFFIC-007/008, draft): per-lane EWMA load; BPR
  `t = t_free · (1 + 0.15 (v/c)^4)`; Gawron damping `remembered' = 0.3·observed + 0.7·remembered`;
  Pathfinding reads the damped cost. Lane D §3.3 gives the cheapest integration: an EMA update in
  `transport_grid_synchronize`, capacity from `length / (jam_distance + mean_vehicle_length)`,
  rerouting only on topology invalidation or terminal stall (SPEC-PATHFINDING-006).
- **Hierarchy and cache** for late-game scale: cache by origin region, destination region, mode,
  vehicle/access class, **topology revision** and **traffic epoch**; keep exact local routing at
  the ends. A contraction hierarchy (`fast_paths` or similar) built on `Map::update()`, behind a
  feature flag, validated against A* results (Lane C2 §3.2).
- **Determinism:** the hierarchy must be built deterministically; cache keys include the revision
  so stale routes cannot survive a topology change ([cache standard](../engineering/performance.md)).

## Migration

1. Per-lane `{ema_load, capacity, remembered_cost}` and the EMA update (independent of routing).
2. Pathfinding reads `remembered_cost` behind a flag; compare routes.
3. Hierarchy behind a flag; compare; measure.

## Open decisions

- Authoritative lane/corridor capacity measure.
- Wait → reroute → stall thresholds.
- Behaviour of in-progress trips on topology change.

## Related

- [Pathfinding (design)](../simulation/transport/pathfinding.md)
- [Traffic (design)](../simulation/transport/traffic.md)
- [Performance](performance.md)
- [Lane D §3.3](../research/conversation-mining-2026-08-28/D-vehicles-traffic-utilities.md)
