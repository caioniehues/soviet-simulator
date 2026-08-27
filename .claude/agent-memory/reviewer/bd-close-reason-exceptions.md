---
name: bd-close-reason-exceptions
description: 3 of 33 closed bd issues lack verification evidence in the close reason — all three are "superseded by Wave 3" retirements, not sloppiness
metadata:
  type: project
---

**Close-reason discipline is strong but has exactly three exceptions, and they share a shape.**

Verified 2026-08-26 by sweeping all 33 closed issues for a commit sha or test evidence in the
`Close reason:` line. The three without:

- `sov-xie` — "docs edited (uncommitted)". The one commonly cited exception.
- `sov-iter0000-wrapup-xwe` — "Retired by charter reset and Wave 3 re-derivation".
- `sov-charter-amend-130-nl0` — "Superseded by Wave 3 story migration".
- (`sov-scenario-coverage-bt0` is borderline: "Superseded by Wave 3 evidence rebuild: 21/21
  requirements and 107/107 EVID anchors have planned coverage" — it cites counts, but no command.)

**Why:** the last three are *retirements*, not completions — the work was cancelled by a scope
reset, so there is no commit that could prove it. That is a legitimately different category, and
counting them as discipline failures overstates the drift.

**How to apply:** when auditing tracker hygiene, separate "closed as done without proof" (a real
defect — only `sov-xie` qualifies) from "closed as superseded" (needs a pointer to the deciding
document, not a test). Any doc claiming "one exception" among the 33 is undercounting; the honest
figure is one discipline failure plus three scope retirements.

See [[gotcha-executed-by-hook-inert]].
