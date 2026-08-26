# EPIC-024 — Hoarding & quota deception (walking skeleton)

**Summary:** Hoarding & quota deception (walking skeleton)
**Stories:** STORY-0105, STORY-0106, STORY-0107
**Primary sources:** `docs/egregoria-substrate-audit.md`, `spec/production.md`
**Status:** 0/3 done

## STORY-0105

**Epic:** EPIC-024 — Hoarding & quota deception (walking skeleton)
**Title:** Let a building request more input than its recipe strictly needs

**As a** planner
**I want** an enterprise's declared input requirement to be a separate, inflatable number from the recipe's true consumption rate
**So that** the core loop — enterprises hoard inputs and inflate their requests — has something to inflate; today the requested amount is always item.amount, so there is nothing to see through

**Acceptance criteria:**
- AC-1: Today Market::buy_until, called from recipe_init/recipe_act, always requests exactly item.amount — the literal recipe quantity — with no distinction between plan quota and honest requirement. [SUBSTRATE: PROVIDED (as the un-deceptive baseline) — market.rs:161-167, called from souls/goods_company.rs:23,47] · impact:`local` · seam:`unit` · scenario:`JOURNEY-0001`
- AC-2: A building must be able to hold a reported requested quantity per input that is independently settable and can exceed the recipe's true per-cycle consumption. [SUBSTRATE: ABSENT — greenfield; this is the hoarding hook the audit names as the cleanest insertion point] · impact:`local` · seam:`unit` · scenario:`JOURNEY-0001`
- AC-3: Market::buy_until (or its successor) requests the reported quantity, not the recipe's literal amount, when the two diverge — the enterprise actually receives and stockpiles the inflated amount if the market can supply it. [SUBSTRATE: ABSENT — greenfield; requires additive fields on BuyOrder per audit §4] · impact:`cross-surface` · seam:`integration` · scenario:`JOURNEY-0001`

**Sources:**
- `spec/production.md:1-9`
- `docs/egregoria-substrate-audit.md:1-1`

**Status:** pending

## STORY-0106

**Epic:** EPIC-024 — Hoarding & quota deception (walking skeleton)
**Title:** Distinguish true consumption from reported request in the production ledger

**As a** planner
**I want** the sim to track, per building per cycle, both what was actually consumed by the recipe and what was requested/received from the market
**So that** a gap between the two is a measurable, queryable fact rather than something only visible by reading source

**Acceptance criteria:**
- AC-1: A building accumulates unconsumed surplus stock when received input exceeds what the recipe consumes per cycle, rather than the surplus being silently discarded or never arising (today request == consumption, so no surplus is possible). [SUBSTRATE: ABSENT — greenfield] · impact:`local` · seam:`integration` · scenario:`JOURNEY-0001`
- AC-2: The per-building surplus/hoard quantity for a given input is queryable (e.g. via a component or market inspection API) distinctly from the input's in-flight order quantity. [SUBSTRATE: ABSENT — greenfield] · impact:`local` · seam:`integration` · scenario:`JOURNEY-0001`
- AC-3: The mill's surplus stock becomes non-zero only as the result of a truck completing an unload, never as a side effect of a market match. [SUBSTRATE: ABSENT — greenfield; today economy/market.rs:277-279 credits buyer capital at trade-MATCH time, before any physical delivery, so a naive surplus implementation could accrue stock with no truck ever moving] · impact:`cross-surface` · seam:`integration` · scenario:`JOURNEY-0001`

**Sources:**
- `spec/production.md:1-9`

**Status:** pending

## STORY-0107

**Epic:** EPIC-024 — Hoarding & quota deception (walking skeleton)
**Title:** Let the planner detect a hoarding enterprise from observable state alone

**As a** player acting as THE PLANNER
**I want** to compare a factory's requested/received input quantity against what it actually consumes and notice when they diverge
**So that** hoarding is a discoverable gameplay fact — the emotional core of the design — not an invisible backend number

**Acceptance criteria:**
- AC-1: Selecting a building shows, for at least one input, both the requested/received quantity and the recipe's true per-cycle consumption, or a derived surplus figure, in the same panel. [SUBSTRATE: ABSENT — greenfield UI, no existing panel exposes recipe consumption vs order quantity] · impact:`journey` · seam:`app-level` · scenario:`JOURNEY-0001`
- AC-2: Given two otherwise-identical buildings of the same type, one with an honest requested quantity and one with an inflated requested quantity, the inflated one visibly accumulates a growing surplus stock over multiple production cycles while the honest one does not. [SUBSTRATE: ABSENT — greenfield, depends on AC from surplus-tracking story] · impact:`journey` · seam:`e2e` · scenario:`JOURNEY-0001`

**Sources:**
- `spec/production.md:1-9`

**Status:** pending