# Research: terrain, water and hydro retrofit (#108)

Resolves the cut-line exception from #90: heightmap terrain + gameplay-grade water +
hydro dams ship in 1.0. Three parts: (a) how W&R actually models this, mined from the
installed game; (b) the retrofit blast radius over our systems, from the source in this
repo; (c) 2–3 candidate models with effort ballparks. Feeds decision ticket #96.

Sources: the W&R install at `~/.local/share/Steam/steamapps/common/SovietRepublic/media_soviet/`
(read in place — buildings_types inis, terrain folders, dlc1–4), and this repo's `src/sim`
and `src/game` at commit `9ee6cf0`.

---

## A. How Workers & Resources models terrain, water, pumping and hydro

### Terrain: a single R32F heightmap per map, GPU-quadtree rendered

- Maps are **2048×2048 R32F DDS heightmaps** — the format is mandated by the game's own
  docs (`terrain_heightmaps/README.txt` and the file literally named
  `Only Format DDS 2048x2048 - R32F.txt`: "note about required size (2048x2048) and
  format R32F DDS"). Template heightmaps ship in `terrain_heightmaps/` (fjord, flatland
  with sea, mountains…).
- A map folder (e.g. `terrain3/`) pairs `heightmap.dds` (16 MB = 2048²×4 bytes, i.e. raw
  R32F) with `script.ini` parameters:
  - `$WORLDSIZE 20000 20000` — 20 km × 20 km, so **~9.77 m per heightmap texel**.
  - `$HEIGHTSCALE 800` — heights are normalized floats scaled to 800 m of relief.
  - `$WORLDPOSITION 0 -230 0` — the terrain block is sunk 230 m, which places **sea
    level at world y = 0**; any texel whose scaled height falls below the offset is
    under water. Water is *where terrain is below a global constant plane*.
  - `$QUADTREESTARTSIZE 2048` / `$QUADTREEENDSIZE 32` — quadtree LOD; `quadtree.dta`
    and `collision.dta` are baked companions (collision is precomputed, not sampled
    live from the texture).
  - `resourcemap.dds` / `resourcemap2.dds` — per-texel resource layers (ores, and
    groundwater quality for wells) painted over the same grid.
- **DLC "biomes" are texture tilesets, not mechanics**: `dlc2/` holds `tiles_asia/`,
  `tiles_middleeast/`, `tiles_siberia/` — diffuse/mask tile swaps over the same
  heightmap pipeline. No DLC changes the terrain or water model.

### Water rendering: a global level plane with a shader kit

`waterriver/` contains the entire water renderer's assets: `foam.dds`, `normalmap.dds`,
`perlinnoise*.dds`, `mirror.dds`, `waterstructure.dds`, plus its own small
`heightmap.dds` for shore-depth falloff. There is **no per-cell water volume anywhere**
— a water body is a rendered plane intersecting the heightfield (`script.ini` even has
`$HEIGHTVISIBLEUNDERWATER 0.015` to tint the drowned terrain). Rivers are authored
terrain carved below the level, not simulated flow.

### Water gameplay: a countable resource on a pipe graph, with scalar "head"

Water is an ordinary resource (`RESOURCE_TRANSPORT_WATER`) produced, piped, stored,
treated and consumed — production/consumption bookkeeping, zero fluid simulation.
The building-type census (`buildings_types/*.ini`, `$TYPE_*` tokens):

