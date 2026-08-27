# Dependency policy baseline

**Kind:** process policy  
**Authority:** operational  
**Status:** active  
**Owner:** project lead  
**Last verified:** 2026-08-27

Tracker: `sov-f1v` (this baseline), `sov-ztg` (CI enforcement).

## Local command

Install the pinned checker outside this repository. The version is pinned and the
install is `--locked`, so the checker's own dependency graph is reproducible:

```sh
cargo install cargo-deny --version 0.20.2 --locked
```

`cargo-deny` 0.20.2 is licensed `MIT OR Apache-2.0`. Verified against the crate's
own manifest, `~/.cargo/registry/src/index.crates.io-*/cargo-deny-0.20.2/Cargo.toml:45`.
This policy is a technical dependency check. It is not legal advice or license
approval.

Run the full local policy command from the repository root:

```sh
cargo-deny check
```

`check` with no sub-argument runs all four checks: `advisories`, `bans`,
`licenses`, `sources`. Duplicate-version detection is part of the `bans` check.

The evidence recorded in this document was produced with the same 0.20.2 binary
installed to a session-local root, invoked by absolute path:

```sh
/tmp/sov-f1v-tools/bin/cargo-deny check
```

Only the invocation path differs. The binary, the version, the sub-command, the
argument list, the working directory and `deny.toml` are the same. `/tmp` is
ephemeral; do not treat that path as durable.

## Baseline rules

**The baseline is a statement about one `Cargo.lock`.** It was recorded against
the `Cargo.lock` at the 2026-08-27 wave-1 state (339 third-party crates, no
`criterion`). Any change to `Cargo.lock` can pull in a crate with a license or
an advisory this baseline never saw. Re-run the check and re-record this
document whenever `Cargo.lock` changes. A green result from an earlier lockfile
says nothing about a later one.

`deny.toml` permits crates.io and only the existing `emilk/egui` and
`Uriopass/yakui` Git repositories. It denies all other registries and Git
sources. It denies wildcard dependency specifications from registries.
Duplicate crate versions **warn**; they do not fail the command. They stay
visible until a separate inventory reduces them. The 2026-08-27 baseline emits
24 `warning[duplicate]` entries and still exits 0.

### Why the 13 workspace members set `publish = false`

Every one of the 13 workspace members carries `publish = false` in its
`[package]` block. This is **load-bearing for two settings in `deny.toml`**, and
removing it silently disables both:

| Setting | What it does | What breaks without `publish = false` |
| --- | --- | --- |
| `bans.allow-wildcard-paths = true` | Permits version-less `path` / `workspace` dependency specifications between our own crates | cargo-deny refuses it: *"allow-wildcard-paths is enabled, but does not apply to public crates as crates.io disallows path dependencies"*, and the `bans` check fails |
| `licenses.private.ignore = true` | Skips license metadata checks on our own unpublished crates | The 11 local crates without `license` metadata are counted as unlicensed third-party crates, and the `licenses` check fails |

cargo-deny decides "private" from `publish`. A crate that does not say
`publish = false` is a publishable crate as far as the tool is concerned, so
both settings above become inert. That was the root cause of the first drafted
baseline failing `bans` and `licenses` with exit 6.

`publish = false` is **truthful metadata, not a suppression trick.** None of the
13 crates is published to crates.io; several could not be, because they depend
on Git sources and on each other by path, both of which crates.io rejects. The
declaration states a fact that was already true. It changes no dependency, no
feature and no compiled code.

Members and their metadata, from `cargo metadata --no-deps` on 2026-08-27:
13 members, 13 with `publish = false`, 11 without `license` metadata.
`egui-inspect` 0.4.0 and `egui-inspect-derive` 0.4.1 are the two that declare a
license (`MIT or Apache-2.0`, inherited from the upstream crates they were
forked from). Repository-wide licensing is GPL-3.0 by inheritance and is
governed by `LICENSE`, not by per-crate manifest metadata.

`licenses.private` ignores only unpublished workspace crates. It does not add a
blanket exception for unknown or unlicensed third-party crates.

### License inventory

Verified 2026-08-27 by re-running `cargo-deny list -f json` and counting the
distinct license identifiers over 339 third-party crates. The inventory contains
exactly 14 identifiers: the 13 in the `deny.toml` allowlist, plus
`LicenseRef-UFL-1.0`, which is covered by the single narrow `epaint` exception.

