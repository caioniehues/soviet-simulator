# Wave 2 built-world, settlement, and utilities substrate fact-sheet

**Kind:** reference
**Authority:** reference
**Status:** active
**Owner:** architecture
**Last verified:** 2026-08-24
**Commit:** `3ec534c`

This Phase 0 fact-sheet constrains Wave 2 documentation only. It records the current Rust,
Lua/prototype, save, UI, and test substrate; it does not ratify a mechanism or claim target
behaviour is implemented. Classifications apply to the present hard fork, not legacy
specifications or the reference game.

## 2A — built world: construction, buildings, and zoning

| Surface | Classification | Current source and reachability | Singular-owner implication |
|---|---|---|---|
| Placement and activation | **PROVIDED** | `WorldCommand` contains immediate house, special-building, and zone commands (`simulation/src/world_command.rs:35-100`). `apply` directly invokes `Map::build_house` or `Map::build_special_building` and inserts `BuildingInfos` (`simulation/src/world_command.rs:223-300`). The building toolbox queues the special-building command (`native_app/src/gui/hud/toolbox/building.rs:163-180`). | `Map` is the only live owner of building existence and topology; future construction state must not compete with it for completed-building ownership. |
| Construction state, labour, and material delivery | **ABSENT** | `Building` stores footprint, kind, mesh, zone, and road link, but no site, phase, bill, material, labour, or delivery field (`simulation/src/map/objects/building.rs:70-159`). Placement materializes the building in the command application path above. | A future construction authority must own the site-to-completion transition and material custody; no current type can honestly be promoted as that authority. |
| Prototype declarations | **PARTIAL** | `BuildingPrototype` parses size, generator, asset, price, and optional power fields (`prototypes/src/prototypes/building.rs:25-54`). Goods-company Lua declares recipes, workers, prices, zones, and power (`base_mod/companies.lua:1-115`); its parsed prototype has recipe, trucks, workers, and optional zone (`prototypes/src/prototypes/goods_company.rs:18-44`). | Lua/prototype fields are declarations, not construction evidence. `Map` owns placed instances; a ratified construction spec must assign the pending-site contract explicitly. |
| Zoning | **PARTIAL** | A zone is geometry attached to `Building`; `UpdateZone` writes it directly (`simulation/src/map/objects/building.rs:46-80`, `simulation/src/world_command.rs:337-340`). Only goods-company prototypes expose an optional zone (`prototypes/src/prototypes/goods_company.rs:18-44`). | `Map` owns the attached zone geometry. No settlement-land-use or allocation owner exists. |
| Domestic-money placement | **CONFLICTING** | Every command applies `Government::action_cost` by debiting `Government.money` (`simulation/src/world_command.rs:223-225`); house, road, zoning, and special-building prices are computed in bucks (`simulation/src/economy/government.rs:21-75`). The debug HUD also displays and checks that money (`native_app/src/debug_gui/hud.rs:47-55`). | This conflicts with the charter/glossary ruling that domestic clearing has no money. Wave 2 must describe it as a substrate conflict to be displaced, never as a target construction mechanism. |
| Save/UI/test reachability | **PARTIAL** | `Map` is registered for Bincode save and `BuildingInfos` is saved too (`simulation/src/init.rs:80-90`); the only cited building-topology test is cache connectivity (`simulation/src/map/electricity_cache.rs:464-490`). | Save coverage is serialization evidence only; it does not establish construction lifecycle, player-facing proof, or determinism. |

**Reference-game comparison (non-authoritative):** the archived import audit reports instant,
uncosted roads/buildings and no full construction process; it is rewrite provenance only, not a
mechanism source (`docs/archive/egregoria-import/substrate-audit-2026-08-22.md:64-105`).

## 2B — settlement: citizens, households, and services

