# Economy and transport documentation review

## Summary

- The domestic dispatch path is physically staged, but the external-import source is not an accountable stock and border money is settled before physical clearance; export still teleports stock.
- Most planned-economy pages are candidly partial or design-only. A few current-substrate claims are wrong: the inflated recipe is `meat-facility`, not `slaughterhouse`, and telemetry records matches rather than receipts or consumption.
- The physical-economy sequence and invariant index overstate universal custody/conservation because `job-opening` intentionally skips the cargo path and bounded dispatch failures drop goods without a declared sink.
- Transport is mostly partial. The rail endpoint can panic after the external station is removed, and the traffic page calls a continuous kinematic integrator binary stop/go.
- All Mechanics-index Markdown links resolve. The rewrite still has stale source line references, a missing current Medicine declaration, and a duplicate `Allocation` H1 warning.

## Findings

### 1. high — Do not mark the import path as physically sourced

**Evidence.** The Mechanics index calls “Import as physical truck” `EXISTS` (`docs/reference/mechanics-index.md:31`), and the logistics page says imports go through a freight station like a domestic trade (`docs/simulation/physical-economy/logistics.md:62-67`). The external-buy branch creates a `Trade` whose seller is a `SoulID::FreightStation` without checking or reserving station stock (`simulation/src/economy/market.rs:629-652`). When the truck reaches `Loading`, `advance_dispatches` simply subtracts the quantity from the seller's `capital` (`simulation/src/economy/market.rs:922-934`), which creates a negative station ledger row rather than consuming a source quantity. `FreightStation` has only `waiting_cargo` and `wanted_cargo` counters (`simulation/src/souls/freight_station.rs:31-36`); the rail system decrements those counters on train loading but never connects them to the Market quantity (`simulation/src/souls/freight_station.rs:94-104`). The import test proves only that the buyer is unchanged on the match tick and that a dispatch exists, then waits for buyer capital (`simulation/src/tests/scenarios/ledger.rs:674-689`); it does not prove a border source balance or conservation. This is a direct gap in the physical-goods rule and in the trade specification's requirement that imported stock appear only after physical clearance (`docs/reference/specifications/trade.md:29-38`).

**Proposed fix.** Downgrade the index and logistics wording to `PARTIALLY IMPLEMENTED`. Add an authoritative border/source cargo balance and make the import dispatch consume that balance only at physical customs clearance; connect the station's rail counters to that balance or remove the counters from the source story. Extend `sov_abs_ext_trade_import_is_physical` to assert source quantity, destination quantity, and the failure path.

### 2. high — Defer border money until physical clearance

**Evidence.** The glossary defines roubles as border-only and settled at physical customs clearance (`docs/reference/glossary.md:32-35`), and the trade contract says order placement, matching, reservation, and route assignment do not settle money (`docs/reference/specifications/trade.md:29-38`). Nevertheless, external imports attach a negative `money_delta` while matching (`simulation/src/economy/market.rs:646-652`), and exports attach a positive delta while also debiting seller capital immediately (`simulation/src/economy/market.rs:732-741`). `market_update` applies every trade's `money_delta` before calling `advance_dispatches` (`simulation/src/economy/mod.rs:84-105,127-136`). Thus an import with no available truck remains in `ToSource` with no capital movement (`simulation/src/economy/market.rs:879-890`) while the government has already paid, and an export receives money even though no export dispatch is created (`simulation/src/economy/market.rs:679-748`). This breaks the payment/custody pairing even if the buyer eventually receives an import.

**Proposed fix.** Keep `Trade.money_delta` pending for external trades. Apply it exactly once from a physical border-clearance transition after the source/destination custody event, and leave it unchanged for stalled, cancelled, or failed orders. Keep domestic trades at `Money::ZERO`.

### 3. high — Account for bounded in-transit losses as a sink

**Evidence.** The conservation row permits only declared sinks but currently says no violation is known (`docs/reference/invariants.md:14-15`), while the logistics page says the dispatch tests prove conservation (`docs/simulation/physical-economy/logistics.md:19-21`). The loading failure branch gives up after `MAX_RETURN_ROUTE_RETRIES`, frees the truck, and removes the dispatch after the seller was already debited (`simulation/src/economy/market.rs:943-971`). The scenario explicitly ends with seller and buyer at zero after five units were produced, calling the result an “honest physical loss” (`simulation/src/tests/scenarios/retail.rs:679-731`). No loss quantity or sink ledger is written. A warning log is not the declared sink required by the documented balance, so the current test suite demonstrates deletion rather than conservation.

**Proposed fix.** Either retain the cargo in a recoverable physical-loss state, or add an authoritative `lost_in_transit` sink and include it in the material-balance and conservation rows. Update the custody and Mechanics-index rows from `EXISTS` to `PARTIALLY IMPLEMENTED` until this accounting exists.

### 4. medium — Scope the universal physical sequence around the labor token

