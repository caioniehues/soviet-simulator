# EPIC-014 — Labour Allocation & Workplace Binding

**Summary:** Labour Allocation & Workplace Binding
**Stories:** STORY-0053, STORY-0054, STORY-0055, STORY-0056
**Primary sources:** `docs/egregoria-substrate-audit.md`, `spec/citizens.md`
**Status:** 0/4 done

## STORY-0053

**Epic:** EPIC-014 — Labour Allocation & Workplace Binding
**Title:** Protect fixed workplace binding

**As a** Planner
**I want** a citizen's assigned workplace to stay a stable single reference until an explicit reassignment event
**So that** labour allocation stays plannable instead of reshuffling itself every tick

**Acceptance criteria:**
- AC-1: A citizen's Work.workplace value does not change absent an explicit reassignment event (job loss, workplace destroyed, plan reassignment). [SUBSTRATE: PROVIDED — souls/desire/work.rs:20-26] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0058`
- AC-2: The workplace binding persists unchanged across a save/load cycle. [SUBSTRATE: PROVIDED — souls/human.rs HumanEnt via CompressedBincode] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0058`

**Sources:**
- `spec/citizens.md:37-46`
- `docs/egregoria-substrate-audit.md:119-129`

**Status:** pending

## STORY-0054

**Epic:** EPIC-014 — Labour Allocation & Workplace Binding
**Title:** Replace distance-only job matching with planner-directed, tier-and-commute-aware allocation

**As a** Planner
**I want** job assignment to respect minimum education tier and commute feasibility, not just proximity
**So that** labour allocation is a plannable resource rather than a euclidean-distance market trade

**Acceptance criteria:**
- AC-1: A vacancy is only filled by a citizen who meets the slot's minimum education tier. [SUBSTRATE: CONFLICTS — today a job vacancy is literally traded as ItemID::new("job-opening") matched purely by squared Euclidean distance, souls/human.rs:267-269 and economy/market.rs:216, with no education check at all] · impact:`journey` · seam:`integration` · scenario:`SCENARIO-0063`
- AC-2: A candidate whose commute time (or distance under an authored feasibility threshold) exceeds the allocator's limit is not assigned to that vacancy even if no closer candidate exists. [SUBSTRATE: CONFLICTS — no commute feasibility check exists in the current distance-ranked trade, market.rs:216-219] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0063`
- AC-3: An overqualified citizen may fill a lower-tier slot but a lower-tier citizen is never assigned above their tier; filling below tier costs the citizen wellbeing. [SUBSTRATE: ABSENT — greenfield; no wellbeing field or overqualification cascade exists on HumanEnt today] · impact:`journey` · seam:`integration` · scenario:`SCENARIO-0063`
- AC-4: A citizen's work-attendance probability scales continuously with wellbeing (workProbability(wellbeing)), and sustained low wellbeing raises that citizen's contribution to the crime rate, mirroring the CS1 GetCrimeRate/GetWorkProbability coupling. [SUBSTRATE: ABSENT — greenfield; no wellbeing-driven crime or attendance-probability coupling exists today, CS1 CONFIRMED per spec/needs.md:65] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0063`
- AC-5: Commute trips are timed by a time-of-day probability curve rather than a uniform/random departure time, matching the CS1 commute-timing shape. [SUBSTRATE: ABSENT — greenfield, CS1 CONFIRMED shape per spec/citizens.md:65] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0063`

**Sources:**
- `spec/citizens.md:37-46`
- `docs/egregoria-substrate-audit.md:119-129`

**Status:** pending

## STORY-0055

**Epic:** EPIC-014 — Labour Allocation & Workplace Binding
**Title:** Allocate student seats through a capacity-limited education pipeline

**As a** Planner
**I want** a student seat to be a planned, capacity-limited allocation like a job vacancy, throttled by per-tier throughput
**So that** education is a physical pipeline with real trade-offs against labour, not ambient coverage that grants diplomas for free

**Acceptance criteria:**
- AC-1: A student is assigned a school seat through the same planned-allocation mechanism as a job vacancy (using workBuilding + a Student flag), unifying commute and capacity accounting between work and study. [SUBSTRATE: ABSENT — greenfield, CS1's SetStudentplace trick, CONFIRMED per spec/citizens.md:51] · impact:`journey` · seam:`integration` · scenario:`SCENARIO-0071`
- AC-2: Each education tier enforces an attended-capacity throughput ceiling (kindergarten 10, school 12, university 3); a citizen cannot occupy a seat beyond the tier's authored throughput. [SUBSTRATE: ABSENT — greenfield, W&R $CITIZEN_ABLE_SERVE CONFIRMED per spec/citizens.md:52] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0071`
- AC-3: A citizen with no attended school/university seat never gains the associated education-tier credential, rejecting CS1's ambient education field where coverage alone grants diplomas. [SUBSTRATE: ABSENT — greenfield; explicitly rejects CS1's ResidentAI.cs:1259-1330 ambient-coverage pattern per spec/citizens.md:52] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0071`

**Sources:**
- `spec/citizens.md:51-52`

**Status:** pending

## STORY-0056

**Epic:** EPIC-014 — Labour Allocation & Workplace Binding
**Title:** Dispatch surge labour via a labour-pool office without touching fixed bindings

**As a** Planner
**I want** unassigned or temporary workers to be dispatchable to short-lived demands (construction, harvest) through a labour-pool office
**So that** surge work gets staffed without disturbing any citizen's fixed Work.workplace binding

**Acceptance criteria:**
- AC-1: The labour-pool office dispatches unassigned/temporary workers to a short-lived demand (e.g. a construction site) and, after the demand ends, every dispatched citizen's fixed Work.workplace value is unchanged from before the dispatch. [SUBSTRATE: ABSENT — greenfield, W&R $RESOURCE_SOURCE_WORKERS CONFIRMED, kept as the one W&R pattern per spec/citizens.md:47] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0072`

**Sources:**
- `spec/citizens.md:47`

**Status:** pending