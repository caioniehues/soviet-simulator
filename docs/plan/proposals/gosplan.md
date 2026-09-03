# GOSPLAN — the development framework, proposed (v2)

**Kind:** decision
**Authority:** proposed — binds nothing until the Planner ratifies it as ADR-0001
**Status:** proposed
**Owner:** project lead
**Verified-at:** 4e9e930b2a73
**Last verified:** 2026-08-28
**Supersedes (on ratification):** `docs/process/development-cycle.md`, `.claude/skills/dev-cycle/`, `~/.claude/agents/team-lead.md` (for this repo)
**Supersedes:** v1 of this document (same path, 2026-08-28, earlier in the day)

Evidence base: fourteen research reports in `.planning/process-overhaul-2026-08-28/` — 01 Scrum
and dev cycles · 02 revfactory/harness · 03 twenty-one comparable repos · 04 Claude Code
orchestration map · 05 multi-agent team patterns · 06 audit of our own roster · 07 documentation
frameworks · 08 obra/superpowers + matt-skills · 09 code-review-skill + socratic · 10 Agent Teams
parallel development · 11 the documentation agent · 12 beads integration · 13 SWE-AF + SwarmForge
delta · 14 matt-skills engineering skills — about 1.3M tokens of agent work, every claim cited
there. This document is the synthesis;
when it and a report disagree, the report's citation wins and this document is wrong.

