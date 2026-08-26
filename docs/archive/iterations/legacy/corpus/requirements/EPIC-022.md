# EPIC-022 — Recipe machinery (already provided)

**Summary:** Recipe machinery (already provided)
**Stories:** STORY-0093, STORY-0094, STORY-0095, STORY-0096, STORY-0097
**Primary sources:** `docs/adr/0017-a-building-is-its-product.md`, `spec/production.md`
**Status:** 0/5 done

## STORY-0093

**Epic:** EPIC-022 — Recipe machinery (already provided)
**Title:** Verify recipes transform multiple inputs into multiple outputs over duration

**As a** planner
**I want** a building's recipe to consume several distinct inputs and produce several distinct outputs over a fixed duration
**So that** co-product chains like an oil refinery's fuel+bitumen are representable without new engine work

**Acceptance criteria:**
- AC-1: A Recipe can declare 2+ input items and 2+ output items simultaneously, each with independent quantities and a shared production duration. [SUBSTRATE: PROVIDED — prototypes/src/types/recipe.rs:35-47] · impact:`local` · seam:`unit` · scenario:`JOURNEY-0001`
- AC-2: One building type is bound to exactly one recipe (ADR-0017's W&R shape: a building is its product, not a generic machine with a swappable recipe category). [SUBSTRATE: PROVIDED — GoodsCompanyPrototype.recipe: Option<Recipe>, per docs/adr/0017-a-building-is-its-product.md] · impact:`local` · seam:`unit` · scenario:`JOURNEY-0001`

**Sources:**
- `spec/production.md:39-71`
- `docs/adr/0017-a-building-is-its-product.md:1-56`

**Status:** pending

## STORY-0094

**Epic:** EPIC-022 — Recipe machinery (already provided)
**Title:** Verify extraction buildings produce with no consumed inputs

**As a** planner
**I want** a mine or field to output raw material from labour and time alone, consuming no input items
**So that** the base of every supply chain (ore, timber, crops) exists without a phantom input recipe

**Acceptance criteria:**
- AC-1: A Recipe with an empty inputs list is valid and produces output at its declared rate whenever labour/power/output-space gates are satisfied. [SUBSTRATE: PROVIDED — base_mod/companies.lua:56] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0082`

**Sources:**
- `spec/production.md:60-64`

**Status:** pending

## STORY-0095

**Epic:** EPIC-022 — Recipe machinery (already provided)
**Title:** Verify full output storage halts production (the cascade engine)

**As a** planner
**I want** a factory to stop producing once its output buffer is full
**So that** a jammed downstream (freight backed up, warehouse full) visibly and physically stalls the factory instead of goods vanishing or piling up unboundedly

**Acceptance criteria:**
- AC-1: When a recipe's output storage cannot accept another batch, production halts entirely (rate → 0) until space frees, and resumes automatically once it does. [SUBSTRATE: PROVIDED — recipe_should_produce, souls/goods_company.rs:36-39] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0083`

**Sources:**
- `spec/production.md:73-87`

**Status:** pending

## STORY-0096

**Epic:** EPIC-022 — Recipe machinery (already provided)
**Title:** Verify declared workforce is sourced live from present population, not stockpiled

**As a** planner
**I want** a factory's WORKERS_NEEDED figure to be satisfied by whichever citizens are actually present and working that tick
**So that** labour behaves as a physical, non-storable input rather than an inventory the player can bank

**Acceptance criteria:**
- AC-1: Workforce sourcing is computed per-tick from n_workers present at the building, never drawn from a stored labour stock. [SUBSTRATE: PROVIDED — goods_company.rs:25,266-295] · impact:`local` · seam:`integration`

**Sources:**
- `spec/production.md:66-71`

**Status:** pending

## STORY-0097

**Epic:** EPIC-022 — Recipe machinery (already provided)
**Title:** Verify production never checks the treasury before running

**As a** planner
**I want** production to run purely on physical availability (labour, power, inputs, output space)
**So that** the game's one rule holds: nothing is produced merely because money is available, and nothing is blocked merely because money is short

**Acceptance criteria:**
- AC-1: No internal recipe execution path reads or checks Money/treasury balance; production gates are exclusively physical factors. [SUBSTRATE: PROVIDED — internal recipes never touch Money, per audit §3] · impact:`cross-surface` · seam:`integration`

**Sources:**
- `spec/production.md:73-79`

**Status:** pending