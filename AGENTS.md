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
- Finish substantive waves with `doc-reality-auditor` and `scribe`; use release and performance roles only at their documented gates.

Use two or three subagents for normal waves and up to five for genuinely independent read-only work. Run at most two writing agents concurrently, with disjoint ownership. Every subagent receives a bounded brief, owned files, acceptance criteria, a `br` issue when applicable, and the exact verification command.

## Verification and delivery

- Run simulation tests as `cargo test -p simulation -- --test-threads=1`; parallel runs intermittently segfault on the known `init.rs` race.
- Name what each check proves and confirm test filters execute at least one test.
- Preserve unrelated changes and never stage with `git add -A` or `git add .`.
- Stage only the four documented `.beads` files when task-ledger state changes.
- Player-facing work finishes with an inspected screenshot or 15–20 second video when `CLAUDE.md` requires visual proof.

For generated visual assets, use Codex's `imagegen` skill and confirm paid generation with the user before the first spend.

<!-- bv-agent-instructions-v3 -->

---

## Beads Workflow Integration

This project uses [beads_rust](https://github.com/Dicklesworthstone/beads_rust) (`br`) for issue tracking and [beads_viewer](https://github.com/Dicklesworthstone/beads_viewer) (`bv`) for graph-aware triage. Issues are stored in `.beads/` and tracked in git. Current `br` workspaces normally export `.beads/issues.jsonl`; older `bd`/legacy workspaces may use `.beads/beads.jsonl`. `bv` auto-discovers the supported JSONL files, so agents should use `br`/`bv` commands instead of hard-coding a single filename.

### Using bv as an AI sidecar

bv is a graph-aware triage engine for Beads projects. Instead of parsing .beads/issues.jsonl / .beads/beads.jsonl directly or hallucinating graph traversal, use robot flags for deterministic, dependency-aware outputs with precomputed metrics (PageRank, betweenness, critical path, cycles, HITS, eigenvector, k-core).

**Scope boundary:** bv handles *what to work on* (triage, priority, planning). `br` handles creating, modifying, and closing beads.

**CRITICAL: Use ONLY --robot-* flags. Bare bv launches an interactive TUI that blocks your session.**

#### The Workflow: Start With Triage

**`bv --robot-triage` is your single entry point.** It returns everything you need in one call:
- `quick_ref`: at-a-glance counts + top 3 picks
- `recommendations`: ranked actionable items with scores, reasons, unblock info
- `quick_wins`: low-effort high-impact items
- `blockers_to_clear`: items that unblock the most downstream work
- `project_health`: status/type/priority distributions, graph metrics
- `commands`: copy-paste shell commands for next steps

```bash
bv --robot-triage        # THE MEGA-COMMAND: start here
bv --robot-next          # Minimal: just the single top pick + claim command

# Token-optimized output (TOON) for lower LLM context usage:
bv --robot-triage --format toon
```

Before claiming, verify current state with `br show <id> --json` or `br ready --json`. `recommendations` can include graph-important blocked or assigned work; only `quick_ref.top_picks` and non-empty `claim_command` fields represent claimable work.

#### Other bv Commands

| Command | Returns |
|---------|---------|
| `--robot-plan` | Parallel execution tracks with unblocks lists |
| `--robot-priority` | Priority misalignment detection with confidence |
| `--robot-insights` | Full metrics: PageRank, betweenness, HITS, eigenvector, critical path, cycles, k-core |
| `--robot-alerts` | Stale issues, blocking cascades, priority mismatches |
| `--robot-suggest` | Hygiene: duplicates, missing deps, label suggestions, cycle breaks |
| `--robot-diff --diff-since <ref>` | Changes since ref: new/closed/modified issues |
| `--robot-graph [--graph-format=json\|dot\|mermaid]` | Dependency graph export |

#### Scoping & Filtering

```bash
bv --robot-plan --label backend              # Scope to label's subgraph
bv --robot-insights --as-of HEAD~30          # Historical point-in-time
bv --recipe actionable --robot-plan          # Pre-filter: ready to work (no blockers)
bv --recipe high-impact --robot-triage       # Pre-filter: top PageRank scores
```

### br Commands for Issue Management

```bash
br ready --json                       # Show issues ready to work (no blockers)
br list --status=open --json          # All open issues
br show <id> --json                   # Full issue details with dependencies
br create --title="..." --type=task --priority=2 --json
br update <id> --status=in_progress --json
br close <id> --reason="Completed" --json
br close <id1> <id2> --reason="Completed" --json
br sync --flush-only                  # Export DB to JSONL after Beads mutations
```

### Workflow Pattern

1. **Triage**: Run `bv --robot-triage` to find the highest-impact actionable work
2. **Claim**: Use `br update <id> --status=in_progress --json`
3. **Work**: Implement the task
4. **Complete**: Use `br close <id> --reason="Completed" --json`
5. **Sync**: Run `br sync --flush-only` after Beads mutations so the JSONL export is current

### Key Concepts

- **Dependencies**: Issues can block other issues. `br ready --json` shows only unblocked work.
- **Priority**: P0=critical, P1=high, P2=medium, P3=low, P4=backlog (use numbers 0-4, not words)
- **Types**: task, bug, feature, epic, chore, docs, question
- **Blocking**: `br dep add <issue> <depends-on>` to add dependencies

### Git Policy

`br` never commits or pushes. Follow this repository's own git instructions before staging, committing, or pushing. If the repository says "commit only when asked," that rule overrides any generic workflow advice.

<!-- end-bv-agent-instructions -->
