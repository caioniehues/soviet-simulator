---
name: gotcha-bd-executed-by-inert
description: The BEADS_ACTOR/Executed-By convention was inert in bd 1.2.2 and got DELETED 2026-08-27 from CLAUDE.md, task-tracking.md, team-lead initialPrompt and debugger.md — attribution is `--author` on comments
metadata:
  type: project
---

bd 1.2.2's `prepare-commit-msg` hook is inert: direct invocation with `BEADS_ACTOR` set leaves the commit message byte-identical; `BEADS_ACTOR` is not in the binary's strings; 0 of the last 60 commits carry the trailer (verified 2026-08-26 by opus gate + lead). Another instance of the 1.2.2 recovery-release doc/binary gap ([[task-tracking]] quirks list).

**RESOLVED 2026-08-27: the convention was deleted**, per user decision. Removed from soviet-simulator `CLAUDE.md`, `~/.claude/rules/task-tracking.md`, the `team-lead.md` initialPrompt, and `debugger.md`. The living convention is `bd comments add <id> "…" --author <roster-name>`, which works.

**How to apply:** never set `BEADS_ACTOR` in briefs; never cite `Executed-By:` trailers as provenance. The historical record lives in `docs/process/doc-audit-2026-08-26.md` §6 — that file is provenance, do not "fix" it.