| License | Crates |
| --- | --- |
| MIT | 344 |
| Apache-2.0 | 272 |
| Zlib | 14 |
| Unlicense | 7 |
| BSD-3-Clause | 6 |
| CC0-1.0 | 4 |
| Apache-2.0 WITH LLVM-exception | 3 |
| BSD-2-Clause | 3 |
| ISC | 2 |
| 0BSD | 1 |
| BSL-1.0 | 1 |
| OFL-1.1 | 1 (`epaint`) |
| Unicode-DFS-2016 | 1 |
| LicenseRef-UFL-1.0 | 1 (`epaint`, by exception) |

Counts sum above 339 because a crate with a disjunctive expression such as
`MIT OR Apache-2.0` is listed under both identifiers.

`epaint` 0.27.2 declares
`(MIT OR Apache-2.0) AND OFL-1.1 AND LicenseRef-UFL-1.0`. Verified in the Git
checkout at `crates/epaint/Cargo.toml:9`, where the upstream comment records
that OFL and UFL apply to the bundled `default_fonts`. Upstream's own
`deny.toml:84` allows `LicenseRef-UFL-1.0` for the same reason. The narrow
`epaint` exception in our `deny.toml` permits that non-SPDX branch only. A
license-file hash is not required, because the release carries an explicit
manifest expression and upstream policy records the same allowance.

### The advisories check is time-varying

`advisories` is checked against the RustSec advisory database, which is fetched
from `https://github.com/RustSec/advisory-db` and cached under
`~/.cargo/advisory-dbs/`. The 2026-08-27 baseline was taken at advisory-db
commit `6420e39260b3d771b049954cf5d52b57e2118da4`.

Consequence: **a green result can turn red with no change to this repository**,
when a new advisory is published against a crate already in `Cargo.lock`. That
is intended behaviour and not a flake. `bans`, `licenses` and `sources` are
derived only from `Cargo.lock` and the local manifests, so those three are
reproducible from the repository alone.

## Time-limited exceptions

Every exception below has owner **project lead** and review date
**2026-11-25**. The entries record known findings in the present lockfile. They
do not waive remediation.

| Finding | Reason | Owner | Review date |
| --- | --- | --- | --- |
| RUSTSEC-2021-0065 (`anymap`) | Transitive through yakui. | project lead | 2026-11-25 |
| RUSTSEC-2021-0139 (`ansi_term`) | Transitive through structopt. | project lead | 2026-11-25 |
| RUSTSEC-2022-0104 (`structopt`) | Used by `headless`; migrate it. | project lead | 2026-11-25 |
| RUSTSEC-2024-0370 (`proc-macro-error`) | Existing macro dependency. | project lead | 2026-11-25 |
| RUSTSEC-2024-0375 (`atty`) | Existing CLI dependency. | project lead | 2026-11-25 |
| RUSTSEC-2024-0379 and RUSTSEC-2025-0003 (`fast-float`) | Existing UI stack. | project lead | 2026-11-25 |
| RUSTSEC-2024-0436 (`paste`) | Existing transitive dependency. | project lead | 2026-11-25 |
| RUSTSEC-2025-0056 (`adler`) | Existing image and backtrace dependencies. | project lead | 2026-11-25 |
| RUSTSEC-2025-0141 (`bincode`) | Direct serialization dependency; choose a maintained format. | project lead | 2026-11-25 |
| RUSTSEC-2026-0007 (`bytes`) | Existing platform dependency. | project lead | 2026-11-25 |
| RUSTSEC-2026-0009 (`time`) | Development-only test dependency. | project lead | 2026-11-25 |
| RUSTSEC-2026-0192 (`ttf-parser`) | Existing font-rendering dependency. | project lead | 2026-11-25 |
| RUSTSEC-2026-0194 and RUSTSEC-2026-0195 (`quick-xml`) | Existing Wayland stack. | project lead | 2026-11-25 |
| RUSTSEC-2026-0204 (`crossbeam-epoch`) | Existing concurrency dependency. | project lead | 2026-11-25 |
| RUSTSEC-2021-0145 (`atty`) | Transitive through `structopt`; migrate the headless CLI. | project lead | 2026-11-25 |
| RUSTSEC-2026-0186 (`memmap2`) | Existing Wayland stack. | project lead | 2026-11-25 |
| RUSTSEC-2026-0097 (`rand`) | Simulation test support and existing UI stack. | project lead | 2026-11-25 |
| RUSTSEC-2025-0055 (`tracing-subscriber`) | Existing profiling stack. | project lead | 2026-11-25 |
| `bytemuck@1.16.1` yanked | Locked UI and renderer graph selection. | project lead | 2026-11-25 |
| `bytes@1.6.0` yanked | Locked platform graph selection. | project lead | 2026-11-25 |
| `epaint@0.27.2` `LicenseRef-UFL-1.0` | Bundled default fonts; upstream allows the same identifier. | project lead | 2026-11-25 |

