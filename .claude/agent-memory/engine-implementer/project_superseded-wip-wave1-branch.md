---
name: superseded-wip-wave1-branch
description: wip/sov-m0q-wave1 @ b699465 is fully superseded by main; its only unique content is the cancelled 250k bench lane. Verify a branch by blob-compare against main before spending a session on it.
metadata:
  type: project
---

`wip/sov-m0q-wave1` @ `b699465` ("UNGATED in-flight work from the sov-m0q wave") is
**superseded**. Verified 2026-08-28 against main @ `f6725f1`.

`git diff --name-status main wip/sov-m0q-wave1` yields exactly two `A` lines —
`simulation/benches/contract/mod.rs` and `simulation/benches/scale_250k.rs` — and everything
else is `D`, i.e. content main has that the branch lacks. Those two files are the **250k
benchmark lane the user cancelled on 2026-08-27** (`sov-1ae`, `sov-1jt` closed cancelled).

The branch does compile (`cargo build --workspace` clean, `cargo test --workspace` 108 tests
0 failed), contrary to `sov-jd4`'s description — but that is irrelevant, because main carries
a strictly better version of every renderer file on it.

**Why this matters as a method, not a fact:** two separate briefs described this branch by
listing files it does not contain (`tools/check_gpu_timing.py`, `run_validation_gate.py`, the
navi3x allowlist and baseline JSON — all on main, never on the branch), and one told me to
re-write the sov-abc query-slot fix that had already shipped as `d47a11a`. A ticket description
and a brief both describe a branch as it was believed to be at some past moment.

**How to apply:** before any work on a WIP branch, run these three, in this order, and let them
overrule the brief:

    git diff --name-status main <branch>            # A-lines are the only novel content
    git log --oneline main..<branch>                # and main..<branch> vs <branch>..main counts
    git cat-file -e main:<path>                     # per-file, for every file the brief names

Blob-compare (`git diff main:<path> <branch>:<path> | wc -l`) settles "is this file novel"
in one call and does not require checking anything out.

Related: [[project_gpu-timing-two-implementations]].
