# Soviet city-builder — a hard fork of Egregoria

**Kind:** process entrypoint
**Authority:** operational
**Status:** active
**Owner:** project lead
**Last verified:** 2026-08-24

Rust, ECS. The fork happened 2026-08-22 and **the earlier Bevy track was discarded**. Bevy is not a
dependency; its documents live only under `docs/archive/bevy-track/`. Trust current code for
substrate behavior. The repo is GPL-3.0 by inheritance, permanently.

The core loop is the **dishonest enterprise**: an enterprise requests more input than its recipe
consumes, hoards the surplus, and the player — acting as THE PLANNER — catches it from observable
state. Two design pillars constrain every change:

- **Nothing teleports.** Goods move physically or they do not move. Stock must never change hands
  at trade-match time.
- **Never game over.** Failure degrades into queues, shortages and colder homes. It never terminates.

Clearing is by queue, substitution and going without — **never by price**. Money is not a gate.

- **How work gets done: `docs/process/development-cycle.md`.** Eight phases, the 15-agent roster and what each is
  for. Every phase names the failure it exists to prevent. Read it before dispatching ANY implementation
  or gate agent — ticket work included, not just waves. The roster there decides which agent gets which
  lane (`sim-implementer`/`ui-implementer`/`data-implementer`, plus the domain gates); global generics are
  fallbacks only.
- Plan of record: `docs/plan/charter-1.0.md`. It **binds on scope**. Ratified files under
  `docs/reference/specifications/` bind mechanism inside that scope; archived legacy documents are
  provenance only. The charter's Post-1.0 and Never lists are absolute.
- Art direction: `docs/reference/art-direction.md`. Palette, current renderer evidence, and asset provenance.
- Live plan: `docs/plan/iterations/` — `RESUME.md` (read this first), requirements and evidence;
  generated status is `docs/generated/iterations/roadmap.md`.
- Current substrate map: `docs/reference/architecture/substrate.md`; follow its fact-sheet citations.
- Keep durable project status in `README.md`: what is built, what is left, and an asset table.
- Generate visual assets with `/asset-gen`. Confirm the spend with the user before the first paid generation.

Run the sim's tests as `cargo test -p simulation` — parallel runs are trustworthy since the
`static mut` race in `init.rs`/`prototypes` was removed (`sov-test-race-initfuncs-qt6`, fixed
2026-08-26). The same defect shape still exists in `native_app/src/init.rs:85-86` (UI crate,
not linked into the test binary).

## Task tracking — `bd` (beads) is the shared surface

`bd` (upstream beads 1.2.x) replaced the retired `br` fork on 2026-08-26. Same `.beads/`
workspace, re-inited with prefix `sov`; all historical slug ids (`sov-dispatch-wedge-ab4`)
survived the import and stay valid. New ids are generated (`sov-9ze`-style) — there is **no
`--slug` flag anymore**; put the meaning in the title and always cite the id in the commit.

**Every agent can reach `bd`** via Bash. The built-in Claude task tools (`TaskCreate`/`TaskUpdate`/
`TaskList`) are **available ONLY to the main session** (verified 2026-08-23) — never coordinate
workers through them; they are the lead's and the user's live dashboard. This section OVERRIDES
two rules in the managed Beads block below, per that block's own precedence clause: mirroring the
macro layer into the built-in task list is sanctioned, and MEMORY.md/agent-memory files remain the
memory system (`bd remember` is not used here).

| Layer | Where | Who writes |
|---|---|---|
| **Macro** — the goal, the why, the traps | a `bd` issue | lead creates, anyone updates status |
| **Micro** — progress, findings, blockers | `bd comments add` | **the worker doing the work** |
| Live session view | Claude tasks | main session only, mirrors the macro layer |

### Adopted conventions (2026-08-26, from `docs/reference/bd-capability-survey.md`)

- **`BEADS_ACTOR=<agent-name>`** in every worker's environment (or `--actor` per command): it
  stamps comments, events, and — via the installed `prepare-commit-msg` hook — an
  `Executed-By:` trailer on commits. Leads set it in briefs; workers use their roster name.
- **Wave setup goes through `bd batch`**: N creates + deps as one transaction (stdin grammar:
  `create <type> <priority> <title>`, `dep add <from> <to>`, `close <id> [reason]`).
- **Session close adds a drift sweep**: `bd stale --days 14` and `bd orphans` (issues cited in
  commit messages but never closed — the failure our commit-sha convention creates).
- **Postponed ≠ blocked**: use `bd defer <id> --until <date> --reason "…"` instead of an
  open issue worded "not now". `bd undefer` reverses.
- **`validation.on-create = warn` is active** (config.yaml): creating without `--acceptance`
  warns, never blocks. Keep acceptance criteria first-class.
- **Gate-chain formula**: `.beads/formulas/gate-chain.formula.toml` encodes the Phase-4 chain
  (wiring → domain → reviewer). Pour per story: `bd mol pour gate-chain --var story=<id>
  --var scope=<range>`. Molecules structure work only — no execution hooks; epics do not
  auto-close, sweep with `bd epic close-eligible`.