**Evidence.** The physical-economy index says every good passes through all thirteen states and that no step may be skipped (`docs/simulation/physical-economy/index.md:20-42`). The catalogue itself identifies `job-opening` as non-physical (`docs/simulation/physical-economy/resources.md:52-56`), and the Lua declaration still declares it as an `item` with external trading disabled (`base_mod/items.lua:1-7`). The market gives a `job-opening` match an immediate seller debit (`simulation/src/economy/market.rs:599-603`) and excludes it from dispatch creation (`simulation/src/economy/market.rs:657-660`); `market_update` hires the human synchronously from the match (`simulation/src/economy/mod.rs:99-112`). Therefore this item skips reservation, vehicle reservation, travel, loading, custody, and delivery by design. The absolute “every good” and “no step” wording contradicts the code and the page's own non-physical classification.

**Proposed fix.** Change the canonical sequence to “every physical cargo good,” and add an explicit adjacent labor-token path for `job-opening`. Keep the job-opening exception out of cargo conservation rather than silently making it an untracked exception.

### 5. medium — Name the prototype that actually inflates requests

**Evidence.** Three pages say the request multiplier of 3 is on `slaughterhouse` (`docs/simulation/planned-economy/enterprise-behavior.md:97-103`, `docs/simulation/planned-economy/reliability-and-buffering.md:92-97`, `docs/simulation/concepts/reliability.md:99-103`). The `slaughterhouse` declaration has no `request_multiplier` (`base_mod/companies.lua:526-543`), so the prototype parser defaults it to 1 (`prototypes/src/types/recipe.rs:69-75`). The multiplier 3 is on the distinct `meat-facility` declaration (`base_mod/companies.lua:568-584`). This sends readers and future evidence tests to the wrong enterprise and reverses which live recipe is dishonest by default.

**Proposed fix.** Replace `slaughterhouse` with `meat-facility` in all three pages and refresh the cited lines, or intentionally add the multiplier to `slaughterhouse` and document both recipes.

### 6. medium — Do not describe match telemetry as delivery telemetry

**Evidence.** The reports page presents `Received` as physically delivered and `Consumed` as measured production records (`docs/simulation/planned-economy/reports-and-information.md:16-31`), while its current section says those fields are absent (`docs/simulation/planned-economy/reports-and-information.md:66-76`). `EcoStats` stores only exports, imports, and internal-trade histories (`simulation/src/economy/ecostats.rs:48-52`), and `handle_trade` accumulates a trade's quantity and money delta (`simulation/src/economy/ecostats.rs:79-93`). `market_update` calls `EcoStats::advance` on the match slice before advancing any dispatch (`simulation/src/economy/mod.rs:84-91,127-136`). Company `bought`/`sold` records are also appended from the same match loop (`simulation/src/economy/mod.rs:99-112`), while recipe consumption and production mutate the single capital counter with no receipt/consumption event ledger (`simulation/src/souls/goods_company.rs:51-66`). The current data can therefore report matched volume but cannot support the four-realities table's `Received`, `Consumed`, or `Request age` measurements.

**Proposed fix.** Label current `Trade`/`EcoStats` values as matched or allocated volume only. Do not call them received or consumed until delivery and recipe/retail events write separate counters; then make the inspector consume those event records.

### 7. medium — Correct dispatch recovery claims

**Evidence.** The logistics page says the current code releases a completed truck at its final position without parking (`docs/simulation/physical-economy/logistics.md:47-50`). The current completion branch instead reserves a nearby spot, calls `park`, then frees the dispatcher reservation (`simulation/src/economy/market.rs:1109-1151`). The same page says no route or no truck causes indefinite retry (`docs/simulation/physical-economy/logistics.md:69-71`), but the `Loading` outbound route failure is bounded and terminates after 20 attempts (`simulation/src/economy/market.rs:943-971`), as is the return route (`simulation/src/economy/market.rs:1109-1133`). Only the `ToSource` no-truck/no-route path remains an indefinite waiting dispatch (`simulation/src/economy/market.rs:879-890`). The present wording hides the difference between a visible waiting queue and an intentional logged-loss terminal state.

**Proposed fix.** State that completion parks the truck. Narrow indefinite retry to the `ToSource` acquisition path, and document the bounded `Loading`/`Returning` terminal loss separately with the conservation caveat in Finding 3.

### 8. medium — Repair invariant guards and stale status claims

**Evidence.** The invariants index lists a nonexistent `sov_ahw_stranded_tosource_import_reposts_and_resumes_production` guard and says unmatched orders are removed with `mem::take` (`docs/reference/invariants.md:20`). The current implementation uses `buy_orders.extract_if(...).collect()` (`simulation/src/economy/market.rs:629-632`), and no source test with the `sov_ahw` name exists in `simulation/src/tests/scenarios/`. The same table says there is no repeat-run determinism test (`docs/reference/invariants.md:22`), but `test_world_survives_serde` runs two simulations from the same replay and compares them at checkpoints (`simulation/src/tests/test_iso.rs:241-314`). Its no-teleport and conservation rows also list the import test as sufficient even though that test checks only buyer timing/dispatch presence (`simulation/src/tests/scenarios/ledger.rs:674-689`) and the export violation is admitted in the same row (`docs/reference/invariants.md:14`).

**Proposed fix.** Replace the phantom AHW name with the actual scenario names, cite `extract_if` if implementation detail is retained, and mark unmatched demand as partial rather than guarded. Add the repeat-run test as a determinism guard. Mark no-teleport and conservation as partial until export, import source custody, and declared-loss accounting are covered.

