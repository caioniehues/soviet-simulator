# Lane D — Vehicles, Traffic, and Utilities

## 0. Summary

The conversation's deep-dive on vehicles, traffic, and utilities (lines 696–892) proposes an
**inertia thesis**: model each network by its characteristic physical inertia rather than treating
infrastructure as generic capacity graphs. The ten most important findings:

1. **D-01** The inertia thesis is sound and novel for a game of this scope. Each network's delay
   signature (mechanical, queue, pressure, thermal, electrical, linepack) gives the player a
   distinct failure mode and planning instrument.
2. **D-05** The current vehicle model has **no mass, no grade, no cargo, no owner, no capacity**.
   Vehicle physics is purely kinematic (acceleration/deceleration as constants per kind). The
   conversation's "loaded vs empty on grades" is entirely absent and would need a physics rewrite.
3. **D-06** The collision avoidance is a spatial-grid nearest-neighbour cone check, not IDM. It has
   a gridlock detector (`Panicking` state, `road.rs:217-225`) that panics and waits, confirming the
   conversation's known gridlock behaviour — but recovery is just a random wait, not a real
   resolution.
4. **D-08** Pathfinding cost is `length / speed_limit + noise` — no BPR, no Gawron damping, no
   congestion signal at all. The spec drafts prescribe BPR `1 + 0.15*(v/c)^4` and Gawron 0.3/0.7
   blending but these are UNIMPLEMENTED.
5. **D-10** Electricity is a union-find over road/intersection adjacency
   (`electricity_cache.rs:6-63`), not explicit wire. Binary blackout, no priority shedding, no
   storage, no ramp rates. The spec requires explicit wire topology (`SPEC-ELECTRICITY-001`).
6. **D-14/15/16/17** Water, sewage, heating, and gas have **zero substrate** — no building kind, no
   registered system, no data structure. Every network the conversation describes for these is
   greenfield.
7. **D-03** Rail has the most developed model: consist mass/length/power/braking from prototype data
   (`train.rs:58-78`), intersection reservation, look-ahead braking (`train.rs:388-475`). Missing:
   siding compatibility, empty-wagon repositioning, signal blocks, cargo capacity per wagon.
8. **D-09** The conversation's CTM/LTM meso layer for spillback is correctly identified as the
   cheapest way to get queue propagation without microsimulating every link, but is a major new
   system that nothing in the current code prepares for.
9. **D-18** The "network reserves" table is a powerful unifying concept for the player — every
   network has a reserve metric that tells the Planner how close to the edge they are.
10. **D-20** The conversation missed several critical items: junction deadlock resolution (Egregoria
    has a known gridlock pattern), rail signalling as the real capacity constraint, loading/unloading
    as the true bottleneck, electricity union-find meaning "wire does not exist yet", and weather
    interacting with every network simultaneously.

## 1. Extracted items

| ID | Statement | Source line(s) | Verdict |
|---|---|---|---|
| D-01 | Each network should be modelled by its characteristic physical inertia, not as a generic capacity graph | 703–714 | PLAUSIBLE — sound design principle, no code yet |
| D-02 | Vehicles should use lane-constrained physics with {lane, s, v, a, length, mass, power, traction, braking, cargo, route} | 718–732 | PLAUSIBLE — the substrate has {lane, s, v, a} only |
| D-03 | Rail should model consist mass, length, locomotive power, traction, braking, grade, track occupancy | 774–790 | PARTIAL-ALREADY-EXISTS — mass, length, power, braking exist in prototype and `calculate_locomotive`; grade absent |
| D-04 | Loaded and empty trucks behave differently on slopes; road grade is an industrial logistics variable | 734–736 | PLAUSIBLE — no mass or grade in current vehicle model |
| D-05 | Jerk limits improve polish cheaply | 738 | PLAUSIBLE — simple clamp on da/dt |
| D-06 | Collision should primarily mean avoidance (IDM/MOBIL as references); overlap is a recoverable invariant failure | 742–746 | PLAUSIBLE — current code does avoidance but is NOT IDM/MOBIL |
| D-07 | BPR/Gawron routing cost is useful but traffic should also represent real queues and spillback | 750–754 | PLAUSIBLE — BPR/Gawron are in specs but unimplemented; spillback absent |
| D-08 | Use hybrid meso/micro traffic: meso for network-scale queue propagation, micro where individual movement matters | 758–760 | PLAUSIBLE — nothing exists |
| D-09 | CTM (Daganzo 1994) or LTM (Yperman) for meso spillback | 758 (implicit) | CONFIRMED — these are the standard models for this |
| D-10 | Industrial gates and docks create public-road congestion | 754–755 | PLAUSIBLE — no dock/gate congestion modelled |
| D-11 | Factory shift changes create passenger waves; shift staggering is a planning mechanic | 764–766 | PLAUSIBLE — no shift system exists |
| D-12 | Public transport needs boarding throughput, dwell time, crowding, headways, bunching, power coupling | 770–771 | PLAUSIBLE — no public transport system exists |
| D-13 | Train length affects siding/loading compatibility; empty-wagon repositioning is real traffic | 784–789 | PLAUSIBLE — length exists in prototype but not used for compatibility |
| D-14 | Water: EPANET-like network with nodes, pipes, pumps, tanks, pressure/head, finite demand | 794–813 | PLAUSIBLE — no water substrate exists |
| D-15 | Sewage: gravity, slope, finite conduits, buffers, pumps, treatment, backpressure | 818–828 | PLAUSIBLE — no sewage substrate exists |
| D-16 | Heating: transport delay, thermal mass, coal disruption takes hours before apartments get cold | 830–835 | PLAUSIBLE — no heating substrate exists |
| D-17 | Electricity: min/max output, ramp rates, startup, reserve contribution, priority load shedding | 839–843 | PLAUSIBLE — current is binary blackout only |
| D-18 | Gas pipelines have linepack: pressurized pipeline itself stores gas; delayed service collapse | 847–852 | PLAUSIBLE — no gas substrate exists |
| D-19 | Reservoirs/hydro: mass balance, flow × head × efficiency | 855–859 | PLAUSIBLE — no hydro substrate exists |
| D-20 | Snow/ice changes safe braking/headway and road capacity; snow clearing is real vehicle logistics | 863–865 | PLAUSIBLE — no weather system exists |
| D-21 | Universal "network reserves" table: each system has a reserve metric the Planner manages | 869–880 | PLAUSIBLE — powerful unifying concept |
| D-22 | Phase lag: a good republic keeps working after disruption because of real buffers; fixes propagate with delay | 886–890 | PLAUSIBLE — the design consequence of inertia |
| D-23 | Pump power couples Water and Electricity | 813 | PLAUSIBLE — cross-network coupling |
| D-24 | Compressor stations are strategic energy-consuming infrastructure (gas) | 852 | PLAUSIBLE — cross-network coupling |

