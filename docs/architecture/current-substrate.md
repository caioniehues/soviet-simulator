# Current substrate

**Kind:** current-state
**Authority:** observational — true for the cited paths at the verified commit; not a target, not a completion tracker
**Status:** active
**Owner:** architecture
**Verified-at:** `266f7b2`
**Last verified:** 2026-09-04

Every statement here was source-inspected on the verified commit, by the ten mining lanes on
2026-08-28 and re-checked by the lead. Symbols are named; line numbers appear only where they
were read this session. When this page and the code disagree, the code wins and this page is
stale — update it ([documentation standard](../engineering/documentation.md)). The older
[substrate map](../reference/architecture/substrate.md) and the
[fact-sheets](../research/fact-sheets/wave1-substrate.md) hold the anchored evidence rows.

## Workspace

Thirteen crates (`Cargo.toml` members): `simulation` (the sim), `native_app` (the game binary,
default member), `engine` (wgpu renderer, terrain, PBR, LOD), `engine_demo`, `geom`, `common`,
`prototypes` (Lua-driven data), `headless` (lockstep server runner), `networking` (lockstep
multiplayer), `goryak`, `egui-inspect`, `egui-inspect-derive`, `assets_gui`. Data lives in
`base_mod/*.lua` (`items`, `companies`, `rollingstock`, `roadvehicles`, `leisure`, `colors`, `data`).

Two git dependencies are **pinned by `rev`** (sov-buw): `egui` at `d4e8966a`
(was: tracks upstream HEAD) and `yakui` at `6c6982ff` (was: fork `dev`
branch). Thirteen lockfile packages come from these two sources. `deny.toml`
allows exactly these two git sources; see the cadence rule in
`docs/process/dependency-policy.md`.

## Tick and schedule

- **Files:** `simulation/src/lib.rs` (`Simulation`, `tick`, `SimulationSer`, `hashes`), `simulation/src/init.rs` (`init`, registry), `simulation/src/utils/scheduler.rs` (`SeqSchedule`), `simulation/src/world_command.rs` (`WorldCommand`, `apply`).
- **Provides:** 50 ticks/second nominal (`prototypes/src/types/time.rs`). `Simulation::tick` applies `WorldCommand`s first, increments time, then runs a serial schedule. `ParCommandBuffer::apply` runs after every system, so later systems see earlier structural changes.
- **Order:** 18 systems in `init.rs`, executed in registration order: `electricity_flow_system` → `dispatch_system` → `update_decision_system` → `company_system` → `pedestrian_decision_system` → `transport_grid_synchronize` → `locomotive_system` → `vehicle_decision_system` → `vehicle_state_update_system` → `routing_changed_system` → `routing_update_system` → `itinerary_update` → `market_update` → `train_reservations_update` → `freight_station` → `random_vehicles` → `update_map` → `add_souls_to_empty_buildings` (via `register_system_sim`, takes `&mut Simulation`). Electricity runs *first*; map update is second-last.
- **Does not provide:** named phases, phase barriers, cadence bands (every system every tick), an event calendar, any parallel execution (`rayon` is used once, in `map/terrain.rs`).
- **Conflicts with target:** [simulation phases](simulation-phases.md), [time and events](time-and-events.md). Four instant commands (`MapBuildHouse`, `MapUpdateIntersectionPolicy`, `UpdateZone`, `SetGameTime` — `simulation/src/world_command.rs:213-220`) bypass ticking via the native fast path, which applies all-instant queues directly with no schedule pass (`native_app/src/network.rs:51-57`); other commands force one tick while paused (`native_app/src/network.rs:67-70`). Full command-seam contract: [simulation phases](simulation-phases.md) (fact-sheet: Foundation / Tick).
- **Initialization:** `static REGISTRY: OnceLock<Registry>` (`init.rs`) since 2026-08-26; the fact-sheets' "unsynchronised `static mut`" rows are stale. The same `static mut` shape still exists in `native_app/src/init.rs` (UI crate, not in the test binary).

