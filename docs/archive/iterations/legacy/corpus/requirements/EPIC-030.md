# EPIC-030 — Healthcare — sickness and treatment

**Summary:** Healthcare — sickness and treatment
**Stories:** STORY-0124, STORY-0125, STORY-0126
**Primary sources:** `docs/egregoria-substrate-audit.md`, `spec/healthcare.md`
**Status:** 0/3 done

## STORY-0124

**Epic:** EPIC-030 — Healthcare — sickness and treatment
**Title:** Get sick from unmet needs, not proximity to a hospital

**As a** citizen
**I want** my baseline health to be driven by whether my food/warmth/water needs are actually satisfied, never by a passive coverage-radius bonus
**So that** sickness is a legible consequence of real material shortage rather than an ambient service field

**Acceptance criteria:**
- AC-1: Each human gains a `sick: bool` and `sickSince` field with no equivalent today (`souls/human.rs` only models a hunger-clock utility score, per the audit's People table); no health or wellbeing field exists on any human type. [SUBSTRATE: ABSENT — greenfield, add to HumanEnt] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0109`
- AC-2: Sickness probability rises as a function of unmet need satisfaction (food, warmth, water) accumulated over time, computed from the same need-satisfaction values consumed by other desire modules — never as a function of Euclidean distance to a hospital building. A citizen with zero hospitals within any radius but fully satisfied needs must not become sicker than one near many hospitals with the same needs. [SUBSTRATE: ABSENT — greenfield] · impact:`journey` · seam:`integration` · scenario:`SCENARIO-0109`
- AC-3: Placing a hospital near a citizen with no dispatch or visit ever occurring produces zero change to that citizen's health — proving the passive coverage-field pattern (CS1's `HealthCare` field) was not reintroduced. [SUBSTRATE: ABSENT — greenfield; explicit negative control against dropped CS1 pattern] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0109`

**Sources:**
- `spec/healthcare.md:16-21`

**Status:** pending

## STORY-0125

**Epic:** EPIC-030 — Healthcare — sickness and treatment
**Title:** Treat a sick citizen via dispatch, bed occupancy, and staffed cure rate

**As a** sick citizen
**I want** a reachable, staffed hospital to either send an ambulance for me or receive me when I self-travel, hold me in one of a finite number of beds, and cure me at a rate that depends on real staffing
**So that** treatment capacity is a physically constrained throughput facility, not an instant or radius-based cure

**Acceptance criteria:**
- AC-1: A hospital exposes `beds: u32` initialised to the CS1-derived reference value of 100 (`patientCapacity`) and `occupied[]`; a sick citizen can only begin treatment if `occupied.len() < beds`, otherwise the citizen remains sick and queued — this never hard-fails the simulation. No `Hospital` type, bed field, or patient list exists anywhere in the codebase today. [SUBSTRATE: ABSENT — greenfield] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0110`
- AC-2: A sick citizen either triggers an ambulance dispatch from the nearest hospital with a free bed and a fuelled vehicle, or self-travels to a hospital, mirroring CS1's two-mode `Sick`/`SickMove` loop; the dispatch/self-travel decision resolves on arrival, following the existing arrival-gated resolution pattern already proven for food purchases at `souls/desire/buyfood.rs:50-54`. [SUBSTRATE: ABSENT — greenfield, follow arrival-gated pattern at souls/desire/buyfood.rs:50-54] · impact:`journey` · seam:`e2e` · scenario:`SCENARIO-0110`
- AC-3: Cure rate per tick for an occupied bed scales with the hospital's current staffing ratio against the required two-tier complement (workers 50 + medically-specialised profesors 50, per W&R `hospital.ini:48-50`); zero staff present yields zero cure progress rather than a fixed timer, and a hospital staffed with workers but zero profesors yields a reduced (not full) cure rate since only the profesor tier performs the specialised treatment, matching the CS1 production-rate-gated probabilistic cure. [SUBSTRATE: ABSENT — greenfield] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0110`
- AC-4: A hospital with zero fuel in its `fuelStore` dispatches no ambulances (an unfuelled hospital dispatches nothing, per the W&R vehicle-base grammar); citizens in its catchment fall back to self-travel or remain untreated, degrading outcomes rather than failing the game. [SUBSTRATE: ABSENT — greenfield] · impact:`journey` · seam:`integration` · scenario:`SCENARIO-0110`
- AC-5: Hospital treatment throughput is additionally capped by a serve-rate of 3 patients processed per cycle (`$CITIZEN_ABLE_SERVE`, identical to the university tier's cap), distinct from and tighter than raw bed occupancy — a hospital with 100 free beds still only advances cure progress for at most 3 patients per cycle. [SUBSTRATE: ABSENT — greenfield] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0110`

**Sources:**
- `spec/healthcare.md:22-34`

**Status:** pending

## STORY-0126

**Epic:** EPIC-030 — Healthcare — sickness and treatment
**Title:** Gate hospital cure rate on a physically supplied medicine chain, degrading rather than halting on shortage

**As a** planner
**I want** hospitals to carry a medicine storage demand fed by freight, and low health to visibly reduce a citizen's work output rather than end the game
**So that** medicine becomes a real production/import chain and sickness is a leaner-tranche consequence, never a hard failure

**Acceptance criteria:**
- AC-1: A hospital exposes a `medicineStore` demand fed via the same storage-demand freight grammar as other goods; cure rate is a function of both staffing and `medicineStore` level, so an unsupplied hospital's cure rate degrades smoothly toward (but does not reach) zero rather than halting treatment outright. No medicine item, storage-demand hook on a hospital, or cure-rate formula exists today. [SUBSTRATE: ABSENT — greenfield, insertion point mirrors Market::buy_until at economy/market.rs:161-167] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0114`
- AC-2: A citizen with `sick: true` has reduced work output/production contribution while sick, proportional to duration sick, matching CS1's confirmed health→work-efficiency coupling; this reduces throughput, it never removes the citizen from the simulation or ends the game. [SUBSTRATE: ABSENT — greenfield] · impact:`journey` · seam:`integration` · scenario:`SCENARIO-0114`
- AC-3: Untreated sickness persisting beyond a threshold duration results in death (citizen entity removed, per the CS1-confirmed sickness→death consequence); this is a leaner-tranche/individual-scale outcome — colder homes, smaller workforce, longer queues elsewhere — and never presents as a simulation-ending or player-losing state, per the project's 'never game over' rule. [SUBSTRATE: ABSENT — greenfield; age/death machinery itself is ABSENT per audit §5 ('age set once at spawn, never incremented')] · impact:`journey` · seam:`e2e` · scenario:`SCENARIO-0114`

**Sources:**
- `spec/healthcare.md:36-48`
- `docs/egregoria-substrate-audit.md:147-158`

**Status:** pending