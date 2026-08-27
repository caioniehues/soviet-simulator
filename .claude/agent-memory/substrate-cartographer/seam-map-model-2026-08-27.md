---
name: seam-map-model-2026-08-27
description: Map model fact-sheet — land is EMPIRICALLY FLAT (clamp at terrain.rs:268), Road::make discards TooSteep, no topology revision exists anywhere; verified at 8531d3c
metadata:
  type: project
---

# SEAM: simulation/src/map/ — the map model

**Verified at commit 8531d3c, 2026-08-27.** LSP was disabled this session; all reachability
established by grep + one executed test. A map is only true for a tree state.

## The claim that would make a brief wrong

**"Terrain is hard-wired flat, the charter needs heightfields" is HALF FALSE and the false half
is the expensive one.** A full heightfield stack already exists and is live: `geom/src/heightmap.rs`
(packed u16 heights, raycast, erosion, convolution), `Environment` with five `TerraformKind`
operations (`terrain.rs:169-257`), a slope-aware road `heightfinder` honouring
`MAX_SLOPE = 0.25` (`map/mod.rs:60`, `road.rs:312-458`), and bridge-pylon positions
(`road.rs:255-272`). A brief saying "add heightfield terrain" would rebuild working machinery.

What IS true: **the generated land surface is uniformly zero.** `terrain.rs:266-270` computes
noise then clamps `if rh > 0.0 { rh = 0.0 }`, and `Environment::height` caps at `max(0.0)`
(`terrain.rs:82-84`). So all relief is *below* sea level; everything walkable is flat.

**Empirical proof** (ran the real `geom::fnoise` through the `generate_chunk` arithmetic over
40,000 sample points, temp test in `geom/tests/`, since removed):
```
samples above sea (clamped to 0): 36242, below: 3758, max surface height returned: 0
```
90.6% of the map is land, and every land sample returns exactly height 0. The terrain code is
not absent — it is **present, correct, and fed a deliberately flattened generator.** The 1.0
terrain work is plausibly a generator change plus consumer wiring, not a new subsystem.

## PROVIDED — exists and is reachable

- **Heightfield storage + terraform.** `Environment` (`terrain.rs:36-39`); `Map::terraform`
  (`map.rs:395-412`) ← `WorldCommand::Terraform` (`world_command.rs:369-380`) ←
  `native_app/src/gui/tools/terraforming.rs:97`. Full production path.
- **Slope-aware road height following.** `Road::heightfinder` (`road.rs:312-458`) called from
  `generate_points` (`road.rs:470,500`) called from `Road::make` (`road.rs:85`). Live on every
  road built.
- **Chunk-level change detection (a genuinely DEEP module).** `MapSubscribers` — interface is
  `subscribe(UpdateType) -> MapSubscriber` + `take_updated_chunks()`. **Six independent adapters**
  consume it: terrain mesh, trees, lamps, map_mesh road, map_mesh building, debug connectivity
  (`native_app/src/rendering/map_rendering/{terrain,trees,lamps,map_mesh}.rs`,
  `debug_gui/debug_window.rs:364`). Two-line interface, whole-renderer invalidation behind it.
  This one earns its keep; do not touch it.
- **Exclusive parking reservation** (`MAP-SUB-005`, holds).

## PRESENT-BUT-DEAD

- **`PointGenerateError::{TooSteep, OutsideOfMap}`** (`road.rs:65-68`). Produced by
  `heightfinder`, but `Road::make` **discards it**: `let (points, _err) = ...` (`road.rs:85`).
  Only consumer is the UI preview (`native_app/src/gui/tools/roadbuild.rs:284-294`, sets
  `is_valid = false`). The queued command carries no verdict. `road.rs:72` states the intent
  outright: *"Must not fail or it make keeping invariants very complicated be cause of road
  splitting"* — refusal is **designed out**, not merely missing.
