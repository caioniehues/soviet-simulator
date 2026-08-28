---
name: renderer-evidence-tooling
description: engine_demo capture / gpu_timing / validation-gate seam — the wgpu 256-byte query-resolve alignment trap, both evidence gates' blind spots, and the proven offscreen-capture determinism hash
metadata:
  type: project
---

Reviewed 2026-08-28 on branch `feat/sov-uy2-capture` (worktree `/home/caio/sov-abc-wt`),
tickets sov-uy2 / sov-abc / sov-91s / sov-uuo / sov-pci.

## The wgpu query-resolve alignment trap (pinned wgpu 0.20.1 / wgpu-core 0.21.1)

`command_encoder_resolve_query_set` rejects any `destination_offset` that is not a
multiple of **256** (`wgpu-core-0.21.1/src/command/query.rs:423`;
`wgpu-types-0.20.0/src/lib.rs:80` `QUERY_RESOLVE_BUFFER_ALIGNMENT = 256`). The
`gpu_timing` resolve buffer is `N_QUERIES * 8 = 144` bytes, so **offset 0 is the only
legal destination** — positional per-run offsets can never work. Any "resolve only the
written runs" fix that keeps slot-positional offsets is wrong for every mask that does
not start at pass 0.

Sibling fact, from sov-abc's own research: `reset_queries` (same file, :48-84) only
resets runs actually used, so resolving an unwritten slot with `WAIT` is Vulkan UB.
Both halves matter; a fix must satisfy both.

## B1/B2 CLOSED 2026-08-28 (re-gate on rewritten history, 7 commits)

`written_query_runs` now resolves EVERY run to `destination_offset: 0` (the only legal
value) and copies each to a positional `readback_offset` — `copy_buffer_to_buffer` needs
only `COPY_BUFFER_ALIGNMENT` = 4 (`wgpu-core-0.21.1/src/command/transfer.rs:656-663`).
The ordering is safe, but NOT for the reason the code comment gives: wgpu-core's buffer
tracker emits a real `transition_buffers` barrier on every COPY_DST<->COPY_SRC flip
(`query.rs:503` and `transfer.rs:735`). In-order execution alone would not be enough.

Hardware-confirmed on the RX 7800 XT with `BASELINE_SETTINGS.ssao = false`:
pre-fix EXIT=101 `Resolve buffer offset has to be aligned to
QUERY_RESOLVE_BUFFER_ALIGNMENT`; post-fix EXIT=0, 8 passes timed, ssao absent, only the
10 allow-listed SYNC-HAZARDs.

## The two evidence gates and what they do not catch

- `tools/run_validation_gate.py` — greps combined child output for
  `sync-hazard|validation error|validation warning|wgpu error` (case-folded). The real
  RADV lines are `Validation Error: [ SYNC-HAZARD-WRITE-AFTER-WRITE ]`, so the markers
  do match. CLOSED 2026-08-28. Zero output is exit 2, and
  `--capture-record` is now bound to the run by an mtime floor taken before the child
  spawns (`require_record_written_by_this_run`). The stale-record bypass I found — capture
  without `--validation` to a new `--out`, record left pointing at the old one — is exit 2.
  `stat()` follows symlinks, so a fresh link to an old record is also rejected.
  Freshness was chosen over deriving the path from the child's `--out` on purpose: argv
  parsing couples a general-purpose wrapper to one binary's CLI and **fails open** when a
  flag is renamed. Residuals, all measured, none reachable by operator slip: mtime binds
  freshness not authorship, so a mid-run `touch`, a concurrent unrelated writer, and a
  sub-second window from flooring (a record written 0.783s BEFORE the gate started was
  accepted) all pass.
- `tools/check_gpu_timing.py` — one global `max_regression_fraction` for all 9 passes.
  At 0.30 it rejects a 31.6% main regression (proved, exit 1) but **accepts a 29% ssao
  regression** whose measured spread is ~1%.
- Neither gate is referenced by any doc, CI file, or cargo hook. Repo still has no test
  CI (see [[repo-has-no-test-ci]]).

## Capture determinism, measured

Two detached debug runs plus the committed release artifact all hash
`e547e2636822621f18c85e129161975cde0dd03c42762dc0ce7a4c6b5fa12603`
(`baseline.png`, 1280x720, RADV NAVI32). Offscreen capture works from a background
agent with no compositor — `./target/debug/engine_demo capture --out DIR` is a cheap,
real end-to-end renderer check available to any agent.

`--gpu-samples N` silently clamps to `warmup_frames + 1` while the sidecar still writes
`"sample_frames": N` — a record that disagrees with its own `samples` field.

## The knowledge graph is useless for branch review here

`get_minimal_context_tool` returns `status: not_ready / stale_graph` whenever HEAD is
not the indexed main SHA, and it refuses. `tests_for("engine/src/gfx.rs")` answers with
8 `indirect: true` geometry tests in `geom/src/obb.rs` and `geom/src/skeleton.rs` —
transitive-dependency noise, not coverage. Read the diff; the graph adds nothing on a
feature branch.

Related: [[review-method-patterns]], [[repo-has-no-test-ci]]
