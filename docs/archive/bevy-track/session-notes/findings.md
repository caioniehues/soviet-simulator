# Findings — M3.3–M3.7

## Codebase map (verified 2026-08-17)
- Stage pipeline (`sim/stages.rs`): ApplyCommands → … → AllocationAndDispatch → Routing → MovementAndTransfers → … Two flush barriers only (ApplyCommands open; post-Commit).
- `sim/storage.rs`: `StoragePolicies` bands per resource; `deficit()`/`surplus()` in tonnes vs shared capacity. Defaults: producers (0,0.05), plant coal (0.6,1), dwelling goods (0.5,1), warehouse all (0.2,0.6), factory goods (0,0.1). Depot: none.
- `sim/vehicles.rs`: `VehicleAsset {id, kind, home_depot, cargo_class}`; pawn = `ActiveVehicle` linked via `PawnOf`/`ActivePawn` relationship (linked_spawn). `ShuttleAssignment` = M1 stub. BFS `find_route` over `RoadNode.segments`; `nearest_node` DOCK_RADIUS=40. TRUCK_SPEED 12, CAPACITY 10 t, TRANSFER_RATE 0.5 t/frame. `depot_slot_pos`, DEPOT_SLOTS=6.
- `sim/save.rs`: SAVE_VERSION=2, postcard columns; VehicleRow {id, home_depot idx, class, assignment}. Pawns transient (respawned). Policies NOT saved yet (defaults reattached via observer + Has guard — loader-set values survive).
- Clock: 60 sim Hz, SECS_PER_PASS=1/60; dt per tick tiny → dirt-road 100 m trip ≈ 900 ticks.
- Game: `game/vehicles.rs` shuttle tool + truck dress/easing (renders any `ActiveVehicle`); `game/hud.rs` panels (tool, population, inspect, fill bars). ToolMode::Shuttle key 5.
- Fixtures: src/bin/{bench_chain, bench_citizens, capture, capture_m2}.rs use BuyTruck + CreateShuttle.

## Design decisions (M3.3+)
- FreightOrder lives in a Resource `DispatchQueue(Vec<FreightOrder>)` (no entities; saved in M3.5).
- Score `priority/(1+d²·w)`, d = Euclidean between buildings, w = 1/150² (haul halves value at 150 m) — meaningful, unlike CS1's 1e-7.
- Priority = deficit fraction of the min line.
- Round-robin: one ResourceKind matched per matching frame; matcher runs every MATCH_INTERVAL ticks.
- Inbound qty on existing orders subtracts from deficit; reserved outbound subtracts from surplus (reservation exclusivity).
- Order qty = one truckload max.
- M3.5 shuttle sugar: CreateShuttle sets source band (0,0) + dest band (0.9,1) for the resource → dispatcher reproduces continuous haul.

## Spec anchors
- logistics.md: buckets not offers; dock loading rate is a real bottleneck; round-robin amortisation adopted from CS1; distance weight must matter.

## B4+ repo survey (agent, 2026-08-17)
Report artifact: https://claude.ai/code/artifact/3db2ef12-792b-4204-b82d-0729b9a5c21a
- Simutrans (goldmine): bucket-heap Dijkstra (weights <200 in flat Vec buckets,
  rest BinaryHeap); transfer cost constants WEIGHT_HALT=1/WAIT=8/MIN=9 (B5);
  connected-component pre-filter before search (we already do this in matcher);
  transit line model = schedule + convoy vec + goods-category index (B5).
- godot-road-generator: offset_curve() in road_segment.gd — parallel lane
  geometry via 4-point sliding window over baked bezier, half-angle bisector
  offsets at joints. Port to glam for B4.5 curved lanes.
- contefran/traffic: IDM car-following (5 params, 1 formula), MOBIL lane change
  — candidate upgrade beyond CS1 reservation if needed (B4.3+).
- SlimCityGame: statistical ambient traffic (aggregate flows, cosmetic sprites)
  alongside real agents — option for visual density at scale.
- Bevy 0.19-compatible: bevy_ecs_tilemap, bevy_mod_index, bevy_async_task,
  bevy_defer, bevy_ecs_ldtk, bevy_debugger_mcp. bevy_lunex still 0.18 (skip).
