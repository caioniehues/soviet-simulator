# Harness Repo Deep Read
**Brief:** process-overhaul-2026-08-28 / 02
**Date:** 2026-08-28
**Repo:** revfactory/harness @ cceac68ea1d0ad198ef4b7b906cd238375836387 (2026-06-10)
**Stars:** 8,849 | **Commits (depth-50 clone):** 45 | **License:** Apache-2.0

---

## 1. What Is It?

Harness is a **meta-skill for Claude Code** that, given a one-sentence domain description,
generates a structured agent team: `.claude/agents/*.md` definitions and `.claude/skills/*/SKILL.md`
skill files. The user says "build a harness for this project" and Harness runs a six-phase workflow
(domain analysis → architecture design → agent definitions → skills → orchestrator → validation).
It is a *factory for process scaffolding*, not a process itself. It is Claude-Code-native and
requires the experimental agent-teams flag (`CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1`). Maturity:
v1.2.0 (current main), with a v2.1.0 rewrite open as PR #51. Actively developed; serious end-to-end
runs have surfaced real runtime bugs (issue #53). The A/B results (+60% quality, n=15,
author-measured, no independent replication) are self-reported.

---

## 2. The Process It Encodes

Harness encodes a **meta-process for designing processes**, not a domain workflow. Its own six-phase
build cycle:

```
Phase 0: Audit — read .claude/agents/ + .claude/skills/ + CLAUDE.md; detect drift vs spec
    |
    v  (branch: new build / expand existing / maintenance)
Phase 1: Domain Analysis — identify tasks, tech stack, user skill level
    |
    v
Phase 2: Architecture Design — choose mode (Agent Team | Subagent | Hybrid); pick pattern
           Patterns: Pipeline | Fan-out/Fan-in | Expert Pool | Producer-Reviewer
                     Supervisor | Hierarchical Delegation
    |
    v
Phase 3: Agent Definition Generation — write .claude/agents/{name}.md for every agent
           Mandatory sections: role, principles, I/O protocol, team comms protocol,
           error handling, collaboration; all agents model: "opus"
    |
    v
Phase 4: Skill Generation — write .claude/skills/{name}/SKILL.md for each agent;
           progressive disclosure: SKILL.md < 500 lines, overflow goes to references/;
           description is the sole trigger; write it "pushy"
    |
    v
Phase 5: Integration / Orchestration — write orchestrator skill that coordinates the team;
           wire CLAUDE.md pointer (trigger rule + changelog only, no agent lists)
    |
    v
Phase 6: Validation — structure check, trigger dry-run, with-skill vs without-skill
           comparison run, should-trigger / should-NOT-trigger eval (8-10 each)
    |
    v
Phase 7: Evolution — collect feedback after every run; update agent/skill/orchestrator;
           log all changes in CLAUDE.md changelog
```

Work tracking uses the file system: intermediate artifacts go to `_workspace/{phase}_{agent}_{artifact}.ext`,
final outputs to a user-specified path. No external tracker. Context passed via file paths in agent
prompts; agents reference `_workspace/` directly. Agent-team agents also pass context via SendMessage.

---

## 3. The Mechanisms

### Agent definitions
`[source]` `.claude/agents/{name}.md` with YAML frontmatter (`name`, `description`, optionally
`tools`). Each file carries: role, principles, I/O protocol, team-comms protocol, error handling,
collaboration. All agent calls specify `model: "opus"`. Defined in `SKILL.md:86-99` and the
`agent-design-patterns.md` template (`skills/harness/references/agent-design-patterns.md:216-251`).

### Skill files
`[source]` `.claude/skills/{name}/SKILL.md` with YAML frontmatter (`name`, `description`).
Three-layer progressive disclosure: metadata (always loaded) → SKILL.md body (on trigger, target
< 500 lines) → `references/` (loaded on demand by the agent). Described in
`skills/harness/references/skill-writing-guide.md`.

### Orchestrator skill
`[source]` A special skill that drives the whole team. Three templates (Agent Team / Subagent /
Hybrid). Defined in `skills/harness/references/orchestrator-template.md`. Orchestrator does:
`TeamCreate` → `TaskCreate` → monitor → `Read` artifacts → synthesize → `TeamDelete`.
File convention: `_workspace/{phase}_{teammate}_{artifact}.{ext}`.

