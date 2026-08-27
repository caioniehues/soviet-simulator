---
name: memory-capped-runs
description: Never run a suspected memory-growth reproduction uncapped — always wrap it in systemd-run with MemoryMax and MemorySwapMax=0; the cap is the evidence, not just a safety net
metadata:
  type: feedback
---

Any run that might exercise an unbounded allocation goes under a hard memory ceiling,
every time, including the very first exploratory run:

```
systemd-run --user --scope -q -p MemoryMax=2G -p MemorySwapMax=0 -- <cmd>
```

**Why:** on 2026-08-27 (sov-bo3) one uncapped `cargo test -p geom` reached anon-rss
17.3 GB, was killed by the SYSTEM OOM killer, pushed the user's desktop into memory
pressure, raised a kernel "Memory Shortage Avoided" notification on their screen, and
left swap at 19/30 GB. The user raised it with the lead. A cargo *test* command looks
harmless — that is exactly why the rule has to be unconditional rather than applied to
runs that "look risky".

**How to apply:**

- The cap is the guard, not merely protection: "killed by the cgroup at the ceiling
  before the fix, completes under the SAME ceiling after" is strictly better evidence
  than an uncapped crash. It is reproducible, bounded, and safe to re-run anywhere.
  Cite the cgroup kill (`journalctl -k | grep "Memory cgroup out of memory"`), never a
  system OOM.
- Never raise the ceiling to let a run "get further". If the fixed code needs more than
  the ceiling, that is a finding to report.
- Do not run several capped memory sweeps concurrently — recovering swap takes a while.
- Capture the exit code without a pipe (`> file 2>&1; echo $?`); a cgroup kill is 137,
  and `$?` after a pipe reads the last stage instead.
- Mark such sweeps `#[ignore]` with a doc comment naming the ceiling command, so CI
  never runs them unguarded.
