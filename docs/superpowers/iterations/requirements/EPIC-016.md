# EPIC-016 — Households & Housing

**Summary:** Households & Housing
**Stories:** STORY-0060, STORY-0061, STORY-0062
**Primary sources:** `docs/egregoria-substrate-audit.md`, `spec/households.md`
**Status:** 0/3 done

## STORY-0060

**Epic:** EPIC-016 — Households & Housing
**Title:** Introduce households as shared-pantry family units

**As a** Planner
**I want** citizens grouped into household entities that share a single dwelling and a single pantry
**So that** housing allocation and shopping demand are computed at the family level, not artificially per-person

**Acceptance criteria:**
- AC-1: A Household entity groups member citizens under one dwellingRef and one shared pantry buffer, capped at an authored constant (not hardcoded to CS1's fixed value of 5) so the cap can be tuned without a code change. [SUBSTRATE: ABSENT — greenfield; every human currently owns its own home and its own pantry, no Household entity exists; cap value is an open question per spec/households.md:65] · impact:`journey` · seam:`unit` · scenario:`SCENARIO-0060`
- AC-2: Consumption by any household member debits the shared household pantry, not a private per-citizen stock. [SUBSTRATE: ABSENT — greenfield] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0060`
- AC-3: A dwellingRef of zero represents a household with no assigned flat (in the housing queue), distinct from an assigned dwelling. [SUBSTRATE: ABSENT — greenfield] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0060`
- AC-4: A new household pantry starts at 200, drains 20 per step from consumption, and triggers a shopping trip once it falls below 200, refilling by 100 per completed trip. [SUBSTRATE: ABSENT — greenfield, CS1 CONFIRMED constants per spec/households.md:34] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0060`

**Sources:**
- `spec/households.md:18-29`
- `docs/egregoria-substrate-audit.md:119-129`

**Status:** pending

## STORY-0061

**Epic:** EPIC-016 — Households & Housing
**Title:** Allocate housing through an explicit, player-visible queue

**As a** Planner
**I want** unhoused households to sit in a policy-weighted, visible queue rather than being matched invisibly or deleted
**So that** housing shortage is a legible planning failure, not a silent population loss

**Acceptance criteria:**
- AC-1: A household with no vacancy assigned enters a queue ranked by policy weights (priority class, workplace proximity, family-size-to-flat-size fit); queue length is a value the player can read, and no dwelling or household carries a price/affordability field of any kind — queue position, never money, gates assignment. [SUBSTRATE: ABSENT — greenfield, OURS per spec/households.md; CS1's equivalent is an invisible priority x distance market, no visible queue exists; spec/households.md:16 'No RCI bar'] · impact:`journey` · seam:`app-level` · scenario:`SCENARIO-0061`
- AC-2: A household displaced by building loss or eviction re-enters the queue; it is never deleted from the simulation for lack of housing. [SUBSTRATE: ABSENT — greenfield; explicitly rejects CS1's ResidentAI.cs:2434-2445 citizen-deletion pattern per spec/households.md evidence log] · impact:`journey` · seam:`integration` · scenario:`SCENARIO-0061`
- AC-3: Overcrowding (two households doubled into one flat) is a representable, distinguishable state from a normally-assigned flat. [SUBSTRATE: ABSENT — greenfield; capacity is authored as flats x max_household_size specifically to make overcrowding representable] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0061`

**Sources:**
- `spec/households.md:31-40`
- `spec/households.md:82-92`

**Status:** pending

## STORY-0062

**Epic:** EPIC-016 — Households & Housing
**Title:** Author dwelling quality and default-on service requirements

**As a** Planner
**I want** each dwelling prefab to carry an authored qualityOfLiving scalar and to require heat/electricity by default
**So that** housing quality feeds citizen satisfaction and no residential building can silently opt out of basic services

**Acceptance criteria:**
- AC-1: Each dwelling prefab carries an authored qualityOfLiving scalar in the observed CONFIRMED range (0.60 khrushchyovka to 1.02 village house), and this value feeds a household member's housingQuality need. [SUBSTRATE: ABSENT — greenfield, W&R CONFIRMED range per spec/households.md:48] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0080`
- AC-2: A residential building requires heat and electricity by default (no dwelling prefab declares a heating/electricity opt-out); only a non-residential prefab may opt out via an explicit disable flag. [SUBSTRATE: ABSENT — greenfield, W&R CONFIRMED-by-absence per spec/households.md:48] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0080`

**Sources:**
- `spec/households.md:48`

**Status:** pending