### 9. medium — Distinguish a domestic cost from a money gate

**Evidence.** The Mechanics index calls this mechanic “Domestic money gate” (`docs/reference/mechanics-index.md:26`), and physical causality says domestic money “gates” construction, wages, and trains (`docs/simulation/concepts/physical-causality.md:58-65`). The source computes domestic action costs for houses, trains, connections, zones, companies, and stations (`simulation/src/economy/government.rs:21-75`) and subtracts them unconditionally before executing a command (`simulation/src/world_command.rs:223-225`). `Money` explicitly allows negative debt and subtraction has no balance check (`prototypes/src/types/money.rs:13-14,161-164`). The scarcity page correctly says this is not a hard gate but a price-like domestic cost (`docs/simulation/concepts/scarcity.md:69-81`). Calling it a gate is internally inconsistent and obscures that the actual violation is off-border rouble use, not price-based access to matching.

**Proposed fix.** Rename the row and prose to “Domestic treasury debit/cost (must retire)” and keep “no hard gate” explicit. Separate this from the correctly implemented domestic matching rule (`Money::ZERO`, distance ordering in `simulation/src/economy/market.rs:528-551`).

### 10. medium — Record the missing Medicine resource

**Evidence.** The charter requires fifteen domestic resources plus import-only Medicine (`docs/plan/charter-1.0.md:43-48`), and the resources page repeats the exact catalogue requirement (`docs/simulation/physical-economy/resources.md:24-26`). The page's current list contains 21 items but no Medicine (`docs/simulation/physical-economy/resources.md:43-56`); the authoritative Lua list also has no `medicine` entry (`base_mod/items.lua:1-108`), and the item prototype has only name/label/id/`optout_exttrade` (`prototypes/src/prototypes/item.rs:7-14`). The current page notes extra items and `job-opening` but does not identify Medicine as absent, so the 1.0 resource gap is easy to miss.

**Proposed fix.** Add Medicine as an import-only declaration with its eventual handling metadata, or explicitly mark Medicine `ABSENT` in the current-substrate and Mechanics-index evidence until its implementation exists.

### 11. medium — Guard rail endpoint removal instead of panicking

**Evidence.** The transport docs and logistics requirement say a missing endpoint should become a stalled or recoverable job, not termination (`docs/simulation/physical-economy/logistics.md:29-30`; `docs/reference/specifications/trade.md:39-44`). When a freight train finishes arriving, `freight_station_system` unwraps the first external station (`simulation/src/souls/freight_station.rs:94-104`). `Map::remove_building` allows an `ExternalTrading` building to be removed and removes it from `external_train_stations` (`simulation/src/map/map.rs:128-136`). If a train is loading after the last external station is removed, `.first().unwrap()` panics instead of leaving an observable pending state. This is a concrete failure-never-ends violation not reflected in the freight-rail page's current-substrate section (`docs/simulation/transport/freight-rail.md:48-66`).

**Proposed fix.** Handle an empty external-station list as a waiting/failed rail state and keep the train reservation recoverable. Add a scenario that removes the last external station while a freight train is loading.

### 12. low — Describe continuous vehicle integration accurately

**Evidence.** The traffic page says there is “no continuous acceleration response, only a binary stop/go” (`docs/simulation/transport/traffic.md:49-55`). The road physics integrates speed continuously with a clamped acceleration/deceleration step (`simulation/src/transportation/road.rs:149-190`), and `calc_decision` supplies either zero or a desired lane speed to that integrator (`simulation/src/transportation/road.rs:300-318`). The current model is not IDM and does use thresholded stop decisions, but it is not binary vehicle motion.

**Proposed fix.** Say “thresholded stop/desired-speed decisions over a continuous kinematic integrator; no IDM” and retain the accurate missing-BPR/Gawron statement.

### 13. low — Say one full loop, not one implemented link

**Evidence.** The causal-loop catalogue says “Today exactly one link is implemented” and everything else is target/research (`docs/simulation/causal-loops.md:9-11`). The same page immediately documents an implemented storage-capacity floor (`docs/simulation/causal-loops.md:25-28`), and the source enforces that floor in `recipe_should_produce` (`simulation/src/souls/goods_company.rs:31-47`). The wording is contradictory if “link” means any implemented causal edge; only the electricity blackout loop is fully closed, while the storage cap is a partial edge.

**Proposed fix.** Replace the opening with “Today exactly one catalogue loop is fully implemented; several target loops have partial links, including storage capacity.”

### 14. low — Refresh source citations after the rewrite

**Evidence.** Multiple active pages cite line numbers from a different source position: the export teleport is cited at `market.rs:774` (`docs/simulation/concepts/physical-causality.md:76-80`; `docs/simulation/physical-economy/logistics.md:62-65`), but the current debit is at `market.rs:732-741`; domestic `Money::ZERO` is cited at `market.rs:584` (`docs/simulation/concepts/scarcity.md:69-73`; `docs/simulation/planned-economy/allocation.md:70-81`), while the current literal is at `market.rs:543-544`; and `reports-and-information.md` cites `market.rs:500-501` for `Market::requested()` (`docs/simulation/planned-economy/reports-and-information.md:73-75`), while those lines are `produce` and the accessor is at `market.rs:77-78`. The checker reports no broken links, but these stale source anchors make the current-substrate claims hard to audit.

