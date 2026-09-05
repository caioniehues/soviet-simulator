# Comparable Repos Survey — Process Overhaul 2026-08-28

**Purpose:** Survey repositories that encode a software-development process/team for coding agents.
Feeds the redesign of soviet-simulator's 8-phase gated cycle + 22-agent roster.
**Verified:** 2026-08-28. Tags: [source] = gh API / repo read; [live] = gh search result; [doc] = README/CLAUDE.md direct read.

---

## Candidate Registry (all repos verified alive)

| Repo | Stars | Last push | License | Target tool |
|---|---|---|---|---|
| bmad-code-org/BMAD-METHOD | 52 418 | 2026-08-28 | MIT | Claude Code, Cursor, web GPT |
| obra/superpowers | 278 894 | 2026-08-19 | MIT | Claude Code, Codex, Antigravity, 12+ harnesses |
| SuperClaude-Org/SuperClaude_Framework | 23 848 | 2026-08-21 | MIT | Claude Code |
| wshobson/agents | 39 218 | 2026-08-26 | MIT | Claude Code, Codex, Cursor, OpenCode, Antigravity |
| Wirasm/prp | 2 239 | 2026-08-28 | MIT | Claude Code, Codex CLI |
| FoundationAgents/MetaGPT | 70 081 | 2026-01-21 | MIT | Python framework (LLM-agnostic) |
| eyaltoledano/claude-task-master | 28 029 | 2026-04-28 | MIT+CC | Cursor, Claude Code, Windsurf, Roo |
| OpenBMB/ChatDev | 34 140 | 2026-07-24 | Apache-2.0 | Python framework |
| hesreallyhim/awesome-claude-code | 53 121 | 2026-08-28 | none | curated list |
| coleam00/context-engineering-intro | 13 804 | 2026-03-16 | MIT | Claude Code |
| Pimzino/claude-code-spec-workflow | 3 853 | 2025-09-07 | MIT | Claude Code → MCP |
| buildermethods/agent-os | 5 342 | 2026-05-05 | — | Claude Code, Cursor, Antigravity |
| snarktank/ai-dev-tasks | 7 791 | 2025-11-05 | MIT | any AI coding tool |
| disler/claude-code-hooks-mastery | 3 905 | 2026-03-04 | MIT | Claude Code |
| OneRedOak/claude-code-workflows | 3 887 | 2025-09-14 | none | Claude Code |
| Caspian-Sun/claude-code-workflow | 10 | 2026-08-28 | MIT | Claude Code |
| marcusgoll/Spec-Flow | 91 | 2026-04-23 | MIT | Claude Code |
| DanielPodolsky/ownyourcode | 276 | 2026-06-27 | MIT | Claude Code |
| maslennikov-ig/template-bridge | 73 | 2026-06-09 | MIT | Claude Code (Superpowers + Beads) |
| catlog22/Claude-Code-Workflow | 2 137 | 2026-06-18 | MIT | Claude Code (→ Maestro Flow) |
| VoltAgent/awesome-claude-code-subagents | 24 710 | 2026-08-12 | MIT | Claude Code |

**Searched but not found / not matching:** github/spec-kit (no public repo), ruvnet/claude-flow
(redirects to ruvnet/ruflo — 69 609 stars, a swarm orchestrator platform, not a dev-cycle framework).

---

## Deep-Read Reports (Top 6)

### 1. bmad-code-org/BMAD-METHOD [source]

**URL:** https://github.com/bmad-code-org/BMAD-METHOD  
**Stars:** 52 418. **License:** MIT. **Active:** daily commits.

#### Process shape

```
Clarify → Plan → Build+Verify → Learn+Adjust  (loop)
```

Vague idea enters at Clarify; a known, small change goes directly to Build. The loop is
explicit in README as an SVG diagram. Three depth tiers inside the loop:
- Trivial change: straight to `bmad-build`
- Medium feature: `bmad-spec` generates a spec doc, then `bmad-build` consumes it
- Complex / new product: full ideation skills (brainstorm, deep-recon, pressure-test), then
  spec, then build

Roles surfaced in docs/reference/agents.md — not seen directly, but structure in src/
has two skill families: `core-skills/` and `bmm-skills/`. Install command: `npx bmad-method install`.

#### Orchestration

