# EPIC-017 — Scale & Performance

**Summary:** Scale & Performance
**Stories:** STORY-0063
**Primary sources:** `docs/egregoria-substrate-audit.md`, `spec/citizens.md`
**Status:** 0/1 done

## STORY-0063

**Epic:** EPIC-017 — Scale & Performance
**Title:** Keep the per-citizen decision loop performant as population grows

**As a** Planner
**I want** citizen decision-making to preserve its per-human staggering and stay within a measured tick budget as headcount rises
**So that** the game remains playable at realistic city population sizes instead of degrading unnoticed

**Acceptance criteria:**
- AC-1: Each human's re-decision interval remains a randomized 30-80 tick stagger, spatially seeded, so the population is not all re-evaluated on the same tick. [SUBSTRATE: PROVIDED — souls/human.rs:185, 30 + rand2(pos)*50 ticks] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0067`
- AC-2: A profiled population ceiling exists and is documented — full-human-collection update time per tick stays under a stated frame budget at that ceiling. [SUBSTRATE: UNAUDITED — population ceiling is UNCONFIRMED per egregoria-substrate-audit.md sec.5/sec.9, needs runtime profiling, not a code read] · impact:`journey` · seam:`process-level` · scenario:`SCENARIO-0067`

**Sources:**
- `spec/citizens.md:70-72`
- `docs/egregoria-substrate-audit.md:119-135`

**Status:** pending