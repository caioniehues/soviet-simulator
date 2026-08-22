# EPIC-003 — Building Lifecycle

**Summary:** Building Lifecycle
**Stories:** STORY-0010, STORY-0011, STORY-0012
**Primary sources:** `spec/buildings.md`
**Status:** 0/3 done

## STORY-0010

**Epic:** EPIC-003 — Building Lifecycle
**Title:** Renovate or expand an operating building in place without evicting occupants

**As a** the planner
**I want** a renovation project to add capacity to an existing building on its existing lot while current occupants and workers remain
**So that** densification never displaces people, keeping CS1's append-on-upgrade behaviour while rejecting its market trigger

**Acceptance criteria:**
- AC-1: Starting a renovation project on an operating building does not remove, relocate, or interrupt its current occupants' or workers' assignment. [SUBSTRATE: ABSENT — greenfield; no renovation mechanic or building-lifecycle state machine exists in the fork] · impact:`journey` · seam:`integration` · scenario:`SCENARIO-0007`
- AC-2: A renovation project is driven by the same phased construction machinery (bill of quantities, ordered phases, stall/bottleneck reporting) as new construction, not a separate code path. [SUBSTRATE: ABSENT — greenfield, depends on the ConstructionProject model itself being greenfield] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0007`
- AC-3: On renovation completion the building's declared capacity increases (e.g. more dwelling slots) without changing its lot footprint or position. [SUBSTRATE: UNAUDITED — no capacity-mutation-in-place path exists to check] · impact:`local` · seam:`integration` · scenario:`SCENARIO-0007`

**Sources:**
- `spec/buildings.md:34-42`

**Status:** pending

## STORY-0011

**Epic:** EPIC-003 — Building Lifecycle
**Title:** Degrade an operating building's condition when maintenance inputs are starved

**As a** the simulation
**I want** an operating building to lose condition/quality when denied ongoing maintenance inputs (heat, repairs, materials) and become uninhabitable or unusable below a threshold
**So that** neglect has a visible physical consequence rather than an abstract land-value stat, and the game never ends — only degrades leaner

**Acceptance criteria:**
- AC-1: Any operating building — residential or non-residential — that goes without its declared maintenance input (e.g. heat, repairs, materials) for N consecutive ticks has its condition value drop below full; residential buildings use $QUALITY_OF_LIVING as their named precedent, and non-residential buildings degrade by the same starvation mechanism. [SUBSTRATE: ABSENT — greenfield; buildings.md:39 names $QUALITY_OF_LIVING as the residential precedent for a maintenance-input sink applying to buildings generally, and no such sink or condition field exists anywhere in the audited code] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0008`
- AC-2: A building whose condition reaches zero becomes uninhabitable/unusable (its dwelling/workplace/service capacity is withdrawn) until renovated or repaired, and is never automatically demolished or deleted. [SUBSTRATE: ABSENT — greenfield; contrasts with CS1's rejected auto-despawn-on-mismatch, spec/zoning.md:16-23] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0008`
- AC-3: Restoring the maintenance input halts further condition decay but does not by itself restore condition — a separate repair process is required to raise it. [SUBSTRATE: UNAUDITED — design intent inferred from the spec's degrading/renovation split (buildings.md:34-42), no code exists either way] · impact:`local` · seam:`integration` · scenario:`SCENARIO-0008`

**Sources:**
- `spec/buildings.md:34-49`

**Status:** pending

## STORY-0012

**Epic:** EPIC-003 — Building Lifecycle
**Title:** Declare a building as flat typed data: function type, capacities, connections

**As a** a designer authoring the building catalogue
**I want** each building prototype to declare one function-type token, per-class storage/dwelling capacity, declared workforce, and explicit connection points
**So that** capacity lives in data and policy lives in the plan, matching the W&R grammar the spec adopts wholesale

**Acceptance criteria:**
- AC-1: A building prototype declares exactly one function-type token (e.g. LIVING, FACTORY, SHOP) with no separate zone/class layer standing in for it. [SUBSTRATE: PARTIAL — ADR-0017 confirms one-recipe-per-building for goods companies (GoodsCompanyPrototype.recipe: Option<Recipe>, PROVIDED) but no generic function-type enum spanning dwelling/service/shop buildings exists per audit §3] · impact:`none` · seam:`unit`
- AC-2: A building prototype declares explicit connection points (road, heating, rail siding) with literal coordinates, and is powered/connected only through that explicit declaration — not by mere geometric adjacency to a road or intersection. [SUBSTRATE: CONFLICTS — ElectricityCache makes every building auto-adjacent to its road via union-find (map/electricity_cache.rs:244-280) with no laid-wire or explicit connection-point model, audit §6] · impact:`cross-surface` · seam:`integration`
- AC-3: A material referenced by a building's construction bill of quantities carries at minimum mass, volume, and storage/transport-class metadata usable to compute delivery logistics. [SUBSTRATE: ABSENT — ItemPrototype is `{base, id, optout_exttrade}` only (prototypes/src/prototypes/item.rs:8-12), no item ontology per audit §3] · impact:`local` · seam:`unit`
- AC-4: A building prototype declares its staffing requirement as $WORKERS_NEEDED and, where relevant, $CITIZEN_ABLE_SERVE, as flat data on the prototype; workers filling that requirement are sourced at runtime by labour allocation, not baked into the declaration. [SUBSTRATE: ABSENT — greenfield; no workforce-declaration field exists on any building prototype today, buildings.md:22 names both tokens] · impact:`local` · seam:`unit`
- AC-5: A building prototype declares its own operating storage capacity per transport class ($STORAGE <class> <n>, $STORAGE_DEMAND_*) and, for residential buildings, a dwelling capacity expressed as a people-bucket — distinct from the construction-material metadata of AC-3. [SUBSTRATE: ABSENT — greenfield; buildings.md:21 names both the storage-class and dwelling-bucket declarations, no such fields exist on any prototype today] · impact:`local` · seam:`unit`

**Sources:**
- `spec/buildings.md:14-27`

**Status:** pending