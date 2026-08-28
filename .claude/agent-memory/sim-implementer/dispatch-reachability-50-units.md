---
name: dispatch-reachability-50-units
description: A building whose door is more than 50 units from a driving lane can never be offered a truck; Map::nearest_lane without a cutoff never returns None, so only the cutoff path bites
metadata:
  type: project
---

Verified 2026-08-28 while fixing sov-abs and sov-jcl.

**`DispatchOne::query` resolves `DispatchQueryTarget::Pos` with
`map.nearest_lane(pos, kind.lane_kind(), Some(50.0))`** (`map_dynamic/dispatch.rs`).
That literal is now the named `pub const DISPATCH_LANE_CUTOFF: f32 = 50.0;` in the
same file. **A building whose door_pos is further than that from a lane of the
right kind can never be assigned a vehicle at all** — the dispatch sits in
`ToSource` with `truck: None` forever, silently.

The hardcoded `START_COMMANDS` freight station at ~(4300,6300) is exactly such a
building in every `TestCtx` city. `economy::market_update`'s `find_external` now
filters on this cutoff for that reason (sov-abs).

**`Map::nearest_lane(p, kind, None)` effectively never returns `None`** — it tries
radius 20, then 100, then falls back to a global `min_by_key` over every lane of
that kind (`map/map.rs:824-859`). So `Itinerary::route` finds *some* lane no matter
how far away, and route failure comes from the A* between lanes, not from lane
lookup. Only the explicit-cutoff path returns `None`.

**Placing a special building in a test:**
- `BuildingGen::NoWalkway { door_pos }` is **relative to the OBB centre**, and is
  then `rotated_by(axis)` (`map/objects/building.rs:107,115`). With `axis = Vec2::X`
  the observed mapping is `(x, y) -> (y, -x)`: a relative `(110, 0)` lands the door
  at `centre + (0, -110)`. Do not assume identity — assert the resulting `door_pos`.
- `WorldCommand::MapBuildSpecialBuilding` uses the OBB you pass, but the
  freight-station prototype is **160x200** (`base_mod/data.lua`), so size your OBB
  and your spacing to that or `build_special_building` refuses on overlap.

**Two road-surgery traps, both cost real time:**
- A road endpoint that lands **inside a building** makes `MapProject` resolve to
  `ProjectKind::Building` and `Map::make_connection` hits `unreachable!()`
  (`map/map.rs:175`). Stop a spur short of the footprint.
- **Removing a road destroys vehicles on it.** Truck count went 3 -> 1 in one
  probe. That silently reroutes a dispatch through the "truck vanished" branch
  and makes a bounded-retry test vacuous.
- To make a route fail while keeping every vehicle alive and the buyer's building
  standing, **build two disconnected road networks** and put seller and buyer on
  different ones. No map mutation at all. Note that `Dispatcher::query` BFSes
  outward from its target, so it can only hand out vehicles on the target's own
  network — count expected trucks per network, not per world.

**`Dispatcher::update` never purges an entity removed from `world.vehicles`.** A
drain-until-`None` pool check can therefore hand out a stale dead id, so assert
*which live trucks came back*, not the count, whenever a test killed a vehicle.

Related: [[dispatcher-truck-pool]], [[sim-test-setup-traps]],
[[testctx-always-has-freight-station]].
