# EPIC-022 — Byproducts and waste (greenfield)

**Summary:** Byproducts and waste (greenfield)
**Stories:** STORY-0084, STORY-0085, STORY-0086
**Primary sources:** `spec/production.md`
**Status:** 0/3 done

## STORY-0084

**Epic:** EPIC-022 — Byproducts and waste (greenfield)
**Title:** Let recipes emit byproducts alongside their primary outputs

**As a** planner
**I want** a recipe to produce sewage, air/ground pollution, or solid waste (ash) as a side effect of its primary output
**So that** production isn't clean — dirty chains have a physical consequence the player must route or absorb, not a free externality

**Acceptance criteria:**
- AC-1: Today Recipe has only consumption/production fields; no byproducts, pollutionTier, or waste emission field exists anywhere in the recipe type. [SUBSTRATE: ABSENT — prototypes/src/types/recipe.rs, per audit §3] · impact:`none` · seam:`unit`
- AC-2: A recipe may declare a byproducts list (e.g. sewage rate, ash quantity) that is emitted every production cycle alongside its primary outputs, using the same output mechanism (must go somewhere — a byproduct with no output-space capacity throttles the recipe exactly like a primary output would). [SUBSTRATE: ABSENT — greenfield] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0093`
- AC-3: A recipe may declare a categorical pollutionTier (small | medium | high) distinct from any numeric byproduct, matching the confirmed W&R POLLUTION_* token shape. [SUBSTRATE: ABSENT — greenfield] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0093`
- AC-4: A recipe's sewage byproduct (declared via a numeric sewage rate) is routed through a network connection to downstream sewage infrastructure, distinct from the bucket-storage mechanism used for solid waste byproducts like ash — sewage never occupies an output-storage slot. [SUBSTRATE: ABSENT — greenfield; spec/production.md:92 ($CONNECTION_SEWAGE_OUTPUT)] · impact:`local` · seam:`integration` · scenario:`SCENARIO-0093`

**Sources:**
- `spec/production.md:90-95`

**Status:** pending

## STORY-0085

**Epic:** EPIC-022 — Byproducts and waste (greenfield)
**Title:** Require waste to be physically stored and hauled, not vanish

**As a** planner
**I want** solid waste byproducts (e.g. ash) to occupy real output storage and require freight to clear, exactly like any other good
**So that** waste follows the same nothing-teleports rule as every other resource in the economy

**Acceptance criteria:**
- AC-1: A waste byproduct accumulates in the producing building's output storage and is subject to the same output-space backpressure gate as primary outputs — a full waste buffer halts the recipe exactly like a full goods buffer does. [SUBSTRATE: ABSENT — greenfield; depends on byproducts existing at all] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0093`

**Sources:**
- `spec/production.md:90-95`

**Status:** pending

## STORY-0086

**Epic:** EPIC-022 — Byproducts and waste (greenfield)
**Title:** Let recipes recover a fractional yield of material from waste inputs

**As a** planner
**I want** a recycling recipe to consume a waste class as an input and recover only a stated fraction of it as usable output
**So that** recycling is the physical reverse of the waste-byproduct chain, not a free 1:1 conversion

**Acceptance criteria:**
- AC-1: Today no recipe field represents consuming a waste class at a fractional recovery yield; waste can only be produced as a byproduct, never consumed and partially recovered. [SUBSTRATE: ABSENT — greenfield; spec/production.md:63,94] · impact:`none` · seam:`unit`
- AC-2: A recycling recipe may declare a waste input class and a recovery yield fraction (0..1), e.g. waste_steel at 0.98 or waste_plastic at 0.9; consuming 100 units of the declared waste class at yield 0.98 produces exactly 98 units of recovered output, not 100. [SUBSTRATE: ABSENT — greenfield; spec/production.md:63 ($WASTE_EXTRACTION waste_steel 0.98)] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0098`

**Sources:**
- `spec/production.md:63`
- `spec/production.md:94`

**Status:** pending