## 2. Validation detail

### D-02 — Vehicle state: what the substrate actually has

**Examined:** `simulation/src/transportation/vehicle.rs:26-45,60-105`

The `Vehicle` struct contains:
- `ang_velocity: f32` — angular velocity
- `wait_time: f32` — gridlock wait
- `max_speed_multiplier: f32` — random 0.95–1.05
- `state: VehicleState` — Parked/Driving/Panicking/RoadToPark
- `kind: VehicleKind` — Car/Truck/Bus
- `tint: Color`
- `flag: u64` — gridlock detection

`VehicleKind` provides:
- `width()`: Car 4.5, Truck 6.0, Bus 9.0
- `acceleration()`: Car 3.0, Truck 2.5, Bus 2.0 m/s²
- `deceleration()`: all 6.0 m/s²
- `min_turning_radius()`: Car 0.5, Truck 3.0, Bus 4.0
- `speed_factor()`: Car 1.0, Truck/Bus 0.8
- `ang_acc()`: Car 1.0, Truck 0.9, Bus 0.8

**Missing from the conversation's model:** mass, power, traction, braking (as mass-dependent),
cargo, route (itinerary is external on `VehicleEnt`), grade response, fuel, owner, depot, capacity.

The physics function (`road.rs:141-183`) does:
```
speed += clamp(desired_speed - speed, -DELTA * decel, DELTA * accel)
```
This is a flat kinematic model — no F=ma, no grade, no mass-dependent braking.

**Verdict: PLAUSIBLE** — the conversation correctly identifies what is needed; the substrate has
roughly 4 of the 11 proposed fields.

### D-03 — Rail model assessment

**Examined:** `simulation/src/transportation/train.rs:26-78`, `base_mod/rollingstock.lua`,
`prototypes/src/prototypes/rolling_stock.rs`

The `Locomotive` struct has `max_speed`, `acc_force`, `dec_force`, `length` — all derived from
rolling stock prototypes via `calculate_locomotive()` which sums forces and divides by total mass.

Prototype data (`rollingstock.lua`):
- Locomotive: length 16.75m, mass 60t, max_speed 200 m/s(!), acc_force 2000 kN, dec_force 360 kN
- Freight wagon: length 16.75m, mass 80t, max_speed 160 m/s, acc_force 0, dec_force 480 kN

**Note:** max_speed of 200 m/s = 720 km/h is unrealistic for any locomotive. These are placeholder
values. The `calculate_locomotive` function folds to take the minimum max_speed across all rolling
stock, so a freight consist would be capped at 160 m/s (576 km/h) — still wildly high.

The train system has:
- Intersection reservation system (`TrainReservations`, `train.rs:19-24`)
- Forward look-ahead for braking distance (`train.rs:388-475`)
- Track occupancy checking (`train.rs:260-308`)
- A 60-second stuck-timer that creeps trains forward (`train.rs:379-383`)

**Missing:** grade effects, siding compatibility checks, wagon type ↔ cargo compatibility,
loading/unloading as capacity constraint, signal blocks (occupancy-based only), yard processing,
empty-wagon repositioning. No wagon cargo capacity field exists.