**Proposed fix.** Refresh every line citation after the final commit, preferably using function/symbol anchors plus a `Verified-at` commit. Treat the code line-reference refresh as one documentation pass rather than patching isolated examples.

## Implemented / partial / design-only classification

The classification below describes the mechanic named by each page, not whether every target proposal on that page exists. `PARTIALLY IMPLEMENTED` means a real current subset exists; `DESIGN-ONLY` means the target mechanic itself is absent.

### Cross-cutting simulation pages

| Page | Classification | Evidence |
|---|---|---|
| `docs/simulation/index.md` | Navigation only | It is an index of design/current/research pages (`docs/simulation/index.md:8-23`). |
| `docs/simulation/causal-loops.md` | PARTIALLY IMPLEMENTED | Electricity blackout reaches company productivity (`simulation/src/map_dynamic/electricity.rs:40-95`; `simulation/src/souls/goods_company.rs:100-126`); storage is a partial edge; other loops are labelled target/research (`docs/simulation/causal-loops.md:9-28`). |

### Concepts

| Page | Classification | Evidence |
|---|---|---|
| `concepts/authority.md` | PARTIALLY IMPLEMENTED | Broad mutable systems and deferred callbacks remain (`docs/simulation/concepts/authority.md:53-70`); the code registers broad systems and `Market` owns dispatch transitions (`simulation/src/init.rs:47-95`; `simulation/src/economy/market.rs:759-778`). |
| `concepts/physical-causality.md` | PARTIALLY IMPLEMENTED | Domestic dispatch staging exists, but export debit is match-time and has no dispatch (`docs/simulation/concepts/physical-causality.md:54-65`; `simulation/src/economy/market.rs:679-748`). |
| `concepts/scarcity.md` | PARTIALLY IMPLEMENTED | Internal matches use zero money and distance ordering, but no durable queue/deficit/substitution and domestic treasury costs remain (`docs/simulation/concepts/scarcity.md:69-81`; `simulation/src/economy/market.rs:528-551`). |
| `concepts/queues.md` | PARTIALLY IMPLEMENTED | `BuyFoodState::WaitingForTrade` is a retail wait; there is no general queue and external unmatched orders can be erased (`docs/simulation/concepts/queues.md:54-66`; `simulation/src/souls/desire/buyfood.rs:16-24,80-112`; `simulation/src/economy/market.rs:629-632`). |
| `concepts/reserves.md` | PARTIALLY IMPLEMENTED | `capital`, `reserved`, and `requested` exist but no reserve classes (`docs/simulation/concepts/reserves.md:79-110`; `simulation/src/economy/market.rs:39-53`). |
| `concepts/phase-lag.md` | DESIGN-ONLY | The page's utility-delay model is absent; current electricity is an instantaneous per-tick blackout (`docs/simulation/concepts/phase-lag.md:63-68`; `simulation/src/map_dynamic/electricity.rs:40-95`). |
| `concepts/reliability.md` | PARTIALLY IMPLEMENTED | Static request inflation and storage floor exist, but no reliability memory or adaptive state (`docs/simulation/concepts/reliability.md:99-107`; `simulation/src/souls/goods_company.rs:22-26,69-78`). |
| `concepts/information.md` | DESIGN-ONLY (with raw inspection) | The target four-realities/Planner snapshot does not exist; UI reads `Simulation` directly (`docs/simulation/concepts/information.md:82-95`; `docs/simulation/planned-economy/reports-and-information.md:66-76`). |
| `concepts/adaptation.md` | PARTIALLY IMPLEMENTED | Human decisions and static recipe multipliers exist; no learning or search memory (`docs/simulation/concepts/adaptation.md:76-90`; `simulation/src/souls/desire/buyfood.rs:46-68`; `simulation/src/souls/goods_company.rs:22-26`). |
| `concepts/social-reproduction.md` | DESIGN-ONLY (minimal human loop) | `HumanEnt` has home/work/food but no household, health, education, or demographic chain (`docs/simulation/concepts/social-reproduction.md:83-92`; `simulation/src/world.rs:87-105`). |

### Planned economy

