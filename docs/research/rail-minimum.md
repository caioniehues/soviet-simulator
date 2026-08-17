# Research: W&R rail — minimum viable shape

**Ticket:** #82 (part of map #81, feeds the "Rail" scope ticket)
**Sources:** the actual Workers & Resources: Soviet Republic install —
`~/.local/share/Steam/steamapps/common/SovietRepublic/media_soviet/buildings_types/`
(1472 ini files) and `media_soviet/trains/` (140 entries, per-vehicle `script.ini`) —
plus this repo's `spec/roads.md`, `spec/pathfinding.md`, `spec/logistics.md` and the
shipped systems inventoried in `docs/wayfinder-brief.md` §2.
**Method:** direct grep/read of the ini corpus; every W&R claim below is CONFIRMED
from a named file unless marked INFERRED.

---

## 1. What rail *is* in W&R's data

### 1.1 Vehicle taxonomy (`media_soviet/trains/*/script.ini`)

Exactly **two rail vehicle types** exist:

| `$TYPE` | count | role |
|---|---|---|
| `VEHICLETYPE_RAIL_LOCOMOTIVE` | 76 | anything self-propelled (locos, EMU/DMU head cars, railcars, track-builders, rail cranes) |
| `VEHICLETYPE_RAIL_VAGON` | 60 | unpowered wagons |

`$TRAINGROUP_*` subdivides the UI catalogue: `LOCOMOTIVE` 28, `VAGON` 33,
`TRAINSET` 7, `MOTORVAGON` 5, `METRO` 18, `TRAM` 18, `TRACKBUILDER` 11
(the rail-construction-office fleet — track is laid *from rail vehicles*).

**Locomotive stat block** (e.g. `russian_loco_753/script.ini`, diesel;
`russian_loco_vl23/script.ini`, electric):

- `$MOVEMENT_SPEED` (40–120 km/h across the fleet; mode 80)
- `$MOVEMENT_POWER_KW` (up to 5100) + `$MOVEMENT_EMPTY_WEIGHT` (tonnes) —
  i.e. **acceleration is power-to-weight over the consist**; a weak loco pulling
  many loaded wagons is slow. This pair is the entire rail "physics" surface in data.
- `$COST_RUB`, `$AVAILABLE <year> <year>`, `$COUNTRY` (era/tech-tree gating)
- Diesel vs electric is *not* a declared token: no `$DIESEL`/`$ELECTRIC` exists
  anywhere in the corpus (CONFIRMED absence — the only fuel-ish token is
  `$MOVEMENT_CONSPUMPTION`). The distinction is carried by sound/particle sets
  (`train_diesel2` vs `train_electro`, `$PARTICLE_MOVEMENT train_eletric`) and
  enforced natively: electric locos need wired track fed by a rail trafo (§1.3).

**Wagon stat block** (`russian_vagon_open/script.ini`):

- `$RESOURCE_CAPACITY` (open 75 t; passenger car 105; fleet range ~50–175)
- `$RESOURCE_TRANSPORT_TYPE` — **one transport class per wagon**, same enum as
  trucks/storages: across the rail fleet OPEN 6, COVERED 4, GRAVEL 3, CEMENT 3,
  COOLER 3, OIL 4, LIVESTOCK 1, WATER 1, SEWAGE 1, PASSANGER 64 (incl. EMU cars).
- `$MOVEMENT_EMPTY_WEIGHT` (18–25 t) — feeds the consist weight above.

