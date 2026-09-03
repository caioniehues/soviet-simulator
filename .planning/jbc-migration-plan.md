# JBC migration plan: documentation target IA + implementation sequence

**Kind:** plan
**Authority:** operational
**Status:** draft
**Owner:** project lead
**Last verified:** 2026-09-03

Discovery output for `sov-jbc`. Read-only inputs; this file proposes no edit
under `docs/`, `book/`, `scripts/`, `.github/`, or any tracked `.md` outside
`.planning/`. Every implementation step below names ONE file owner, its
SUMMARY/checker budget, and the docs validation command.

Validation command (every step must end green):

```bash
python3 scripts/check_docs.py && mdbook build
```

Current baseline (2026-09-03): 244 active files, 0 errors, 2 warnings
(duplicate H1 `Soviet Simulator` in `README.md` + `docs/SUMMARY.md`;
duplicate H1 `Allocation` in the two allocation pages).

## Binding inputs (taken as given, never silently drifted)

| Input | What it fixes | This plan reuses |
|---|---|---|
| `sov-rut` (closed, `3cb484f`) | Single Kind/Authority vocabulary; exactly one canonical substrate map (`docs/architecture/current-substrate.md`; `docs/reference/architecture/substrate.md` is the cited derivative); templates + generator headers aligned | Target taxonomy §3 |
| `sov-brv` (closed, `a18de9e`) | Research-corpus metadata, `AGENTS.md` header, beads page in SUMMARY, generated ledger header | Corpus state §2 |
| `sov-ik2` (closed, `9f09a5d`) | Checker scope: metadata on specs + 10 `WIKI_SECTIONS` (incl. `research/`, `explanation/`) + root entrypoints; orphans are errors; `Verified-at` via the `Implementation claims` marker; rules 1–5, 7, 10–11 review-only | Checker budget per step §7 |
| ADR-0001 `sov-m7r` (accepted) | Charter now 11 rows (Households/citizens + Utilities = electricity, water, heating, waste; sewage cut); scope vocabulary exactly `1.0` (traceable to a charter row) or `Post-1.0`; `candidate` banned | Scope rows §6 |

Any proposed delta to the above becomes a named follow-up `bd` issue (§8),
never an inline redefinition.

## 1. Standards research (primary sources)

1. **Diátaxis** — four reader intents (tutorial, how-to, reference,
   explanation) kept on separate pages; one page answers one intent.
   Source: <https://diataxis.fr> (see also
   <https://diataxis.fr/how-to-use-diataxis/>). Local mapping: `guide` =
   how-to, `concept` = explanation, `reference`/`specification` = reference,
   `tutorial` = tutorial. The model page already states this
   (`docs/meta/documentation-model.md:60-61`); the migration enforces it
   instead of re-stating it.
2. **Google developer documentation style guide** — task-first titles,
   second person for procedures, present tense for behaviour, normative
   `MUST`/`SHOULD` only where RFC 8174 is declared; word list prefers one
   name per thing. Source: <https://developers.google.com/style>. Local
   mapping: glossary `Avoid:` lines are the project word list; normative
   language stays inside specifications/standards per
   `docs/meta/document-authority.md:60-65`.
3. **Write the Docs** — docs-like-code (reviewed, versioned, CI-checked),
   explicit audience analysis, and findability through a single tested
   navigation spine. Sources: <https://www.writethedocs.org/guide/>,
   <https://www.writethedocs.org/guide/docs-as-code/>.
   Local mapping: `check_docs.py` + `mdbook build` in CI
   (`.github/workflows/docs.yml`) are the docs-as-code gates; `SUMMARY.md`
   is the single tested spine.

Consequence for the target IA: no new page types, no new authority labels,
no new navigation root. The work is placement, splitting mixed-intent
pages, and closing reachability gaps — all inside the `sov-rut` vocabulary.

## 2. Audience inventory (human-facing corpus, ~244 active files)

Counts from `docs/*/`: simulation 69, plan 27, research 22, architecture 21,
reference 32, engineering 13, developer 12, product 8, process 7, meta 6,
explanation 5, decisions 4, generated 3, templates 5, vision 1.

| Audience | Entry points | Working set |
|---|---|---|
| Developer (builds the sim) | `README.md`, `AGENTS.md`, `CONTEXT.md`, `CLAUDE.md`, `docs/developer/index.md`, `docs/architecture/index.md` | `docs/developer/*` (12 how-tos), `docs/architecture/*` (21), `docs/engineering/*` (13), `docs/reference/code-intelligence.md`, `docs/reference/subagent-tooling.md`, `docs/reference/bd-capability-survey.md` |
| Contributor (changes docs/process) | `docs/developer/how-to-read-the-docs.md`, `docs/meta/*`, `docs/process/development-cycle.md`, `docs/engineering/documentation.md` | `docs/meta/*`, `docs/process/*` (7), `docs/templates/*` (5), `docs/agents/*` (3, see gap G3) |
| Player (plays as the Planner) | `docs/index.md`, `docs/product/*` (8), `docs/vision/design-bible.md` | `docs/product/*`, `docs/simulation/concepts/*` (10), `docs/reference/glossary.md`, `docs/reference/mechanics-index.md` |
| Modder (data-driven content) | `docs/developer/adding-a-resource.md`, `docs/developer/adding-a-building.md`, `base_mod/*.lua` + generated evidence | `docs/reference/specifications/*` (22, all `Status: draft`), `docs/simulation/physical-economy/*` (9), `docs/generated/evidence/*` |

