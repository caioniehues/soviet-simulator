# EPIC-027 — Crime — deviance and the justice chain

**Summary:** Crime — deviance and the justice chain
**Stories:** STORY-0097, STORY-0098, STORY-0099, STORY-0100
**Primary sources:** `spec/crime.md`
**Status:** 0/4 done

## STORY-0097

**Epic:** EPIC-027 — Crime — deviance and the justice chain
**Title:** Generate per-building crime pressure from unemployment and unhappiness

**As a** planner
**I want** crime to build up in a building as a function of its occupants' unemployment duration and wellbeing, present from turn one
**So that** crime is a legible, physical consequence of neglect rather than a random dice-roll or a mechanic locked behind unlocking police

**Acceptance criteria:**
- AC-1: Each citizen gains a `crimePropensity` computed as min(rate(unemploymentLength), maxRate(wellbeing)) — unemployment duration bands 0/1/2/3/4/5+ mapping to 10/15/20/25/35/50, wellbeing cap ranging VeryUnhappy=100 down to VeryHappy=40 — with a ~4x multiplier for citizens already flagged `criminal`. No unemployment-duration tracking, wellbeing scalar, or crimePropensity field exists on HumanEnt today. [SUBSTRATE: ABSENT — greenfield] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0116`
- AC-2: Each building accrues a `crimeBuffer` as a randomized sum over its current occupants' `crimePropensity`, increased 25% at night, hard-capped at `occupantCount * 100`; no `crimeBuffer` field exists on any building type today. [SUBSTRATE: ABSENT — greenfield] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0116`
- AC-3: Crime buffer accrual happens from the first simulated tick with zero police buildings placed — there is no gating condition requiring a PoliceStation to exist before crimeBuffer can be non-zero, unlike CS1's unlock-gated version which we explicitly reject. [SUBSTRATE: ABSENT — greenfield; explicit rejection of CS1's police-unlock gate] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0116`

**Sources:**
- `spec/crime.md:16-30`

**Status:** pending

## STORY-0098

**Epic:** EPIC-027 — Crime — deviance and the justice chain
**Title:** Arrest a specific criminal and hold them in a fed prison cell

**As a** planner
**I want** a staffed police station to send an officer who travels to and arrests a specific citizen, then transports and holds them in a real prison cell that must be fed
**So that** crime clearance is a physical logistics loop, not a radius-based coverage debit

**Acceptance criteria:**
- AC-1: A PoliceStation dispatches an officer (a vehicle trip) to a building whose `crimeBuffer` exceeds a threshold; on arrival, a specific citizen is flagged `arrested: bool = true` and transported to a Court to await sentencing (see 'Throttle sentencing through a staffed court between arrest and prison'), then on to Prison once sentenced — mirroring CS1's `ArrestCriminals`/`CriminalMove` path. We keep only this arrest path; the coverage-style patrol-drain-on-arrival mechanic (CS1's second clearance channel) is explicitly dropped. No PoliceStation, Prison, Court, arrest flag, or dispatch logic exists today. [SUBSTRATE: ABSENT — greenfield; explicit rejection of CS1's patrol-debit channel] · impact:`journey` · seam:`e2e` · scenario:`SCENARIO-0118`
- AC-2: A Prison exposes `cells: u32`; an arrested citizen occupies one cell up to capacity, otherwise the arrest is deferred (the citizen stays flagged `arrested` pending a free cell) rather than the simulation failing. [SUBSTRATE: ABSENT — greenfield] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0118`
- AC-3: A Prison carries a `foodDemand` fed via the freight/storage-demand chain (dry + refrigerated, per the W&R prison grammar) and the lowest quality-of-living value in the simulation; an unsupplied prison degrades inmate quality-of-living/health rather than instantly killing or releasing inmates — consistent with the project's never-game-over, leaner-tranche rule. [SUBSTRATE: ABSENT — greenfield, storage-demand grammar mirrors spec/logistics.md pattern] · impact:`journey` · seam:`integration` · scenario:`SCENARIO-0118`
- AC-4: A PoliceStation with zero fuel in its vehicle fuel store dispatches no officer, mirroring the identical unfuelled-hospital-dispatches-nothing rule (`PoliceStation { staff; vehicles+fuel; cells }`); `crimeBuffer` at affected buildings continues accruing undisturbed rather than the simulation stalling or an arrest being silently teleported. [SUBSTRATE: ABSENT — greenfield] · impact:`journey` · seam:`integration` · scenario:`SCENARIO-0118`
- AC-5: A Prison requires worker-guards staffed to admit new inmates (prison is worker-guards only, per the staffed-shell grammar); a prison with zero guards on duty defers admission of arrested citizens — the same deferred-not-dropped behavior as a full prison — rather than accepting inmates unstaffed. [SUBSTRATE: ABSENT — greenfield] · impact:`local` · seam:`integration` · scenario:`SCENARIO-0118`

