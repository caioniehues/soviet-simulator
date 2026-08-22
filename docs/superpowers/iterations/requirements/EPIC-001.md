# EPIC-001 — Construction Process

**Summary:** Construction Process
**Stories:** STORY-0001, STORY-0002, STORY-0003, STORY-0004, STORY-0005, STORY-0006
**Primary sources:** `spec/buildings.md`, `spec/construction.md`
**Status:** 0/6 done

## STORY-0001

**Epic:** EPIC-001 — Construction Process
**Title:** Gate building activation on physical construction completion

**As a** the planner
**I want** a placed building blueprint to become a construction site whose declared capabilities stay inactive until materials and labour are physically consumed
**So that** nothing teleports and money never substitutes for a missing crane or steel delivery

**Acceptance criteria:**
- AC-1: Placing a building blueprint creates a ConstructionProject with zero active capabilities (no dwelling/workplace/storage usable) until the final phase completes. [SUBSTRATE: CONFLICTS — Building::make (map/objects/building.rs) and Road::make materialize fully and instantly today per audit §1 'Roads/buildings as construction output: CONFLICTS'; no ConstructionProject type exists (audit §3)] · impact:`journey` · seam:`integration` · scenario:`SCENARIO-0002`
- AC-2: No Money is debited or credited at the moment a blueprint is placed. [SUBSTRATE: CONFLICTS — BuildingPrototype.price: Money (prototypes/src/prototypes/building.rs:33) is the flat purchase price spec/construction.md:26-36 explicitly rejects] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0002`
- AC-3: Querying a building entity mid-construction (e.g. for housing allocation or production) returns no available capacity — it is invisible to consumers of finished buildings. [SUBSTRATE: ABSENT — greenfield; today's Building is fully queryable the instant Building::make returns] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0002`

**Sources:**
- `spec/construction.md:1-24`
- `spec/buildings.md:34-42`

**Status:** pending

## STORY-0002

**Epic:** EPIC-001 — Construction Process
**Title:** Author building construction cost as a bill of quantities, not a flat price

**As a** a designer authoring the building catalogue
**I want** each building prototype to declare a per-phase materials-and-work bill of quantities instead of a flat Money price
**So that** what a building costs to build is physical resources and labour-work, matching the W&R grammar the spec adopts wholesale

**Acceptance criteria:**
- AC-1: A building prototype declares, per construction phase, a list of (material class, quantity) pairs and a work amount, replacing or augmenting the current flat price field. [SUBSTRATE: CONFLICTS — building.rs:33 price: Money is the only cost field today; no per-phase bill-of-quantities schema exists, greenfield per audit §3] · impact:`local` · seam:`unit`
- AC-2: Editing a building's bill of quantities in the Lua catalogue and restarting the game changes the material/work required for the next construction of that type; no running session picks up the change without a restart. [SUBSTRATE: PROVIDED (as a constraint to honour) — prototypes load once into a leaked static with no hot reload, prototypes/load.rs:17-31,61-63 per audit §3 and ADR-0017] · impact:`local` · seam:`integration`

**Sources:**
- `spec/construction.md:26-36`
- `spec/construction.md:100-119`

**Status:** pending

## STORY-0003

**Epic:** EPIC-001 — Construction Process
**Title:** Progress a construction project through ordered, stallable phases

**As a** the simulation
**I want** a ConstructionProject to advance through ordered phases (earthworks, foundations, structure, utilities, finishing), each gated on its own material delivery and machine assignment
**So that** the player can see exactly where and why a build stalls, never a silent all-or-nothing wait

**Acceptance criteria:**
- AC-1: A phase cannot begin accumulating work while its predecessor phase is not yet marked complete, and a building's phase list declares only the subset of phases its construction method actually uses (e.g. a panel building lists PANELS_LAYING, not BRICKS_LAYING), in craft order; the earthworks phase always carries a zero work-multiplier (pure earthmoving, never itself a material stall). [SUBSTRATE: ABSENT — no ConstructionPhase/ConstructionProject type exists anywhere in the fork, greenfield per audit §3] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0003`
- AC-2: A phase makes zero progress while its required material class has not been fully delivered to the site, and resumes automatically the tick delivery satisfies the bill — no manual retrigger needed. [SUBSTRATE: ABSENT — audit §3 'Full construction process: ABSENT/CONFLICTS'; nearest existing gate is the boolean all-or-nothing input check in recipe_should_produce, souls/goods_company.rs:36-39, not phase-aware] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0003`
- AC-3: A phase makes zero progress while no vehicle carrying the phase's required construction skill (CRANE/GROUNDWORKS/ASPHALT_LAYING/etc.) is assigned and parked at its station slot, even if all materials are on site. [SUBSTRATE: ABSENT — greenfield; Vehicle (transportation/vehicle.rs:34-44) carries no skill/economic fields at all per audit §2] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0003`
- AC-4: The project exposes a bottleneck field reporting no-material / no-machine / no-worker, readable by the same UI/data surface that reports production bottlenecks. [SUBSTRATE: PARTIAL — recipe_should_produce returns a bare bool with no reason (souls/goods_company.rs:36-39), so the shape exists but reason-surfacing is ABSENT per audit §3] · impact:`cross-surface` · seam:`app-level` · scenario:`SCENARIO-0003`
- AC-5: A phase makes zero progress while its assigned worker count sourced from the office's WORKERS pool is below the phase's requirement, and its bottleneck reports no-worker distinctly from no-machine. [SUBSTRATE: ABSENT — greenfield; no worker-sourcing-to-phase mechanism exists, and bottleneck today is a bare bool per audit §3] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0003`
- AC-6: Construction material accounting is recomputed at the medium simulation-clock frequency while assigned vehicle movement to/from the site is simulated at high frequency — the two subsystems never share a single tick rate. [SUBSTRATE: OURS — spec/construction.md:119 names this dual-frequency requirement; architecture/simulation-clock.md defines the tiers, no construction system exists yet to check against] · impact:`local` · seam:`process-level` · scenario:`SCENARIO-0003`

**Sources:**
- `spec/construction.md:27-30`
- `spec/construction.md:37-56`
- `spec/construction.md:72-74`
- `spec/construction.md:100-119`

**Status:** pending

## STORY-0004

**Epic:** EPIC-001 — Construction Process
**Title:** Derive construction phase duration from assigned machine throughput, never a fixed timer

**As a** the simulation
**I want** phase completion time to equal phase_work divided by the sum of assigned vehicles' matching skill throughput
**So that** more or better machines finish a phase faster, and a phase with no matching machine never completes on its own

**Acceptance criteria:**
- AC-1: Assigning a second crane to a structure phase reduces its remaining completion time versus one crane, all else equal, in proportion to the added throughput; a vehicle's declared skill throughput falls within its class's spec-named range (cranes 21-95, groundworks 15-37, rolling 18-27). [SUBSTRATE: UNAUDITED — greenfield; nearest existing pattern, Recipe { duration } (prototypes/src/types/recipe.rs:35-47), is a fixed duration independent of assigned worker count and is not the throughput-summed law this spec requires] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0005`
- AC-2: A phase whose material bill is fully delivered but which has zero matching vehicles assigned makes no progress indefinitely — it stalls, it does not fail or time out. [SUBSTRATE: ABSENT — greenfield; contrasts with CS1's rejected fixed constructionTime ramp cited in spec/construction.md:57-66] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0005`
- AC-3: The building's rendered construction progress (a 0-255 visual ramp) moves only when phase_work / Σskill actually advances; advancing simulation ticks with zero eligible vehicles assigned leaves the render value unchanged, never advancing on elapsed time alone. [SUBSTRATE: OURS — spec/construction.md:119 requires the ramp be driven by real material/machine state, explicitly rejecting CS1's timer-driven m_constructState, no such render state exists yet] · impact:`local` · seam:`integration` · scenario:`SCENARIO-0005`

