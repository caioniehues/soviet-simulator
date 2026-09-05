# Report 13 — SWE-AF full read + SwarmForge delta

**Kind:** explanation
**Authority:** advisory
**Status:** active
**Owner:** researcher
**Verified-at:** 4e9e930
**Last verified:** 2026-08-28

Evidence base: SWE-AF cloned at SHA `0c64fe7cc4fc216f4d32d0b855015509750eb4aa` (2026-08-23);
SwarmForge cloned all branches, depth 30, delta computed from 2026-08-26 baseline.
Prior SwarmForge review: `docs/process/review-2026-08-26-vs-swarmforge.md` — not repeated here.
GOSPLAN reference: `docs/plan/proposals/gosplan.md` §3 and §7.

---

## Part 1 — SWE-AF (Agent-Field/SWE-AF)

### 1.1 What it is and how mature

SWE-AF is an autonomous software engineering factory. `README.md:5` describes it as "Built on
AgentField." One API call receives a natural-language goal plus a repo URL and orchestrates 22
specialized agent roles through planning, parallel issue execution, merging, verification, and GitHub
PR creation. Public beta. Last commit 2026-08-23 (five days before this report). Apache 2.0 license.
Primary language is Python (asyncio); a Go port under `go/` is now the default `af install` target.
70+ test files; CI in place; several real bugs closed (`#43`, `#49`, `#82`). Not abandoned.
Not production-hardened — "public beta" is accurate.

**Target runtime**: NOT Claude Code subagents. SWE-AF calls `claude --dangerously-skip-permissions
-p <prompt>` (or `opencode` or `codex`) as a subprocess per agent turn from an AgentField control
plane (`agentfield-package.yaml:63`). There is no `.claude/agents/` directory, no frontmatter,
no hook system. The only Claude Code integration point is a skill at
`.claude/skills/delegate-issue/SKILL.md` — a Claude Code main session can invoke SWE-AF as a
sub-harness for scoped tasks.

### 1.2 ASCII pipeline

```
INPUT: { goal, repo_url, config }
  |
  v
[Phase 1 — Plan (parallel)]
  |-- Product Manager  →  prd.md
  |-- Architect        →  architecture.md
  |-- Tech Lead        →  bounded review loop (auto-approves on exhaust)
  |-- Sprint Planner   →  Issue DAG + IssueGuidance per issue
  |-- Issue Writers    →  issue-*.md specs (parallel fan-out)
  |-- Git Init         →  integration branch + initial SHA
  |
  v
[Phase 2 — Execute Issue DAG]
  For each dependency LEVEL (Kahn's topo sort on depends_on):
    [Issues in same level run in parallel via asyncio.gather]
    Per issue — INNER LOOP (up to max_coding_iterations=5):
    |-- Coder → code + tests + commit (isolated git worktree per issue)
    |-- Default path: Reviewer → approve/fix/block
    |-- Flagged path: QA + Reviewer in parallel → Synthesizer → approve/fix/block
    |-- If fix: loop with enriched feedback
    |-- If approve: COMPLETED
    |-- If block or exhaust: → MIDDLE LOOP
    MIDDLE LOOP — Issue Advisor (up to max_advisor_invocations=2):
    |-- RETRY_APPROACH: new strategy, retry inner
    |-- RETRY_MODIFIED: relaxed ACs, retry inner; gaps → typed debt
    |-- ACCEPT_WITH_DEBT: record debt_notes, advance
    |-- SPLIT: inject sub-issues into remaining levels
    |-- ESCALATE_TO_REPLAN: → OUTER LOOP
    Level barrier gates (sequential):
    |-- Merger agent (semantic conflict resolution)
    |-- Integration Tester (if cross-boundary changes)
    |-- Debt propagation to dependents
    |-- Sub-issue injection
    |-- Replanner if FAILED_UNRECOVERABLE/ESCALATED
    |-- Checkpoint → .artifacts/execution/checkpoint.json
    OUTER LOOP — Replanner (up to max_replans=2):
    |-- CONTINUE / MODIFY_DAG / REDUCE_SCOPE / ABORT
    |-- Crash fallback: always CONTINUE, never ABORT on orchestration error
  |
  v
[Phase 3 — Verify-Fix Loop]
  Verifier → criterion-by-criterion pass/fail against PRD
  On fail: Fix Generator → fix issues → re-enter executor (bounded)
  |
  v
[Phase 4 — Repo Finalize]
  Cleanup artifacts, update .gitignore
  |
  v
[Phase 5 — Push + Draft PR + CI Gate]
  Push integration branch → GitHub PR Creator → watch Actions → bounded fix-and-repush
  |
  v
OUTPUT: BuildResult { plan, dag_state, verification, success, pr_url }
```

