# Review: society

Verdict: incorrect (confidence 0.98)

# Society / infrastructure / specifications slice review

## Summary
- None of the 29 assigned mechanism pages is a complete implemented 1.0 contract. Nine are partial narrow substrates (citizens, housing, time, labor, workplaces, provisioning, infrastructure index, network architecture, electricity); the remaining 20 are design-only. The index pages themselves correctly describe the intended separation between target design and current code (`docs/simulation/society/index.md:8-13`, `docs/simulation/infrastructure/index.md:9-17`).
- Current society code has only HumanEnt/PersonalInfo, Home/Work/Food desires, one-time age, one worker scalar, direct house assignment, and one-bread retail settlement. There is no lifecycle/death system, household, education, healthcare, migration, or social-network system (`simulation/src/souls/human.rs:42-72,127-230,237-278`; `simulation/src/world.rs:88-105`; `simulation/src/init.rs:54-143`).
- Current infrastructure code has road-derived ElectricityCache/blackout and terrain heightmap/trees only; Water, Sewage, Heating, Waste, reservoir, hydro, and Weather have no simulation owner (`simulation/src/map/map.rs:31-48`; `simulation/src/map/electricity_cache.rs:6-63`; `simulation/src/map_dynamic/electricity.rs:40-93`; `simulation/src/map/terrain.rs:35-64`).
- All 22 files in `docs/reference/specifications/` are marked binding but draft and therefore proposed, not authoritative implementation contracts (`docs/reference/specifications/README.md:9-18,58-70`; representative headers such as `docs/reference/specifications/citizens.md:3-7` and `docs/reference/specifications/zoning.md:3-7`). The mechanics index has no Crime row and only a Handling-classes hook for Resources (`docs/reference/mechanics-index.md:33-43,78-88`).
- Failure behavior passes the requested non-terminal check: no gameplay game-over/loss/end-state exists in the simulation loop or native app; only explicit user save/exit paths terminate the app (`simulation/src/lib.rs:235-259`; `native_app/src/gui/mod.rs:157-164`; `native_app/src/gui/hud/menu.rs:59-90`).

## Page classification

Classification is by the mechanism actually supplied by the page's current-substrate section, not by the presence of a future design section.

- **Partial:** `docs/simulation/society/citizens.md`, `housing.md`, `time.md`, `labor.md`, `workplaces.md`, `provisioning.md`; `docs/simulation/infrastructure/index.md`, `network-architecture.md`, `electricity.md`.
- **Design-only:** `docs/simulation/society/index.md`, `households.md`, `education.md`, `healthcare.md`, `demography.md`, `migration.md`, `social-networks.md`; `docs/simulation/society/institutions/index.md`, `soviet-workplaces.md`, `trade-unions.md`, `workplace-representation.md`, `local-soviets.md`, `worker-self-management.md`, `alternate-socialist-institutions.md`; `docs/simulation/infrastructure/water.md`, `sewage.md`, `heating.md`, `hydrology.md`, `waste.md`; `docs/simulation/national-projects/index.md`.
- The partial classification is intentionally narrow: `citizens.md` says the only current identity is HumanEnt plus name/age/gender and no lifecycle (`docs/simulation/society/citizens.md:75-94`); `housing.md` has only Home's constant score and immediate assignment (`docs/simulation/society/housing.md:101-108`); `time.md` has work intervals but no budget/scheduling (`docs/simulation/society/time.md:127-133`); `labor.md` has worker-count productivity only (`docs/simulation/society/labor.md:95-103`); `workplaces.md` has CompanyEnt/workers/recipe but no welfare (`docs/simulation/society/workplaces.md:92-100`); and `provisioning.md` has only the one-bread BuyFood path (`docs/simulation/society/provisioning.md:116-125`).
- The design-only classifications are explicit in the pages: no household (`docs/simulation/society/households.md:93-104`), education (`docs/simulation/society/education.md:73-82`), healthcare (`docs/simulation/society/healthcare.md:91-100`), migration (`docs/simulation/society/migration.md:104-109`), social networks (`docs/simulation/society/social-networks.md:104-108`), institutions (`docs/simulation/society/institutions/index.md:20-22`), and utilities other than electricity (`docs/simulation/infrastructure/water.md:60-65`; `sewage.md:53-57`; `heating.md:65-72`; `hydrology.md:37-40`; `waste.md:59-63`).

