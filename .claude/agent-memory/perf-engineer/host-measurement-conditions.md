---
name: host-measurement-conditions
description: What the dev box does to benchmark numbers, and how much noise concurrent agents add
metadata:
  type: project
---

Dev machine "hal": AMD Ryzen 5 7600X, 6 cores / 12 threads, 30 GiB RAM, CachyOS
kernel 7.2.0, cpu governor `performance`, rayon uses 12 threads.

**Concurrent agents are the dominant noise source.** Measured 2026-08-27 during a
four-agent wave sharing one `target/` directory:

| loadavg at run | p10-p90 spread as % of p50 |
|---|---|
| ~2.5 | 5-6% |
| ~10 | 94% |
| ~28 | 137% |

At loadavg 10+ single ticks came back 50x the median (5.1 ms against a 0.10 ms
median). Those are scheduler preemptions, not simulation behaviour.

**Why:** a 5% difference is inside the noise floor even on a quiet box, and the
noise floor moves by more than 20x with system load.

**How to apply:** record loadavg with every number and report median plus spread,
never a single run. Treat `min` as the least-contaminated estimate when the box is
busy. Do not accept any regression claim whose delta is smaller than the measured
spread for that run. `cargo bench` waits on the shared cargo lock; a long wait is
contention, not a hang.

Related: [[baselines-250k]]
