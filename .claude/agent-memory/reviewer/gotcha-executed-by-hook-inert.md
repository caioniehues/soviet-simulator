---
name: gotcha-executed-by-hook-inert
description: The Executed-By trailer and BEADS_ACTOR convention are DELETED (2026-08-27); bd 1.2.2's hook was inert; attribution is now `--author` on bd comments
metadata:
  type: project
---

**RESOLVED 2026-08-27.** The `BEADS_ACTOR`/`Executed-By` convention was DELETED from
CLAUDE.md, task-tracking.md, and agent definitions. Attribution is now `--author` on
`bd comments add`.

**Background (verified 2026-08-26):** bd 1.2.2's `prepare-commit-msg` hook was a no-op
even with `BEADS_ACTOR` set. Direct invocation left the commit message byte-identical.
0 of the last 60 commits carried the trailer. `BEADS_ACTOR` was not in the binary's
env strings. Same class of gap as work leases / events journal / HTTP API in the 1.2.2
recovery re-release.

**How to apply:** never set `BEADS_ACTOR` in briefs. Use `--author <name>` on bd comment
commands. Do not cite the hook as evidence of anything.

See [[bd-close-reason-exceptions]].