## Findings

### 1. Severity: med — Remove the obsolete no-consumption claim from both provisioning pages

**Evidence:** `docs/simulation/society/provisioning.md:116-125` says `last_ate` updates on arrival “without consuming inventory” and cites `buyfood.rs:70-90`. The current arrival branch first verifies the live claim, settles it at eat-time, and then advances `last_ate` (`simulation/src/souls/desire/buyfood.rs:150-168`); `Market::settle_retail` removes the claim, debits seller capital, and releases the reservation (`simulation/src/economy/market.rs:474-490`). The same stale assertion and stale line citation were copied into `docs/simulation/society/healthcare.md:98-100`. This makes the current-substrate evidence factually wrong and falsely labels the implemented retail settlement as a target-spec violation.

**Proposed fix:** Replace both paragraphs with the current behavior: a live retail claim is consumed/settled on arrival, seller capital and reservation are decremented, and `last_ate` advances only on successful settlement; then list the actual remaining gaps (bread-only, no household pantry, no Meat/substitution/search queue). Keep the healthcare page focused on its real absence rather than using an obsolete food example.

### 2. Severity: med — Correct the workplace current-substrate data model

**Evidence:** `docs/simulation/society/workplaces.md:92-97` claims `GoodsCompanyState` stores `workers: WorkerHolder`, `max_workers`, and `comp: CompanyState`. The source defines `GoodsCompanyState` with `proto`, `building`, `max_workers`, `progress`, `driver`, and `trucks`—no workers or `comp` field (`simulation/src/souls/goods_company.rs:69-78`). Workers, sold, and bought are fields of the enclosing `CompanyEnt` (`simulation/src/world.rs:187-195`), and the worker type is `Workers(Vec<HumanID>)` (`simulation/src/economy/mod.rs:35-43`). Implementers following this paragraph will inspect or modify nonexistent fields and miss the actual ownership boundary.

**Proposed fix:** Rewrite the paragraph to identify `CompanyEnt.comp: GoodsCompanyState` plus sibling `CompanyEnt.workers: Workers`, `sold`, and `bought`, and cite the actual source paths/fields. Retain the welfare/canteen absence as a separate design-gap statement.

### 3. Severity: med — Add the missing Crime and Resources coverage to the mechanics index

**Evidence:** The active index says its scope is “one row per mechanic” (`docs/reference/mechanics-index.md:9-13`), but its full table has no Crime mechanic row (`docs/reference/mechanics-index.md:17-76`) even though `crime.md` is a registered draft specification (`docs/reference/specifications/README.md:58-70`). Resources appears only as “Handling classes” with specification `—` (`docs/reference/mechanics-index.md:33-43`), while the draft Resources specification defines the 1.0 resource catalogue, separate Food/Meat needs, and import-only Medicine (`docs/reference/specifications/resources.md:13-19`). The omissions make the active navigation table unable to lead a reviewer from either registered spec to its stated mechanism, and Resources is presented as a Post-1.0-style hook rather than a 1.0 draft target.

**Proposed fix:** Add a `Crime` row marked `P1.0`/`ABSENT` and linked to `crime.md` and `crime` spec, preserving its explicit charter cut; add a `Resources catalogue, stock, Food/Meat boundary, import-only Medicine` row linked to `resources.md` and `resources` spec, while retaining the separate Handling-classes hook if it is still useful.

### 4. Severity: low — Mark healthcare death language as cross-module target behavior

**Evidence:** `docs/simulation/society/healthcare.md:17-20,41-43` says a sick worker “may die” in the healthcare narrative and target design. The page's own current substrate says no health system exists (`docs/simulation/society/healthcare.md:91-98`), and the draft Citizens specification assigns death/lifecycle state and immutable `DeathResultID` to Citizens, with Healthcare excluded from performing lifecycle mutation (`docs/reference/specifications/citizens.md:40-55`; `docs/reference/specifications/healthcare.md:13-18`). No death system is registered in the simulation (`simulation/src/init.rs:54-143`). Without an ownership qualifier, readers can infer that Healthcare owns mortality or that death is currently implemented.

