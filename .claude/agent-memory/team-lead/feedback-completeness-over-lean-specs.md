---
name: feedback-completeness-over-lean-specs
description: In requirements/spec work caio wants exhaustive capture including numeric constants as ACs — do not trim findings for leanness or budget
metadata:
  type: feedback
---

In requirements-extraction and spec work, capture **everything**. Do not propose trimming a findings list for leanness, and do not treat numeric constants (thresholds, ratios, capacities, rates) as "balance values too churny to assert" — they become ACs.

**Why:** 2026-08-22, at the PAR omission-review gate on the Soviet Simulator extraction, I offered four dispositions for 172 findings and recommended "fix omissions + scenarios, defer the 44 thin-ACs", arguing that baking balance numbers into acceptance criteria creates brittle tests. Caio chose **"Fix everything"** instead. Earlier the same session he also upgraded my PAR sizing from a proposed 8-agent single-lens pass to the full 16-agent two-reviewer version, and told me to raise wave parallelism from 4 to 8+. The pattern is consistent: for spec/requirements work he buys completeness and pays the token cost.

Note this is domain-specific and does NOT override ponytail for *implementation*. Ponytail's ladder still governs code. The distinction: a lean spec loses information that is expensive to re-derive; lean code is just less code.

**How to apply:** In extraction/review/audit work, default to the exhaustive option and size the wave for full coverage. Still *announce* the cost and still surface the leaner alternative as a named option — he wants the tradeoff visible, he just usually declines it. When he overrides toward completeness, build the full version without re-arguing.

See [[decision-single-rouble]].
