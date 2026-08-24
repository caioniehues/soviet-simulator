# Documentation consolidation migration manifest

**Kind:** plan
**Authority:** operational
**Status:** superseded
**Owner:** project lead
**Last verified:** 2026-08-24

This move-first contract was superseded by
[`controlled-documentation-rewrite-plan.md`](controlled-documentation-rewrite-plan.md) after the
user approved an archive-first controlled rewrite. It remains the file-level inventory; the newer
plan controls sequencing and authority. Its historical/provenance paths were archived during Wave
1 under the newer plan; active-authority cutover paths remain pending.

This was the exact move contract for consolidating human-maintained project knowledge under
`docs/`. It does not authorize blind promotion of stale material. During the original move-first
proposal, current code was substrate authority, `docs/charter-1.0.md` was scope authority, `spec/`
was in-scope mechanism authority, and `br` was task-state authority. The controlled rewrite has
since activated `docs/plan/charter-1.0.md`; specifications remain at their legacy paths until their
Wave 1 replacements pass review.

## Root contract

The following stay at repository root because tools, hosts, or people discover them there:

| Path | Disposition |
|---|---|
| `README.md` | Keep; update navigation and remove authority conflicts during migration. |
| `AGENTS.md` | Keep; update every moved authority path atomically. |
| `CLAUDE.md` | Keep; update every moved authority path atomically. |
| `LICENSE` | Keep unchanged. |
| `NOTICE.md` | Keep; update archived upstream-document links. |

`CONTEXT.md` becomes a short discovery redirect to `docs/reference/glossary.md`; it must contain no
second copy of the glossary. `AGENTS.md` and `CLAUDE.md` will point directly to the canonical file.

## Active authority moves

| Count | Source | Destination | Required same-change work |
|---:|---|---|---|
| 1 | `CONTEXT.md` | `docs/reference/glossary.md` | Create root redirect; update charter and decision links. |
| 23 | `spec/README.md`, `spec/*.md` | `docs/reference/specifications/` | Preserve filenames; rewrite repository citations and internal relative links. Mark unresolved spec-review findings rather than silently normalizing them. |
| 1 | `docs/charter-1.0.md` | `docs/plan/charter-1.0.md` | Update all root entrypoints, requirements, archive successors, and process references. |
| 1 | `docs/dev-cycle.md` | `docs/process/development-cycle.md` | Update AGENTS, CLAUDE, RESUME, agent definitions, and framework research references. |
| 1 | `docs/art-direction.md` | `docs/reference/art-direction.md` | Update CLAUDE and `tools/bake_ground.py`; retain `active-needs-reality-audit` status until stale Bevy enforcement paths are repaired. |
| 36 | `docs/superpowers/iterations/requirements/EPIC-*.md` | `docs/plan/iterations/requirements/` | Preserve filenames and STORY IDs; update all `spec/` and audit citations. |
| 8 | `docs/superpowers/iterations/extract/*.json` | `docs/plan/iterations/extract/` | Preserve filenames and keep the validator beside the data. |
| 1 | `docs/superpowers/iterations/extract/validate.py` | `docs/plan/iterations/extract/validate.py` | Re-run against all eight JSON inputs after the move. |
| 1 | `docs/superpowers/iterations/RESUME.md` | `docs/plan/iterations/RESUME.md` | Reality-audit stale agent state, counts, and commit claims before marking active. |
| 3 | `docs/superpowers/iterations/{behavior-scenarios,behavior-corpus,coverage-ledger}.md` | `docs/plan/iterations/evidence/` | Preserve as maintained evidence indexes; do not call generated because no reproducible generator exists. |
| 1 | `docs/superpowers/iterations/build_roadmap.py` | `docs/plan/iterations/tools/build_roadmap.py` | Replace hard-coded input/output paths; execute from repository root. |
| 1 | `docs/superpowers/iterations/roadmap.md` | `docs/generated/iterations/roadmap.md` | Regenerate rather than copy; fix the current `1 deferred` versus `19 deferred` contradiction. |

`docs/decisions/` starts empty except for its index and template. No pre-fork ADR is current merely
because it was once accepted.

## Explanation and research moves

| Count | Source | Destination | Classification |
|---:|---|---|---|
| 1 | `docs/framework-design.md` | `docs/explanation/research/agent-frameworks/design.md` | Dated research and proposed operating design; not process authority. |
| 1 | `docs/framework-study-brief.md` | `docs/explanation/research/agent-frameworks/study-brief.md` | Research input retained beside its output. |
| 1 | `docs/research/documentation-architecture.md` | `docs/explanation/research/documentation-architecture.md` | Primary-source-backed rationale for this migration. |
| 1 | `docs/research/documentation-migration-manifest.md` | `docs/plan/documentation-migration.md` | This approved move contract becomes the executable plan. |

No current product-vision document exists. The 2,136-line `docs/vision/session-2026-08-16.md` is a
raw conversation export and therefore belongs in the archive, not in active explanation.

## Historical and provenance moves