**Proposed fix:** Say “illness may contribute to a future Citizens-owned death outcome” (or equivalent), link to the Citizens spec/demography page, and label it target design. Keep the current-substrate paragraph explicit that neither health nor death is implemented.

### 5. Severity: low — Qualify the infrastructure index as a target architecture

**Evidence:** `docs/simulation/infrastructure/index.md:9-17` uses present-tense wording that each listed network has its own physical solver, inertia, and failure mode. The same page immediately says Electricity is “the only network with any substrate” (`docs/simulation/infrastructure/index.md:31-38`), while `Map` contains only `electricity` and `environment` among utility/environment resources (`simulation/src/map/map.rs:31-48`) and `init` registers no water, sewage, heating, waste, hydrology, or weather systems (`simulation/src/init.rs:54-143`). The present-tense table therefore overstates the implementation status of five networks and hydrology/weather.

**Proposed fix:** Label the table “Target network architecture” and add a current column (Electricity: partial; all other listed networks: absent). Keep the reading path and solver distinctions as design guidance.

### 6. Severity: low — Qualify demography’s opening population claim

**Evidence:** `docs/simulation/society/demography.md:15-19` states that the game's population changes through death and potentially births/migration. Its own current-substrate section says there is no death, birth, or lifecycle system (`docs/simulation/society/demography.md:89-94`), and the runtime only stores one-time `PersonalInfo` age/name/gender with no age progression or death system (`simulation/src/souls/human.rs:42-72`; `simulation/src/init.rs:54-143`). This makes the opening read as a current-game statement before the later target/current distinction corrects it.

**Proposed fix:** Change the opening to “The 1.0 target population model includes death; current code does not yet change population through lifecycle, births, or migration.” Preserve the charter commitment and open-question labels afterward.

## Current-code contract check

- **Demographics/death:** `PersonalInfo` is only `name`, integer `age`, and `gender`; `PersonalInfo::new` samples age once from 20 through 49 (`simulation/src/souls/human.rs:42-72`). `HumanEnt` has no lifecycle/death fields (`simulation/src/world.rs:88-105`), and the registered schedule has no age, mortality, birth, or migration system (`simulation/src/init.rs:54-143`). Entity cleanup is ECS removal, not a modeled death transition.
- **Education:** `HumanEnt` has no education/qualification field (`simulation/src/world.rs:88-105`); `BuildingKind` is exhaustive over House, GoodsCompany, RailFreightStation, TrainStation, and ExternalTrading (`simulation/src/map/objects/building.rs:18-25`); human decisions enumerate only `None`, `Home`, `Work`, and `Food` (`simulation/src/souls/human.rs:127-154`). There is no School or Technical Institute runtime service.
- **Healthcare:** The same BuildingKind and decision limits apply, and no healthcare system/resource consumer is registered (`simulation/src/map/objects/building.rs:18-25`; `simulation/src/souls/human.rs:127-154`; `simulation/src/init.rs:54-143`). The base item catalogue has no Medicine (`base_mod/items.lua:1-108`).
- **Construction Sites:** `Map` stores buildings/lots but no Site collection (`simulation/src/map/map.rs:31-48`); `Map::build_house` removes the lot and calls `Building::make` immediately (`simulation/src/map/map.rs:300-325`); WorldCommand exposes direct map-build commands rather than Site/project lifecycle (`simulation/src/world_command.rs:32-83,210-225,284-297`).
- **Failure/game-over:** The simulation tick increments time and executes the schedule with no terminal game state (`simulation/src/lib.rs:235-259`); the schedule has no game-over/loss path (`simulation/src/init.rs:54-143`). Native-app exit state is only `NoExit`, `ExitAsk`, or `Saving` (`native_app/src/gui/mod.rs:157-164`), and process exit is reached by explicit Save and exit / Exit without saving menu actions (`native_app/src/gui/hud/menu.rs:59-90`). Ordinary panic/error paths are process failures, not gameplay game-over states.