| Page | Classification | Evidence |
|---|---|---|
| `planned-economy/plan-cycle.md` | DESIGN-ONLY (trade/request subset) | No quota, plan period, or reporting cycle; production is continuous and EcoStats is trade-only (`docs/simulation/planned-economy/plan-cycle.md:85-99`; `simulation/src/souls/goods_company.rs:201-205`; `simulation/src/economy/ecostats.rs:48-52`). |
| `planned-economy/material-balance.md` | DESIGN-ONLY (trade telemetry only) | The balance needs opening/closing, production, consumption counters that do not exist (`docs/simulation/planned-economy/material-balance.md:82-96`; `simulation/src/economy/ecostats.rs:79-93`). |
| `planned-economy/enterprise-behavior.md` | PARTIALLY IMPLEMENTED | `request_multiplier` is wired and static, with a storage floor; adaptive state and Planner visibility are absent (`docs/simulation/planned-economy/enterprise-behavior.md:97-111`; `simulation/src/souls/goods_company.rs:22-26,31-47,69-78`). |
| `planned-economy/reports-and-information.md` | DESIGN-ONLY (raw inspector subset) | Inspector shows capital but not request/receipt/consumption/provenance (`docs/simulation/planned-economy/reports-and-information.md:66-76`; `native_app/src/gui/inspect/inspect_building.rs:150-267`). |
| `planned-economy/reserves.md` | PARTIALLY IMPLEMENTED | Single capital/reservation maps and storage floor exist; five reserve classes/confiscation do not (`docs/simulation/planned-economy/reserves.md:94-105`; `simulation/src/economy/market.rs:39-53`). |
| `planned-economy/priorities.md` | DESIGN-ONLY | Current `make_trades` sorts distance, not Planner priority or deficit (`docs/simulation/planned-economy/priorities.md:51-56`; `simulation/src/economy/market.rs:528-551`). |
| `planned-economy/reliability-and-buffering.md` | PARTIALLY IMPLEMENTED | Static multiplier and storage cap exist; reliability, credibility, quota, and ratchet state do not (`docs/simulation/planned-economy/reliability-and-buffering.md:92-105`; `simulation/src/souls/goods_company.rs:22-26,69-78`). |
| `planned-economy/storming.md` | DESIGN-ONLY | Production is continuous with no period/deadline/storming state (`docs/simulation/planned-economy/storming.md:77-84`; `simulation/src/souls/goods_company.rs:192-218`). |
| `planned-economy/allocation.md` | PARTIALLY IMPLEMENTED | Distance-only domestic matching and physical dispatch exist; target-stock/deficit/substitution do not (`docs/simulation/planned-economy/allocation.md:69-81`; `simulation/src/economy/market.rs:528-551,679-748`). |

### Physical economy

| Page | Classification | Evidence |
|---|---|---|
| `physical-economy/index.md` | PARTIALLY IMPLEMENTED | Market dispatch has ToSource/Loading/ToDestination/Unloading, but the full thirteen-state contract and job-opening exception are not represented (`docs/simulation/physical-economy/index.md:20-42`; `simulation/src/economy/market.rs:174-218,759-778`). |
| `physical-economy/resources.md` | PARTIALLY IMPLEMENTED | Lua has 21 simple item declarations, but no units/classes/capacity metadata and no Medicine (`docs/simulation/physical-economy/resources.md:43-56`; `base_mod/items.lua:1-108`; `prototypes/src/prototypes/item.rs:7-14`). |
| `physical-economy/requests.md` | PARTIALLY IMPLEMENTED | `BuyOrder`/`requested` exist, but non-human unmatched orders are extracted and dropped when no external partner exists (`docs/simulation/physical-economy/requests.md:45-57`; `simulation/src/economy/market.rs:629-652`). |
| `physical-economy/allocation.md` | PARTIALLY IMPLEMENTED | Distance matching exists; target-stock deficit and policy ordering are absent (`docs/simulation/physical-economy/allocation.md:35-42`; `simulation/src/economy/market.rs:528-551`). |
| `physical-economy/reservation.md` | PARTIALLY IMPLEMENTED | `reserved` is a real non-additive hold and tests cover cancellation, but there is no typed reservation/custody ledger (`docs/simulation/physical-economy/reservation.md:33-51`; `simulation/src/economy/market.rs:39-50,879-934`). |
| `physical-economy/custody.md` | PARTIALLY IMPLEMENTED | Dispatch stores item/quantity/state and cancellation paths are tested, but Vehicle/RailWagon carry no cargo and losses lack a sink (`docs/simulation/physical-economy/custody.md:40-62`; `simulation/src/economy/market.rs:174-218`; `simulation/src/transportation/vehicle.rs:34-45`; `simulation/src/transportation/train.rs:45-56`). |
| `physical-economy/storage.md` | PARTIALLY IMPLEMENTED | Capital/reserved storage floor is implemented and tested, but five accounting states are absent (`docs/simulation/physical-economy/storage.md:41-59`; `simulation/src/souls/goods_company.rs:31-47`; `simulation/src/tests/scenarios/recipe_provided.rs:148-190`). |
| `physical-economy/production.md` | PARTIALLY IMPLEMENTED | Input/storage gates and recipe transformations exist, but binding-constraint recording, water gate, and run IDs are absent (`docs/simulation/physical-economy/production.md:51-78`; `simulation/src/souls/goods_company.rs:31-78`). |
| `physical-economy/logistics.md` | PARTIALLY IMPLEMENTED | Domestic truck lifecycle and physical endpoint debit/credit exist; border source, export, vehicle cargo, terminal-loss accounting, and rate-limited docks do not (`docs/simulation/physical-economy/logistics.md:51-75`; `simulation/src/economy/market.rs:879-1151`). |
| `physical-economy/construction.md` | DESIGN-ONLY | Placement immediately creates a building and BuildingInfos; no Site, bill, gate, or construction phase (`docs/simulation/physical-economy/construction.md:63-75`; `simulation/src/world_command.rs:223-225,284-299`; `simulation/src/map/map.rs:279-305`). |

### Transport

