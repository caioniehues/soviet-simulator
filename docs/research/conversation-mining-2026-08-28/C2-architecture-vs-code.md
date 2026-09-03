# C2 — Rust architecture proposals versus the real codebase

## 0. Summary (top ten findings)

1. **C2-01** The ten-phase deterministic order (COMMAND->ACCOUNTING) does NOT match the codebase. Egregoria runs 18 systems in a flat registration order with no phase boundaries; reordering changes replay hashes.
2. **C2-06** The PlannerSnapshot boundary does not exist. The UI reaches into `Simulation` through `Arc&lt;RwLock&lt;Simulation&gt;&gt;` and calls `sim.read::&lt;T&gt;()` / `sim.map()` / `sim.world()` directly. At least 40 call sites in `native_app/` read arbitrary resource types.
3. **C2-04** Market already holds six responsibilities in one 1,500-line struct but they are NOT the six the conversation named. The actual six are: order book, trade matching, dispatch lifecycle, retail claims, external trade, and price calculation.
4. **C2-03** No CitizenRecord/CitizenBody split exists. `HumanEnt` is a single monolithic struct combining identity (PersonalInfo), spatial (Transform, Pedestrian), economic (Bought, BuyFood, Work), and routing (Router, Itinerary) into one heap allocation per human.
5. **C2-09** Randomness is NOT keyed. A single global `RandProvider` (Xorshift128) is drawn sequentially; any insertion-order change in entity iteration breaks determinism.
6. **C2-05** No cadence bands exist. Every system runs every tick (50 Hz). The only frequency-varying thing is `HumanDecision.wait` (a per-citizen random 30-80 tick cooldown).
7. **C2-11** The Change Journal / incremental Observatory does not exist. `rerun.rs` is 48 lines of dead code (all commented out). `EcoStats` records ring-buffer trade history at 4 frequency bands but emits no events.
8. **C2-13** The "no hidden dishonest flag" rule is ALREADY SATISFIED at the simulation level. The walking skeleton uses `request_multiplier` in Lua recipes (flour-factory: 4, meat-facility: 3), a per-prototype data field, not a per-entity runtime flag. The UI does not yet expose the requested-versus-consumed discrepancy to the Planner.
9. **C2-14** The multiplayer crate assumes lockstep determinism (Frame-numbered inputs, identical ticking). Any proposed parallelism within a tick must produce bit-identical results or multiplayer breaks.
10. **C2-07** Typed IDs exist for entities (VehicleID, HumanID, etc.) via `slotmapd::new_key_type!` but there are NO unit newtypes for physical quantities beyond `Money` and `Power`. Mass, Volume, Energy, etc. are raw `f32`/`i32`.

## 1. Extracted items