Skills are Claude Code Agent Skills (SKILL.md files) installed via `npx bmad-method install`.
Agents are defined as `.md` files. Context passes through **artifact files on disk** — spec docs,
architecture docs, story files — which each downstream skill reads as its input. State between
sessions = the artifact files committed to the repo. No external DB; no MCP server needed
(web bundles for planning exist as Gemini Gems / ChatGPT GPTs).

#### Documentation patterns

- Artifacts: PRD, architecture doc, story file — each produced by a named skill
- `docs/reference/workflow-map.md` — canonical routing guide
- AGENTS.md at root — minimal (465 bytes), defers to docs
- Extensive external docs site at docs.bmad-method.org

#### Planning vs building

Explicit split. Planning skills (brainstorm, recon, spec) run in chat / web bundles; building
skill (`bmad-build`) writes and reviews code. Human makes all scope decisions at plan-to-build
handoff. `bmad-help` as a meta-skill that guides the human when they are unsure what comes next.

#### Tracking / Review / Human role

- No built-in issue tracker (pairs with external tools)
- Build skill includes a review step but the mechanism is not a named gate agent — it is
  inline within `bmad-build`
- Human controls the loop: no autonomous multi-epic runs without the `bmad-loop` add-on module
  (BMad Loop "builds, verifies, and retros a whole epic unattended")
- Cost: no explicit budget controls in core; token efficiency not a stated goal

**Distinctive idea:** Right-sized process — the methodology auto-routes based on change size.
A trivial edit never touches planning. A large initiative gets as much depth as it needs.
The process is a funnel, not a railroad.

**Likely failure mode:** The spec-to-build handoff relies entirely on the quality of the spec
artifact. If the spec is ambiguous, `bmad-build` has no adversarial checker to catch it before
the agent starts writing code.

---

### 2. obra/superpowers [source]

**URL:** https://github.com/obra/superpowers  
**Stars:** 278 894. **License:** MIT. **Active:** 2026-08-19.

#### Process shape

```
using-superpowers (bootstrap) →
  brainstorming →
  writing-plans →
  executing-plans (subagent-driven-development loop) →
  verification-before-completion →
  finishing-a-development-branch →
  requesting-code-review / receiving-code-review
```

Skills directory enumerates the full set: `brainstorming`, `writing-plans`, `executing-plans`,
`subagent-driven-development`, `verification-before-completion`, `finishing-a-development-branch`,
`requesting-code-review`, `receiving-code-review`, `systematic-debugging`, `test-driven-development`,
`dispatching-parallel-agents`, `using-git-worktrees`, `writing-skills`.

The `using-superpowers` bootstrap auto-triggers when the agent fires up — this is the key mechanic.
Skills auto-fire at the right moments without the human opting in per session.

#### Orchestration

Skills auto-trigger via the bootstrap. `subagent-driven-development` skill dispatches subagents
per engineering task, inspects their work, and loops. Parallel dispatch uses git worktrees so
subagents operate on isolated branches. Multi-harness: same skills work on Claude Code, Codex,
Antigravity, Devin CLI, Gemini CLI, etc. Context passes through the plan file written by
`writing-plans`. Session state = plan file + git history.

The project has a 94% PR rejection rate and highly evolved quality standards for contributions;
contributors must disclose model, harness, and version.

#### Documentation patterns

- Skills as self-contained directories under `skills/`
- No frontmatter schema in core (frontmatter lives in CLAUDE.md / AGENTS.md for each harness)
- `CLAUDE.md` is the harness bootstrap — 8 873 bytes
- Per-harness plugin directories: `.claude-plugin/`, `.codex-plugin/`, `.cursor-plugin/`, etc.

#### Planning vs building

Planning is explicit: agent brainstorms, produces a spec, human signs off, then `executing-plans`
runs. Implementation is subagent-driven so the orchestrator does not write code — it dispatches,
verifies, and loops. TDD is built in as a named skill.

#### Tracking / Review / Human role

- `requesting-code-review` / `receiving-code-review` as named skills — structured review loop
- No issue tracker; task state lives in the plan file and git
- Human role: approves spec, monitors, handles blocked sub-agents; can run autonomously for hours
- Evals harness (superpowers-evals) runs real tmux sessions to verify skill compliance

**Distinctive idea:** Skills auto-trigger — the agent behaves correctly at every moment without
the human remembering to invoke skills. The bootstrap is the entire activation mechanism.

