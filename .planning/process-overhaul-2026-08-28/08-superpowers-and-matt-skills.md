# 08 — Superpowers & Matt-Skills Research

**Kind:** gate-report  
**Authority:** advisory  
**Status:** active  
**Owner:** researcher  
**Last verified:** 2026-08-28

Sources:
- `obra/superpowers` @ `b36e0829c6d0140e93cfef2ca599b1b07d4a7797` (2026-08-12)
- `tt-a1i/matt-skills-with-to-goal` @ `974c932292f0c7cca6481ea8029c17a7dd91b063` (2026-08-21)

All claims below are `[source]` — read directly from the cloned skill files.

---

## Q1 — Protocol, artifacts, and verification for each skill

### `using-superpowers` [source: skills/using-superpowers/SKILL.md:1-65]

**Protocol:** Injected at SessionStart by `hooks/session-start` as `additionalContext` JSON. Not a skill the agent loads; it is **the bootstrap text that teaches the agent how to load skills**. Steps: (1) Before any response, including clarifying questions or codebase reads, check for applicable skills. (2) Announce "Using [skill] to [purpose]". (3) Create a todo per checklist item and follow the skill exactly. (4) Process skills (brainstorming, systematic-debugging) fire before implementation skills.

**Stop conditions:** The `<SUBAGENT-STOP>` block at the top instructs dispatched subagents to **ignore this skill entirely**. The bootstrap is for the orchestrator only.

**Forbids:** Rationalising away skill use (12-item Red Flags table). "This is just a simple question" → invalid. "I need context first" → invalid. Skill check comes before everything.

**Artifacts produced:** None directly. Its output is changed agent behaviour.

**How it fires without the user invoking it:** The `hooks/session-start` bash script reads `skills/using-superpowers/SKILL.md`, escapes it for JSON, and emits it into the platform-specific `additionalContext` field on every SessionStart event. The hook detects the runtime (Cursor, Claude Code, Copilot CLI, other) from env vars and picks the right JSON key. The skill text is then part of every turn's system prompt — no user action required.

---

### `brainstorming` [source: skills/brainstorming/SKILL.md:67-267]

**Protocol — three paths, classified on first message:**
- **Spike** (feasibility question): present probe, get nod, investigate cheaply, report recommendation as throwaway.
- **Bounded** (change to existing code): clarify → short in-chat design → **HARD GATE: stop and wait for explicit yes** → implement directly, no plan doc.
- **Architectural** (new subsystem, structural): clarify questions one at a time → propose 2–3 approaches with trade-offs → sectioned design → written spec to `docs/superpowers/specs/YYYY-MM-DD-<topic>-design.md` → spec self-review → user reviews → invoke `writing-plans`.

**Stop conditions:** The `<HARD-GATE>` block forbids any implementation action before the user approves the design. "Simple" never exempts from the approval gate; only the artifact scales, never the gate. Hidden complexity mid-task upgrades the path (one-way ratchet).

**Forbids:** Writing code before approval. Calling any implementation skill directly after brainstorming on an architectural path — only `writing-plans` may follow.

**Artifacts:** For architectural path: `docs/superpowers/specs/YYYY-MM-DD-<topic>-design.md`.

---

### `writing-plans` [source: skills/writing-plans/SKILL.md:268-440]

**Protocol:** (1) Scope check — if spec covers multiple independent subsystems, split first. (2) Map file structure with clear boundaries. (3) Size tasks: smallest unit carrying its own test cycle. (4) Write bite-sized steps (2–5 min each): write failing test → run it → implement minimal code → run tests → commit. (5) Mandatory plan header including the sub-skill directive for executors. (6) Self-review against spec: coverage, placeholder scan, type consistency. (7) Offer execution choice: Subagent-Driven (recommended) or Inline.

**Artifacts:** `docs/superpowers/plans/YYYY-MM-DD-<feature-name>.md`

**Plan header format (required):**
```
# [Feature] Implementation Plan
> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development or superpowers:executing-plans
**Goal:** [one sentence]
**Architecture:** [2–3 sentences]
**Spec:** [path]
## Global Constraints
[exact values from spec, one line each]
```

**No placeholders rule** (explicit list of forbidden phrases: "TBD", "Add appropriate error handling", "Similar to Task N", etc.).

---