### Data passing strategies
`[source]` `SKILL.md:224-235`: message-based (SendMessage for real-time), task-based (TaskCreate/
TaskUpdate for status), file-based (agreed paths in `_workspace/`), return-value-based (for
subagent mode). Recommended combo for team mode: task + file + message.

### CLAUDE.md pointer
`[source]` `SKILL.md:258-277`: only a trigger rule and a changelog table go into CLAUDE.md. No
agent lists, no directory maps. Rationale: those are in the files themselves.

### Hooks
`[source]` None. No Claude Code hooks (PreToolUse / PostToolUse / Stop / Notification) are used
anywhere in this repo. The plugin structure is entirely passive markdown consumed by Claude Code.

### Validation mechanisms
`[source]` `skill-testing-guide.md` describes a manual framework: write 2-3 realistic prompts,
run with-skill vs without-skill subagents in parallel, collect `grading.json` and `timing.json`.
No automated runner; no CI for skill quality. PR #41/33 adds a `validate_skills` linter (open,
not merged).

### Plugin manifest
`[source]` `.claude-plugin/plugin.json` and `marketplace.json`. Installed via
`/plugin marketplace add revfactory/harness` then `/plugin install harness@harness-marketplace`.

---

## 4. Documentation Patterns

| Doc type | Location | Purpose |
|---|---|---|
| Meta-skill | `skills/harness/SKILL.md` | Master workflow (7 phases, checklists) |
| Pattern library | `references/agent-design-patterns.md` | 6 patterns + decision tree + templates |
| Orchestrator templates | `references/orchestrator-template.md` | Three fill-in-the-blank templates |
| Real examples | `references/team-examples.md` | 5 worked teams with full agent files |
| Skill writing | `references/skill-writing-guide.md` | Style guide, schemas, anti-patterns |
| Skill testing | `references/skill-testing-guide.md` | Eval framework, grading schemas |
| QA guide | `references/qa-agent-guide.md` | 7 real bugs, boundary-mismatch patterns |
| Workspace logs | `_workspace/*.md` | Intermediate artifacts from runs |

Frontmatter: `name` + `description` required on every skill. Agent files use the same two fields.
No ADR format, no spec/PRD distinction. Documents are **prescriptive guides for Claude**, not
human-readable specs. The "why" is embedded in body prose, not in separate rationale documents.

Naming convention: `{phase}_{agent}_{artifact}.{ext}` for workspace files. Agent files:
`{name}.md` flat under `.claude/agents/`. Skills: `{name}/SKILL.md` under `.claude/skills/`.

---

## 5. Collaboration Patterns

`[source]` `agent-design-patterns.md:1-78` and `SKILL.md:186-220`.

- **Agent team mode (default):** peer-to-peer SendMessage; shared TaskCreate/TaskUpdate list;
  leader monitors via TaskGet; teammates self-assign tasks by "claiming" them.
- **Subagent mode:** fan-out from main, no peer comms, results returned to caller.
- **Hybrid:** subagent for collection phases, team for synthesis/debate phases; requires
  TeamDelete before switching.

Context handoff: file paths written to `_workspace/`. The orchestrator passes these paths in the
prompt when spawning the next phase's team. Cross-agent review is explicit: reviewer agents read
producer outputs and SendMessage findings back. The code-review example shows reviewers talking
directly to each other without going through the leader (`team-examples.md:265-273`).

Session constraint: one active team per session. Phase-to-phase team recomposition is possible
by TeamDelete + TeamCreate (`agent-design-patterns.md:37`).

---

## 6. Critical Assessment

### Genuinely good

**Progressive disclosure on skill files** (`skill-writing-guide.md`). The three-layer system
(metadata → SKILL.md body → references/) is a real token budget discipline. The 500-line cap and
the "load references/ only when needed" rule are practical. Worth stealing directly.

