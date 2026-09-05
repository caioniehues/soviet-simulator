---
name: cargo-mutants-recipe
description: How to run and audit cargo-mutants in this repo — the TMPDIR trap, costs, and the control mutation that makes a survivor list believable
metadata:
  type: project
---

Policy: `docs/process/mutation-policy.md` (sov-4f7). Trial result:
`docs/process/mutation-trial-sov-mwy.md` (sov-mwy). Tool pinned at **cargo-mutants 27.1.0, MIT**
(`Copyright (c) 2021 Martin Pool`) — build-time only, never linked, absent from every `Cargo.toml`,
so MIT places no obligation on this GPL-3.0 tree.

**The TMPDIR trap.** `cargo-mutants` copies the whole source tree into `TMPDIR`. A `TMPDIR`
*inside* the tree recurses into itself and aborts in ~6 s with `File name too long (os error 36)`.
Put it in a **sibling** directory under `/home` — never inside the worktree, and never `/tmp`
(16 GB tmpfs; a target dir there kills every Bash call in the session).

**Measured costs** (host `hal`, 12 threads): `market.rs` file-scoped, 163 mutants, `-t 90 -j 4`
→ **37.7 min**. Whole `simulation` package is 3036 mutants ≈ 25 h, which is why a full scan per
change is forbidden rather than discouraged. Baseline `cargo test -p simulation` ≈ 34 s at 52
tests; each hand-run mutant costs ~31-48 s of test plus an incremental rebuild.

**Reading the output.** `mutants.out/` holds `caught.txt`, `missed.txt`, `timeout.txt`,
`unviable.txt` and `debug.log` — audit a trial report against these files, one survivor at a time,
not against its prose. `debug.log`'s final timestamp corroborates the wall-clock claim. A
**timeout is inconclusive**: never a survivor, never a kill, and never re-run at higher `-t` to
manufacture a kill. **Unviable** means it did not compile — mostly synthesised
`Box::leak(Default::default())` for `&T` returns — and is evidence of nothing.

**Always run a control before reporting survivors.** A list of "all survived" is
indistinguishable from a harness that cannot fail. Mutate a line the run recorded as *caught*,
in the same file, and paste the red. The one I used:
`market.rs` `settle_retail` `-=`→`+=` → `test result: FAILED. 51 passed; 1 failed`.

Auditing by hand beats re-running the tool when the base has moved: apply one mutation, run the
full suite, revert, in a single chained command so the restore survives a timeout. See
[[proven-tests]] and [[weak-evidence-shapes]].
