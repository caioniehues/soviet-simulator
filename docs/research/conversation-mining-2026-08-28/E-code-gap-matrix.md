# Lane E — Claim-by-claim code gap audit

## 0. Summary (ten most important findings)

1. **E-007**: The dishonest enterprise is now wired end-to-end — `request_multiplier` in `recipe_init` calls `set_requested`, so enterprises over-request in the running game. Two companies use it today (flour mill: 4x, butcher: 3x). **But the Planner still cannot see it** — no UI exposes requested-vs-consumed. STATUS: **PARTIAL**.
2. **E-023**: Export sell side is still a teleport — `market.rs:774` debits seller capital immediately at match time, goods vanish without a truck driving them. Import buy side was fixed (sov-abs): imports now go through dispatch. STATUS: **CONTRADICTED**.
3. **E-035**: Domestic money still gates actions — `Government.money` is debited by road/building/train commands (`world_command.rs:225`, `economy/mod.rs:54`). Workers cost money per minute. This contradicts "no domestic money". STATUS: **CONTRADICTED**.
4. **E-041**: No household entity exists. Citizens are individual `HumanEnt` with `Home` (just a `BuildingID`). No shared-pantry model, no household identity, no family structure. STATUS: **ABSENT**.
5. **E-049**: No needs system exists. Citizens have exactly one desire: `BuyFood` (bread only). No housing quality, no healthcare, no heating, no education, no leisure needs. STATUS: **ABSENT**.
6. **E-063**: No utilities exist beyond electricity. No water, sewage, heating, gas. The electricity model is a union-find on road adjacency (not laid wire), with a binary blackout when consumed > produced. STATUS: **PARTIAL** (electricity only).
7. **E-072**: No citizen lifecycle. `PersonalInfo` has name, age, gender — but age never increments, no birth/death/education/migration/qualification. Citizens are spawned with random age 20–50 and persist unchanged. STATUS: **ABSENT**.
8. **E-105**: The UI shows none of the STATUS/CAUSE/TREND/POLICY inspector model. Building inspector shows storage, workers, productivity, power, progress. Human inspector shows location, destination, house, work, last-ate. No causal chain, no trend, no root-cause display. STATUS: **ABSENT**.
9. **E-088**: Determinism test is serialization round-trip hash comparison, not repeat-run determinism. `TestCtx::check_determinism()` encodes→decodes and compares hashes of the result — proves save-load stability, not "same inputs → same state". STATUS: **PARTIAL**.
10. **E-003**: The resource catalogue has 21 items (including job-opening), not the charter's 15 physical resources. Items have identity + `optout_exttrade` only — no mass, volume, storage class, transport class, or unit. STATUS: **PARTIAL**.

---

## 1. Extracted items

### Pass 1: Economy control loop (export lines 85–318)

| ID | Statement | Source line(s) | Status | Evidence | Wired? |
|---|---|---|---|---|---|
| E-001 | Causal distinctness rule: split a resource only when it changes routing/storage/substitution/bottlenecks/allocation/quality/timing/consequences | 109–110 | ABSENT | No resource metadata beyond identity + optout_exttrade; `prototypes/src/prototypes/item.rs:6-25`, `base_mod/items.lua` | N/A (design rule, not code) |
| E-002 | Planning deforms the physical systems themselves, not a UI overlay | 123–127 | PARTIAL | `request_multiplier` exists and feeds `recipe_init` (`goods_company.rs:23`), so enterprises over-request. But no quota/allocation/priority system exists. | Yes (over-request works) |
| E-003 | Resources: Lua declares 21 items; metadata is identity + optout_exttrade only | items.lua | PARTIAL | `base_mod/items.lua:1-108` — 21 items counted. No mass, volume, storage class, transport class. Charter names 15 physical resources; current 21 includes job-opening and several not in charter (gold, high-tech-product, flower, wool, cloth, polyester). | Yes |
| E-004 | Plan → quotas → enterprises → request/produce/hoard → logistics → actual flows → observed results → reports → Plan (closed loop) | 145–163 | PARTIAL | The request/produce portion works (E-002). No quota system, no plan periods, no reporting, no observable results loop beyond raw storage view. | Partially |
| E-005 | Reported need is not true need — a plant requiring 100t might request 145t | 166–174 | EXISTS | `request_multiplier` on Recipe (`prototypes/src/types/recipe.rs:52`); flour mill requests 4x, butcher 3x (`base_mod/companies.lua:40,582`); `recipe_init` at `goods_company.rs:23` calls `set_requested(soul, item.id, item.amount * request_multiplier)` | Yes |
| E-006 | Self-generated shortage spiral from over-requesting | 178–198 | PARTIAL | The over-request mechanism exists (E-005), but no feedback loop: enterprises don't adapt request_multiplier based on experienced reliability. It's static per recipe. | Yes (static) |
| E-007 | Dishonest enterprise: observable discrepancy, no hidden flag | 48 (section 1) | PARTIAL | No `dishonest` flag exists (confirmed: grep finds nothing). `set_requested` is called from `recipe_init` (`goods_company.rs:24`) — production caller. But the UI shows neither `requested` nor `consumed` nor `surplus`. `inspect_building.rs:244-267` shows only storage capital. The Planner cannot observe the discrepancy. | Partially (sim yes, UI no) |
| E-008 | Priority cannot solve scarcity, only decides where scarcity appears | 209 | ABSENT | No priority system exists in allocation. `make_trades` sorts by distance only (`market.rs:577-591`). No plan priority, no request age. | N/A |
| E-009 | Freight-plan stability: storming bunches demand near period end | 216–228 | ABSENT | No plan periods exist. No storming mechanic. No temporal demand variation. Production is continuous. | No |
| E-010 | Ratchet effect: exceptional performance raises next quota | 251–255 | ABSENT | No quota system. No performance tracking. No ratchet mechanic. | No |
| E-011 | Planning credibility / institutional trust through reliability | 258–274 | ABSENT | No reliability memory, no trust metric, no reporting distortion. | No |
| E-012 | Slack as resilience: less aggressive plan outperforms taut one | 276–277 | ABSENT | No plan tautness concept. | No |
| E-013 | Multiple reserve purposes: operating/safety/enterprise/state/project | 279–285 | ABSENT | Only one stock level per soul per item (`capital` in `SingleMarket`). No reserve categories. | No |
| E-014 | Indicator design: what you measure becomes what enterprises optimize | 293–295 | ABSENT | No measurement/reporting system. | No |
| E-015 | Construction opportunity cost: housing vs heavy industry | 297–299 | ABSENT | Buildings are placed with money cost but no material consumption. Construction is instant (no construction phase). | No |
| E-016 | Material-balance UI: national stock/reserves/in-transit/production/consumption/allocation/demand clickable to physical entities | 308–311 | ABSENT | No material-balance view. Inspector shows per-building storage only. | No |
| E-017 | Every macroeconomic number must resolve into physical or institutional state | 315 | ABSENT | No macroeconomic numbers exist in the game. | No |

