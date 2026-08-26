# EPIC-004 — Zoning & Siting

**Summary:** Zoning & Siting
**Stories:** STORY-0013, STORY-0014, STORY-0015, STORY-0016
**Primary sources:** `spec/zoning.md`
**Status:** 0/4 done

## STORY-0013

**Epic:** EPIC-004 — Zoning & Siting
**Title:** Stop roads from auto-spawning speculative building lots

**As a** the planner
**I want** building a road segment to never generate building lots along it as a side effect
**So that** nothing appears on the map that the planner did not explicitly site, matching the accepted-2026-08-22 decision to disable Lot::generate_along_road entirely

**Acceptance criteria:**
- AC-1: Building any road segment produces zero new Lot entities as a side effect, immediately and after subsequent ticks. [SUBSTRATE: CONFLICTS — Lot::generate_along_road (map/objects/lot.rs:59-104) currently auto-spawns randomly-sized (20/30/40m) lots along every road built; decision 2026-08-22 is to disable this entirely, per audit §2 and the task brief] · impact:`journey` · seam:`integration` · scenario:`SCENARIO-0001`
- AC-2: Disabling the auto-spawn call site does not remove or break any explicit/manual lot-siting path the planner uses, if one exists elsewhere in map/objects/lot.rs. [SUBSTRATE: UNAUDITED — audit did not enumerate other callers of Lot construction] · impact:`local` · seam:`integration` · scenario:`SCENARIO-0001`

**Sources:**
- `spec/zoning.md:10-13`
- `spec/zoning.md:24-27`

**Status:** pending

## STORY-0014

**Epic:** EPIC-004 — Zoning & Siting
**Title:** Author land-use zoning as a planning overlay, never a spawn trigger

**As a** the planner
**I want** to paint district-level land-use polygons (residential/industrial/agricultural/mixed) that constrain what may be sited there and surface mismatches
**So that** zoning expresses plan intent — a plan not yet fulfilled, not latent market demand — without ever causing a building to exist on its own

**Acceptance criteria:**
- AC-1: Painting a zoning polygon over empty land creates no building entity and no ConstructionProject, immediately or on any later tick. [SUBSTRATE: ABSENT — greenfield, no zoning-as-overlay data structure exists. Note: Building.zone (map/objects/building.rs:49,78) is an unrelated per-company production-footprint polygon and must not be conflated with city zoning, per audit §2] · impact:`journey` · seam:`integration` · scenario:`SCENARIO-0009`
- AC-2: A district zoned residential but left empty persists indefinitely with no timer or probability roll ever causing a building to spawn there — contrasting directly with CS1's `rand(100) < demand` spawn gate. [SUBSTRATE: ABSENT — greenfield; the anti-pattern (ZoneBlock.cs:1156-1177 per spec/zoning.md:18-20) has no analog in this codebase to begin with, so this is a requirement to never introduce one] · impact:`journey` · seam:`e2e` · scenario:`SCENARIO-0009`
- AC-3: Siting a building whose function type mismatches its district's declared land-use is rejected or flagged per a defined policy, never silently rewritten to match. [SUBSTRATE: UNAUDITED — policy explicitly left open in spec/zoning.md:35-40 ('Does zone mismatch ever force anything... or is it advisory only?')] · impact:`local` · seam:`integration` · scenario:`SCENARIO-0009`
- AC-4: Independent of whatever mismatch-enforcement policy is chosen, a building whose function mismatches its district's rezoned land-use is never automatically demolished or removed as a consequence of the mismatch alone. [SUBSTRATE: OURS — spec/zoning.md:38 confirms CS1's auto-despawn-on-mismatch is rejected, even though the broader enforcement policy itself is left open] · impact:`journey` · seam:`integration` · scenario:`SCENARIO-0009`

**Sources:**
- `spec/zoning.md:28-34`
- `spec/zoning.md:35-40`

**Status:** pending

## STORY-0015

**Epic:** EPIC-004 — Zoning & Siting
**Title:** Validate building placement against a physical siting checklist, not a spawn grid

**As a** the planner
**I want** placement at my cursor to be validated against flatness, network adjacency, in-bounds, unoccupied, and utility reach, evaluated at the exact footprint I chose
**So that** illegal placements are rejected before any construction project starts, without reintroducing CS1's road-adjacent grid-cell zoning swarm

**Acceptance criteria:**
- AC-1: Attempting to site a building on land that fails the flatness/adjacency/occupancy/utility-reach checklist is rejected and creates no ConstructionProject. [SUBSTRATE: ABSENT — greenfield; no siting validator exists in the fork today, and it must specifically avoid the road-adjacent ZoneBlock-grid pattern (ZoneBlock.cs:6-56, spec/zoning.md:16-17) that the fork's own Lot::generate_along_road (map/objects/lot.rs:59-104) structurally resembles per audit §2] · impact:`local` · seam:`integration` · scenario:`SCENARIO-0010`
- AC-2: The checklist evaluates at the cursor's exact chosen footprint, not against a fixed grid cell or the 4-cell/32m depth cap CS1 imposes. [SUBSTRATE: UNAUDITED — no placement-validation code exists to check against] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0010`

**Sources:**
- `spec/zoning.md:16-23`
- `spec/zoning.md:28-34`

**Status:** pending

## STORY-0016

**Epic:** EPIC-004 — Zoning & Siting
**Title:** Surface a shortage dashboard instead of driving an autospawn demand loop

**As a** the planner
**I want** housing/jobs/service shortage numbers (homeless minus vacancies, empty jobs minus unemployed, service coverage gaps) computed and displayed
**So that** I decide what to build next from real numbers, with nothing ever built automatically in response to them

**Acceptance criteria:**
- AC-1: A rising homeless-minus-vacancies number never itself triggers a building to be created, at any value. [SUBSTRATE: ABSENT — no RCI-style spawner exists in the fork to begin with; the underlying number itself is ABSENT since households and needs beyond food are unmodelled, audit §5 'Needs as 0..1 satisfaction: ABSENT', 'Households: ABSENT'] · impact:`journey` · seam:`e2e`
- AC-2: The dashboard's housing-shortage number is read from the same housing-queue data structure other systems use (single source of truth), not a separately tracked demand scalar that can drift from it. [SUBSTRATE: ABSENT — greenfield, no housing queue exists yet per audit §5] · impact:`local` · seam:`integration`

**Sources:**
- `spec/zoning.md:28-34`

**Status:** pending