Fast mode (`swe-fast`): simplified single-pass planning, no full PRD/architecture, no replanning.

### 1.3 Mechanisms [source]

**Agent spawning**: each "agent" is an async task calling `app.harness()` on the AgentField control
plane, which launches the coding CLI as a subprocess (`swe_af/reasoners/pipeline.py:34`,
`swe_af/execution/coding_loop.py:618`). Concurrency via `asyncio.gather` within a dependency level.

**Communication**: no live inter-agent messaging. All state passes through function parameters and
shared disk artifacts. The `DAGState` object is the single authoritative state record.
Cross-issue shared memory is an async key-value store keyed by `codebase_conventions`,
`failure_patterns`, `bug_patterns`, `interfaces/{issue_name}`, `build_health`
(`swe_af/execution/coding_loop.py:93-141`).

**State persistence**: `DAGState` serialized to `.artifacts/execution/checkpoint.json` at every
level boundary (`docs/ARCHITECTURE.md:269`). Per-issue iteration checkpoints at
`.artifacts/execution/iterations/{build_id}/{issue_name}.json` (`coding_loop.py:48-66`).

**Task tracking**: internal only — `DAGState.all_issues`, `completed_issues`, `failed_issues`,
`skipped_issues`, `in_flight_issues`. No external tracker integration.

**Parallelism and file conflicts**: issues within a level run via `asyncio.gather`; each gets an
isolated git worktree on a dedicated branch `issue/{NN}-{slug}`. File conflicts detected at planning
time by `_validate_file_conflicts()` and annotated for the Merger — they do not block parallel
execution (`docs/ARCHITECTURE.md:186`).