- **Version is pinned at 1.2.2** — a recovery re-release of 1.1.2. Never run `bd upgrade`
  casually (1.2.1 schema-skew trap); `bd doctor` does not work in embedded mode; upstream doc
  pages on work leases / events journal / sync federation / HTTP API describe an unreleased
  version. Telemetry is disabled (`metrics.disabled=true`, user-level config).

### If you are a worker

Your brief names your `bd` issue id. Then:

```bash
bd show <id>                     # the goal, and the traps — read the DESCRIPTION, not just the title
bd update <id> --claim           # atomic claim: assignee=you, status=in_progress
bd comments add <id> "<what you found / where you are>" --author <your-name>
bd close <id> --reason "commit <sha>: <the check that proves it>"
```

Log a comment when you learn something the next agent would otherwise rediscover — a wrong
premise in your brief, a blocked path, a file that is not what it claims. This is how three
agents avoided repeating each other's dead ends on STORY-0149.

Close with **evidence, not a claim**: the commit sha and the command output that proves it. A
closed issue must be auditable months later.

### Conventions

- **P1 is for gates** — checks that stop the line. Ordinary work is P2, cleanup P3. Do not inflate.
- Put the *traps* in the description (`-d`); acceptance criteria go in `--acceptance`. A future
  agent reads the description and nothing else; a title cannot warn it which mistake to avoid.
- Dependencies: `bd dep <blocker-id> --blocks <blocked-id>` — note the direction is REVERSED from
  the old `br dep add <issue> <depends-on>`.
- Storage: the live DB is Dolt under `.beads/embeddeddolt/` (local, gitignored; syncs via
  `refs/dolt/data` on push). `.beads/issues.jsonl` is a passive export that we STILL version as
  the durable, greppable, fresh-clone-recoverable record (`bd bootstrap` rebuilds from it — proven
  2026-08-26). After mutating tracker state, run `bd export -o .beads/issues.jsonl` before
  committing it; the installed git hooks may cover this — trust them only once observed.
- Version exactly these, never `git add .beads/` and never `git add -A`:
  `.beads/.gitignore .beads/config.yaml .beads/issues.jsonl .beads/metadata.json .beads/README.md`

## Delivery

Judge progress from the running game, never from a clean build: verify the structural things yourself (it loads, no errors, assets present) and let what you see drive the next iteration.

Decide from how the task is framed how to work. A task that invites collaboration — open-ended, exploratory, phrased as a direction rather than a spec — gets the live game early: checkpoint at decisions of taste, scope, or cost, and build freely in between. A task handed over as a finished brief to execute gets reasonable calls and steady progress, no blocking. Either way the result is proven, not claimed — if the user hasn't seen it running, finish with a 15–20s video of the game in action, and watch it back before you call the work done.


<!-- BEGIN BEADS INTEGRATION v:1 profile:minimal hash:6cd5cc61 -->
## Beads Issue Tracker

This project uses **bd (beads)** for issue tracking. Run `bd prime` to see full workflow context and commands.

### Quick Reference

```bash
bd ready              # Find available work
bd show <id>          # View issue details
bd update <id> --claim  # Claim work
bd close <id>         # Complete work
```

### Rules

- Use `bd` for ALL task tracking — do NOT use TodoWrite, TaskCreate, or markdown TODO lists
- Run `bd prime` for detailed command reference and session close protocol
- Use `bd remember` for persistent knowledge — do NOT use MEMORY.md files

**Architecture in one line:** issues live in a local Dolt DB; sync uses `refs/dolt/data` on your git remote; `.beads/issues.jsonl` is a passive export. See https://github.com/gastownhall/beads/blob/main/docs/SYNC_CONCEPTS.md for details and anti-patterns.

## Agent Context Profiles

The managed Beads block is task-tracking guidance, not permission to override repository, user, or orchestrator instructions.

- **Conservative (default)**: Use `bd` for task tracking. Do not run git commits, git pushes, or Dolt remote sync unless explicitly asked. At handoff, report changed files, validation, and suggested next commands.
- **Minimal**: Keep tool instruction files as pointers to `bd prime`; use the same conservative git policy unless active instructions say otherwise.
- **Team-maintainer**: Only when the repository explicitly opts in, agents may close beads, run quality gates, commit, and push as part of session close. A current "do not commit" or "do not push" instruction still wins.

## Session Completion

This protocol applies when ending a Beads implementation workflow. It is subordinate to explicit user, repository, and orchestrator instructions.

1. **File issues for remaining work** - Create beads for anything that needs follow-up
2. **Run quality gates** (if code changed) - Tests, linters, builds
3. **Update issue status** - Close finished work, update in-progress items
4. **Handle git/sync by active profile**:
   ```bash
   # Conservative/minimal/default: report status and proposed commands; wait for approval.
   git status

   # Team-maintainer opt-in only, unless current instructions forbid it:
   git pull --rebase
   git push
   git status
   ```
5. **Hand off** - Summarize changes, validation, issue status, and any blocked sync/commit/push step

**Critical rules:**
- Explicit user or orchestrator instructions override this Beads block.
- Do not commit or push without clear authority from the active profile or the current user request.
- If a required sync or push is blocked, stop and report the exact command and error.
<!-- END BEADS INTEGRATION -->
