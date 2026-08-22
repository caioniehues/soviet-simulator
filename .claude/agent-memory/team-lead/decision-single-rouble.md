---
name: decision-single-rouble
description: 2026-08-22 decision — 1.0 ships a single rouble; dual currency (roubles/dollars) is deferred to Post-1.0 despite spec/trade.md marking it adopted
metadata:
  type: project
---

Soviet Simulator 1.0 ships **one domestic rouble**. Dual-ledger stories may be written but must be marked `deferred`.

**Why:** `spec/trade.md:26-31` marks dual currency "CONFIRMED, adopted" (from Workers & Resources), but `docs/charter-1.0.md:108` lists dual currency in Post-1.0, `:27` says "money is the foreign rouble at the border", `:95` says "single rouble". After the 2026-08-22 Egregoria fork the charter's identity/posture/never/post-1.0 lists still bind — only rung sequencing and estimates were re-cut. Additionally, Egregoria has **no household money or wages at all**, so nal/beznal is already net-new machinery; a second foreign ledger on top triples the surface before the core loop is proven. User chose the charter over the spec.

Middle option considered and NOT taken: one domestic rouble plus a single scarce `hard_currency` scalar at the border, replenished only by export. Revisit this if the rouble-rich/dollar-poor squeeze turns out to be load-bearing for the fantasy.

**How to apply:** No 1.0 requirement, AC, or roadmap iteration may assume a second currency or per-currency loans. When `spec/trade.md` and `docs/charter-1.0.md` conflict elsewhere, the charter wins on identity/scope; the spec wins only on mechanism detail inside an already-in-scope rung.

See [[charter-1-0]], [[economy-model-kornai]], [[substrate-audit-decisions]].