**Likely failure mode:** The auto-trigger depends on the bootstrap being loaded at session start.
Without it (wrong harness integration, plugin not installed) all skills are silent. The 94%
PR rejection rate suggests the integration barrier is high.

---

### 3. wshobson/agents [source]

**URL:** https://github.com/wshobson/agents  
**Stars:** 39 218. **License:** MIT. **Active:** 2026-08-26.

#### Process shape

No explicit dev-cycle phases. Instead: a **marketplace** of 202 agents grouped by plugin (93
plugins) that can be composed. Quality is enforced by a three-layer eval framework and CI.

```
plugins/<name>/
  agents/<agent>.md
  skills/<skill>/SKILL.md
  commands/<cmd>
```

The `plugin-eval` framework in `docs/plugin-eval.md` defines how agents are validated before
publishing.

#### Orchestration

Adaptor pattern: one Markdown source → per-harness transpilation via `make generate`. Each
harness gets its own artifact tree (`.codex/`, `.opencode/`, `.cursor/`, `.antigravity/`).
Context passes through the AGENTS.md file (kept to ~150 lines per OpenAI harness-engineering
practice). Skills use progressive disclosure: detail in `references/`, not in the skill body.

#### Documentation patterns

- One source-of-truth in `plugins/` — never edit generated files
- AGENTS.md is a map, not an encyclopedia
- Per-skill: `SKILL.md` spine + optional `references/`, `templates/`, `scripts/`
- `docs/authoring.md` defines the portable-content style guide

#### Planning vs building

No built-in planning phase; this repo supplies components that users plug into their own workflow.
Closest thing to planning: `prp-orchestrate` and `prp-plan` skills from the PRP pack (if installed).

#### Tracking / Review / Human role

- CI runs `make validate STRICT=1`, `make garden`, `make test`, `make smoke-test` on every PR
- Real-CLI subprocess tests (`make smoke-test`) run against generated artifacts
- Human role: author; the CI gates quality

**Distinctive idea:** Single Markdown source generates per-harness artifacts. Agents work
across Claude Code, Codex, Cursor, OpenCode, and Antigravity from one definition.

**Likely failure mode:** The adaptor transpilation is complex and tested mechanically but not
behaviorally. A skill that works on Claude Code may behave differently on Codex because the
transpilation rewrites tool-dispatch calls. The round-trip results doc tracks known deltas.

---

### 4. SuperClaude-Org/SuperClaude_Framework [source]

**URL:** https://github.com/SuperClaude-Org/SuperClaude_Framework  
**Stars:** 23 848. **License:** MIT. **Active:** 2026-08-21.

#### Process shape

```
/sc:pm  → PRD + confidence check
/sc:research → evidence gathering
/sc:implement → Wave→Checkpoint→Wave parallel execution
/sc:review → post-implementation validation
```

For features: Requirements → Design → Tasks → Implementation  
For bugs: Report → Analysis → Fix → Verification

30 slash commands, 20 domain-specialist agents, 7 behavioral modes.

#### Orchestration

Three patterns from the Python `pm_agent/` module:
1. **ConfidenceChecker**: before any task, assess confidence (≥90% proceed, 70-89% present
   alternatives, <70% stop and ask). Claimed 25-250x token savings ROI.
2. **SelfCheckProtocol**: post-implementation evidence-based validation
3. **ReflexionPattern**: cross-session error learning

Parallel execution via Wave→Checkpoint→Wave (claimed 3.5x speedup).
Agent activation: keyword-based routing — `@agent-security` for auth work, etc.
Context: slash commands load the right agent and KNOWLEDGE.md accumulates cross-session learnings.
Git worktrees used for parallel Claude Code sessions.

#### Documentation patterns

- CLAUDE.md (12 982 bytes), PLANNING.md, TASK.md, KNOWLEDGE.md — four always-read files
- 20 agent `.md` files installed to `~/.claude/agents/`
- pytest plugin auto-loaded via entry point (provides fixtures and markers)
- `docs/user-guide/`, `docs/developer-guide/`, `docs/reference/`

#### Planning vs building

Pre-execution confidence gate is the planning proxy — if confidence < 70% the agent stops and
asks questions before writing any code. Token budgets by complexity tier (simple 200, medium
1000, complex 2500).

#### Tracking / Review / Human role