**Verdict: PARTIAL-ALREADY-EXISTS** — rail is the most developed system but still lacks ~half
of what the conversation proposes.

### D-06 — IDM/MOBIL validation

**IDM** (Intelligent Driver Model, Treiber, Hennecke, Helbing 2000): A time-continuous car-following
model. The acceleration equation is:
```
a = a_max * [1 - (v/v0)^δ - (s*(v,Δv)/s)²]
```
where `s*` is the desired gap as a function of speed and speed difference, incorporating safe time
headway `T` and jam distance `s0`. Six interpretable parameters: `v0, a_max, b, T, s0, δ`.

[Source: Wikipedia — Intelligent Driver Model](https://en.wikipedia.org/wiki/Intelligent_driver_model);
[Treiber et al. original](https://arxiv.org/html/2506.05909v1)

**MOBIL** (Minimizing Overall Braking Induced by Lane Changes, Kesting, Treiber, Helbing 2007): A
lane-change decision model with a safety criterion (deceleration threshold for the following
vehicle) and an incentive criterion (net acceleration gain minus politeness-weighted disadvantage
to others). Published in Transportation Research Record 1999, pp. 86-94.

[Source: Kesting et al. 2007](https://journals.sagepub.com/doi/10.3141/1999-10);
[PDF](https://www.mtreiber.de/publications/MOBIL_TRB.pdf)

**Current code comparison (`road.rs:186-407`):**
The `calc_decision` function does:
1. Compute `danger_length = speed² / (2 * decel)` — braking distance
2. Query spatial grid for neighbours within `12 + danger_length`
3. For each neighbour: compute cone angle, distance, ray intersection
4. Return minimum front distance and gridlock flag

This is **not IDM**. It is a geometric cone-based avoidance check. IDM would compute acceleration
from gap, speed, and relative speed using the continuous ODE. The current code returns a binary
stop/go from distance thresholds.

MOBIL is not applicable to the current code at all — there is no lane-change model. Vehicles follow
a fixed itinerary lane sequence.

**Verdict: PLAUSIBLE** — IDM/MOBIL are correct references for a richer model, but the conversation
implies the current code uses them ("IDM/MOBIL-style models are good conceptual references") when
it does not.

### D-07/D-08 — BPR and Gawron

**BPR function** (Bureau of Public Roads, 1964): `t = t_free * (1 + α * (v/c)^β)` with standard
α=0.15, β=4. Already codified in `SPEC-TRAFFIC-007` as
`1 + 0.15 * (load / capacity)^4`.

[Source: BPR 1964](https://medium.com/@yasingoka/importance-of-volume-delay-function-bpr-parameters-shap-analysis-c86b8bf9364b)

**Gawron** (Christian Gawron, 1998, "Simulation-Based Traffic Assignment"): An iterative route
choice method that blends current observed travel times with remembered travel times. The
specification `SPEC-TRAFFIC-008` codifies this as:
`remembered' = 0.3 * observed + 0.7 * remembered`.

[Source: SUMO DUA documentation](https://sumo.dlr.de/docs/Demand/Dynamic_User_Assignment.html);
[Gawron 1998](https://www.semanticscholar.org/paper/Simulation-Based-Traffic-Assignment.-Computing-user-Gawron/01a5a1310f25e265897993ae6b2dc008e9ad5254)

**Current pathfinding (`map/pathfinding.rs:189-268`):**
```rust
cost = l.points.length() / l.speed_limit;
cost += common::rand::randu(l.dist_from_bottom.to_bits() ^ base_random);
```
This is free-flow time plus deterministic noise. No BPR, no load/capacity, no Gawron damping.
No `EMA`, no `load`, no `capacity` field exists on lanes.

**Verdict: PLAUSIBLE** — the specs prescribe BPR/Gawron correctly; nothing is implemented.

### D-09 — CTM and LTM for meso traffic

**CTM** (Daganzo, 1994, "The cell transmission model: a dynamic representation of highway traffic
consistent with the hydrodynamic theory"): Partitions roads into cells, tracks density per cell,
uses supply/demand functions for flow between cells. Captures shockwaves and spillback.

[Source: escholarship.org](https://escholarship.org/uc/item/9pz309w7)

**LTM** (Yperman et al., 2005-2007): An efficiency improvement over CTM that tracks cumulative
vehicle counts at link boundaries (Newell's simplified kinematic wave), avoiding the spatial
discretization errors of CTM while preserving spillback.

[Source: ResearchGate](https://www.researchgate.net/publication/237532918_The_link_transmission_model_An_efficient_implementation_of_the_kinematic_wave_theory_in_traffic_networks)

For a game, CTM is simpler to implement and debug. LTM is more accurate for long links. Either is
appropriate for the meso layer. The conversation does not name either explicitly but describes
"mesoscopic lane flow/cell models for cheap network-scale queue propagation" which matches CTM.

**Verdict: CONFIRMED** — standard models correctly identified for the purpose.

### D-10 — Electricity substrate audit

**Examined:** `simulation/src/map/electricity_cache.rs:6-63,203-279`,
`simulation/src/map_dynamic/electricity.rs:40-92`

The `ElectricityCache` is a union-find over `NetworkObjectID` which can be a `Building`,
`Intersection`, or `Road`. Edges are derived from building→road and road→intersection adjacency
(`map_electricity_edges`, `electricity_cache.rs:244-279`).

The `electricity_flow_system` (`electricity.rs:43-93`) does:
1. For each network, sum consumed/produced power across buildings
2. Houses consume fixed 100W
3. Companies consume/produce based on prototype fields and productivity
4. If consumed > produced, set `blackout = true`

This is:
- **No explicit wire** — connectivity follows road topology
- **Binary blackout** — no brownout, no priority shedding
- **No storage** — no battery/capacitor state
- **No ramp rates** — instant generation
- **No load shedding priority** — all-or-nothing

The spec (`SPEC-ELECTRICITY-001`) requires: "A road, intersection, or building road link MUST NOT
itself be an electrical connection." This directly contradicts the current implementation.

**Verdict: CONFIRMED** — the conversation correctly identifies the need for explicit wire, priority
shedding, and ramp rates. The substrate is road-adjacency union-find, confirmed at
`electricity_cache.rs:244-279`.

### D-14 — Water: is a full GGA needed?

**EPANET** uses the Global Gradient Algorithm (GGA), a Newton-based solver for steady-state
hydraulic equations on a pipe network with mass and energy conservation.

[Source: ScienceDirect](https://www.sciencedirect.com/science/article/abs/pii/S0570644322000211)

For a game, the question is whether a full GGA is needed or a simpler model suffices.

**Answer: A tree-network static head calculation is enough for the game's causal distinctions.**
The conversation wants "connection is not the same as adequate pressure" and "high-rise floors
experience different service." These require:
- Network connectivity (graph reachability from a source)
- Static head at each node: `H = H_source - friction_loss - elevation_gain`
- Pressure at a building = `H - building_elevation`

This is a tree traversal with head-loss summation, not a Newton iteration. A game does not need
the looped-network simultaneous equation solver that GGA provides. Tree networks (source →
branching pipes → endpoints) are solvable in O(n) by topological sort.

For the "floor 9 has no pressure" effect, the model needs: building elevation from terrain +
building height, and a minimum pressure threshold per floor. This is a lookup, not a solver.

**W&R reference:** W&R models water as a piped network with `$CONNECTION_WATERPIPE_INPUT` and
`$CONNECTION_WATERPIPE_OUTPUT` connection points per building, water quality requirements
(`$CONSUMPTION_WATER_REQUIRED_QUALITY 0.55–0.97`), water wells that produce water
(`$PRODUCTION water 215`), and treatment plants that consume sewage and produce water. W&R does NOT
model pressure explicitly — it uses connection-based binary availability.

**Verdict: PLAUSIBLE** — a tree-based head solver is the cheapest model that gives the
"connected but no pressure" distinction. Full GGA is overkill.

### D-15 — Sewage

**SWMM** (EPA Storm Water Management Model) uses kinematic wave or full dynamic wave routing for
gravity-driven sewer networks. It models surcharging, reverse flow, and surface ponding.

[Source: EPA SWMM](https://www.epa.gov/water-research/storm-water-management-model-swmm)

For the game, the conversation wants: gravity flow, backpressure from downstream saturation, finite
conduit capacity, and treatment as a processing step. A simplified model:
- Directed acyclic graph (DAG) from source to treatment plant following gravity
- Per-pipe capacity limit (flow rate)
- Buffer at each junction/treatment plant
- Backpressure when downstream buffer is full → upstream pipes back up

This is a finite-capacity DAG flow with buffers — much simpler than SWMM's PDE solver.

**W&R reference:** W&R has sewage treatment plants (`$TYPE_SEWAGE_TREATMENT`), sewage pumps
(`sewage_pump.ini`), sewage substations (`sewage_substation.ini`), and sewage switches
(`sewage_switch.ini`). Buildings connect via `$CONNECTION_SEWAGE_OUTPUT`. Sewage has a
pollution metric (`$PRODUCTION_SEWAGE_POLLUTION 0.24–0.78`). Treatment produces water with a
quality cap (`$OUTWATER_MAX_QUALITY 0.85`).

**Verdict: PLAUSIBLE** — gravity DAG with buffers is the cheapest adequate model.

### D-16 — Heating transport delay

**Node method** for district heating: tracks transport delay by calculating time for water mass to
move from source node to consumer node. Inlet temperature propagated with pipe heat loss.

[Source: ResearchGate](https://www.researchgate.net/publication/281968104_A_comparative_study_for_simulation_of_heat_transport_in_large_district_heating_network)

For the game:
- Per-pipe FIFO temperature delay line: store (temperature, volume) packets
- Building thermal mass ODE: `dT/dt = (Q_in - Q_loss) / C_thermal`
- When coal supply stops, the pipe FIFO drains over time → delayed cold

**W&R reference:** W&R has heating plants (`$TYPE_HEATING_PLANT`, consuming coal,
producing heat 300 units), heating pumping stations, heating end stations, and
`$HEATING_ENABLE`/`$HEATING_DISABLE` per building. Coal power plants produce `eletric 70` from
`coal 1.2`. The `$ELETRIC_CONSUMPTION_HEATING_WORKER_FACTOR` suggests electric heating as fallback.

**Verdict: PLAUSIBLE** — pipe FIFO + building thermal ODE is the cheapest model that gives transport
delay and thermal mass.

### D-18 — Gas linepack

**Linepack** is the gas stored in a pressurized pipeline. Under isothermal conditions:
`p = a²ρ` where `a² = ZRT/M`. Linepack = total mass of gas in the pipe. When supply drops,
pressure falls gradually as stored gas is consumed.

[Source: OGJ](https://www.ogj.com/pipelines-transportation/article/17240448/lp-model-uses-line-pack-to-optimize-gas-pipeline-operation);
[arXiv](https://arxiv.org/pdf/2001.11496)

For the game: one linepack integrator per pipeline segment:
- State: `{pressure, mass_stored, flow_in, flow_out}`
- Tick: `mass_stored += (flow_in - flow_out) * dt`; `pressure = f(mass_stored)`
- When `pressure < threshold`, service degrades

**Verdict: PLAUSIBLE** — a single ODE per segment gives the delayed collapse effect.

## 3. Deeper mechanics

### 3.1 — Vehicle physics rewrite: cheapest model

The conversation asks for mass-dependent physics. The cheapest model that preserves loaded/empty
distinction on grades:

**State per vehicle:** `{s, v, mass_loaded, mass_empty, power_max, f_brake_max}`
Plus the existing `kind`, `lane`, `width`.

**Tick equation:**
```
grade = terrain_slope_at(lane, s)
F_gravity = mass * g * sin(grade)
F_traction = min(power_max / max(v, 0.1), traction_limit)
F_drag = drag_coeff * v²
F_net = F_traction - F_drag - F_gravity - F_brake (when braking)
a = F_net / mass
v += clamp(a * dt, -jerk_limit * dt, jerk_limit * dt)
```

**Grade data source:** Lane points already have Z coordinates (`PolyLine3`). The grade is
`dz/ds` along the polyline — a free derivative.

**Test:** Loaded truck uphill takes longer than empty truck. Empty truck downhill brakes more.
This is two scenario tests.

**Cost:** One `sin()` + one division per vehicle per tick. Negligible.

### 3.2 — Collision avoidance: IDM integration

Replacing the current cone check with IDM:
```
s* = s0 + v*T + v*Δv/(2*sqrt(a*b))
a_IDM = a_max * [1 - (v/v0)^4 - (s*/s)²]
```

This replaces the `calc_front_dist` function's binary stop/go with a continuous acceleration
response. The vehicle naturally decelerates as it approaches a slower leader.

**Parameters for the game:**
| Parameter | Car | Truck | Bus |
|---|---|---|---|
| v0 (desired speed) | speed_limit | speed_limit * 0.8 | speed_limit * 0.8 |
| a_max (max accel) | 3.0 | 2.5 | 2.0 |
| b (comfortable decel) | 3.0 | 2.0 | 2.0 |
| T (safe headway) | 1.0s | 1.5s | 1.5s |
| s0 (jam distance) | 2.0m | 3.0m | 3.0m |
| δ (free-road exponent) | 4 | 4 | 4 |

MOBIL lane-changing is Post-1.0 (there is no multi-lane road model in the current substrate).

### 3.3 — Traffic: BPR + Gawron on existing substrate

The cheapest addition to existing pathfinding:

1. **Per-lane state:** `{ema_load: f32, capacity: f32, remembered_cost: f32}`
2. **EMA update** (constant time, in `transport_grid_synchronize`):
   Count vehicles on each lane from the transport grid.
   `ema_load = 0.9 * ema_load + 0.1 * current_count`
3. **BPR cost:** `t_bpr = (length/speed_limit) * (1 + 0.15 * (ema_load/capacity)^4)`
4. **Gawron:** `remembered_cost = 0.3 * t_bpr + 0.7 * remembered_cost`
5. **In pathfinding A*:** Replace `cost = length/speed_limit + noise` with
   `cost = remembered_cost + noise`

**Capacity source:** Lane capacity could be `length / (jam_distance + avg_vehicle_length)`.

**Cadence:** EMA update every tick (50 Hz). Gawron damping every tick. Pathfinding queries read
the damped value. Rerouting happens only on topology invalidation or terminal stall, not
on ambient cost changes (per `SPEC-PATHFINDING-006`).

### 3.4 — Electricity: priority load shedding on single island

The cheapest solver that preserves priority ordering:

**State per network:** `{total_generation: Power, demands: Vec<(BuildingID, Power, Priority)>}`

**Algorithm:**
1. Sum generation across all producers in the network
2. Sort demands by priority (hospitals > factories > houses)
3. Serve demands in priority order until generation exhausted
4. Remaining demands get `curtailed` status with binding reason

**State per building:** `{served_power: Power, curtailed: bool, curtailment_reason: CurtailmentReason}`

This replaces the binary `blackout: bool` with per-building served/curtailed. Cost: one sort per
network per tick. With ~100 buildings per network, this is negligible.

### 3.5 — Water: tree-based head solver

**State per node:** `{elevation: f32, head: f32, demand: f32, supplied: f32}`
**State per pipe:** `{capacity: f32, head_loss_per_m: f32, length: f32}`

**Algorithm:**
1. Topological sort from sources
2. For each node in order: `head = parent_head - pipe_head_loss * pipe_length - elevation_diff`
3. If `head < min_pressure_for_floor(building)`: curtailed
4. Finite capacity: flow through pipe capped at pipe capacity

**Player sees:** Building on floor 9 has no water because head is too low. Pump station needed.
Pump power couples to electricity — if power is curtailed, pump stops, water stops.

### 3.6 — Heating: pipe FIFO + thermal ODE

**State per pipe:** `FIFO<(temperature: f32, volume: f32)>`
**State per building:** `{T_interior: f32, C_thermal: f32, Q_demand: f32}`

**Tick:**
1. Source pushes `(T_source, flow_rate * dt)` into pipe FIFO
2. Pipe FIFO pops from the other end when total volume > pipe volume (transport delay)
3. Building receives heat: `Q_in = flow_rate * c_p * (T_supply - T_return)`
4. Building thermal ODE: `T_interior += (Q_in - Q_loss) / C_thermal * dt`
5. `Q_loss = U * A * (T_interior - T_exterior)` (heat loss to environment)

**Player sees:** Coal supply stops → T_source drops → pipe FIFO delivers old hot water for a while
→ building T_interior drops slowly (thermal mass) → eventually apartments are cold. Hours of delay
before visible failure.

### 3.7 — Gas: linepack integrator

**State per segment:** `{mass: f32, pressure: f32, flow_in: f32, flow_out: f32, volume: f32}`

**Tick:**
```
mass += (flow_in - flow_out) * dt
pressure = mass * R * T / (M * volume)  // ideal gas, isothermal
if pressure < service_threshold: curtail downstream consumers
```

**Player sees:** Supply disruption → pressure drops slowly (linepack drains) → delayed collapse.
Compressor stations consume electricity to maintain pressure.

### 3.8 — Meso traffic layer (CTM sketch)

**State per cell (one cell per lane segment):**
`{density: f32, capacity: f32, free_flow_speed: f32}`

**Tick:**
```
demand_i = min(density_i * free_flow_speed_i, capacity_i)
supply_j = capacity_j * (1 - density_j / jam_density_j)
flow_ij = min(demand_i, supply_j)
density_i -= flow_ij * dt / cell_length_i
density_j += flow_ij * dt / cell_length_j
```

When `supply_j = 0` (downstream jammed), `flow_ij = 0` → queue propagates upstream. This is
Daganzo's CTM. One multiply-add per cell per tick.

Freight vehicles that need identity tracking are promoted to the micro layer — they have exact
position and itinerary. Background traffic stays in the meso layer.

## 4. Missed / not apparent

### 4.1 — Junction deadlock (Egregoria's known gridlock)
The current code has a gridlock detector (`road.rs:217-225`): when speed < 0.2 and front_dist < 1.5,
the vehicle enters `Panicking` state and waits up to 200 seconds. The `flag` field propagates a
gridlock token through following vehicles. But the recovery is a random wait
(`wait_time = fract(pos.x * 1000) * 0.5`) — this is a **random perturbation**, not a resolution.
Two vehicles facing each other at a junction with no room to pass will deadlock permanently.
The conversation does not address junction conflict resolution.

### 4.2 — Rail signalling as the true capacity constraint
The current train model uses intersection reservation (`TrainReservations`), not signal blocks.
This means two trains can occupy the same lane segment simultaneously (only intersection conflicts
are prevented). Real rail capacity is constrained by signal block length + braking distance. The
conversation mentions "braking/headway/junction conflicts determine capacity" but does not design
a signal block system.

### 4.3 — Loading/unloading as the real bottleneck
The conversation focuses on line-haul (movement between points) but the real bottleneck in Soviet
logistics was loading and unloading. `SPEC-LOGISTICS-011` correctly identifies "finite loading and
unloading rate budgets" and "dock power" but the conversation does not mention this. The current
code has no loading/unloading time — freight station cargo is a counter
(`freight_station.rs:139`).

### 4.4 — Wagon type ↔ cargo compatibility
The `RailWagon` struct has `kind: RailWagonKind` (Locomotive/Passenger/Freight) and
`rolling_stock: RollingStockID`, but no cargo type, capacity, or compatibility field. A freight
wagon should only carry compatible cargo types. The conversation mentions this briefly but does
not design the compatibility system.

### 4.5 — Road wear from axle load
Soviet roads were famously poor, and heavy truck traffic destroyed them quickly. The conversation
does not mention road degradation as a function of axle load, which would create a maintenance
planning dimension. This is probably Post-1.0 but worth noting.

### 4.6 — The electricity union-find means "wire does not exist yet"
This is critical and easy to miss: the current `ElectricityCache` uses road/intersection adjacency
for electrical connectivity (`electricity_cache.rs:244-279`). Every building connected to a road is
automatically on the electrical grid. There is no wire object. The spec (`SPEC-ELECTRICITY-001`)
explicitly requires wire topology separate from roads. This is not an incremental improvement — it
is a full replacement of the connectivity model.

### 4.7 — Freeze-up of water mains
In Soviet winter conditions, poorly insulated water mains freeze. This couples the water network
to weather (temperature) and heating (insulation). The conversation does not mention this. It would
create an interesting planning challenge: bury pipes deeper or insulate them, or lose water service
in winter.

### 4.8 — Coal quality (calorific value) as a heat variable
Not all coal is equal. Lignite has ~half the calorific value of anthracite. The conversation does
not distinguish coal grades, which would affect both heating and electricity output per ton.
W&R treats coal as uniform.

### 4.9 — Weather interacting with every network simultaneously
The conversation mentions snow affecting roads but does not design how weather interacts with all
networks at once:
- Roads: reduced speed/capacity, snow clearing logistics
- Water: freeze risk, increased heating demand
- Heating: increased demand, transport losses in cold pipes
- Electricity: peak demand in winter, solar reduction
- Gas: increased demand
- Sewage: snowmelt infiltration
- Rail: frozen switches, reduced braking

A single weather state drives correlated stress across every network. This is a powerful
planning challenge but requires a unified weather system.

### 4.10 — Lane-changing determinism
The conversation mentions MOBIL for lane changes but does not address the determinism requirement.
Lane-change decisions in a parallel simulation must be deterministic. MOBIL's incentive criterion
depends on other vehicles' projected accelerations, creating ordering dependencies. The current
substrate has no multi-lane roads, so this is future work, but the determinism constraint must be
designed in from the start.

### 4.11 — Rolling stock speed values are placeholder
The locomotive's max_speed of 200 m/s (720 km/h) and the EMU's 360 m/s (1296 km/h) are clearly
placeholder values. A realistic Soviet locomotive would be ~30 m/s (108 km/h) for freight and
~44 m/s (160 km/h) for passenger. The conversation does not flag this.

### 4.12 — The 60-second stuck train creep
`train.rs:379-383`: when a train has waited for > 60 seconds, it creeps forward at 0.1 * DELTA
per tick, moving its past_travers distances. This is a deadlock-breaker hack, not a proper
resolution. It would cause a train to phase through a blocked junction.

## 5. Cross-lane hooks

| Hook | Target lane | What they must know |
|---|---|---|
| Electricity priority shedding affects factories → production shortfall → goods shortage | Lane A (Economy) | Electricity curtailment reduces productivity; this flows into the Kornai shortage model |
| Pump power couples water/sewage to electricity | Lane A (Economy) | Water/sewage failure from power shortage is a cross-network cascade |
| Shift changes create passenger waves | Lane B1 (Society) | Worker shift schedules drive peak transit demand |
| Snow clearing is real vehicle logistics | Lane C1 (Crates) | Snow plows are vehicles that consume fuel and need maintenance |
| Transport delay in heating affects citizen comfort/needs | Lane B1 (Society) | Heating shortage → cold homes → need satisfaction failure |
| Weather affects every network simultaneously | All lanes | A unified weather system must be designed as a shared authority |
| Loading/unloading rate-limited docks | Lane A (Economy) | Dock throughput limits production throughput; power coupling |
| Road congestion from industrial traffic | Lane A (Economy) | Factory gate traffic spills onto public roads |
| Vehicle fleet is finite → logistics queue | Lane A (Economy) | Truck shortage is a Kornai scarcity signal |

## 6. Open questions for the user

1. **Grade physics priority:** Is loaded-vs-empty-on-grades a 1.0 feature or Post-1.0? The vehicle
   physics rewrite is a good time to add it, but it requires terrain slope data per lane segment.

2. **IDM vs current avoidance:** Is replacing the cone-check with IDM worth the behavioural change
   risk, or should the current model be improved incrementally?

3. **Meso traffic layer scope:** The CTM meso layer is the biggest new system implied by the
   conversation. Is it 1.0, or should BPR/Gawron on the existing micro model be the first target?

4. **Gas network:** Gas is not in the charter scope. The conversation describes it as "compelling
   because of linepack." Is it in scope for further design work?

5. **Rolling stock realism:** Should the placeholder max_speed values (200 m/s for locomotive) be
   corrected now, or wait until rail gets fuller treatment?

6. **Weather system scope:** Weather affects every network simultaneously. Is a unified weather
   authority in 1.0 scope, or is static heating demand the initial target (per `SPEC-HEATING-003`)?

## 7. Sources

### Academic papers
- Treiber, M., Hennecke, A., Helbing, D. (2000). "Congested Traffic States in Empirical Observations and Microscopic Simulations." [arXiv review](https://arxiv.org/html/2506.05909v1)
- Kesting, A., Treiber, M., Helbing, D. (2007). "General Lane-Changing Model MOBIL for Car-Following Models." Transportation Research Record 1999, 86-94. [PDF](https://www.mtreiber.de/publications/MOBIL_TRB.pdf)
- Bureau of Public Roads (1964). Traffic Assignment Manual. US Dept. of Commerce.
- Gawron, C. (1998). "Simulation-Based Traffic Assignment." [Semantic Scholar](https://www.semanticscholar.org/paper/Simulation-Based-Traffic-Assignment.-Computing-user-Gawron/01a5a1310f25e265897993ae6b2dc008e9ad5254)
- Daganzo, C. (1994). "The cell transmission model: a dynamic representation of highway traffic." [eScholarship](https://escholarship.org/uc/item/9pz309w7)
- Yperman, I. (2007). "The link transmission model for dynamic network loading." [ResearchGate](https://www.researchgate.net/publication/237532918_The_link_transmission_model_An_efficient_implementation_of_the_kinematic_wave_theory_in_traffic_networks)
- Rossman, L. (2000). "EPANET 2 Users Manual." EPA/600/R-00/057.
- EPA SWMM. [EPA](https://www.epa.gov/water-research/storm-water-management-model-swmm)

### Codebase files examined
- `simulation/src/transportation/vehicle.rs:26-45,60-105` — Vehicle struct and VehicleKind
- `simulation/src/transportation/road.rs:40-80,141-183,186-407` — Vehicle decision and physics
- `simulation/src/transportation/train.rs:19-78,120-199,230-386,388-475` — Train system
- `simulation/src/transportation/pedestrian.rs:1-122` — Pedestrian physics
- `simulation/src/transportation/mod.rs:1-106` — TransportGrid and TransportState
- `simulation/src/map/pathfinding.rs:189-268` — A* pathfinding (CarPath)
- `simulation/src/map/traffic_control.rs:1-76` — Traffic light schedule
- `simulation/src/map/electricity_cache.rs:6-63,203-279` — Union-find electricity
- `simulation/src/map_dynamic/electricity.rs:40-93` — Binary blackout flow
- `simulation/src/map_dynamic/dispatch.rs:27-39,82-164,246-313` — Dispatcher
- `simulation/src/map_dynamic/router.rs:1-391` — Human routing steps
- `simulation/src/map_dynamic/itinerary.rs:170-198` — Route advance
- `base_mod/rollingstock.lua:1-80` — Rolling stock prototype data
- `prototypes/src/prototypes/rolling_stock.rs:1-54` — Rolling stock prototype struct
- `docs/reference/architecture/substrate.md` — Substrate classifications
- `docs/reference/specifications/traffic.md` — Traffic spec (UNIMPLEMENTED)
- `docs/reference/specifications/vehicles.md` — Vehicles spec (UNIMPLEMENTED)
- `docs/reference/specifications/electricity.md` — Electricity spec (UNIMPLEMENTED)
- `docs/reference/specifications/water.md` — Water spec (UNIMPLEMENTED)
- `docs/reference/specifications/sewage.md` — Sewage spec (UNIMPLEMENTED)
- `docs/reference/specifications/heating.md` — Heating spec (UNIMPLEMENTED)
- `docs/reference/specifications/waste.md` — Waste spec (UNIMPLEMENTED)
- `docs/reference/specifications/roads.md` — Roads spec (UNIMPLEMENTED)
- `docs/reference/specifications/pathfinding.md` — Pathfinding spec (UNIMPLEMENTED)
- `docs/reference/specifications/logistics.md` — Logistics spec (UNIMPLEMENTED)
- `docs/research/fact-sheets/wave1-logistics.md` — Logistics substrate fact-sheet

### W&R reference install
- `~/.local/share/Steam/steamapps/common/SovietRepublic/media_soviet/buildings_types/` — 1,472 building type files
- Examined: `heating_plant_small.ini`, `powerplant_coal.ini`, `water_well_big.ini`, `sewage_treatment_small.ini`, `alumina_plant.ini`
- Key W&R utility keywords: `$TYPE_HEATING_PLANT`, `$TYPE_POWERPLANT`, `$TYPE_MINE_WATER`, `$TYPE_SEWAGE_TREATMENT`, `$HEATING_ENABLE/DISABLE`, `$CONNECTION_WATERPIPE_INPUT/OUTPUT`, `$CONNECTION_SEWAGE_OUTPUT`, `$CONSUMPTION_PER_SECOND eletric`, `$CONSUMPTION water`, `$CONSUMPTION_WATER_REQUIRED_QUALITY`, `$PRODUCTION_SEWAGE_POLLUTION`, `$OUTWATER_MAX_QUALITY`, `$ELETRIC_CONSUMPTION_HEATING_WORKER_FACTOR`
