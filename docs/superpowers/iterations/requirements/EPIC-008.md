# EPIC-008 — Foreign currency

**Summary:** Foreign currency
**Stories:** STORY-0030, STORY-0031
**Primary sources:** `spec/trade.md`
**Status:** 0/2 done

## STORY-0030

**Epic:** EPIC-008 — Foreign currency
**Title:** Track two hard-currency ledgers, roubles and dollars

**As a** planner
**I want** domestic-bloc trade to settle in roubles and hard-currency trade to settle in dollars, in two separate treasury balances with no free conversion
**So that** being rouble-rich does not automatically mean I can buy Western goods

**Acceptance criteria:**
- AC-1: Treasury carries two distinct currency balances (roubles, dollars); a good's trade currency is a property of the good/origin, not a runtime player choice. [SUBSTRATE: ABSENT — Government.money is a single undifferentiated Money(i64) scalar today, economy/government.rs:10, docs/egregoria-substrate-audit.md:123-128] · impact:`cross-surface` · seam:`unit` · scenario:`SCENARIO-0021`
- AC-2: There is no direct rouble-to-dollar conversion action; dollars are obtainable only via export settlement or a dollar-denominated loan. [SUBSTRATE: ABSENT — greenfield, exchange rate/conversion mechanic explicitly flagged as an open gap in trade.md's own evidence log] · impact:`journey` · seam:`integration` · scenario:`SCENARIO-0021`

**Sources:**
- `spec/trade.md:18-24,66-68`

**Status:** pending

## STORY-0031

**Epic:** EPIC-008 — Foreign currency
**Title:** Offer per-currency loans with interest and borrowing caps

**As a** planner
**I want** to borrow roubles or dollars separately, each with its own interest rate, penalty rate, and borrowing cap
**So that** debt pressure is currency-specific, matching the two-ledger split the rest of foreign trade already enforces

**Acceptance criteria:**
- AC-1: (DEFERRED to Post-1.0 per docs/charter-1.0.md:108 — captured, not scheduled for 1.0) Treasury supports a loan per currency, each carrying its own principal, interest rate, and penalty rate, matching the `loans: [{currency, principal, rate, penaltyRate}]` shape in the design draft. [SUBSTRATE: ABSENT — greenfield; no loan mechanic or Treasury type exists at all, spec/trade.md:22-23,53] · impact:`cross-surface` · seam:`unit`

**Sources:**
- `spec/trade.md:22-23,53,64`

**Status:** pending