### Pass 2: Society and citizens (export lines 320–516)

| ID | Statement | Source line(s) | Status | Evidence | Wired? |
|---|---|---|---|---|---|
| E-018 | Social reproduction loop: Plan → production → goods/housing/services → household life → health/education/time/migration/family → labour force → Plan | 337–352 | ABSENT | No household life, health, education, migration, family formation in code. | No |
| E-019 | Enterprise as miniature welfare state (production + labour + welfare) | 356–373 | ABSENT | Companies have workers and a recipe. No welfare, childcare, canteen, culture, transport, clinic, dormitory. | No |
| E-020 | Social cost of industrialization: 8000-worker plant needs ~20000-person settlement | 377 | ABSENT | No settlement calculation. No household dependency. | No |
| E-021 | Housing shortage → overcrowding → turnover → production shortfall | 381–382 | ABSENT | No housing quality, no overcrowding, no turnover mechanic. | No |
| E-022 | Housing as persistent non-price queue | 387–388 | ABSENT | Houses are placed by the player; humans are assigned a `Home(BuildingID)` on spawn (`souls/human.rs`). No queue. | No |
| E-023 | Mikrorayon completeness: housing plan can succeed while daily life fails | 391–393 | ABSENT | No service coverage, no school/shop/transit/clinic/heating dependency. | No |
| E-024 | Time as citizen resource: sleep/work/commute/shopping/queueing/childcare/domestic/healthcare/household production/leisure | 396–410 | ABSENT | Citizens have `Work` with a time interval (8h-18h) and `BuyFood`. No time budget. | No |
| E-025 | Household plots/dachas as food buffer | 419–421 | ABSENT | No household agriculture. | No |
| E-026 | Citizens adapt: seek substitutes, alternate stores, queues, social contacts, walking routes, household sharing, overcrowding, new employers | 423–424 | ABSENT | Citizen decision is max-score over {Home, Work, BuyFood}. No adaptation. | No |
| E-027 | Informal networks / blat as alternate allocation topology | 427–429 | ABSENT | No social network. | No |
| E-028 | Non-monetary inequality from workplace/housing/geography/service access/contacts/qualifications/privilege | 431–440 | ABSENT | All citizens are identical in access. | No |
| E-029 | Labour shortage: socialist economy faces worker scarcity | 443–446 | PARTIAL | `recipe_should_produce` gates on `workers.len() / n_workers` for productivity (`goods_company.rs:85`). Unfilled jobs reduce output. But no active recruitment, no migration, no housing-as-recruitment. | Yes (productivity gate only) |
| E-030 | Qualifications and life course: birth/school/education/qualification/employment/housing/relocation/children/death | 449–451 | ABSENT | `PersonalInfo` has name, age, gender. Age is static. No lifecycle events. | No |
| E-031 | Persistent CitizenRecord + expensive CitizenBody | 453–454 | ABSENT | Only `HumanEnt` exists — a single struct with all state. No split. | No |
| E-032 | Queues as first-class scarcity objects at different time scales | 456–458 | PARTIAL | `BuyFood` has WaitingForTrade state. But no explicit queue data structure. No clinic/school/housing queue. | No (sim internal only) |
| E-033 | Queue burden: measure human time lost to scarcity | 461–462 | ABSENT | No time-loss metric. | No |
| E-034 | Science cities / closed cities / priority geography | 464–466 | ABSENT | No city types, no priority allocation by geography. | No |
| E-035 | Migration | 469–470 | ABSENT | Citizens don't move between areas. | No |

### Pass 3: Rust architecture (export lines 518–693)