- TASK.md tracks current work; KNOWLEDGE.md accumulates findings
- `make test` runs 136 tests; `make lint` / `make format`
- Human role: invokes commands; the framework gates wrong-direction work pre-execution

**Distinctive idea:** Confidence threshold as a process gate — the agent self-assesses before
starting and stops rather than building in the wrong direction.

**Likely failure mode:** The confidence assessment is self-reported by the model; it can be
wrong without knowing it is wrong. High confidence on an incorrect premise still proceeds.

---

### 5. Wirasm/prp [source]

**URL:** https://github.com/Wirasm/prp (was Wirasm/PRPs-agentic-eng)  
**Stars:** 2 239. **License:** MIT. **Active:** daily commits 2026-08-28.

#### Process shape

```
prp-prd (requirements) →
prp-plan (task breakdown) →
prp-implement (single-pass implementation with validation loop) →
prp-review (review) →
prp-deliver (PR + changelog) →
prp-loop (autonomous loop until green)
```

Plus lateral skills: `prp-spike` (research), `prp-debug`, `prp-orchestrate` (multi-agent),
`prp-research-team` (fan-out research), `prp-worklist` (task queue management).

#### Orchestration

PRP = PRD + curated codebase intelligence + agent/runbook. Each PRP is a Markdown file with:
- Goal and justification (from PRD)
- Precise file paths, library versions, code snippets (context)
- Existing codebase patterns to follow
- Executable validation commands the agent runs to verify work

`prp-loop` runs the implement-validate cycle autonomously until gates pass.
`prp-orchestrate` dispatches subagents with PRPs as their briefs.
State: `~/.prp/<key>/` stores per-project artifacts out-of-repo.
Context: PRP files on disk; Codex render generated by `sync_plugin.py`.

One source of truth in `.claude/skills/`; plugin distribution and Codex render generated.

#### Documentation patterns

- Each skill: `SKILL.md` spine + `references/` + `templates/` + optional `scripts/`
- `prp-meta-skill` encodes the craft of writing skills
- `claude_md_files/` — CLAUDE.md templates per stack (Rust, Python, Node, React)
- Conventional commits; contributions are scrutinized (load-bearing in production)

#### Planning vs building

Explicit. `prp-prd` and `prp-plan` are separate from `prp-implement`. The PRP artifact carries
enough context for a single-pass implementation without further human prompting. Validation
loops are executable commands, not self-assessment.

#### Tracking / Review / Human role

- `prp-worklist` manages task queues
- `prp-review` is a named review skill; `prp-pr` creates the PR with changelog
- Human role: defines PRD, reviews PRP before implementation, monitors loop
- No built-in issue tracker; pairs with external tools

**Distinctive idea:** The PRP as a minimum-viable context packet — everything an agent needs
to ship a vertical slice in one pass, with executable validation built in.

**Likely failure mode:** PRP quality depends on the human writing a good PRD and the codebase
context being curated correctly. A stale or incomplete PRP causes a confident but wrong
single-pass implementation with no planning fallback.

---

### 6. FoundationAgents/MetaGPT [source]

**URL:** https://github.com/geekan/MetaGPT (now FoundationAgents)  
**Stars:** 70 081. **License:** MIT. **Active:** 2026-01-21 (slowing).

#### Process shape

```
User requirement (one line) →
ProductManager (PRD + competitive analysis) →
Architect (system design, API, data structures) →
ProjectManager (task breakdown) →
Engineer (code) →
QA (test cases, review)
```

Code = SOP(Team). Roles are Python classes; the SOP is wired into the framework.

#### Orchestration

Python classes with `Role` base, `Action` handlers, message-passing via a `Environment`.
Each role watches for specific message types, executes its action, publishes output.
State: in-memory during a run; `./workspace/` for file artifacts. Async `asyncio`-based.
No Claude Code integration; works with any OpenAI-compatible API.

#### Documentation patterns

- Code-first: roles and actions are Python; documentation is secondary
- `metagpt/roles/` — one file per role
- `metagpt/actions/` — discrete action implementations
- Config via `~/.metagpt/config2.yaml`

#### Planning vs building

Roles are separate; the PRD flows through Architect to ProjectManager before Engineering
starts. But it is fully automated — the human gives one line and the whole pipeline fires
without further checkpoints.

#### Tracking / Review / Human role

