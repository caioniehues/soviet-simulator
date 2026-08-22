---
name: gotcha-inherited-claims
description: Two substrate claims inherited from RESUME.md were false and got propagated to ~20 agents unchecked — verify handoff-brief claims against code before putting them in briefs
metadata:
  type: project
---

Claims inherited from a handoff brief (`RESUME.md`, a prior session's notes, a memory file) are **untrusted until re-derived from code**. Two in this repo were wrong and I propagated both into roughly twenty agent dispatches before checking either.

**The two errors, corrected 2026-08-22:**

1. *"`Lot::generate_along_road` is disabled; nothing may depend on auto-lot generation."* — **False.** It is called live from `Map::connect` (`simulation/src/map/map.rs:719`). Roads auto-spawn lots today. Disabling it is *pending work*: STORY-0013, verdict CONFLICTS, scheduled ITER-0005. The real constraint is the opposite of what the brief said — things currently *do* depend on it, including `TestCtx::build_house_near`, which selects from `map().lots()` and will break when STORY-0013 lands.

2. *"`simulation/src/tests/test_iso.rs` has a proven determinism harness."* — **Wrong file.** `TestCtx` is defined in `simulation/src/tests/mod.rs`; `test_iso.rs` merely consumes it. The harness is real (its `tick()` does serialize → deserialize → per-key hash compare), but it is `pub(crate)` under `#![cfg(test)]`, so no external integration-test crate can reach it.

**Why:** neither claim was ever checked against the code — each was copied forward from a brief written by an earlier session. A false constraint is worse than a missing one: it is confidently worded, it reads exactly like a verified fact, and workers design around it. Cost here was contained only because the work was requirements extraction, where a wrong constraint produces a slightly wrong AC rather than broken code.

**How to apply:** before a substrate claim goes into a dispatch brief, run the one check that proves it — `grep`/LSP `findReferences` for "X is disabled", a `Read` for "the harness is at path P". This is the same discipline `rules/delegation.md` already requires for a *worker's* file+quote claim; it applies identically to claims inherited from a previous session, including my own earlier notes. Treat `RESUME.md` as a lead, not a source.

See [[substrate-audit-decisions]], [[decision-single-rouble]].
