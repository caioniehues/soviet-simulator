# Soviet Simulator agent guide

**Kind:** process entrypoint
**Authority:** operational
**Status:** active
**Owner:** project lead
**Last verified:** 2026-08-24

## Start here

1. Read `CLAUDE.md` before any work. It contains the fork reality, domain pillars, task ledger, verification command, and delivery bar.
2. Read `docs/reference/glossary.md` before naming domain concepts or changing the simulation model.
3. Read `docs/process/development-cycle.md` before planning or running a multi-agent wave.
4. Read `docs/plan/iterations/RESUME.md` before resuming an iteration.
5. Treat `docs/plan/charter-1.0.md` as scope authority, `docs/reference/specifications/` as mechanism authority after each specification is ratified, `br` as task-state authority, and current code as substrate authority. Archived legacy specifications are rewrite provenance, not current authority.

`docs/archive/bevy-track/ROADMAP.md` preserves the discarded Bevy-era history. It is not the plan of record.

## Non-negotiable model

- This is a Rust/Egregoria hard fork. Bevy guidance and Bevy memories are stale for this tree.
- Goods move physically; matching, payment, or allocation never teleports stock.
- Failure degrades into queues, shortages, substitution, and going without. It never ends the game.
- Domestic clearing is never price-based. Roubles exist only at the border.
- The player is the Planner; presentation reads authoritative simulation state.

## Orchestration

- Delegate Phase 0 mapping to `substrate-cartographer` plus the relevant domain advisor before a brief asserts substrate behavior.
- Keep Phase 1 planning and Phase 5 finding disposition in the lead thread.
- In Phase 2, use `sim-implementer`, `ui-implementer`, and `data-implementer` only on disjoint ownership; serialize shared files and write contracts before parallel consumers.
- Run Phase 3 `evidence-auditor`, then Phase 4 in order: `wiring-auditor`, conditional `ledger-invariant-checker`, `reviewer`, relevant domain sign-off.
- Finish substantive waves with `doc-reality-auditor`; use release and performance roles only at their documented gates.

Use two or three subagents for normal waves and up to five for genuinely independent read-only work. Run at most two writing agents concurrently, with disjoint ownership. Every subagent receives a bounded brief, owned files, acceptance criteria, a `bd` issue when applicable, and the exact verification command.

## Verification and delivery

- Run simulation tests as `cargo test -p simulation`; parallel runs are trustworthy since the `static mut` race fix (`sov-test-race-initfuncs-qt6`, 2026-08-26).
- Name what each check proves and confirm test filters execute at least one test.
- Preserve unrelated changes and never stage with `git add -A` or `git add .`.
- Stage only the four documented `.beads` files when task-ledger state changes.
- Player-facing work finishes with an inspected screenshot or 15–20 second video when `CLAUDE.md` requires visual proof.

For generated visual assets, use Codex's `imagegen` skill and confirm paid generation with the user before the first spend.

## Task tracking — `bd` (beads)

The old `br`/`bv` fork tooling is retired (2026-08-26); upstream `bd` replaced it, same
`.beads/` workspace, prefix `sov`, all historical slug ids preserved. The canonical policy —
worker commands, conventions, what to version, and two repo-level overrides of the managed
Beads blocks below (built-in task list stays the lead's dashboard; MEMORY.md files stay the
memory system, `bd remember` is not used) — lives in **CLAUDE.md § Task tracking**. Follow it.

`bd` never commits or pushes. This repository's git instructions override any generic
workflow advice in the managed blocks.

<!-- BEGIN BEADS INTEGRATION v:1 profile:minimal hash:970c3bf2 -->
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
   bd dolt push
   git push
   git status
   ```
5. **Hand off** - Summarize changes, validation, issue status, and any blocked sync/commit/push step

**Critical rules:**
- Explicit user or orchestrator instructions override this Beads block.
- Do not commit or push without clear authority from the active profile or the current user request.
- If a required sync or push is blocked, stop and report the exact command and error.
<!-- END BEADS INTEGRATION -->

<!-- BEGIN BEADS CODEX SETUP: generated by bd setup codex -->
## Beads Issue Tracker

Use Beads (`bd`) for durable task tracking in repositories that include it. Use the `beads` skill at `.agents/skills/beads/SKILL.md` (project install) or `~/.agents/skills/beads/SKILL.md` (global install) for Beads workflow guidance, then use the `bd` CLI for issue operations.

### Quick Reference

```bash
bd ready                # Find available work
bd show <id>            # View issue details
bd update <id> --claim  # Claim work
bd close <id>           # Complete work
bd prime                # Refresh Beads context
```

### Rules

- Use `bd` for all task tracking; do not create markdown TODO lists.
- Run `bd prime` when Beads context is missing or stale. Codex 0.129.0+ can load Beads context automatically through native hooks; use `/hooks` to inspect or toggle them.
- Keep persistent project memory in Beads via `bd remember`; do not create ad hoc memory files.

**Architecture in one line:** issues live in a local Dolt DB; sync uses `refs/dolt/data` on your git remote; `.beads/issues.jsonl` is a passive export. See https://github.com/gastownhall/beads/blob/main/docs/SYNC_CONCEPTS.md for details and anti-patterns.
<!-- END BEADS CODEX SETUP -->
