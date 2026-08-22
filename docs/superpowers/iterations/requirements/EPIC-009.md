# EPIC-009 — Price administration

**Summary:** Price administration
**Stories:** STORY-0044, STORY-0045
**Primary sources:** `docs/egregoria-substrate-audit.md`, `spec/trade.md`
**Status:** 0/2 done

## STORY-0044

**Epic:** EPIC-009 — Price administration
**Title:** Set administered retail prices distinct from enterprise shadow prices

**As a** planner
**I want** to set a fixed retail price table for household-facing consumer goods, separate from the enterprise-side ext_value used at the border
**So that** prices never float to clear excess demand — the queue does the clearing instead

**Acceptance criteria:**
- AC-1: A retail price table exists per consumer-good item, settable by the player/plan, and is read by every citizen purchase instead of any per-trade computed price. [SUBSTRATE: ABSENT — no policy layer exists; the only price value is ext_value from calculate_prices, computed once at startup and never recalculated, economy/market.rs:343-403, docs/egregoria-substrate-audit.md:121-122,37] · impact:`journey` · seam:`app-level` · scenario:`SCENARIO-0019`
- AC-2: Setting a retail price and then simulating excess household demand at that price produces a longer queue/wait time and/or empty shelves, never an automatic price increase. [SUBSTRATE: ABSENT — greenfield, no queue-vs-price clearing distinction exists in citizen purchase logic today] · impact:`journey` · seam:`integration` · scenario:`SCENARIO-0019`
- AC-3: Enterprise-facing shadow/settlement prices used for internal beznal trade can differ from the household retail price for the same item, and changing one does not change the other. [SUBSTRATE: ABSENT — greenfield; SingleMarket.capital is goods-on-hand not currency and is not a price signal, economy/market.rs:32, docs/egregoria-substrate-audit.md:126-127] · impact:`cross-surface` · seam:`unit` · scenario:`SCENARIO-0019`

**Sources:**
- `spec/trade.md:34-40`
- `docs/egregoria-substrate-audit.md:121-127`

**Status:** pending

## STORY-0045

**Epic:** EPIC-009 — Price administration
**Title:** Publish the world-market price model to the player

**Deferred:** true
**Deferred reason:** charter:95 "All 16 resources trade both ways at fixed per-kind prices (no market)"

**As a** planner
**I want** to see the current export/import price of each traded good and the drivers moving it (era, world events, my own export volume)
**So that** the moving market is legible rather than a black box

**Acceptance criteria:**
- AC-1: A player-facing view shows, for a traded item, its current border price and at least the named drivers that changed it since the last check. [SUBSTRATE: ABSENT — greenfield; the only price value that exists is ext_value, a fixed per-item value computed once at startup and never recalculated or exposed as a curve, economy/market.rs:35, calculate_prices at market.rs:343-403, docs/egregoria-substrate-audit.md:121-122] · impact:`journey` · seam:`app-level` · scenario:`SCENARIO-0029`
- AC-2: The border price for a good changes over simulated time in response to at least one modelled driver (e.g. cumulative export volume), rather than remaining the single startup-computed constant it is today. [SUBSTRATE: CONFLICTS — ext_value from calculate_prices is computed once, never recalculated, economy/market.rs:343-403, docs/egregoria-substrate-audit.md:37-38,121-122] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0029`

**Sources:**
- `spec/trade.md:30-33`

**Status:** pending