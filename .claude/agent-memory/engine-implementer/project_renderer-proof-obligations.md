---
name: renderer-proof-obligations
description: How renderer claims get proven in this repo — the single-scene capture blind spot, and the trick for exercising an alternate scene without shipping one
metadata:
  type: project
---

`engine_demo` ships **one** hard-coded capture scene (`engine_demo/src/capture.rs`,
`BASELINE_SETTINGS`) with all nine timed passes enabled. Every conditional-pass code path is
therefore untested by anything automated.

**Why:** this blind spot is what let an alignment panic (see
[[wgpu-query-resolve-alignment]]) ship green through a build and a unit-test run.

**How to apply:** to exercise a pass-off path on real hardware, flip one field of
`BASELINE_SETTINGS` locally (e.g. `ssao: true` -> `false`), build, run
`./target/debug/engine_demo capture --scene baseline --out <dir> --gpu-timings`, then
**revert the file**. Confirm the revert with `git diff --stat` before committing. Doing the
same run against the pre-fix code (`git show HEAD:<file> > <file>`) gives a genuine
before/after, which is the only honest proof for a renderer defect — the sim test harness
cannot drive the renderer, and a passing build is not visual proof.

Reproducibility facts measured 2026-08-28 on AMD RX 7800 XT / RADV NAVI32, KDE Plasma on
Wayland: the offscreen no-surface capture path yields `sha256(baseline.png)` =
`e547e2636822621f18c85e129161975cde0dd03c42762dc0ce7a4c6b5fa12603`, reproduced three times
including across an unrelated `gpu_timing.rs` change. A `--validation` run on that host emits
**10** allowed messages, not the 15 some tickets claim — five `vkGetDeviceProcAddr` warnings are
environment-dependent. Never encode a fixed validation-message count as an invariant.