- QA role generates tests and reviews
- No human checkpoints in the default pipeline; human is an observer
- Cost: no explicit budget controls; full pipeline runs regardless of change size

**Distinctive idea:** SOP as code — the process is a Python class hierarchy. Roles and
handoffs are type-safe; you can unit-test the pipeline.

**Likely failure mode:** One-line input → automated waterfall with no human in the loop.
The PRD and architecture docs are generated by LLMs; if the one-line requirement is ambiguous,
every downstream artifact inherits the ambiguity. No adversarial gate.

---

## Skim-Read Summaries

**eyaltoledano/claude-task-master** (28k stars): Task management MCP server + CLI. Parses a PRD
into a JSON task tree; each task has subtasks, dependencies, and a `status` field. AI autopilot
mode (`autopilot_start/next/complete_phase/commit/resume`) runs TDD loops. Strong on task tracking
and multi-context (tagged task lists for parallel branches). Weak on roles/gates — no named reviewer
agents. [source]

**OpenBMB/ChatDev** (34k stars): Python framework like MetaGPT — roles (CTO, CPO, programmer,
reviewer, tester) communicate via chat messages. Phase-gated: Design → Coding → Testing →
Documenting. Supports "incremental development" mode that reads existing code. Good reference
for role-message protocol design. [source]

**coleam00/context-engineering-intro** (13k stars): INITIAL.md → `/generate-prp` → PRP file →
`/execute-prp`. Two commands, one artifact. Minimal process scaffold. Best-practice examples
folder. Strongest teaching piece: context engineering >> prompt engineering. [source]

**Pimzino/claude-code-spec-workflow** (3.8k, now shifted to MCP): Five slash commands for spec
workflow (requirements → design → tasks → implement) + five for bug workflow. Four specialized
agents. Straightforward two-pipeline design; now deprecated in favor of MCP version. [source]

**buildermethods/agent-os** (5.3k): Extracts codebase conventions into documented standards, then
injects relevant standards into agent context based on what is being built. Closest thing in
the space to codified institutional knowledge. No phases or roles — purely a context-injection
system. [source]

**snarktank/ai-dev-tasks** (7.8k): Three Markdown files — `create-prd.md`, `generate-tasks.md`,
`process-tasks-list.md` — that the human runs in sequence. Deliberately minimal; no agents, no
framework. The task list enforces one-task-at-a-time execution with human approval at each step.
[source]

**OneRedOak/claude-code-workflows** (3.9k): Code review, security review, design review workflows
as slash commands + GitHub Actions. Pattern: dual-loop architecture (fast automated check +
deeper agent analysis). Good reference for CI-integrated review gates. [source]

**disler/claude-code-hooks-mastery** (3.9k): Focuses on Claude Code hooks (PreToolCall,
PostToolCall, Stop, Notification). Shows how to enforce constraints automatically (e.g.,
run lint before commit, block dangerous commands). Hooks as process enforcement. [source]

**hesreallyhim/awesome-claude-code** (53k): Curated directory. Best source for discovering new
repos. Not itself a process framework. [live]

**Caspian-Sun/claude-code-workflow** (10 stars, 2026-08-28): 8-step SDLC, hard gates,
`@rules` traceability. Requirements → Breakdown → Implementation → Verification → Review →
Delivery → Release. Human supervises every critical checkpoint. AI executes. Very close in
spirit to this project's own cycle. Small but actively developed. [source]

**VoltAgent/awesome-claude-code-subagents** (24.7k): 100+ specialized Claude Code subagents with
enforced workflow patterns. More of a catalog than a process. [live]

---

## Comparison Table