## Draft-spec audit (all 22)

The register declares the specification mechanism layer and says drafts cannot establish completion or override the charter (`docs/reference/specifications/README.md:9-18`). Every file below has `Kind: specification`, `Authority: binding`, `Status: draft`, `Owner`, and `Last verified: 2026-08-24`; representative headers and the complete register are `docs/reference/specifications/buildings.md:3-7`, `docs/reference/specifications/citizens.md:3-7`, `docs/reference/specifications/resources.md:3-7`, `docs/reference/specifications/zoning.md:3-7`, and `docs/reference/specifications/README.md:58-70`.

The following inventory gives each draft's mechanics-index coverage, current simulation-page contradiction/gap, and charter result. “No charter contradiction” means the proposed target was checked against the active charter; the implementation gap is not itself a charter contradiction. The charter's invariants are physical goods, non-price domestic clearing, failure-as-queues/going-without, and border-only roubles (`docs/plan/charter-1.0.md:22-31`).

- **Buildings** — index bundled with Construction (`docs/reference/mechanics-index.md:41`); target Site/completed-building lifecycle conflicts with immediate building activation (`docs/simulation/physical-economy/buildings.md:78-93`); no direct charter contradiction.
- **Citizens** — index row “Citizen persistent identity” (`docs/reference/mechanics-index.md:43`); current HumanEnt has identity but no lifecycle (`docs/simulation/society/citizens.md:75-94`); no direct charter contradiction, and the draft explicitly excludes Crime/kindergarten/deathcare/epidemics from 1.0 (`docs/reference/specifications/citizens.md:24-26`).
- **Construction** — bundled with Buildings in the Construction Site row (`docs/reference/mechanics-index.md:41`); Ghost/Site/material gates conflict with direct money-paid, immediate placement (`docs/simulation/physical-economy/construction.md:110-119`); no direct charter contradiction.
- **Crime** — no mechanics-index row in the active table (`docs/reference/mechanics-index.md:17-76`); the draft correctly records no 1.0 state/transition and says current building kinds/decisions have no crime (`docs/reference/specifications/crime.md:15-23,36-48`); no contradiction—the charter explicitly cuts Crime (`docs/plan/charter-1.0.md:59-65`).
- **Education** — index row “Education two tiers” (`docs/reference/mechanics-index.md:53`); target School/Technical Institute and qualification have no current service (`docs/simulation/society/education.md:73-82`); no contradiction—the draft excludes kindergarten as a charter cut (`docs/reference/specifications/education.md:22-25,91-93`).
- **Electricity** — index row “Electricity: wire, storage, priority shedding” (`docs/reference/mechanics-index.md:69`); explicit wire/storage/continuous shedding conflicts with road-derived binary blackout (`docs/simulation/infrastructure/electricity.md:50-68`); no contradiction—the draft excludes voltage tiers/transformers/CHP/electric fallback (`docs/reference/specifications/electricity.md:13,52`).
- **Healthcare** — index row “Healthcare, Medicine chain” (`docs/reference/mechanics-index.md:54`); target physical care/Medicine chain has no current healthcare type or consumer (`docs/simulation/society/healthcare.md:91-100`); no contradiction—the draft excludes epidemics/deathcare/fees and domestic prices (`docs/reference/specifications/healthcare.md:25-27,99-101`).
- **Heating** — index row “Heating, no electric fallback” (`docs/reference/mechanics-index.md:72`); target thermal network/weather interface has no current heating system (`docs/simulation/infrastructure/heating.md:65-72`); no contradiction—the draft defers Weather and repeats the CHP/electric fallback cuts (`docs/reference/specifications/heating.md:52`).
- **Households** — index rows “Household entity, shared pantry” and “Housing queue” (`docs/reference/mechanics-index.md:44-45`); target household/queue conflicts with no Household entity and only Home pointer (`docs/simulation/society/households.md:93-104`; `housing.md:101-108`); no contradiction—the charter requires persistent identities/observable failure, not a specific household data shape (`docs/plan/charter-1.0.md:22-31`).
- **Logistics** — index rows for physical hauls, truck lifecycle, custody, loading/unloading (`docs/reference/mechanics-index.md:17-35`); target finite custody/recovery conflicts with the current dispatch seam and missing cargo/recovery details (`docs/simulation/physical-economy/logistics.md:117-131`); no direct charter contradiction—the target is an implementation of the no-teleport/failure-queue pillars.
- **Needs** — index rows for durable unmet demand and Food/Meat/going-without (`docs/reference/mechanics-index.md:38,46-47`); target durable demand conflicts with current one-tick demand replacement and bread-only BuyFood (`docs/simulation/society/provisioning.md:116-125`); no contradiction—the draft explicitly prohibits domestic money and preserves going-without (`docs/reference/specifications/needs.md:15-18,107-109`).
- **Pathfinding** — index A* row (`docs/reference/mechanics-index.md:65`); target invalidation/failure/recovery is broader than current static/retry-only routing (`docs/simulation/transport/pathfinding.md:80-98`); no direct charter contradiction.
- **Production** — index rows for production/resource/recipe constraints (`docs/reference/mechanics-index.md:17-19,39-40`); target bounded physical inputs/outputs conflicts with current integer recipe seam (`docs/simulation/physical-economy/production.md:100-113`); no contradiction—the draft explicitly keeps Water utility and Medicine import-only (`docs/reference/specifications/production.md:21-24`).
- **Resources** — only the Handling-classes hook has a row and its spec column is `—` (`docs/reference/mechanics-index.md:33-43`); target catalogue/physical stock is broader than the current unitless Lua item list (`docs/simulation/physical-economy/resources.md:89-103`); no contradiction—the draft explicitly excludes perishability/containers/fuel lifecycle (`docs/reference/specifications/resources.md:26-28,102-103`).
- **Roads** — index rows for junction/parking/roads (`docs/reference/mechanics-index.md:63-64`); Planner-authored road/lots target conflicts with auto-generated lots/current placement (`docs/simulation/transport/roads.md:84-100`); no contradiction—the draft excludes road pricing and transport cuts (`docs/reference/specifications/roads.md:22-24,98-100`).
- **Sewage** — index row “Sewage” (`docs/reference/mechanics-index.md:71`); target separate finite network conflicts with no current sewage kind/system (`docs/simulation/infrastructure/sewage.md:53-57`); no direct charter contradiction.
- **Trade** — index rows non-price matching/export/import/dispatch (`docs/reference/mechanics-index.md:30-32`); physical border-clearing target conflicts with domestic matching that settles a claim before physical movement (`docs/simulation/physical-economy/trade.md:111-123`); no contradiction—the draft restates border-only roubles and no domestic price clearing (`docs/reference/specifications/trade.md:15-17,22-25`).
- **Traffic** — index rows collision/congestion/traffic pressure (`docs/reference/mechanics-index.md:60-62`); target durable congestion ledger conflicts with current microscopic collision/retry behavior and no ledger/readout (`docs/simulation/transport/traffic.md:89-108`); no contradiction—the draft explicitly forbids road pricing/game-over modes (`docs/reference/specifications/traffic.md:21-24,106-108`).
- **Vehicles** — index rows cargo/capacity, movement, freight rail (`docs/reference/mechanics-index.md:35,58-59,66`); target persistent cargo/capacity/owner/depot/recovery conflicts with current vehicles lacking those logistics fields (`docs/simulation/transport/vehicles.md:76-87`); no contradiction—the draft excludes manufacture/fuel/passenger rail and keeps Water out of cargo (`docs/reference/specifications/vehicles.md:20-23,86-87`).
- **Waste** — index row “Waste” (`docs/reference/mechanics-index.md:73`); target container/haul/treatment chain conflicts with no current waste kind/system (`docs/simulation/infrastructure/waste.md:59-63`); no direct charter contradiction.
- **Water** — index row “Water transfer, quality, border meter” (`docs/reference/mechanics-index.md:70`); target pressure/quality/meter network conflicts with no Water runtime owner (`docs/simulation/infrastructure/water.md:60-65`); no contradiction—the charter requires Water as a utility and never cargo (`docs/plan/charter-1.0.md:35-45`).
- **Zoning** — index row “Auto-generated lots” (`docs/reference/mechanics-index.md:42`); target zoning/lots contract conflicts with direct road-derived lot generation and immediate placement (`docs/simulation/transport/roads.md:84-100`); no direct charter contradiction.

