---
name: review-method-this-repo
description: What works when reviewing this repo — LSP health-check first, gate commands and their true expected numbers, scope boundaries per track
metadata:
  type: project
---

# Review method that works in soviet-simulator

**Why:** proving a negative ("nothing else changed") needs the right instrument; grep
cannot do it and a green suite is not self-evidently meaningful.

**How to apply:** run these at the top of every phase review.

## Health-check LSP before trusting an empty result

`findReferences` returning "No references found" is ambiguous — it means either *no callers*
or *cold index*. Always confirm the server is live by first running `findReferences` on a
symbol you know has callers.

**rust-analyzer takes ~45s to index this project from cold.** At Phase 2 the first call
returned "No references found" and the second returned an explicit
`server is starting` error — so the *first* answer was a false negative. Sleep 45s, then
re-run the canary, before trusting anything.

Canary: `BuildingKind::footprint` in `src/sim/buildings.rs`. Reference counts by phase:

| function | Phase 1 | Phase 2 |
|---|---|---|
| `footprint` | 11 / 7 files | 10 / 6 files |
| `inventory_capacity` | 13 / 11 | 12 / 10 |
| `workers_needed` | 9 / 4 | 8 / 3 |
| `default_policies` | 21 / 10 | 21 / 10 |

The Phase 2 drop of exactly −1 on the first three is the three equivalence tests being
re-pointed at `PINNED_COLUMNS`; `default_policies` is flat because its test still calls the
live function. **Reference-count deltas that decompose cleanly like this are good evidence
that no production call site moved** — better than reading the diff.

## Beware: bare `grep` is hook-rewritten to rtk, piped `grep` is not

A symbol-multiset comparison between `git show HEAD:path | grep …` and `grep … path` produced
a phantom "symbol added" at Phase 2. The two sides had gone through different tools. Run both
sides through the same path — `git show HEAD:path > /tmp/old` then `rtk proxy bash -c '…'`
over both files — before believing a structural difference.

## Budget the FIRST build; every one after is ~6s

rust-analyzer's initial index and `cargo` contend for the same build lock. At Phase 3 the
**first** `cargo test --lib sim::` of the session blew a 300s Bash timeout and had to be
recovered from the background task file; the next one took **5.8s wall**. So: kick off the
first build in the background, or raise the timeout to 500s+, and do not restructure a
mutation plan around "builds are slow" — they are not, after the first.

**Danger this creates:** a timed-out foreground command may not have reached its `cp`
restore step. Always chain mutate/run/restore in ONE command so the restore runs even on a
timeout path, and re-check `md5sum -c` after every single mutation, not just at the end.

## Scoped gate: `cargo test --lib sim::`

`126 passed; 0 failed; 22 filtered out` as of Phase 3 (124 pre + the band pin + the
discriminant pin). Use this, not whole-lib, whenever the `game/` track is in flight —
whole-lib is 148 and includes another agent's uncommitted `src/game/art.rs`.

## Gate commands and true numbers

- `cargo test --lib` — 138 green as of Phase 1 (129 pre-refactor + 9 catalogue tests).
  Fast, ~0.5s. Run it yourself; never accept a pasted result.
- `cargo clippy --lib` — exactly **4** warnings: `game/juice.rs:125`,
  `game/vehicles.rs:436`, `sim/dispatch.rs:179`, `sim/households.rs:129`.
- `cargo clippy --lib --tests` — surfaces **5 more** (`wires.rs:348`, `water.rs:175`,
  `network.rs:116/117/132`), all pre-existing test-profile lints. Reports sometimes say
  "the four warnings" while running `--tests`; that phrasing is imprecise but not wrong in
  substance. Check *which files* the warnings name, not the count.
- `cargo fmt --check` — clean, exit 0.
- `cargo build --lib` — clean.

## Scope boundaries

Each implementer track is `src/sim/` **or** `src/game/`, never both, never `src/bin/`,
never an ADR. `git status --porcelain` at the top of the review, and again at the end —
files can appear mid-review from concurrent teammates.

At Phase 1, `docs/adr/0017-a-building-is-its-product.md` appeared during the review and was
**not** a violation: it belongs to the research track, cited from
`.planning/…/findings.md:30`. Check `findings.md` for attribution before calling an ADR a
scope breach.

## Uncommitted-change reviews

Phases here are reviewed **before commit**, so `HEAD` predates the work. The diff is
`git diff <tracked files>` plus the untracked new files from `git status`. Do not assume
`git show HEAD~1:path` gives the pre-change state of a new file — there isn't one.

## Reviewing while other reviewers mutate the same tree (learned Phase 5, 2026-08-18)

Five blind reviewers gated Phase 3+5 simultaneously and two of them had licence to mutate.
Consequences to plan for:

- **A suite result is meaningless without an md5 of the changed files taken in the same
  command.** `cargo test --lib` gave me `147 passed; 1 failed` twice, on
  `default_policies_column_holds_its_pinned_bands` (`Quarry / Gravel left: Some((0.0, 0.5))`)
  — another reviewer's live mutation of `catalogue.rs`, not a defect. I identified it as theirs
  by finding the mutated line in a sibling background-task output file under
  `…/tasks/*.output`. Always run `md5sum <changed files>` in the *same* bash call as the test,
  and record the pristine md5s at the top of the review.
