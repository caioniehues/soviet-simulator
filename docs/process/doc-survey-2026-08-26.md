---
Kind: process
Authority: informational
Status: draft
Owner: doc-audit
Last verified: 2026-08-26
---

# Documentation corpus survey (2026-08-26)

Read-only inventory for `sov-2xt`, feeding the mdBook restructure plan in `sov-q7h`. No
files were moved, edited, or deleted to produce this survey — the corpus stays exactly
where it lives; this document only maps it.

**Totals:** 184 markdown files under `docs/`, plus `README.md`, `CLAUDE.md`, `AGENTS.md`
at the repo root. **122 files live under `docs/archive/`**; **62 are active (non-archive)
files**. Orphan count (no other tracked markdown file references the file's basename):
**58**, all but two of them archived legacy requirement/PAR documents that were never
meant to be linked individually (see Orphans below).

"Linked-from" was computed by grepping every other markdown file for each file's
basename — it catches wiki-style and relative markdown links but not prose mentions of a
directory without a filename. Treat it as a lower bound, not a proof of true isolation.

## Inventory

Kind and authority-header presence are read from each file's own front matter — a
`**Kind:**` (or historical variants: `Kind:`, `Authority:`, `Status:`, `Owner:`) block at
the top. "header?" counts how many of the four fields (Kind/Authority/Status/Owner) are
present; 4 = full block, 0 = none, partial values in between are noted.

### Root entrypoints

| path | lines | kind | header? | linked-from |
|---|---|---|---|---|
| CLAUDE.md | 178 | process entrypoint | 4/4 | AGENTS.md, docs/plan/controlled-documentation-rewrite.md, docs/plan/documentation-migration.md, docs/process/development-cycle.md, docs/process/review-2026-08-26-vs-swarmforge.md, docs/reference/bd-capability-survey.md |
| AGENTS.md | 136 | process entrypoint | 4/4 | CLAUDE.md, docs/plan/controlled-documentation-rewrite.md, docs/plan/documentation-migration.md |
| README.md | 70 | repository entrypoint | 4/4 | ORPHAN |

### docs/process/ (active)

| path | lines | kind | header? | linked-from |
|---|---|---|---|---|
| docs/process/development-cycle.md | 334 | process | 4/4 | CLAUDE.md |
| docs/process/review-2026-08-26-vs-swarmforge.md | 83 | process audit | 2/4 (Kind, Owner only) | ORPHAN |

### docs/plan/ (active — plan of record + live iteration)

| path | lines | kind | header? | linked-from |
|---|---|---|---|---|
| docs/plan/charter-1.0.md | 93 | plan | 4/4 | CLAUDE.md (via `docs/plan/charter-1.0.md`) |
| docs/plan/controlled-documentation-rewrite.md | 185 | plan | 4/4 | CLAUDE.md, AGENTS.md |
| docs/plan/documentation-migration.md | 137 | plan | 4/4 | CLAUDE.md, AGENTS.md |
| docs/plan/iterations/RESUME.md | 64 | plan handoff | 4/4 | CLAUDE.md |
| docs/plan/iterations/requirements/README.md | 30 | requirements-index | 4/4 | linked from sibling requirement files (built-world, economy, movement, settlement, utilities all share `README.md` basename cross-refs — see note below) |
| docs/plan/iterations/requirements/built-world.md | 57 | requirements | 4/4 (13 header-line hits incl. cross-refs) | docs/plan/iterations/requirements/README.md |
| docs/plan/iterations/requirements/economy.md | 57 | requirements | 4/4 | docs/plan/iterations/requirements/README.md |
| docs/plan/iterations/requirements/movement.md | 89 | requirements | 4/4 | docs/plan/iterations/requirements/README.md |
| docs/plan/iterations/requirements/settlement.md | 89 | requirements | 4/4 | docs/plan/iterations/requirements/README.md |
| docs/plan/iterations/requirements/utilities.md | 89 | requirements | 4/4 | docs/plan/iterations/requirements/README.md |
| docs/plan/iterations/evidence/coverage.md | 40 | generated evidence coverage | 4/4 | docs/plan/iterations/RESUME.md (referenced by name in evidence discussion) |
| docs/plan/iterations/evidence/current-regression-inventory.md | 39 | generated current-regression inventory | 4/4 | docs/plan/iterations/RESUME.md |
| docs/plan/traceability/story-migration.md | 167 | traceability | 4/4 | docs/plan/documentation-migration.md |

**Note on requirements/README.md link direction:** the basename-grep method above
reports `README.md` as widely "linked" because many directories contain a `README.md`
and the grep matches on the bare filename across all of them — inflating apparent
cross-links between unrelated READMEs. Treat any `README.md` / `INDEX.md` row's
linked-from column as noisy; verify by path, not basename, before trusting it in the
restructure.

### docs/reference/ (active)

| path | lines | kind | header? | linked-from |
|---|---|---|---|---|
| docs/README.md | 43 | reference | 4/4 | (docs-root index; not itself deep-linked, expected) |
| docs/reference/architecture/substrate.md | 87 | reference | 4/4 | CLAUDE.md |
| docs/reference/art-direction.md | 79 | reference | 4/4 | CLAUDE.md |
| docs/reference/bd-capability-survey.md | 206 | survey report | 4/4 | CLAUDE.md |
| docs/reference/glossary.md | 103 | reference | 4/4 | cross-referenced by specification files (glossary terms) |
| docs/reference/specifications/README.md | 71 | reference | 4/4 | index for the 21 specification files below |
| docs/reference/specifications/{buildings,citizens,construction,crime,education,electricity,healthcare,heating,households,logistics,needs,pathfinding,production,resources,roads,sewage,trade,traffic,vehicles,waste,water,zoning}.md | 56–139 each | specification | 4/4 each | docs/reference/specifications/README.md (index); several also referenced from CLAUDE.md's "Ratified files under docs/reference/specifications/ bind mechanism" line |

That's 21 domain specification files (buildings through zoning), each with a full
4/4 authority header — this is the most disciplined, consistently-headered cluster in
the whole corpus.

### docs/research/ (active — fact sheets)

| path | lines | kind | header? | linked-from |
|---|---|---|---|---|
| docs/research/fact-sheets/wave1-economy.md | 90 | reference | 4/4 | referenced by CLAUDE.md's "follow its fact-sheet citations" via substrate.md |
| docs/research/fact-sheets/wave1-logistics.md | 112 | reference | 4/4 | same |
| docs/research/fact-sheets/wave1-substrate.md | 97 | reference | 4/4 | same |
| docs/research/fact-sheets/wave2-substrate.md | 56 | reference | 4/4 | same |
| docs/research/fact-sheets/wave3-corpus.md | 94 | reference | 4/4 | ORPHAN |

### docs/explanation/ (active — research/explanation)

| path | lines | kind | header? | linked-from |
|---|---|---|---|---|
| docs/explanation/research/documentation-architecture.md | 168 | explanation | 4/4 (8 hits — has extra Kind-like lines) | referenced from docs/plan/documentation-migration.md |
| docs/explanation/research/technical-stack-upstream-2026-08-24.md | 345 | explanation | 4/4 | ORPHAN |
| docs/explanation/research/agent-frameworks/design.md | 1171 | explanation | 4/4 | docs/explanation/research/agent-frameworks/study-brief.md |
| docs/explanation/research/agent-frameworks/study-brief.md | 194 | research brief | 4/4 | docs/explanation/research/agent-frameworks/design.md |

### docs/generated/ (active — generated status)

| path | lines | kind | header? | linked-from |
|---|---|---|---|---|
| docs/generated/iterations/roadmap.md | 44 | generated roadmap | 4/4 | CLAUDE.md |

### docs/decisions/ and docs/templates/ (active — scaffolding)

| path | lines | kind | header? | linked-from |
|---|---|---|---|---|
| docs/decisions/README.md | 32 | reference | 4/4 | (empty ADR directory — see oddities) |
| docs/templates/decision.md | 39 | decision | 4/4 | docs/decisions/README.md (as the template to copy) |
| docs/templates/generated.md | 23 | generated | 4/4 | ORPHAN |
| docs/templates/process.md | 32 | process | 4/4 | ORPHAN |
| docs/templates/research.md | 31 | explanation | 4/4 | docs/explanation/research files reference "use this template" conceptually, not by link |
| docs/templates/specification.md | 47 | specification | 4/4 | docs/reference/specifications/README.md |

### docs/archive/ (122 files — provenance only, not authoritative)

Kind headers are almost entirely absent in `docs/archive/` — only the per-directory
`INDEX.md` files (bevy-track, egregoria-import, iterations, raw-sessions,
upstream-egregoria — 5 files) and `docs/archive/README.md` carry a `**Kind:** historical`
header. The other 116 archived files (decision records, specs, EPICs, PAR docs, session
notes, the Bevy README/ROADMAP, the legacy corpus) have **no authority header at all** —
consistent with them being frozen provenance rather than documents anyone should edit
in place.

| subtree | files | header? | notes |
|---|---|---|---|
| docs/archive/bevy-track/ | 23 | 1/23 (INDEX.md only) | discarded Bevy track: architecture, 17 numbered decision records, engine-guide, README, ROADMAP, research, session-notes, wayfinder-brief |
| docs/archive/egregoria-import/ | 2 | 1/2 (INDEX.md) | fork-point substrate audit |
| docs/archive/iterations/ | 92 | 1/92 (top INDEX.md) | ITER-0000 briefs, the legacy corpus (behavior-corpus, 3828-line behavior-scenarios, coverage-ledger, 36 EPIC files, RESUME, roadmap), legacy/README, requirements-pass PAR docs (16 files, r1/r2 per domain), scope-cut-plan |
| docs/archive/legacy/ | 1 | 0/1 | charter-1.0.md (superseded by docs/plan/charter-1.0.md) |
| docs/archive/legacy-specifications/ | 23 | 1/23 (README.md has 0, actually none do — see below) | 21 domain specs superseded by docs/reference/specifications/, plus README |
| docs/archive/raw-sessions/ | 2 | 1/2 (INDEX.md) | one long (2136-line) vision session transcript |
| docs/archive/upstream-egregoria/ | 4 | 1/4 (INDEX.md) | imported upstream Egregoria docs (architecture, CONTRIBUTING, README) |

The `docs/archive/legacy-specifications/README.md` itself carries no header (0/4) —
correction to the row estimate above, it is the one archive README without one.

## Orphans (58 total)

Two are outside the expected archive-noise pattern and worth a second look before the
restructure:

- `docs/process/review-2026-08-26-vs-swarmforge.md` — an active, non-archive process
  document with zero inbound references. Either it should be linked from
  `docs/process/development-cycle.md` (if still relevant) or explicitly marked
  superseded/archived.
- `docs/research/fact-sheets/wave3-corpus.md` — active fact sheet, same situation: the
  wave1/wave2 fact sheets are reachable via substrate.md's citation trail, wave3 is not.

The remaining 56 orphans are all under `docs/archive/` (36 EPIC files under
`docs/archive/iterations/legacy/corpus/requirements/`, 16 PAR files under
`docs/archive/iterations/requirements-pass/par/`, plus
`docs/explanation/research/technical-stack-upstream-2026-08-24.md`,
`docs/templates/generated.md`, `docs/templates/process.md`, and two Bevy decision
records `0004` and `0006`). These are expected: EPIC/PAR files are meant to be read as a
directory listing (via their own INDEX/README), not cross-linked individually, and the
two orphaned templates are meant to be copied, not linked.

## Draft SUMMARY.md

Proposed mdBook navigation tree. Covers all 62 active files; archive collapses to one
section (122 files, not individually enumerated in the tree — mdBook would still need
per-file entries to render them, but readers should not be expected to browse them one
by one). Files flagged `⚠` don't fit cleanly into intent-based grouping — see notes
below the tree.

```markdown
# Summary

[Introduction](../README.md)

# Process — how work gets done

- [Development cycle](process/development-cycle.md)
- [Process audit: 2026-08-26 vs Swarmforge](process/review-2026-08-26-vs-swarmforge.md)  ⚠ orphan, confirm still current

# Plan of record

- [Charter 1.0](plan/charter-1.0.md)
- [Documentation migration plan](plan/documentation-migration.md)
- [Controlled documentation rewrite](plan/controlled-documentation-rewrite.md)
- [Story migration traceability](plan/traceability/story-migration.md)
- [Live iteration](plan/iterations/RESUME.md)
  - [Requirements index](plan/iterations/requirements/README.md)
    - [Built world](plan/iterations/requirements/built-world.md)
    - [Economy](plan/iterations/requirements/economy.md)
    - [Movement](plan/iterations/requirements/movement.md)
    - [Settlement](plan/iterations/requirements/settlement.md)
    - [Utilities](plan/iterations/requirements/utilities.md)
  - [Evidence: coverage](plan/iterations/evidence/coverage.md)
  - [Evidence: current regression inventory](plan/iterations/evidence/current-regression-inventory.md)

# Reference & specifications

- [Reference index](README.md)
- [Substrate architecture](reference/architecture/substrate.md)
- [Art direction](reference/art-direction.md)
- [Glossary](reference/glossary.md)
- [bd capability survey](reference/bd-capability-survey.md)
- [Specifications index](reference/specifications/README.md)
  - [Buildings](reference/specifications/buildings.md)
  - [Citizens](reference/specifications/citizens.md)
  - [Construction](reference/specifications/construction.md)
  - [Crime](reference/specifications/crime.md)
  - [Education](reference/specifications/education.md)
  - [Electricity](reference/specifications/electricity.md)
  - [Healthcare](reference/specifications/healthcare.md)
  - [Heating](reference/specifications/heating.md)
  - [Households](reference/specifications/households.md)
  - [Logistics](reference/specifications/logistics.md)
  - [Needs](reference/specifications/needs.md)
  - [Pathfinding](reference/specifications/pathfinding.md)
  - [Production](reference/specifications/production.md)
  - [Resources](reference/specifications/resources.md)
  - [Roads](reference/specifications/roads.md)
  - [Sewage](reference/specifications/sewage.md)
  - [Trade](reference/specifications/trade.md)
  - [Traffic](reference/specifications/traffic.md)
  - [Vehicles](reference/specifications/vehicles.md)
  - [Waste](reference/specifications/waste.md)
  - [Water](reference/specifications/water.md)
  - [Zoning](reference/specifications/zoning.md)
- [Fact sheets: wave 1 — economy](research/fact-sheets/wave1-economy.md)
- [Fact sheets: wave 1 — logistics](research/fact-sheets/wave1-logistics.md)
- [Fact sheets: wave 1 — substrate](research/fact-sheets/wave1-substrate.md)
- [Fact sheets: wave 2 — substrate](research/fact-sheets/wave2-substrate.md)
- [Fact sheets: wave 3 — corpus](research/fact-sheets/wave3-corpus.md)  ⚠ orphan, confirm still current
- [Decisions (ADR log)](decisions/README.md)  ⚠ empty log, template only — see oddities

# Explanation & research

- [Documentation architecture](explanation/research/documentation-architecture.md)
- [Technical stack (upstream, 2026-08-24)](explanation/research/technical-stack-upstream-2026-08-24.md)  ⚠ orphan
- [Agent frameworks: study brief](explanation/research/agent-frameworks/study-brief.md)
- [Agent frameworks: design](explanation/research/agent-frameworks/design.md)

# Generated status

- [Iteration roadmap (generated)](generated/iterations/roadmap.md)

# Templates

- [Decision record template](templates/decision.md)
- [Generated-doc template](templates/generated.md)  ⚠ orphan, reference-only
- [Process-doc template](templates/process.md)  ⚠ orphan, reference-only
- [Research-doc template](templates/research.md)
- [Specification template](templates/specification.md)

---

# Archive (provenance only — superseded, not authoritative)

- [Archive index](archive/README.md)
  - [Bevy track (discarded 2026-08-22)](archive/bevy-track/INDEX.md)
  - [Egregoria import](archive/egregoria-import/INDEX.md)
  - [Legacy iterations](archive/iterations/INDEX.md)
  - [Legacy charter 1.0](archive/legacy/charter-1.0.md)
  - [Legacy specifications](archive/legacy-specifications/README.md)
  - [Raw sessions](archive/raw-sessions/INDEX.md)
  - [Upstream Egregoria](archive/upstream-egregoria/INDEX.md)
```

Files that fit nowhere cleanly in an intent-based tree: none outright unplaceable, but
three are marked `⚠` above because their *content* is fine while their *link health or
currency* is unconfirmed — a restructure should either re-link or explicitly retire
them rather than silently carry them forward:

1. `docs/process/review-2026-08-26-vs-swarmforge.md` — orphaned process audit, unclear
   if superseded by the current `development-cycle.md`.
2. `docs/research/fact-sheets/wave3-corpus.md` — orphaned fact sheet, wave1/wave2 are
   reachable via substrate.md, this one isn't.
3. `docs/explanation/research/technical-stack-upstream-2026-08-24.md` — orphaned,
   345 lines, worth confirming it's still cited by anything before promoting it in nav.

## Structural oddities

1. **`docs/decisions/README.md` describes a process with zero decisions recorded.** The
   directory holds only the README and points at `docs/templates/decision.md`; no actual
   ADR exists there, while 17 numbered decision records live frozen under
   `docs/archive/bevy-track/decisions/` for the discarded track. If ADRs are still the
   intended mechanism, the directory is currently empty of real content — a dir with a
   README and no decisions apart from an archived, obsolete set from a discarded track.

2. **Duplicate topic: domain specifications exist in two places with different
   authority.** `docs/reference/specifications/*.md` (21 files, ratified per CLAUDE.md)
   and `docs/archive/legacy-specifications/*.md` (21 files, same domain names:
   buildings, citizens, construction, crime, education, electricity, healthcare,
   heating, households, logistics, needs, pathfinding, production, resources, roads,
   sewage, trade, traffic, vehicles, waste, water, zoning) are a 1:1 name match. This is
   presumably intentional (superseded-by relationship) but nothing in either file states
   it explicitly — no "superseded by" pointer in the legacy files, no "supersedes"
   pointer in the current ones. A reader who lands on the archive copy via search has no
   in-document signal to redirect to the ratified one.

3. **Duplicate topic: two charter-1.0.md files.** `docs/plan/charter-1.0.md` (current,
   93 lines) and `docs/archive/legacy/charter-1.0.md` (frozen, 199 lines) — same
   filename, different directories, more than double the line count difference
   suggesting real content divergence, not just a copy. Same missing cross-pointer
   problem as #2.

4. **Generated files mixed with authored ones, same directory tree.** `docs/plan/`
   contains hand-authored plan documents (`charter-1.0.md`,
   `controlled-documentation-rewrite.md`) alongside `docs/plan/iterations/evidence/*.md`,
   which carry `**Kind:** generated …` headers — i.e., machine-produced content lives
   inside the same subtree as durable plan-of-record prose. `docs/generated/` exists as
   a separate top-level directory for exactly this purpose (currently holding only
   `iterations/roadmap.md`), so the evidence files under `docs/plan/iterations/evidence/`
   are generated content that didn't land in the directory named for generated content.

5. **Directories with a single file:** `docs/decisions/` (README.md only — see #1),
   `docs/generated/iterations/` (roadmap.md only), `docs/archive/legacy/` (charter-1.0.md
   only). None of these are wrong on their own, but each is a directory-per-file where a
   flatter placement (e.g. `docs/generated/roadmap.md`) would carry the same information
   with one less nesting level — worth a look during the mdBook restructure rather than
   preserving the nesting by default.

6. **`docs/archive/legacy-specifications/README.md` is the one archive-subtree README
   without a `**Kind:**` header**, while every sibling archive INDEX/README (bevy-track,
   egregoria-import, iterations, raw-sessions, upstream-egregoria, and the top-level
   `docs/archive/README.md`) has one. Minor inconsistency, but it means one archive
   subtree gives a reader no explicit "this is historical" marker at its own entry point.

7. **`docs/archive/iterations/` is the largest and most heterogeneous archive subtree**
   (92 of the 122 archived files): it mixes ITER-0000 briefs, a 3828-line
   `behavior-scenarios.md`, a 36-file EPIC requirement set, a 16-file PAR
   (requirements-pass) set with r1/r2 revisions per domain, and a scope-cut-plan — five
   distinct document types under one directory with only a single top-level `INDEX.md`
   to navigate all of it. The mdBook tree above collapses this to one link
   (`archive/iterations/INDEX.md`) for that reason; a reader who wants a specific EPIC or
   PAR file has to go through that index, not the SUMMARY.md tree.
