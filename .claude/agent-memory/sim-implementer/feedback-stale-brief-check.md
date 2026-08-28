---
name: feedback-stale-brief-check
description: On a multi-round bd issue, read the comment thread before implementing — the brief may describe work that is already done
metadata:
  type: feedback
---

On any bd issue with an existing comment thread, run `bd show` AND
`bd comments` and read to the END before writing code. If the last comment is a
lead handoff, THAT is the current scope, not the dispatch brief.

**Why:** on sov-dispatch-wedge-ab4 (2026-08-26) the dispatch brief described
the kornai Option C ruling and wedge (a)/(b) cancellation as unstarted work.
Rounds 1-3 were in fact already implemented, uncommitted in the working tree,
and had passed both the ledger-invariant-checker (CONSERVED) and the opus
reviewer (APPROVE-WITH-FIXES). Implementing the brief literally would have
re-written finished, gate-approved code over itself. The real remaining scope
was a three-item round-4 list in the lead's own final comment. The user
confirmed stopping to check was the right call ("good catch on the stale
brief; that stop-and-check is exactly right").

**How to apply:** cheap tell that a brief is stale — `git status` shows the
files the brief asks you to create already modified, and `git log` HEAD matches
a sha cited in a handoff comment. Verify with the test suite count: a brief
that says "add a test" against a suite already containing it is describing the
past. Report the discrepancy as a finding rather than silently adapting; on
this project that finding is often worth more than the task.

**Second occurrence, 2026-08-28 (sov-dda.3).** A brief said "build the
lane-queue prototype spike; claim it". `bd show` line 1 read `[● P2 · CLOSED]`
— done the previous day, commit 38bf942, branch and worktree already removed,
preserved as tag `archive/sov-dda-3-lane-queues`. The cheapest tell is the
FIRST LINE of `bd show`: read the status badge before the description. When the
ticket is closed, the useful work is re-deriving its recorded evidence rather
than rebuilding it — that is how the 2.3x error in this one was caught
([[rederive-recorded-spike-numbers]]).

Related: [[dispatcher-truck-pool]], [[rederive-recorded-spike-numbers]].