| Page | Classification | Evidence |
|---|---|---|
| `transport/index.md` | Navigation/target authority map | It lists Route, Movement, Traffic, Vehicle, Haul, and Cargo custody as separate authorities (`docs/simulation/transport/index.md:18-33`), while current code places dispatch/custody in `Market` and vehicle reservation in `Dispatcher` (`simulation/src/economy/market.rs:759-778`; `simulation/src/map_dynamic/dispatch.rs:17-24`). |
| `transport/roads.md` | PARTIALLY IMPLEMENTED | Typed roads and parking reservations exist; auto-lot creation and durable capacity readout conflict with target (`docs/simulation/transport/roads.md:45-59`; `simulation/src/map/map.rs:682-730`; `simulation/src/map_dynamic/parking.rs:24-90`). |
| `transport/pathfinding.md` | PARTIALLY IMPLEMENTED | Flat A* and retry exist, but no BPR/Gawron/load/topology revision (`docs/simulation/transport/pathfinding.md:43-65`; `simulation/src/map/pathfinding.rs:189-268`). |
| `transport/traffic.md` | PARTIALLY IMPLEMENTED | Microscopic cone/gridlock avoidance exists; EWMA/BPR/Gawron and durable load are absent (`docs/simulation/transport/traffic.md:49-64`; `simulation/src/transportation/road.rs:186-407`). |
| `transport/vehicles.md` | PARTIALLY IMPLEMENTED | Vehicle identity, states, and kinematic constants exist; mass, cargo, capacity, owner, depot, and fuel are absent (`docs/simulation/transport/vehicles.md:62-87`; `simulation/src/transportation/vehicle.rs:34-105`). |
| `transport/freight-rail.md` | PARTIALLY IMPLEMENTED | Locomotive consist physics, intersection reservations, and look-ahead braking exist; cargo/capacity/signalling/yards do not, and endpoint removal can panic (`docs/simulation/transport/freight-rail.md:48-66`; `simulation/src/transportation/train.rs:19-77,373-475`; `simulation/src/souls/freight_station.rs:94-104`). |
| `transport/public-transport-future.md` | DESIGN-ONLY | `VehicleKind::Bus` only supplies width/speed constants; there are no passengers, stops, routes, schedules, or boarding (`docs/simulation/transport/public-transport-future.md:34-43`; `simulation/src/transportation/vehicle.rs:25-105`). |

## Glossary versus code

The glossary is authoritative terminology, but it deliberately does not claim implementation (`docs/reference/glossary.md:8-10`). The following audit maps every domain term in the requested glossary page to the targeted code:

| Glossary term(s) | Code status |
|---|---|
| Planner, Plan, Quota, Tranche (`docs/reference/glossary.md:12-31`) | No matching domain identifiers. `Government` only stores `money`; no plan/period/quota/tranche state (`simulation/src/economy/government.rs:7-11`). |
| Rouble (`docs/reference/glossary.md:32-35`) | No `Rouble` identifier. Code uses generic `Money` for government balances, worker upkeep, action costs, and border deltas (`prototypes/src/types/money.rs:13-14`; `simulation/src/economy/government.rs:21-75`; `simulation/src/economy/mod.rs:53-54,103-104`). Semantics currently violate the glossary's border-only rule. |
| Ghost, Verdict, Refusal, Material bill, Site, Ground broken, Rescind (`docs/reference/glossary.md:38-61`) | No matching construction state identifiers. Placement directly builds a `Building` and inserts `BuildingInfos` (`simulation/src/map/objects/building.rs:53-78`; `simulation/src/map/map.rs:279-305`). |
| Binding constraint (`docs/reference/glossary.md:63-67`) | No binding-constraint field or enum; `GoodsCompanyState` has only progress/driver/trucks (`simulation/src/souls/goods_company.rs:69-78`). |
| Request (`docs/reference/glossary.md:69-72`) | No `Request` type. `BuyOrder` and `SingleMarket.requested` are the partial implementation (`simulation/src/economy/market.rs:23-50,453-459`). |
| Custody (`docs/reference/glossary.md:74`) | No `Custody` type. `Dispatch` and `DispatchState` carry a partial surrogate (`simulation/src/economy/market.rs:174-218`). |
| Dispatcher (`docs/reference/glossary.md:76`) | Exact `Dispatcher` identifier and resource exist (`simulation/src/map_dynamic/dispatch.rs:17-24`; `simulation/src/init.rs:141-142`). Its current role is nearest-entity reservation, not a complete haul authority. |
| Border (`docs/reference/glossary.md:78-80`) | No `Border` identifier. `ExternalTrading`, `FreightStation`, and `external_train_stations` are the code concepts (`simulation/src/map/objects/building.rs:17-31`; `simulation/src/souls/freight_station.rs:31-36`; `simulation/src/map/map.rs:35-36`). |
| Going without (`docs/reference/glossary.md:82-84`) | No exact state. Retail uses `BuyFoodState::{Empty,WaitingForTrade,BoughtAt}` and logs expired/demolished outcomes as going without (`simulation/src/souls/desire/buyfood.rs:16-24,80-112`). |
| Storming, Ratchet, Capital dilution (`docs/reference/glossary.md:86-98`) | No matching simulation identifiers or plan-period state; the only economy control state is `Government.money` (`simulation/src/economy/government.rs:7-11`). |
| Mikrorayon, Monotown, Blat, Tolkach, Propiska, Kommunalka (`docs/reference/glossary.md:100-139`) | No matching code concepts. `HumanEnt` has home/work/food and personal info only (`simulation/src/world.rs:87-105`). |
| Policy (`docs/reference/glossary.md:141-144`) | No generic Planner `Policy`. `LightPolicy` and `TurnPolicy` are real transport-control types (`simulation/src/map/light_policy.rs:7-15`; `simulation/src/map/turn_policy.rs:20-32`). |
| Player control (`docs/reference/glossary.md:146-148`) | No `PlayerControl` identifier; pacing is represented by `GameTime`/simulation options rather than a persisted policy (`simulation/src/init.rs:126-142`; `simulation/src/lib.rs:235-255`). |