| Role | Type token | Example ini, key facts |
|---|---|---|
| Groundwater well | `$TYPE_MINE_WATER` | `water_well_big.ini`: `$PRODUCTION water 215` (per building, "not per worker"), `$CONSUMPTION_PER_SECOND eletric 0.095`, `$STORAGE_EXPORT RESOURCE_TRANSPORT_WATER 500`, 7 workers; quality read from the resourcemap |
| Open-water intake | `$TYPE_MINE_WATER_SURFACE` | `water_to_water.ini`: `$PRODUCTION water 150`, **`$HARBOR_OVER_WATER_FROM -3`** — a siting rule: must stand at a shoreline over ≥3 m of water; output needs treatment |
| Treatment | `$TYPE_WATER_TREATMENT` | `water_treatment_big.ini`: consumes `usagewater 3.0` + `chemicals 0.047` + electric → `$PRODUCTION water 30` |
| Pipe pump | `$TYPE_WATER_PUMP` | `water_pumping_station_big.ini`, `water_reservoir_big.ini` |
| Distribution | `$TYPE_WATER_ENDSTATION`, `$TYPE_WATER_SWITCH` | `water_substation.ini` (delivers to buildings), `water_switch.ini` |
| Sewage mirror | `$TYPE_SEWAGE_PUMP` / `_TREATMENT` / `_DISCHARGE` / `_ENDSTATION` | nine pump variants, two treatment sizes, `sewage_discharge.ini` (dump raw into open water) |
| Tanker fallback | cargo stations | `water_filling_station.ini`, `rail_water_station.ini`, `road/rail_sewage_station.ini` — water and sewage are truckable/trainable resources |

The one nod to physics is **`$WATER_STORAGE_POSITION`** — a single signed metre offset
per building for where its water column sits: `-14.0` on the big well (underground),
`-4.0` on the big pumping station, **`+50.0` on `water_reservoir_big.ini`** (a water
tower: elevated storage = gravity head). Pipes care about elevation only through this
scalar and the pumping stations that reset it. Head is a per-building constant, not a
computed hydraulic state.

Pipes attach via authored sockets: `$CONNECTION_WATERPIPE_INPUT/_OUTPUT` coordinate
pairs at negative Y (buried depth) in every water ini.

### Hydro: **absent from the entire install**

The complete `$TYPE_POWERPLANT` census: `powerplant_coal(.v2)`, `powerplant_gas`,
`powerplant_nuclear_single/double`, `powerplant_solar`, `powerplant_wind1/2`,
`incinerator_powerplant`. A recursive search for dam/hydro building types across base
and all four DLC building folders finds nothing. Power plants are plain producers —
`powerplant_coal.ini`: `$PRODUCTION eletric 70`, `$STORAGE_IMPORT_SPECIAL
RESOURCE_TRANSPORT_GRAVEL 100 coal`, 20 workers.

**Precedent finding: W&R shipped 1.0 and four DLCs without hydro dams.** Our #90
mandate deliberately exceeds the reference game. The useful transfer is the shape of
everything *around* the gap: heightfield + constant water level + water-as-resource is
what "gameplay-grade" looked like to 3Division, and their only concession to hydraulics
is one scalar head per building.

---

## B. Retrofit blast radius over our systems

The headline: **the codebase is already Vec3-native end to end** — road nodes, curves,
building positions, vehicle positions, save columns all carry Y today (flat at 0). The
blast radius concentrates in the renderer/cursor and in *new siting rules*, not in the
sim graph. Per system:

### 1. Ground plane & cursor — `src/game/world.rs`, `src/game/tools.rs` — cost M