**Sources:**
- `spec/construction.md:57-66`
- `spec/construction.md:119`

**Status:** pending

## STORY-0005

**Epic:** EPIC-001 — Construction Process
**Title:** Reuse the production factor-gate model for construction phases

**As a** a designer/engineer implementing construction
**I want** each construction phase to behave like a production recipe (inputs + machine-work -> phase-complete) sharing code with production's bottleneck logic
**So that** construction stalls and production stalls are diagnosed and displayed through one shared mechanism, not two parallel ones

**Acceptance criteria:**
- AC-1: The existing Recipe type (consumption, production, duration) is reused or extended as the per-phase bill-of-quantities representation rather than a parallel type invented from scratch. [SUBSTRATE: PROVIDED — Recipe exists with consumption/production/duration, prototypes/src/types/recipe.rs:35-47, though it lacks per-material work-skill matching needed for construction] · impact:`none` · seam:`unit`
- AC-2: A construction phase's stall detection calls the same underlying gating primitive as a production recipe's producibility check, extended to report a reason rather than a bare bool. [SUBSTRATE: PARTIAL — souls/goods_company.rs:36-39 recipe_should_produce is boolean/all-or-nothing today, not yet the multiplicative Liebig or reason-surfacing model per audit §3 'Liebig multiplicative bottleneck: PARTIAL', 'Bottleneck reason surfaced to player: ABSENT'] · impact:`cross-surface` · seam:`integration`

**Sources:**
- `spec/construction.md:50-56`
- `spec/construction.md:75-82`

**Status:** pending

## STORY-0006

**Epic:** EPIC-001 — Construction Process
**Title:** Progress road and infrastructure construction through its own physical phase pipeline

**As a** the simulation
**I want** a road/infrastructure segment to progress through its own phase sequence — earthworks, sub-base (gravel), paving (concrete), surfacing (asphalt), markings, open — worked in order by GROUNDWORKS, then ASPHALT_LAYING/ROLLING vehicles, parallel to but distinct from the building phase table
**So that** the player can watch a highway visibly progress grading, laying, surfacing — the mechanic the project is named for — and see exactly where it stalls, instead of the road appearing the instant it is drawn

**Acceptance criteria:**
- AC-1: A road/infrastructure construction project advances through its own ordered phase sequence — earthworks, sub-base, paving, surfacing, markings, open — distinct from the building phase table, and a later phase cannot begin while its predecessor is incomplete. [SUBSTRATE: CONFLICTS — Road::make (map/objects/road.rs) materialises a road fully and instantly today, no phase sequence of any kind exists for roads; spec/construction.md:53 names this pipeline OURS and 'the mechanic the project is named for'] · impact:`journey` · seam:`integration` · scenario:`SCENARIO-0014`
- AC-2: The sub-base phase consumes gravel and the surfacing phase consumes asphalt, worked respectively by GROUNDWORKS-then-ASPHALT_LAYING-then-ROLLING skilled vehicles in that order; a phase makes zero progress while its required material class has not been delivered, resuming automatically once delivered. [SUBSTRATE: ABSENT — greenfield; no road material-gating or vehicle-skill matching exists, spec/construction.md:53 names the material and vehicle-skill sequence explicitly] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0014`
- AC-3: TUNNELING is available as a special earthworks-phase variant substituting for ordinary earthworks when a road/infrastructure project is declared as underground work. [SUBSTRATE: ABSENT — greenfield; spec/construction.md:51 names TUNNELING as a special earthworks variant for underground work, no such variant exists in the fork] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0014`

**Sources:**
- `spec/construction.md:41-56`

**Status:** pending