**Failure/retry**: three nested loops as shown above. Stuck-loop detection: 3-iteration window
checking repeated non-blocking `fix` cycles without new file changes
(`coding_loop.py:266-279`). Non-converging stuck loop → `COMPLETED_WITH_DEBT`. Crash fallback:
default to `CONTINUE`. Fatal errors (credit exhaustion, invalid key) abort instead of retrying
silently (issue #49, closed).

**Cost control**: risk-proportional routing (2-call vs 4-call path, `coding_loop.py:662-710`).
Per-agent timeout (`agent_timeout_seconds=2700`), per-agent turn budget (`agent_max_turns=150`),
iteration/advisor/replan caps. `delegate-issue` skill warns: "Cap fan-out at 3 concurrent
delegations per repo" (`.claude/skills/delegate-issue/SKILL.md:83`). Model configurable per role.

### 1.4 Evaluation evidence [source]

**Self-reported**: custom 5-dimension scoring on a Node.js CLI todo app (`README.md:442-530`).
SWE-AF (haiku): 95/100, ~$20, ~400 agent instances. Claude Code Sonnet single-agent: 73/100.
Codex o3 single-agent: 62/100. No SWE-bench numbers claimed. The benchmark is
self-designed on a trivial task; structural hygiene advantages (modular layout, git discipline) are
mechanical outputs of having an issue-writer agent and a git-init agent, not evidence of superior
code quality. The comparison baseline is single-agent, the most favorable possible. **Treat
as unverified for quality claims; the cost and timing numbers are plausible.**

**Real production evidence**: PR #179 on the AgentField repo — 10 issues, 217 tests, 79 agent
invocations, $19.23 (`README.md:176-205`). Cited with exact numbers; PR link present but not
independently verified.

### 1.5 What SWE-AF has that GOSPLAN lacks vs what GOSPLAN has that SWE-AF lacks

**SWE-AF has; GOSPLAN lacks:**

| Mechanism | File:line | Gap in GOSPLAN |
|---|---|---|
| Stuck-loop detection | `coding_loop.py:266-279` | No detection when builder and auditor cycle without convergence |
| Typed technical debt | `ARCHITECTURE.md:223-239`, `schemas.py` | No typed schema for dropped scope (`dropped_acceptance_criterion`, `missing_functionality`, severity) |
| IssueGuidance routing block | `sprint_planner.py:43-87`, `ARCHITECTURE.md:75-88` | Lane routing rationale not captured in ticket; future agents cannot audit why S/M/L was chosen |
| Failure-note propagation | `ARCHITECTURE.md:237-239` | When an upstream story fails, dependent builders start with no structured upstream-failure context |
| Cross-issue shared memory | `coding_loop.py:113-141` | Builders in a wave re-discover the same conventions independently |
| DAG-level replanning mid-execution | `dag_utils.py`, `ARCHITECTURE.md:249-259` | No mid-wave replanning; if a ticket fails, the Plan-level circuit breaker is the only tool |
| Per-issue sub-harness entry point | `issue/build.py`, `README.md:662-776` | No formal `implement_issue` equivalent; delegation is informal |
| Parallel QA + reviewer with synthesizer | `coding_loop.py:384-426` | Race play has parallel builder + auditor but no synthesizer for conflicting signals |
| Kahn cycle detection in dependency graph | `reasoners/pipeline.py:90-93` | bd dependency graph is not cycle-checked at plan time |
| CI gate post-PR | `swe_af/execution/ci_gate.py` | No CI watch after commit; relies on human |
| Delegate-issue preflight checklist | `.claude/skills/delegate-issue/SKILL.md` | Worker delegation is informal |

**GOSPLAN has; SWE-AF lacks:**

| GOSPLAN feature | SWE-AF gap |
|---|---|
| Blind parallel review (reviewers never see producer) | SWE-AF reviewer sees coder result; no adversarial independence |
| Domain advisors (kornai-economist, logistics-modeller, etc.) | All roles are generic engineering functions; no domain knowledge |
| Evidence-not-assertion gate (sha + command output required) | Verifier checks criteria but does not require proof artifacts |
| DoR enforcement hooks (PreToolUse blocks underspecified briefs) | No hook system; harness starts on underspecified issues |
| bd external task tracker integration | Everything in-memory DAGState; no persistent external record |
| Plan as appetite box (Shape Up framing, not sprint) | Sprint planner terminology; no appetite/circuit-breaker shape |
| Spec-Mob parallel brief synthesis | Issue-writer is a single agent; no advisor+builder+auditor parallel read |
| Race play (auditor writes tests before seeing implementation) | QA always runs after the coder; no test-first adversarial separation |
| Pair play (advisor as navigator) | No equivalent |
| gosplan as synthesizing orchestrator (reads parallel outputs, writes briefs) | Orchestration is pure code; no reasoning agent that synthesizes |
| Planner consent at seven named decision points | Fully automated; no human consent model |

### 1.6 Ranked steal list

**Steal — high priority:**

1. **Stuck-loop detection** — `coding_loop.py:266-279`
   In GOSPLAN: a `SubagentStop` hook or evidence-auditor check detects when builder and auditor
   cycle on the same `fix` signal for 3 iterations without new file changes. Trigger middle-loop
   advisor action (RETRY_APPROACH) rather than exhausting the budget.
   Why: GOSPLAN's inner loop has no explicit convergence check.

2. **Typed debt schema** — `ARCHITECTURE.md:223-239`, `schemas.py`
   In GOSPLAN: a standard `bd` comment schema for accepted debt:
   `type: dropped_acceptance_criterion|missing_functionality`, `severity: high|medium|low`,
   `criterion`, `justification`. Used when an advisor accepts-with-debt or a gate relaxes a
   criterion. Propagates downstream via enriched brief context.
   Why: GOSPLAN tracks failures via bd comments but has no typed, propagatable debt record.

3. **IssueGuidance routing block** — `sprint_planner.py:43-87`, `ARCHITECTURE.md:75-88`
   In GOSPLAN: add `estimated_scope`, `touches_interfaces`, `review_focus`, and `risk_rationale`
   to every ticket at intake. These become explicit inputs to lane selection (S/M/L) and gate
   routing. gosplan documents its routing decision in the ticket.
   Why: GOSPLAN lanes exist but routing rationale is not captured; future agents cannot audit it.

4. **Cross-issue shared memory** — `coding_loop.py:113-141`
   In GOSPLAN: a `memory.json` artifact in `.artifacts/` written after each completed ticket:
   `codebase_conventions`, `failure_patterns`, `interfaces/{issue_name}`. gosplan injects relevant
   sections into each subsequent builder's brief context.
   Why: builders in a wave re-discover the same conventions. The propagation cost is near zero.

5. **Failure-note propagation to dependents** — `ARCHITECTURE.md:237-239`
   In GOSPLAN: when a bd ticket closes as failed, gosplan enriches all dependent ticket descriptions
   with a structured `upstream_failure_notes` section before dispatching the dependent builder.
   Why: dependent builders currently start with no upstream failure context and repeat the same
   wrong assumptions.

6. **Delegate-issue preflight + fan-out discipline** — `.claude/skills/delegate-issue/SKILL.md`
   In GOSPLAN: a formal sub-harness delegation checklist: (a) preflight: is the spec fully scoped?
   (b) cap: max 3 concurrent delegations; (c) poll-not-re-fire rule. The "garbage spec in →
   garbage branch out" framing is the correct warning.
   Why: GOSPLAN worker delegation is informal; this is a usable pattern document.

7. **Parallel QA + reviewer synthesizer** — `coding_loop.py:384-426`
   In GOSPLAN: in Race play on the L lane, add an explicit synthesizer step when the evidence-
   auditor and a blind reviewer disagree. Currently there is no defined resolution path for
   conflicting parallel signals.
   Why: eliminates the "who wins" ambiguity when parallel outputs conflict.

**Do not steal:**

- **AgentField control plane** (`agentfield-package.yaml`): tightly coupled to AgentField runtime
  (DID/VC, node registration, `af install`). Zero value for GOSPLAN running in Claude Code.
- **Sprint planner as God-planner** (`sprint_planner.py`): decomposes a vague goal into all issues
  without domain knowledge. GOSPLAN's Spec-Mob (advisor + builder + auditor in parallel, gosplan
  synthesizes) is architecturally superior for a domain-specific project.