Today: one `Plane3d` mesh, `GROUND_HALF = 1024` (world.rs:10,78), and the cursor ray is
`ray.intersect_plane(Vec3::ZERO, InfinitePlane3d::new(Vec3::Y))` (tools.rs:110).
Change: a chunked heightfield mesh sampled from an `f32` grid (256×256 cells at 8 m
matches our 2 km field; W&R's own texel density is ~10 m), and the cursor ray marched
against it (coarse step + refine is plenty at our scale). This is the only genuinely
*new* rendering work in the retrofit. **~2–4 sessions**, plus a `Terrain` resource
(`height(x,z)` bilinear sampler) that every other system calls.

### 2. Spline roads / lane graph — `src/sim/roads.rs` — cost S–M

`RoadNode.pos` is `Vec3` (roads.rs:56); `RoadSegment::recompile` is a Hermite sampled
into `curve: Vec<Vec3>` (roads.rs:94,104) and already works in 3D. Change: snap node Y
to `Terrain::height` on placement, drape the sampled curve points, and add a **grade
gate** (reject placement above ~10% slope per class) plus a cut/fill surcharge on the
existing gravel bill (`PAVED_GRAVEL_PER_METRE`, roads.rs:21). Vehicles inherit Y for
free — they position via `segment.point_at` (vehicles.rs:391). Bridges/embankments are
a separate later feature (+M) and not required for 1.0 terrain. **~1–2 sessions.**

### 3. PathService — `src/sim/pathfinding.rs` — cost S

`GraphSnapshot.positions` is `Vec<Vec3>` (pathfinding.rs:56); A* edge lengths come from
segment geometry, so 3D lengths flow through automatically once curves drape. Optional:
a slope multiplier folded into the congestion multiplier already on `SnapEdge`
(pathfinding.rs:47). **Near-zero for correctness; ~half a session for slope cost.**

### 4. Construction earthworks — `src/sim/construction.rs` — cost S–M

`SitePhase::Earthworks` bills gravel and work by pad *area* (construction.rs:94–101),
with the comment already noting W&R derives cost from geometry. Change: bill by
**cut/fill volume** under the footprint (sum of |h − pad_level| over footprint cells),
and on earthworks completion write the flattened pad back into the heightmap — the
first terrain *edit*, reusing the phase machinery unchanged. **~1 session for the
volume bill; +1 if pad-flattening visibly edits the mesh (chunk re-mesh hook).**

### 5. Building siting — `src/sim/buildings.rs` — cost S–M

`footprint()` is a `Vec2` pad on flat ground (buildings.rs:50). Change: a slope check
(max Δh across the footprint above a threshold → refusal, same UX as the NO FUNDS /
gravel-shortfall refusals we already have) and a **water-adjacency rule** for the
surface pump and dam (our analogue of W&R's `$HARBOR_OVER_WATER_FROM`). Dam siting
additionally validates a valley profile (see part C). **~1–2 sessions.**

### 6. Save format v6 — `src/sim/save.rs` — cost S–M

Positions already serialize as 3-float arrays (save.rs:429,525,553,614) and road
geometry is recompiled, never stored (save.rs:12). New in v6: a terrain section —
**authored-map id + RLE delta of terrain edits** (pad flattenings), not the raw grid —
plus water state (per-basin level/volume floats, a handful) and the dam building
discriminant. The save-hash discipline (hash the columns, save.rs:6) survives because
all water state is a few deterministic floats. **~1–2 sessions including migration
tests.**

### 7. Pool solver / power — `src/sim/network.rs`, `src/sim/wires.rs` — cost ~0

This is the good news the ticket asked to confirm: `solve_power` pools **any entity
with a `PowerOutput` component** (wires.rs:212,235–239) through union-find + priority
allocation (`network.rs::allocate`). A hydro dam is a building whose `PowerOutput.0`
is recomputed each tick from reservoir state — the solver, priority classes, and
brownout behavior need **zero changes**. Same pattern the coal plant already uses
(starved plant ⇒ zero output ⇒ same-tick blackout, wires.rs:203–209).

### 8. Water utility loop — `src/sim/water.rs` — cost 0–S

The B8.2 pump⇄sewage cycle is a binary gate over `NetKind::Water` spans on the shared
pool solver (water.rs:63–105). It stays as-is; the new water-bodies layer sits *under*
it: the surface pump variant requires body adjacency to place, and later a dry basin
can zero `PUMP_SUPPLY` — a one-line coupling, not a rewrite.

### Untouched

`commute`, `dispatch`, `transit`, `labour`, `households`, `needs`, `plan`, `customs`,
`traffic`, `zoning` — all operate on entity positions and the road graph, both of
which inherit Y transparently.

**Blast-radius total: roughly 6–10 sessions for heightmap terrain end-to-end,**
dominated by the ground mesh/cursor and the polish of refusal UX — not by sim surgery.

---

## C. Candidate water/terrain models

### Model 1 — Heightfield + static water levels (the literal W&R model)

An `f32` grid (512² over our 2 km field ⇒ 4 m/texel — finer than W&R's 10 m), one
global water level (or a fixed level per authored basin); water body = cells below
level, rendered as level planes. A dam is a building with a valley-profile siting
check; its "reservoir" is a flood-fill of the upstream basin to crest height computed
once at placement; hydro output = `k × head` with flow assumed constant.
- **Effort: ~5–8 sessions** (terrain retrofit from part B + ~2–3 for water/dam).
- **Enables:** terrain that reads, surface-pump siting, dams that produce through the
  pool solver from day one.
- **Fails:** the reservoir never *fills* — placement instantly conjures the lake, the
  level never moves, drought/flood can't exist. It is exactly the model W&R chose when
  it decided *not* to ship hydro; taking it and adding a dam gives a dam without the
  drama. Risks the B8 "unfelt depth" verdict in reverse: a marquee feature that is
  visibly hollow.

### Model 2 — Reservoir graph with flow accounting (recommended)

Terrain as Model 1, plus a **coarse hydrology graph derived from the heightmap at map
load** — this is the "gameplay-grade, not CFD" middle:

- **Basins as nodes.** Flood-fill from local minima gives each basin a precomputed
  *stage–volume curve* (histogram of cell heights — `level(volume)` becomes a table
  lookup). A basin's dynamic state is **one float: volume**.
- **Flow as edges.** D8 steepest-descent routing over the heightmap yields catchment
  areas and spill edges between basins; each basin receives `inflow = rain_rate ×
  catchment_area` (seasonal constant from the existing clock, not weather sim).
- **A dam** is a building placed across a spill edge (siting check: valley profile —
  both abutments above crest within footprint width). It converts the upstream basin
  into a reservoir. Per `SimTick`:
  `volume += inflow − turbine_flow − spillway_overflow; level = stage(volume);`
  `PowerOutput = η · turbine_flow · head(level − tailwater)`, clamped by installed
  capacity — and that `PowerOutput` feeds `solve_power` **untouched** (part B.7).
- **Render:** per-basin level plane at `stage(volume)`; wet cells are those below
  their basin's current level. Rising water after dam completion is just the plane
  climbing — the reservoir visibly fills over game-days.
- **Cost per tick:** O(number of basins) — single digits on our map. Deterministic,
  save state is a few floats per basin (save-hash safe).
- **Effort: ~8–12 sessions** (terrain 3–5, hydrology graph derivation 3–4, dam +
  turbine/spillway + siting 2–3).
- **Enables:** the dam-filling arc (a plan-ladder event worthy of the First Plan),
  droughts and floods as quota-period events, surface pumps that can draw a small
  basin down, sewage discharge inheriting a downstream direction for later pollution —
  all without touching a single cell-level fluid equation.

### Model 3 — Hybrid cell-volume diffusion (shallow-water-lite) — not recommended

Per-cell water depth with neighbor relaxation (pipe-model cellular automaton) on a
coarse grid. **Effort: 15+ sessions** once stability tuning, per-tick cost over
~10⁵–10⁶ cells, and save bloat (a full dynamic grid in the columns, hostile to the
save-hash discipline) are counted. It buys dynamic flood-spread visuals — which #90
explicitly rules out of scope ("NOT CFD") — and nothing the plan economy can price
that Model 2 doesn't already provide. Only worth revisiting post-1.0 if flooding
becomes a designed disaster system.

### Recommendation

**Model 2, staged so Model 1 is its first milestone.** Land the heightfield + cursor +
grade/slope rules + static level first (playable, shippable checkpoint ≈ W&R parity);
derive the basin graph second; land the dam third. Save v6 carries: authored-map id,
RLE terrain-edit deltas, per-basin volumes, dam discriminant. The pool solver, the
water utility cycle, PathService and the whole logistics stack are confirmed
unaffected beyond the couplings named in part B.