Important code concepts with no exact glossary entry are: `Market`, `SingleMarket`, `BuyOrder`, `SellOrder`, `Trade`, `RetailClaim`, `DispatchState`, `Dispatch`, `capital`, `reserved`, `requested`, `money_delta`, and `EcoStats` (`simulation/src/economy/market.rs:23-122,174-218`; `simulation/src/economy/ecostats.rs:48-93`); `Recipe`, `RecipeItem`, `GoodsCompanyState`, and `WorkKind` (`prototypes/src/types/recipe.rs:9-75`; `simulation/src/souls/goods_company.rs:69-78`; `simulation/src/souls/desire/work.rs:12-15`); and `BuyFoodState`, `VehicleState`, `VehicleKind`, `Itinerary`, `TrainReservations`, `FreightTrainState`, `RailWagon`, `ParkingManagement`, `ExternalTrading`, and `job-opening` (`simulation/src/souls/desire/buyfood.rs:16-24`; `simulation/src/transportation/vehicle.rs:16-45`; `simulation/src/map_dynamic/itinerary.rs:17-45`; `simulation/src/transportation/train.rs:19-56`; `simulation/src/souls/freight_station.rs:16-36`; `simulation/src/map/objects/building.rs:17-31`; `base_mod/items.lua:1-7`). Some have conceptual glossary equivalents, but the exact identifiers and their ownership are not discoverable from the glossary.

## Invariant guard audit

| Invariant | Guard today | Assessment |
|---|---|---|
| No teleportation | `scenario_0082_dispatch_gates_stock_not_match`; `sov_abs_ext_trade_import_is_physical`; ledger tests (`docs/reference/invariants.md:14`; `simulation/src/tests/scenarios/hoarding.rs:138-186`; `simulation/src/tests/scenarios/ledger.rs:595-689`) | PARTIAL. Domestic timing is guarded; export has a match-time debit and import has no source-cargo assertion. |
| Conservation | `scenario_demolish_buyer_building_end_to_end_conserves`, `scenario_ledger_*`, `scenario_dead_*` (`docs/reference/invariants.md:15`; `simulation/src/tests/scenarios/ledger.rs:135-493,881-930`) | PARTIAL. The bounded-loss test removes already-debited cargo with no declared sink (`simulation/src/tests/scenarios/retail.rs:679-731`). |
| Single authority | Review-only `wiring-auditor`/`reviewer` references (`docs/reference/invariants.md:16`) | UNGUARDED by an executable economy test. `Market` owns several ledgers and deferred callbacks are broad (`simulation/src/economy/market.rs:23-218`; `simulation/src/utils/par_command_buffer.rs`). |
| Idempotent transitions | None (`docs/reference/invariants.md:17`) | UNGUARDED. No `ProductionRunID`/delivery transition ID exists in the targeted code (`simulation/src/souls/goods_company.rs:69-78`; `simulation/src/economy/market.rs:174-218`). |
| Stable identity | `TestCtx::check_determinism` round-trip (`docs/reference/invariants.md:18`; `simulation/src/tests/mod.rs:96-115`) | PARTIAL only for serialization/key equality; no lifetime/reuse guard for dead citizens. |
| Finite capacity | `scenario_0095_full_output_storage_halts_production`, `scenario_0083_zero_trucks_blocks_delivery` (`docs/reference/invariants.md:19`; `simulation/src/tests/scenarios/recipe_provided.rs:148-190`; `simulation/src/tests/scenarios/hoarding.rs:186-218`) | PARTIAL. Vehicle cargo and dock rates are absent (`simulation/src/transportation/vehicle.rs:34-45`; `simulation/src/economy/market.rs:1109-1151`). |
| Failure persistence | Retail waiting and TTL tests plus the no-truck scenario (`docs/reference/invariants.md:20`; `simulation/src/tests/scenarios/retail.rs:241-323`; `simulation/src/tests/scenarios/hoarding.rs:186-218`) | PARTIAL. The listed AHW test is absent; non-human unmatched orders are extracted on the fallback path (`simulation/src/economy/market.rs:629-632`). |
| Non-price domestic clearing | `scenario_0097_production_never_checks_treasury` (`docs/reference/invariants.md:21`; `simulation/src/tests/scenarios/recipe_provided.rs:231-280`) | PARTIAL. It guards production only; no direct domestic matching-money test is listed, and domestic action/upkeep debits remain (`simulation/src/economy/government.rs:21-75`; `simulation/src/economy/mod.rs:53-54`). |
| Determinism | `TestCtx::check_determinism` and `test_world_survives_serde` (`docs/reference/invariants.md:22`; `simulation/src/tests/mod.rs:96-115`; `simulation/src/tests/test_iso.rs:241-314`) | GUARDED for round-trip and repeat replay. The “no repeat-run test” text is stale. |
| Observable discrepancy, no hidden verdict | `scenario_0151_inflated_request_hoards_honest_does_not` (`docs/reference/invariants.md:23`; `simulation/src/tests/scenarios/hoarding.rs:225-309`) | PARTIAL. The simulation seed is tested, but the Planner/UI cannot read `requested` (`docs/simulation/planned-economy/enterprise-behavior.md:109-111`). |
| Physical opportunity cost | None (`docs/reference/invariants.md:24`) | UNGUARDED. No priority or displaced-use ledger exists (`docs/simulation/planned-economy/priorities.md:51-56`). |
| Provenance on Planner values | None (`docs/reference/invariants.md:25`) | UNGUARDED. No `PlannerSnapshot` or provenance metadata exists (`docs/simulation/concepts/information.md:82-95`). |

