# Soviet city-builder — a hard fork of Egregoria

**Kind:** process entrypoint
**Authority:** operational
**Status:** active
**Owner:** project lead
**Last verified:** 2026-08-24

**Speak to the user in ASD-STE100** (Simplified Technical English): short sentences, active voice, one instruction per sentence, simple words, one name per thing. Full rule: `~/.claude/rules/ste100.md`. Code, commits, and repo docs keep their own conventions.

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

- **How work gets done: `docs/process/development-cycle.md`.** Eight phases, the 22-agent roster and what each is
  for. Every phase names the failure it exists to prevent. Read it before dispatching ANY implementation
  or gate agent — ticket work included, not just waves. The roster there decides which agent gets which
  lane (`sim-implementer`/`ui-implementer`/`data-implementer`, plus the domain gates); global generics are
  fallbacks only.
- Plan of record: `docs/plan/charter-1.0.md`. It **binds on scope**. Ratified files under
  `docs/reference/specifications/` bind mechanism inside that scope; archived legacy documents are
  provenance only. The charter's Post-1.0 and Never lists are absolute.
- Art direction: `docs/reference/art-direction.md`. Palette, current renderer evidence, and asset provenance.
- Live plan: `docs/plan/iterations/` — `RESUME.md` (read this first), requirements and evidence;
  generated status is `docs/generated/roadmap.md`.
- Current substrate map: `docs/reference/architecture/substrate.md`; follow its fact-sheet citations.
- Code intelligence: `docs/reference/code-intelligence.md`. Which of LSP and the knowledge
  graph answers which question, the two traps that produce confident wrong answers (a cold
  language server reporting "No references found"; `head_matches_build` comparing SHAs rather
  than file content), and what must be installed for the graph hooks to do anything at all.
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

### Adopted conventions

Recorded in **`docs/reference/bd-capability-survey.md` §5** — attribution via `--author`, `bd batch`
for wave setup, the `bd stale`/`bd orphans` close sweep, `bd defer` for postponed work,
`validation.on-create = warn`, and the Phase-4 gate-chain formula. Read that section before
running a wave. Two things stay here because a session must not be able to miss them:

- **Never run `bd upgrade` casually** — bd is pinned at 1.2.2, and a machine that ever ran 1.2.1
  has a v65 schema 1.2.2 cannot read.
- **The `BEADS_ACTOR`/`Executed-By:` trailer convention is DELETED** (2026-08-27, the hook is
  inert). Do not set it; do not cite those trailers as provenance.

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


<!-- The managed Beads block that stood here was removed 2026-08-28: the `bd prime`
     SessionStart hook injects the same command reference and session-close protocol
     every session, and the `## Task tracking` section above states this repo's real
     policy, which overrides two of that block's rules. Run `bd prime` for commands. -->

**Git authority:** do not commit, push, or run `bd dolt push` without explicit authority
from the user. If a required sync or push is blocked, stop and report the exact command
and the error — never work around it.

<!-- code-review-graph MCP tools -->
## MCP Tools: code-review-graph

**This project has a knowledge graph. Start with the code-review-graph
MCP tools to narrow scope, then read the source.**

> **Precedence, repo rule:** narrow scope with the graph, but **LSP stays first for
> symbol-level intelligence** — who calls what, types, rename safety, compiler warnings. The
> graph's call edges are AST heuristics carrying a confidence tier; LSP is compiler truth.
> Warm the language server with one `documentSymbol` call before your first load-bearing
> query: a cold server answers `findReferences` with "No references found", which reads
> exactly like a true negative. Full rules and the measured evidence:
> `docs/reference/code-intelligence.md`. The graph is cheaper than scanning files and
gives you structural context (callers, dependents, test coverage) that file search cannot.

### When to use graph tools FIRST

- **Exploring code**: `semantic_search_nodes_tool` or `query_graph_tool` instead of Grep
- **Understanding impact**: `get_impact_radius_tool` instead of manually tracing imports
- **Code review**: `detect_changes_tool` + `get_review_context_tool` instead of reading entire files
- **Finding relationships**: `query_graph_tool` with callers_of/callees_of/imports_of/tests_for
- **Architecture questions**: `get_architecture_overview_tool` + `list_communities_tool`

### Verify in the source

- Narrow scope with the graph, then read the source. Do not change code from graph output alone.
- For any non-trivial change, read the implementation and the relevant tests before concluding.
- Verify the exact source when touching behavior, database logic, migrations, retries, fallbacks,
  recovery, or compatibility code.
- When the graph and the source disagree, the source wins. The graph may be stale or may not
  model that relationship.
- An empty graph result can mean "not indexed" or "not statically visible", not "does not exist".

### Key Tools

| Tool | Use when |
| ------ | ---------- |
| `detect_changes_tool` | Reviewing code changes — gives risk-scored analysis |
| `get_review_context_tool` | Need source snippets for review — token-efficient |
| `get_impact_radius_tool` | Understanding blast radius of a change |
| `get_affected_flows_tool` | Finding which execution paths are impacted |
| `query_graph_tool` | Tracing callers, callees, imports, tests, dependencies |
| `semantic_search_nodes_tool` | Finding functions/classes by name or keyword |
| `get_architecture_overview_tool` | Understanding high-level codebase structure |
| `refactor_tool` | Planning renames, finding dead code |

### Workflow

1. The graph auto-updates on file changes (via hooks).
2. Use `detect_changes_tool` for code review.
3. Use `get_affected_flows_tool` to understand impact.
4. Use `query_graph_tool` pattern="tests_for" to check coverage.
<!-- /code-review-graph MCP tools -->
