# EPIC-028 — Environmental production modifiers (greenfield)

**Summary:** Environmental production modifiers (greenfield)
**Stories:** STORY-0118, STORY-0119, STORY-0120
**Primary sources:** `spec/production.md`, `spec/vehicles.md`
**Status:** 0/3 done

## STORY-0118

**Epic:** EPIC-028 — Environmental production modifiers (greenfield)
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

## STORY-0119

**Epic:** EPIC-028 — Environmental production modifiers (greenfield)
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

## STORY-0120

**Epic:** EPIC-028 — Environmental production modifiers (greenfield)
**Title:** Retire vehicles by lifespan and gate them by historical availability window

**As a** planner
**I want** vehicle types to carry an age-limit lifespan and a historical production window
**So that** aging fleets retire realistically and vehicle types cannot be manufactured or imported outside their era

**Acceptance criteria:**
- AC-1: A Vehicle type may carry a lifespan (age limit in years, as an authored field on specific types such as trolleybuses, not a universal field); a vehicle whose age exceeds its lifespan is retired from dispatch eligibility independent of its wear/condition value. [SUBSTRATE: ABSENT — greenfield; no age/lifespan field exists in transportation/vehicle.rs] · impact:`local` · seam:`unit`
- AC-2: A Vehicle type may carry an available-from/available-to historical production-window field; manufacturing or importing that vehicle type outside its window is rejected. [SUBSTRATE: ABSENT — greenfield; no era-gating exists for vehicle types anywhere in the codebase] · impact:`local` · seam:`integration`

**Sources:**
- `spec/vehicles.md:1-33`

**Status:** pending