So a *train* is data-wise just **loco(s) + wagons, each an ordinary classed cargo
vehicle**; the compatibility gate is the exact one our dispatcher already enforces
for trucks (class = bucket = vehicle, `spec/logistics.md` §"confirmed compatibility
gate").

### 1.2 Building taxonomy (`buildings_types/*.ini`)

The complete rail-related `$TYPE_*` set, measured:

| `$TYPE_*` | files | examples | what it declares |
|---|---|---|---|
| `TYPE_RAILDEPO` | 2 | `rail_depo.ini`, `raildepobig.ini` | home of the fleet: 6 `$CONNECTION_RAIL` dead-end tracks into the shed; no cargo storage |
| `TYPE_CARGO_STATION` | 9 rail (43 total incl. road/harbor) | `rail_station_cargo*.ini`, `bulk_rail_station.ini`, `rail_station_rumble_(un)loading*.ini`, `rail_pumping/water/sewage_station.ini` | loading interface: `$STORAGE <class> 1` per accepted class (a **pass-through token bucket, not a warehouse**) + `$CONNECTION_CONNECTION` factory-links to adjacent storages/factories + through tracks |
| `TYPE_PASSANGER_STATION` | 5 | `rail_station_passanger.ini` | `$STORAGE RESOURCE_TRANSPORT_PASSANGER 1200` waiting hall + through tracks |
| `TYPE_WAITING_STATION` | 1 | `rail_endstation.ini` | line-end turnaround; `$STORAGE_FUEL RESOURCE_TRANSPORT_OIL 75` (refuel while waiting) |
| `TYPE_CONSTRUCTION_OFFICE_RAIL` | 2 | `rail_construction_office.ini` (+`_big`) | builds/renews track: `$WORKING_VEHICLES_NEEDED 4/8` (TRACKBUILDER group), `$WORKERS_NEEDED 60/170`, material stores (gravel 150/350, OPEN 250/500, ecomponents 35/70), 6 rail stubs |
| `TYPE_DISTRIBUTION_OFFICE_RAIL` | 1 | `distribution_office_rail.ini` | the rail *dispatcher*: `$WORKING_VEHICLES_NEEDED 8` trains, `$STORAGE RESOURCE_TRANSPORT_VEHICLES 150`, fuel 50, ten `$CONNECTION_RAIL_DEAD` siding stubs + 4 through tracks. **No policy/route tokens** — identical finding to the road office (`spec/logistics.md`): the wiring lives in the savegame, the solver is native. |
| `TYPE_GAS_STATION` (rail variant) | 1 | `rail_gas_station.ini` | diesel refuelling on a through track |
| `TYPE_RAIL_TRAFO` | 1 | `rail_trafo.ini` | electrification feed: `$CONNECTION_ELETRIC_LOW_INPUT` from the LV grid + 4 `$CONNECTION_RAIL_ALLOWPASS` — powers overhead wire so electric locos can run |
| `TYPE_CUSTOMHOUSE` | 2 with rail | `zoll_milhost.ini` | border crossing carries `$CONNECTION_RAIL_BORDER` tracks — rail import/export is physical, same shape as our G1 road customs |
| `TYPE_PRODUCTION_LINE` | 2 | `production_train.ini` | wagon/loco *factory* (late-game vehicle manufacture; not rail-core) |

### 1.3 The `$CONNECTION_*` network grammar (rail subset, counted corpus-wide)

| token | count | meaning |
|---|---|---|
| `$CONNECTION_RAIL` | 40 | dead-end track stub into a building (depot shed, CO) — trains terminate |
| `$CONNECTION_RAIL_ALLOWPASS` | 108 | **through track** — the building sits *on* the line; trains may pass without stopping (all stations, trafo, distribution office) |
| `$CONNECTION_RAIL_ALLOWPASS_INPUT/_OUTPUT` | 20+20 | directional through track (metro + aboveground stations — enforced one-way flow) |
| `$CONNECTION_RAIL_DEAD` | 10 | unbuilt siding stub (player may extend; distribution office, harbor) |
| `$CONNECTION_RAIL_BORDER` | 4 | track that exits the map at customs |
| `$CONNECTION_RAIL_HEIGHT` / `_DEADEND` | 8 / 1 | elevated variants / capped stub |
| `$RAIL_SECONDARY_NOT_RENDER_ELETRIC` | — | visual flag (depot/CO second track shown unwired) |

Two structural facts with direct engine consequences:

1. **Stations are *on-line*, not *off-line***: cargo/passenger stations use
   ALLOWPASS through tracks; only depots/COs use dead-end `$CONNECTION_RAIL`.
   A rail line is a corridor threaded *through* station buildings.
2. **Signals are not buildings.** Zero signal/semaphore entries exist in
   `buildings_types/` (CONFIRMED absence); semaphore models are loose track-side
   assets (`media_soviet/semaphore*.nmf`). Signal placement and block logic are
   entirely native to the track tool — like pathfinding, it is **ours to design
   from scratch** (`spec/pathfinding.md` "W&R contrast" all over again).

### 1.4 What the data does *not* contain (all CONFIRMED absences)

- No track-class inis: no speed/gauge/capacity tokens for track anywhere
  (matches the road finding, `spec/roads.md` §"Road class"). Track speed limits,
  curve limits, electrification state, signal blocks — all native runtime state.
- No consist definitions: trains are assembled at the depot UI at runtime; data
  only supplies the parts (loco/wagon inis).
- No schedules/lines/routes: like the road distribution office, the rail office
  ini carries fleet + fuel + geometry only. Lines/timetables are savegame state.
- No signal or block tokens of any kind.

---

## 2. (a) Smallest coherent rail stage

The measured minimum W&R itself ships is larger than *our* minimum needs to be,
because W&R 1.0 carries electrification, fuel, passengers and metro at once. The
smallest stage that is still *rail* (and not a decorated road) is:

**Buildings (3):** rail depot (`TYPE_RAILDEPO` shape: dead-end tracks, homes the
fleet), rail cargo station (`TYPE_CARGO_STATION` shape: through track + per-class
pass-through bucket + factory-links to adjacent storage), and a rail connection at
the existing customs office (`$CONNECTION_RAIL_BORDER` shape) so trains join the
Plan economy (exports/imports) on day one.

**Vehicles (2 kinds):** one diesel locomotive (`speed`, `power_kw`,
`empty_weight`, `cost_rub`) + one open wagon (`RESOURCE_TRANSPORT_OPEN`,
capacity ~75). One consist = 1 loco + N wagons, N fixed or player-chosen at the
depot. Diesel first — electrification is exactly the mechanic W&R gates behind
an extra building (`rail_trafo`) and we should too (§3).

**Mechanics it minimally needs:**

1. **A rail edge class in the lane graph.** Track = a new lane type with
   `rail`-only vehicle mask, no lane changing, junction = switch. Our unified
   lane-graph + incremental recompile (`spec/roads.md`, shipped in M2)
   carries this directly — W&R data confirms track carries *no* parameters of
   its own, so a rail segment is *simpler* than a road segment (one lane,
   speed from the vehicle, `spec/roads.md` §"W&R's discrete enum" applies
   verbatim to rail).
2. **PathService reuse, one new mask.** Async A* takes a `Rail` vehicle-class
   mask; cost stays `length/speed`. No congestion term needed at this stage —
   see occupancy below.
3. **Consist movement.** The one genuinely new mover: a train is a chain of
   bodies following one path with offsets (wagons trail the loco's lane-hop
   chain by arclength). Speed = min(vehicle top speed, power/weight curve).
   Our car-following model does *not* apply (no gaps negotiated mid-block);
   trains only interact at block boundaries.
4. **Exclusive-block occupancy instead of signals.** Minimum-viable safety:
   each track edge (or edge-run between switches) is a mutex; a train reserves
   the run ahead, others wait at the switch. This is degenerate signalling
   (every switch-to-switch run is one block) and needs no player-facing signal
   tool. W&R's native signals reduce to exactly this when none are placed
   (observed W&R behaviour: unsignalled track is one block per line section —
   INFERRED from play, consistent with data absence).
5. **Dispatcher reuse, verbatim.** The band-driven dispatcher already matches
   deficit buckets to fleets under the class gate. The rail cargo station is a
   *dock* with huge batch size: a station's pass-through bucket (`$STORAGE
   <class> 1` in W&R) plus factory-links means "whatever the adjacent storage
   holds, trains can lift". Dispatch rule: a consist is one dispatch with
   capacity = Σ wagon capacities (~10× a truck), served from the rail depot's
   finite fleet exactly as trucks are served from the road depot. Dock rates
   (`$VEHICLE_LOADING_FACTOR` equivalent — we already model dock rates) make
   station dwell real.
6. **Station-adjacency transfer.** Rail→road transfer happens because the cargo
   station factory-links to a storage, and the road dispatcher already serves
   that storage. No new transfer mechanic: the station is just a storage door
   that trains can dock at. This is exactly W&R's `$CONNECTION_CONNECTION`
   pattern (`rail_station_cargo.ini`).

**What makes this coherent as gameplay:** one bulk corridor — e.g. border
customs → cargo station beside the coal/gravel storages — moving tonnage the
road fleet can't. It exercises: build track (construction already phased),
buy loco+wagons through the Treasury/customs pipeline (G1.2 shipped), set
bands, watch trains relieve a saturated road. That is the W&R rail pitch in
miniature, and every supporting system already exists.

**Explicitly cut at this stage** (each is a building W&R gates it behind, so
cutting is precedent-faithful): electrification (rail trafo), passenger
stations, fuel consumption (rail gas station), player-placed signals,
rail construction office (ordinary construction system builds track;
the CO is flavour we can defer), waiting/end stations, rail distribution
office as a separate building (the depot doubles as dispatcher home).

## 3. (b) Medium stage

Add, in rough dependency order:

1. **Passenger rail:** `TYPE_PASSANGER_STATION` (waiting-hall bucket 1200,
   through tracks) + passenger wagon (`RESOURCE_TRANSPORT_PASSANGER 105`) +
   `TYPE_WAITING_STATION` turnaround at line ends. Reuses the transit-line
   machinery (bus lines, stops, walk→queue→ride — shipped in the Transit
   milestone) with train-sized batches; extends labour catchment map-wide.
2. **Player-placed signals + real blocks:** promote the degenerate
   block-per-run to signal-delimited blocks; path reservation through
   junctions (deadlock avoidance: reserve through the switch cluster, not one
   edge). This is the native layer W&R never exposes in data — ours to design;
   the traffic StallBoard pattern (stall→reroute→report) generalises to
   "train waiting at signal" readouts.
3. **Electrification:** wire as a per-edge flag built by construction;
   `TYPE_RAIL_TRAFO` feeds a wired section from the LV grid (one
   `$CONNECTION_ELETRIC_LOW_INPUT`, our union-find utility pool solver already
   handles the membership); electric locos (cheaper to run, stronger) refuse
   unwired edges via the path mask — a second, richer use of the exact
   mask mechanism from stage (a). Unpowered wire = parked electrics: a real
   brownout consequence through the existing priority classes.
4. **Diesel fuel as a consumable** + rail gas station / `$STORAGE_FUEL` at end
   stations — but only once fuel exists as a resource at all (B10 already owns
   fuel; rail should not drag it in early).
5. **Rail construction office:** track built/renewed by TRACKBUILDER consists
   from a rail CO (gravel + rails-as-OPEN + components), making remote track
   extension a rail-served operation — W&R's distinctive "the railway builds
   the railway" loop.
6. **More wagon classes** tracking the resource tree as it grows (GRAVEL,
   COVERED, OIL/tank...) — zero new mechanics, pure data, the class gate does
   the rest.
7. **Rail distribution office** as a distinct building once fleet count makes
   the depot-as-dispatcher feel cramped (W&R separates them; 8 trains, ten
   siding stubs).

## 4. (c) What W&R rail depends on that we lack entirely

1. **Consist physics and rendering** — multi-body vehicles: chained bodies on
   one path, power/weight acceleration, per-wagon cargo visualisation
   (`$RESOURCE_VISUALIZATION` blocks per wagon). Nothing in our vehicle layer
   is multi-body today.
2. **Block/signal reservation** — trains cannot use the car-following +
   congestion-pricing model at all; safety is reservation-based. Entirely
   native in W&R (zero data tokens), entirely absent in ours; the one new
   *algorithmic* system rail forces.
3. **Depot-based consist assembly** — a UI/state layer where locos and wagons
   (each a separately purchased vehicle, `$COST_RUB`, arriving via customs like
   any G1.2 purchase) are composed into a named train. No analogue exists;
   trucks are atomic.
4. **Through-station docking** — our docks are terminal (vehicle arrives,
   loads, leaves the way it came). ALLOWPASS through-tracks mean a station
   is *on* the line: dwell-on-line, then continue forward. Touches lane-graph
   building integration, not just dispatch.
5. **The wider resource tree** (45 vs our 3) — not a rail mechanic, but the
   reason rail exists in W&R (steel/ore/oil tonnage). With 3 resources, rail's
   minimum stage must justify itself on bulk border trade + coal/gravel; fine
   for stage (a), but rail's *pull* scales with B10's resource expansion.
6. **Era/availability gating** (`$AVAILABLE 1956 1967`, `$COUNTRY`) — every
   train is year-gated; we have no calendar-driven catalogue (B10 owns the era
   calendar).
7. **Electrified-track state + rail trafo feed** — per-edge wire state fed by a
   grid building; our lane graph has no per-edge utility membership today
   (the pool solver is building-level).

---

## 5. Reuse verdict (summary table)

| Existing system | Rail reuse |
|---|---|
| Unified lane graph + incremental recompile | direct — rail is a new lane class with a vehicle mask; track is *parameter-free* per W&R data |
| Async A* PathService | direct — one new mask; later a "wired-only" mask for electrics |
| Band-driven dispatcher + class gate + finite fleets + dock rates | direct — consist = one big dispatch; W&R's rail office confirms no new policy vocabulary exists |
| Customs / Treasury / vehicle purchase (G1) | direct — locos/wagons are `$COST_RUB` purchases; `$CONNECTION_RAIL_BORDER` mirrors our road border |
| Construction (phased sites, material bills) | direct for track-as-built-object; rail CO variant deferred to medium |
| Transit lines (bus machinery) | medium stage — passenger rail rides on it |
| Utility pool solver (union-find) | medium stage — rail trafo feeds wired sections |
| Car-following / congestion pricing | **not reusable** — replaced by block reservation (the one new core system) |

## 6. Source index

- Depot: `buildings_types/rail_depo.ini`, `raildepobig.ini`
- Cargo stations: `rail_station_cargo{,1,2,3}.ini`, `bulk_rail_station.ini`, `rail_station_rumble_{loading,unloading}{,_v2}.ini`, `rail_{pumping,water,sewage}_station.ini`
- Passenger: `rail_station_passanger{,_small}.ini`, `railstation_{groundlevel,aboveground}{,_big}.ini`, `rail_endstation.ini`
- Offices: `rail_construction_office{,_big}.ini`, `distribution_office_rail.ini`
- Support: `rail_trafo.ini`, `rail_gas_station.ini`, `zoll_milhost.ini`/`zoll_siatre.ini` (rail border), `production_train{,2}.ini`
- Vehicles: `media_soviet/trains/*/script.ini` (76 locos, 60 wagons; e.g. `russian_loco_753`, `russian_loco_vl23`, `russian_vagon_open`, `russian_vagon_passanger`)
- Repo: `spec/roads.md`, `spec/pathfinding.md`, `spec/logistics.md`, `docs/wayfinder-brief.md`