| ID | Statement | Source line(s) | Status | Evidence | Wired? |
|---|---|---|---|---|---|
| E-036 | Three worlds: physical/institutional/planner with module privacy enforcement | 533–545 | ABSENT | UI reads `Simulation` directly (`native_app/src/gui/mod.rs:40-88`). No `PlannerSnapshot`, no information restriction. | No |
| E-037 | Typed IDs: CitizenId/HouseholdId/HaulId/ConsumptionId etc | 548–564 | PARTIAL | Typed IDs exist for entities: `VehicleID`, `TrainID`, `HumanID`, `WagonID`, `FreightStationID`, `CompanyID` (`world.rs:26-33`). But no HouseholdId, HaulId, ConsumptionId, Mass, Volume, Energy, Power typed newtypes for quantities. `Power` and `Money` newtypes do exist in prototypes. | Yes (entity IDs) |
| E-038 | Persistent social identity via stable dense typed indexes | 568–569 | PARTIAL | `HopSlotMap<HumanID, HumanEnt>` is generational slotmap. Dense but not stable across save/load (generational). | Yes |
| E-039 | SoA citizens / CitizenStore | 573–574 | ABSENT | `HumanEnt` is AoS in `HopSlotMap`. | No |
| E-040 | Temporal LOD: nothing relevant changed → don't simulate; scheduled wake-ups | 577–579 | ABSENT | Every human is evaluated every decision tick (controlled by `decision.wait` countdown, 30–80 ticks). No event-driven wake. | No |
| E-041 | No household entity | N/A | ABSENT | Grep for "Household" in simulation/ returns zero results. `Home` is just `{ house: BuildingID, last_score: f32 }` (`desire/home.rs:8-11`). | No |
| E-042 | Deterministic event calendar / timing wheel | 582–584 | ABSENT | No event calendar. Sequential system schedule only. | No |
| E-043 | Semantic LOD: keep aggregated until causal distinction matters | 589–590 | ABSENT | All resources are item-level from start. No aggregation/disaggregation. | N/A |
| E-044 | Fixed resource arrays instead of per-holder hash maps | 593–594 | ABSENT | `BTreeMap<SoulID, i32>` per item in `SingleMarket`. `BTreeMap<ItemID, SingleMarket>` overall. Not arrays. | No |
| E-045 | Integer/fixed-point authoritative state | 597–598 | PARTIAL | Capital is `i32`. But production progress is `f32` (`goods_company.rs:75`), productivity is `f32`, positions are `f32`. | Partially |
| E-046 | Deterministic parallelism: parallel compute → intent buffers → deterministic merge → stable sort → commit | 600–611 | ABSENT | Schedule is fully serial (`SeqSchedule`). `ParCommandBuffer` exists but executes sequentially. | No |
| E-047 | Typed system contexts replacing `&mut Simulation` | 614–615 | ABSENT | Systems take `&mut World, &mut Resources` — broad mutable access. | No |
| E-048 | Keyed randomness: stable keys from seed + domain + entity + event | 618–620 | ABSENT | Global `RandProvider` with sequential state. `common::rand::rand2` uses position-based hashing for some randomness but it's not a domain-keyed system. | No |
| E-049 | Bitset society for cohort queries | 622–624 | ABSENT | No bitsets. Linear iteration over all humans. | No |
| E-050 | Incremental Observatory / Change Journal | 639–648 | ABSENT | No change journal. No derived/incremental layer. | No |
| E-051 | Causal history with parent-cause links | 656–661 | ABSENT | No causal history. | No |
| E-052 | Shadow simulation / Gosplan Computer | 662–665 | ABSENT | No headless branching forecast. | No |
| E-053 | LP/MILP feasibility analysis | 668–670 | ABSENT | No mathematical analysis. | No |
| E-054 | Network architecture: shared topology, distinct solvers per utility | 673–674 | PARTIAL | `ElectricityCache` is a graph-based network model. But only electricity exists; no shared topology abstraction for other utilities. | Partially |
| E-055 | Separate immutable Planner/Render/Audio/Debug snapshots | 677–678 | ABSENT | `Simulation` is accessed directly by presentation. No snapshots published. | No |
| E-056 | Compile-time size assertions for hot structs | 683 | ABSENT | No `static_assertions` or `size_of` checks. | No |
| E-057 | Stable versioned release saves vs fast internal snapshots | 685–686 | PARTIAL | `VERSION` file exists ("0.6.1"). Save format includes version string. But version mismatch only warns; resource decode failure leaves defaults (`lib.rs:404-415`). No schema migration. | Yes (warning only) |
| E-058 | Canonical deterministic hashes and property-based testing | 687 | PARTIAL | `hashes()` method exists (`lib.rs:268-279`), hashes per resource. But no property-based testing framework. | Yes (hashing only) |

### Pass 4: Vehicles, traffic, utilities (export lines 696–891)

| ID | Statement | Source line(s) | Status | Evidence | Wired? |
|---|---|---|---|---|---|
| E-059 | Specialized lane-constrained physics | 718–719 | EXISTS | Vehicles follow lanes with speed/acceleration/deceleration per VehicleKind (`vehicle.rs:60-105`). Trains have mass/force/speed/length physics (`train.rs:25-35`). | Yes |
| E-060 | Vehicle model: lane, position, speed, acceleration, length, mass, power, traction, braking, cargo, route | 720–731 | PARTIAL | `Vehicle` has: ang_velocity, wait_time, max_speed_multiplier, state, kind, tint, flag. VehicleKind provides width, acceleration, deceleration, min_turning_radius, speed_factor. NO cargo, mass, power, traction, length on road vehicles. Trains have mass/force/speed/length. | Yes (motion only) |
| E-061 | Loaded and empty trucks behave differently on slopes | 733 | ABSENT | No cargo on truck, no slope physics for road vehicles. | No |
| E-062 | IDM/MOBIL collision avoidance | 743–744 | PARTIAL | Follow/collision/signal/gridlock exists (`transportation/road.rs:15-78,185-250`). It's custom, not IDM/MOBIL. | Yes |
| E-063 | BPR/Gawron routing cost | 750 | ABSENT | Pathfinding uses lane length/speed + noise. No volume-delay, no BPR, no Gawron (`map/pathfinding.rs:189-268`). | No |
| E-064 | Traffic spillback | 751 | ABSENT | No spillback model. Collision avoidance is local. | No |
| E-065 | Hybrid micro/meso traffic | 756–758 | ABSENT | Purely microscopic. | No |
| E-066 | Factory shift changes create passenger waves | 763 | PARTIAL | Workers have work intervals with offset randomization (`desire/work.rs:32-37`). Shift timing varies. But no explicit shift-change wave mechanic. | Yes (timing exists) |
| E-067 | Public transport: trams/trolleybuses/buses with boarding/dwell/crowding/headways | 769–771 | ABSENT | `VehicleKind::Bus` exists but has no passengers, no boarding, no route, no schedule. Buses are just wider/slower vehicles. | No |
| E-068 | Rail: consist mass, length, locomotive power, traction, braking, grade, track occupancy | 775–786 | PARTIAL | Trains have mass, force, speed, length (`train.rs:25-35`). `calculate_locomotive` sums these from wagons (`train.rs:58-78`). `TrainReservations` tracks intersection occupancy (`train.rs:20-23`). No grade physics for trains. | Yes |
| E-069 | Empty wagon repositioning as real traffic | 788 | ABSENT | Trains return empty to external station after unloading (`freight_station.rs:109-113`). The return trip IS traffic. But no explicit empty-wagon logistics model. | Partially |
| E-070 | Rail yards as logistics processors | 790 | ABSENT | No yard model. | No |
| E-071 | Water: EPANET-like network with nodes/pipes/pumps/tanks/reservoirs/pressure/demand/quality | 796–808 | ABSENT | No water system at all. | No |
| E-072 | Sewage: gravity/slope/conduits/buffers/pumps/treatment/backpressure | 819–825 | ABSENT | No sewage system. | No |
| E-073 | Heating: transport delay, thermal mass, pipe network, building storage | 830–835 | ABSENT | No heating system. | No |
| E-074 | Electricity: generators with min/max output, ramp rates, startup, reserve; priority load shedding | 839–843 | PARTIAL | `ElectricityFlow` has produced_power, consumed_power, binary blackout (`map_dynamic/electricity.rs:10-93`). `ElectricityCache` provides union-find network model (`map/electricity_cache.rs`). No generator ramp, no load shedding, no reserve. Blackout is immediate binary: consumed > produced. | Yes (binary blackout only) |
| E-075 | Gas: linepack, compressor stations | 847–852 | ABSENT | No gas system. | No |
| E-076 | Reservoirs/hydro: basin/reach/reservoir mass balance | 855–858 | ABSENT | No hydro system. | No |
| E-077 | Weather: snow/ice changing road capacity, snow clearing as vehicle logistics | 862–864 | ABSENT | No weather system. | No |
| E-078 | Network reserves concept (per utility type) | 867–880 | ABSENT | No reserve tracking for any network. | No |
| E-079 | Physical momentum / phase lag | 884–890 | ABSENT | No buffer/delay model. Electricity blackout is instant; no other networks exist. | No |