| Count | Source | Destination | Reason |
|---:|---|---|---|
| 1 | `ROADMAP.md` | `docs/archive/bevy-track/ROADMAP.md` | AGENTS explicitly identifies it as discarded Bevy history. |
| 2 | `architecture/*.md` | `docs/archive/bevy-track/architecture/` | Both documents are explicitly Bevy mappings. |
| 17 | `docs/adr/*.md` | `docs/archive/bevy-track/decisions/` | Every ADR predates the fork; its implementation/status claims were recorded against the removed Bevy tree. Revalidate surviving concepts into new ADRs individually. |
| 1 | `research/bevy-ecosystem.md` | `docs/archive/bevy-track/research/bevy-ecosystem.md` | Engine research for the discarded track. |
| 2 | `findings.md`, `progress.md` | `docs/archive/bevy-track/session-notes/` | Ignored Bevy-era working notes with deleted source paths. |
| 1 | `docs/archive/bevy-track-README.md` | `docs/archive/bevy-track/README.md` | Existing Bevy status archive; update root README link. |
| 1 | `docs/archive/bevy.md` | `docs/archive/bevy-track/engine-guide.md` | Existing Bevy operating guide. |
| 1 | `docs/wayfinder-brief.md` | `docs/archive/bevy-track/wayfinder-brief.md` | Its own header declares it historical and superseded. |
| 1 | `docs/vision/session-2026-08-16.md` | `docs/archive/raw-sessions/vision-session-2026-08-16.md` | Raw conversation transcript, not curated project truth. |
| 1 | `2026-08-17-215718-local-command-caveatcaveat-the-messages-below.txt` | `docs/archive/raw-sessions/local-command-2026-08-17.txt` | Ignored raw terminal/session transcript. |
| 1 | `docs/egregoria-architecture.md` | `docs/archive/upstream-egregoria/architecture.md` | Imported upstream explanation, not current fork architecture. |
| 2 | `docs/archive/egregoria-README.md`, `egregoria-CONTRIBUTING.md` | `docs/archive/upstream-egregoria/` | Upstream provenance documents; preserve filenames. |
| 1 | `docs/egregoria-substrate-audit.md` | `docs/archive/egregoria-import/substrate-audit-2026-08-22.md` | Import-day snapshot now contradicted by later production commits; do not promote it as current substrate truth. |
| 16 | `docs/superpowers/iterations/par/*.md` | `docs/archive/iterations/requirements-pass/par/` | Historical omission-review audit trail. |
| 1 | `docs/superpowers/iterations/task_plan.md` | `docs/archive/iterations/requirements-pass/scope-cut-plan.md` | Superseded: it says the already-applied scope cut is pending. |
| 1 | `docs/superpowers/iterations/apply_cut.py` | `docs/archive/iterations/requirements-pass/apply-cut.py` | One-off, completed migration tool retained for auditability. |
| 1 | `docs/superpowers/iterations/brief-truck.md` | `docs/archive/iterations/ITER-0000/brief-truck.md` | Fixed-commit execution brief whose task has already moved on. |
| 1 | `task_plan.md` | `docs/archive/iterations/ITER-0000/lead-task-plan.md` | Ignored session plan; `br` and RESUME remain the live state surfaces. |

The consolidation must create a new `docs/reference/architecture/substrate.md` from a current,
code-cited substrate audit. The stale import-day audit cannot be renamed into that role.

## Explicitly excluded from consolidation

| Paths | Disposition | Reason |
|---|---|---|
| `.claude/**`, `.codex/**` | Keep in place | Agent definitions, skills, adapters, and memory are operational inputs with path-based discovery. |
| `.beads/**` | Keep in place | `br` task ledger storage; only its four documented files may be staged. |
| `.planning/**`, `.remember/**` | Keep in place | Tool/session state, including stale Bevy history; not repository navigation documents. |
| `vectiler/**/README.md`, third-party readmes | Keep in place | Package-local or vendored documentation must remain with its consumer. |
| `assets/**/*.txt`, `simulation/src/**/*.txt` | Keep in place | Licences, data, and runtime name tables—not project documentation. |

The obsolete `.claude/skills/bevy-*` packages and Bevy-era agent memories need a separate agent
hygiene disposition. Moving them would change tool routing, so it is deliberately outside this
documentation-only migration.

## Files created during migration

| Path | Purpose |
|---|---|
| `docs/README.md` | Sole navigation index and authority/lifecycle map. |
| `docs/decisions/README.md` | Current ADR index; initially records that no pre-fork ADR is automatically active. |
| `docs/reference/architecture/substrate.md` | Current code-cited fork substrate, replacing the stale import snapshot as a live reference. |
| `docs/templates/{specification,decision,research,process,generated}.md` | Five per-kind formats with the approved metadata fields. |

## Atomic execution order

1. Create indexes, templates, archive directories, and the current substrate document.
2. Archive stale Bevy, upstream, raw-session, audit, and completed-iteration material.
3. Move active authorities and update AGENTS, CLAUDE, README, NOTICE, tools, and internal links in the same change.
4. Move iteration inputs/tools, regenerate the roadmap at its destination, and validate the eight extraction files.
5. Run repository-wide link/anchor checks and prove every discovery path and generator command resolves.

## Completion gates

- `rg` finds no live reference to old `spec/`, `architecture/`, `docs/adr/`, or
  `docs/superpowers/iterations/` paths outside archive snapshots and this manifest.
- Every active document carries Kind, Authority, Status, Owner, and Last verified fields.
- The roadmap is reproduced from the relocated requirements and reports 130 scheduled plus 19
  deferred stories, or the verified current counts if they changed before execution.
- Root discovery files resolve directly to the new canonical authorities.
- `git status` contains no unrelated staged file and no operational directory was relocated.

Estimated execution time: 4 hours, including path rewrites and validation. This estimate excludes
rewriting all stale specifications or re-ratifying the 17 archived ADRs.