| ID | Statement | Source line(s) | Verdict |
|---|---|---|---|
| C2-01 | Ten-phase deterministic order COMMAND->TOPOLOGY->ALLOCATION->DECISION->ROUTING->MOVEMENT->ARRIVAL->PRODUCTION->UTILITIES->ACCOUNTING | conv:39-40 | WRONG — see §2.1 |
| C2-02 | Parallelize inside phases using deterministic intent buffers and merge/commit | conv:41, 601-610 | PLAUSIBLE — ParCommandBuffer exists but is sequential; intent-buffer pattern is there, parallelism is not |
| C2-03 | Split persistent CitizenRecord from active CitizenBody | conv:37, 574-575, 1102-1106 | ABSENT — single HumanEnt struct |
| C2-04 | Decompose Market into demand, allocation, inventory reservation, logistics/fulfillment, retail, border trade | conv:42 | PLAUSIBLE — Market already has six responsibilities but differently factored |
| C2-05 | Cadence bands rather than one universal tick frequency | conv:38 | ABSENT — every system runs every 20ms tick |
| C2-06 | PlannerSnapshot — UI receives snapshot, not &Simulation | conv:533-545, 677-679 | ABSENT — UI holds `Arc&lt;RwLock&lt;Simulation&gt;&gt;` |
| C2-07 | Typed IDs and unit/state newtypes | conv:548-565 | PARTIAL — entity IDs yes, physical unit newtypes no |
| C2-08 | Persistent vs ephemeral identity — stable dense indexes vs generational slotmaps | conv:567-571 | PARTIAL — all entities use generational slotmaps; no stable dense CitizenId |
| C2-09 | Keyed randomness — stable random keys from seed+domain+entity+event | conv:618-620 | ABSENT — single global RandProvider, sequentially drawn |
| C2-10 | Hierarchical routing and topology/traffic caches | conv:43 | ABSENT — flat A* per pathfind, no contraction hierarchy |
| C2-11 | Incremental Observatory / Change Journal | conv:638-654 | ABSENT — rerun.rs is dead; EcoStats is ring-buffer history only |
| C2-12 | Shared topology across utilities, domain physics distinct | conv:44, 673-674 | PARTIAL — electricity uses road adjacency as topology; no other utility shares it |
| C2-13 | No hidden dishonest flag — Planner infers from observable state | conv:47 | CONFIRMED — request_multiplier is data, not runtime flag |
| C2-14 | Deterministic parallelism with intent buffers | conv:601-610 | PLAUSIBLE — ParCommandBuffer is the mechanism but execution is sequential |
| C2-15 | SoA citizens / CitizenStore for dense columnar storage | conv:572-575 | ABSENT — AoS HopSlotMap<HumanID, HumanEnt> |
| C2-16 | Temporal LOD — scheduled wake-ups rather than every-frame | conv:577-580 | PARTIAL — HumanDecision.wait is a crude wake-up; not a general mechanism |
| C2-17 | Deterministic event calendar / timing wheel | conv:581-586 | ABSENT |
| C2-18 | Semantic LOD — aggregate until causal distinction matters | conv:588-592 | ABSENT — no LOD system for resources |
| C2-19 | Fixed resource arrays instead of per-holder hash maps | conv:593-595 | ABSENT — BTreeMap<ItemID, SingleMarket> with BTreeMap<SoulID, i32> capital |
| C2-20 | Integer/fixed-point authoritative state | conv:597-599 | PARTIAL — Money is i64 fixed-point; quantities are plain i32 (close); positions are f32 |
| C2-21 | Typed system contexts replacing &mut Simulation | conv:614-617 | ABSENT — systems take (&mut World, &mut Resources) or (&mut Simulation) |
| C2-22 | Bitset society — cohort queries via bitset intersection | conv:622-624 | ABSENT |
| C2-23 | Heterogeneous data structures (dense arrays, bitsets, spatial grids, graphs, event calendar, causal DAG) | conv:626-636 | PARTIAL — spatial grid (TransportGrid), graph (ElectricityCache), but no bitsets/event calendar/causal DAG |
| C2-24 | Shadow simulation / Gosplan Computer — branch headless forecasts | conv:662-668 | PLAUSIBLE — headless binary exists, Replay exists; no branching API |
| C2-25 | LP/MILP feasibility analysis | conv:669-672 | ABSENT |
| C2-26 | Four snapshots (Planner, Render, Audio, Debug) | conv:677-679 | ABSENT — single shared Simulation, everything reads the same lock |
| C2-27 | Causal inspector STATUS/CAUSE/TREND/POLICY/PHYSICAL CHAIN | conv:45 | ABSENT |
| C2-28 | SIMD/contiguous memory discipline | conv:680-682 | PARTIAL — HopSlotMap is not contiguous; no SIMD |
| C2-29 | Compile-time size assertions for hot structs | conv:683-684 | ABSENT |
| C2-30 | Stable versioned release saves | conv:685-686 | PARTIAL — VERSION string is checked on load; major.minor mismatch only warns |
| C2-31 | Property-based testing for conservation/state-machine correctness | conv:687-688 | PARTIAL — quickcheck_map_ser exists for map serde; no property test for economy conservation |

## 2. Validation detail

### 2.1 C2-01: Ten-phase deterministic order vs actual system order

The conversation proposes: `COMMAND -> TOPOLOGY -> ALLOCATION -> DECISION -> ROUTING -> MOVEMENT -> ARRIVAL -> PRODUCTION -> UTILITIES -> ACCOUNTING`.

The actual system order in `simulation/src/init.rs:54-114` (registration order = execution order per `SeqSchedule::execute`):

| # | System name | Proposed phase equivalent |
|---|---|---|
| 1 | `electricity_flow_system` | UTILITIES |
| 2 | `dispatch_system` | ALLOCATION/LOGISTICS |
| 3 | `update_decision_system` | DECISION |
| 4 | `company_system` | PRODUCTION |
| 5 | `pedestrian_decision_system` | DECISION |
| 6 | `transport_grid_synchronize` | MOVEMENT |
| 7 | `locomotive_system` | MOVEMENT |
| 8 | `vehicle_decision_system` | DECISION/MOVEMENT |
| 9 | `vehicle_state_update_system` | MOVEMENT |
| 10 | `routing_changed_system` | ROUTING |
| 11 | `routing_update_system` | ROUTING |
| 12 | `itinerary_update` | MOVEMENT |
| 13 | `market_update` | ALLOCATION/ACCOUNTING |
| 14 | `train_reservations_update` | ROUTING |
| 15 | `freight_station` | LOGISTICS |
| 16 | `random_vehicles` | (test) |
| 17 | `update_map` | TOPOLOGY |
| 18 | `add_souls_to_empty_buildings` | ARRIVAL/SPAWNING |

Key observations:

