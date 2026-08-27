# Mutation-test policy

**Kind:** process policy
**Authority:** operational
**Status:** active — **no tool is mandatory under this policy**
**Owner:** project lead
**Last verified:** 2026-08-27

This project's standing rule is that **a guard nobody has seen fail proves nothing**. Phase 3
enforces that by hand: `evidence-auditor` breaks the production behaviour a test claims to guard,
watches the test go red, pastes the real output, and reverts. Mutation testing is the mechanical
form of the same rule — it does the breaking automatically, over every mutable expression in a
file, and reports which ones no test noticed.

Mechanical does not mean free, and it does not mean better. This document decides **where that
mechanical evidence is worth its cost and where it is not**, so it never becomes a scan everybody
learns to scroll past.

Every number in this document was measured on 2026-08-27 in an isolated worktree at commit
`2550026`, and the command that produced it is shown. Nothing here is estimated.

---

## What this policy is not

- It is **not** a mandate. No agent and no CI job is required to run a mutation scan to land a
  change. Adoption is decided per trial, and the first trial is recorded in `sov-mwy`.
- It is **not** a replacement for any Phase 4 gate. See
  [What mutation evidence does not replace](#what-mutation-evidence-does-not-replace).
- It is **not** a full-repo scan on every change. That is measured below as roughly a day of
  machine time and is forbidden as a per-change practice.

---

## Where this sits in the cycle

`docs/process/development-cycle.md` already places the evidence work:

| Phase | Who | Asks |
|---|---|---|
| 3 PROVE | `evidence-auditor` | Has every new guard been seen failing? |
| 4 gate 1 | `wiring-auditor` | Is the new code reachable from the running game? |
| 4 gate 2 | `ledger-invariant-checker` | Is quantity conserved across this economic seam? |
| 4 gate 3 | `reviewer` | General adversarial gate |
| 4 gate 4 | domain advisor | Is the mechanic model-consistent? |

**Mutation evidence belongs inside Phase 3, and nowhere else.** It is one more input to
`evidence-auditor`'s existing judgement, run at that agent's discretion when the change falls in an
eligible class below. It never gates on its own, it never runs after Phase 4 has passed, and its
output is never forwarded to a Phase 4 gate as a substitute for that gate's own question.

Rationale: a surviving mutant is a statement about **test sensitivity**. Every Phase 4 gate asks a
different question — reachability, conservation, correctness, model fit — and none of them is
answered by test sensitivity. A file can have a 100% mutant kill rate and still be dead code that
nothing calls, still leak units across a seam its tests never construct, and still implement a
mechanic the Kornai model forbids.

---

## Eligible changes

A change is eligible for mutation evidence when it touches one of the four classes below. The file
globs are the exact `--file` arguments to use; each was verified to resolve to real files with
`--list-files` on 2026-08-27.

### 1. Economy — the ledger core

```
-f 'simulation/src/economy/**.rs'
```

Resolves to `economy/mod.rs`, `ecostats.rs`, `government.rs`, `market.rs`. Eligible because units
and money are conserved here or they are not, and a silent arithmetic change is exactly the defect
class a surviving mutant names.

### 2. Logistics — physical movement

```
-f 'simulation/src/map_dynamic/**.rs' -f 'simulation/src/transportation/**.rs'
```

Resolves to 13 files including `dispatch.rs`, `itinerary.rs`, `router.rs`, `parking.rs`, `road.rs`,
`train.rs`, `vehicle.rs`. Eligible because of the **nothing teleports** pillar: a state-machine
guard that no test can see failing is how a good crosses a map without a vehicle.

### 3. Persistence — save and load

```
-p common -f 'common/src/saveload.rs' -f 'common/src/hash.rs'
```

Eligible for a reason specific to this codebase, described in the harness section below.

### 4. Determinism — tick ordering and RNG

```
-f 'simulation/src/utils/**.rs'
```

Resolves to `scheduler.rs`, `rand_provider.rs`, `par_command_buffer.rs`, `replay.rs`,
`resources.rs`, `mod.rs`.

### Not eligible

Presentation (`native_app/`), rendering (`engine*/`), map geometry, and Lua data. Not because they
matter less, but because their behaviour is judged from frames and from the running game, not from
`cargo test`, so a surviving mutant there carries no information about a guard that was ever
supposed to exist.

---

## The tool

Pinned and installed **outside this repository**, following the pattern in
`docs/process/dependency-policy.md`. It is never added to any `Cargo.toml` and never changes
`Cargo.lock`.

```sh
cargo install cargo-mutants --version 27.1.0 --locked
```

The version is pinned and the install is `--locked`, so the tool's own dependency graph is
reproducible. Then invoke it as a cargo subcommand:

```sh
cargo mutants [OPTIONS]
```

The evidence in this document was produced with the same 27.1.0 binary installed to a
session-local root and invoked by absolute path:

```sh
cargo install cargo-mutants --version 27.1.0 --locked --root /tmp/sov-tools
/tmp/sov-tools/bin/cargo-mutants mutants [OPTIONS]
```

Only the invocation path differs; the binary, version, sub-command and arguments are the same.
`/tmp` is ephemeral — do not treat that path as durable. This mirrors the arrangement in
`docs/process/dependency-policy.md`.

**Version and licence, verified 2026-08-27:**

| | |
|---|---|
| Version | `cargo-mutants 27.1.0` (from `--version`) |
| Licence | **MIT** — `license = "MIT"` in the published manifest, and the shipped `LICENSE` file reads `MIT License / Copyright (c) 2021 Martin Pool` |
| Repository | https://github.com/sourcefrog/cargo-mutants |

MIT places no obligation on this GPL-3.0 repository. The tool is a build-time analysis binary; it
is never linked into a shipped artifact and its source is never vendored here. This is a technical
dependency note, not legal advice.

---

## The scoped commands

Every command below is run **from a dedicated git worktree**, never from the main checkout. See
[Worktree discipline](#worktree-discipline-and-restoration).

### Setup, once per run

```sh
git worktree add /tmp/sov-mut-<ticket> HEAD --detach
cd /tmp/sov-mut-<ticket>
cargo test -p simulation                 # baseline must be green before anything else
```

### A. Diff-scoped run — the default, and the only one appropriate per change

```sh
git diff origin/main... > /tmp/sov-mut-<ticket>/change.diff
/tmp/sov-tools/bin/cargo-mutants mutants \
    -p simulation \
    --in-diff /tmp/sov-mut-<ticket>/change.diff \
    -t 90 --build-timeout 300 \
    -j 4 \
    -o /tmp/sov-mut-<ticket>/out \
    -v -V
```

`--in-diff` restricts mutants to lines the change actually touched. **Time limit: 30 minutes
wall-clock.** If it has not finished by then, stop it and record the result as partial — a partial
run with an honest count is evidence; a run left going all afternoon is not.

### B. File-scoped run — for a change that rewrites one high-risk file

```sh
/tmp/sov-tools/bin/cargo-mutants mutants \
    -p simulation \
    -f 'simulation/src/economy/market.rs' \
    -t 90 --build-timeout 300 \
    -j 4 \
    -o /tmp/sov-mut-<ticket>/out \
    -v -V
```

**Time limit: 2 hours wall-clock.** Substitute the eligible-class glob from the table above.

### C. Class-scoped run — periodic only, never per change

Substitute one eligible class's globs into command B. **Time limit: 6 hours wall-clock.** Use
`--shard i/k` to split across sessions rather than raising the limit.

### The flags, and why each is there

| Flag | Why |
|---|---|
| `-p simulation` | Confines mutant generation to one package. Without it, `--workspace` behaviour costs hours. |
| `-t 90` | Per-mutant test timeout in seconds. Measured baseline is 12.6 s, so 90 s is roughly 7× headroom — generous enough that a slow machine does not manufacture false timeouts. |
| `--build-timeout 300` | A mutant that makes the crate build pathologically slowly is stopped rather than left. |
| `-j 4` | Parallel jobs. Safe: the `static mut` race in `init.rs`/`prototypes` was fixed 2026-08-26 (`sov-test-race-initfuncs-qt6`). |
| `-o <dir>` | Writes `mutants.out/` **outside the repository**. Never let it land in the working tree. |
| `-v` | Prints caught mutants too. A run that only prints survivors cannot prove it killed anything. |
| `-V` | Prints unviable mutants — the ones that failed to compile. They are not evidence either way and must be counted separately. |

**`-v` is not optional.** A report listing only survivors is indistinguishable from a run whose
tests never executed. At least one **killed** mutant must be pasted in every report, for exactly
the reason `evidence-auditor` exists: an unfalsified check proves nothing about the checker.

### The vacuous-run trap — read this before reporting any diff-scoped result

A diff-scoped run whose diff touches no Rust source generates **zero mutants and exits without
error**. Measured 2026-08-27 in an isolated worktree:

```
$ /tmp/sov-tools/bin/cargo-mutants mutants --list -p simulation --in-diff /tmp/probe.diff
 INFO Diff changes no Rust source files
```

That is the same shape as `cargo test -p simulation sentinel` printing `running 0 tests ... ok` —
a green result whose subject does not exist. **A run that produced zero mutants is NOT RUN.** Report
it as `not run — 0 mutants generated`, never as passing, and never count it toward the evidence for
a change.

The check is one line, and it is mandatory before any diff-scoped result is reported:

```sh
/tmp/sov-tools/bin/cargo-mutants mutants --list -p simulation --in-diff <diff> | wc -l
```

Zero means stop. The same rule applies to a `-f` glob that matches no file: `--list-files` printing
nothing means the glob is wrong, not that the code is clean.

### Before the run: check the size

```sh
/tmp/sov-tools/bin/cargo-mutants mutants --list -p simulation -f '<glob>' | wc -l
```

This needs no build and takes seconds. Multiply the count by roughly 30 s and compare against the
limit above **before** starting. If the estimate exceeds the limit, narrow the glob or shard it.
Do not start a run you already know will be cut off.

---

## What a TIMEOUT means

**A timeout is INCONCLUSIVE. It is not a defect, it is not a survivor, and it must never be
reported as either.**

`cargo-mutants` reports a mutant as `TIMEOUT` when the mutated test run exceeded `-t`. The most
common cause in this codebase is benign: the mutation removed a loop bound or a termination
condition, so the simulation ran forever. That tells you the mutant was *not* caught within the
budget. It does not tell you the mutant would have survived a longer budget, and it says nothing at
all about whether the production code is correct.

Rules:

1. Timeouts are counted and reported in **their own column**, never folded into survivors and never
   folded into kills.
2. A report that says "N survivors" while N includes timeouts is a defective report. Send it back.
3. Each timeout gets a disposition like any survivor (see below). The usual correct disposition is
   `inconclusive — re-run at higher -t`, and that is a complete and acceptable answer.
4. **Never** raise `-t` merely to convert a timeout into a kill and then report the kill. If a
   higher budget is used, the budget is stated with the result.
5. A timeout is never grounds for opening a bug ticket on the production code.

The same applies to **unviable** mutants (`-V`): a mutant that does not compile was never tested.
It is neither killed nor survived. Count it separately and move on.

---

## Survivor disposition — every survivor gets an explicit decision

**Silence is not a disposition.** A run that lists 40 survivors and says nothing about them has
produced zero evidence and cost hours. Every surviving mutant gets exactly one of these four
labels, written down with a one-line reason:

| Disposition | Meaning | What happens next |
|---|---|---|
| **REAL GAP** | The mutant changes behaviour the code is supposed to guarantee, and no test noticed. | Write the missing assertion, or file a `bd` ticket naming it. This is the finding worth the whole run. |
| **EQUIVALENT** | The mutation does not change observable behaviour. Common for `Default` impls, `Display`, getters returning the same value under all reachable inputs. | Record the argument for equivalence in one line. No action. |
| **OUT OF CONTRACT** | The mutated behaviour is real but nothing has ever promised it — an unused accessor, a debug path, a code path no story covers. | No test is owed. Consider whether the code is reachable at all, and if not, say so to `wiring-auditor`. |
| **ACCEPTED** | A real gap that is not worth closing now. | Must name a reason and a `bd` ticket id. An accepted survivor without a ticket is silence wearing a label. |

The kill *rate* is not a target and must not be quoted as a score. Chasing a percentage produces
assertions written to kill mutants rather than to protect behaviour — the tautology shape Phase 3
already exists to catch. **The unit of value is one REAL GAP with a named missing assertion**, not
a ratio.

A run that ends with every survivor labelled EQUIVALENT or OUT OF CONTRACT is a good outcome and a
finished job. Report it that way.

---

## Periodic broad-run cadence

| Run | Scope | Cadence | Limit |
|---|---|---|---|
| Diff-scoped (A) | The change's own lines | At `evidence-auditor`'s discretion, on an eligible change | 30 min |
| File-scoped (B) | One high-risk file | When a change rewrites that file | 2 h |
| Class-scoped (C) | One eligible class | **One class per iteration wrap (Phase 6), rotating** — economy, then logistics, then persistence, then determinism | 6 h |

The rotation is deliberate. Running all four classes every iteration is the theatre this policy
exists to prevent: it costs most of a day, produces the same survivor list each time, and gets
skimmed. One class per wrap means each class is examined roughly every four iterations, and each
examination gets read.

**Full-workspace scans are not scheduled.** Measured: `--list -p simulation` alone generates
**3,036** mutants; at roughly 30 s each that is about 25 hours for one package. There is no cadence
at which that is worth its cost, and it is explicitly forbidden as a per-change practice.

The rotation resets whenever the `-f` globs above change, and whoever changes them re-runs
`--list-files` and updates the counts in this document.

---

## Worktree discipline and restoration

**Mutation testing edits source files. It must never run in the shared checkout.**

`cargo-mutants` copies the tree to a scratch directory by default, which is safe. But `--in-place`
edits the tree directly, and any interrupted run — Ctrl-C, OOM, a killed session — can leave a
mutated file behind. In a shared checkout that mutation would be invisible to everyone and would
poison every other agent's test run.

Rules:

1. **Always run from a dedicated worktree**: `git worktree add /tmp/sov-mut-<ticket> HEAD --detach`.
   Never from `/home/caio/soviet-simulator`.
2. **Never use `--in-place` in a worktree that is not disposable.**
3. **`-o` points outside any git tree.** `mutants.out/` is a build artifact and is never committed.
4. **Verify restoration before reporting.** The check is one command and its output must be pasted
   in the report:

   ```sh
   cd /tmp/sov-mut-<ticket> && git status --porcelain
   ```

   Empty output means clean. Anything else means a mutation survived the run and must be reverted
   with `git checkout -- .` before any result is believed.
5. **Remove the worktree when done**, and prove it:

   ```sh
   git worktree remove /tmp/sov-mut-<ticket> --force
   git worktree list
   ```

   A worktree left behind is a stale checkout another agent will eventually mistake for live code.
   This has already happened once in this repo (`/tmp/sov-f1v-worktree`, retired 2026-08-27).
6. **The main `target/` is shared and cargo locks it.** A worktree gets its own `target/`, which
   means a full cold rebuild — measured at 41 s here, but that is with a warm cargo registry.
   Budget for it; it is the price of isolation, not a fault.

---

## What mutation evidence does not replace

**Stated explicitly, because a green mutation report is the most persuasive-looking artifact this
project can produce, and it answers none of the following questions:**

| Gate | Its question | Why a mutation run does not answer it |
|---|---|---|
| `wiring-auditor` (Phase 4.1) | Is this reachable from the running game? | Mutants are killed by *tests*. Code called only from `cargo test` can score a perfect kill rate while nothing in the game ever calls it. A high kill rate on unwired code is the exact false comfort this gate exists to strip. |
| `ledger-invariant-checker` (Phase 4.2) | Is quantity conserved across this seam? | A mutant is killed if *any* existing test fails. If no test ever constructs the sequence that leaks units, every mutant on that path can be killed by unrelated tests while the leak stands. Conservation is proved by an invariant assertion, not by test sensitivity. |
| `reviewer` (Phase 4.3) | Is this correct, and is it the right change? | Mutation says nothing about design, naming, duplication, or whether the change was worth making. |
| domain advisor (Phase 4.4) | Is the mechanic model-consistent? | Mutation cannot know that clearing by price is forbidden, or that households are shared-pantry units. It has no model. |
| `evidence-auditor` (Phase 3, by hand) | Does this assertion prove the story it cites? | This is the gap mutation is worst at. A test can assert something **weaker than its story claims** and still kill every mutant in the file. Mutation finds unguarded code; it cannot find a guard pointed at the wrong claim. |

**Mutation evidence is additional evidence, never a substitute.** No gate may be skipped, shortened
or deferred because a mutation run was green. A mutation report is not a reviewable artifact on its
own — it is an attachment to `evidence-auditor`'s Phase 3 findings.

### One codebase-specific trap

`TestCtx::tick()` in `simulation/src/tests/mod.rs` bincode-round-trips the whole `Simulation` and
hash-compares the result. That check proves **serialize/deserialize round-trips**. It cannot detect
a simulation desync, because there is only ever one run; and it is blind to any field omitted from
a `Serialize` derive — such a field is neither saved nor hashed, and the comparison still matches.

Mutation testing does not fix that blindness and can disguise it: mutants inside a field that is
never serialized will be reported normally, and the round-trip check will never be the thing that
kills them. This is precisely why persistence is an eligible class — mutants in
`common/src/saveload.rs` that survive are a direct measurement of how much that harness is *not*
checking.

**Never weaken `TestCtx::tick()`'s determinism check to make a mutant die.**

---

## Measured cost, 2026-08-27

Commit `2550026`, isolated worktree, `cargo-mutants 27.1.0`.

| Measurement | Command | Result |
|---|---|---|
| Baseline test suite | `cargo test -p simulation` | 45 passed, 0 failed, **12.60 s** (12.9 s wall) |
| Cold worktree build | `cargo test -p simulation --no-run` | **41.11 s** |
| Mutants in `market.rs` | `--list -p simulation -f 'simulation/src/economy/market.rs'` | **163** |
| Mutants in economy class | `--list -p simulation -f 'simulation/src/economy/**.rs'` | **230** |
| Mutants in logistics class | `--list -p simulation -f '…/map_dynamic/**.rs' -f '…/transportation/**.rs'` | **677** |
| Mutants in determinism class | `--list -p simulation -f 'simulation/src/utils/**.rs'` | **198** |
| Mutants in persistence class | `--list -p common -f 'common/src/saveload.rs' -f 'common/src/hash.rs'` | **63** |
| Mutants in whole `simulation` package | `--list -p simulation` | **3,036** |

These counts are what make the cadence table above a decision rather than a preference: the
per-change scope is tens of mutants, the per-iteration scope is hundreds, and the full package is
thousands. Re-run the `--list` commands after any large refactor; they cost seconds.

---

## Status and adoption

**No tool is mandatory under this policy.** `cargo-mutants` is not installed by any project script,
is not in CI, is not in any `Cargo.toml`, and no gate requires it.

Adoption is decided by trial, one module at a time. The first trial is `sov-mwy`
(`simulation/src/economy/market.rs`). A trial concludes with an explicit **adopt** or **remove**
decision recorded on its `bd` ticket. Only after a trial adopts does any command here become part
of a routine, and even then it stays inside Phase 3 and stays non-blocking.