## Entities and identity

- **Files:** `simulation/src/world.rs` (`World`, `new_key_type!`, `HumanEnt`, `VehicleEnt`, `TrainEnt`, `WagonEnt`, `FreightStationEnt`, `CompanyEnt`), `simulation/src/utils/par_command_buffer.rs`.
- **Provides:** a hand-rolled typed store — one `HopSlotMap<ID, Ent>` per entity type from `slotmapd` (Uriopass's fork of `slotmap` for serialisation-cycle determinism). Typed keys `VehicleID`, `TrainID`, `HumanID`, `WagonID`, `FreightStationID`, `CompanyID`. Per-type `ParCommandBuffer` for deferred kill/exec; `exec_ent` takes `FnOnce(&mut Simulation)`.
- **`HumanEnt` fields:** `Transform`, `Speed`, `Location`, `Pedestrian`, `Option<Transporter>`, `Router`, `Itinerary`, `HumanDecision`, `Home`, `BuyFood`, `Bought`, `Option<Work>`, `Box<PersonalInfo>`. `PersonalInfo` = `{name: String, age: u8, gender}`. Age never increments.
- **Does not provide:** an ECS with component queries; a record/body split; dense append-only citizen IDs; households (no `Household` identifier anywhere in `simulation/`); lifecycle (birth, death, education, migration); any need beyond bread.
- **Conflicts with target:** [entity identity](entity-identity.md), [state storage](state-storage.md). The `exec_ent` closure channel is the main cross-system mutation path and blocks typed contexts ([authority boundaries](authority-boundaries.md)).

## Economy and the dishonest enterprise

- **Files:** `simulation/src/economy/market.rs` (`Market`, `SingleMarket`, `Trade`, `Dispatch`, `DispatchState`, `RetailClaim`, `make_trades`, `advance_dispatches`, `calculate_prices`, `set_requested`, `requested`), `economy/mod.rs` (`market_update`), `economy/government.rs` (`Government { money }`), `economy/ecostats.rs` (`EcoStats`), `souls/goods_company.rs` (`recipe_init`, `recipe_should_produce`, `recipe_act`, `company_system`, `GoodsCompanyState`), `prototypes/src/types/recipe.rs` (`Recipe { request_multiplier, storage_multiplier, … }`).
- **Provides:** `SingleMarket` holds `capital` (on hand), `reserved`, `requested` per soul. Domestic matching is price-free (`money_delta = 0`) and sorts by distance. **The dishonest enterprise is wired end-to-end:** `recipe_init` computes `qty = amount × request_multiplier` and calls `market.set_requested` then `buy_until`; `flour-factory` uses 4, `slaughterhouse` 3 (`base_mod/companies.lua`); proven by `scenario_0151_inflated_request_hoards_honest_does_not` and `sov_lpj_*`. **Hoarding has a physical floor:** `recipe_should_produce` refuses to buy when `capital − reserved ≥ amount × (storage_multiplier + 1)`. No `dishonest` identifier exists in non-test code.
- **Does not provide:** any Planner observation of `requested` vs consumed (`Market::requested()` is public and unread by `native_app/`); adaptive inflation (the multiplier is a static prototype constant); quotas, plan periods, priority classes, reserve classes, credibility, storming, a material balance (`EcoStats` tracks trade volumes only); partial multi-seller fill or request age.
- **Contradicts the pillars:** `Government.money` is debited for buildings, roads and trains (`world_command.rs`, `sim.write::<Government>().money -= cost`) and for workers per minute (`economy/mod.rs`, `WORKER_CONSUMPTION_PER_MINUTE`); it can go negative. **Export side teleports:** the ext-trade block of `make_trades` debits seller capital at match time and creates no `Dispatch` (the import side is physical since `sov-abs`). The fact-sheet row ECO-SUB-005 ("`set_requested` has no non-test caller") is stale since commit `0caee71`.
- **Spec:** production, resources, trade, logistics. **Tests:** `tests/scenarios/{hoarding,inflation,recipe_provided,validation}.rs`.

## Logistics

- **Files:** `economy/market.rs` (`Dispatch`, `DispatchState::{ToSource, Loading, ToDestination, Unloading}`, `advance_dispatches`, `release_tosource_truck`), `map_dynamic/dispatch.rs` (`Dispatcher`, `DispatchOne::query`, `DISPATCH_LANE_CUTOFF`), `souls/freight_station.rs`.
- **Provides:** a real truck leg — a truck is reserved from the `Dispatcher`, drives a real itinerary, capital is debited at `Loading` and credited at `Unloading`. Border imports are physical trucks from a freight station. A default city's border is closed until the player lays road to the station (ratified 2026-08-28; `sov_ie6_*`). Recovery for dead buyers, dead sellers, dead trucks, severed roads and TTL expiry is tested.
- **Does not provide:** authoritative cargo or capacity on the vehicle (`LOG-SUB-005`); loading/unloading time (freight-station cargo is a counter); return-to-depot; a single fulfilment authority (company drivers react to `Sold` while the market drives its own trucks — `ECO-SUB-006`); a bounded `ToSource` wait (in progress as `sov-ahw`).
- **Spec:** logistics, vehicles. **Tests:** `tests/scenarios/ledger.rs` (16 functions incl. `sov_abs_ext_trade_import_is_physical`, `scenario_demolish_buyer_building_end_to_end_conserves`, `scenario_dead_truck_tosource_cancels_without_leak`), `retail.rs` (retail claims, TTL, settlement, despawn).

## Citizens and needs

- **Files:** `souls/human.rs` (`spawn_human`, `update_decision_system`, `HumanDecision`), `souls/desire/{home,buyfood,work}.rs`, `souls/mod.rs` (`add_souls_to_empty_buildings`).
- **Provides:** one person spawned per empty house; decisions are a max-score over `Home` (constant 0.2), `Work` (time interval with random offset), `BuyFood` (bread only; a global market buy order matched by distance; the citizen walks to the matched seller; `last_ate` advances only on physical receipt since the retail two-leg model).
- **Does not provide:** households, housing queues, any second need, time budgets, search or queueing time, knowledge limits (the citizen knows the matched seller's location), education, health, lifecycle, migration, social ties.
- **Spec:** citizens, households, needs, education, healthcare — all target; all `EVID-*` unimplemented.

## Vehicles, traffic, rail

- **Files:** `transportation/{vehicle,road,pedestrian,train}.rs`, `map/{pathfinding,traffic_control}.rs`, `map/objects/{lane,road,parking,lot}.rs`, `map_dynamic/{itinerary,router,parking}.rs`, `prototypes/src/prototypes/{road_vehicle,rolling_stock}.rs`, `base_mod/rollingstock.lua`.
- **Provides:** lane-constrained kinematic motion — `speed += clamp(desired − speed, −decel·dt, accel·dt)` with per-`VehicleKind` (Car/Truck/Bus) width, acceleration, deceleration, turning radius; spatial-grid cone-check collision avoidance with a gridlock detector (`VehicleState::Panicking`, random wait); traffic lights (`traffic_control.rs`); exclusive parking reservations (`MAP-SUB-005`); A* pathfinding via the `pathfinding` crate with cost `length / speed_limit + deterministic noise`; trains with consist mass, length, tractive and braking force summed from rolling-stock prototypes (`calculate_locomotive`), intersection reservations (`TrainReservations`), look-ahead braking, and a 60-second stuck-creep.
- **Does not provide:** mass, cargo, capacity, owner, fuel or wear on road vehicles; grade physics; IDM/MOBIL; BPR/Gawron or any load-aware routing cost; spillback; a meso layer; signal blocks (two trains can share a segment); wagon cargo type or capacity; yards; bus passengers or routes. Rolling-stock `max_speed` values are placeholders (a locomotive at 200 m/s).
- **Conflicts with target:** `SPEC-TRAFFIC-007/008`, `SPEC-PATHFINDING`, `SPEC-VEHICLES`. Auto-generated roadside lots on road construction (`map/map.rs`, `MAP-SUB-002`) conflict with `SPEC-ZONING-003`. **Tests:** `tests/vehicles.rs`; `transportation/testing_vehicles.rs`.

## Utilities

- **Files:** `map/electricity_cache.rs` (`ElectricityCache`, `NetworkObjectID`, `BTreeMap` graph over buildings, roads, intersections with BFS reachability — `graph` at `simulation/src/map/electricity_cache.rs:62`, `path_exists` at `:179-186`), `map_dynamic/electricity.rs` (`electricity_flow_system`, `ElectricityFlow { produced_power, consumed_power, blackout }`).
- **Provides:** connectivity by road adjacency (a building on a road is on the grid); per-network sums of produced and consumed power (houses 100 W fixed; companies per prototype × productivity); a binary `blackout` when consumed > produced; blackout halts production.
- **Does not provide:** wire topology (`SPEC-ELECTRICITY-001` forbids road-as-wire); storage; priority shedding; brownout; ramp rates; water, sewage, heating, gas, waste, weather, hydrology — no building kind, no system, no data structure for any of them.
- **Spec:** electricity, water, sewage, heating, waste — all target.

## Map and construction

- **Files:** `map/map.rs` (`Map`, `build_house`, `build_special_building`, lot generation), `map/objects/*`, `map/procgen/*`, `map/terrain.rs`, `map/serializing.rs`, `world_command.rs`.
- **Provides:** typed lanes (driving, parking, walking, rail); Planner-authored roads and buildings via `WorldCommand`; heightmap terrain; instant building placement with a rouble cost.
- **Does not provide:** ghosts, verdicts, refusals, material bills, Sites, ground-broken, rescind — construction is instant; lots auto-generate on road construction.
- **Spec:** construction, buildings, zoning, roads — all target.

## Persistence and determinism

- **Files:** `lib.rs` (`SimulationSer { world, version, res }`, `hashes`, `deserialize`), `common/src/saveload.rs` (`Encoder`, `Bincode`, `CompressedBincode`, `JSON`), `utils/replay.rs` (`Replay`, `SimulationReplayLoader`), `tests/test_iso.rs`, `tests/mod.rs` (`TestCtx::check_determinism`).
- **Provides:** bincode saves with `miniz_oxide` compression; a `version` string (`VERSION` = 0.6.1) that **warns** on major mismatch and proceeds; per-resource hashes (`hashes()` over `FxHasher`); a replay of `(Tick, WorldCommand)`; `test_world_survives_serde` replays `world_replay.json` over two runs and fails on divergence (`simulation/src/tests/test_iso.rs:308-309`, armed post-`7fa08e8`/`eed5ead`, World compared since `7e771ce`); a round-trip check (encode → decode → compare hashes) run every tick in `TestCtx::tick` and every 25 ticks in `advance_ticks`.
- **Does not provide:** cross-platform repeat-run determinism (same seed + commands → same digest holds same-machine only — portable digest missing since `FxHasher` is not stable across platforms); a migration path (missing resources get defaults; nothing transforms old payloads); cross-platform float determinism (`geom/` calls `sin`, `cos`, `sqrt`, `atan2` as intrinsics; no `libm`).
- **Randomness:** one global `RandProvider` (Xorshift128, `utils/rand_provider.rs`, seeded `RNG_SEED`) drawn sequentially by `spawn_human` and others; a stateless positional hash in `common/src/rand.rs` (`rand2(x, y)`, `randu`).

## Multiplayer

- **Files:** `networking/src/{lib,authent,catchup,connections,packets,worldsend}.rs`, `client/`, `server/` (the lockstep netcode); `native_app/src/network.rs`; `headless/src/main.rs`; `simulation/src/multiplayer/` (in-sim chat state only — `mod.rs` 9 lines, `chat.rs` 44 lines, `MultiplayerState { chat }`).
- **Provides:** a lockstep client/server in `networking/` — `Frame(u64)`, `PlayerInput`, `MergedInputs`; the server merges inputs per frame and broadcasts; both client and headless `assert_eq!(frame, tick + 1)`; authentication; world-state catch-up. `simulation/src/multiplayer/` provides no transport: it is the chat box drained by the HUD, not a netcode seam.
- **Constraint:** any non-deterministic parallelism breaks it. `WorldCommand` has no role filter. Transport is unauthenticated/unencrypted and reads peer-controlled frame sizes into growable buffers (technical-stack research, 2026-08-24) — trusted-LAN only.

## Presentation

- **Files:** `native_app/src/game_loop.rs` (`sim: Arc<RwLock<Simulation>>`), `gui/`, `gui/inspect/{inspect_building,inspect_human,…}.rs`, `debug_gui/`, `rendering/`, `uiworld.rs`; `engine/src/`.
- **Provides:** tools that read immutable `Simulation` state, mutate UI state and queue `WorldCommand`s; a building inspector (workers, productivity, power, network health, progress, recipe, per-item storage) and human, vehicle, train and freight-station inspectors; a forward wgpu renderer (PBR IBL, cascaded shadows, SSAO, instancing).
- **Does not provide:** a Planner snapshot or any information boundary (~40 call sites read arbitrary resources from `Simulation`); STATUS/CAUSE/TREND/POLICY/PHYSICAL CHAIN; requested-vs-consumed; render culling/LOD adequate to 250k (the renderer draws every human and vehicle).

## Prototypes

- **Files:** `prototypes/src/{load,validation}.rs`, `prototypes/src/prototypes/*.rs`, `prototypes/src/types/*.rs`.
- **Provides:** Lua-declared items (21; `id`, `label`, `optout_exttrade`), goods companies (`kind`, `recipe`, `n_trucks`, `n_workers`, `zone`), rolling stock, road vehicles, leisure, colours; validation refuses recipe numbers below 1 (`sov-k3w`). Lua is runtime authority only where a parsed field has a reachable consumer (substrate map §Prototype authority).
- **Does not provide:** mass, volume, storage or transport class on items; wagon cargo capacity.

## Tests

`cargo test -p simulation` — parallel-safe since 2026-08-26. 42 scenario-test fns across
`tests/scenarios/{hoarding,inflation,ledger,recipe_provided,retail,validation,mod}.rs` (count includes the non-corpus `scenario_harness_smoke`; the `#[test]` mention in `mod.rs` docs is not a test), plus
`test_iso.rs` and `vehicles.rs`. **None of the 107 `evid_*` tests named in the specifications
exists** (`grep -r 'fn evid_' simulation/src` is empty), which matches the generated roadmap's "0
implemented". Mutation testing with cargo-mutants is adopted for eligible changes
([mutation policy](../process/mutation-policy.md)).

## Related

- [Substrate architecture map](../reference/architecture/substrate.md) — the anchored evidence rows
- [Fact-sheets](../research/fact-sheets/wave1-economy.md), [wave1-logistics](../research/fact-sheets/wave1-logistics.md), [wave1-substrate](../research/fact-sheets/wave1-substrate.md), [wave2-substrate](../research/fact-sheets/wave2-substrate.md)
- [Lane E code-gap matrix](../research/conversation-mining-2026-08-28/E-code-gap-matrix.md) — 136 claims audited
- [Target architecture](target-architecture.md)
- [Authority index](../reference/authority-index.md)