- **DID/VC cryptographic attestation** (`ARCHITECTURE.md:323-373`): irrelevant overhead for a
  single-developer project.
- **Product Manager agent** (PRD from vague goal): the charter-1.0.md plays this role with human
  authority. Auto-generated PRDs conflict with binding scope.
- **GitHub PR Creator as final gate automation**: GOSPLAN prohibits commit/push without Planner
  authority; the CI gate concept is useful but the automation model is incompatible.
- **`open_code` / `codex` runtime**: GOSPLAN is Claude-only by design.

---

## Part 2 — SwarmForge delta (baseline: 2026-08-26)

Prior review not repeated. Cite `docs/process/review-2026-08-26-vs-swarmforge.md` for full baseline.

### 2a. Commits since 2026-08-26

13 commits, all on `main`. All non-adversaries branches compose from `main` via `get-swarm-forge`,
so the changes apply everywhere. Key changes:

**Reverse git_handoff (back-one / back-all)** [source: `swarm_handoff.bb:775-783`, commit `a2558d8`
2026-08-27]:
`swarmforge.conf` window lines now accept a propagation token (`forward-only` / `back-one` /
`back-all`). When the last window sends a `git_handoff`, it also queues merge-only
`non-forwarding` copies to earlier windows. The copies carry a distinct body instruction:
"The inbound tree is the structure. Replay this role's current task onto that shape."
All three packs now ship these tokens in their conf files (commits `e6ca276`, `7d903e0`, `9a4320c`,
2026-08-27). Fully built and tested — not a brainstorm item.

