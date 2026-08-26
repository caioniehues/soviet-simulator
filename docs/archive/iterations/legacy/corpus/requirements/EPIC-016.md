# EPIC-016 — Persistent Identity & Lifecycle

**Summary:** Persistent Identity & Lifecycle
**Stories:** STORY-0072, STORY-0073, STORY-0074, STORY-0075
**Primary sources:** `docs/egregoria-substrate-audit.md`, `spec/citizens.md`, `spec/households.md`, `spec/needs.md`
**Status:** 0/4 done

## STORY-0072

**Epic:** EPIC-016 — Persistent Identity & Lifecycle
**Title:** Protect persistent citizen identity across save/load

**As a** Planner
**I want** every citizen's identity and personal state to survive a save/load cycle unchanged
**So that** I can follow a specific person across sessions instead of a respawned, fungible population

**Acceptance criteria:**
- AC-1: Saving then loading the simulation reproduces every HumanEnt's PersonalInfo fields byte-identical to the pre-save state. [SUBSTRATE: PROVIDED — souls/human.rs PersonalInfo in HumanEnt, whole sim serialized via CompressedBincode] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0056`
- AC-2: Two citizens with identical attribute values (age, education, etc.) remain distinguishable by stable entity/soul reference after a reload — souls are not fungible. [SUBSTRATE: PROVIDED — HumanEnt entity identity via ECS, egregoria-substrate-audit.md sec.5] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0056`

**Sources:**
- `spec/citizens.md:22-35`
- `docs/egregoria-substrate-audit.md:119-129`

**Status:** pending

## STORY-0073

**Epic:** EPIC-016 — Persistent Identity & Lifecycle
**Title:** Model health, sickness and hospital-capacity resolution

**As a** Planner
**I want** citizen health to be derived from physical service inputs and sickness to be a resolvable service event
**So that** neglecting people (no water/sewage/care coverage) physically reduces output through a legible causal chain

**Acceptance criteria:**
- AC-1: Citizen health recomputes from physical inputs only (water/sewage access, pollution, garbage, care coverage vs. age-phase requirement) — money never appears in the calculation. [SUBSTRATE: ABSENT — greenfield; HumanEnt has no health or wellbeing field today, only age set once at spawn] · impact:`journey` · seam:`integration` · scenario:`SCENARIO-0064`
- AC-2: A sustained low-health streak triggers a sickness roll; a sick citizen posts a hospital-transport need whose resolution consumes hospital capacity. [SUBSTRATE: ABSENT — greenfield] · impact:`journey` · seam:`integration` · scenario:`SCENARIO-0064`
- AC-3: While sick, a citizen's economic life (work attendance, shopping trips) is frozen until the sickness resolves. [SUBSTRATE: ABSENT — greenfield] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0064`
- AC-4: A citizen's work efficiency scales continuously across the 10-100% range as a function of health, not as a binary sick/not-sick cutoff. [SUBSTRATE: ABSENT — greenfield, CS1 CONFIRMED table `workEfficiency(health)` per spec/citizens.md:61] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0064`

**Sources:**
- `spec/citizens.md:48-52`
- `spec/needs.md:13-21`

**Status:** pending

## STORY-0074

**Epic:** EPIC-016 — Persistent Identity & Lifecycle
**Title:** Progress citizen age and introduce death without deleting the household

**As a** Planner
**I want** citizen age to advance over simulated time and citizens to die within a widened age window, freeing a household slot rather than dissolving the household
**So that** the population has a real demographic lifecycle instead of a frozen age field

**Acceptance criteria:**
- AC-1: A citizen's age field increases as simulated time passes. [SUBSTRATE: ABSENT — greenfield; age is set once at spawn and never incremented, per egregoria-substrate-audit.md sec.5] · impact:`journey` · seam:`unit` · scenario:`SCENARIO-0065`
- AC-2: Death occurs by an age window widened by poor health and a small accident chance; on death the citizen's household persists with the slot freed, not dissolved. [SUBSTRATE: ABSENT — greenfield] · impact:`journey` · seam:`integration` · scenario:`SCENARIO-0065`
- AC-3: Population aging or decline never halts or ends the simulation — it only produces a visible demographic strain signal, consistent with the project's no-game-over principle. [SUBSTRATE: ABSENT — greenfield; no death/lifecycle exists today so no failure-mode behaviour to regress against] · impact:`journey` · seam:`app-level` · scenario:`SCENARIO-0065`
- AC-4: A citizen transitions through coarse lifecycle stages (child -> student -> worker -> pensioner) at named age thresholds, gaining/losing school- and job-eligibility at each transition, rather than a fixed life-stage that never changes as age increases. [SUBSTRATE: ABSENT — greenfield, CS1 shape (AgeGroup thresholds a la 15/45/90/180/240) per spec/citizens.md:32,70] · impact:`journey` · seam:`integration` · scenario:`SCENARIO-0065`

**Sources:**
- `spec/citizens.md:68-69`
- `spec/households.md:72-76`

**Status:** pending

## STORY-0075

**Epic:** EPIC-016 — Persistent Identity & Lifecycle
**Title:** Grow the population through household birth

**As a** Planner
**I want** eligible couples with a free household slot to have a per-step chance of a birth, boosted by childcare access
**So that** population growth is a real demographic event coupled to service coverage, not a fixed/absent counter

**Acceptance criteria:**
- AC-1: A birth can occur only when a household has a present couple, both adults, and a free member slot; a household missing any of these three preconditions never produces a birth. [SUBSTRATE: ABSENT — greenfield, CS1 CONFIRMED birth gate per spec/households.md:54] · impact:`journey` · seam:`integration` · scenario:`SCENARIO-0068`
- AC-2: The per-step birth chance rises from a baseline of 1/12 to 1/8 when the household has childcare access, so improving childcare coverage measurably raises the observed birth rate. [SUBSTRATE: ABSENT — greenfield, CS1 CONFIRMED constants per spec/households.md:54] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0068`

**Sources:**
- `spec/households.md:54`

**Status:** pending