## Mechanics-index pointer check

`python3 scripts/check_docs.py` completed with zero errors. Its checks cover relative links, SUMMARY targets, active-page reachability, duplicate H1 titles, and metadata (`scripts/check_docs.py:5-16`). Every Markdown design-page pointer in `docs/reference/mechanics-index.md` resolves, including both `allocation.md` pages and the future public-transport page. The checker emits a duplicate-H1 warning for the two pages named `Allocation`; this is not a dead pointer, but it makes the two competing domain pages easy to confuse. The remaining semantic pointer corrections are:

- `Import as physical truck` (`docs/reference/mechanics-index.md:31`) should be `PARTIALLY IMPLEMENTED` until source cargo and clearance settlement are authoritative.
- `Custody conservation on cancel/return` (`docs/reference/mechanics-index.md:32`) should be `PARTIALLY IMPLEMENTED` until bounded losses are accounted for.
- `Domestic money gate` (`docs/reference/mechanics-index.md:26`) should be renamed to distinguish a non-gating treasury debit from price-based matching.
- The determinism guard text (`docs/reference/mechanics-index.md:54` and `docs/reference/invariants.md:22`) should include `test_world_survives_serde`.
- Unlinked shorthand such as `production §003` and `logistics` is readable but is not a machine-checked Markdown pointer; consider linking the specification anchors in the next consolidation pass.

## Consolidation proposals

1. Make `docs/reference/mechanics-index.md` the one current-substrate matrix. Keep domain pages explanatory, but have them link to row anchors instead of copying status and line numbers.
2. Split the two `Allocation` pages by boundary: planned-economy allocation should describe target policy/priority; physical-economy allocation should describe the request-to-dispatch state transition. Rename one H1 or add a scope-qualified title.
3. Add one authoritative border-trade page/section covering source cargo, customs clearance, physical import/export, and deferred rouble settlement. Link the glossary, trade specification, logistics page, and Mechanics rows to it.
4. Add a short “virtual labor token” section next to the physical sequence. Keep `job-opening` out of cargo states and cargo conservation, while preserving its synchronous hiring behavior.
5. Add a current-versus-target event ledger table. Distinguish `matched`, `reserved`, `loaded`, `delivered`, `consumed`, `lost-in-transit`, and `reported`; map each to its owning code field and guard test.
6. Maintain the invariant index from executable scenario names. Mark each guard as full or partial and record deliberate sinks, not only warning logs.
7. Reconcile the resource catalogue against the charter: Medicine is absent; `job-opening` is non-physical; the six extra current items are outside the charter catalogue.
8. Add a transport current-authority table. The target six-authority map is useful, but current code ownership is `Market` for dispatch state, `Dispatcher` for vehicle reservation, `FreightStation` for rail counters, and `Vehicle`/`Itinerary` for movement.

## Out of slice

- `docs/reference/specifications/trade.md`, `production.md`, and `logistics.md` are higher-authority targets; this review cites their contracts but does not propose editing their wording here.
- The Planner/UI observability gap is implemented in `native_app/src/gui/inspect/inspect_building.rs`, outside the requested economy-document tree.
- `docs/research/fact-sheets/wave1-economy.md` and `wave1-logistics.md` supply ECO/LOG evidence IDs; reconcile their stale references in the research slice.

## Open questions

- Is a freight station an authoritative foreign-stock source, or is it only a domestic endpoint whose rail counters represent pending border demand?
- Should a failed in-transit load be recoverable cargo or a declared `lost_in_transit` sink?
- Should `job-opening` become a first-class labor transition rather than remain an `item` with a special branch?
- Should domestic construction/upkeep treasury debits be removed, or retained as a clearly non-gating player resource outside the rouble model?
- What is the intended behavior when the last external customs station is removed while a train is loading?
- Which current code module is the authoritative owner for border clearance once the target Trade contract is implemented?
