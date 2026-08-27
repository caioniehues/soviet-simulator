---
name: repo-has-no-test-ci
description: soviet-simulator has NO automated test CI — the only workflow is dependency-policy.yml (cargo-deny); every cargo test gate is manual
metadata:
  type: project
---

`.github/workflows/` contains exactly one file, `dependency-policy.yml`, and it runs
one `cargo-deny` job. There is no Makefile, no justfile, no xtask. Verified
2026-08-27 during the sov-bo3 gate: `grep -rn "cargo test" .github/` returns nothing.

**Why it matters:** any argument of the shape "this test is `#[ignore]`d so CI will
not catch a regression" is only half the story — CI catches NO test regression here.
The real difference `#[ignore]` makes is whether a developer's plain
`cargo test -p <crate>` runs it.

**How to apply:** when judging test-coverage findings, score them against "does a
default local `cargo test` run it", not against CI. And do not cite CI as the reason
a guard is safe. Follow-up issue for wiring the sov-bo3 memory guard: `sov-cnq`.
