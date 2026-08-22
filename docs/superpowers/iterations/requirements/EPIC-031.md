# EPIC-031 — Water — quality-graded pipe network

**Summary:** Water — quality-graded pipe network
**Stories:** STORY-0127, STORY-0128, STORY-0129, STORY-0130
**Primary sources:** `spec/water.md`
**Status:** 0/4 done

## STORY-0127

**Epic:** EPIC-031 — Water — quality-graded pipe network
**Title:** Water sourced, treated, and piped with a quality grade

**As a** planner
**I want** water to flow from a source through treatment and pipes to consumers, carrying a 0-1 quality value end to end
**So that** shortage and contamination propagate as unmet need, not a coverage toggle

**Acceptance criteria:**
- AC-1: A water source has an output rate and source quality; a treatment plant consumes dirty water plus chemicals plus power and outputs water capped at quality 0.99. [SUBSTRATE: ABSENT — greenfield, zero water-system footprint in simulation/src per keyword sweep] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0127`
- AC-2: Quality degrades through use and pipe transport is tracked per flow, not reset to a fixed value at each hop. [SUBSTRATE: ABSENT — greenfield] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0127`
- AC-3: A sensitive consumer with a declared required quality is gated: it receives no usable water input if delivered quality is below its own class threshold, even though flow rate is sufficient — an animal farm gates below 0.93, a food factory below 0.97, and nuclear cooling below 0.60, so different consumer classes gate at different numeric thresholds, not one shared cutoff. [SUBSTRATE: ABSENT — greenfield, thresholds sourced from research/utilities.md §D2] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0127`
- AC-4: The water network includes reservoirs/switches as in-line storage and junction hardware, distinct from substations acting as leaf buffers — a reservoir smooths a temporary supply/demand mismatch by drawing down its stored volume instead of immediately failing delivery downstream. [SUBSTRATE: ABSENT — greenfield] · impact:`local` · seam:`integration` · scenario:`SCENARIO-0127`

**Sources:**
- `spec/water.md:16-23`
- `spec/water.md:18`

**Status:** pending

## STORY-0128

**Epic:** EPIC-031 — Water — quality-graded pipe network
**Title:** Pipe network with capacity and pump-hop loss

**As a** planner
**I want** water pipes to have throughput capacity and pumps that push water over distance
**So that** network topology and pump placement are real engineering decisions

**Acceptance criteria:**
- AC-1: A pipe segment carrying flow above its capacity delivers no more than its capacity downstream. [SUBSTRATE: OURS/ABSENT — greenfield, no $CAPACITY token in W&R source data either] · impact:`local` · seam:`unit`

**Sources:**
- `spec/water.md:24-27`

**Status:** pending

## STORY-0129

**Epic:** EPIC-031 — Water — quality-graded pipe network
**Title:** Water tanker as off-grid delivery

**As a** planner
**I want** a building not connected to the pipe network to instead be served by a water tanker truck via the logistics dispatcher
**So that** water reaches buildings the pipe grid hasn't reached yet, at a real logistics cost

**Acceptance criteria:**
- AC-1: Water is registered as an ItemID so a truck can carry it as ordinary cargo between a water station and an off-grid building's local buffer. [SUBSTRATE: PARTIAL — item registration substrate exists (prototypes/src/prototypes/item.rs:8-12) but no water dispatch office or truck cargo type is wired up] · impact:`cross-surface` · seam:`integration`
- AC-2: An off-grid building fed by tanker shows the same shortage/queue behaviour under insufficient truck capacity as a pipe-fed building shows under insufficient pipe capacity — no special-cased unlimited truck supply. [SUBSTRATE: ABSENT — greenfield, depends on the logistics dispatcher (spec/logistics.md) which the audit could not confirm as a generic reusable abstraction] · impact:`cross-surface` · seam:`integration`

**Sources:**
- `spec/water.md:28-30`

**Status:** pending

## STORY-0130

**Epic:** EPIC-031 — Water — quality-graded pipe network
**Title:** Reserve substations from cross-class draw

**As a** planner
**I want** to flag a substation as reserved for one consumer class so another class cannot draw from it
**So that** critical residential water supply cannot be crowded out by industrial demand at a shared endstation

**Acceptance criteria:**
- AC-1: A substation flagged residential-only rejects draw requests from industrial consumers even when it has spare capacity; industrial demand is routed elsewhere or left unmet rather than drawing from the reserved substation. [SUBSTRATE: ABSENT — greenfield, adopted planner policy per spec/water.md:32-34] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0137`

**Sources:**
- `spec/water.md:32-34`

**Status:** pending