- **UTILITIES (electricity) runs FIRST**, before any decision or production. The conversation puts it ninth.
- **COMMAND is not a system — it's `WorldCommand::apply` inside `Simulation::tick` at `lib.rs:246-249`**, before the schedule runs. This part matches the proposal.
- **TOPOLOGY (`update_map`) runs second-last**, not second. The proposed order has it after COMMAND.
- **PRODUCTION (company_system) runs before ROUTING**, not after.
- **ACCOUNTING (market_update) runs after MOVEMENT**, which roughly matches, but it also handles ALLOCATION (trade matching + dispatch advancement) — these are not separated.

**Reordering these systems would change replay hashes.** The determinism test `test_world_survives_serde` (`simulation/src/tests/test_iso.rs`) replays commands and checks that serialized state matches after round-tripping. Any system reorder changes when entities interact within a tick, producing different state.

### 2.2 C2-03: CitizenRecord/CitizenBody split

`world.rs:87-105` defines `HumanEnt`:
```rust
pub struct HumanEnt {
    pub trans: Transform,          // spatial
    pub speed: Speed,              // movement
    pub location: Location,        // spatial state
    pub pedestrian: Pedestrian,    // movement flavor
    pub collider: Option<Transporter>, // physics
    pub router: Router,            // pathfinding
    pub it: Itinerary,             // movement plan
    pub decision: HumanDecision,   // behavior
    pub home: Home,                // desire
    pub food: BuyFood,             // desire
    pub bought: Bought,            // economy
    pub work: Option<Work>,        // labor
    pub personal_info: Box<PersonalInfo>, // identity (name, age, gender)
}
```

`PersonalInfo` (`souls/human.rs:42-46`) is Box-allocated, suggesting some awareness of the hot/cold split, but it is not a separate store — it's a field on the entity struct.

For the proposed CitizenRecord/Body split, every `HumanEnt` field would need to be classified as record (persistent, cold: name, age, gender, employment history, housing queue position, qualifications) vs body (ephemeral, hot: position, speed, itinerary, current decision). Currently there is no mechanism for a citizen to exist without a body — `spawn_human` creates the full `HumanEnt` at once.

### 2.3 C2-04: Market decomposition

The conversation proposes decomposing Market into six responsibilities: **demand, allocation, inventory reservation, logistics/fulfillment, retail provisioning, border trade**.

The actual Market (`economy/market.rs`) has these responsibilities, traced by method:

1. **Order book** — `buy()`, `sell()`, `sell_all()`, `buy_until()`, `register()`, `produce()` (lines 281-545)
2. **Trade matching** — `make_trades()` lines 551-789: the O(n²) buyer-seller match, distance-sorted
3. **Dispatch lifecycle** — `advance_dispatches()` lines 825-1289: truck assignment, routing, loading/unloading, physical goods movement
4. **Retail claims** — `RetailClaim` struct lines 184-191, `settle_retail()` lines 520-530, TTL-based expiry in `advance_dispatches`
5. **External trade** — integrated into `make_trades()` lines 663-786: buy-side imports via freight station, sell-side exports
6. **Price calculation** — `calculate_prices()` lines 1298-1368: recursive cost-based pricing for border trade

The conversation's six map roughly to these six but with different boundaries. The conversation puts "logistics/fulfillment" as one responsibility; in the code, dispatch is by far the largest (464 lines, nearly a third of the file). Retail is absent from the conversation's list but is a first-class concern in the code.

### 2.4 C2-06: PlannerSnapshot boundary

The UI in `native_app/src/game_loop.rs:33` holds:
```rust
pub sim: Arc<RwLock<Simulation>>,
```

Every GUI function receives `&Simulation` directly. Examples from `native_app/src/`:
- `gui/hud/time_controls.rs:34`: `sim.read::<GameTime>().daytime`
- `gui/hud/menu.rs:37`: `sim.read::<Government>().money`
- `gui/hud.rs:63`: `sim.map()`, `sim.read::<ElectricityFlow>()`
- `debug_gui/debug_inspect.rs:131`: `sim.read::<Market>()`
- `game_loop.rs:163`: `gui::run_ui_systems(&self.sim.read().unwrap(), &self.uiw)`

There are at least 40 places where the UI reads arbitrary typed resources. To introduce a PlannerSnapshot, every one of these call sites must be audited and either: (a) converted to read from a snapshot struct, or (b) explicitly documented as debug/render access that bypasses the information boundary. This is the largest migration surface of any proposal.

### 2.5 C2-09: Randomness

`simulation/src/utils/rand_provider.rs` implements Xorshift128. It is registered as a single global resource (`init.rs:138-140`):
```rust
register_resource::<RandProvider, Bincode>(&mut registry, "randprovider", || {
    RandProvider::new(RNG_SEED)
});
```

All randomness flows through this one stream: `spawn_human` calls `sim.write::<RandProvider>()` three times (name, shirt color, pedestrian), `spawn_parked_vehicle` draws from it, etc. The stream is deterministic given the same seed and the same call order, but any change in entity creation order (adding a new building type that spawns before an existing one) changes the entire downstream random sequence.

