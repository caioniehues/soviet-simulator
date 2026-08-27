---
name: sweep-uncommitted-docs-2026-08-27
description: What was swept in the pre-commit documentation audit of 2026-08-27 at HEAD ba7e8e7, and which artifacts drifted — check these first next time
metadata:
  type: project
---

Sweep of the uncommitted documentation diffs before the six-way commit split. Tree state:
HEAD `ba7e8e7` plus an uncommitted working tree of 162 paths.

**Swept and verified clean:** `docs/plan/charter-1.0.md` (one word, `br`→`bd`; scope, Post-1.0 and
Never lists untouched), `.gitignore` (adds `/book/` and `.code-review-graph/`; both rooted or
dir-only, no shadowing — the old `target*` incident stays fixed at `.gitignore:26-27`),
`docs/SUMMARY.md` (181 links, all resolve), the 22 `docs/archive/legacy-specifications/*.md`
supersede banners (every named target exists), the roadmap `--check`, and the counts
21/107/0/26 in `RESUME.md` against the artifacts.

**Artifacts that drifted — check these first next time:**

1. **Generator header strings.** See [[generated-artifacts-and-generators]]. Half of them were
   repointed after the `docs/generated/` move and half were not.
2. **`docs/plan/iterations/RESUME.md` "Next work" section.** It names a `bd` id as the front of
   the queue; that id was already closed when the line was written. Always re-run `bd ready` and
   `bd show` on every id RESUME names.
3. **Roster counts and tiers in `docs/process/development-cycle.md` and `CLAUDE.md`.** Cheap to
   verify: `ls .claude/agents/*.md | wc -l`, and `grep -m1 '^model:'` / `'^effort:'` per file.
   The per-lane line counts (`simulation/` ~17,700 etc.) are also cheap and were accurate.
4. **New process-doc links to untracked files.** A doc committed in one commit and its link target
   left untracked reads exactly like a live reference. Check `git status` for the target, not just
   the filesystem.

**Standing traps found:**

- The managed Beads blocks in `CLAUDE.md` and `AGENTS.md` say "use `bd remember`, do NOT use
  MEMORY.md files". `AGENTS.md:54-55` and the `CLAUDE.md` task-tracking section explicitly
  override both. That is not a contradiction — do not re-report it.
- `.claude/settings.json` and `.mcp.json` are the wiring behind the `code-review-graph` block in
  `CLAUDE.md`; `.claude/settings.json` is gitignored by `.claude/*`, so a fresh clone gets the
  instruction without the hook.

Related: [[sweep-agent-roster-2026-08-27]].
