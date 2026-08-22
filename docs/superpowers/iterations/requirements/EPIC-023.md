# EPIC-023 — Liebig bottleneck (build)

**Summary:** Liebig bottleneck (build)
**Stories:** STORY-0098, STORY-0099, STORY-0100, STORY-0101, STORY-0102, STORY-0103, STORY-0104
**Primary sources:** `spec/production.md`
**Status:** 0/7 done

## STORY-0098

**Epic:** EPIC-023 — Liebig bottleneck (build)
**Title:** Scale output continuously with staffing fraction, not linearly

**As a** planner
**I want** understaffed factories to lose output along the diminishing-returns curve the spec adopts, not a flat linear ratio
**So that** partial staffing behaves like the reference model (full staffing + full power + no iron = no steel; the curve, not just presence/absence, communicates how badly understaffed a factory is)

**Acceptance criteria:**
- AC-1: Today's labour factor is workers_present/workers_needed applied linearly. [SUBSTRATE: PARTIAL — souls/goods_company.rs (labour scaling is linear, per audit §3)] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0084`
- AC-2: The labour factor must instead follow the adopted curve rate = 2·e − 200·e/(staffFrac+100) (e = worker efficiency), producing a strictly smaller output than the linear model at all staffing fractions strictly between 0 and 1. [SUBSTRATE: ABSENT — greenfield, curve not implemented; spec adopts CS1's PrivateBuildingAI.cs:397] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0084`
- AC-3: At staffFrac = 0 the labour factor is 0 (zero workers → zero output) and at staffFrac = 1 it is 1 (full staffing → no labour penalty). [SUBSTRATE: ABSENT — greenfield curve boundary condition] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0084`
- AC-4: A recipe may opt out of per-worker rate scaling and declare its production rate as a whole-building constant unaffected by staffFrac curve math — the documented water-well exception — while every other recipe still applies the per-worker curve from AC-2. [SUBSTRATE: ABSENT — greenfield; spec/production.md:32 (water_well_*.ini exception)] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0084`

**Sources:**
- `spec/production.md:28-38`
- `spec/production.md:73-79`

**Status:** pending

## STORY-0099

**Epic:** EPIC-023 — Liebig bottleneck (build)
**Title:** Throttle output continuously with available power instead of a binary blackout gate

**As a** planner
**I want** a brownout (partial power deficit) to reduce a factory's output proportionally rather than the plant running at full rate until power fails, then dropping to zero
**So that** power reads as a graduated physical constraint the player can manage (shed load, prioritize feeds) rather than an on/off trap

**Acceptance criteria:**
- AC-1: Today power is a binary gate: any network blackout stops output entirely; there is no partial-power state. [SUBSTRATE: PARTIAL — souls/goods_company.rs:95-101, binary blackout gate] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0085`
- AC-2: f_power must scale continuously with electricity_available (0..1), matching W&R's CONSUMPTION_PER_SECOND gating-input model: at partial availability output is throttled proportionally, not merely on or off. [SUBSTRATE: ABSENT — greenfield continuous power factor] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0085`
- AC-3: Idle-draw fallback: a factory not currently producing due to another gate still draws a reduced power fraction (idle-vs-producing modal ~0.4) rather than zero or full draw. [SUBSTRATE: ABSENT — greenfield, W&R ELETRIC_WITHOUT_WORKING_FACTOR token] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0085`
- AC-4: A separate lighting-off power draw fraction (distinct from the idle-vs-producing fallback in AC-3) reduces a factory's power draw when its lighting is disabled, independent of whether it is currently producing. [SUBSTRATE: ABSENT — greenfield; spec/production.md:58 ($ELETRIC_WITHOUT_LIGHTING_FACTOR)] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0085`

**Sources:**
- `spec/production.md:28-38`
- `spec/production.md:73-79`

**Status:** pending

## STORY-0100

**Epic:** EPIC-023 — Liebig bottleneck (build)
**Title:** Throttle output continuously with scarcest input stock instead of an all-or-nothing stall

**As a** planner
**I want** a factory running low on one input to slow down proportionally to that input's remaining stock, not run at full rate until the input hits zero
**So that** the scarcest-factor bottleneck (Liebig's law) reads as a rate decay the player can see coming, not a cliff

**Acceptance criteria:**
- AC-1: Today recipe_should_produce is a boolean stall: production runs at full rate while every input is above its threshold and stops entirely the tick an input hits zero. [SUBSTRATE: PARTIAL — recipe_should_produce is a bare bool, souls/goods_company.rs] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0086`
- AC-2: f_inputs must become min over all inputs of (available/required), each input independently throttling the rate to its own stock fraction; a single input at zero drives f_inputs (and thus output) to exactly 0. [SUBSTRATE: ABSENT — greenfield, spec adopts CS1's ProcessingFacilityAI.cs:471 per-input throttle] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0086`
- AC-3: Two inputs at different stock fractions (e.g. 80% and 30% of required) yield a rate bounded by the lower (30%) factor, not the higher or an average. [SUBSTRATE: ABSENT — greenfield min() semantics] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0086`

**Sources:**
- `spec/production.md:28-38`
- `spec/production.md:73-79`

**Status:** pending

## STORY-0101

**Epic:** EPIC-023 — Liebig bottleneck (build)
**Title:** Combine all production factors multiplicatively so the scarcest factor wins

**As a** planner
**I want** the realized output rate to be the product of every independent factor (labour × power × inputs × machinery × water), not the minimum of any single one
**So that** compounding deficiencies compound their effect exactly as physical causality demands — full staffing and full power cannot compensate for a half-supplied input

**Acceptance criteria:**
- AC-1: output_rate = base_rate × f_labour × f_power × f_inputs × f_machinery × f_water_quality × f_output_space, where each f_x ∈ [0,1]; the final rate equals the arithmetic product of all factor values, not their minimum. [SUBSTRATE: ABSENT — greenfield; today the composed multiplicative formula does not exist, only the independent binary/boolean/linear gates cited above] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0087`
- AC-2: Any single factor at 0 drives the composed rate to exactly 0 regardless of the other factors' values (full staffing + full power + zero iron = zero steel). [SUBSTRATE: ABSENT — greenfield] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0087`

**Sources:**
- `spec/production.md:14-27`

**Status:** pending

## STORY-0102

**Epic:** EPIC-023 — Liebig bottleneck (build)
**Title:** Surface the active bottleneck reason to the player

**As a** planner
**I want** to see, per stalled or throttled factory, which single factor (labour, power, a specific input, output space) is currently limiting it
**So that** I can act on the cause instead of guessing why output dropped — a past playtest verdict found critical warnings invisible, and a bare bool return today gives the player nothing to look at

**Acceptance criteria:**
- AC-1: recipe_should_produce returns only a bare bool today; no bottleneck identity is captured or stored anywhere in the production path. [SUBSTRATE: ABSENT — souls/goods_company.rs, recipe_should_produce bare bool] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0088`
- AC-2: ProductionState must record a bottleneck field naming the single lowest-value active factor (e.g. NoResources(item), NoPower, NoWorkers, NoPlaceForGoods) whenever currentRate0to1 < 1, updated on every factor re-evaluation. [SUBSTRATE: ABSENT — greenfield ProductionState.bottleneck] · impact:`local` · seam:`integration` · scenario:`SCENARIO-0088`
- AC-3: Selecting a throttled or stalled building in the UI displays its current bottleneck reason in player-readable form (e.g. "blocked: no iron"), visible without opening a debug view. [SUBSTRATE: ABSENT — greenfield; audit notes no severity-tiered notification system exists, only a single-slot ErrorTooltip] · impact:`journey` · seam:`app-level` · scenario:`SCENARIO-0088`

**Sources:**
- `spec/production.md:66-87`
- `spec/production.md:109-124`

**Status:** pending

## STORY-0103

**Epic:** EPIC-023 — Liebig bottleneck (build)
**Title:** Gate production below a required water purity threshold

**As a** planner
**I want** a recipe that requires a minimum water purity to be blocked outright when its water input falls below that threshold
**So that** water quality reads as a distinct binary gate among the six named production factors, not folded invisibly into the generic multiplicative formula

**Acceptance criteria:**
- AC-1: Today no waterQualityMin field, threshold check, or f_water_quality computation exists anywhere in the recipe or production types. [SUBSTRATE: ABSENT — greenfield; spec/production.md:119,143] · impact:`none` · seam:`unit`
- AC-2: A Recipe may declare waterQualityMin (0..1); when the recipe's water input quality is below waterQualityMin, f_water_quality is 0 and the recipe is blocked outright (not merely throttled proportionally) regardless of every other factor's value. [SUBSTRATE: ABSENT — greenfield; spec/production.md:45,82] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0096`
- AC-3: When the recipe has no waterQualityMin declared, or the supplied water quality meets or exceeds it, f_water_quality is exactly 1 and contributes no penalty to the composed output_rate. [SUBSTRATE: ABSENT — greenfield; spec/production.md:14-27] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0096`

**Sources:**
- `spec/production.md:14-27`
- `spec/production.md:45`
- `spec/production.md:82`
- `spec/production.md:119-127`

**Status:** pending

## STORY-0104

**Epic:** EPIC-023 — Liebig bottleneck (build)
**Title:** Recompute production factors when a gating input changes, not only on a fixed tick

**As a** planner
**I want** a factory's production rate and bottleneck to update as soon as its labour, power, or input stock changes
**So that** the player sees a stall or recovery promptly instead of waiting up to a full production-frequency period for the sim to notice

**Acceptance criteria:**
- AC-1: Today no change-triggered re-evaluation exists; nothing in the codebase asserts factor recomputation is driven by anything other than the fixed medium-frequency production pass. [SUBSTRATE: ABSENT — greenfield; spec/production.md:131] · impact:`none` · seam:`unit`
- AC-2: A change to a building's present-worker count, available power fraction, or an input's stock level triggers factor re-evaluation (and bottleneck field update) on the tick the change is observed, not only at the next fixed medium-frequency production pass boundary. [SUBSTRATE: ABSENT — greenfield; spec/production.md:131] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0097`

**Sources:**
- `spec/production.md:131`

**Status:** pending