### Pass 5: CIA society research (export lines 897–1170) and Closing thesis (1175–1208)

| ID | Statement | Source line(s) | Status | Evidence | Wired? |
|---|---|---|---|---|---|
| E-080 | Consumer scarcity as daily-life management, not abstract need score | 919 | PARTIAL | `BuyFood` models bread-seeking with physical walk to seller. `last_ate` tracks time since eating. But only bread exists as a need. | Yes (bread only) |
| E-081 | Large shopping/search-time burdens | 920 | ABSENT | Humans walk directly to matched seller building. No search, no queueing time, no multi-store search. | No |
| E-082 | High female labor participation with disproportionate household workload | 921 | ABSENT | Gender exists as a field (`PersonalInfo`) but has no gameplay effect. | No |
| E-083 | Childcare tied to workplaces/residential districts | 922 | ABSENT | No childcare. | No |
| E-084 | Housing used to attract workers and reduce turnover | 923 | ABSENT | Workers are assigned to jobs via market match; housing has no effect on recruitment. | No |
| E-085 | Priority projects using housing/services as recruitment privileges | 924 | ABSENT | No priority system. | No |
| E-086 | Private household plots as food buffer | 927 | ABSENT | No private agriculture. | No |
| E-087 | Second economy / informal access | 929 | ABSENT | No informal economy. | No |
| E-088 | Determinism testing: repeat-run determinism | section 1:41 | PARTIAL | `TestCtx::check_determinism()` at `tests/mod.rs:106-121`: serializes current state, deserializes, compares hashes. This proves round-trip stability. Does NOT prove: same initial conditions + same commands → same final state (true repeat-run determinism). | Yes (round-trip only) |
| E-089 | Save format: version stored, mismatch only warns | section 1:34 | EXISTS | `SimulationSer` includes `version: String` (`lib.rs:376`). Deserialization warns on major mismatch, proceeds anyway (`lib.rs:404-415`). VERSION is "0.6.1". Resource decode failures silently leave defaults. | Yes |
| E-090 | Citizen model: split CitizenRecord / CitizenBody | 453–454 | ABSENT | Only `HumanEnt` — single entity. No record/body split. | No |
| E-091 | Time poverty: retail supply failure → longer search → queues → household time burden → fatigue → late arrivals → labor performance | 956–970 | ABSENT | No time tracking, no fatigue, no search time, no performance impact from personal state. | No |
| E-092 | Household scheduling: work/shopping/childcare/domestic/allotment/health/leisure | 975–976 | ABSENT | Human decisions are {Home, Work, BuyFood} max-score, not a schedule. | No |
| E-093 | Childcare as labor-supply transformer | 978–979 | ABSENT | No childcare. | No |
| E-094 | Housing as labor routing (enterprises compete via housing/dormitories/transit/childcare/canteens/cultural facilities) | 983–984 | ABSENT | No enterprise welfare. | No |
| E-095 | Long-term fertility/labor consequences | 987–990 | ABSENT | No reproduction, no demographic model. | No |
| E-096 | Informal economy as alternate allocation topology; scarce goods from real stock | 997–1000 | ABSENT | No informal economy. | No |
| E-097 | Sparse social graph / reciprocity | 1002–1006 | ABSENT | No social graph. | No |
| E-098 | Access privilege / non-monetary inequality as actual access channels | 1008–1009 | ABSENT | No privilege system. | No |
| E-099 | Education → qualification → assignment → relocation | 1012–1013 | ABSENT | No education system. | No |
| E-100 | Labor adaptation costs: replacement workers need time to become productive | 1015–1016 | ABSENT | All workers contribute equally from tick 1. | No |
| E-101 | Labor hoarding by enterprises | 1020–1021 | ABSENT | No enterprise labor strategy. | No |
| E-102 | Storming: overtime, sleep loss, fatigue, absenteeism, quality, family time, turnover | 1024–1026 | ABSENT | No storming mechanic. | No |
| E-103 | Health as production of future capacity | 1028–1030 | ABSENT | No health system. | No |
| E-104 | Four realities: actual physical / reported institutional / planner knowledge / household lived | 1049–1058 | ABSENT | One reality: `Simulation` state, read directly by UI. | No |
| E-105 | STATUS / CAUSE / TREND / POLICY / PHYSICAL CHAIN inspector | 46 (section 1) | ABSENT | Building inspector: `inspect_building.rs:150-267` shows workers, productivity, power, progress, storage, recipe. Human inspector: `inspect_human.rs:17-80` shows location, destination, house, last-ate, work. No causal chain, no trend, no policy display. | No |
| E-106 | Citizen knowledge: citizens don't know every shop's inventory; act from local/social info | 1062–1063 | ABSENT | `BuyFood` places a market buy order and is matched globally by distance. Citizen has perfect knowledge of matched seller location. | No |
| E-107 | Delivery → information propagation → crowd → queue → stock depletion (emergent) | 1066–1079 | ABSENT | No information propagation. Market matches instantly. | No |
| E-108 | Cohort expectations: generations learn different expectations | 1082–1083 | ABSENT | No expectation system. | No |
| E-109 | Deterministic phase order: COMMAND→TOPOLOGY→ALLOCATION→DECISION→ROUTING→MOVEMENT→ARRIVAL→PRODUCTION→UTILITIES→ACCOUNTING | 41 (section 1) | ABSENT | Current schedule is registered systems in `init.rs:54-114`: electricity→dispatch→human_decision→company→pedestrian→transport_grid→locomotive→vehicle_decision→vehicle_state→routing_changed→routing_update→itinerary→market→train_reservations→freight_station→random_vehicles→update_map→add_souls. No named phases. | No (flat sequential) |
| E-110 | 250,000 persistent citizen identities at 60fps | 34 (section 1) | ABSENT | Current architecture: `HopSlotMap<HumanID, HumanEnt>`, evaluated every 30-80 ticks via decision.wait. No LOD, no wake-up, no SoA. Cities in testing have ~50-100 humans. | No |
| E-111 | Buildings are planned, physically constructed, not auto-spawned from zones | 29 (section 1) | CONTRADICTED | Road construction auto-generates roadside lots (`map/map.rs:682-720`, MAP-SUB-002). Building placement IS player-authored, but lots are auto-generated. Construction is instant — no construction phase, no material consumption. | Partially |
| E-112 | Public transit culturally/economically dominant; private cars from household mobility needs | 29 (section 1) | ABSENT | `VehicleKind::Bus` exists but has no route/schedule/passengers. `VehicleKind::Car` exists for random traffic. No transit system. | No |
| E-113 | Domestic allocation is non-price-based | 25 (section 1) | EXISTS | `money_delta: Money::ZERO` for domestic trades (`market.rs:586`). Domestic matching sorts by distance, no price. | Yes |
| E-114 | Border roubles exist only for foreign trade/customs | 26 (section 1) | CONTRADICTED | Foreign trade uses `ext_value * qty` for money_delta. But `Government.money` also debits for roads/buildings/trains/worker wages (`government.rs:22-75`, `economy/mod.rs:53-54`). Domestic money IS a gate. | No |
| E-115 | Failure appears as queues, shortages, substitution, cold homes, missed service, going without | 28 (section 1) | PARTIAL | `BuyFood`: unmatched bread order persists until matched or claim expires → "went without" (last_ate not advanced, `buyfood.rs:103-115`). But no queues, no substitution, no cold homes (no heating), no missed service. | Yes (food only) |
| E-116 | Automate execution, not decisions | 32 (section 1) | PARTIAL | Enterprises auto-produce/consume/trade. Player places buildings and roads. No decision automation vs execution distinction in code. | N/A |
| E-117 | The Plan is a sequence of quota periods on one continuous save | 23 (section 1) | ABSENT | No plan periods, no quotas. Continuous time only. | No |

