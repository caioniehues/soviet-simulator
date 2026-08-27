---
name: dependency-policy-baseline
description: cargo-deny 0.20.2 policy baseline for this repo - the publish=false root cause, the two mutations that prove the gate, and what makes the baseline go stale
metadata:
  type: project
---

The dependency gate is `cargo-deny check` at repo root, pinned to **0.20.2**
(`cargo install cargo-deny --version 0.20.2 --locked`). Config: `deny.toml`.
Evidence and exceptions: `docs/process/dependency-policy.md`. Tickets `sov-f1v`
(baseline, closed 2026-08-27) and `sov-ztg` (CI, deliberately left open).

**Why:** the repo had no dependency policy and no CI at all. The failure the
chain existed to avoid was a gate that passes because it silently checks nothing.

**How to apply:**

- **`publish = false` on all 13 workspace members is load-bearing.** cargo-deny
  decides "private" from `publish`. Without it, BOTH `bans.allow-wildcard-paths
  = true` and `licenses.private.ignore = true` become inert, and `bans` +
  `licenses` fail with exit 6. This cost two agents a full RED cycle before the
  cause was found. Guard comments now sit next to both settings in `deny.toml`.
  If a manifest ever loses `publish = false`, that is the first thing to check.
- **Two mutations are known to make the gate go red.** Prefer B for any
  cross-machine comparison, because `sources` is derived only from `Cargo.lock`
  and so is identical everywhere.
  - A: delete the `RUSTSEC-2022-0104` ignore entry -> `advisories FAILED`, exit 1,
    `error[unmaintained]: structopt is in maintenance mode`.
  - B: remove `https://github.com/Uriopass/yakui` from `sources.allow-git` ->
    `sources FAILED`, exit **8**, five `error[source-not-allowed]` findings
    (yakui, yakui-core, yakui-wgpu, yakui-widgets, yakui-winit).
- **Green output is** `advisories ok, bans ok, licenses ok, sources ok`, exit 0,
  0 errors, **24** `warning[duplicate]`. Duplicates are `multiple-versions =
  "warn"` and do NOT gate. Do not describe duplicates as enforced.
- **The baseline goes stale two ways.** (1) Any `Cargo.lock` change can add an
  unseen licence or advisory - re-run and re-record. (2) `advisories` is checked
  against the RustSec DB (`~/.cargo/advisory-dbs/`), so a green result can turn
  red with no repo change. `bans`/`licenses`/`sources` are lockfile-derived and
  are the reproducible three.
- `deny.toml` and the policy doc were untracked at the time of writing, so
  `git diff` cannot prove a restore. **Use `sha256sum`.**

See [[licence-inventory]] and [[verification-procedures]].
