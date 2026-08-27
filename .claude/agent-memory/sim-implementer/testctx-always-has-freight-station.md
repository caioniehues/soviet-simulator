---
name: testctx-always-has-freight-station
description: TestCtx::new() unconditionally seeds a real RailFreightStation + ExternalTrading zone, and find_external has no distance cutoff, so ext-trade is live in every scenario unless actively demolished
metadata:
  type: project
---

`Simulation::new_with_options` (simulation/src/lib.rs) unconditionally replays a
hardcoded `START_COMMANDS` JSON script after `init_funcs()`. That script builds a
real `RailFreightStation` + `ExternalTrading` zone at a fixed far-away map position
(~4300,6300) on EVERY `Simulation`, `TestCtx::new()` included. There is no
`SimulationOptions` flag to skip it.

`market_update`'s `find_external` closure (economy/mod.rs) does
`world.freight_stations.iter().min_by_key(distance)` with NO reachability check
and NO distance cutoff. With exactly one freight station in the world (the
default), it is always returned as `Some`, regardless of how far away or
road-disconnected a test's own buildings are. Verified: a flour-factory built at
`(150,20)` with no road anywhere near `(4300,6300)` still ext-traded against it.

**Consequence:** any test relying on "no freight station exists" for its
assertions is confounded unless it actively removes the default one.
`hoarding.rs::scenario_0151_inflated_request_hoards_honest_does_not` (2026-08-26)
does NOT remove it and silently ext-trades underneath its own bounded-hoard
assertion — traced with `--nocapture`, saw `seller: FreightStation ...
money_delta: -1$` inside its delivery loop. The assertion still happens to pass,
but not purely from the mechanism the test's own doc comment claims.

**How to actually remove it (two-step, `remove_building` alone is NOT enough):**
```rust
let station_building = ctx.g.map().buildings().iter()
    .find(|(_, b)| matches!(b.kind, BuildingKind::RailFreightStation(_)))
    .map(|(id, _)| id).unwrap();
ctx.apply(&[WorldCommand::MapRemoveBuilding(station_building)]);
ctx.tick(); // load-bearing: see below
assert!(ctx.g.world().freight_stations.is_empty());
```
`Map::remove_building` deletes the map building but does NOT touch
`world.freight_stations` (the ECS entity). `souls::freight_station::freight_station_system`
only kills the orphaned `FreightStationEnt` on the tick AFTER it notices its
building is gone (`if !map.buildings.contains_key(station.building) { cbuf.kill(me); }`).
Skip the `ctx.tick()` and `find_external` still matches the entity that same tick.

Full helper: `simulation/src/tests/scenarios/inflation.rs::remove_default_freight_station`.

**Recommended but not filed as this ticket's scope:** `find_external` should have
a distance cutoff or a road-reachability check — right now any city, however
isolated, has a working zero-cost export market the instant the default station
exists (which is always).

Related: [[dispatcher-truck-pool]], [[sim-test-setup-traps]].
