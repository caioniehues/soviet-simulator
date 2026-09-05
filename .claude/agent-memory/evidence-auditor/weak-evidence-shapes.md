---
name: weak-evidence-shapes
description: Recurring shapes of weak or overclaimed evidence in soviet-simulator, with the concrete case that established each
metadata:
  type: project
---

Patterns to check for first, because they have each already shipped here at least once.

**Half a fix is tested, the other half is announced.** A change with a *detect* half and a
*react* half is usually only tested on the detect half. sov-bo3 (2026-08-27) bounded a runaway
walk AND made `skeleton()` return `None` rather than truncate. Every test proved "no OOM";
deleting the whole refusal half kept all 24 tests green. The reaction half is the half the
commit message argues hardest for, and the half nobody asserts.
**Why:** the detect half is easy to unit-test on a hand-built input; the react half needs the
corrupt state to travel to a caller, which nobody wants to construct.
**How to apply:** for any diff whose commit message reasons about *what we do when we notice*,
neuter the noticing→reacting link (not the noticing) and rerun. If it stays green, that is the
finding.

**The `#[ignore]` guard nobody re-runs.** sov-bo3's memory sweep is `#[ignore]`, referenced by
nothing outside its own file, and this repo runs no CI with `--ignored`. It is real evidence the
day a human types the command and dead weight every day after.
**How to apply:** always grep for the test name outside its file, and for `--ignored` in CI. Say
plainly in the report that the guard is manual-only.

**Prose evidence in a bd comment is not output.** sov-bo3's implementer recorded accurate numbers
in a ticket comment and the commit message, but no verbatim terminal output existed anywhere in
the repo. The numbers turned out to be right — I reproduced them — but that was luck from the
auditor's side, not verification.
**How to apply:** re-run it yourself. Never grade a summary.

**A ticket's own counts drift.** sov-bo3's DESCRIPTION said "Seven call sites" and listed six.
The code-review-graph's `callers_of` said five, because two calls inside one function collapse
into one edge. Three numbers, none of them agreeing, all in play at once.
**How to apply:** an edge count is never a call-site count; recount in source.

**Reconfirmed 2026-08-28 (sov-mwy):** the half-tested shape again, in the economy this time.
`ledger::sov_abs_ext_trade_import_is_physical` asserts an import is carried by a `Dispatch`,
that capital is 0 at match time, and that it arrives — the *physical* half, thoroughly. It
asserts nothing at all about `Trade.money_delta`. Deleting the `-` from
`money_delta: -(*ext_value * qty_buy)` leaves all 52 tests green. When a fix is described as
"X now moves physically", the money half is the half to mutate.

**A mutation trial's findings go stale against `main`, and only a re-run can tell you.**
The sov-mwy trial ran at `345a79a`; `main` was six `market.rs` commits ahead within a day, two
of which added ext-trade tests. Reading those tests could not settle whether the gap had
closed — re-running the 11 mutants could, and it had not closed. Always re-run the mutants,
never re-read the tests, when the base has moved.
**How to apply:** check `git log <trial-base>..main -- <file>` first; if it is non-empty, the
trial's survivor list is a hypothesis again.

Older, from prior audits: the vacuous command (a test filter matching nothing exits zero), the
tautology (asserting a literal the test itself just wrote), and the weaker claim (STORY-0096
asserting arithmetic while claiming sourcing).