**Sources:**
- `spec/crime.md:31-38`

**Status:** pending

## STORY-0099

**Epic:** EPIC-027 — Crime — deviance and the justice chain
**Title:** Throttle sentencing through a staffed court between arrest and prison

**As a** planner
**I want** an arrested citizen to await sentencing at a staffed Court whose caseThroughput limits how many cases resolve per cycle, before being transported to prison
**So that** arrest volume cannot outpace the justice system's own capacity to process it, mirroring the police -> court -> prison pipeline the spec adopts

**Acceptance criteria:**
- AC-1: A Court building exposes `staff` and `caseThroughput`; an arrested citizen queues at the Court and is only transported onward to Prison once a court slot processes their case, capped at `caseThroughput` cases resolved per cycle — court capacity throttles sentencing throughput between arrest and prison. No `Court` type, staff field, or caseThroughput field exists anywhere in the codebase today. [SUBSTRATE: ABSENT — greenfield] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0120`
- AC-2: Court staffing follows the adopted staffed-shell grammar (profesor-judges only); a zero-staffed court resolves zero cases per cycle, and arrested citizens accumulate in a pending-sentencing queue rather than being dropped or teleported straight to Prison. This degrades throughput, it never hard-fails the simulation. [SUBSTRATE: ABSENT — greenfield] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0120`

**Sources:**
- `spec/crime.md:41-53`

**Status:** pending

## STORY-0100

**Epic:** EPIC-027 — Crime — deviance and the justice chain
**Title:** Leak state inventory into a shortage-driven black market

**As a** citizen with a chronically unmet need
**I want** a parallel, unofficial allocation channel to satisfy shortages from state inventories at the cost of feeding crime and corruption pressure
**So that** chronic planned-economy shortage has a visible, physical, in-world consequence rather than staying an invisible statistic

**Acceptance criteria:**
- AC-1: A warehouse or district gains a `leakRate` computed from local shortage severity (unmet demand vs. supply) and inversely from local enforcement presence (police staffing/crimeBuffer clearance activity); this entire mechanic — black market, leak rate, shortage-to-crime linkage — has no substrate in either reference game and no code today. [SUBSTRATE: ABSENT — greenfield, this is OURS per spec with no CS1 or W&R precedent] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0123`
- AC-2: Goods leaked via the black market are drawn from real state inventory stock (decrementing an existing warehouse/market quantity), never conjured — satisfying the project's 'nothing teleports' rule; the leaked goods measurably satisfy the receiving citizen's unmet need. [SUBSTRATE: ABSENT — greenfield] · impact:`journey` · seam:`integration` · scenario:`SCENARIO-0123`
- AC-3: Black-market activity in a district increases that district's aggregate crime pressure (feeding back into `crimeBuffer`/enforcement dynamics) without ever producing a hard-fail or game-over state — chronic shortage degrades into visible unofficial redistribution and rising unrest, not simulation termination. [SUBSTRATE: ABSENT — greenfield] · impact:`journey` · seam:`e2e` · scenario:`SCENARIO-0123`

**Sources:**
- `spec/crime.md:43-46`

**Status:** pending