No draft target contradicted the active charter on review. The important distinction is that all are non-ratified proposals, while several current substrates are explicitly marked `CONTRADICTED` in the index (domestic money gate, teleporting export clearance, auto-generated lots, and binary road-derived electricity) (`docs/reference/mechanics-index.md:27-31,41-42,69`).

## Consolidation proposals

1. Add a single “current substrate matrix” to the society and infrastructure indexes. Keep each page's target design, but make the first navigation table say `PARTIAL` versus `ABSENT` and link the exact source heading. This prevents present-tense architecture prose from being read as implementation.
2. Keep the Household specification as the owner of both household state and the housing queue, and make `housing.md` a focused concept page that links to those anchors. The current page already says no separate housing specification exists (`docs/simulation/society/housing.md:24-31`); repeat that ownership in the mechanics index and avoid future competing `housing.md` identifiers.
3. Split “current retail settlement” from “target provisioning.” `buyfood.rs`/`Market::settle_retail` now provide a narrow physical claim/consumption path, while Food/Meat, household pantry, search, substitution, and durable queues remain absent. Use one shared current-substrate paragraph rather than the stale copy in Provisioning and Healthcare.
4. Give death one explicit owner in the society docs: Citizens records lifecycle/DeathResultID; Households consumes membership result; Healthcare can contribute a future outcome but does not mutate lifecycle. Link demography and healthcare to the same Citizens anchors.
5. Give the infrastructure index two status columns: `Target solver/inertia` and `Current owner`. Electricity can point to `ElectricityCache`/`ElectricityFlow`; Water, Sewage, Heating, Waste, Hydrology, and Weather should say no runtime owner. Terrain's implemented `Environment` (heightmap/trees) should not be conflated with reservoir/hydro.
6. Expand mechanics-index navigation to every registered draft (at least Crime and Resources) and preserve the `Current` verdict. For Resources, separate the 1.0 catalogue/stock row from the optional Handling-classes hook. For Crime, retain the P1.0/charter-cut status so the row cannot be mistaken for 1.0 scope.
7. Keep the no-game-over invariant in each failure-oriented page's current/target text. The simulation has no gameplay terminal state, so future queue/shortage pages should use the charter's “going without” vocabulary rather than implying a deathcare, utility, or service failure ends the plan.