### `executing-plans` [source: skills/executing-plans/SKILL.md:441-506]

**Protocol:** Worktree setup → read plan critically → raise concerns → create todos → execute step-by-step → invoke `finishing-a-development-branch`.

**Stop conditions:** Blocks (missing dep, failing test, unclear instruction), critical plan gaps, repeated verification failures.

**When used vs subagent-driven:** No subagent tools available, or user prefers inline. Slower; the SDD skill is preferred when subagent dispatch is possible.

---

### `subagent-driven-development` [source: skills/subagent-driven-development/SKILL.md:507-700, implementer-prompt.md, task-reviewer-prompt.md]

**Protocol (per-task loop):**

1. **Setup:** Create/verify isolated worktree (via `using-git-worktrees`). Run `scripts/sdd-workspace PLAN_FILE` → git-ignored directory `.superpowers/sdd/<plan-basename>/`. Create ledger at `<workspace>/progress.md` with plan file path as first line (compaction recovery).
2. **Pre-flight scan:** Scan for task conflicts, shared files, interface mismatches. Write a table to the ledger — "The scan is clean" without rows is not a scan.
3. **Dispatch implementer** with `implementer-prompt.md` (model selected per complexity tier — not session default). Implementer: read brief → ask questions if needed → implement → write tests (TDD) → commit → self-review → write full report to `[REPORT_FILE]` → reply with ≤ 15 lines (status, commits, test summary, concerns, report path).
4. **Dispatch task reviewer** (fresh subagent, `task-reviewer-prompt.md`, model chosen per diff size/risk). Reviewer: spec compliance first, then code quality. Never re-runs tests unless a specific named doubt exists. Never dispatches its own subagents.
5. **Fix loop (5 rounds max):** R≤3 resume implementer; R≥4 fresh implementer + more capable model. Each round: dispatch scoped re-review. At R=5 (breaker): adjudicate open findings — load-bearing → rule and continue or stop; non-load-bearing → park in ledger with rulings.
6. **Final whole-branch review:** `requesting-code-review/code-reviewer.md` on the most capable available model. One fix dispatch, one re-review, adjudicate residuals.
7. **Finishing:** Invoke `finishing-a-development-branch`.

**Rulings rule:** The orchestrator never stops to ask humans mid-plan unless the action is irreversible, security-sensitive, has side effects outside the worktree (merge, push, publish), or the plan is so broken every path is a guess. Everything else: decide, record in ledger as `Ruling: <what> — <why> — <what it costs if wrong>`, continue.

**Ledger:** Primary compaction-recovery mechanism. Tasks with `Task <N>: complete` are done — do not re-dispatch. Mid-loop tasks resume at next round. The ledger names commits; trust it and `git log` over memory after compaction.

**Model selection:** Mechanical isolated tasks → cheapest fast model. Integration/judgment tasks → standard. Architecture/design and the final whole-branch review → most capable.

**What the orchestrator verifies rather than trusts:** The implementer's report is unverified claims. The reviewer compares the report against the diff directly. A stated rationale ("left it per YAGNI") never downgrades a finding's severity.

---

### `dispatching-parallel-agents` [source: skills/dispatching-parallel-agents/SKILL.md:709-870]

**Protocol:**
1. Identify independent domains (failures in different test files / subsystems with no shared state).
2. Craft focused prompts: specific scope, clear goal, constraints ("Do NOT change production code"), expected output format.
3. Issue all subagent dispatches **in the same response** — multiple calls in one response = parallel; one per response = sequential.
4. On return: review each summary, verify fixes don't conflict, run full test suite, integrate.

**When not to use:** Related failures (fix one might fix others), need full system context, exploratory debugging, shared state between agents.

**Prompt structure requirements:** (a) Focused — one problem domain. (b) Self-contained — all context needed. (c) Specific output spec — what should the agent return.

---

### `verification-before-completion` [source: skills/verification-before-completion/SKILL.md:878-998]

**Protocol — the Gate Function:**
1. IDENTIFY: what command proves this claim?
2. RUN: execute it fresh and complete
3. READ: full output, check exit code, count failures
4. VERIFY: does output confirm the claim?
5. ONLY THEN: make the claim

**Stop conditions:** Any wording implying success without having run verification. "Should", "probably", "seems to" are red flags.

**Red flags table:** Trusting agent reports; partial verification; "linter passed" ≠ build; expressing satisfaction before verification.