**"Pushy" description principle** (`skill-writing-guide.md:20-54`). The instruction to write
descriptions as assertive trigger declarations — including follow-up keywords to keep the skill
alive after first run — addresses a real failure mode: skills that fire on first use and then
disappear. Evidence: the orchestrator template explicitly warns `SKILL.md:282-286` that "without
follow-up keywords the harness is dead code after first run." This is observed behavior, not
theory.

**Boundary-mismatch QA patterns** (`qa-agent-guide.md`). The seven real bugs from the SatangSlide
project are concrete and generalizable. The "read both sides simultaneously" principle addresses
a genuine gap in single-agent review. For a Rust/Bevy-replacement codebase this translates to:
verify both the system that produces a component and the system that consumes it.

**Agent reuse audit (Phase 0 and 3-0/4-0)** (`SKILL.md:19-35, 80-84, 110-115`). The explicit
drift detection — compare `.claude/agents/` against the orchestrator's agent list — is a process
hygiene mechanism that prevents agent proliferation. Our 22-agent roster would benefit from this.

**Orchestrator changelog in CLAUDE.md** (`SKILL.md:386-398`). Dated change table: date, what
changed, which file, why. Not fancy, but it gives a future agent a reason for the current structure
rather than just the structure.

### Hand-waving

**The +60% A/B claim** is self-reported, n=15, no blinding, no independent replication. The FAQ
acknowledges this (`README.md:272`). Treat it as motivation, not evidence.

**Model: "opus" everywhere** (`SKILL.md:93`, `agent-design-patterns.md:213`). The rationale is
quality maximization with no cost consideration. Issue #29 is open requesting per-task model
tiering. For a solo developer with a token budget, forcing opus on all agents is incorrect
economics. The rule exists because Harness targets demo quality, not production cost.

**TeamCreate/TeamDelete behavior** is described as designed, but issue #53 documents that the
runtime behavior diverges in at least three ways: (a) declared `tools:` are not reliably injected
(TaskCreate/TaskUpdate absent even when listed); (b) phase "completion" is not enforced — agents
rewrite artifacts after the next phase has already read them; (c) the retry-once rule does not
distinguish retryable from non-retryable failures (quota exhaustion is not retryable).

**Trigger verification** (Phase 6-4) is manual: the author writes 8-10 should-trigger and 8-10
should-NOT-trigger queries and checks them. No runner. PR #41 proposes a `validate_skills` script
but it is unmerged. For a stable team this is acceptable; for a growing roster it does not scale.

### What breaks for a one-human Rust team with a token budget

1. **Opus on all agents.** A wave of 5 opus agents on a 2-hour research run costs more than a
   week of focused work. The correct call for our domain agents is: opus only for gate roles
   (ledger checker, physics auditor), sonnet for implementers and explorers.

2. **Agent Teams require `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1`.** We do not have this
   verified as on. Subagent mode (plain `Agent` tool) is fully available without the flag and
   is what our current roster uses. Adopting team mode requires validating the flag works in our
   environment first.

3. **No tracker integration.** Harness uses `_workspace/` files and Claude task tools. Our
   `bd` issue tracker is not in scope for Harness. The CLAUDE.md pointer and changelog table
   are the only durable state. For a long-lived project (months, not hours), this is insufficient.

4. **The validation framework is labor-intensive.** With-skill vs without-skill comparison runs
   double the token cost of every skill audit. For creative content that is fine. For gates on a
   60k-line Rust codebase the correct check is "does cargo test pass" plus a diff review, not a
   parallel baseline run.

5. **No hooks.** Harness encodes nothing about Claude Code hooks. Our harness already uses hooks
   (or plans them). The stop hook for bd export, the pre-tool permission list — none of this is
   modeled. Harness is complementary, not a replacement for that layer.

### What Claude Code already does natively (no need to steal)

- Model selection per Agent call — already in the Agent tool `model` parameter.
- Subagent fan-out — already how we dispatch workers.
- CLAUDE.md as session context loader — already how our process works.
- Progressive disclosure of reference files — already the pattern in our skills.

---

## 7. Steal List (Ranked)

### Steal immediately (adopt verbatim or near-verbatim)

**1. Progressive disclosure file discipline for skills**
Source: `skills/harness/references/skill-writing-guide.md:139-184`
Adopt the 500-line SKILL.md cap, `references/` overflow convention, and the "load only when
needed" pointer pattern. Apply to all our existing agent `.md` files and skills.