**Pack install no longer clobbers host or shared constitution** [source: commits `b431255`,
`5f23afb`, 2026-08-26]:
`get-swarm-forge` now composes from an allowlist: shared articles from `main`, pack-owned files
from the pack branch. Two-pack no longer ships `engineering.prompt` / `workflow.prompt` (they
overwrote main's copies). The old same-name override mechanism is removed; `local-*.prompt` files
are the only sanctioned override path.

**Per-document Attention review with per-file comments** [source: commit `1dcb0b9`, `fc85777`,
2026-08-26/08-28]:
The Attention approval flow now opens each document individually; the operator attaches per-file
comments (green/red); Approve is disabled if any file has non-empty comments. Retry delivers those
comments to the master role as audit findings. Retry now shows previous saved comments and a
two-version colored diff.

**Codex card status extraction** [source: commit `fc85777`, 2026-08-28]:
Card status extraction has a Codex-specific path using the last `•` bullet instead of the
I'll/I'm/let-me pattern.

**Merging card in dashboard** [source: `fc85777`, 2026-08-28]:
While a reverse (non-forwarding) git_handoff is in process, the lane shows a transient
light-yellow "merging card." The real forward card stays queued.

**Playwright dashboard tests** [source: commits `1e76016`, `5f23afb`, 2026-08-26]:
`test/dashboard/` is now a Playwright test suite driven from `bb test`.

**Bug fix: completed retry notes collision** [source: commit `60e9280`, 2026-08-26]:
Inbox globs missed files in `completed/`, so Retry could write a second note with the same name.
Fixed in `done_with_current_task.bb` and `pack_web.bb`.

### 2b. Previously "unbuilt" or "stale" items

**Platoon**: still unbuilt as code. The brainstorm document was expanded in commit `a2558d8`
(2026-08-27): it now specifies `platoonctl` tooling, dependency scheduling (squads blocked on
interface contracts stay unstarted), the integration-worktree pattern, a single aggregate
dashboard, naming conventions, and recovery semantics. This is a detailed design spec, not an
implementation. No `platoonctl`, no `platoonforge.conf` parser, no Lieutenant role prompt, no
platoon scripts exist in the repo. **Still unbuilt; design is now detailed enough to build from.**

**Adversaries branch**: still stale. Last commit remains `7aa2f3a` dated 2026-06-26. No new
commits. Unchanged from prior review.

**Wake-up deadlock**: the prior issues.md entry is gone — SwarmForge uses `issues.md` as a single
living item, not an accumulation. Whether the deadlock was resolved on `main` is **unverified**;
the fix on the `adversaries` branch (`7aa2f3a`) is still the only known resolution.

### 2c. Re-evaluation of two steal mechanisms vs Claude Code native hooks

**Mechanism 1: Daemon-delivered handoff files as structural completion**

Prior recommendation: steal it because delivery = card moves; no delivery = card stays.

With Agent Teams (TeammateIdle / TaskCompleted): these fire when a subagent goes idle or completes,
but they are prompt-discipline signals with no durable artifact. TeammateIdle fires on any idle;
there is no handoff object being delivered; the lead still has to interpret what "idle" means.
If the main session is restarted, prior-session TeammateIdle events are gone. Agent Teams are
experimental with harness issue #53 bugs and no `/resume` support.

**Verdict: the handoff-file invariant is still worth adopting.** The thing to steal is not the
daemon but the invariant: a worker's completion is not recorded until it produces a durable file
the lead can inspect. TeammateIdle / TaskCompleted are convenience signals, not completion proofs.
The ledger.sh hook fires on SubagentStop and extracts the final message, but if an agent exits
without producing a file, nothing stops the lead from advancing the board on a false close.

The practical form for GOSPLAN: require workers to write a handoff file (e.g.,
`.artifacts/done/<id>.json` with their bd close evidence and the Verify command output) before
calling `scripts/bd-close.sh`. gosplan does not count a story closed until it can read that file.
Compatible with both file relay and SendMessage; adds structural durability Agent Teams does not
provide.

**Mechanism 2: Machine-filled commit metadata "By <role>."**

The SwarmForge commit hook (`swarmforge/scripts/commit-msg-hook.bb`) reads `SWARMFORGE_ROLE`; if
unset, infers role from worktree path; appends `By <role>.` if absent.

GOSPLAN's ledger.sh records agent type and final message on SubagentStop. These answer different
questions: the commit hook answers "which role made this commit?" when reading `git log`; the ledger
answers "what did the agent report when it stopped?" Both are useful; neither makes the other
redundant.

**Verdict: still worth adding as a project-local `commit-msg` hook reading `GOSPLAN_ROLE`** (or the
bd issue id). The ledger.sh approach does not replace it. The role stamp in `git log` is especially
useful in GOSPLAN because workers commit in independent lanes (sim-implementer, ui-implementer,
data-implementer); the lane is immediately visible in `git log` without consulting `ledger.jsonl`.

### 2d. Constitution composition — anything new to copy into GOSPLAN

Commits `b431255` + `5f23afb` make three rules explicit and now enforced at install time:

1. **Shared articles live only on `main`** — pack branches must not ship files named
   `engineering.prompt`, `workflow.prompt`, or `handoffs.prompt`. Verified: two-pack now has only
   `articles/project.prompt`; four-pack has `local-workflow.prompt` + `project.prompt`; six-pack
   has `local-engineering.prompt` + `local-workflow.prompt` + `project.prompt`.

2. **`local-*.prompt` naming is the only sanctioned override** — local files are additive (extra
   requirements, exceptions), never full replacements. Same-name override is removed.

3. **Install composes at runtime** — `get-swarm-forge` copies shared articles from `main`, then
   overlays only pack-owned files. It does not copy host-project files.

**For GOSPLAN**: `.claude/rules/` path-scoped files serve the analogous role to `local-*.prompt`.
There is currently no convention preventing a project rule file from being named `engineering.md`
and silently affecting load order. SwarmForge's lesson: the override mechanism must be distinctly
named (`local-*`) and only additive; same-name overrides degrade silently and only surface in
production runs.

**Steal**: adopt a `local-` prefix convention for project-specific rule files that add to or
refine a global rule (e.g., `local-commit-discipline.md` rather than `commit-discipline.md`).
Global rules that a file extends keep their exact name. This makes override intent explicit and
auditable. Applies to `.claude/rules/` only — agent bodies under `.claude/agents/` are already
project-local by definition.

---

## Part 3 — Gaps and what would close them

| Gap | What would close it |
|---|---|
| SWE-AF star count unverified | `gh api repos/Agent-Field/SWE-AF --jq '.stargazers_count'` |
| SWE-AF production evidence (PR #179 on AgentField repo) | Read the PR directly; not independently verified here |
| Wake-up deadlock resolution on SwarmForge main | Read current `issues.md` and `done_with_current_task.bb` on main vs adversaries branch |
| `.claude/rules/` load order for same-name files | Wave 0 probe already planned in GOSPLAN §4.1; result decides whether `local-` prefix convention is needed |
| Reverse git_handoff utility for GOSPLAN | **unverified** whether the back-one/back-all pattern is useful in a single-lead context; needs a use case |

---

## Part 4 — Summary for the decision

**From SWE-AF**: seven mechanisms worth stealing (stuck-loop detection, typed debt schema,
IssueGuidance routing rationale, cross-issue shared memory, failure-note propagation to dependents,
delegate-issue preflight + fan-out cap, parallel QA+reviewer synthesizer). Do not steal the runtime,
the God-planner, the DID/VC governance, or the PR automation. SWE-AF's core quality mechanism
(sequential reviewer sees producer output) is architecturally inferior to GOSPLAN's blind parallel
review; the gap is measurable and already measured.

**From SwarmForge delta**: no architectural changes since 2026-08-26. The two prior steal
recommendations hold: (1) adopt the handoff-file invariant (a durable file is required before the
board advances — Agent Teams signals do not replace this); (2) add a `commit-msg` hook for
`GOSPLAN_ROLE` (the ledger.sh approach answers a different question; both are useful). Add the
`local-*` prefix convention for override-intent clarity in `.claude/rules/`. Platoon is still
unbuilt; adversaries branch is still stale.