- **Read the file you are reviewing once, early, in full, and keep the transcript as your
  reference.** `src/game/art.rs` changed under me three times. My opening `cat -n` was the only
  stable record; I cross-checked it against the mutating reviewer's own `.orig` backup in the
  shared scratchpad and they agreed, which is what let me trust it.
- The shared scratchpad path is *not* per-agent. Other reviewers' `.orig` backups appear in it.
  Read them, never write over them; put your own copies in a subdirectory.
- **~13 concurrent `cargo` processes serialise on `target/debug/.cargo-lock`.** A `cargo test
  --lib` that normally takes 0.5s took over 4 minutes and blew a 120s Bash timeout. Budget a
  400–600s timeout, or run scoped (`cargo test --lib -- game::art game::toolbar`).
- Ending state to assert in the verdict: `git diff --stat <baseline>` identical to the opening
  snapshot **and** every untracked file's md5 back to pristine. `git status --porcelain` alone
  will not show a mutated untracked file.

## A red suite in a parallel-review gate is probably another reviewer, not the diff

Learned at Phase 3 (2026-08-18), when five blind reviewers worked one dirty tree at once.
My first `cargo test --lib` came back `147 passed; 1 failed` —
`default_policies_column_holds_its_pinned_bands`, `Quarry / Gravel left: Some((0.0, 0.5))`.
That is *exactly* the corruption recorded in [[vacuous-checks-data-driven-buildings]] §2: a
mutation reviewer was mid-cycle on `catalogue.rs`. `ls --time-style` showed its mtime 13s
before my run and the live literal was already back to `0.05`; the re-run was 148 green.

**Protocol: fence every gate run with `md5sum` of the files you care about, before and after,
and print `date` + file mtimes if anything is red.** A failure whose message matches a
mutation recorded in memory is a concurrency artefact — confirm before reporting it. Do not
"fix" it and never restore with git.

## Read-only reviewers can still run a full oracle — do it out of tree

`rustc --edition 2021` on a standalone `/tmp` file holding the old implementation (from
`git show 7c3b4ce:`) beside the new one gives full differential-oracle strength with zero
write risk to a tree full of other tracks' untracked files. Mutate the *standalone copy* to
prove the oracle non-vacuous. Faster than a mutate-and-restore cycle in the tree, and there
is nothing to restore. See [[save-wire-format-baseline]].

## Comparing halves of a file beats reading a diff for "nothing else moved"

`awk '/^mod tests|^#\[cfg\(test\)\]/{exit} {print}'` over both versions, then `diff` the two
production halves with **no filters**. At Phase 3 this reduced "did anything in snapshot/
restore move?" to a 30-line diff containing only the two intended functions. Much stronger
than eyeballing `git diff` hunk headers, and it makes "all hunks are inside `mod tests`"
checkable rather than assumed.

## LSP reference counts — `BuildingKind::ALL`

| symbol | 7c3b4ce | after phases 3+5 |
|---|---|---|
| `BuildingKind::ALL` | 9 / 1 file (catalogue.rs only) | 21 / 4 files |
| `kind_to_u8` | 2 (def + 1 prod caller) | 3 (def + 1 prod + 1 test) |
| `kind_from_u8` | 2 (def + 1 prod caller) | 5 (def + 1 prod + 3 test) |

The `ALL` jump is the load-bearing one: its order stopped being catalogue-private.

## Commit-readiness checks that are cheap and keep paying (added Phase 3+5, 2026-08-18)

Run these whenever a phase adds a file or the tree is dirty:

- `git cat-file -e :<new file>` — is it in the **index**, not just on disk? A phase that adds
  `foo.rs` and a tracked `pub mod foo;` produces a `git commit -am` that does not compile.
- `git diff <baseline> --numstat` vs `git diff <baseline> -w --numstat` — identical rows means
  zero whitespace-only changed lines, i.e. no formatter collateral. This is the fast
  discriminator; `--stat` works too.
- `git show <baseline>:f > /tmp/f && rustfmt --edition 2024 --check /tmp/f` — if the *old* file
  was already canonical, a bare `cargo fmt` could not have reflowed it. Note: a copied-out
  `mod.rs` always errors with "failed to resolve mod X"; that is the copy, not the file.
- `git diff --name-only <baseline> | grep -v '^src/'` — should be empty. Catches ADR/README/
  Cargo.toml scope breaches in one line.
- `#[test]` attribute count over `src/` minus `src/bin/` equals the `cargo test --lib` count in
  this crate (138 at 7c3b4ce = 138 tests), so it is a valid per-file proxy for attributing an
  added-test delta to a track without building anything.
- `ls .git/hooks | grep -v sample` — no active hooks here, so nothing reformats on commit.

## Clippy line numbers move; check the lint text, not the line

`game/vehicles.rs` was `:436` at Phase 1 and `:423` at Phase 3+5 — the file lost 13 net lines.
Same `very complex type` lint, byte-identical source. Before calling a warning new, diff the
lint *text* and the source line against `git show <baseline>:`.