### Additional specific audit items from brief

| ID | Statement | Source line(s) | Status | Evidence | Wired? |
|---|---|---|---|---|---|
| E-118 | Dishonest enterprise: where over-request is decided | brief (1) | EXISTS | `recipe_init` at `goods_company.rs:22-26`: `let qty = item.amount as u32 * recipe.request_multiplier as u32; market.set_requested(soul, item.id, qty); market.buy_until(soul, near, item.id, qty)`. Decided at company creation time, static from then on. | Yes |
| E-119 | Dishonest enterprise: whether a flag exists | brief (1) | ABSENT (correctly) | No dishonest flag. Design principle says Planner infers from observable state. This is correct by design. | N/A |
| E-120 | Dishonest enterprise: what the Planner can observe | brief (1) | ABSENT | UI (`inspect_building.rs:244-267`) shows `capital` per item only. Does NOT show `requested`, `consumed`, `reserved`, `in-transit`, or `surplus`. `Market::requested()` is a public API but nothing in native_app calls it. | No |
| E-121 | Remaining teleport paths in market.rs | brief (2) | CONTRADICTED | IMPORT buy side: FIXED (sov-abs). External buys go through dispatch now (`market.rs:686-693`). EXPORT sell side: `market.rs:774` does `*cap -= qty_sell` at match time — goods vanish instantly from seller. No truck, no physical movement for exports. This is a teleport. | N/A (export teleports) |
| E-122 | Price/money gate in domestic clearing | brief (3) | CONTRADICTED | `Government.money` debited for: building construction (`world_command.rs:225`), worker wages per minute (`economy/mod.rs:53-54`), train spawning, road construction. Money can go negative (no hard gate), but it IS a price-like cost in a supposedly non-price domestic economy. | Yes |
| E-123 | Citizen model: fields a human has | brief (4) | EXISTS | `HumanEnt` (`world.rs:87-105`): Transform, Speed, Location, Pedestrian, collider, Router, Itinerary, HumanDecision, Home(BuildingID), BuyFood(last_ate, state), Bought, `Option&lt;Work&gt;`, PersonalInfo(name, age, gender). | Yes |
| E-124 | Households exist as an entity | brief (4) | ABSENT | No household entity. `Home` is a single `BuildingID` reference. Multiple humans can share a house building but have no shared state. | No |
| E-125 | Needs exist | brief (4) | PARTIAL | Only food (bread): `BuyFood` desire. No other needs — no housing quality, heating, healthcare, education, leisure, clothing. | Yes (bread only) |
| E-126 | Vehicle model: fields | brief (5) | EXISTS | `Vehicle` (`vehicle.rs:34-45`): ang_velocity, wait_time, max_speed_multiplier, state(Parked/Driving/Panicking/RoadToPark), kind(Car/Truck/Bus), tint, flag. Per-kind: width, acceleration, deceleration, min_turning_radius, speed_factor (`vehicle.rs:60-105`). NO: cargo, capacity, owner link, fuel, wear, driver. | Yes |
| E-127 | Trains exist | brief (5) | EXISTS | `TrainEnt` (`world.rs:128-137`): Transform, Speed, Itinerary, Locomotive(max_speed/acc_force/dec_force/length), LocomotiveReservation, ItineraryLeader. `RailWagon` (`train.rs:53-56`): kind(Locomotive/Passenger/Freight), rolling_stock_id. Physics: mass/force/speed/length composed from wagon prototypes (`train.rs:58-78`). | Yes |
| E-128 | Dispatch is a real truck | brief (5) | EXISTS | `Dispatch` in market.rs creates physical truck dispatch: ToSource→Loading→ToDestination→Unloading. Truck is reserved from `Dispatcher`, drives real itinerary. Capital debited at Loading, credited at Unloading. The current scenario corpus contains 43 tests. | Yes |
| E-129 | Utilities beyond electricity | brief (6) | ABSENT | No water, sewage, heating, gas, waste. Only `ElectricityCache` (union-find network) + `ElectricityFlow` (binary blackout). | No |
| E-130 | Electricity union-find | brief (6) | EXISTS | `ElectricityCache` (`map/electricity_cache.rs:53-63`): BTreeMap<NetworkObjectID, ElectricityNetworkID> ids, BTreeMap<ElectricityNetworkID, ElectricityNetwork> networks. Network is road-adjacency based (objects include roads, intersections, buildings). Merge/split on object add/remove. | Yes |
| E-131 | UI: which sim state the inspector shows | brief (7) | EXISTS | Building: title, workers/max_workers progress, driver link, productivity %, recipe inputs/outputs, power consumption/production, network health %, progress %, storage per item. Human: location, destination, house, last_ate, work building/kind. Train: speed, itinerary. Vehicle: kind, state. FreightStation: waiting/wanted cargo, train list with state. | Yes |
| E-132 | UI: STATUS/CAUSE/TREND exists | brief (7) | ABSENT | No STATUS/CAUSE/TREND display anywhere in native_app. | No |
| E-133 | Determinism: how replay hashing works | brief (8) | EXISTS | `Simulation::hashes()` (`lib.rs:268-279`): Bincode-encodes World, then each registered resource; hashes each with `common::hash_u64`. `Replay` (`utils/replay.rs`): records (Tick, WorldCommand) pairs. `SimulationReplayLoader`: replays commands at correct ticks. `TestCtx::check_determinism` compares hashes of original vs serialize→deserialize. | Yes |
| E-134 | Determinism: what it covers | brief (8) | PARTIAL | Hashes cover: world entities (all HopSlotMaps), and all registered resources (Market, EcoStats, Government, Map, Dispatcher, etc — see `init.rs:116-142`). Does NOT prove repeat-run: same seed + same commands → same state; only proves serialize round-trip preserves state. | Yes |
| E-135 | Save format: what is versioned | brief (9) | EXISTS | `SimulationSer` (`lib.rs:375-380`): world (all entities), version string ("0.6.1"), res (map of resource-name → bincode bytes). `CompressedBincode` for disk saves. `JSON` for replay files. VERSION mismatch warns, does not reject. | Yes |
| E-136 | Test corpus: which mechanisms have scenario tests | brief (10) | EXISTS | 43 scenario tests across 7 files: **hoarding.rs** (3 tests: honest vs inflated demand, market cleanup, isolation of over-request), **inflation.rs** (2 tests: recipe_init sets request_multiplier, buy_until respects it), **ledger.rs** (13 tests: dispatch lifecycle, teleport fix, border closure, demolition recovery, return-to-seller, timeout, reservation cleanup), **recipe_provided.rs** (5 tests: production gates, storage cap, recipe act), **retail.rs** (14 tests: retail claim lifecycle, TTL, settlement, expired claims, demolished store, reservation cleanup), **validation.rs** (5 tests: request_multiplier validation, negative/zero rejection), **mod.rs** (1 test: scenario harness smoke). Additionally 2 freight_station tests, dispatch unit tests, and vehicle tests outside scenarios/. | Yes |

