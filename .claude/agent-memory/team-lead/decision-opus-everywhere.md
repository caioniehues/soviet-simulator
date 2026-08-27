---
name: decision-opus-everywhere
description: 2026-08-27 user decision — ALL agent rosters (16 project + user-scope generics) stay pinned opus/high; supersedes the sonnet-implements default, but the review gate stays mandatory
metadata:
  type: project
---

**All agent rosters run on opus** (user decision 2026-08-27, chosen over "revert to sonnet" when the
contradiction was put to them directly): the 16 soviet-simulator agents pin `model: opus` /
`effort: high` (miner-style exceptions keep their own effort), and the user-scope generics
(`implementer`, `researcher`, `miner` at low effort) pin opus too. `delegation.md` and
`docs/process/development-cycle.md` were rewritten the same day to match.

**Why:** the working tree had flipped all 16 to opus while the docs still said "uniform sonnet";
asked which side was intended, the user picked opus.

**How to apply:**
- Do not "fix" an opus pin back to sonnet because an older doc or memory says sonnet — the sonnet
  doctrine is superseded. Sonnet remains the named fallback under explicit cost pressure
  ("remember to use cheap subagents" is still a live correction the user may issue).
- A high worker tier does NOT earn a gate skip: the review-gate-catches-defects result was measured
  with an opus reviewer over an opus implementer. Every non-trivial diff still runs the chain.
- See [[feedback-specialist-gates]] (the gate is the quality lever) and
  [[feedback-agent-thoroughness-over-cost]] (depth is never capped).