---

### `finishing-a-development-branch` [source: skills/finishing-a-development-branch/SKILL.md:1000-1200]

**Protocol:** (1) Run full test suite — stop and report on failure. (2) Detect environment: `GIT_DIR == GIT_COMMON` (normal) vs linked worktree vs detached HEAD. (3) Determine base branch — confirm before merging. (4) Present exactly the defined menu (3 options for normal/named branch; 2 for detached HEAD). (5) Execute choice with merge-then-test-then-cleanup ordering. (6) Cleanup: `git worktree remove + prune` for Superpowers-created worktrees; others left in place.

---

### `requesting-code-review` [source: skills/requesting-code-review/SKILL.md + code-reviewer.md]

**Protocol:** Get base and head SHAs → dispatch `code-reviewer.md` template as general-purpose subagent → act on feedback: Critical = fix immediately; Important = fix before proceeding; Minor = note for later; push back on wrong findings with technical reasoning.

**Reviewer brief template** (`code-reviewer.md`): description, plan/requirements, git range (`BASE_SHA..HEAD_SHA`), then sections: Plan alignment, Code quality, Architecture, Testing, Production readiness. The reviewer is told explicitly: do not dispatch subagents; do not re-run tests unless a specific doubt; read-only on checkout.

**Severity scheme:** Critical (bugs, security, data loss, broken functionality) / Important (architecture problems, missing features, poor error handling, test gaps) / Minor (style, optimisation, docs polish).

**Format:** Strengths → Issues (Critical / Important / Minor) → Recommendations → Assessment (Ready to merge? Yes / No / With fixes).

**Blindness rule:** Reviewer gets precisely crafted context, never the orchestrator's session history. This keeps the reviewer on the work product, not the thought process.

---

### `receiving-code-review` [source: skills/receiving-code-review/SKILL.md:1299-1500]

**Protocol:** READ complete feedback → UNDERSTAND (restate in own words or ask) → VERIFY against codebase → EVALUATE for this codebase → RESPOND with technical acknowledgment or reasoned pushback → IMPLEMENT one item at a time, test each.

**Forbidden responses:** "You're absolutely right!", "Great point!", "Let me implement that now" (before verification). No gratitude expressions of any kind.

**When to push back:** Suggestion breaks existing functionality, reviewer lacks context, violates YAGNI, technically incorrect, legacy reasons, conflicts with architectural decisions.

**From external reviewers:** Five checks before implementing: technically correct for this codebase? breaks existing functionality? reason for current implementation? works on all platforms/versions? does reviewer understand full context?

**Partial understanding rule:** If items 4 and 5 of 6 are unclear, ask for clarification on all unclear items before implementing any. "Partial understanding = wrong implementation."

---

### `systematic-debugging` [source: skills/systematic-debugging/SKILL.md:1503+]

**Protocol — four phases, all required in order:**
1. Root cause investigation: read errors, reproduce consistently, check recent changes, gather evidence with diagnostic instrumentation per component boundary, trace data flow.
2. Pattern analysis: find working examples, read reference implementations completely, identify differences.
3. Hypothesis and testing: single hypothesis stated explicitly, smallest possible change to test, verify before continuing. If unknown: say so, don't pretend.
4. Implementation: failing test case first, single fix, verify. If ≥3 fixes have failed: stop and question the architecture.

**Iron law:** "NO FIXES WITHOUT ROOT CAUSE INVESTIGATION FIRST"

---

### `test-driven-development` [source: skills/test-driven-development/SKILL.md:1703+]

**Protocol — Red-Green-Refactor:**
- RED: write one minimal test, run it, confirm it fails for the right reason (not an error).
- GREEN: write minimal code to pass (YAGNI), run tests.
- REFACTOR: clean up while keeping green.

**Iron law:** "NO PRODUCTION CODE WITHOUT A FAILING TEST FIRST. Write code before the test? Delete it."

---

### `using-git-worktrees` [source: skills/using-git-worktrees/SKILL.md:1905+]

**Protocol:** (0) Detect isolation first with `GIT_DIR`/`GIT_COMMON` comparison (submodule guard: check `--show-superproject-working-tree`). Already in a worktree → skip creation. (1a) Native worktree tool preferred (`EnterWorktree`, `/worktree`, etc.). (1b) Git fallback: use `.worktrees/` or `worktrees/` if found, default to `.worktrees/`, verify `git check-ignore`, then `git worktree add`. (2) Project setup (cargo build, npm install, etc.). (3) Verify clean baseline — ask before proceeding past failures.

