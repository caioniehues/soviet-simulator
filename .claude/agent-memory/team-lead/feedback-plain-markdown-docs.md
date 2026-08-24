---
name: feedback-plain-markdown-docs
description: Process/design documentation for this project goes in plain repo .md files — not published Artifacts, not a designed page; "no no, just document it in md files"
metadata:
  type: feedback
---

**Write project process and design documentation as plain `.md` files in the repo.** Do not reach
for an Artifact, a designed HTML page, or any published-page treatment.

**Why:** 2026-08-23, asked to capture the dev cycle, I began down the Artifact path (the
`artifact-design` skill fired and injected its treatment guidance). The user cut it off in one line:

> "no no, just document it in md files."

The reason it is right, and why it generalises: **the readers of these documents are agents.** Every
agent can `Read` a repo `.md`; none of them can open a published Artifact. A process doc that lives
outside the repo is invisible to the roster it is meant to govern — the same failure as
`docs/dev-cycle.md` sitting unreferenced until `CLAUDE.md` and `RESUME.md` were made to point at it.
An undiscoverable doc is a doc that does not exist.

**How to apply:**

- Process, architecture, roadmap, agent-roster and decision docs → `docs/*.md` in the repo, and add
  a pointer from `CLAUDE.md` in the same change, or it will not be found.
- Do not spend effort on visual treatment for these. Structure and tables are worth it because they
  aid reading; styling is not.
- This governs *documentation*. It says nothing about the game's own presentation, where the
  standing bar is the opposite — see [[playtest-polish-verdict]].