**2. "Pushy" description + follow-up keyword rule**
Source: `skills/harness/references/skill-writing-guide.md:20-54` and
`skills/harness/references/orchestrator-template.md:277-286`
Write agent `description:` fields with: (a) what the agent does, (b) concrete trigger situations,
(c) near-miss exclusions, (d) follow-up keywords so re-invocation works. Apply to our 22 agents.

**3. Orchestrator changelog in CLAUDE.md**
Source: `SKILL.md:387-398`
Add a dated changelog table to soviet-simulator's CLAUDE.md. Format: `date | what | which file | why`.
This is a two-minute addition that future agents will actually read.

**4. Drift detection (Phase 0 audit pattern)**
Source: `SKILL.md:19-35`
Before any process overhaul wave: read `.claude/agents/`, `.claude/skills/`, CLAUDE.md; diff
actual files vs what the orchestrator expects; surface mismatches. We have 22 named agents; drift
is already a real risk.

**5. Boundary-mismatch QA principle (read both sides simultaneously)**
Source: `skills/harness/references/qa-agent-guide.md:40-99`
Adapt to Rust: when a system produces data that another system consumes, the reviewer must read
both the production site and the consumption site. Apply in our Phase 4 review gate brief.

### Steal with adaptation

**6. Agent definition template structure**
Source: `agent-design-patterns.md:216-251`
The mandatory sections (role, principles, I/O protocol, team comms, error handling, collaboration)
are good hygiene. Our current agent files vary in depth. Audit and standardize to this template,
but drop "팀 통신 프로토콜" (team comms) for agents that are plain subagents — we do not use
TeamCreate.

**7. Phase selection matrix for expansion**
Source: `SKILL.md:27-35`
When we add a new agent or skill, use the matrix to decide which phases to re-run. Avoids
re-running the full 8-phase cycle for a one-agent addition.

**8. Workspace naming convention**
Source: `SKILL.md:239-240`
Adopt `{phase}_{agent}_{artifact}.{ext}` for files agents produce in planning runs. Currently we
use ad-hoc names in `.planning/`. Consistent names let the next agent find artifacts without being
told their paths.

### Do not steal

**Agent Teams (TeamCreate/SendMessage/TaskCreate)**
Issue #53 documents three runtime bugs that are not fixed in main. Our subagent model (plain
`Agent` tool) is stable. Evaluate again after v2.1.0 ships and those issues close.

**Model: "opus" on all agents**
Wrong economics for a token-budgeted developer. Keep our current tiering: opus for gates
(ledger checker, wiring auditor), sonnet for implementers.

**With-skill vs without-skill baseline comparison runs**
The validation methodology is good for a skill marketplace. For our internal agents, `cargo test`
plus a reviewer pass is the right gate.

**Plugin installation / marketplace**
We do not need Harness itself as a plugin. We are studying it as a reference, not installing it.

---

## Gaps and Uncertainties

- **PR #51 ("v2 全面再構築") is open**: the v2.1.0 rewrite includes a Team/Subagent detection
  fix and v1-artifact migration. Main branch is v1.2.0. Some of what is written above may be
  superseded by v2.1.0 when merged. Key change: TeamCreate/TeamDelete pattern is revised.
- **Star count (8,849) vs repo depth (45 commits)**: the ratio suggests most stars came from
  Hacker News / Claude-related newsletters, not sustained usage. The workspace audit file
  (`_workspace/01_auditor_repo_audit.md`) is a readiness audit Harness ran on itself, not
  evidence of production use.
- **Issue #53 runtime bugs are unverified on our environment** (our platform is Linux/zsh;
  the reporter used Windows). The deferred-tool injection failure for TaskCreate/TaskUpdate
  may not reproduce on our stack.
- **The A/B data** (`revfactory/claude-code-harness`) was not read directly. Citation is from
  the README disclosure.

---

*[source] claims verified against the cloned repo at cceac68ea1d0ad198ef4b7b906cd238375836387.*
*[live] checks not run (no Claude Code session with the plugin installed).*