---

### `writing-skills` [source: skills/writing-skills/SKILL.md:2074+]

**Protocol — TDD applied to skill documentation:**
- RED: run a pressure scenario with a subagent WITHOUT the skill. Document exact rationalizations the agent uses.
- GREEN: write the skill addressing those specific violations. Verify the agent now complies.
- REFACTOR: find new rationalizations → plug → re-verify.

**Skill description rule:** Description = triggering conditions ONLY. Never summarize the skill's workflow in the description. Reason: tested and confirmed that a description summarizing workflow causes agents to follow the description instead of reading the skill body. A description saying "code review between tasks" caused an agent to do ONE review where the flowchart showed TWO. [source: skills/writing-skills/SKILL.md:2225-2234]

**Frontmatter:** `name` + `description` ≤ 1024 chars total. `description` starts with "Use when…" and describes symptoms/triggering conditions, not process.

---

## Q2 — Trigger mechanism

**SessionStart hook** [source: hooks/hooks.json, hooks/session-start]:
- `hooks/hooks.json` registers a `SessionStart` matcher for "startup|clear|compact" events.
- On match, `hooks/run-hook.cmd session-start` runs `hooks/session-start` (a bash script).
- The script reads `skills/using-superpowers/SKILL.md`, JSON-escapes it, and emits it as platform-specific additional context injected into every new session's system prompt.
- The `using-superpowers` skill text then instructs the agent: "If you think there is even a 1% chance a skill might apply, you ABSOLUTELY MUST invoke the skill." This creates the auto-trigger: the bootstrap sets the behaviour rule; skills auto-trigger because the rule is always in context.

**Skills reference each other** via explicit sub-skill invocations:
- `brainstorming` → `writing-plans` (architectural path terminal state)
- `writing-plans` → `subagent-driven-development` or `executing-plans`
- `subagent-driven-development` → `using-git-worktrees`, `requesting-code-review`, `finishing-a-development-branch`
- `executing-plans` → `finishing-a-development-branch`
- `systematic-debugging` → `test-driven-development`, `verification-before-completion`

**The `<SUBAGENT-STOP>` fence** at the top of `using-superpowers` exempts dispatched subagents from the bootstrap rule: "If you were dispatched as a subagent to execute a specific task, ignore this skill." Subagents get task-specific prompts, not the orchestrator's bootstrap.

---

## Q3 — Subagent-driven-development and parallel agents in depth

### Task granularity
Tasks are sized to carry their own test cycle and be independently reviewable. "The smallest unit where a reviewer could meaningfully reject one while approving its neighbor." Steps within a task are 2–5 minutes each.

### Worktree use
Every plan gets an isolated workspace from `scripts/sdd-workspace PLAN_FILE`. The workspace is git-ignored at `.superpowers/sdd/<plan-basename>/`. Different plans never share workspaces. Native worktree tools preferred over manual `git worktree add`.

### How results merge
The ledger at `<workspace>/progress.md` is the integration record. Each task completion is appended. After all tasks, the final whole-branch reviewer reads the cumulative diff from `BASE_SHA..HEAD_SHA`. The orchestrator runs `cargo test` / `npm test` / etc. as the integration point between tasks in the Race-equivalent pattern.

### Conflict avoidance
The pre-flight scan before Task 1 builds a table of every pair of tasks that share a file or interface. The scan must have rows — "clean" without them is invalid. Rulings on conflicts are logged before dispatch. The ledger is the shared state medium.

### Failure handling
- R≤3: resume the same implementer with findings (subagent resume, `SendMessage` to agent ID).
- R≥4: fresh implementer on a more capable model.
- R=5 (circuit breaker): adjudicate each open finding — load-bearing → rule and continue or stop; non-load-bearing → park with ruling in ledger.
- Final whole-branch review: one fix dispatch only, one re-review, then adjudicate residuals.

### Parallel dispatch
For independent problems: craft focused prompts (scope, goal, constraints, expected output) and issue ALL dispatch calls in the same response. Multiple calls in one response = concurrent execution.

---

## Q4 — Review protocols

