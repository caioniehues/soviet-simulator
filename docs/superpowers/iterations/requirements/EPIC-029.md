# EPIC-029 — Education — schooling pipeline

**Summary:** Education — schooling pipeline
**Stories:** STORY-0121, STORY-0122, STORY-0123
**Primary sources:** `docs/egregoria-substrate-audit.md`, `spec/education.md`
**Status:** 0/3 done

## STORY-0121

**Epic:** EPIC-029 — Education — schooling pipeline
**Title:** Enrol a child in a staffed school as a workplace-slot binding

**As a** citizen (child of school age)
**I want** to be assigned a seat at a reachable, staffed school the same way a worker is assigned a job
**So that** education capacity is physically constrained by real buildings and staff, not an ambient radius

**Acceptance criteria:**
- AC-1: A school building exposes a finite `studentCapacity` and an `enrolled[]` list; a citizen can only hold a seat if `enrolled.len() < studentCapacity`, mirroring CS1's serviceable-seat throttle, where `studentCapacity` derives from `StudentCount * 5/4` slots and is additionally throttled down by the school's current production rate. No `School` type, capacity field, or enrolment list exists anywhere in the codebase today. [SUBSTRATE: ABSENT — greenfield] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0100`
- AC-2: Enrolment occupies the same binding slot a job occupies: while a citizen is enrolled they cannot simultaneously hold a `Work.workplace` binding. The attachment point is `WorkKind` in `souls/desire/work.rs:11-17`, which today is only `Driver | Worker` — a `Student` variant (or a sibling `School.enrolled_at` binding with the same mutual-exclusion invariant as `Work.workplace`) must be added there. [SUBSTRATE: ABSENT — greenfield, attaches at souls/desire/work.rs:11-17] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0100`
- AC-3: A citizen without a reachable school with a free seat remains unenrolled indefinitely (a legibility signal to the planner) rather than being silently granted education progress; this never blocks or fails the simulation tick. [SUBSTRATE: ABSENT — greenfield] · impact:`journey` · seam:`app-level` · scenario:`SCENARIO-0100`
- AC-4: A citizen's decision to seek enrolment is scored as a new desire module in the shape of existing desires (`score()` returning a utility, arbitrated by max alongside `BuyFood`/`Home`/`Work` in `souls/human.rs:190-231`), not a bespoke code path bolted elsewhere. [SUBSTRATE: ABSENT — greenfield, follow pattern at souls/human.rs:190-231 and souls/desire/buyfood.rs] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0100`
- AC-5: Serviceable seat count degrades below raw `studentCapacity` when the school's production rate falls below full (e.g. partial staffing), distinct from the flat capacity cap — a citizen can be refused enrolment even when `enrolled.len() < studentCapacity` if the school is underproducing. [SUBSTRATE: ABSENT — greenfield] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0100`
- AC-6: A school requires its own two-tier staff complement to operate (workers 10 + profesors 15, per `school.ini:25-26`), distinct from a merely nonzero-staff check; a school staffed below this composition (e.g. workers present but profesors short) produces a proportionally reduced production rate rather than a binary staffed/unstaffed toggle. [SUBSTRATE: ABSENT — greenfield] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0100`

**Sources:**
- `spec/education.md:16-23`
- `docs/egregoria-substrate-audit.md:148-158`

**Status:** pending

## STORY-0122

**Epic:** EPIC-029 — Education — schooling pipeline
**Title:** Progress a citizen through education tiers by seat-time

**As a** planner
**I want** citizens to advance through kindergarten → school → university strictly by accumulated months physically enrolled at an operating, staffed facility
**So that** the labour market's qualification supply is a real, tractable pipeline the planner can see and invest in, not an instant unlock

**Acceptance criteria:**
- AC-1: A citizen gains a `seatTimeMonths` counter that increments only while enrolled at a school currently operating (staffed and not blacked out); a citizen at an unstaffed or unpowered school accrues zero seat-time. `HumanEnt`/`PersonalInfo` today carries no such field. [SUBSTRATE: ABSENT — greenfield, add to PersonalInfo or a new field on HumanEnt] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0104`
- AC-2: Crossing a tier's required seat-time threshold sets `educationTier` to the next ordered value (school < university); a citizen never regresses a tier once earned, matching the CS1 ordered-levels shape adopted for our qualification ladder. [SUBSTRATE: ABSENT — greenfield] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0104`
- AC-3: Each education tier enforces its own throughput ceiling on simultaneous `enrolled[]`, narrowing up the ladder per `$CITIZEN_ABLE_SERVE`: school 12 per cycle, university 3 per cycle — university's cap is a hard cap tighter than school's, so the pipeline visibly bottlenecks upward at each tier, not only at the university stage. [SUBSTRATE: ABSENT — greenfield] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0104`
- AC-4: Because job matching today is pure Euclidean-distance barter of an `ItemID::new("job-opening")` with no tier/overqualification concept (`souls/human.rs:267-269`, `market.rs:216`), an AC requiring a school-graduated citizen to preferentially fill a tier-matched vacancy CANNOT be proven until that labour-allocation rewrite lands; this story stops at producing a correct `educationTier` value, and downstream tier-matched hiring is out of scope pending that dependency. [SUBSTRATE: CONFLICTS — souls/human.rs:267-269, market.rs:216] · impact:`journey` · seam:`e2e` · scenario:`SCENARIO-0104`

**Sources:**
- `spec/education.md:24-40`
- `docs/egregoria-substrate-audit.md:156-158`

**Status:** pending

## STORY-0123

**Epic:** EPIC-029 — Education — schooling pipeline
**Title:** Send a graduate to a university specialisation feeding the matching profession

**As a** planner
**I want** a university choice (medical, technical, general) to determine what kind of qualified staff a citizen becomes
**So that** medical and technical institutions can only be staffed by the education pipeline the planner deliberately funded, making education planning a real allocation decision

**Acceptance criteria:**
- AC-1: A citizen enrolled at a university gains a `specialisation` field (e.g. medical, technical) taken from the university's own subtype, persisted on graduation; no specialisation or subtype concept exists on any building or human type today. [SUBSTRATE: ABSENT — greenfield] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0108`
- AC-2: A hospital or university's `profesor`-tier staff slot can only be filled by a citizen whose `specialisation` matches the required kind (medical for hospitals, per spec/healthcare.md staff loop); an unmatched citizen is rejected from that staffing slot rather than silently accepted. [SUBSTRATE: ABSENT — greenfield] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0108`
- AC-3: At game start, with zero graduates in existence, a university requiring 100 profesor-tier staff to run (per the two-tier W&R staff shape) cannot self-bootstrap; the planner must either import specialists (an external-trade equivalent) or accept the university runs at reduced/zero throughput until its own graduates arrive — this chicken-and-egg is intentional, never a hard failure or game-over. [SUBSTRATE: ABSENT — greenfield] · impact:`journey` · seam:`app-level` · scenario:`SCENARIO-0108`

**Sources:**
- `spec/education.md:24-44`

**Status:** pending