---
name: gpu-timing-two-implementations
description: The hand-rolled gpu_timing.rs and capture.rs ARE on main as of 2026-08-28 (merge 1d358fb), with the sov-abc fix folded in; the wgpu-profiler rival survives only as a tag. Do not trust the older "neither is on main" claim.
metadata:
  type: project
---

**CORRECTED 2026-08-28.** This note previously said neither implementation was on `main`.
That is now FALSE.

`main` carries the hand-rolled path: `engine/src/capture.rs` and `engine/src/gpu_timing.rs`,
landed via `1d358fb` "Merge feat/sov-uy2-capture: renderer evidence contract, offscreen
capture, two real gates". The stack on main includes:

- `b1a0d40` feat(engine): fixed capture contract with per-pass GPU timing (sov-uy2, sov-abc)
- `4dbeb0b` feat(engine): capture offscreen with no surface and no window (sov-pci)
- `d47a11a` fix(engine): resolve every query run at the only alignment-legal offset (sov-abc)

So the sov-abc never-reset-query-slot repair is **on main**, as `written_query_runs()` in
`gpu_timing.rs` plus four unit tests (the 2^9-mask alignment guard is
`every_resolve_offset_is_alignment_legal_for_all_masks`). Also on main:
`engine_demo/validation_allowlists/radv-navi3x.txt`,
`engine_demo/gpu_timing_baselines/radv-navi3x/baseline.json`, `tools/check_gpu_timing.py`,
`tools/run_validation_gate.py`, `docs/process/mutation-policy.md`.

The rival: **`spike/sov-dda-gpu` NO LONGER EXISTS**; the wgpu-profiler code survives only as
tag `archive/sov-dda-1-wgpu-profiler` = `69ffed1`. `sov-dda.1` is CLOSED (ADOPT-but-deferred);
the remaining swap is `sov-ip7` (P3). Do not re-run the spike.

**How to apply:** before any GPU-timing work, run `git log main --oneline -- engine/src/gpu_timing.rs`
and read the file on main. Two branches now hold *stale* copies and will mislead you:
`wip/sov-m0q-wave1` @ `b699465` and `feat/sov-uy2-capture`. See
[[project_superseded-wip-wave1-branch]].

Facts worth keeping from the wgpu-profiler spike (re-derivable from
`~/.cargo/registry/src/index.crates.io-*/wgpu-profiler-0.17.0/`):
- `wgpu-profiler` v0.17.0 is the ONLY release built against wgpu 0.20.x. A caret range silently
  pulls a second wgpu into the tree. The `=0.17.0` pin is load-bearing.
- `scope()` takes `&self` (`src/profiler.rs:131-136`), so it survives the rayon
  `in_place_scope` parallel render at `gfx.rs:736-763`.
- `begin_pass_query` needs only `Features::TIMESTAMP_QUERY`.
- It cannot have the never-reset-slot UB by construction: `resolve_queries` resolves only
  `num_resolved..num_used`.
- It does NOT provide min/median/max aggregation; that thin layer must be kept.
- Measured baseline, RX 7800 XT (RADV NAVI32) @1280x720, medians n=30, 2026-08-28, hand-rolled
  path: main 93.2us, ssao 33.2us, fog 28.0us, depth_prepass 25.5us, background 24.4us,
  shadow_cascade_0 14.2us, cascades 1-3 ~10.5-10.9us.
