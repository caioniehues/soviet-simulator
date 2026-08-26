# EPIC-008 — Two-circuit money

**Summary:** Two-circuit money
**Stories:** STORY-0040, STORY-0041, STORY-0042, STORY-0043
**Primary sources:** `docs/charter-1.0.md`, `docs/egregoria-substrate-audit.md`
**Status:** 0/4 done

## STORY-0040

**Epic:** EPIC-008 — Two-circuit money
**Title:** Give households a cash balance (nal)

**As a** citizen
**I want** to hold a personal cash balance
**So that** I can buy consumer goods at state-fixed retail prices instead of receiving them for free

**Acceptance criteria:**
- AC-1: Every human entity has a nal (cash) balance field that persists across save/load. [SUBSTRATE: ABSENT — greenfield; audit §4/§5 confirm humans have no currency field at all, docs/egregoria-substrate-audit.md:37-38] · impact:`cross-surface` · seam:`unit` · scenario:`SCENARIO-0022`
- AC-2: A citizen's buy order for a consumer good is rejected (or queued/deferred) if their nal balance is below the item's administered retail price, replacing today's unconditional 'purchases resolve on arrival' fulfilment. [SUBSTRATE: CONFLICTS — souls/desire/buyfood.rs:50-54 resolves purchase on arrival with no price param, docs/egregoria-substrate-audit.md:37-38] · impact:`journey` · seam:`integration` · scenario:`SCENARIO-0022`

**Sources:**
- `docs/egregoria-substrate-audit.md:121-160`
- `docs/charter-1.0.md:17-24`

**Status:** pending

## STORY-0041

**Epic:** EPIC-008 — Two-circuit money
**Title:** Pay wages from employer to worker in nal

**As a** worker
**I want** to receive a periodic wage in cash from my workplace
**So that** I have money to spend at retail without a free item handout

**Acceptance criteria:**
- AC-1: A working human's workplace debits its own account and credits the worker's nal balance on a defined wage interval. [SUBSTRATE: ABSENT — greenfield; audit confirms no wages exist, grep for wage/salary returns nothing except a flat government upkeep debit at economy/mod.rs:51, docs/egregoria-substrate-audit.md:17-19] · impact:`journey` · seam:`integration` · scenario:`SCENARIO-0023`
- AC-2: An unemployed human (no Work.workplace binding) accrues no wage income over any simulated period. [SUBSTRATE: PARTIAL — Work.workplace binding exists (souls/desire/work.rs:20-26) but nothing reads it for money movement today, docs/egregoria-substrate-audit.md:17-19] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0023`

**Sources:**
- `docs/egregoria-substrate-audit.md:17-19,154-158`

**Status:** pending

## STORY-0042

**Epic:** EPIC-008 — Two-circuit money
**Title:** Give enterprises a separate accounting-rouble (beznal) settlement account

**As a** enterprise
**I want** to settle input purchases and wage payroll from a non-cash accounting account distinct from any household cash
**So that** enterprise-to-enterprise trade and payroll never share a pool with retail consumer spending

**Acceptance criteria:**
- AC-1: Every company/enterprise entity carries a beznal balance distinct in type from a citizen's nal balance (not the same undifferentiated Money(i64) scalar reused for both). [SUBSTRATE: ABSENT — Money(i64) is one undifferentiated scalar with no account types today, prototypes/src/types/money.rs:14, docs/egregoria-substrate-audit.md:123-125] · impact:`cross-surface` · seam:`unit` · scenario:`SCENARIO-0024`
- AC-2: Internal (domestic, non-border) trades between enterprises settle in beznal with money_delta no longer hardcoded to zero, replacing today's price-free barter clearing. [SUBSTRATE: CONFLICTS — every internal trade is money_delta: Money::ZERO today, economy/market.rs:226, docs/egregoria-substrate-audit.md:29-31] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0024`

**Sources:**
- `docs/egregoria-substrate-audit.md:121-160`

**Status:** pending

## STORY-0043

**Epic:** EPIC-008 — Two-circuit money
**Title:** Forbid beznal from ever buying a consumer good

**As a** planner
**I want** a hard rule that enterprise accounting roubles cannot purchase retail consumer goods under any circumstance
**So that** the nal/beznal separation that makes plan fulfilment and shopping queues distinct problems cannot be silently bypassed

**Acceptance criteria:**
- AC-1: An attempt to fund a citizen-facing retail purchase from an enterprise's beznal account is rejected at the transaction boundary regardless of balance available, with no code path that converts beznal to nal implicitly. [SUBSTRATE: ABSENT — no account types or circuit separation exist at all; Money(i64) serves treasury, ext price and trade delta alike with no distinction, docs/egregoria-substrate-audit.md:123-125] · impact:`journey` · seam:`integration` · scenario:`SCENARIO-0018`
- AC-2: There is no explicit or implicit exchange function that converts a beznal balance into nal outside of the documented wage-payment path (enterprise beznal debit -> worker nal credit is the only legal bridge between circuits). [SUBSTRATE: ABSENT — greenfield, no such boundary exists to violate or preserve yet, docs/egregoria-substrate-audit.md:154-158] · impact:`cross-surface` · seam:`unit` · scenario:`SCENARIO-0018`

**Sources:**
- `docs/egregoria-substrate-audit.md:154-158`

**Status:** pending