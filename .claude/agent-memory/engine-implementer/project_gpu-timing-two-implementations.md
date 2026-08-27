---
name: gpu-timing-two-implementations
description: Two rival GPU pass-timing implementations exist off main (hand-rolled gpu_timing.rs on wip/sov-m0q-wave1, wgpu-profiler v0.17.0 spike on spike/sov-dda-gpu) - check which branch you are on before assuming either exists
metadata:
  type: project
---

As of 2026-08-27 there are TWO per-pass GPU timing implementations, and **neither is on `main`**.

- `wip/sov-m0q-wave1` → `engine/src/gpu_timing.rs`, hand-rolled against wgpu directly, flat
  9-pass enum with fixed query slots (`sov-sqs`). `fix/sov-abc` adds 94 lines repairing a
  never-reset-query-slot Vulkan UB in it.
- `spike/sov-dda-gpu` (worktree `/home/caio/sov-dda-gpu-wt`) → `wgpu-profiler = "=0.17.0"` wired
  into `engine/src/gfx.rs`, opt-in via `SOV_GPU_PROFILE` (`sov-dda.1`, commit `69ffed1`).

**Why:** the lead's first brief for `sov-dda.1` did not know the hand-rolled module existed, and
my own agent-definition module map lists `gpu_timing.rs` and `capture.rs` as if they were on
`main` — they are not. I wasted no code on it only because the lead caught it mid-task.

**How to apply:** before any GPU-timing work, run `git branch -a` and confirm which of these your
worktree carries. Never assume the module map in your own system prompt matches `main`; check
`ls engine/src/` first. Both branches are ungated — treat their code as evidence of intent, not
as proven-correct.

Pinning fact worth keeping: `wgpu-profiler` v0.17.0 is the ONLY release built against wgpu 0.20.x.
v0.16.0 → wgpu 0.19, v0.18.0 → wgpu 22, v0.28.0 → wgpu 30. A caret range silently pulls a second
wgpu into the tree. The `=0.17.0` pin is load-bearing and the user approved that exact version.