| Repo | Shape | Orchestration | Docs | Tracking | Human role | Distinctive idea |
|---|---|---|---|---|---|---|
| BMAD-METHOD | Clarify→Plan→Build loop; right-sized | Skills on disk; artifact files carry context between agents | Docs site + AGENTS.md + artifact templates | None built-in | Controls loop depth and handoffs | Auto-routing by change size |
| Superpowers | brainstorm→plan→subagent-loop→verify→review | Bootstrap auto-triggers skills; git worktrees for parallel | CLAUDE.md + skills dirs | Plan file + git | Approves spec; monitors loop | Skills fire automatically without human invocation |
| wshobson/agents | Marketplace, no prescribed cycle | Single-source → per-harness transpilation; CI with smoke tests | AGENTS.md ≤150 lines + authoring guide | CI gates | Author; CI enforces | One source, N harnesses |
| SuperClaude | req→design→tasks→impl; bug→report→fix→verify | Slash commands; keyword-routed agents; Wave→Checkpoint→Wave | CLAUDE.md + PLANNING + TASK + KNOWLEDGE | TASK.md | Invokes commands; confidence gate stops wrong turns | Confidence threshold pre-gate |
| Wirasm/prp | prd→plan→implement(loop)→review→deliver | PRP artifact + prp-loop; subagents get PRPs as briefs | Skill SKILL.md + references/ + templates/ | prp-worklist | Defines PRD; reviews PRP; monitors | PRP = minimum-viable context packet |
| MetaGPT | PM→Arch→PM→Eng→QA waterfall | Python classes; message-passing Environment | Code-first; roles are .py | In-memory workspace | One-line input only | SOP as typed code |
| claude-task-master | PRD→tasks JSON→autopilot TDD loop | MCP server + CLI; tagged task lists for parallel branches | JSON task tree + doc site | JSON task file | Reviews tasks before autopilot | AI-driven task decomposition + autopilot loop |
| ChatDev | Design→Coding→Testing→Documenting | Python; role-message chat protocol | Role .py files | workspace/ | One-line input only | Role-message protocol (testable) |
| coleam00 context-eng | INITIAL→PRP→execute | Two slash commands; examples/ folder | INITIAL.md + PRP template | None | Writes INITIAL.md | Context engineering as a discipline |
| snarktank/ai-dev-tasks | PRD→tasks→one-task-at-a-time | Three Markdown prompts; human runs each | Three .md files | Task .md file | Approves each task before execution | Human approval gate at every task |
| OneRedOak/workflows | PR review cycles | Slash commands + GitHub Actions dual-loop | Workflow .md files | GitHub Actions | Triggers reviews | CI-integrated review gates |
| Caspian-Sun CCW | 8-step SDLC with hard gates | Commands + skills + subagents; @rules traceability | @rules files | None noted | Supervises every checkpoint | @rules traceability across SDLC |

---

## Recurring Patterns (what ~80% do)

1. **Artifact handoff.** Context passes between agents via Markdown files on disk (spec, PRP,
   task list). No shared in-memory state; no database. Files are the bus.

2. **Two-phase split.** Planning (human-driven) is separated from building (agent-driven). The
   boundary is always an artifact the human reviews before the agent starts coding.

3. **Named slash commands as workflow entry points.** `/generate-prp`, `/bmad-build`, `/sc:implement`,
   `/prp-plan`. Every framework invokes phases via a named command rather than a prose prompt.

4. **CLAUDE.md / AGENTS.md as the bootstrap.** Almost every repo uses a root-level context file
   to wire in the methodology. Without it, the agents are just agents.

5. **No built-in issue tracker.** Frameworks rely on external tools (GitHub Issues, Linear,
   Beads, `bd`) or a simple Markdown task file. None build a tracker into the core.

6. **Self-validation loops.** Implementation agents loop on a validation command (tests, lint,
   type check) until it passes rather than reporting "done" and stopping.

7. **Role-per-domain, not role-per-phase.** Specialist agents (security engineer, architect,
   QA) are invoked when their domain comes up, not at a fixed pipeline stage. The exception is
   MetaGPT/ChatDev (pure waterfall), which are Python frameworks rather than Claude Code plugins.

---

## Rare / Distinctive Ideas (worth stealing)

| Idea | Source | Why it is rare |
|---|---|---|
| Right-sized routing (skip planning for trivial changes) | BMAD | Most frameworks apply the same process regardless of change size |
| Skills auto-trigger at session start | Superpowers | Others require explicit invocation; Superpowers inverts this |
| Single-source → multi-harness transpilation | wshobson/agents | Almost nobody targets multiple harnesses from one definition |
| Confidence threshold as a pre-gate (stops wrong-direction work) | SuperClaude | Gates are usually post-implementation; this is pre-execution |
| Executable validation inside the PRP artifact | Wirasm/prp | Most plans are prose; PRP includes runnable commands the agent loops on |
| Hooks as process enforcement (PreToolCall, PostToolCall) | disler hooks-mastery | Hooks enforce constraints without relying on the agent's goodwill |
| @rules traceability across SDLC | Caspian-Sun | Linking requirements to implementation to verification artifacts |
| Adversarial eval harness (real tmux sessions, LLM verifier) | Superpowers evals | Others test with unit tests; Superpowers runs real sessions |
| SOP as typed Python code | MetaGPT | Roles and handoffs are statically defined and testable |
| Human approval at every task (not at phase boundaries) | snarktank/ai-dev-tasks | Most frameworks checkpoint at phases; this checkpoints per task |

