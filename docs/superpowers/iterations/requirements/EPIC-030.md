# EPIC-030 — Sewage — the return half of the water cycle

**Summary:** Sewage — the return half of the water cycle
**Stories:** STORY-0113, STORY-0114, STORY-0115
**Primary sources:** `spec/sewage.md`
**Status:** 0/3 done

## STORY-0113

**Epic:** EPIC-030 — Sewage — the return half of the water cycle
**Title:** Sewage produced wherever water is consumed

**As a** planner
**I want** sewage to accumulate as a byproduct of household water use and as a declared industrial output
**So that** sewage volume is a physical consequence of water consumption, not an independent input

**Acceptance criteria:**
- AC-1: A household consuming water produces a corresponding sewage quantity into its local buffer; an industrial recipe building can also declare a fixed sewage output rate independent of water use. [SUBSTRATE: ABSENT — greenfield, zero sewage-system footprint per keyword sweep] · impact:`local` · seam:`integration`

**Sources:**
- `spec/sewage.md:10-18`

**Status:** pending

## STORY-0114

**Epic:** EPIC-030 — Sewage — the return half of the water cycle
**Title:** Sewage network with treat-or-discharge choice

**As a** planner
**I want** a pipe network separate from the water network, carrying sewage to either a treatment plant (recovers water at quality 0.85, costs chemicals+power+workers) or a free discharge point (pollutes)
**So that** cheap-now-vs-clean-later is a real planning decision with physical consequences

**Acceptance criteria:**
- AC-1: Sewage pipes and water pipes are distinct graphs; a building connected only to the water network cannot route sewage anywhere without a separate sewage pipe connection. [SUBSTRATE: ABSENT — greenfield; CONFLICTS with the CS1 shared-grid approach the substrate audit already rejected for electricity] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0128`
- AC-2: Routing sewage to a treatment plant instead of a discharge point costs chemicals and power and produces usable water capped at quality 0.85 (lower than fresh treatment's 0.99 ceiling), so recovered water never substitutes for fresh water at food-grade thresholds. [SUBSTRATE: ABSENT — greenfield] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0128`
- AC-3: Routing sewage to a discharge point instead requires no chemical/power input but emits pollution into the shared environment model at the discharge location. [SUBSTRATE: ABSENT — greenfield, depends on a shared pollution model not yet specified] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0128`

**Sources:**
- `spec/sewage.md:16-29`

**Status:** pending

## STORY-0115

**Epic:** EPIC-030 — Sewage — the return half of the water cycle
**Title:** Sewage overflow backs up and gates water use

**As a** planner
**I want** sewage that exceeds treatment/discharge capacity to back up to the nearest discharge point, or, absent one, fill producers' local buffers and stop their water consumption
**So that** a blocked drain physically stops the tap rather than sewage vanishing

**Acceptance criteria:**
- AC-1: When treatment capacity is exceeded and a discharge point exists on the network, excess sewage routes there automatically (auto-pollute) instead of being silently dropped. [SUBSTRATE: OURS/ABSENT — greenfield, declared Gaps in both source games] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0129`
- AC-2: When no discharge point exists and local sewage buffers fill, the producing building's water consumption is gated (blocked) until buffer space frees up. [SUBSTRATE: OURS/ABSENT — greenfield] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0129`

**Sources:**
- `spec/sewage.md:35-37`

**Status:** pending