**Reviewer brief** (`code-reviewer.md`): description of what was built, plan/requirements, git range. Sections: Plan alignment (matches requirements? deviations justified?), Code quality (separation of concerns, error handling, DRY, edge cases), Architecture (design, scalability, security, integration), Testing (real behavior vs mocks, edge cases, integration), Production readiness (migration, compat, docs, bugs).

**Task reviewer** (`task-reviewer-prompt.md`): Spec compliance (Missing/Extra/Misunderstood) first, then Code quality. Reads a pre-generated diff file — never reads changed files separately unless a hunk is cut off mid-function. Treats the implementer's report as unverified claims. Rationales stated by the implementer ("left it per YAGNI") never downgrade findings.

**Severity:** Critical (must fix: bugs, security, data loss, broken) / Important (should fix: architecture problems, missing features, poor error handling, test gaps, verbatim logic duplication, swallowed errors, tests that assert nothing) / Minor (nice to have: style, optimisation, docs).

**Blindness rule:** Reviewer gets precisely crafted context — never the orchestrator's session history. The two whole-branch reviewers in SDD are dispatched fresh and independently (no anchoring).

**Author response:** Technical acknowledgment or reasoned pushback. No performative agreement ("You're absolutely right!"), no gratitude. Push back when technically incorrect, YAGNI violation, or reviewer lacks context. Acknowledge correct feedback by fixing it, not by praising it.

---

## Q5 — Writing-skills and evals

**Evals harness** (`tests/claude-code/`): Shell scripts that run `claude -p` (headless) with specific prompts and assert on output using a library of helpers: `run_claude`, `assert_contains`, `assert_not_contains`, `assert_count`, `assert_order`. Tests are `bash` + string matching, not an LLM grader.

**Test structure:** Each test sources `test-helpers.sh`, runs Claude with a prompt, asserts expected patterns. The `test-subagent-driven-development.sh` example: asks the agent to describe SDD workflow, then asserts keyword presence ("self-review", "worktree") and ordering ("First: spec compliance → Second: code quality"). [source: tests/claude-code/test-subagent-driven-development.sh]

**The `writing-skills` process** is the reusable testing framework for agent definitions:
1. Run a pressure scenario WITHOUT the skill — document exact rationalizations.
2. Write the skill to address those specific failure modes.
3. Re-run the scenario WITH the skill — verify compliance.
4. Find new rationalizations (loopholes), close them, re-verify.

**What is reusable for GOSPLAN:** This exact pattern — run the agent without the rule, document the failure mode, write the rule, verify — can test any agent definition, hook, or brief field. The headless `claude -p` invocation pattern transfers directly. The key insight: the description's trigger condition must describe *when* to use (symptoms), never *how* to use (workflow), because agents short-circuit by following the description rather than the body.

---

## Q6 — Matt-skills: the goal mechanism and what differs from superpowers

### The goal pipeline [source: skills/engineering/to-goal/SKILL.md]

Matt-skills introduces a full spec-to-execution pipeline absent in superpowers:

1. **`to-spec`**: Takes the current conversation, synthesizes it into a spec (no re-interview), publishes to issue tracker, classifies as fork-ready or `to-tickets` needed. Emits a `SPEC READY` block with source URL, baseline, test seam, non-goals, external authority.

2. **`to-tickets`**: Breaks a spec into tracer-bullet vertical slices — each cuts through all layers and is independently demoable. Publishes as tracker issues with native blocking edges. Quizzes the user on granularity and blocking edges before publishing.

3. **`to-goal`** (the key differentiator): Takes an approved spec, tracker frontier, or partially implemented ticket and compiles a **copy-pasteable execution goal** for a fresh agent session. Crucially: no re-interview, no re-planning — it synthesizes from evidence already in the tracker.

**What `to-goal` produces** — the Goal template:
```
## Goal          [one ticket-scoped outcome]
## Current state [branch, HEAD, dirty files, evidenced-complete, known gaps, existing failures]
## Execution order [dependency-respecting path through this ticket]
## Completion criteria [each ticket criterion as a checkbox, plus: "Ran smallest applicable validation", "Ran code-review flow against pre-implementation fixed point", "Commit only after all criteria pass and source authorizes"]
## Constraints  [fixed list: no push/PR/merge/close issues without authorization]
## Context      [source ticket, design docs, test seam, inspect-first commands]
```

