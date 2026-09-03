# System authority index

**Kind:** reference
**Authority:** operational — the 1.0 rows restate the draft specification register (binding on ratification); the "Today" column is observational; target-only rows are labelled
**Status:** active
**Owner:** project lead
**Verified-at:** `4e9e930b2a73`
**Last verified:** 2026-08-28

Which module owns which state, who may reference it, and who actually writes it today. A field
with two writers is a defect; this table exists to make duplicated ownership hard to introduce.

| State | Authority (1.0 spec) | Referenced by | Today (code) |
|---|---|---|---|
| Resource catalogue identity | Resources | everyone | `prototypes` (`ItemPrototype`); no Resources module |
| Resource on-hand stock | Resources | Production, Logistics, Needs, Trade | `Market.markets[item].capital` per soul (`economy/market.rs`) — Market writes it |
| Reservation (encumbrance) | Logistics | Resources, Production | `SingleMarket.reserved` — Market writes it |
| Durable demand and unmet outcome | the requester: Needs, Production, Trade | Logistics | `SingleMarket.requested` and buy orders — written by `recipe_init`/`recipe_act` and `BuyFood` via `Market` |
| Allocation, vehicle reservation, pickup, custody, delivery, return | Logistics | Resources, Vehicles, Roads | `Market::make_trades` / `advance_dispatches` (`Dispatch`); `Dispatcher` reserves the truck |
| Haul custody quantity | Logistics | Vehicles, Trade | **none** — no cargo on the vehicle (`LOG-SUB-005`) |
| Vehicle identity, capacity, location, owner/depot, recovery | Vehicles | Logistics, Traffic | `VehicleEnt` in `World`; `Dispatcher` position cache; companies keep truck IDs that global dispatch ignores (`LOG-SUB-006`) |
| Road topology | Roads | Pathfinding, Traffic, Vehicles, utilities | `Map` (`map/map.rs`) via `WorldCommand` |
| Parking-slot reservation | Roads | Vehicles, Logistics | `map_dynamic/parking.rs` (`MAP-SUB-005`) |
| Route request and result | Pathfinding | Vehicles, citizens | `map/pathfinding.rs`, `map_dynamic/router.rs`, `Itinerary` |
| Load, queue, pressure, stall | Traffic | Pathfinding, Planner view | **none durable** — cone-check avoidance and `Panicking` per vehicle (`MAP-SUB-004`) |
| Industrial consumption and production | Production | Resources, Planner view | `souls/goods_company.rs` (`recipe_act`) |
| Dwelling consumption and satisfaction | Needs | Households, Planner view | `souls/desire/buyfood.rs` (`last_ate`) |
| Household membership, residence, pantry | Households | Citizens, Needs | **none** — `Home` is a `BuildingID` on `HumanEnt` |
| Citizen identity and lifecycle | Citizens | Households, Production (labour), services | `HumanEnt.personal_info` (name, age, gender); no lifecycle |
| Labour assignment | Production (spec: workplace binding) | Citizens | `Work` desire + `job-opening` item traded on the market |
| Customs clearance and rouble settlement | Trade | Resources, Logistics | `Market::make_trades` ext-trade block; `Government.money` |
| Treasury | Trade (border only) | — | `Government.money` — **also written by** `world_command.rs` (buildings, roads) and `economy/mod.rs` (wages): pillar violation |
| Electricity topology, allocation, service result | Electricity | Production, Buildings, pumps | `map/electricity_cache.rs` (union-find over roads), `map_dynamic/electricity.rs` (binary `blackout`) |
| Water / Sewage / Heating / Waste topology, transfer, service | that utility | Needs, Buildings, Production | **none** |
| Plan, quota, period, priority class *(target; no spec)* | Plan / institutions | everything | **none** — `Government` holds only `money` |
| Reported requirement, credibility, reserve classes *(target)* | institutions / enterprises | Planner view | **none** beyond `requested` |
| Change events *(target)* | change journal | observatory, indexes, snapshots | **none** |
| Derived balances and discrepancy *(target)* | observatory | Planner snapshot | `EcoStats` trade volumes only |
| Planner-visible view *(target)* | snapshot | UI | UI reads `Simulation` directly |
| Causal facts *(target)* | causality | inspector, notifications | **none** |
| Random outcomes | core/random *(target: keyed)* | all | `RandProvider` global stream; `common::rand` positional hash |
| Save envelope and migrations *(target)* | persistence | all | `SimulationSer` version string; warn on mismatch |
| Frame inputs (multiplayer) | networking server | simulation | `networking/` lockstep; `WorldCommand` has no role filter |

## Rules this table enforces

[Authority standard](../engineering/authority.md): one owner per mutable field; references,
results and intents across seams; every new field names its owner here before review.

## Related

- [Specification register](specifications/README.md) — the source of the 1.0 column
- [Authority boundaries (architecture)](../architecture/authority-boundaries.md)
- [Mechanics index](mechanics-index.md)
- [Current substrate](../architecture/current-substrate.md)