Observations (inventory, not edits):

- Player path is intact: portal → product → concepts → glossary/mechanics-index.
- Modder path is draft-gated by design: all specifications are `Status: draft`
  (bind nothing per the authority hierarchy rank 3); the plan must not
  present them as buildable contracts.
- Contributor path has the gaps: `docs/agents/*.md` (3 files, no metadata
  headers, not in SUMMARY, outside `WIKI_SECTIONS` so checker-silent) and
  the empty `docs/superpowers/` directory.
- `docs/explanation/research/*` (5 files incl. the beads integration page)
  is listed under the SUMMARY `Research` heading while living under
  `explanation/`; checker passes (both sections are wiki-checked) but the
  placement contradicts the layout rule.

## 3. Target taxonomy (equals `sov-rut`; not re-litigated)

Kinds (from `docs/meta/documentation-model.md` + aligned templates):
`standard`, `reference`, `concept`, `guide`, `specification`, `research`,
`generated`, `decision`, `process`, `tutorial`, page-type index pages.
Authority labels (from `docs/meta/document-authority.md:32-42`):
`binding`, `operational`, `advisory`, `reference`, `observational`,
`research`, `historical`, `derived`. Canonical substrate map:
`docs/architecture/current-substrate.md`; `docs/reference/architecture/substrate.md`
is its cited derivative.

Target IA in Diátaxis terms (labels are readings, not new Kinds):

- Tutorial: `docs/developer/getting-started.md` (+ repository tour as
  orientation). Gap: no true end-to-end tutorial exists; §8 names it.
- How-to (`guide`): all 12 `docs/developer/*` guides; each answers one task.
- Reference: `docs/reference/specifications/*`, glossary, mechanics-index,
  invariants, `docs/generated/*` (derived, regenerate-only).
- Explanation (`concept` + `research` + vision/product): simulation concepts,
  architecture target pages, design bible, research corpus. Mixed-intent
  pages split here (§7 steps S4–S5).

## 4. SUMMARY-shaped navigation (reachability-preserving)

Constraint: every active (non-`archive/`) page stays listed in
`docs/SUMMARY.md` after every step; orphans are checker errors (`sov-ik2`).
Known gaps to close, each inside its owning step:

- G1: duplicate-H1 warnings (2): `README.md` vs `SUMMARY.md`
  (`Soviet Simulator`); `physical-economy/allocation.md` vs
  `planned-economy/allocation.md` (`Allocation`, duplication pointer still
  outstanding from `sov-6uy`).
- G2: `docs/explanation/research/*` listed under the wrong SUMMARY heading
  (placement contradicts layout; both sections checker-covered so any move
  is budget-neutral).
- G3: `docs/agents/*.md` unlisted, headerless, checker-silent (outside
  `WIKI_SECTIONS`); `docs/superpowers/` empty. Decision needed: list under
  a SUMMARY heading + extend checker scope (a `sov-ik2` delta → follow-up
  issue) or move content under an owned section.
- G4: `plan/proposals/*` (advisory) vs `plan/` record pages share one
  heading; target keeps the split explicit (Proposals vs Plan of record).

## 5. Ownership map (reuse only; invent none)

Model owners (from page headers): `project lead` (meta, process entrypoints),
`economy` (simulation economy pages), plus domain owners on specs/concepts.
Slice owners reused from the closed `sov-6pr` wave (Owns-ONLY rows):

| Slice | Owns (closed scope, reusable for its follow-ups) |
|---|---|
| `sov-rut` | templates, `document-authority.md`, `documentation-model.md`, `_writer-brief-common.md`, generator headers |
| `sov-brv` | research-corpus headers, `AGENTS.md` header, SUMMARY research entries, generated ledger |
| `sov-ik2` | `scripts/check_docs.py`, `docs.yml`, `documentation.md` |
| `sov-m7r`/ADR-0001 | charter, scope-1.0, portal index, post-1.0, water/sewage scope lines, design-bible scope rows |
| `sov-6uy` | `planned-economy/**`, `physical-economy/**`, `transport/**`, `simulation/index.md`, `causal-loops.md` |
| `sov-kvn` | `society/**`, `infrastructure/**`, `national-projects/**` |
| `sov-9mz` | architecture pages incl. both substrate maps |
| `sov-bpp` | mechanics-index, invariants, glossary |
| `sov-ipc` | dependencies, dependency-policy, code-intelligence, subagent-tooling, bd-capability-survey |
| `sov-8d1` | design bible (+2 product pointer fixes) |
| `sov-a2p`/`sov-bu6`/`sov-0kc` | fact-sheet rewrite, test-command fixes, determinism pages |

Rule: a migration step touches files of exactly ONE slice owner. Cross-owner
moves split into two steps (pointer first, content second) so each commit
stays attributable and revertible.