---

## 2. Validation detail

### E-005 / E-007 / E-118: Dishonest enterprise — now wired

The fact-sheet (ECO-SUB-005) said `set_requested` had no production caller — this was true at commit `186e08179b`. Per RESUME.md line 75–79, commit `0caee71` landed `sov-lpj` (REQ-PRODUCTION-001), wiring `request_multiplier` into `recipe_init`.

Current code path:
- `goods_company.rs:22-26`: `recipe_init` reads `recipe.request_multiplier`, computes `qty = item.amount * request_multiplier`, calls `market.set_requested(soul, item.id, qty)` and `market.buy_until(soul, near, item.id, qty)`.
- `prototypes/src/types/recipe.rs:52,63`: `request_multiplier: i32`, defaults to 1.
- `base_mod/companies.lua:40`: flour mill has `request_multiplier = 4` (requests 4x its consumption).
- `base_mod/companies.lua:582`: butcher has `request_multiplier = 3`.
- All other companies default to 1 (no inflation).

**What the Planner CAN observe today**: Only `capital` per item via the building inspector. `Market::requested()` is a public method but `native_app/` never calls it. The surplus (requested - consumed) is invisible to the player.

**What the Planner CANNOT observe**: requested, consumed, reserved, in-transit, surplus. The `sov-hoard-panel-mko` story (RESUME.md:81-83) is the natural next step.

### E-023 / E-121: Export teleport path remains

