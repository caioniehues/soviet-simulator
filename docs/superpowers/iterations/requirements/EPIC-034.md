# EPIC-034 — Weather and climate (greenfield)

**Summary:** Weather and climate (greenfield)
**Stories:** STORY-0138
**Primary sources:** `spec/heating.md`
**Status:** 0/1 done

## STORY-0138

**Epic:** EPIC-034 — Weather and climate (greenfield)
**Title:** Provide a deterministic weather and climate model

**As a** planner and every system that depends on outdoor conditions
**I want** a temperature signal T(t) with an annual and diurnal cycle that is deterministic, save/load-stable, and updated on an authored cadence
**So that** heat demand and other weather-driven systems have a real, reproducible prerequisite to build on

**Acceptance criteria:**
- AC-1: A temperature signal T(t) exists and combines an annual cycle (seasonal swing) with a diurnal cycle (day/night swing) rather than a flat constant or a seasonal flag. [SUBSTRATE: ABSENT — greenfield; zero hits for weather|climate|temperature|season across simulation/src] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0141`
- AC-2: T(t) is fully deterministic under a fixed seed: the simulation's per-tick state-hash harness produces identical hashes across repeated runs with weather enabled. [SUBSTRATE: ABSENT — greenfield; zero hits for weather|climate|temperature|season across simulation/src] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0141`
- AC-3: Weather state survives a save/load round-trip: T(t) resumes from the exact point it was saved at, not from a re-seeded or reset clock. [SUBSTRATE: ABSENT — greenfield; zero hits for weather|climate|temperature|season across simulation/src] · impact:`local` · seam:`integration` · scenario:`SCENARIO-0141`
- AC-4: Weather state updates on an authored cadence (a fixed tick interval), not once at world creation and never again. [SUBSTRATE: ABSENT — greenfield; zero hits for weather|climate|temperature|season across simulation/src] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0141`

**Sources:**
- `spec/heating.md:26-36`

**Status:** pending