## Out of slice

- The physical-economy and transport pages/specs above were inspected only where referenced by a society/infrastructure draft; deeper economy/trade/logistics contradictions belong to the EconomyDomain or another owner.
- Architecture/current-substrate authority, product/game modes, research/archive provenance, and engineering-process checks belong to the other review slices.
- `docs/archive/**` was treated as historical only and no edit is proposed.

## Open questions

- Should the mechanics index explicitly require one row for every registered spec, including Post-1.0 specs such as Crime, or should the register mark navigation-only specs separately?
- Is the existing `Market` seller capital/reservation model intended to remain the authoritative narrow retail inventory for the next implementation step, or should Provisioning defer to the Resources/Needs stock interfaces immediately?
- Which Citizens transition currently triggers a future death outcome, given that no death system is registered and Healthcare is explicitly not the lifecycle owner?
- Should hydrology/reservoir/hydro be represented as a separate implemented terrain/environment status, rather than grouped with absent utility solvers?
- Do the draft indexes need an explicit generated status table so a page cannot silently drift from `PARTIAL`/`ABSENT` when source code changes?


## Findings

### [medium] Remove the obsolete no-consumption claim from both provisioning pages
`docs/simulation/society/provisioning.md:116-125`

`docs/simulation/society/provisioning.md:116-125` says `last_ate` updates on arrival “without consuming inventory” and cites `buyfood.rs:70-90`, and the same assertion appears in `docs/simulation/society/healthcare.md:98-100`. The current arrival branch verifies the live retail claim, settles it at eat-time, and only then advances `last_ate` (`simulation/src/souls/desire/buyfood.rs:150-168`); `Market::settle_retail` removes the claim, debits seller capital, and releases the reservation (`simulation/src/economy/market.rs:474-490`). This makes the current-substrate evidence factually wrong and labels the implemented retail settlement as a target-spec violation.