**What changed from v1.** Three agents added: `planner` (task decomposer), `steward` (process
auditor and retro drafter — the Scrum-Master's judgment work; the mechanical part is hooks) and
`doc-agent` (code docs, requirements traceability, wiki). Agent Teams adopted for the **lane team**
build only; race and pair stay file-relayed subagents because teams add a known messaging bug and
nothing else there. The cycle is now nine stages, each described with inputs, steps, artifact, exit
rule and the failure it stops. Hook scripts exist (report 12). Steal lists from six more repos are
folded in (Appendix A).

---

## 0. The verdict in one paragraph

Our process has one proven asset and one proven disease. The asset: **adversarial gates that
re-derive from source catch real bugs** — 7 confirmed catches across 3 stories; the ledger checker
found 5 defects where the general reviewer found 2 on the same seam (06 §5). The disease: **the
process layer is a dishonest enterprise.** It requests more than it consumes — 26 agents, 6,860
lines of definitions of which 50.5% is pasted boilerplate, 5 agents never dispatched, ~55 ad-hoc
dispatch names against 26 roster names, one story in 136 that ever ran the full cycle — and it
reports state the code does not have (11 internal contradictions, 15 open findings from its own
audits). The fix is the game's own fix: **the Planner judges enterprises from observable state,
never from their reports.** GOSPLAN keeps the gates; replaces the sprint with a *Plan* (a scope box
with an appetite and an evidence exit, never a clock); routes work by size; decomposes in a fresh
context; builds in parallel lane teams; turns "please remember" rules into hooks; audits the process
the way gates audit code; cuts the roster from 26 to 20 by moving lane knowledge into path-scoped
rules; and gives every document a freshness field a script can diff.

---

## 1. What the research settled

| # | Question | Answer (report) |
|---|---|---|
| 1 | Does Scrum transfer? | Structure yes, ceremonies no. Sprint→scope box, Planning→lead-written pitch, Daily→drop, Review→watch the running game, Retro→post-mortem that must produce a file diff. Scrum Master→hooks + a process auditor. PO = the human; PO absence is the #1 pathology. Shape Up appetite and circuit breaker apply unchanged. (01) |
| 2 | harness? | A roster factory. Steal: 500-line cap, pushy descriptions, CLAUDE.md changelog table, roster drift audit, boundary-mismatch review rule. Not: opus-everywhere, Agent Teams as it uses them (issue #53). (02) |
| 3 | The field? | 80% of 21 repos: file handoff, human-reviewed plan before code, slash-command phase entry, no built-in tracker, self-validation loops. Rare and worth it: right-sized routing (BMAD), executable validation in the brief (prp), hooks as enforcement (disler). (03) |
| 4 | Claude Code today? | Subagents (`skills:` preload, `maxTurns`, resume), 14 hook events (5 block), `.claude/rules/` with `paths:`, Workflow scripts, Agent Teams (experimental). (04) |
| 5 | Team patterns that pay? | Blind generator–verifier (87% vs 63%), parallel-independent-merge, blind parallel review (sequential reviewers anchor), a pre-build Spec-Mob. Selectively: racing test-writer (~2×), driver/navigator (~2.5×). Theatre: debate, mob, tournament, auction. (05) |
| 6 | What is wrong with ours? | 50.5% duplication; 5 never-dispatched agents; 1 full-cycle story; no light path, retro, iteration boundary, dispute path, completion mechanism, cost log, or list of what the human does; cost table measured at sonnet, fleet runs opus. (06) |
| 7 | Documentation shape? | Ten kinds, `Verified-at: <sha>`, enumerated Status, a brief template, ADRs, tables over prose. (07) |
| 8 | superpowers / matt-skills? | Per-task loop: implementer → fresh reviewer reads a pre-generated diff → ≤5 fix rounds with adjudication → whole-branch review. Ledger first line = plan identity (compaction recovery). ≤15-line report with 4 status codes. Description = trigger conditions only (empirically: a workflow summary in the description makes agents skip the body). Pre-flight file-conflict table, rows mandatory. matt-skills: every acceptance criterion classified evidenced-complete / incomplete / unverified before dispatch. (08) |
| 9 | code-review-skill / socratic? | Our gate chain is structurally ahead (blind, skeptic-verified, re-deriving). Steal the Rust checklists (async cancellation, `unsafe` needs `SAFETY:`, thiserror vs anyhow), the reuse-audit step, diff triage. Socratic's silent Mode A pre-build contract (assumed / open questions ≤3 / risks) fits the brief; its interactive mode is the PO-absence pathology. (09) |
| 10 | Agent Teams for parallel build? | Only the lead spawns; `gosplan` must be the lead. A lane team (one builder per lane, own worktree) is buildable today; race and pair are better as subagents. `SendMessage` by name silently drops (issue #42999, closed not-planned) — probe or resolve ids by hook. No `/resume` for in-process teammates; `skills:` does not preload into teammates; `SubagentStop` carries `agent_type` + `last_assistant_message`. (10) |
| 11 | The documentation agent? | One agent, three disjoint surfaces. Measured: rustdoc coverage on `simulation` is **7.4%**; `mdbook` 0.5.4 installed; the graph already holds a 201-page structural wiki; the four-script traceability chain runs but does not check that `SPEC-*` anchors are real headings or that `EVID-*` ids match test names. (11) |
| 12 | beads? | Epics, labels, `--claim` (atomic), `list --json` timestamps cover every metric except tokens and send-backs. Measured: median close age 1.1 h, median 0 comments, 58% of closes cite a sha. Three hook scripts written; one synthetic-tested. Recommendation: `bd` only — teammates claim from `bd`, the team task list is not a second store. (12) |
| 13 | SWE-AF / SwarmForge delta? | SWE-AF: a Python control plane driving Claude Code subprocesses; sequential reviewer sees the coder's work (inferior to ours). Steal: stuck-loop detection, typed debt records, upstream-failure notes on dependents, per-ticket memory, a synthesizer when parallel verifiers disagree. SwarmForge: the durable handoff file and the `By <role>` commit trailer still beat hook signals; `local-*` naming for rule overrides. (13) |

---

## 2. The diagnosis, in numbers

From reports 06 and 12, re-derived from files, `bd` and git:

- **6,704 lines** across 22 project agents; **3,386 duplicated** (three pasted sections, two drifted). Unique knowledge per agent averages **151 lines**.
- **Zero dispatch evidence** for `data-implementer`, `widget-implementer`, `net-implementer`, `common-implementer`, `miner`. **~55 distinct `bd` authors** against 26 roster names.
- **One story** (`sov-dispatch-wedge-ab4`) ran the full cycle. Median story closes in **1.1 h** with **0 comments**; **42% of closes** cite no sha.
- **The gates work**: 7 send-backs that caught real defects.
- **The cost table is fiction**: measured at sonnet, quoted for opus; one gate run cost 1.39M, twice the "full iteration".
- **11 contradictions** between `development-cycle.md` and the agent files it describes.
- **rustdoc coverage 7.4%**; 0 ADRs; the wiki that exists is a code index, not a narrative.

Every rule lives in prose, prose drifts, nothing mechanical notices. The gates are strong because they run against code; the process is weak because it runs against itself.

---

## 3. The design

### 3.1 The metaphor, used once

The player is THE PLANNER. Enterprises request inputs, hoard, and report success; the Planner
catches the dishonest ones from observable state. Here **the human is the Planner, every agent is
an enterprise, and evidence is the observable state.** A report is a request, not a fact. A stage
that cannot show its artifact did not happen. Everything below is mechanism for that one rule.

### 3.2 Roles

| Role | Who | Holds |
|---|---|---|
| **Planner** | the human | Product Goal (charter), Plan Goals, bets, mechanism rulings, non-obvious dispositions, commit/push, the running-game verdict |
| **gosplan** | the main session, persona `gosplan` | The lead: routes, dispatches, spawns the lane team, runs gates, disposes obvious findings, synthesises, holds the ledger. **Must be the lead session** — only the lead can spawn teammates (10 §Q4) |
| **planner** | subagent, fresh context per Plan | Task decomposition: atomic tasks, the DAG, lane + play per task, the file-conflict table, DoR-complete briefs, the `bd batch` script. The `task-coordinator` role from the lead's reference files, minus dispatch — dispatch stays with gosplan |
| **steward** | subagent, fresh context at three points | Process audit: DoR audit before dispatch, DoD audit at close, appetite vs ledger, retro draft. The Scrum-Master's judgment; hooks carry the mechanical rules. Never edits code, briefs or tracker state |
| **Builders** | `sim-`, `ui-`, `engine-`, `data-implementer` — as lane-team teammates or subagents | One lane each; lane knowledge arrives from path-scoped rules |
| **Ground** | `substrate-cartographer` | Fact-sheets with file:line, three sources |
| **Advisors** | `kornai-economist`, `logistics-modeller`, `settlement-modeller`, `utilities-modeller`, `soviet-authenticity` | Model consistency; Spec-Mob; Pair navigator; conditional gate sign-off |
| **Gates** | `wiring-auditor`, `ledger-invariant-checker`, `evidence-auditor`, `reviewer` (global), `drift-auditor` | Re-derive from source; never edit code |
| **Docs** | `doc-agent` | rustdoc coverage, traceability chain, wiki |
| **Diagnosis** | `debugger` | Root cause + minimal repro, never the fix |
| **Research** | `researcher` (global) | External evidence, cited |

20 agents (from 26). Retired and why — §4.3.

**The Planner's seven decision points.** gosplan stops for these and nothing else:

1. Bet: which stories, and the Plan Goal sentence.
2. Definition of Ready sign-off for L-lane briefs (M and S: the steward's audit suffices).
3. Any advisor ruling that sets a mechanism.
4. Any gate finding whose disposition is not an obvious fix.
5. Every commit and every push.
6. Plan Review: the running game, 15–20 s, watched back.
7. Retro: accept or replace the one change it proposes.

Teammates never ask the human; `AskUserQuestion` is confirmed for the lead only (10 §Q1).

### 3.3 The unit of work: the Plan

A Plan is a `bd` epic (`bd create --parent`; `bd epic status`; `bd epic close-eligible` — epics never auto-close):

| Field | Content | Source |
|---|---|---|
| **Goal** | One sentence — why this Plan, now. The Planner writes it. | Sprint Goal (01) |
| **Scope box** | 3–6 stories fixed at Bet. Nothing added mid-Plan. | 01 §5 |
| **Appetite** | Token budget and dispatch count, set from the previous Plan's ledger. A limit, not an estimate. | Shape Up (01) |
| **Evidence exit** | Ends when every story is closed with evidence or deferred (`bd defer`). No calendar. | 01 §5 |
| **Circuit breaker** | Second send-back on a task → `planner` re-shapes it (split, shrink, defer), Planner re-bets. Appetite spent → defer what is open. A *decision*, not an auto-kill: `sov-dispatch-wedge`'s four send-backs all caught real bugs. | Shape Up + superpowers' round-5 adjudication (08) |
| **Debt record** | Every deferral or scope cut is typed: `type · severity · the criterion it drops · justification`, and propagates into dependent tasks' briefs. | SWE-AF (13) |
| **Directory** | `.planning/plans/plan-NN-<slug>/` — `plan.md`, `brief-*.md`, `refine_*.md`, `gate-*.md`, `done/*.json`, `ledger.jsonl`, `review.md`, `retro.md`. First line of `ledger.jsonl` = the plan path (compaction recovery, 08) | |

WIP limit: **one Plan in flight.**

### 3.4 Lanes

| Lane | Fits when | Runs | Skips |
|---|---|---|---|
| **S · patch** | ≤ 1 file; no economy/market/dispatch/storage seam; no charter/spec touch; fact-sheet exists | brief → solo build → `wiring-auditor` → one `reviewer` → dispose | Ground, Spec-Mob, Prove, domain gate, steward DoR audit (hook only) |
| **M · story** | Bounded feature or bug inside one lane; testable acceptance | Ground (reuse/refresh) → brief → steward DoR → **Race** or lane-team solo → Prove → wiring → ledger if economy → two blind reviewers → dispose | Spec-Mob unless an advisor's cluster is touched |
| **L · system** | New mechanic, cross-lane, ratified spec touched, or the Planner says so | Ground → **Spec-Mob** → steward DoR → Planner sign-off → Race or Pair → Prove → full gate incl. advisor → dispose → ADR if a mechanism was set | Nothing |

Every ticket carries its routing rationale (`lane:`, `estimated_scope`, `touches_interfaces`,
`risk_rationale`) so a later agent can audit why (SWE-AF `IssueGuidance`, 13). Unsure → heavier lane.

**Definition of Ready.** A task is not dispatched until its brief carries: the *why* (Plan Goal
link) · appetite · acceptance criteria as EARS sentences, each **independently decidable** and each
classified *evidenced-complete / demonstrably incomplete / unverified* (matt-skills, 08) · `Verify:`
command plus the shape of its expected output · file:line pointers resolved in the main session
(subagents have no LSP) · traps · out-of-scope · file ownership · play · the pre-build contract
(socratic Mode A, 09): `Assumed:`, `Open questions:` (≤3, else NOT READY), `Risks:`. The
`dor-gate` hook enforces the machine-checkable subset; the steward audits the rest.

### 3.5 The cycle — nine stages

```
                       ┌──────────────────────── one Plan (one bd epic) ─────────────────────────┐
 0 BET ─► 1 DECOMPOSE ─► 2 REFINE ─► 3 BUILD ─► 4 PROVE ─► 5 GATE ─► 6 DISPOSE ─► 7 REVIEW ─► 8 RETRO
 Planner   planner        cartographer builders   evidence   gates     gosplan /    Planner +   steward
 + gosplan (fresh ctx)    · Spec-Mob   (lane team, auditor    (blind,   Planner      drift +     drafts,
                          · steward    race, pair)            ordered)               doc-agent   Planner
                            DoR audit                                                            accepts
                       └─── breaker: 2nd send-back → planner re-shapes · appetite spent → defer ──┘
```

Each stage: who · inputs · steps · artifact · exit · the failure it stops.

**0 · Bet** — Planner + gosplan.
Inputs: `bd ready`, last `retro.md` + `ledger.jsonl`, the charter. Steps: gosplan shows the ranked
queue and last Plan's numbers; the Planner writes the Goal and picks 3–6 stories; gosplan sets the
appetite. Artifact: `plan.md` header, the epic, children linked. Exit: the Planner says "bet".
Stops: Scrum-but; the 40-issue wave.

**1 · Decompose** — `planner`, fresh context.
Inputs: the epic, existing fact-sheets, the roster, lane rules. Steps: (1) split stories into atomic
tasks — one verifiable output, ≤5 per chain, split at lane boundaries never by size; (2) the DAG —
hard vs soft dependencies, a stub interface for each soft one; (3) lane + play per task; (4) the
**file-conflict table** — every task pair sharing a file or interface, rows mandatory, "clean with no
rows" is invalid (08); (5) one brief per task from `docs/templates/brief.md`, symbols resolved by
gosplan's LSP on request; (6) `batch.json` for `bd batch` + `bd dep`. Artifact: `plan.md` body,
`brief-<id>.md` per task. Exit: batch applied; stage 2's DoR audit READY. Stops: the shared
`scenarios/mod.rs` clobber; briefs with false premises; tasks that straddle two lanes.

**2 · Refine** — `substrate-cartographer`, the Spec-Mob, `steward`.
Ground (L, and M on unfamiliar seams): a fact-sheet with file:line for every seam a brief cites;
reused if its `Verified-at` sha diffs clean. Spec-Mob (L only, parallel, ~1.5×): advisor, lane
builder and `evidence-auditor` each read the raw story cold and write `refine_advisor.md`
(constraints missing, model consistency), `refine_builder.md` (sketch, edge cases, unknowns),
`refine_test.md` (the tests that would prove it — before code). `planner` (resumed) merges them
into the brief. Steward DoR audit (M, L): READY / NOT READY per task with the missing field.
Artifact: fact-sheets, `refine_*.md`, final briefs, the audit. Exit: all READY; L briefs signed by
the Planner. Stops: the three founding Phase-0 failures; "looks good" as a criterion.

**3 · Build** — builders; three plays (§3.6).
gosplan dispatches the DAG's ready front in one response. Lane team for parallel lanes; race or
pair as subagents on a single story. Builder loop: `bd update --claim` → read brief + pointed files
→ implement → run `Verify:` until green → self-review the diff → commit on the lane branch (the
commit trailer `By: <agent>` is added by a `prepare-commit-msg` hook, never typed — SwarmForge, 13)
→ write the report file → write the **done file** `done/<id>.json` (status, commits, verify output
path — durable; a completion that is not on disk did not happen, 13) → reply ≤15 lines with
`DONE` / `DONE_WITH_CONCERNS` / `BLOCKED` / `NEEDS_CONTEXT` (08). Builders never spawn reviewers.
gosplan runs **stuck-loop detection**: three build–verify rounds without new file changes →
stop, hand to `debugger` or re-shape (SWE-AF, 13). Artifact: branch, report, done file, ledger row
(hook). Exit: `DONE` and the verify command green in gosplan's own run. Stops: "tests pass" as a
claim; two writers in one file; self-graded work; infinite fix loops.

**4 · Prove** — `evidence-auditor`.
Every new guard mutated and seen red, output pasted, reverted; `cargo-mutants` floor per
`mutation-policy.md`. In the race play the auditor wrote the tests; here it mutation-tests them.
Artifact: red-then-green outputs in the task's `bd` comments. Exit: no guard never seen failing.
Stops: the sentinel filter that ran 0 tests; 24 green tests that never touched the refusal path.

**5 · Gate** — cheap to expensive, blind.
gosplan pre-generates the diff file; gates read it, never the builder's report (08). (1)
`wiring-auditor` — reachable from the running game? (2) `ledger-invariant-checker` — economy seams
only — quantity and money conserved, concrete failing sequence or "none". (3) **Two blind reviewers
in parallel** via `gate-review.js` (blind dimensions → skeptic verify → completeness critic →
verdict), with the Rust checklist (async cancel-safety, `unsafe` needs `SAFETY:`, thiserror vs
anyhow, reuse audit, both sides of every producer/consumer seam — 09, 02). (4) Advisor sign-off only
when the diff diverged from Refine. When two parallel verifiers disagree, a **synthesizer** step
(gosplan, or one blind skeptic) resolves before disposition (13). Artifact: `gate-<type>-<id>.md`,
findings CONFIRMED / PLAUSIBLE / REFUTED with severity, verdict. Exit: no CONFIRMED blocker.
Stops: reviewer anchoring; graded homework; unwired features.

**6 · Dispose** — gosplan, then the Planner.
Every finding fixed / accepted / filed, re-verified against the current commit. Obvious → back to
the builder with findings in the brief. Not obvious → Planner. Disputes → §3.8. Breaker on the
second send-back. Failed or deferred tasks write **upstream-failure notes** into dependents' briefs
before those dispatch (13). Close with `scripts/bd-close.sh <id> "<proof>"` — the sha is
substituted, never typed. `bd epic close-eligible` sweep. Exit: no orphan finding; every close has
a sha and a command output. Stops: the 359-line rewrite between filing and reading; the 42% of
closes with no sha.

**7 · Review** — the Planner, then `drift-auditor` and `doc-agent`.
gosplan merges lane branches, runs the full suite, records a 15–20 s video; the Planner watches it
back. `drift-auditor` sweeps docs, agent frontmatter and the process layer against code and runs
`scripts/doc-check.sh`. `doc-agent` then runs its three surfaces (§5.4). Artifact: `review.md`
(what shipped, video path, drift report, doc numbers), the commit the Planner approves. Exit: seen
running; committed by the Planner. Stops: "looks like a child made it" shipping unseen; docs that
claim what code does not do.

**8 · Retro** — `steward` drafts, the Planner accepts.
`scripts/plan-metrics.sh` prints the numbers (§3.9). The steward drafts: what broke (ids), the
metric that moved, and **exactly one file change** — a rule, a trap into a `bd` description, a
template line, a tier change — or "no change, because…". The next appetite comes from this ledger.
Artifact: `retro.md` + the diff. Exit: the change landed or refused in writing. Stops: retros that
change nothing; cost tables that stay at the wrong tier.

Wrap and Ship as phases are gone: Wrap is stage 7's drift + docs pass; Ship is a `/release`
checklist skill run per release.

### 3.6 Build plays and the lane team

| Play | Shape | Runtime | When | Cost | Evidence |
|---|---|---|---|---|---|
| **Solo** | One builder, self-validation loop | subagent, or one lane-team teammate | S; M without testable acceptance | 1× | current practice |
| **Lane team** | One builder per lane in its own worktree, file ownership in the spawn prompt, gates dispatched afterwards as subagents | **Agent Teams** (in-process, ≤3 teammates) | Any Plan with tasks in ≥2 lanes on the same ready front | ~1.5–2× vs sequential solo; same wall-clock as one | 10 §Q3 V1 — buildable today |
| **Race** | Builder ∥ `evidence-auditor` writing adversarial tests blind to the code; gosplan integrates with `cargo test`; tests are ground truth | two subagents, file relay | M default; L | ~2× | AgentCoder; verifier asymmetry 87/63 (05) |
| **Pair** | Builder with `maxTurns` checkpoints; advisor navigator (read-only) returns PASS / RETURN per checkpoint; gosplan resumes the builder | subagents, file relay | L with spec-ambiguity flagged in Refine | ~2.5× | PairCoder (05) |
| ~~Best-of-2~~ | — | — | not adopted; +2.1 pp for 2.5× (05) | | |

**Lane-team protocol** (10 §Q3, 12 §3):

1. gosplan creates a worktree per lane: `git worktree add .claude/worktrees/<lane> plan-NN/<lane>` — teammates get no automatic isolation.
2. Spawns each builder *from its definition* with a prompt containing: `bd:<id>`, `Verify:`, the file-ownership list, the worktree path, the done-file path. `skills:` does **not** preload into teammates — house rules reach them through `.claude/rules/house-rules.md` (no `paths:` → loads at launch) and CLAUDE.md.
3. Teammates claim in `bd` (`bd update --claim`, atomic via Dolt; a losing claimant re-queries `bd ready`). The team task list is not a second store — `bd` only (12 §3).
4. Teammates report by writing the done file and their `bd` comment, then messaging the lead. Until issue #42999 is probed, the done file is the completion signal and `SendMessage` is a courtesy.
5. Gates never run as teammates — they are sequential, blind, and read-only.
6. No `/resume` for in-process teammates: progress must be recoverable from `bd` comments and done files alone. gosplan re-spawns after a crash; it never messages ghosts.
7. Fan-out cap: 3 teammates (Anthropic's 3–5 guidance, SWE-AF's preflight cap).

### 3.7 Mechanised discipline

Scripts exist in report 12 §4 (`dor-gate.sh`, `ledger.sh`, `export-before-commit.sh`, registration
block); the rest are ≤40-line siblings.

| Rule today (prose) | Mechanism | Event | Blocks |
|---|---|---|---|
| Every brief names its bd id and verify command | `dor-gate.sh`: builder/gate dispatch without `bd:<id>` + `Verify:` → exit 2 naming the missing fields; researcher, Explore, cartographer, debugger, drift-auditor, planner, steward exempt | `PreToolUse` · `Agent` | yes |
| Log at run end; the report is the final message | `ledger.sh`: `agent_type` + `last_assistant_message` (both in the payload — 12 §4.2) → row in `ledger.jsonl`; token counts are not in the payload (Wave 0 probe) | `SubagentStop` | no |
| A worker is done when it says so | `done/<id>.json` must exist with verify output path; gosplan's close step refuses without it | file invariant (13) | — |
| Which role committed | `prepare-commit-msg` appends `By: <agent>` from `$CLAUDE_AGENT` / the worktree's lane (13) | git hook | — |
| Close with the sha | `scripts/bd-close.sh <id> "<proof>"` | helper | — |
| `bd export` before commit | `export-before-commit.sh`: export; block if `issues.jsonl` dirty and unstaged (synthetic-tested, 12) | `PreToolUse` · `Bash` | yes |
| Docs must not claim what code does not | `doc-check.sh`: retired names outside `archive/`; `git diff <Verified-at> HEAD -- <cited>` ⇒ STALE; every `SPEC-*` resolves; `check_traceability.py` (11) | script | reports |
| Stuck loops | gosplan: 3 rounds without new file changes → stop (13) | lead protocol | — |
| Name-routed messages may drop | `PreToolUse` · `SendMessage` resolves teammate names to ids from `subagents/*.meta.json` if probe A1 fails (10) | hook | — |
| Gate workflow lives in the repo | `.gitignore`: track `.claude/{agents,rules,skills,workflows,settings.json}`; ignore `worktrees/`, `*.bak*`, `settings.local.json` (D3) | git | — |

Each hook < 2 s or async; a failing hook prints its failure — a starved fleet degrades silently.

### 3.8 Dispute procedure

| Dispute | Path |
|---|---|
| Builder vs gate finding | `bd comments add --author <builder> "DISPUTE <finding>: <why>"` → gosplan dispatches **one blind skeptic** on that finding only → CONFIRMED / REFUTED → gosplan disposes, recorded in `bd` |
| Gate vs gate; race verifiers disagree | Same skeptic as synthesizer; gosplan disposes; mechanism-setting → Planner |
| Advisor vs gosplan | Advisor is not a veto; gosplan decides; if it sets a mechanism, an ADR *before* build, Planner ratifies |
| Anyone vs the Planner | Recorded as an accepted finding with the reason; the retro may reopen it |

### 3.9 Metrics (computed, read at Retro)

| Metric | Source | Baseline (12) |
|---|---|---|
| Stories closed per Plan | `bd epic status` | — (no epics wired yet) |
| Story age open→close, median | `bd list --json` `created_at`/`closed_at` | 1.1 h |
| Comments per story | `comment_count` | median 0 |
| Closes citing a sha | `close_reason` grep | 58% |
| Send-backs per story; first-pass gate rate | gate reports + `bd` comments | — |
| Tokens per task per lane; appetite vs actual | `ledger.jsonl` (+ manual until tokens are in a payload) | — |
| Findings CONFIRMED vs REFUTED per gate | gate reports | ledger 5 vs reviewer 2 on one seam |
| Doc coverage; stale wiki pages; EVID-* without a test | `doc-agent` report | 7.4% · — · — |

No velocity, ever.

### 3.10 What to measure in Plans 01–02

Spec-Mob on every L story vs M stories without it (send-back rate); Race vs Solo alternated on M
stories (confirmed findings per story); sonnet builders vs the opus baseline (D1); lane-team vs
sequential (wall-clock, tokens, conflicts).

---

## 4. The roster

### 4.1 Agent-file architecture

| Layer | Holds | Mechanism | Size |
|---|---|---|---|
| Agent body | Only what this agent uniquely knows: role, refusals, method, report, memory rule. Description = **trigger conditions only**, never a workflow summary (08) | `.claude/agents/<name>.md` | ≤ 150 lines |
| House discipline | The three pasted sections, brief/report contracts, "you do not dispatch reviewers" | `.claude/rules/house-rules.md` (no `paths:` — loads at launch in every session, teammates included) + preloaded `skills:` for subagents | one copy |
| Gate discipline | `## How to judge` | `.claude/rules/judge-rules.md` scoped to gate agents by convention + `skills:` preload | one copy |
| Lane traps | bincode FIXINT/VARINT, RNG draw order, trucks are not trains, `optout_exttrade` 1-of-21, yakui layer stack… | `.claude/rules/<crate>.md` with `paths: ["<crate>/**"]`; refinements of a global rule use a `local-` prefix, never a same-name override (13) | 20–60 lines each |
| Reference | W&R paths and counts, three-source method, bench notes | `.claude/agents/references/*.md` | unbounded |

**Wave 0 probes (before any rewrite):** do `paths:` rules load in subagents and in teammates
(10 §Q5 A2)? Fallback: per-lane skills. Does `SendMessage` by name deliver (A1)? Do manual
worktrees isolate two teammates (A3)? Does `SubagentStop` carry `agent_type` for a named
definition (A4)? What field carries the type in the `PreToolUse Agent` payload (12 §4.1)?

### 4.2 The 20

| Agent | Tier | Stage | Change from today | bd evidence |
|---|---|---|---|---|
| `gosplan` (lead persona) | opus | all | new; replaces `team-lead` here; `initialPrompt` orients from `bd`, the Plan, the ledger | team-lead 22 |
| `planner` | opus | 1, 6 (re-shape) | **new** — the decomposer | — |
| `steward` | sonnet | 2, 6, 8 | **new** — DoR/DoD audit, appetite, retro draft | — |
| `sim-implementer` | D1 | 3 | absorbs `common/`, `headless/` | 29 |
| `ui-implementer` | D1 | 3 | absorbs `goryak/`, `egui-inspect*`, `assets_gui/` | 2 |
| `engine-implementer` | D1 | 3 | absorbs `geom/`, `networking/` | 9 |
| `data-implementer` | D1 | 3 | trimmed; kept for the Lua layer's blast radius, not its usage | 0 |
| `substrate-cartographer` | opus | 2 | fact-sheets get `Verified-at` | 10 |
| `kornai-economist` · `logistics-modeller` · `settlement-modeller` · `utilities-modeller` | opus | 2, 3 (pair), 5 | Spec-Mob and navigator roles; fix `model:` pin, `color:`, `TestCtx::tick()` claim | 4 · 11 · 6 · 7 |
| `soviet-authenticity` | opus | 2, 7 | advisor only, never a merge gate; delete the fabricated quotation | 1 |
| `wiring-auditor` · `ledger-invariant-checker` | opus | 5 | unchanged | 18 · 19 |
| `evidence-auditor` | opus | 2 (Spec-Mob), 3 (race), 4 | gains the test-writer role | 15 |
| `reviewer` (global) | opus | 5 | + Rust checklist, reuse audit, both-sides rule; reads a diff file | 18 |
| `drift-auditor` | sonnet | 7 | renamed from `doc-reality-auditor`; sweeps the process layer; runs `doc-check.sh` | 7 |
| `doc-agent` | sonnet | 7, on new spec/ADR | **new** (§5.4) | — |
| `debugger` | opus | on demand, stuck loops | + loop-building priority order, falsifiable hypothesis form, correct-seam regression test, ARCHITECTURE hand-off field (14); builders' bodies gain "unknown cause → debugger" | 2 |
| `researcher` (global) | sonnet | on demand | unchanged | 5 |

### 4.3 Retired

| Agent / artifact | Why | Where its knowledge goes |
|---|---|---|
| `common-`, `geom-`, `net-`, `widget-implementer` | 0, 2, 0, 0 dispatches; lane names plus traps | `.claude/rules/<crate>.md`; ownership → the absorbing builder |
| `implementer` (generic) | the path of least resistance (more usage than most specialists, contradiction 11) | delete; `dor-gate` refuses dispatches without a lane |
| `miner` | 0 dispatches | delete |
| `perf-engineer` | no bench exists; its own body says so | `references/bench.md`; recreate with a bench runner |
| `release-engineer` | a procedure | `/release` skill |
| `team-lead` (global) | replaced here by `gosplan`; stays global for other repos | — |
| four vestigial skills, `dev-cycle` skill, `development-cycle.md`, 15 codex adapters | < 30 lines / replaced / never used as a gate | archive; incidents → `docs/process/incidents.md` |

---

## 5. The documentation framework

### 5.1 Ten kinds, one header

| Kind | Authority | Lifecycle | Written by |
|---|---|---|---|
| `charter` | binding | draft → ratified → amended | Planner |
| `specification` | binding | draft → review → ratified → superseded | advisor |
| `decision` (ADR, MADR) | binding | proposed → accepted → deprecated \| superseded | gosplan / Planner |
| `fact-sheet` | observed | draft → verified → stale | cartographer |
| `brief` | operational | assigned → in-progress → closed | planner |
| `process` | operational | draft → active → superseded \| archived | gosplan |
| `explanation` | explanatory | draft → active → archived | anyone |
| `generated` | derived | active (regenerate only) | scripts |
| `gate-report` | advisory | active → archived | gates |
| `handoff` (`plan.md`, `review.md`, `retro.md`, `RESUME.md`) | operational | active → archived | gosplan / steward |

```
**Kind:** <one of ten>
**Authority:** binding | operational | observed | explanatory | derived | advisory
**Status:** <lifecycle state — nothing else>
**Owner:** <role, never a person>
**Verified-at:** <commit sha>        ← any document that cites code
**Last verified:** YYYY-MM-DD
**Supersedes:** / **Superseded by:** <path>
```

### 5.2 Templates to add (`docs/templates/`)

| Template | Fields |
|---|---|
| `brief.md` | id · lane + routing rationale · Plan Goal link · why · appetite · acceptance (EARS, each classified) · `Verify:` + expected output shape · pointers (file:line) · traps · out of scope · file ownership · play · pre-build contract (Assumed / Open questions ≤3 / Risks) · upstream-failure notes · report contract (≤15 lines, 4 status codes) · done-file path · closing line (`bd-close.sh …`) |
| `plan.md` | Goal · stories · DAG · file-conflict table · lane/play per task · appetite · Planner decision points scheduled · debt records · exit state |
| `retro.md` | metrics table · what broke (ids) · the one change (path + diff, or "none, because") · traps promoted into `bd` descriptions |
| `gate-report.md` | gate · scope (range / diff file) · findings ranked, CONFIRMED / PLAUSIBLE / REFUTED + severity · verdict · verification output |
| `agent.md` | frontmatter (all fields explicit: `model`, `effort`, `skills`, `memory`, `tools`) · description (trigger conditions + near-miss exclusions + re-trigger keywords) · role · refuses · method · report · memory rule · the six socratic agent-design questions as a checklist (09) |
| `decision.md` | keep; add `deprecated`, `Verified-at`, and the three-gate threshold for when a decision earns an ADR (14) |
| `arch-candidate.md` | problem · proposed solution · before/after sketch · strength of evidence · source (debugger post-mortem, Spec-Mob, cartographer hot-spots). Files into the Bet queue as a candidate story; no grilling loop (14) |

### 5.3 Layout

```
docs/
  plan/charter-1.0.md · plan/iterations/{requirements,evidence,extract,build_roadmap.py} · plan/RESUME.md
  decisions/ADR-0001-gosplan.md
  process/gosplan.md (≤ 250 lines) · process/incidents.md · process/{mutation,dependency}-policy.md
  reference/fact-sheets/ (from docs/research/) · reference/specifications/
  wiki/ (mdBook: book.toml, src/SUMMARY.md — §5.4)
  templates/{brief,plan,retro,gate-report,agent,decision,…}.md
.planning/README.md · .planning/plans/plan-NN-<slug>/ · .planning/process-overhaul-2026-08-28/
.claude/agents/<20>.md · agents/references/ · rules/{house-rules,judge-rules,<crate>,local-*}.md
  · skills/{gosplan,release}/ · workflows/gate-review.js · settings.json (hooks)
scripts/{bd-close.sh, doc-check.sh, plan-metrics.sh, check_traceability.py, hooks/{dor-gate,ledger,export-before-commit,commit-by-role}.sh}
CLAUDE.md ≤ 200 lines: pillars, pointers, the seven Planner points, a dated changelog table
```

### 5.4 `doc-agent` (report 11)

One agent, sonnet, three disjoint surfaces; runs at stage 7 after `drift-auditor`, on any new spec
or ADR, and on request. Never per story. Refuses specs (advisors), tests (evidence-auditor),
fact-sheets (cartographer), any mechanism.

| Surface | Method | Number it reports |
|---|---|---|
| **Code docs** | `cargo +nightly rustdoc -p simulation -- -Z unstable-options --show-coverage`; add `///` to undocumented public items, one sentence minimum, linking the governing `SPEC-*`; `RUSTDOCFLAGS="-W rustdoc::broken-intra-doc-links"` must be clean; `[workspace.lints.rust] missing_docs = "warn"` proposed | coverage %: **7.4% → +5 pp per Plan** |
| **Traceability** | `build_requirements.py --check`, `build_roadmap.py --check`, new `check_traceability.py` (every cited `SPEC-*` is a real heading; every `EVID-*` matches a test function in `tests/scenarios/`); gaps filed as `bd` comments for `evidence-auditor` | uncovered `REQ-*`; `EVID-*` without a test |
| **Wiki** | `docs/wiki/` in mdBook (0.5.4 installed): refresh pages whose `Verified-at` sha diffs; stub a page for every new spec/ADR; link the graph's 201-page structural wiki as the index, never duplicate it; `mdbook build` exit 0 | stale pages at end of pass |

Wiki skeleton (arc42-shaped, seeded from existing docs): `introduction` · `architecture/{overview,
substrate, economy, transport, settlement, utilities}` · `how-it-works/{dispatch-cycle, retail-flow,
dishonest-enterprise, map-generation}` · `decisions/index` · `glossary` · `reference/{crate-map,
spec-index, req-index}`. Economy, transport, dispatch, glossary and req-index have source material
today; retail-flow and utilities need an advisor's review first.

---

## 6. Standing rules that survive unchanged

Nothing teleports; never game over. Re-derive, never inherit. Gates read source, never the
producer's summary. Every guard seen failing. Evidence, not assertion — a close is a sha plus
output. Narrow scope, never depth. `cargo test -p simulation` in parallel is trustworthy. No
commit, push or `bd dolt push` without the Planner.

---

## 7. Decisions the Planner must make

| # | Decision | Evidence | Recommendation |
|---|---|---|---|
| **D1** | Builder tier: opus-uniform (2026-08-27) or sonnet builders + researcher, opus for planner/ground/advisors/gates/gosplan | evidence-log 2026-08-20: 5/5 sonnet tickets, zero send-backs; the quality lever is the gate, measured; real opus cost unmeasured, ≥2× the quoted 675k | sonnet builders, measured Plan 01 vs 02; revert per lane on a gate-rate drop |
| **D2** | Agent Teams scope: lane team from Plan 01 after Wave 0 probes pass, or after one probe Plan | flag enabled 2026-08-28; #42999 unprobed; no `/resume`; everything else file-relayed | lane team in Plan 01 **only if** probes A1–A3 pass; otherwise Plan 01 runs lanes as parallel subagents (same protocol, no teammates) and teams start in Plan 02 |
| **D3** | `.claude/` in git: keep `.claude/*` ignored with force-adds, or track agents/rules/skills/workflows/settings.json | 167 force-tracked; `gate-review.js` and the hook scripts are not; worktree agents cannot read them | track it |
| **D4** | Ratify as ADR-0001 and start Wave 0, or send back | — | — |

---

## 8. Cost (estimates until the ledger replaces them)

Measured inputs (06 §4): opus reviewer 105–113k; ledger checker ~101k; wiring ~40k; sonnet
implementer 110–155k; opus implementer unmeasured (assumed 1.5–2× sonnet); researcher 47–119k
this session. New roles: planner ~80–120k per Plan; steward 3 × 20–40k; doc-agent 60–100k per Plan.

| Lane / play | Dispatches | opus builders | sonnet builders (D1) |
|---|---|---|---|
| S · patch, solo | builder, wiring, 1 reviewer | 350–450k | 250–300k |
| M · story, race | (cartographer 0–80k), builder ∥ evidence, wiring, (ledger), 2 blind reviewers | 800k–1.1M | 600–800k |
| L · system, pair | ground, Spec-Mob ×3, builder + navigator rounds, evidence, wiring, ledger, 2 reviewers, advisor | 1.4–1.9M | 1.1–1.5M |
| Plan overhead | bet, planner, steward ×3, review, drift-auditor, doc-agent, metrics | 300–450k | same |
| **4-story Plan** (1 L + 2 M + 1 S) | | **3.7–4.8M** | **2.8–3.6M** |

A lane team does not change the token total; it changes wall-clock. Appetite is set per Plan from
the previous ledger.

---

## 9. Migration — four waves

| Wave | Scope | Scale | Exit |
|---|---|---|---|
| **0 · decide & probe** | D1–D4; `ADR-0001-gosplan.md`; restart with the teams flag; probes A1–A5 (10 §Q5) + the `PreToolUse Agent` field probe (12); `.gitignore` fix | gosplan + ≤3 probe teammates/subagents, ~150k | five probe results in `incidents.md`; ADR accepted |
| **1 · framework** | `docs/process/gosplan.md`; `incidents.md`; templates; `.planning/README.md`; fact-sheets move; `RESUME.md` move; CLAUDE.md trimmed + changelog; `/gosplan` skill; `house-rules` + `judge-rules` extracted once from the drifted copies; `docs/wiki/` skeleton; hook scripts + `bd-close.sh` + `doc-check.sh` + `check_traceability.py` installed | 1 opus writer + 1 sonnet scripts writer ∥, then `drift-auditor`, ~450k | `doc-check.sh` exits 0; hooks fire on a synthetic dispatch; no contradiction between `gosplan.md` and any frontmatter |
| **2 · roster** | 20 agent files to template (≤150 lines, trigger-only descriptions, explicit `effort`/`skills`/`memory`, tier per D1); 9 retirements; `.claude/rules/{common,geom,networking,goryak,simulation,base_mod}.md` from retired bodies; all 11 contradictions resolved | 3 sonnet writers ∥ (builders+rules · advisors+cartographer+planner+steward · gates+debugger+doc-agent+gosplan) + 1 opus `drift-auditor`, ~800k | restart; `/list-agents` shows 20; no drifted duplicate block (md5) |
| **3 · pilot** | **Plan 01**: 3–4 stories from `bd ready` (`sov-n8v` L · `sov-5yc` + `sov-q5p` M race · `sov-snw` S) through all nine stages, lane team per D2; Review with video; first Retro; `plan-metrics.sh` output | per §8 | Plan 01 closed with evidence; `retro.md` has one diff; a ledger row and a done file per dispatch; Plan 02's appetite is a measurement |

Migration before Plan 01: roughly **1.4M**.

---

## 10. Risks and what is unverified

| Risk | Mitigation |
|---|---|
| `paths:` rules may not load in subagents / teammates | probe A2; fallback per-lane skills (subagents) + `house-rules` without `paths:` (teammates) |
| `SendMessage` by name silently drops (#42999) | probe A1; done file is the completion signal regardless; name→id hook if needed |
| In-process teammates do not survive `/resume`; lead context grows with idle notices | `bd` comments + done files are the only state; ≤3 teammates; gosplan re-spawns, never messages ghosts |
| Token counts absent from hook payloads | ledger rows carry agent/story/verdict; tokens filled from the session view until a payload carries them |
| Spec-Mob, Race, lane team unverified at LLM scale here | §3.10 measures all three in Plans 01–02 |
| Breaker could kill a story whose send-backs catch real bugs | it triggers a re-shape *decision*, never an auto-defer |
| Cutting agents loses lane knowledge | every retired body diffed into a rules file first; `drift-auditor` checks each trap sentence survives |
| Dolt concurrent `--claim` from two teammates | optimistic concurrency: one wins, one re-queries (12 §5, unprobed on embedded 1.2.2) |
| Sonnet builders raise send-backs | measured; revert per lane |
| Cost numbers are estimates | replaced by ledger data after Plan 01 |

Carried from the reports: XP/Lean/SPACE secondary sources (01); harness #53 not reproduced on
Linux (02); BMAD `workflow-map.md`, SuperClaude `confidence.py` unread (03); MAST not fully read
(05); `TaskCreated/TaskCompleted` payload fields undocumented (10); `rustdoc::broken_intra_doc_links`
lint name on stable 1.97.1, `mdbook-linkcheck` on 0.5.x unverified (11); SWE-AF PR #179 evidence
not independently verified (13).

---

## Appendix A — consolidated steal list (what → where)

| From | Mechanism | Lands in |
|---|---|---|
| Scrum / Shape Up (01) | scope box, appetite, circuit breaker, DoR, retro-must-diff | §3.3, §3.4, stage 8 |
| harness (02) | 500-line cap, pushy descriptions, CLAUDE.md changelog, roster drift audit, both-sides review | §4.1, `drift-auditor`, `reviewer` |
| BMAD / prp / disler (03) | lanes, `Verify:` in the brief, hooks as enforcement | §3.4, `brief.md`, §3.7 |
| Team patterns (05) | blind verifier, blind parallel review, Spec-Mob, race, pair | §3.5, §3.6 |
| Doc frameworks (07) | ten kinds, `Verified-at`, enumerated Status, EARS | §5 |
| superpowers (08) | ledger identity line; ≤15-line report + 4 status codes; "you do not dispatch reviewers"; description = triggers only; file-conflict table; round-5 adjudication; reviewer reads a diff file; `claude -p` + `assert_contains` tests for agent behaviour | §3.3, `brief.md`, `house-rules`, `agent.md`, stage 1, §3.3, stage 5, Wave 2 exit |
| matt-skills (08) | criteria classified evidenced-complete / incomplete / unverified; independently decidable criteria; capability tiers per brief | §3.4, `brief.md` |
| code-review-skill (09) | Rust async/`unsafe`/error checklists, reuse audit, diff triage, severity alignment | `reviewer`, `.claude/rules/simulation.md`, `gate-review.js` |
| socratic (09) | Mode A pre-build contract; domain-signal → Spec-Mob participants; six agent-design questions; grade → lane | `brief.md`, stage 2, `agent.md` |
| Agent Teams (10) | lane team protocol; gates as subagents; bd-only claiming; name→id hook; probe plan | §3.6, Wave 0 |
| Docs agent (11) | `doc-agent`, coverage gate, `check_traceability.py`, mdBook wiki skeleton, `[workspace.lints]` | §5.4 |
| beads (12) | epics as Plans, lane labels, `--claim`, `list --json` metrics, three hook scripts, `bd-close.sh` | §3.3, §3.7, §3.9 |
| SWE-AF (13) | stuck-loop detection, typed debt, routing rationale in tickets, per-ticket memory → `bd` handoff comment, upstream-failure notes, fan-out cap 3, synthesizer for disagreeing verifiers | stage 3, §3.3, §3.4, stage 6, §3.6, §3.8 |
| SwarmForge (13) | durable done file as the completion invariant; `By: <role>` commit trailer; `local-*` override naming | §3.7, §4.1 |
| matt-skills engineering (14) | `debugger` gains: correct-seam framing for the regression test, the falsifiable hypothesis form "if X then changing Y flips the symptom", the ten-strategy loop-building order, an ARCHITECTURE hand-off field; three-gate ADR threshold; `arch-candidate.md` card (problem / solution / before-after / strength) that `debugger` and Spec-Mob file into the Bet queue; git hot-spots heuristic for the cartographer; two-adapter rule and dependency taxonomy in house rules and `brief.md` traps. **Not** stolen: its "fix" phase (our diagnosis-never-the-fix split stands), a third glossary store, HTML reports. No new stage — the work maps onto stages 1, 2 and Bet | `debugger.md`, `decision.md`, `docs/templates/arch-candidate.md`, `substrate-cartographer.md`, `house-rules`, `brief.md` |

## Sources

The thirteen reports under `.planning/process-overhaul-2026-08-28/` carry every citation: primary
sources verified live on 2026-08-28 (scrumguides.org; basecamp.com/shapeup; kanbanguides.org;
teamtopologies.com; dora.dev; code.claude.com/docs — sub-agents, agent-teams, workflows, hooks,
skills, memory, sessions, settings, tools-reference; diataxis.fr; adr.github.io/madr; RFC 2119/8174;
alistairmavin.com/ears); repositories at pinned SHAs (revfactory/harness cceac68; obra/superpowers
b36e0829; tt-a1i/matt-skills-with-to-goal 974c932; awesome-skills/code-review-skill 277f479;
m4vic/socratic 3cfaf6e; Agent-Field/SWE-AF 0c64fe7; unclebob/swarm-forge main; 21 more via `gh api`);
papers (arXiv 2503.13657, 2607.05391, 2409.05001, 2312.13010, 2608.25869, 2508.21433, 2406.12639,
2512.24103); anthropic.com/engineering; cognition.com; claude-code issues #42999, #48160, #58762,
#34750. Local: `.beads/issues.jsonl` (136 issues), `git log` (203 commits), evidence-log.md, the
three prior audits, `cargo +nightly rustdoc --show-coverage` output.
