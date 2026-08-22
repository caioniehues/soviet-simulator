# EPIC-027 — Machinery and skilled labour (greenfield)

**Summary:** Machinery and skilled labour (greenfield)
**Stories:** STORY-0116, STORY-0117
**Primary sources:** `spec/production.md`
**Status:** 0/2 done

## STORY-0116

**Epic:** EPIC-027 — Machinery and skilled labour (greenfield)
**Title:** Let machinery condition throttle production output

**As a** planner
**I want** a factory's equipment presence and condition to act as an independent production factor
**So that** wear becomes a maintenance decision the player manages, not an invisible constant

**Acceptance criteria:**
- AC-1: Today a recipe only tracks n_workers; no machinery/equipment presence or condition field exists on any building or recipe type. [SUBSTRATE: ABSENT — single n_workers tier only, per audit §3] · impact:`none` · seam:`unit`
- AC-2: A recipe may declare workingVehiclesNeeded (or equivalent machinery requirement); f_machinery contributes multiplicatively to output_rate exactly as the other factors do, and missing equipment reduces or zeroes the rate. [SUBSTRATE: ABSENT — greenfield] · impact:`local` · seam:`unit`

**Sources:**
- `spec/production.md:14-27`
- `spec/production.md:73-79`

**Status:** pending

## STORY-0117

**Epic:** EPIC-027 — Machinery and skilled labour (greenfield)
**Title:** Gate advanced recipes on skilled/educated labour separately from general labour

**As a** planner
**I want** some recipes to require a second, educated-labour tier (professorsNeeded) alongside ordinary workersNeeded
**So that** high-tech production chains distinguish general staffing shortfall from a shortage of skilled labour specifically

**Acceptance criteria:**
- AC-1: Today Recipe carries a single workforce tier (n_workers); no second skilled/educated labour field exists. [SUBSTRATE: ABSENT — single n_workers tier only, per audit §3] · impact:`none` · seam:`unit`
- AC-2: A recipe may optionally declare professorsNeeded as a second labour requirement; a shortfall in skilled labour throttles output independently of (and multiplicatively with) the ordinary labour factor, and the bottleneck reason (per the legibility story) distinguishes the two. [SUBSTRATE: ABSENT — greenfield] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0099`

**Sources:**
- `spec/production.md:39-64`
- `spec/production.md:96-108`

**Status:** pending