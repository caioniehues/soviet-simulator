---
name: audit-worktree-may-be-clean
description: A brief's "audit this worktree" can point at a clean tree; the diff may still be uncommitted in the main checkout. Check `git diff --stat` in the worktree first.
metadata:
  type: feedback
---

Before auditing, run `git status --porcelain && git diff --stat` inside the named worktree AND
`git worktree list`. If the worktree is clean and at the same SHA as main, the diff was never
carried over: look for it as uncommitted state in `/home/caio/soviet-simulator` (`git diff -- simulation/`).

**Why:** 2026-09-02, sov-ahw: the lead's comment said "split off main's dirty tree into
/home/caio/sov-ahw-wt", but that worktree was clean at 4e9e930. The six-file diff (market.rs,
mod.rs, four scenario tests) existed only in main's working tree, dated 2026-08-28. Auditing the
worktree as instructed would have produced a report on zero changes.

**How to apply:** verify the scope exists where the brief says before reading a line; if not,
snapshot the real diff (`git diff -- <paths> > scratchpad/x.diff`) so evidence is pinned, say so
in the first line of the report, and tell the lead later gates need repointing. See also
[[Hazard: OTHER AGENTS EDIT THE AUDIT WORKTREE WHILE YOU AUDIT IT]] in MEMORY.md.
