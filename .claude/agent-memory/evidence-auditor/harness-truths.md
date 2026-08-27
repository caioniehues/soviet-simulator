---
name: harness-truths
description: What this project's test harnesses genuinely prove and what they are repeatedly claimed to prove but do not
metadata:
  type: project
---

This is the thing most often overclaimed here. Correct people on it every time.

**`TestCtx::tick()` bincode-round-trips the whole `Simulation` and hash-compares.**
That proves serialize/deserialize round-trips. It **cannot** detect a simulation desync, because
there is only ever one run to compare. It is also blind to any field omitted from a `Serialize`
derive: such a field is neither saved nor hashed, and the comparison still matches.
Never let it be cited as a determinism proof. Never weaken it to make anything pass.

**Parallel `cargo test -p simulation` is trustworthy** since the `static mut` race in
`init.rs`/`prototypes` was removed (`sov-test-race-initfuncs-qt6`, 2026-08-26). Evidence produced
under parallel runs *before* that date may be unreliable — check the date before trusting it.
The same defect shape still exists in `native_app/src/init.rs:85-86`, which is not linked into
the test binary.

**A memory-ceiling run proves the allocation is bounded, and nothing about correctness.**
sov-bo3's 100k-placement sweep asserts flat RSS and `!mesh.faces.is_empty()`. It says nothing
about whether the geometry produced is *right*. A change that silently substitutes wrong-but-
non-empty geometry passes it. See [[weak-evidence-shapes]] and [[memory-ceiling-recipe]].

**`cargo test -p simulation` baseline as of 2026-08-27:** 45 passed, 0 failed, 1 ignored.
`cargo test -p geom`: 23 passed, 0 failed. A report claiming more passing tests than that
without adding any is wrong.