Import side was fixed by sov-abs: external buys now go through the dispatch system (`market.rs:674-693`). But export side at `market.rs:757-784` still debits `capital` at match time (`*cap -= qty_sell` at line 774) and pushes a Trade — goods vanish from the seller's balance without any physical truck movement. The Trade's buyer is the freight station, but no Dispatch is created for it (the loop at `market.rs:700-747` only creates dispatches for `all_trades[dispatch_start..]`, and export surplus trades are pushed after that loop at lines 777-784).

### E-035 / E-114 / E-122: Domestic money contradicts design pillars

- `Government.money` starts at 150,000 bucks (`government.rs:16`).
- Every tick multiple of `TICKS_PER_MINUTE`: `gvt.money -= n_workers * 10 cents` (`economy/mod.rs:53-54`).
- Road construction: distance-based cost (`government.rs:78-82`).
- Building placement: per-type cost (`government.rs:23-75`).
- `world_command.rs:225`: `sim.write::<Government>().money -= cost`.
- Money CAN go negative (no hard gate at `world_command.rs:223-225`).

This is a price system operating on domestic construction and labour, contradicting "clearing by queue, never by price" and "border roubles only for foreign trade."

### E-088: Determinism test methodology

`TestCtx::check_determinism()` at `tests/mod.rs:106-121`:
1. Bincode-encode current `Simulation`
2. Bincode-decode that into a new `Simulation`
3. Compare `hashes()` of original vs decoded

This proves: serialization format is complete (encode→decode is lossless for the hash function).
This does NOT prove: running the same commands from the same initial state produces the same state (true determinism). For that you would need two independent runs with identical inputs and compare their final state.

The test helper IS called every tick in tests via `TestCtx::tick()`, and every 25 ticks via `advance_ticks()`, which gives good regression coverage for serialization stability. But the `f32` physics (vehicle positions, production progress) remain a determinism risk across platforms/compilers.

### E-074: Electricity model limitations

`ElectricityCache` (`map/electricity_cache.rs`):
- Union-find of NetworkObjectID (Building, Intersection, Road)
- Adjacency is road-based: a building connects to the road network, not to laid wires
- `ElectricityFlow` (`map_dynamic/electricity.rs:43-93`): sums consumed_power and produced_power per network
- Blackout: `consumed_power > produced_power` → binary blackout flag
- Houses consume 100W fixed; companies consume/produce per prototype * productivity

Missing vs charter specs: no brownout-before-blackout, no load shedding, no ramp rates, no reserve, no wire infrastructure, no multi-generator dispatch.

---

## 3. Heat map — EXISTS/PARTIAL/ABSENT/CONTRADICTED counts by pass

| Pass | Total | EXISTS | PARTIAL | ABSENT | CONTRADICTED | SPEC-ONLY | DATA-ONLY |
|---|---|---|---|---|---|---|---|
| Pass 1: Economy control loop (E-001 to E-017) | 17 | 1 | 4 | 12 | 0 | 0 | 0 |
| Pass 2: Society/citizens (E-018 to E-035) | 18 | 0 | 2 | 16 | 0 | 0 | 0 |
| Pass 3: Rust architecture (E-036 to E-058) | 23 | 0 | 8 | 15 | 0 | 0 | 0 |
| Pass 4: Vehicles/traffic/utilities (E-059 to E-079) | 21 | 2 | 5 | 14 | 0 | 0 | 0 |
| Pass 5: CIA society + closing (E-080 to E-117) | 38 | 3 | 4 | 28 | 3 | 0 | 0 |
| Specific audit items (E-118 to E-136) | 19 | 10 | 3 | 4 | 2 | 0 | 0 |
| **TOTAL** | **136** | **16** | **26** | **89** | **5** | **0** | **0** |

### Ten cheapest PARTIAL → EXISTS moves

1. **E-007 Planner observes dishonest enterprise** — `Market::requested()` is already public. Wire it into `inspect_building.rs` next to capital display. ~30 lines of UI code.
2. **E-029 Labour shortage gates production** — Already works: `raw_productivity` returns `workers.len() / n_workers`. Make it more visible in the inspector and add a scenario test asserting output drops with fewer workers.
3. **E-066 Shift changes create passenger waves** — Work intervals already have random offsets. Adding a per-company shift parameter and clustering arrivals would make this visible.
4. **E-045 Integer authoritative state** — Capital is already `i32`. Changing `progress: f32` to fixed-point is a targeted change in `goods_company.rs`.
5. **E-058 Canonical hashes** — `hashes()` already exists. Adding a repeat-run determinism test (two fresh Simulations, same seed, same commands, compare hashes) is straightforward.
6. **E-057 Save version warning** — Already warns. Adding a hard reject for major version mismatch is a one-line change.
7. **E-074 Brownout before blackout** — `ElectricityFlow::blackout` is one `>` comparison. Adding a `brownout: bool` for `consumed_power > 0.8 * produced_power` is trivial.
8. **E-069 Empty wagon return** — Trains already return empty to external station. Exposing this as a visible logistics event is cheap.
9. **E-037 Typed quantity newtypes** — `Power` and `Money` already exist as newtypes. Adding `Mass`, `Volume` follows the same pattern.
10. **E-080 Multiple food needs** — `BuyFood` structure already supports any `ItemID`. Adding a second need (meat, already in items.lua) requires duplicating the desire with `ItemID::new("meat")`.

### Five CONTRADICTED rows that must be fixed before building on top