| Surface | Classification | Current source and reachability | Singular-owner implication |
|---|---|---|---|
| Individual identity and lifecycle | **PARTIAL** | `HumanEnt` persists identity-facing `PersonalInfo`, location, home, food, optional work, routing, and purchase state (`simulation/src/world.rs:86-104`). Empty houses cause `spawn_human`; companies and freight stations receive other souls (`simulation/src/souls/mod.rs:15-55`). | The `HumanEnt` is the sole present individual identity. A household must be a new authority, not an alias for a house or a market account. |
| Residence, work, food, and trips | **PARTIAL** | Human spawn assigns a house, personal car, food desire, and a job-opening market request; it records the human as the building owner (`simulation/src/souls/human.rs:234-274`). Decisions choose among home, work, and food, then route to destinations (`simulation/src/souls/human.rs:135-230`). | `HumanEnt` owns personal desires and itinerary; no household owns shared needs, residence assignment, or a pantry. |
| Households and shared provisioning | **ABSENT** | The only live building kinds are House, goods company, freight station, train station, and external trading (`simulation/src/map/objects/building.rs:17-37`); human state has no household identifier or shared inventory (`simulation/src/world.rs:86-104`). | A future household authority must own membership and shared provision state; do not distribute those transitions between humans, buildings, and Market. |
| Education, healthcare, crime, and service capacity | **ABSENT** | Current building kinds and human decisions enumerate no school, healthcare, crime, or service state (`simulation/src/map/objects/building.rs:17-37`, `simulation/src/souls/human.rs:127-230`). Leisure Lua declares capacity, hours, and fee, but no current simulation consumer is shown here (`base_mod/leisure.lua:1-18`). | Each future service needs one authoritative capacity/queue and outcome owner. Leisure declaration alone must not be represented as a reachable service. |
| Save/UI/test reachability | **PARTIAL** | Human entities derive serialization and the scheduler registers human decisions (`simulation/src/world.rs:86-104`, `simulation/src/init.rs:52-70`). Existing settlement-adjacent tests manually spawn a human for freight behaviour (`simulation/src/souls/freight_station.rs:164-205`); no cited household/service evidence exists. | Serialization and fixture use do not prove household, education, health, or crime behaviour. |

## 2C — utilities: electricity, Water, sewage, heating, and waste

| Surface | Classification | Current source and reachability | Singular-owner implication |
|---|---|---|---|
| Electricity topology | **PROVIDED, but limited** | `ElectricityCache` creates networks of buildings, roads, and intersections (`simulation/src/map/electricity_cache.rs:6-63`). Building creation inserts the building and connects it to its selected road (`simulation/src/map/objects/building.rs:132-153`). | `Map.electricity` is the sole live topology/cache owner; a new utility network must not silently share this road-derived graph without an explicit contract. |
| Electricity flow and consequences | **PARTIAL** | Each tick the flow sums production and consumption per connected network, then records a binary blackout (`simulation/src/map_dynamic/electricity.rs:40-92`); the system and Bincode resource are registered (`simulation/src/init.rs:52-52`, `simulation/src/init.rs:80-80`). The HUD renders a no-power marker for blackout networks (`native_app/src/gui/hud.rs:61-105`). | `ElectricityFlow` owns current aggregate flow/blackout state. It has no rate, storage, tier, load-shedding, or consumer-service outcome authority. |
| Lua/prototype power declarations | **PARTIAL** | Building prototypes parse optional power consumption/production (`prototypes/src/prototypes/building.rs:25-54`), and current company Lua declares both consumers and power plants (`base_mod/companies.lua:1-115`). The UI displays those prototype values (`native_app/src/gui/hud/toolbox/building.rs:137-149`). | Parsed fields feed the electricity calculation only through placed, owned companies; declarations are not a generic utilities substrate. |
| Water transfer | **ABSENT** | The current building-kind and scheduled-system lists contain no Water network/system (`simulation/src/map/objects/building.rs:17-37`, `simulation/src/init.rs:52-70`). A repository sweep found renderer/terrain water only, not a simulation Water utility. | Water is tradable but never cargo by project ruling. A future Water authority must own connected, rate-limited transfer and completion before Trade clearance; no vehicle/freight authority may stand in for it. |
| Sewage, heating, waste | **ABSENT** | No corresponding current building kinds or registered simulation systems appear in the same authoritative enumerations (`simulation/src/map/objects/building.rs:17-37`, `simulation/src/init.rs:52-70`). | Each requires a separately named authoritative network/transfer or accumulation owner; none may be inferred from electricity or Market. |
| Save/UI/test reachability | **PARTIAL for electricity; ABSENT otherwise** | Electricity cache topology has a connectivity test (`simulation/src/map/electricity_cache.rs:464-490`), flow is registered for save (`simulation/src/init.rs:80-80`), and blackout is visible in HUD (`native_app/src/gui/hud.rs:61-105`). There is no cited Water/sewage/heating/waste save, UI, or test surface. | Electricity evidence proves only current binary network behaviour; it supplies no evidence for the absent utilities. |

## Rewrite boundary

Wave 2 specifications may use this sheet to state current substrate, conflicts, and absences.
They must remain draft and unratified until their named evidence bindings exist. The later
verification command is `cargo test -p simulation -- --test-threads=1`; it was not run for this
read-only Phase 0 artifact.