Keyed randomness would derive per-entity random state from `seed + entity_id + event_index`, making each entity's randomness independent of insertion order. This is a precondition for parallelism.

### 2.6 C2-10: Hierarchical routing

Pathfinding (`map/pathfinding.rs`) uses `pathfinding::directed::astar::astar` (the `pathfinding` crate) for all three path types (pedestrian, vehicle, rail). There is no contraction hierarchy, no hierarchical decomposition, no cached routing. Every `Itinerary::route` call runs a fresh A* over the lane graph.

The cost model is pure distance (lane length as `OrderedFloat`), with a 1.3x heuristic multiplier for pedestrians. There is no traffic-aware cost (no BPR/Gawron despite the conversation's references to them).

### 2.7 C2-12: Shared topology

`ElectricityCache` (`map/electricity_cache.rs`) builds a graph over `NetworkObjectID` which includes roads, intersections, and buildings. Buildings connect to the road they face. Two buildings share an electricity network if and only if they are connected by a path of roads and intersections.

This IS the union-find over road adjacency that the CLAUDE.md warns must be replaced by laid wire. There is no Water, Sewage, Heat, or Gas topology. The only shared infrastructure is the road graph itself.

### 2.8 C2-13: The dishonest enterprise — no hidden flag

The walking skeleton's dishonest enterprise mechanism:
- `prototypes/src/types/recipe.rs:52`: `pub request_multiplier: i32`
- `base_mod/companies.lua:40`: `request_multiplier = 4` (flour factory requests 4x its consumption)
- `souls/goods_company.rs:23-24`: `recipe_init` calls `market.set_requested(soul, item.id, item.amount as u32 * recipe.request_multiplier as u32)` then `market.buy_until(soul, near, item.id, qty)`

This is a **per-prototype data field**, not a per-entity runtime boolean. A flour factory requests 4 units of cereal per cycle but only consumes 1. The surplus sits as inventory (`market.capital`). There is no `dishonest: bool` on `GoodsCompanyState` — the field's absence is confirmed by reading `goods_company.rs:70-78`.

The conversation's rule "do not store a hidden dishonest flag" is already satisfied. This establishes a simulation-level discrepancy, but the current native app does not expose `Market::requested()`; the Planner cannot yet observe it through the UI.

### 2.9 C2-14: Multiplayer and determinism

The `networking/` crate implements lockstep deterministic multiplayer:
- `networking/src/lib.rs:31`: `Frame(pub u64)` — every tick has a frame number
- Server collects `PlayerInput` from all clients, merges them, broadcasts `MergedInputs` for frame N
- `headless/src/main.rs:65-69`: `assert_eq!(frame.frame.0, w.get_tick() + 1)` — frames must advance by exactly one
- `native_app/src/network.rs:199`: same assertion on the client side

Any proposal that introduces non-deterministic parallelism (e.g., rayon's work-stealing) would break this. The conversation's "deterministic parallelism" proposal (intent buffers + deterministic merge + stable sort) is compatible only if the merge order is canonical (e.g., sorted by entity ID).

### 2.10 C2-24: Shadow simulation / Gosplan Computer

The headless binary (`headless/src/main.rs`) proves that `Simulation` can tick without a renderer. The `Replay` mechanism records `WorldCommand`s per tick and can replay them (`Simulation::from_replay`). The determinism test (`test_iso.rs`) runs two replays in parallel and checks byte-equality.

What is missing for a Gosplan Computer:
- No branching API: you cannot fork a `Simulation` at tick N, feed it hypothetical commands, and compare outcomes
- No snapshot-from-Planner-view: a shadow sim would see physical truth, not reported state
- Serialization round-trip is CompressedBincode, which is fast enough for save/load but may be too heavy for frequent forking

The seam for this exists (Simulation is Serialize + Deserialize, headless proves headless ticking), but the cost of cloning a full `Simulation` is the barrier.

## 3. Deeper mechanics — migration sketches

### 3.1 Dependency order of proposals

The proposals form a DAG. Some unlock others:

```
C2-09 Keyed randomness ──────────────────┐
                                          ├─> C2-14 Deterministic parallelism
C2-21 Typed system contexts ──────────────┘
                                          │
C2-01 Phase ordering ─────────────────────┤
                                          │
C2-05 Cadence bands ──────────────────────┤
                                          │
C2-02 Intent buffers (parallelism) ───────┘

C2-03 CitizenRecord/Body ──> C2-15 SoA citizens ──> C2-22 Bitset society
                         └──> C2-16 Temporal LOD ──> C2-17 Event calendar

C2-06 PlannerSnapshot ──> C2-26 Four snapshots ──> C2-24 Gosplan Computer

C2-11 Change Journal ──> C2-27 Causal inspector

C2-04 Market decomposition ──> (independent, no downstream)

C2-10 Hierarchical routing ──> (independent, improves perf)

C2-12 Shared topology ──> (independent, unlocks Water/Sewage/Heat/Gas)
```

### 3.2 Per-proposal migration sketch

#### C2-09 Keyed randomness (MUST-DO-FIRST for parallelism)

**What exists:** Single `RandProvider` in `Resources`, drawn sequentially.
**Seam:** Replace `sim.write::<RandProvider>().next_u32()` calls with `keyed_rand(seed, entity_id, event_index)`. A pure function, no mutable state.
**First commit:** Add `fn keyed_rand(seed: u64, entity_key: u64, event: u32) -> u32` to `simulation/src/utils/`. Convert one system (e.g., `spawn_human`) to use it. Verify determinism test still passes.
**Invariant:** `test_world_survives_serde` must pass after every change; replay hashes must match.
**Size:** ~50 call sites to audit; medium effort (2-3 days).

#### C2-01 Phase ordering

**What exists:** 18 systems registered in `init.rs` in a flat list, executed sequentially.
**Seam:** Group the 18 systems into named phases in `init.rs`. Add phase markers to `SeqSchedule` (e.g., `schedule.begin_phase("COMMAND")`). Do NOT reorder yet — just label.
**First commit:** Add phase labels without reordering. The schedule output can now report time-per-phase. Verify determinism unchanged.
**Second commit:** Reorder systems within phases if the phase boundary guarantees it's safe. Any reorder that changes replay hashes requires a replay migration (invalidating old replays or bumping a replay version).
**Invariant:** Replay determinism. Old saves.
**Risk:** The proposed order conflicts with the existing order significantly (electricity first vs last, map update last vs second). Adopting the proposed order is a non-trivial behavioral change.
**Size:** Labeling: small (1 day). Actual reorder: large, requires extensive determinism testing.

#### C2-03 CitizenRecord/Body split

**What exists:** `HumanEnt` is a single struct in `HopSlotMap<HumanID, HumanEnt>`.
**Seam:** Extract `PersonalInfo`, `Home` (building assignment), employment history into a `CitizenRecord` struct stored in a parallel `SlotMap<HumanID, CitizenRecord>` in Resources. `HumanEnt` keeps only hot movement/decision state. `HumanID` remains the key.
**First commit:** Move `personal_info` out of `HumanEnt` into `Resources` as a separate `SlotMap`. `spawn_human` writes to both. Systems that need the name (debug inspector) read from the new store.
**Invariant:** Save format changes — serialization of `HumanEnt` changes. Old saves break unless migration is added.
**Size:** Large — touches `spawn_human`, `update_decision`, every inspector, save/load. 1-2 weeks.

#### C2-04 Market decomposition

**What exists:** `Market` struct, 1,500 lines, six interleaved responsibilities.
**Seam:** Extract `DispatchManager` (lines 825-1289 of `market.rs`) into its own struct in `economy/dispatch_manager.rs`. Market holds dispatches by delegation. `RetailClaim` handling also extracts.
**First commit:** Move `Dispatch`, `DispatchState`, `advance_dispatches`, and `release_tosource_truck` into `DispatchManager`. Market delegates to it. All tests pass.
**Invariant:** Behavior and serialization must be identical. No logic change.
**Size:** Medium — mostly mechanical code movement. 2-3 days.

#### C2-05 Cadence bands

**What exists:** Every system runs every tick.
**Seam:** Add a `cadence: u32` field to system registration. `SeqSchedule::execute` skips systems whose `tick % cadence != 0`. Start with `cadence=1` for everything (no change).
**First commit:** Add the field, default to 1. No behavior change.
**Second commit:** Move `electricity_flow_system` to cadence=5 (every 100ms). Verify the game still behaves correctly — blackouts may respond 100ms slower.
**Invariant:** Replay hashes change at every cadence boundary. Replays become cadence-version-dependent.
**Size:** Small mechanism, large validation. 1 day for mechanism, unknown for tuning.

#### C2-06 PlannerSnapshot boundary

**What exists:** UI calls `sim.read::<Market>()`, `sim.map()`, etc. directly.
**Seam:** Create a `PlannerView` struct that exposes only what the Planner should know. Populate it once per tick from `Simulation`. UI receives `&PlannerView` instead of `&Simulation`.
**First commit:** Create `PlannerView` with one field (e.g., `money: Money`). Populate it in `game_loop.rs` after `tick()`. Convert `menu_bar` to read from it. Leave all other UI on `&Simulation`.
**Invariant:** No simulation behavior change. UI-only refactor.
**Call sites to move:** ~40 in `native_app/`. This is the most tedious migration of any proposal.
**Size:** Large — 2-4 weeks to fully migrate all UI. Can be incremental.

#### C2-10 Hierarchical routing

**What exists:** Per-query A* on the lane graph.
**Seam:** Build a contraction hierarchy (or similar) on `Map::update()`. `Itinerary::route` queries the CH instead of raw A*.
**First commit:** Add a CH build step (can use the `fast_paths` crate or similar). Gate it behind a feature flag. Compare path results with A* to validate.
**Invariant:** Routes must be identical (or acceptably similar) to not change game behavior. Determinism preserved if the CH is built deterministically.
**Size:** Medium — depends on crate choice. 1-2 weeks.

#### C2-11 Change Journal

**What exists:** `EcoStats` records trade history in ring buffers at 4 frequency levels. `rerun.rs` is dead code.
**Seam:** Define a `ChangeEvent` enum. Systems emit events into a `Vec<ChangeEvent>` resource. The Change Journal collects them per tick and makes them queryable.
**First commit:** Create `ChangeJournal` resource with `push(event)` and `drain()`. Emit one event type (e.g., `TradeMatched { buyer, seller, kind, qty }`). Wire `market_update` to emit it.
**Invariant:** Read-only addition — no simulation logic change. The journal is not serialized (transient).
**Size:** Small to start, grows as more systems emit events. 1-2 days for the skeleton.

#### C2-14 Deterministic parallelism

**Depends on:** C2-09 (keyed randomness), C2-21 (typed contexts), C2-01 (phase ordering).
**What exists:** `SeqSchedule::execute` runs systems one at a time. `ParCommandBuffer` exists as the merge mechanism.
**Seam:** Within a phase, systems that touch disjoint resources can run in parallel. The merge step (ParCommandBuffer::apply) already runs between systems.
**First commit:** Identify two systems within the same phase that provably access disjoint data. Run them on a rayon thread pool. Verify determinism.
**Invariant:** `test_world_survives_serde` must pass. Multiplayer determinism must hold.
**Risk:** Very high. Egregoria's Resources use `RefCell`-like interior mutability — two systems reading the same resource type from different threads would panic (or require `RwLock` per resource). This is a deep architectural change.
**Size:** Very large — months of work. Not recommended until the simpler prerequisites are done.

#### C2-21 Typed system contexts

**What exists:** Systems take `(&mut World, &mut Resources)` or `(&mut Simulation)`. Resources is a type-erased `HashMap<TypeId, Box<dyn Any>>` with runtime borrow checking.
**Seam:** Define trait-bounded context structs per system (e.g., `struct CompanyContext<'a> { map: Ref<'a, Map>, market: Ref<'a, Market>, ... }`). Construct them at the call site.
**First commit:** Convert one system (e.g., `electricity_flow_system`) to take a typed context instead of raw Resources. The system still runs through the scheduler via a wrapper.
**Invariant:** No behavior change. Pure refactor.
**Risk:** Explosion of context types — one per system or per system group. Trait bounds become verbose.
**Size:** Large — every system must be converted. 2-4 weeks.

#### C2-24 Shadow simulation / Gosplan Computer

**Depends on:** C2-06 (PlannerSnapshot — so the shadow sees reported state, not truth).
**What exists:** Simulation is Serialize+Deserialize. Headless binary ticks without a renderer.
**Seam:** Add `Simulation::fork() -> Simulation` that serializes and deserializes (expensive but correct). The shadow runs headless with hypothetical commands.
**First commit:** Add `fork()`. Add a test that forks at tick N, advances both independently, and verifies they diverge only based on different commands.
**Invariant:** The fork must not share mutable state with the original. Currently Resources uses `Rc&lt;RefCell&gt;` internally — verify no aliasing.
**Size:** Small for the API, large for making it fast enough to be useful. The serialization cost is ~100ms for a moderate save, too slow for frequent use.

## 4. Missed / not apparent — architectural risks

### 4.1 Save-format churn

Nearly every structural proposal (C2-03, C2-15, C2-19, C2-05) changes serialized state. Egregoria's save format is raw `Bincode::encode` of every resource and the World struct. There is no schema migration mechanism — `Simulation::deserialize` (`lib.rs:389-441`) initializes defaults for missing resources but cannot restructure existing ones. The version check (`lib.rs:407-415`) only warns on major version mismatch; it does not transform data.

**Risk:** Each migration that changes the layout of a serialized resource invalidates all existing saves. There is no migration path — the user must start a new game. For a project that values "one continuous save," this is severe.

**Mitigation:** Before any structural change, implement a `SaveMigration` trait that can transform `FastMap<String, Vec<u8>>` entries from version N to N+1. The VERSION file controls which migrations apply.

### 4.2 Replay-test brittleness

`test_world_survives_serde` (`test_iso.rs:241-314`) replays from a baked-in `world_replay.json`. It checks: (a) two replays from the same recording produce identical state, and (b) serialize/deserialize round-trips produce identical state. It does NOT check: (c) that a replayed world is identical to one that was simulated live — because there is no reference "live" world to compare against.

Any change to system ordering, randomness, or entity iteration order produces a different world from the same replay, causing this test to diverge. The test detects the divergence but cannot tell you which change caused it. The binary search narrowing loop (`check_size`, `check_start`) helps locate the tick but not the system.

**Risk:** As proposals are implemented, this test will fail constantly. Each fix requires recording a new `world_replay.json` from the current code, which means the test only proves "serde round-trip is stable for THIS version," not "the simulation is deterministic across versions."

**Mitigation:** Add a second test mode that runs two fresh simulations (not from replay) for N ticks and verifies they produce identical state. This is the conversation's proposed "hash every tick" approach and would catch non-determinism introduced by code changes regardless of replay format.

### 4.3 The cost of typed contexts in a &mut Simulation world

The conversation proposes replacing `&mut Simulation` with narrow typed contexts (C2-21). Currently 17 of 18 systems take `fn(&mut World, &mut Resources)` and 1 takes `fn(&mut Simulation)`. The Resources type provides runtime-checked borrows — `resources.read::<T>()` panics if T is already mutably borrowed.

Typed contexts would move this check to compile time, which is strictly better. But the current codebase has a pattern the conversation did not address: **deferred command callbacks**. `ParCommandBuffer::exec_ent` takes a closure `FnOnce(&mut Simulation)` that runs later. These closures capture arbitrary state and mutate arbitrary resources:

```rust
cbuf_vehicle.exec_ent(v, move |sim| {
    sim.write::<Market>().release_tosource_truck(v);
    sim.write::<Dispatcher>().free(DispatchID::SmallTruck(v));
});
```

These closures must have access to `&mut Simulation` because their resource needs are not known at registration time. Typed contexts cannot express this without either: (a) giving deferred callbacks full `&mut Simulation` access (breaking the narrowing), or (b) pre-declaring which resources each callback needs (verbose and error-prone).

### 4.4 Multiplayer lockstep and proposed parallelism

The `networking/` crate (`networking/src/lib.rs`, `networking/src/server/`) implements frame-locked multiplayer. The server collects inputs from all clients for frame N, merges them, broadcasts the merged set, and every participant applies the same commands in the same order.

The conversation's "deterministic parallelism" (C2-14) is compatible IF:
- Parallel workers within a phase produce identical results regardless of thread scheduling
- The merge/commit step is a deterministic function of intent buffers, ordered by a canonical key
- The RNG is keyed per entity (C2-09), not drawn from a shared mutable stream

The conversation glossed over the fact that `ParCommandBuffer`'s current `exec_ent` / `exec_on` closures run sequentially in `SeqSchedule::execute`. If these were run in parallel, they could observe each other's mutations to shared Resources (Market, Dispatcher). The current architecture cannot support parallel execution of these closures without a deep refactor of the Resource system.

### 4.5 The &mut Simulation leak surface

17 of the 18 registered systems use the `register_system` path which takes `fn(&mut World, &mut Resources)`. 1 uses `register_system_sim` which takes `fn(&mut Simulation)`. But `ParCommandBuffer::apply` runs **after every system** (`scheduler.rs:46-51`) and its `exec_ent` closures take `FnOnce(&mut Simulation)` — so effectively every system has access to `&mut Simulation` through deferred callbacks.

This means the conversation's proposal for narrow system contexts (C2-21) must address the deferred callback path, which is the primary mutation channel for cross-system effects (market releases, dispatcher frees, worker assignments). This is not a minor edge case — it is the main way entities affect each other.

### 4.6 Float determinism

The codebase uses `f32` for positions, speeds, distances, and pathfinding costs. `f32` arithmetic is deterministic on a single platform with consistent compiler settings, but not across platforms or compiler versions (different FPU rounding, SSE vs x87, etc.). The conversation mentions "integer/fixed-point authoritative state" (C2-20) but the codebase is heavily f32-based.

For multiplayer across different machines, this is a live risk. The current test (`test_iso.rs`) runs on one machine; it does not test cross-platform determinism. `OrderedFloat` is used for pathfinding costs, which handles NaN but not rounding.

### 4.7 The `common::rand::rand2` backdoor

Besides `RandProvider`, there is a second random source: `common::rand::rand2(pos.x, pos.y)` used in `update_decision` (`souls/human.rs:188`). This is a deterministic hash function of position, not a sequential RNG draw, so it does not break determinism — but it IS a keyed-random pattern already in use, contradicting the finding that keyed randomness is absent. It is partial: keyed by position, not by entity ID + event.

## 5. Cross-lane hooks

| What | Lane | Why it matters |
|---|---|---|
| `request_multiplier` is Lua data, not a runtime flag | Lane A (economy) | The dishonest enterprise mechanism is a prototype field; lane A must map how it produces the hoarding observable |
| Market's actual six responsibilities vs proposed six | Lane A (economy) | Lane A evaluates the Kornai model against the code; the decomposition is their concern |
| Electricity as union-find over roads | Lane C1 (crates) | C1 should evaluate whether `fast_paths` or similar CH crate is viable for routing |
| `PersonalInfo` (name, age, gender) is the only citizen identity | Lane B1 (society) | B1's citizen simulation proposals depend on CitizenRecord; the current state is minimal |
| Multiplayer lockstep constrains all parallelism proposals | All lanes | Any proposal must preserve frame-level determinism |
| Save format has no migration mechanism | All lanes | Every structural change risks breaking save continuity |

## 6. Open questions for the user

1. **Phase reordering**: Is the ten-phase order from the conversation a hard requirement, or can the actual system order be relabeled into fewer phases that preserve current behavior?
2. **Save migration**: Should a save-migration mechanism be built before any structural refactor, or is "new save required" acceptable during pre-1.0 development?
3. **PlannerSnapshot scope**: Which resources should the Planner NOT see? The conversation says "reported institutional reality" but does not enumerate which data that excludes. The current UI reads Market capital directly — should the Planner only see aggregated trade history (EcoStats)?
4. **Multiplayer priority**: Is lockstep multiplayer a 1.0 requirement, or can it be dropped to unblock architectural changes? The networking crate is a dependency of headless; removing it affects the headless server.
5. **Replay compatibility**: Should the project maintain replay compatibility across versions, or is `world_replay.json` allowed to be regenerated after each structural change?
6. **Float determinism**: Is cross-platform multiplayer a goal? If so, an f32-to-fixed-point migration for authoritative state is needed before any parallelism work.

## 7. Sources

### Files read

| File | What it told us |
|---|---|
| `simulation/src/world.rs` | Hand-rolled ECS: World struct with HopSlotMap per entity type, Entity/EntityID traits, WorldTransform |
| `simulation/src/lib.rs` | Simulation struct (world + Resources), tick() method, COMMAND application, serialization |
| `simulation/src/init.rs:54-114` | Actual system registration order — the ground truth for C2-01 |
| `simulation/src/init.rs:116-143` | Resource registration — all serialized resources listed |
| `simulation/src/world_command.rs` | WorldCommand enum — the COMMAND phase; apply() method |
| `simulation/src/economy/market.rs` | Market struct, SingleMarket, Trade, Dispatch, RetailClaim, make_trades, advance_dispatches |
| `simulation/src/economy/mod.rs` | market_update system — trade matching, worker payment, dispatch advancement |
| `simulation/src/economy/ecostats.rs` | EcoStats ring-buffer trade history at 4 frequency levels |
| `simulation/src/souls/human.rs` | HumanEnt, PersonalInfo, update_decision_system, spawn_human |
| `simulation/src/souls/goods_company.rs` | recipe_init, recipe_should_produce, recipe_act, company_system — request_multiplier usage |
| `simulation/src/souls/mod.rs` | add_souls_to_empty_buildings |
| `simulation/src/map/pathfinding.rs` | Flat A* pathfinding, no hierarchical routing |
| `simulation/src/map/electricity_cache.rs` | ElectricityCache — union-find over road adjacency graph |
| `simulation/src/map_dynamic/electricity.rs` | electricity_flow_system — blackout detection |
| `simulation/src/map_dynamic/dispatch.rs` | Dispatcher — entity position cache for truck/train dispatch |
| `simulation/src/utils/rand_provider.rs` | Xorshift128 single global RNG |
| `simulation/src/utils/scheduler.rs` | SeqSchedule — sequential system execution with ParCommandBuffer::apply between each |
| `simulation/src/rerun.rs` | Dead code — 48 lines, all commented out |
| `simulation/src/tests/test_iso.rs` | Determinism test — replay + serde round-trip equality |
| `simulation/src/multiplayer/mod.rs` | MultiplayerState — just a Chat struct |
| `native_app/src/game_loop.rs` | `Arc&lt;RwLock&lt;Simulation&gt;&gt;` — UI access pattern |
| `native_app/src/network.rs` | Lockstep multiplayer — Frame-based deterministic ticking |
| `networking/src/lib.rs` | Frame type, lockstep protocol |
| `headless/src/main.rs` | Headless server — ticks Simulation without renderer |
| `common/src/timestep.rs` | Fixed timestep (20ms) |
| `prototypes/src/types/recipe.rs:52` | `request_multiplier: i32` definition |
| `prototypes/src/validation.rs:65-74` | request_multiplier validation (must be >= 1) |
| `base_mod/companies.lua:40,582` | request_multiplier = 4 (flour), 3 (meat) |
| `docs/reference/architecture/substrate.md` | Substrate fact-sheet classifications |
| `docs/plan/proposals/gosplan.md` | GOSPLAN process overhaul proposal |

### Conversation source
`/home/caio/Downloads/soviet_simulator_conversation_export.md` — lines 36-51 (architecture conclusions), 522-693 (Rust architecture research), 1102-1116 (CIA-derived Rust model).