1. **E-121 Export teleport** — `market.rs:774` debits seller capital immediately without a truck. Any logistics/custody work built on the "nothing teleports" pillar will be undermined by this path.
2. **E-114 / E-122 Domestic money** — `Government.money` prices domestic construction and labour. Any non-price clearing or queue-based allocation work will conflict with this gate.
3. **E-111 Auto-lot generation** — Roads auto-create lots (`map/map.rs:682-720`). Any player-planned-only building placement will collide with this.
4. **E-035 Domestic treasury debits workers** — `economy/mod.rs:53-54` charges money per worker per minute. This is a domestic price that makes `Government.money` a binding constraint.
5. **E-006 Static request_multiplier** — Not technically CONTRADICTED (it's PARTIAL), but building any adaptive hoarding/institutional trust mechanic on top of a static multiplier requires making it dynamic first.

---

## 4. Fact-sheet drift

| Fact-sheet claim | Drift observed |
|---|---|
| `substrate.md` line 63 (ECO-SUB-005): "Dishonest-enterprise behavior is test-only ... `set_requested` has no non-test caller" | **STALE.** `recipe_init` at `goods_company.rs:24` is now a production caller. Landed at commit `0caee71` (sov-lpj). |
| `wave1-economy.md` line 61 (ECO-SUB-005): "Production reads `Market.requested`, but `set_requested` has no non-test caller" | **STALE.** Same fix: `goods_company.rs:24` is a production caller. |
| `substrate.md` line 36 (Initialization): "`static mut` registries, preventing safe parallel test initialization" | **STALE.** Fixed 2026-08-26 per CLAUDE.md and memory. `init.rs:168` uses `OnceLock<Registry>`, not `static mut`. |
| `wave1-substrate.md` line 19 (Initialization CONFLICTING): "unsynchronized `static mut` vectors" | **STALE.** Same fix: `init.rs` now uses `std::sync::OnceLock`. |
| `substrate.md` line 64 (ECO-SUB-002): "Imports credit buyers directly" | **PARTIALLY STALE.** Import buy side fixed (sov-abs): imports now go through dispatch. Export sell side still teleports. The substrate.md statement needs to distinguish import vs export paths. |
| `substrate.md` line 63 (ECO-SUB-001): "Unmatched demand can be removed instead of persisting as a shortage queue" | **PARTIALLY STALE.** Human buy orders now survive the external pass (humans are explicitly excluded from ext-trade at `market.rs:670-672`). Company unmatched orders are still consumed by the ext-trade pass, but companies now go through dispatch. The "removed" behavior is more nuanced now. |
| `wave1-logistics.md` line 85-86 (LOG-SUB-008): "Completion releases truck without parking it" | **NEEDS VERIFICATION.** Multiple dispatch-wedge fixes landed (sov-jcl, sov-xyx, sov-abs, sov-dii, sov-6qx per RESUME.md:85-86). The return/park behavior may have been improved. |

---

## 5. Cross-lane hooks

- **Lane A (economy)**: E-005/E-007 (dishonest enterprise is wired but unobservable), E-121 (export teleport), E-122 (domestic money contradiction), E-006 (static request_multiplier). The economy fact-sheet's ECO-SUB-005 is stale.
- **Lane B1 (society)**: E-041 (no household), E-049 (no needs beyond bread), E-030 (no lifecycle), E-031 (no citizen split). Everything in pass 2 is absent.
- **Lane B2 (CIA)**: Same as B1 — E-080 to E-108 are almost entirely absent. The entire citizen adaptation thesis has no code.
- **Lane C1 (crates)**: E-039 (no SoA), E-042 (no timing wheel), E-049 (no bitset), E-050 (no Salsa/incremental). Architecture is stock inherited Egregoria.
- **Lane C2 (architecture)**: E-036 (no three worlds), E-046 (no parallel phases), E-047 (no typed contexts), E-109 (no phase ordering). Architecture is flat sequential.
- **Lane D (physics)**: E-060 (vehicle model partial), E-063 (no BPR/Gawron), E-071-E-076 (no utilities beyond electricity), E-074 (electricity is binary blackout).

---

## 6. Open questions for the user

1. The export sell-side teleport (E-121) — should this be filed as a dispatch-wedge sibling and fixed before further economy work?
2. The domestic money system (E-122) — is this inherited debt to be removed, or is there a planned transition from money to material-cost construction?
3. Is the static `request_multiplier` the intended 1.0 mechanic for dishonest enterprises, or should it become dynamic (adapting to experienced reliability)?
4. The auto-lot generation (E-111) — is this expected to be removed for 1.0, or is it accepted as a simplification?

---

## 7. Sources

### Files read
- `simulation/src/lib.rs` (lines 1-448)
- `simulation/src/init.rs` (full)
- `simulation/src/economy/market.rs` (full, 1100+ lines)
- `simulation/src/economy/mod.rs` (full)
- `simulation/src/economy/government.rs` (full)
- `simulation/src/souls/goods_company.rs` (full)
- `simulation/src/souls/human.rs` (lines 1-200)
- `simulation/src/souls/freight_station.rs` (full)
- `simulation/src/souls/desire/buyfood.rs` (full)
- `simulation/src/souls/desire/home.rs` (full)
- `simulation/src/souls/desire/work.rs` (full)
- `simulation/src/transportation/vehicle.rs` (full)
- `simulation/src/transportation/train.rs` (lines 1-120)
- `simulation/src/map_dynamic/dispatch.rs` (full)
- `simulation/src/map_dynamic/electricity.rs` (full)
- `simulation/src/map/electricity_cache.rs` (lines 1-80)
- `simulation/src/world.rs` (lines 1-150)
- `simulation/src/tests/mod.rs` (full)
- `simulation/src/utils/replay.rs` (full)
- `native_app/src/gui/inspect/inspect_building.rs` (full)
- `native_app/src/gui/inspect/inspect_human.rs` (lines 1-80)
- `base_mod/items.lua` (full)
- `base_mod/companies.lua` (grep for request_multiplier)
- `prototypes/src/types/recipe.rs` (grep for request_multiplier)
- `docs/reference/architecture/substrate.md` (full)
- `docs/research/fact-sheets/wave1-substrate.md` (full)
- `docs/research/fact-sheets/wave1-logistics.md` (full)
- `docs/research/fact-sheets/wave1-economy.md` (full)
- `docs/plan/iterations/RESUME.md` (lines 1-88)
- `/home/caio/Downloads/soviet_simulator_conversation_export.md` (full, 1210 lines)
- `VERSION` file
