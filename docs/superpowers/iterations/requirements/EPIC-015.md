# EPIC-015 — Needs & Consumption

**Summary:** Needs & Consumption
**Stories:** STORY-0057, STORY-0058, STORY-0059
**Primary sources:** `docs/egregoria-substrate-audit.md`, `spec/citizens.md`, `spec/needs.md`
**Status:** 0/3 done

## STORY-0057

**Epic:** EPIC-015 — Needs & Consumption
**Title:** Protect purchase settlement on physical arrival, not on trade match

**As a** Planner
**I want** shop stock to debit and a buyer's goods to credit only when the buyer physically arrives at the shop
**So that** the CS1 teleport-credit exploit (goods appearing before the trip completes) cannot return as a regression

**Acceptance criteria:**
- AC-1: When a buy order is matched but the citizen has not yet arrived at the shop, neither shop stock nor citizen/household inventory changes. [SUBSTRATE: PROVIDED — souls/desire/buyfood.rs:50-54, the spec's target bug is already fixed] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0059`
- AC-2: Stock debit and buyer credit occur atomically at the tick of physical arrival, not at the tick the trade was matched. [SUBSTRATE: PROVIDED — souls/desire/buyfood.rs:50-54] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0059`

**Sources:**
- `spec/citizens.md:63-66`
- `docs/egregoria-substrate-audit.md:119-129`

**Status:** pending

## STORY-0058

**Epic:** EPIC-015 — Needs & Consumption
**Title:** Model per-citizen need satisfaction beyond food

**As a** Planner
**I want** each citizen to track 0..1 satisfaction across warmth, water, health, housing space/quality and rest, not only food
**So that** shortages other than hunger become visible economic signals instead of invisible gaps

**Acceptance criteria:**
- AC-1: A CitizenNeeds structure exists with distinct 0..1 fields for food, warmth, water, health, housingSpace, housingQuality and rest. [SUBSTRATE: ABSENT — greenfield; today only three score() functions (Home constant 0.2, Work binary window, BuyFood hunger clock) exist in souls/human.rs:190-231, no CitizenNeeds struct] · impact:`journey` · seam:`unit` · scenario:`SCENARIO-0077`
- AC-2: Each need decays over simulated time and refills only on a real satisfying event (goods delivered, or a trip to a stocked/staffed building), never by ambient coverage. [SUBSTRATE: ABSENT — greenfield] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0077`
- AC-3: Wants (consumer goods, culture, leisure) and aspirations (car, bigger flat, education) are tracked as a separate slower-updating layer whose chronic pressure can cross a threshold and become economic demand. [SUBSTRATE: ABSENT — greenfield, OURS per spec/needs.md] · impact:`journey` · seam:`integration` · scenario:`SCENARIO-0077`
- AC-4: A loyalty/political-legitimacy meta-need is tracked as a satisfaction-driven aggregate distinct from the per-citizen needs/wants/aspirations fields, moved by broadcast/propaganda and monument-style sources. [SUBSTRATE: ABSENT — greenfield, W&R $MONUMENT_GOVERNMENT_LOYALTY per spec/needs.md:67] · impact:`journey` · seam:`integration` · scenario:`SCENARIO-0077`
- AC-5: The wants layer has named fields (goods, alcohol, culture, sport, spiritual, recreation) and each field's satisfaction is spatial and quality-weighted: pollution near the serving building lowers it, nature/water proximity raises it. [SUBSTRATE: ABSENT — greenfield, W&R $ATTRACTIVE_FACTOR_* per spec/needs.md:30-36,72-75] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0077`
- AC-6: Durable needs (housing, appliances, car) resolve and are tracked at household level, while consumable needs (food, health, education) resolve and are tracked at individual-citizen level. [SUBSTRATE: ABSENT — greenfield per spec/needs.md:82 open-question resolution] · impact:`journey` · seam:`integration` · scenario:`SCENARIO-0077`
- AC-7: Needs recompute on the simulation clock's low-frequency band while aspirations recompute on the distinctly slower very-low (months) band; an aspiration's pressure value does not change on a tick where only the needs band fired. [SUBSTRATE: ABSENT — greenfield, architecture/simulation-clock.md per spec/needs.md:79] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0077`

**Sources:**
- `spec/needs.md:13-40`
- `spec/needs.md:58-73`
- `docs/egregoria-substrate-audit.md:119-129`

**Status:** pending

## STORY-0059

**Epic:** EPIC-015 — Needs & Consumption
**Title:** Make unmet needs legible: wait, substitute, or visibly go without

**As a** Planner
**I want** a citizen whose need cannot currently be met to show a visible queuing or going-without state, rather than an order that silently sits unmatched
**So that** shortage is felt and actionable — a longer queue or going without, never an invisible stall or a game-over

**Acceptance criteria:**
- AC-1: An unmatched buy order surfaces the citizen in an observable waiting/queued state rather than the order sitting with no player-visible signal. [SUBSTRATE: ABSENT — greenfield; there is no wait/substitute/go-without state machine, an unmatched buy order just sits, economy/market.rs] · impact:`journey` · seam:`app-level` · scenario:`SCENARIO-0066`
- AC-2: Wait-time urgency reuses the elapsed-since-last-satisfied clock already computed for hunger as its felt-urgency signal. [SUBSTRATE: PARTIAL — souls/human.rs:190-231, BuyFood::score already computes urgency from elapsed time since last eating, ready-made hook per audit sec.5] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0066`
- AC-3: When urgency crosses a threshold and no substitute good/building is available, the citizen enters an observable 'going without' state rather than despawning, error-looping, or waiting silently forever. [SUBSTRATE: ABSENT — greenfield] · impact:`journey` · seam:`app-level` · scenario:`SCENARIO-0066`

**Sources:**
- `spec/needs.md:1-11`
- `docs/egregoria-substrate-audit.md:119-129`

**Status:** pending