### [medium] Correct the workplace current-substrate data model
`docs/simulation/society/workplaces.md:92-97`

`docs/simulation/society/workplaces.md:92-97` claims `GoodsCompanyState` stores `workers: WorkerHolder`, `max_workers`, and `comp: CompanyState`. The source defines `GoodsCompanyState` with `proto`, `building`, `max_workers`, `progress`, `driver`, and `trucks`—no workers or `comp` field (`simulation/src/souls/goods_company.rs:69-78`). Workers, sold, and bought are fields of the enclosing `CompanyEnt` (`simulation/src/world.rs:187-195`), and the worker type is `Workers(Vec<HumanID>)` (`simulation/src/economy/mod.rs:35-43`), so the page directs implementers to nonexistent fields and hides the actual ownership boundary.

### [medium] Add the missing Crime and Resources coverage to the mechanics index
`docs/reference/mechanics-index.md:33-43`

The active index defines its scope as “one row per mechanic” (`docs/reference/mechanics-index.md:9-13`), but its table has no Crime mechanic row (`docs/reference/mechanics-index.md:17-76`) even though Crime is a registered draft specification (`docs/reference/specifications/README.md:58-70`). Resources appears only as “Handling classes” with specification `—` (`docs/reference/mechanics-index.md:33-43`), while its draft defines the 1.0 resource catalogue, separate Food/Meat needs, and import-only Medicine (`docs/reference/specifications/resources.md:13-19`). Reviewers therefore cannot navigate from either registered spec to its stated mechanism, and Resources is presented as a hook rather than a 1.0 draft target.

### [low] Mark healthcare death language as cross-module target behavior
`docs/simulation/society/healthcare.md:17-20`

`docs/simulation/society/healthcare.md:17-20,41-43` says a sick worker “may die” in healthcare narrative and target design, but its current substrate has no health system (`docs/simulation/society/healthcare.md:91-98`). The draft Citizens specification assigns lifecycle state and immutable `DeathResultID` to Citizens, while Healthcare does not perform lifecycle mutation (`docs/reference/specifications/citizens.md:40-55`; `docs/reference/specifications/healthcare.md:13-18`), and no death system is registered (`simulation/src/init.rs:54-143`). Without the ownership qualifier, readers can infer that Healthcare owns mortality or that death is currently implemented.

### [low] Qualify the infrastructure index as a target architecture
`docs/simulation/infrastructure/index.md:9-17`

`docs/simulation/infrastructure/index.md:9-17` uses present-tense wording that each listed network has its own physical solver, inertia, and failure mode. The same page says Electricity is “the only network with any substrate” (`docs/simulation/infrastructure/index.md:31-38`), while `Map` contains only `electricity` and `environment` among utility/environment resources (`simulation/src/map/map.rs:31-48`) and `init` registers no water, sewage, heating, waste, hydrology, or weather systems (`simulation/src/init.rs:54-143`). The present-tense table overstates implementation status for those absent networks.

### [low] Qualify demography’s opening population claim
`docs/simulation/society/demography.md:15-19`

`docs/simulation/society/demography.md:15-19` states that the game's population changes through death and potentially births/migration, but its own current-substrate section says there is no death, birth, or lifecycle system (`docs/simulation/society/demography.md:89-94`). Runtime stores only one-time `PersonalInfo` age/name/gender with no age progression or death system (`simulation/src/souls/human.rs:42-72`; `simulation/src/init.rs:54-143`), so the opening reads as a current-game statement before the later target/current distinction corrects it.
