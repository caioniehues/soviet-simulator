---
name: feedback-specialist-gates
description: A domain-specialist reviewer beats a general adversarial reviewer on its own seam, for the same or less cost — measured twice on 2026-08-23; build the specialist rather than upgrading the generalist
metadata:
  type: project
---

**On a seam with a nameable invariant, dispatch a specialist reviewer rather than a general one.**
It finds more, for the same or less.

**Why — measured, not assumed (2026-08-23, `market.rs` delivery seam):**

| | general opus `reviewer` | `ledger-invariant-checker` |
|---|---|---|
| Cost | 112k | **100.8k** |
| CONFIRMED findings on that seam | 2 | **5** |
| Quality of the shared finding | "the ext loop ignores `reserved`" | Showed the domestic pass is self-consistent, located the real hole in `sell_all` re-posting between passes, derived the break window algebraically, then found a shipping prototype sitting inside it |

The three the general gate missed entirely were not minor: a reservation that acquires and never
releases; a wedge where every bread purchase creates a dispatch while stores spawn no trucks; and
**the buy path crediting goods free whenever no freight station exists — scarcity, the game's whole
pressure source, switched off.** It also disproved a comment in the code.

The same held at the cheap end. `wiring-auditor` (sonnet) independently reproduced two findings the
112k general gate produced and added one it missed — `Market::dispatches()` has no in-game
observation surface, so the story's promise that "the planner catches it from observable state" is
unmet outside `cargo test`.

**How to apply:**

- When a diff touches a seam with a statable invariant — conservation, reachability, determinism,
  scope — write or dispatch the specialist for that invariant. Do not upgrade the generalist's tier
  and hope.
- The specialist's power comes from **one question asked relentlessly**, not from breadth. Keep its
  scope narrow and its depth uncapped. See [[feedback-agent-thoroughness-over-cost]].
- Order gates cheap → expensive, but **for the correctness reason, not the cost one**: a
  reachability defect makes every later review moot, since there is no point auditing the logic of
  code nothing calls.
- The general reviewer is still needed for diffs with no nameable invariant. This is a routing rule,
  not a replacement.
- **Corollary worth taking seriously:** three gate runs, three sets of real CONFIRMED findings — a
  100% hit rate. That does not mean the gates are thorough. It means the codebase holds more
  defects than have been looked for.