## 6. Lifecycle (reuse; invent none)

- Any page with implementation claims carries `Last verified` and
  `Verified-at:` (commit); checker enforces via the `Implementation claims`
  marker (`sov-ik2`). Conceptual pages do not churn dates.
- Archive moves go under `docs/archive/`, keep the body unchanged, index
  entry says *Historical. Not current architecture. Not mechanism
  authority.* Nothing is deleted for obsolescence.
- Generated files (`Kind: generated`, `Authority: derived`) regenerate only;
  hand-edits go to authoritative inputs + generator.
- Scope labels per ADR-0001: exactly `1.0` (charter-row traceable) or
  `Post-1.0`; `candidate` banned. No page binds what the charter does not.

## 7. Migration sequence (each step ends `check_docs` + `mdbook` green)

Each step: ONE owner; SUMMARY + checker-expectation updates ship inside the
same step as the content move (never a dangling pointer commit).

- **S1 — Allocation duplication pointer (owner: `sov-6uy` scope).**
  Files: `docs/simulation/physical-economy/allocation.md` →
  pointer/short stub at the canonical `planned-economy/allocation.md`
  (or the reverse; owner picks, one survives). Budget: SUMMARY target
  unchanged (both entries stay, one becomes the pointer — or one entry
  repointed in the same commit); clears 1 duplicate-H1 warning.
  Validate: `python3 scripts/check_docs.py && mdbook build`.
- **S2 — Root H1 uniqueness (owner: project lead).**
  Files: `README.md` H1 vs `docs/SUMMARY.md` H1 (retitle one; body
  untouched). Budget: no SUMMARY target change; clears 1 warning.
  Validate: same command.
- **S3 — Explanation/research placement (owner: `sov-brv` scope).**
  Files: `docs/explanation/research/*` (5) → `docs/research/*` (or SUMMARY
  heading renamed to match layout; owner picks). Budget: SUMMARY research
  entries rewritten in-step; checker-neutral (both sections wiki-checked).
  Validate: same command.
- **S4 — Mixed-intent splits, economy (owner: `sov-6uy` scope).**
  Files: pages holding `concept` + how-to or current + target without
  section labels (from the review-economy implemented/partial/design-only
  table). Split or add the five-state H2s; new page = SUMMARY entry in-step.
  Validate: same command.
- **S5 — Mixed-intent splits, society/infrastructure (owner: `sov-kvn` scope).**
  Same pattern per the review-society classification table; ADR-0001 scope
  lines (`sov-m7r` scope) re-touched only if the split orphans them —
  otherwise hands off. Validate: same command.
- **S6 — Contributor-path reachability (owner: project lead; G3 decision).**
  Files: `docs/agents/*.md` listed in SUMMARY under Developer guide or
  moved under an owned section; headers added. If checker scope must widen
  to `agents/`, file the `sov-ik2` follow-up FIRST (§8) and land it as its
  own step — never bundle scope-widening with content moves. Empty
  `docs/superpowers/` either gains an index or is removed from the tree.
  Validate: same command.
- **S7 — Proposal/record separation (owner: project lead).**
  Files: SUMMARY headings only — `plan/proposals/*` stays under
  `Proposals (advisory)`; record pages under `Plan of record`; move any
  straggler in-step. Checker-neutral. Validate: same command.
- **S8 — Lifecycle sweep (owners per file, batched by slice).**
  Missing `Verified-at` on implementation-claim pages, stale `Last verified`
  on touched pages, archive-index wording — each file in its slice owner's
  step only. Validate: same command.

Out of scope for this plan (no canonical-docs content changes here):
`open sov-z9x` remainder lines, `open sov-buw` (rev-pin gate), and any
mechanism correction — all owned elsewhere.

## 8. Named follow-up `bd` issues (deltas, never silent drift)

1. `docs-tutorial` — no end-to-end tutorial exists (Diátaxis gap §3); propose
   scope/owner for a Planner-first tutorial.
2. `checker-agents-scope` — `sov-ik2` delta IF S6 lists `docs/agents/*`:
   extend metadata + orphan checks to `agents/` (or record why not).
3. `agents-headers` — `docs/agents/*.md` metadata + owner assignment (needs
   a model-owner decision: project lead or tooling).
4. `allocation-canonical` — only if S1 cannot pick a survivor: decision on
   which allocation page is canonical.
5. `scope-label-sweep` — any residual non-ADR-0001 scope label found during
   S4/S5 (e.g. `candidate`, un-traceable `1.0 binding`); owned by the
   `sov-m7r` scope unless the charter itself must change (new decision).

## Acceptance mapping

- Standards cited: §1 (Diátaxis, Google style, Write the Docs, with sources).
- Audiences inventoried: §2 (developer/contributor/player/modder + gaps).
- Target taxonomy, navigation, ownership, lifecycle implementation-ready:
  §3–§6 (all vocabularies reused, no re-litigation).
- Migration sequence: §7 (one owner + SUMMARY/checker budget + validation
  command per step; every step green).
- No documentation content changed: this file is the only output; all other
  inputs read-only.