**Session recommendation** (portable, not model-specific): Capability tier (Lightweight/Standard/Advanced) × Reasoning intensity (Low/Medium/High), with one evidence-based reason. Model named only when the target harness's available models are known from context.

**Readiness checklist** (must all pass before drafting goal):
- Source is agent-ready: every product decision in evidence
- Ticket is unblocked
- Exactly one frontier (no silent combining)
- Pre-implementation HEAD recorded as code-review fixed point
- Every acceptance criterion classified: evidenced complete / demonstrably incomplete / unverified
- Validation commands discovered from repo scripts, CI, existing tests
- Every completion criterion independently decidable (no "looks good")

**`spec-executor`**: A forked-thread skill that finds the `SPEC READY` block in conversation history, locks scope with an execution lock message, implements with TDD, runs `/code-review` against the pre-implementation fixed point, and returns a `SPEC EXECUTION RECEIPT`. The receipt is the subagent's return value.

**`triage`**: State machine for issues and PRs: needs-triage → needs-info / ready-for-agent / ready-for-human / wontfix. Verifies claims (reproduces bugs, checks out PRs), grills if needed, posts agent-ready briefs.

**`wayfinder`**: For work too large for one agent session — breaks it into a shared map of *decision tickets* on the tracker (each ticket is a question, not a build slice). Works the frontier one decision at a time. The map body is an index (gist + link per closed ticket), not a store.

### What differs from superpowers

| Feature | Superpowers | Matt-skills |
|---|---|---|
| Scope | General engineering patterns | Full spec-to-execution pipeline + tracker integration |
| Goal artifact | Plan file in `docs/superpowers/plans/` | Copy-pasteable Goal block from `to-goal` |
| Tracker integration | None | Native (GitHub Issues, Linear, local `.scratch/`) |
| Session handoff | Ledger in `.superpowers/sdd/` | `SPEC READY` / `SPEC EXECUTION RECEIPT` blocks in conversation history |
| Skills invocable by model | All except where `disable-model-invocation: true` | Explicit flag per skill; `ask-matt` is the router for user-reachable skills |
| Evals | Shell + headless claude-p assertions | No dedicated eval harness (behavioral assertions via tracker state) |
| Readiness gate | Pre-flight scan before Task 1 | `to-goal` readiness checklist before compiling a goal |
| Decision tracking | Ledger rulings | Wayfinder decision tickets on tracker |

---

## Q7 — Steal list, ranked

### STEAL (ranked highest to lowest value for GOSPLAN)

**1. Ledger-as-compaction-recovery** (SDD SKILL.md:637-659)
→ **What to steal:** `.superpowers/sdd/<plan>/progress.md` pattern — first line is the plan path (identity check), `Task N: complete` lines are resumption markers, ledger records rulings and commits. After compaction, trust the ledger and `git log` over memory.
→ **What it becomes in GOSPLAN:** `ledger.jsonl` already proposed in §3.7 of gosplan.md. Adopt the "first line is the plan identity" convention to prevent the wrong-plan resume failure. Add `Ruling: <what> — <why> — <cost if wrong>` as a required format for all mid-plan orchestrator decisions.
→ **Why:** The doc says compaction is "the single most expensive failure observed" in practice. Our `ledger.jsonl` exists but does not have a resume-safe identity check.

**2. Implementer report contract** (implementer-prompt.md)
→ **What to steal:** The ≤15-line report format (status, commits, test summary, concerns, report file path), the four status codes (DONE / DONE_WITH_CONCERNS / BLOCKED / NEEDS_CONTEXT), and the rule "write full detail to [REPORT_FILE]; the brief return is the signal".
→ **What it becomes in GOSPLAN:** Our brief template's "report format" field. Add the four status codes and the two-file convention (full report to a file; ≤15-line signal to the orchestrator).
→ **Why:** Our current reports have no defined length or status vocabulary. Workers sometimes return walls of text that choke the orchestrator's context.

**3. Implementer self-ban on spawning reviewers** (implementer-prompt.md, task-reviewer-prompt.md)
→ **What to steal:** "You Do Not Dispatch Subagents" block — present in BOTH the implementer and reviewer prompts. Implementer: never spawn a reviewer, never spawn a sub-implementer. Reviewer: never spawn a second reviewer. The controller dispatches every agent.
→ **What it becomes in GOSPLAN:** A required paragraph in every builder and gate agent body. Currently implicit; make it explicit and add it to the house-rules skill.
→ **Why:** Agents that spawn their own reviewers duplicate costs and produce verdicts that count for nothing in the process (the superpowers doc says this explicitly).

