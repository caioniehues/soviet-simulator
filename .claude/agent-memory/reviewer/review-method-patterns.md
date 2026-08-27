---
name: review-method-patterns
description: Reusable review patterns from the data-driven-buildings refactor — LSP warmup, out-of-tree oracles, mutation testing protocol, concurrent-review fencing
metadata:
  type: reference
---

Distilled from the refactor-reviewer's 6-phase campaign (2026-08-17 to 2026-08-19,
data-driven buildings on the discarded Bevy track). The specific file paths and test
counts are gone, but the shapes recur.

## LSP warmup — always canary first

`findReferences` returning "No references found" is ambiguous: it can mean cold index.
Confirm the server is live by running `findReferences` on a symbol you KNOW has callers.
rust-analyzer takes ~45s on this project. Sleep, then re-run the canary before trusting
anything.

## Mutation testing is cheap here and works

`cargo test --lib` runs in ~0.5s. Back up all touched files (`cp` + `md5sum`), plant one
error per field group, run, confirm each names the right kind, restore, re-verify md5
**and** `git diff --numstat` against the pre-review counts.

Chain mutate/run/restore in ONE command so the restore runs even on a timeout. Re-check
`md5sum -c` after every single mutation, not just at the end.

## Out-of-tree oracle — strongest instrument found

To prove a deleted match was faithfully replaced: `rustc --edition 2021` on a standalone
`/tmp` file holding the old implementation (from `git show <baseline>:`) beside the new
one gives full differential-oracle strength with zero write risk. Mutate the *standalone
copy* to prove the oracle non-vacuous. Faster than a mutate-and-restore cycle in the
tree.

## Concurrent-review fencing

When multiple reviewers mutate the same tree simultaneously:
- A suite result is meaningless without an md5 of the changed files taken in the SAME
  command.
- Read the file you are reviewing once, early, in full, and keep the transcript as your
  reference.
- Ending state to assert: `git diff --stat <baseline>` identical to opening AND every
  untracked file's md5 back to pristine.

## Every phase that deletes a per-kind match turns its own equivalence test into a tautology

A test `spec(kind).X == kind.X()` goes circular the moment `kind.X()` is made to read
`spec(kind).X`. **Never assess a test by its own diff.** Assess it by what its two sides
resolve to *after* the change. The cheap detector: corrupt a table row and run the suite.

## A pin that reads private struct fields cannot witness the accessor

`Surface::mat()` was the accessor; the pin compared private fields. Deleting
`.shade(self.shade)` from the accessor left all tests green. When a table column is read
via a method, a pin must also exercise the method.

## Approximate counts, reliable reasoning

Pattern confirmed 3× on this implementer: line numbers and counts drift; their reasoning
and mutation proofs are reliable. Verify citations, trust arguments.
