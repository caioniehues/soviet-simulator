# Handoff — border settlement wave 1, 2026-09-04

**Kind:** plan handoff
**Authority:** operational handoff only; `bd` remains task-state authority
**Status:** wave 1 built and committed; `sov-5ut` ready; grill + spec done
**Owner:** project lead
**Last verified:** 2026-09-04

> Read this, then run `bd ready` — the queue is authoritative, this page is context.

## What happened this session

Closed out the docs-review aftermath and built the first half of the border settlement:

1. **Doc-sweep wave** (7 beads, 7 parallel agents): `sov-0kc sov-bu6 sov-a2p sov-6uy sov-bpp sov-kvn sov-9mz` — all committed per-bead, gates green.
2. **Pair**: `sov-brv sov-rut` → `sov-ik2` → `sov-3mi` — process/formula/skill/roster reconciled; checker widened with red-proven mutation tests.
3. **Border grill** (`/grill-with-docs`): all five cluster claims CONFIRMED by two scout briefs, with two frame corrections. Recorded as **ADR-0003** (`docs/decisions/0003-border-settlement.md`) + glossary terms **Border custody**, **Lost**.
4. **Spec** `sov-o1w` (ready-for-agent) + **tickets**: five beads re-AC'd, `sov-5ut` blocked by the other four.
5. **Build wave 1**: four implementers in isolated worktrees, red-then-green each; orchestrator merged to main in one landing commit.

Commits (all on `main`, nothing pushed):
```
74b5bda beads: close border wave 1, unblock sov-5ut
c252bb7 feat(simulation): border settlement lands together (sov-7f7, sov-20g, sov-uo5, sov-bub; spec sov-o1w, ADR-0003)
```

## Key facts for the next session

- **The five must land together** (ADR-0003) — wave 1 did; `sov-5ut` (shared exit helper) is the capstone and is READY.
- **Domestic money deltas are ZERO by construction** (`market.rs:539-545`) — never "fix" a zero domestic delta. Panel Expenses/Income prove border trade only.
- **Deletions are intentional honest loss** (7+ logged sites, re-credit only by physical return). `sov-bub` was observability work; behavior unchanged.
- **ToSource retries forever** — no timeout exists; the helper must bound it.
- **Fixture tail must stay under ~250k ticks** (`sov-aam` deadlock beyond ~300k).
- **Merge discipline that worked**: one worktree per agent, patches merged in dependency order, shared-file mtimes checked for strays, fuzz patches verified for duplicated blocks, guards proven red via stash, tmp probes deleted.
- **New open defects filed**: `sov-aam` (convoy freeze), `sov-ssv` (batched-road panic), `sov-ijo` (two trucks trip per-tick transport_grid check).

## Suggested next wave

1. `sov-5ut` via `/implement` (single agent, high effort): shared exit helper per spec `sov-o1w`, then `/code-review`, gate, commit, close.
2. Then `/diagnosing-bugs` for `sov-aam` (has failing-assertion sketch) and `sov-ssv`.
3. Then the remaining queue per `bd ready` (epic `sov-6pr` children mostly closed; check orphans).

## Working-tree state

Clean for `simulation/`, `native_app/`, `docs/` (verify with `git status`). Known pre-existing noise, not ours: `.claude/agent-memory/*` modifications, `simulation/src/transportation/vehicle.rs:125` fmt diff (leave it), `opencode.jsonc` `$schema` line (harness-owned). `world/` is gitignored cache. No live game process should remain (sov-app exited 0).

## Environment notes (omp, not project)

- Model moved from opus (which 429'd twice) to **muse-spark-1.3** mid-session; standing instruction is now **high effort** on all subagent dispatches.
- Retained in memory: orchestration preferences, delivery contract, economy/determinism truths, merge discipline, docs authority model, shipped-vs-open ledger.