**4. Description = triggering conditions, never workflow** (writing-skills SKILL.md:2222-2234)
→ **What to steal:** The tested observation that a description summarizing workflow causes agents to follow the description instead of reading the skill body. Verified with a concrete case: "code review between tasks" → agent does ONE review; changed to just the triggering condition → agent reads the flowchart and does TWO.
→ **What it becomes in GOSPLAN:** A rule for every agent description and skill description field. Current agent descriptions summarize what the agent does, not when to use it. Rewrite them during Wave 2.
→ **Why:** This is an empirically demonstrated failure mode, not speculation. It is directly relevant to GOSPLAN's agent roster rewrite.

**5. Pre-flight conflict scan before Task 1** (SDD SKILL.md:665-689)
→ **What to steal:** Before dispatching any task, build a table: every pair of tasks sharing a file or interface (what one produces vs what the other consumes), and every task whose own text contradicts itself. Write the table to the ledger. Rule on every conflict before execution. "The scan is clean without those rows is not a scan you ran."
→ **What it becomes in GOSPLAN:** A mandatory step in `gosplan`'s brief-dispatch protocol for M and L stories. Currently we have "traps" in the description but no pre-dispatch conflict table across the tasks of a Plan.
→ **Why:** Interface mismatches between tasks that surface at integration time are expensive. The table forces the orchestrator to reason about the seams before dispatch.

**6. Circuit breaker with adjudication** (SDD SKILL.md: R=5 block)
→ **What to steal:** At round 5, do not keep sending back — adjudicate. Load-bearing findings: rule and continue or stop. Non-load-bearing: park in ledger with ruling. This ends the loop, not the work.
→ **What it becomes in GOSPLAN:** A precise definition for GOSPLAN's circuit breaker. Currently §3.3 says "sent back twice → re-shape decision by the Planner". The SDD model provides the intermediate step: adjudication before escalation. The Planner is consulted only on load-bearing open findings after adjudication.
→ **Why:** The existing breaker skips adjudication. Two send-backs might resolve with different findings at each pass; lumping them into a Planner escalation wastes a decision point.

**7. Reviewer gets diff file, not file reads** (task-reviewer-prompt.md)
→ **What to steal:** The controller pre-generates a diff package (via `scripts/review-package PLAN BASE HEAD`), writes it to a path in the workspace, and the reviewer reads that file once. The reviewer "does not crawl the broader codebase" and inspects code outside the diff only for a concrete, named risk, with one focused check per named risk.
→ **What it becomes in GOSPLAN:** Change the gate brief to: orchestrator generates `git diff --stat BASE..HEAD && git diff BASE..HEAD` into a file; gate agent reads that file; `get_review_context_tool` supplements only for named risks. This also prevents gates from reading stale graph nodes as the source of truth.
→ **Why:** Our current gates read files; some read stale graph edges. Pre-generating the diff fixes the scope problem and eliminates the graph-vs-source ambiguity for the gate's first pass.

**8. `to-goal` readiness checklist** (to-goal SKILL.md: readiness-checklist block)
→ **What to steal:** The seven readiness checklist items, especially: (a) every acceptance criterion classified as evidenced-complete / demonstrably incomplete / unverified; (b) validation commands discovered from the repo's own scripts, CI, and existing tests; (c) every completion criterion independently decidable (no "looks good").
→ **What it becomes in GOSPLAN:** The DoR (Definition of Ready) gate check. Our current DoR lists fields but not these three classification requirements. The "independently decidable" criterion directly prevents the "acceptance criteria as aspirational prose" failure mode.
→ **Why:** Our acceptance criteria are sometimes unverifiable because the verification command's "expected output shape" is not specified. The `to-goal` checklist makes that gap machine-visible.