Remove an exception when its dependency update lands. Before the review date,
the owner must renew it with a new reason and date or remove the affected
dependency. Asset provenance and Git revision pinning are out of scope for this
policy baseline; `Uriopass/yakui` is consumed on branch `dev`, which is a
reproducibility finding tracked separately, not an exception granted here.

## Proof that the check fails

A guard nobody has watched fail proves nothing. Both mutations below were
applied to `deny.toml`, run, and reverted on 2026-08-27. `deny.toml` was
restored byte-for-byte afterwards, verified by SHA-256.

**Mutation A — remove one advisory ignore entry** (`RUSTSEC-2022-0104`):

```
error[unmaintained]: `structopt` is in maintenance mode
    ┌─ /home/caio/soviet-simulator/Cargo.lock:293:1
    │
293 │ structopt 0.3.26 registry+https://github.com/rust-lang/crates.io-index
    │ ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ unmaintained advisory detected
    ├ ID: RUSTSEC-2022-0104
    ├ structopt v0.3.26
      └── headless v0.1.0

advisories FAILED, bans ok, licenses ok, sources ok
```

Exit code 1.

**Mutation B — remove `https://github.com/Uriopass/yakui` from `sources.allow-git`:**

```
error[source-not-allowed]: detected 'git' source not explicitly allowed
    ┌─ /home/caio/soviet-simulator/Cargo.lock:402:13
    │
402 │ yakui 0.2.0 git+https://github.com/Uriopass/yakui?branch=dev
    │             ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ source

advisories ok, bans ok, licenses ok, sources FAILED
```

Exit code 8, five `error[source-not-allowed]` findings (`yakui`, `yakui-core`,
`yakui-wgpu`, `yakui-widgets`, `yakui-winit`).

**After restoring `deny.toml`:**

```
advisories ok, bans ok, licenses ok, sources ok
```

Exit code 0, zero errors, 24 duplicate warnings.

Mutation B is the mutation to use when checking that CI still gates. It is
derived only from `Cargo.lock`, so it produces the identical finding on any
machine, whereas Mutation A depends on the advisory database snapshot.

## CI enforcement

`.github/workflows/dependency-policy.yml` is the enforcement job. One workflow,
one job, four steps, 28 non-comment lines. It checks out the repository,
installs `cargo-deny` 0.20.2 with `--locked`, asserts the running binary really
is 0.20.2, and runs `cargo-deny check`. It contains no `continue-on-error` and
no `|| true`. It adds no build, test, packaging or release step.

`actions/checkout` is pinned to commit
`11d5960a326750d5838078e36cf38b85af677262` (tag `v4.4.0`), for the same reason
the checker version is pinned: a moving tag is not a reproducible input.

### What is proven, and what is not

Verified locally on 2026-08-27:

- `cargo-deny check` exits 0 on the clean tree and prints
  `advisories ok, bans ok, licenses ok, sources ok`.
- `cargo-deny check` exits non-zero under a policy mutation (1 for the advisory
  mutation, 8 for the sources mutation), and prints which check failed.
- The version assertion passes for 0.20.2 and fails for any other pin.
- Running the two `run:` step bodies in sequence under GitHub's default Linux
  shell for `run:` steps, `bash -e -o pipefail`, gives exit 0 on the clean tree,
  exit 8 with the sources mutation applied, and exit 0 again after restoring
  `deny.toml`.

**Not verified, and not claimed.** The workflow has never executed on GitHub
Actions. `act` and Docker are unavailable on the development machine, and the
change is not pushed. These remain open until the first real run:

- that the job runs at all, and that `cargo install cargo-deny --version 0.20.2
  --locked` succeeds on `ubuntu-latest` with its preinstalled toolchain;
- that a red step actually renders as a failed check on the pull request;
- that the CI finding text matches the local finding text for the same
  mutation. Use Mutation B for that comparison: `sources` is derived only from
  `Cargo.lock`, so it is the one check whose output must be identical on both
  machines.

The remote does allow the job to run: `repos/.../actions/permissions` reports
`{"enabled":true,"allowed_actions":"all"}`, and the repository had zero
workflows before this one.

### After the first real run

Compare the CI log for `cargo-deny check` against the local output recorded
above. Then apply Mutation B on a branch, confirm the job goes red with the
same five `error[source-not-allowed]` findings, and revert. Record the run URL
here. Until that is done, treat this workflow as installed but unproven.
