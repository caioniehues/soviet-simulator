# Handoff — sov-m0q tooling evidence wave, 2026-08-27

**Kind:** session handoff
**Authority:** operational handoff only; `bd` is task-state authority
**Status:** wave cut short by the user; 2 of 10 children closed
**Owner:** project lead
**Last verified:** 2026-08-27

## Read this first

The goal was "finish the tooling epic". **It is not finished.** Two of ten children are closed.
Three agents were stopped mid-task at the user's instruction. Their unfinished work is committed
on a branch, NOT on main.

## Where everything is

| Thing | Location |
|---|---|
| Finished, gated, on main | commits `2550026`, `a6bff8d`, `6525f40` |
| UNGATED in-flight work | branch **`wip/sov-m0q-wave1`**, commit `b699465` (pushed) |
| Task state | `bd`, epic `sov-m0q` |
| Dead worktree, safe to remove | `/tmp/sov-f1v-worktree` — its files were copied into main |
| Pinned tool | `/tmp/sov-f1v-tools/bin/cargo-deny` 0.20.2 (ephemeral `/tmp`; the durable install is in the policy doc) |

## What landed on main and is proven

- **`sov-f1v` CLOSED.** `deny.toml` + `docs/process/dependency-policy.md`. The gate was watched
  failing twice and restored: dropping `RUSTSEC-2022-0104` gave `advisories FAILED` exit 1;
  dropping yakui from `sources.allow-git` gave `sources FAILED` exit 8. Restoration proven by
  `sha256sum`, because `deny.toml` is untracked and `git diff` on it is vacuous.
- **The root cause that unblocked it:** all 13 workspace members omitted `publish = false`, which
  made both `allow-wildcard-paths` and `licenses.private` inert. One key per manifest fixed
  `bans` and `licenses` together.
- **`sov-4f7` CLOSED.** `docs/process/mutation-policy.md`, ~415 lines, every command executed
  before being written down. Measured: 3036 mutants for the `simulation` package at ~30s each is
  ~25 hours, which is why a full scan per change is forbidden rather than discouraged.
- **Roster.** Five new implementers own what nothing owned: `engine-implementer`,
  `geom-implementer`, `widget-implementer`, `net-implementer`, `common-implementer`. Measured
  34,762 previously unowned lines, not the ~26,000 `development-cycle.md` claimed. Both roster
  tables now match the agents on disk.

## The one urgent thing

**`sov-bo3`, P1 — `geom/src/skeleton.rs:721` `LAV::iter_keys` is unbounded and OOM-kills the game.**
It terminates only on `next == head`, so a cycle not passing through `head` grows a `Vec` forever.
The struct has a `len` field four lines above that the walk ignores. Seven call sites.
Measured: 17.6 GB RSS, OOM-killed, allocation request of exactly 2^33 bytes. Reachable from the
player's own `MapBuildSpecialBuilding` path, so it is not confined to the benchmark. It breaks the
never-game-over pillar and it BLOCKS `sov-1ae`.

**Do not re-run the uncapped 250k contract on the user's machine until this is fixed.** The user
reported it "almost crashes my pc". That is this defect, not benchmark weight.

## Two user constraints that change how work is dispatched

1. **Renderer tickets are iterative-session work.** `sov-uy2`, `sov-ba8`, `sov-sqs` must be driven
   from the main session with the user watching frames. Do not hand them to a background agent
   again. `sov-ba8`'s acceptance explicitly forbids an uninspected capture, and the user inspects
   personally on a Radeon RX 7800 XT and an OLED display, on Wayland.
2. **The 250k contract needs an RSS cap** or a fixed `sov-bo3`, whichever comes first.

## Leads worth chasing, not yet verified

- **wgpu validation reports real synchronization hazards.** The chain-D agent's last message said
  validation works on the pinned wgpu and immediately reports hazards. Unverified by the lead.
  Recorded on `sov-ba8`. If true, `sov-ba8`'s own trap was correct and the stale code comments
  claiming validation is unavailable are wrong.
- **The knowledge graph has a readable SQLite database.** The chain-C agent, blocked from the MCP
  tools, read `.code-review-graph/`'s database directly, read-only. That is a viable fallback for
  any agent that cannot reach the MCP server.

## The tooling trap that cost this session three rounds of churn

All 16 project agents carried `tools: Read, Edit, Write, Grep, Glob, Bash, ToolSearch, Agent,
SendMessage, Skill`. **`tools:` is an allowlist**, it named no `mcp__` pattern, so it silently
excluded every MCP tool from every subagent. The user removed the line from all 16; the five new
agents pin no `tools:` key either. Verified with `git show HEAD:.claude/agents/sim-implementer.md`.

Two rules follow. A running subagent keeps its **spawn-time** toolset, so check
`git show HEAD:<agent file>` when a live agent's capability is in question. And a worker saying a
tool is "not in my tool list" proves nothing — deferred schemas never appear there; only
`ToolSearch("select:...")` returning *no matching deferred tools found* proves absence.

## Next actions, in order

1. **`sov-bo3`** — give it to `geom-implementer`. Read the trap in its description first: a silent
   `len` cap converts a crash into wrong geometry, which may be worse.
2. **`sov-m0q.1`** — the CI workflow ran for the first time on push (`33103583555`). Confirm it
   went green, then apply Mutation B on a PR to prove it renders as FAILED, then close `sov-ztg`.
   Use Mutation B, not an advisories mutation: `sources` derives only from `Cargo.lock` and is
   identical on any machine, whereas `advisories` varies with the RustSec snapshot.
3. **`sov-1ae`** — only after `sov-bo3`. `simulation/benches/scale_250k.rs` already exists on the
   WIP branch and its digest relationship is proven. What is missing is
   `docs/reference/benchmark-contract.md`; the agent was cut one step before writing it.
4. **The WIP branch needs the gate** — `wiring-auditor` then the opus `reviewer` — before any of
   `b699465` reaches main. None of it has been reviewed and it is not proven to compile.
