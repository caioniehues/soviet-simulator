# EPIC-024 — Environmental production modifiers (greenfield)

**Summary:** Environmental production modifiers (greenfield)
**Stories:** STORY-0089, STORY-0090
**Primary sources:** `spec/production.md`
**Status:** 0/2 done

## STORY-0089

**Epic:** EPIC-024 — Environmental production modifiers (greenfield)
**Title:** Modulate renewable-connected recipes' output by weather and day/night cycle

**As a** planner
**I want** a recipe connected to wind or solar power to have its output rate scaled by current weather and time-of-day conditions
**So that** renewable-dependent production is a legible physical factor rather than a constant rate regardless of conditions

**Acceptance criteria:**
- AC-1: Today no field or mechanism ties any recipe's output rate to weather or day/night state; no renewable-connection flag exists on Recipe. [SUBSTRATE: ABSENT — greenfield; spec/production.md:37,59,107] · impact:`none` · seam:`unit`
- AC-2: A recipe may declare a connectedToWind or connectedToSun flag; when set, its base_rate is scaled by a current wind-strength or sunlight-availability value (0..1) sourced from the weather/day-cycle system, in addition to the other multiplicative factors. [SUBSTRATE: ABSENT — greenfield; spec/production.md:59] · impact:`local` · seam:`unit`

**Sources:**
- `spec/production.md:37`
- `spec/production.md:59`
- `spec/production.md:107`

**Status:** pending

## STORY-0090

**Epic:** EPIC-024 — Environmental production modifiers (greenfield)
**Title:** Let extraction recipes' output drift downward over calendar time as the resource depletes

**As a** planner
**I want** a mine or field's base output rate to decline over in-game calendar time
**So that** depletion/ageing is a physical consequence of continued extraction rather than an eternal constant rate

**Acceptance criteria:**
- AC-1: Today no field or mechanism reduces a recipe's base_rate as calendar time elapses; base_rate is a static constant. [SUBSTRATE: ABSENT — greenfield; spec/production.md:60,107] · impact:`none` · seam:`unit`
- AC-2: A recipe may declare a yearly depletion rate; over N in-game years of continuous operation with this flag set, the recipe's effective base_rate is strictly lower than at year 0, while a recipe without the flag holds a constant base_rate over the same period. [SUBSTRATE: ABSENT — greenfield; spec/production.md:60] · impact:`local` · seam:`integration`

**Sources:**
- `spec/production.md:60`
- `spec/production.md:107`

**Status:** pending
