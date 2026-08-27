---
name: memory-ceiling-recipe
description: How to watch an OOM guard fail in this repo without endangering the user's desktop — verified invocation and the ceiling self-check
metadata:
  type: project
---

Watching an unbounded-allocation guard fail means deliberately running the runaway. Do it inside
a cgroup, never bare.

**Rule: no uncapped runs.** The lead ruled on 2026-08-27 (sov-bo3) that the *capped* kill is the
"before" evidence, not the uncapped crash. An uncapped run on the user's desktop reached 17.3 GB
anon-rss, raised a kernel "Memory Shortage Avoided" notification and pushed swap to 19/30 GB.
**Why:** the evidence a cgroup kill gives is identical in kind and reproducible on any machine.
**How to apply:** wrap every such run, and say in the report that you did.

Verified working invocation (measured, not recalled):

```
systemd-run --user --scope -q -p MemoryMax=2G -p MemorySwapMax=0 -- <command>
```

**Prove the ceiling is really applied** before trusting a "did not die" result — a wrong
invocation gives a false "the guard never fires":

```
systemd-run --user --scope -q -p MemoryMax=2G -p MemorySwapMax=0 -- sh -c \
  'cat /sys/fs/cgroup/$(cut -d: -f3 /proc/self/cgroup | head -1 | sed s@^/@@)/memory.max'
# -> 2147483648
```

Read the kill back from `journalctl --user --since "-25 minutes"`: look for
`run-pNNN.scope: The kernel OOM killer killed some processes in this unit` plus a
`NG memory peak` line. A *system* OOM-killer line without a scope name means it escaped the
cgroup — that is the dangerous shape.

Traps found while doing this:
- Do not pipe the capped run through `grep`/`tail`; the pipeline can hold open long past the
  kill and you burn a 900 s timeout learning nothing. Redirect to a log file and read the file.
- `cargo test --release --no-run` first, outside the scope. Building under a 2G ceiling risks
  killing rustc instead of the test.
- `gen_exterior_house` reads `crate::colors()`, so any harness outside `TestCtx` must call
  `crate::init::init()` first, or the process dies in `unwrap_unchecked` — a debug-assert panic
  in dev and a bare SIGSEGV in release.

Related: [[proven-tests]], [[harness-truths]].