---

## Ranked Steal List for Soviet-Simulator Context

Priority: impact on Rust/60k-lines/one-human/token-budget/adversarial-gate setup.

### Tier 1 — Adopt now

**A. Right-sized routing (BMAD).** Our current 8-phase cycle applies the same depth to a
one-line bug fix and a new gameplay system. A routing decision at ticket-intake (trivial →
skip planning phases; complex → full cycle) would cut token spend on low-risk work.

**B. Executable validation inside briefs (Wirasm/prp).** Our agent briefs tell workers what
to do but not how to verify it is done. PRP's pattern of embedding the exact cargo test command
(and expected output shape) in the brief would close the loop without requiring the gate agents
to re-derive the acceptance test.

**C. Hooks as process enforcement (disler).** PreToolCall hooks can enforce that no agent
writes to production files without first reading the relevant spec. PostToolCall hooks can
auto-run `cargo test -p simulation` after any sim file edit. This is cheaper and more reliable
than trusting agent goodwill.

### Tier 2 — Adapt for our context

**D. Confidence pre-gate (SuperClaude).** Our gate agents are post-implementation reviewers.
A pre-implementation confidence check by the briefing agent — "do I have enough context to
avoid the known traps?" — would catch wrong-direction work before it burns budget. The
self-reported confidence number is unreliable alone; pair it with a checklist (trap list from
bd description, acceptance criteria, relevant spec file).

**E. Skill auto-trigger / bootstrap (Superpowers).** Our CLAUDE.md is already the bootstrap.
The missing piece is skills that fire automatically at the right moments (e.g., the
`finishing-a-development-branch` skill equivalent — automatically run export + bd export +
gate checklist when a worker closes its task).

**F. @rules traceability (Caspian-Sun).** Our ratified spec files already bind mechanism.
Adding a lightweight traceability link — each ticket cites the spec section it implements —
would let the doc-reality-auditor and review agents verify that the code matches the ratified
spec, not just that it passes tests.

### Tier 3 — Reference only

**G. SOP as typed code (MetaGPT).** Not worth porting to Rust/Claude Code, but the concept
that role handoffs should be as explicit as function signatures informs how we structure agent
briefs (typed artifacts in, typed artifacts out).

**H. Human approval per task (snarktank).** Our human is too available for high-cost work but
too expensive to consult on every subtask. The sweet spot is approval at gate boundaries (which
we already have) rather than per-task. Reference for when we add junior-developer–level agents.

**I. Multi-harness transpilation (wshobson).** Not applicable today; we are Claude Code only.
File the idea for if/when we add Codex or another harness.

---

## Gaps and What Would Close Them

- BMAD's workflow-map.md was not read directly (docs site, not a raw GitHub file). The routing
  logic may be more nuanced than the README implies. Fetch `docs/reference/workflow-map.md`
  directly if adopting the right-sized routing idea.

- SuperClaude's confidence.py implementation was not read. The numeric thresholds (90%/70%) are
  stated in PLANNING.md but the scoring model is unknown. Before adopting, read
  `src/superclaude/pm_agent/confidence.py` to understand whether the score is a calibrated
  probability or a heuristic label.

- Superpowers' `executing-plans` and `subagent-driven-development` skill bodies were not read
  (skills directory confirmed, bodies not fetched). The mechanism by which subagents receive
  their briefs and report back is inferred from the README. Read the SKILL.md files before
  designing the auto-trigger bootstrap.

- ChatDev 2.0's incremental mode (reads existing code before generating) was not deeply read.
  This is directly relevant to our 60k-line codebase. Fetch `metagpt/actions/` for the
  incremental flow before designing Phase 0 context-gathering.

---

*Generated by researcher agent. Verified against gh API and direct repo reads 2026-08-28.*
*Do not modify files outside .planning/process-overhaul-2026-08-28/ and the scratchpad.*