- **Terrain coupling in lot siting.** `Lot::try_make` rejects when
  `(height - at.z).abs() > 1.0` (`lot.rs:36-39`). Correct code, inert while land is flat.
- **`Building::make` returns `Option` but never returns `None`** (`building.rs:83-159`) — the
  sole refusal is the caller's overlap check (`map.rs:253`).

## ABSENT

- **No topology revision / route invalidation anywhere.** `grep -rn "revision|topology_version|
  dirty" simulation/src/map/` returns **zero lines**. `SPEC-ROADS-003` requires altered topology
  to invalidate affected routes; `EVID-ROADS-002` names the guard. Worse, `Traversable::can_pass`
  returns **`true`** for a lane that no longer exists (`traversable.rs:60`, `unwrap_or!(lanes.get(id),
  return true)`) — a deleted lane reads as "green light", not "invalid".
- **No Ghost / Verdict / Site / Refusal type**, map side or anywhere. Confirmed for the whole sim
  in [[seam-simwide-structure-2026-08-27]] at this same commit. Map commands return `()` or drop
  their `Option` (`world_command.rs:234-300`), so no reason ever reaches the Planner.

## CONTRADICTS

- **`SPEC-ROADS-005`** ("Automatic lot creation is not accepted as the target placement contract")
  vs `map.rs:693,717-720`: `connect()` computes `let gen_lots = !matches!(segment,
  RoadSegmentKind::Arbitrary(_))` then calls `Lot::generate_along_road`. The switch that decides
  whether land parcels appear is **a geometry-representation detail**, not a Planner decision:
  splits/merges use `Arbitrary` and generate nothing; fresh straight/curved roads generate lots.
  Same call also bulldozes trees in a 40m band (`map.rs:724-729`).
- **`SPEC-CONSTRUCTION-001`** ("a UI preview alone MUST NOT authorize placement") vs the
  roadbuild path above — preview validity is computed in `native_app`, discarded at commit.
- **`SPEC-CONSTRUCTION-002`** ("Map … MUST NOT become a parallel Site owner") — Map is currently
  the *only* owner; `build_special_building`/`build_house` materialize a finished building
  immediately (`map.rs:245-329`).

## TRAPS

1. **Do not "add heightfields."** They exist. The lever is the clamp at `terrain.rs:268-270`
   plus `height()`'s `max(0.0)` at `terrain.rs:83`. Removing the clamp will immediately activate
   dormant consumers (`Lot::try_make`'s 1.0m tolerance, `heightfinder`'s `TooSteep`, pylon
   generation) — this is a **behaviour change across the whole map**, not a terrain-local edit.
2. **`Road::make` cannot currently fail by design.** Any Verdict work must first answer the
   `road.rs:72` invariant problem for road *splitting*, which calls `connect` internally.
3. **Tests run `terrain_size: 1`** (`simulation/src/tests/mod.rs:34`) — one 512m chunk. No test
   can exercise multi-chunk terrain, chunk borders, or `OutsideOfMap`.
4. **The harness already forked around the auto-lot seam.** `build_house_at` exists with the
   comment *"survives lot auto-generation being removed"* (`tests/mod.rs:67-68`), while
   `build_house_near` still depends on auto-generated lots and is used by
   `tests/vehicles.rs` and `souls/freight_station.rs:179`. Removing auto-lots breaks those.
5. `check_invariants` is a **no-op in release** (`map.rs:889-890`); the real one (`map.rs:893+`)
   only runs under `debug_assertions`.

## LUA

No map/terrain authority in `base_mod/*.lua`. Buildings declare footprint/zone via
`prototypes/`, not map geometry. The map model is Rust-only — unusual for this repo, and worth
stating because other seams are Lua-inverted.

## REFERENCE (W&R)

Not consulted for this seam; W&R `buildings_types/*.ini` governs the economic grammar
(`$STORAGE`, `$PRODUCTION`), not map topology. No requirement card numbers were in scope here.