**9. Session recommendation with portable capability tiers** (to-goal SKILL.md: session-recommendation)
→ **What to steal:** The three capability tiers (Lightweight / Standard / Advanced) × three reasoning intensities (Low / Medium / High) with one evidence-based reason. Name a model only when the target harness's available models are known.
→ **What it becomes in GOSPLAN:** The brief template's "play" field. Currently we specify the build play (Solo/Race/Pair) but not the capability tier. Adding capability + intensity to the brief makes the model selection decision explicit and auditable.
→ **Why:** This matches GOSPLAN's D1 decision — sonnet builders for Standard mechanical work, opus for Advanced judgment.

**10. Skill test helper pattern** (tests/claude-code/test-helpers.sh, test-subagent-driven-development.sh)
→ **What to steal:** The `run_claude "prompt" [timeout]` + `assert_contains/assert_order/assert_count` pattern using headless `claude -p`. Tests that probe agent behavior by asking it to describe the skill and asserting keyword/ordering presence.
→ **What it becomes in GOSPLAN:** A `tests/skills/` directory with shell tests for each GOSPLAN skill. Verification that the Wave 0 probes (rules-in-subagents, SubagentStop payload) can be automated as regression tests.
→ **Why:** We currently have no automated tests for agent behavior. Adding even the simple assert_contains pattern gives us a regression layer that catches skill regressions before they reach a Plan.

---

### DO NOT STEAL

**SDD as the build play** — the full SDD loop (implementer → task reviewer → re-review × 5 → final reviewer) is the equivalent of GOSPLAN's Race play. We already have a more domain-specific version with the Race play (builder ∥ evidence-auditor, wiring-auditor, ledger-checker, two blind reviewers). The SDD loop adds the task-scoped reviewer between tasks; GOSPLAN does this at the gate layer. Adopting SDD wholesale would add a third review tier without clear benefit over our gate chain. **Cost:** duplicated review overhead; our gates already exceed SDD's task-reviewer in depth.

**Superpowers' `brainstorming` skill as-is** — it targets human-led conversational design; GOSPLAN's Refine phase is gosplan-led with a pre-written brief template and Spec-Mob for L stories. The brainstorming flow (ask questions → propose approaches → write spec → get approval) is for the user, not for gosplan. The approval gate it encodes is already in GOSPLAN's Planner decision points. **Reason:** we already have a more structured equivalent; importing brainstorming would duplicate the DoR/Spec-Mob mechanism.

**Matt-skills' tracker-native blocking edges** — `to-tickets` uses GitHub Issues or Linear's native sub-issue relationships. We use `bd` (beads), which has `bd dep --blocks`. The concept transfers but the implementation is already in place. **Reason:** don't re-implement `bd dep`.

**Matt-skills' `wayfinder`** — our equivalent is the Plan betting phase plus the Planner's charter. Wayfinder targets large, foggy exploration across sessions; our Plans are bounded (3–6 stories with an appetite). If a piece of work is too large for a Plan, the Planner breaks it into a separate Plan, which is simpler than the wayfinder map mechanism. **Reason:** token cost of maintaining a decision-ticket map for our scale is not justified.

**The `<SUBAGENT-STOP>` fence approach for the bootstrap** — our subagents already receive task-specific prompts and do not get the session context; the fence is not needed because our dispatch pattern (brief-only context) already prevents the bootstrap from reaching subagents. **Reason:** we solve this at the dispatch level, not the skill level.

---

## Gaps and what would close them

1. **Eval harness for our agent definitions:** The superpowers `tests/claude-code/` pattern is known; implementing it requires the `claude -p` headless mode and a decision on which behaviors to assert (skill trigger order, report format, self-ban on spawning reviewers). Wave 3 probe could add the first two shell tests.

2. **Compaction failure in multi-hour Plans:** The superpowers ledger recovery pattern is well-documented, but we have not measured how often our Plans span compaction events. The risk is real; the Wave 3 pilot Plan is the right moment to validate that the `ledger.jsonl` first-line identity pattern prevents wrong-plan resumes.

3. **Whether `.claude/rules/` paths load inside subagents:** Superpowers does not use this mechanism (it uses `skills:` preloads). Our Wave 0 probe will settle this; until then, the fallback (per-lane preloaded skills) is documented in gosplan.md §4.1.

4. **`SPEC EXECUTION RECEIPT` pattern:** The matt-skills receipt block is a clean way to surface what the subagent actually did vs claimed. Our current evidence requirement ("sha + command output") covers the same ground but is less structured. The receipt format would improve gate reviews that compare the builder's report against the diff.
