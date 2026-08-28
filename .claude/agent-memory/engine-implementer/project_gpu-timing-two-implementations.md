---
name: gpu-timing-two-implementations
description: Two rival GPU pass-timing implementations exist off main - the wgpu-profiler spike is DONE and archived as a tag (sov-dda.1 closed); the hand-rolled gpu_timing.rs lives on feat/sov-uy2-capture. Check the branch before assuming either exists.
metadata:
  type: project
---

There are TWO per-pass GPU timing implementations, and **neither is on `main`** (still true
2026-08-28).

- `feat/sov-uy2-capture` → `engine/src/gpu_timing.rs`, hand-rolled against wgpu directly, flat
  9-pass enum with fixed query slots (`sov-sqs`); the `fix/sov-abc` never-reset-query-slot Vulkan
  UB repair is folded in. Worktree `/home/caio/sov-abc-wt`. This branch is the *live* one.
- **`spike/sov-dda-gpu` NO LONGER EXISTS.** The spike was finished and the branch + worktree were
  deleted. Its code survives only as the tag **`archive/sov-dda-1-wgpu-profiler`** = commit
  `69ffed1` (4 files, +157/-5: `Cargo.lock`, `engine/Cargo.toml`, `engine/src/gfx.rs`,
  `engine_demo/src/main.rs`). Opt-in via `SOV_GPU_PROFILE`.

**Status: `sov-dda.1` is CLOSED.** Verdict was ADOPT-but-deferred; the remaining swap is
`sov-ip7` (P3, open, blocked on `feat/sov-uy2-capture` landing). Do not re-run the spike.

**Why:** on 2026-08-28 a brief arrived asking me to create `spike/sov-dda-gpu` and build the
spike from scratch. `bd show sov-dda.1` showed it already closed with the full verdict. A brief
can be stale; the tracker is the authority. Read the ticket BEFORE creating the worktree.

**How to apply:** before any GPU-timing work run `bd show sov-dda.1 sov-ip7`, then `git tag -l`
and `git worktree list`. Never assume the module map in your own system prompt matches `main`;
`gpu_timing.rs` and `capture.rs` are listed there but are not on `main`.

Facts worth keeping from the spike (re-derivable from
`~/.cargo/registry/src/index.crates.io-*/wgpu-profiler-0.17.0/`):
- `wgpu-profiler` v0.17.0 is the ONLY release built against wgpu 0.20.x. v0.16.0 → wgpu 0.19,
  v0.18.0 → wgpu 22, v0.28.0 → wgpu 30. A caret range silently pulls a second wgpu into the tree.
  The `=0.17.0` pin is load-bearing and the user approved that exact version.
- `scope()` takes `&self` (`src/profiler.rs:131-136`), so it survives the rayon `in_place_scope`
  parallel render at `gfx.rs:736-763` with no device/queue ownership change.
- `begin_pass_query` needs only `Features::TIMESTAMP_QUERY` — the SAME bar as the hand-rolled
  path, not the narrower `_INSIDE_ENCODERS`/`_INSIDE_PASSES`.
- It cannot have the never-reset-slot UB by construction: `resolve_queries` resolves only
  `num_resolved..num_used` and offsets the destination by `num_resolved * QUERY_SIZE`.
- What it does NOT provide: min/median/max aggregation. That thin layer must be kept.
- Measured baseline to beat, RX 7800 XT (RADV NAVI32) @1280x720, medians n=30, 2026-08-28, via
  the hand-rolled path: main 93.2us, ssao 33.2us, fog 28.0us, depth_prepass 25.5us,
  background 24.4us, shadow_cascade_0 14.2us, cascades 1-3 ~10.5-10.9us.
