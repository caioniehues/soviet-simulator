# Documentation model

**Kind:** standard
**Authority:** binding for documentation practice
**Status:** active
**Owner:** project lead
**Last verified:** 2026-08-28

The Markdown files are canonical. `mdBook` renders them (`book.toml` at the repository root,
source tree `docs/`); GitHub and any editor render them equally well. No generator, site or
database is the source of truth.

## Layout

```text
docs/
  index.md            front door — start here
  SUMMARY.md          curated navigation (mdBook); navigation, not authority
  product/            what the game is; 1.0 portal; Post-1.0 direction         (concept, plan)
  simulation/         the design knowledge tree by domain; concepts/; causal loops (concept)
  architecture/       software architecture handbook: current, target, migration (architecture, current-state)
  engineering/        standards: what new code must and should do              (standard)
  developer/          task-oriented guides                                     (guide)
  reference/          glossary, specifications, indexes (mechanics, authority, invariants) (reference, specification)
  decisions/          numbered decisions                                       (decision)
   plan/               charter, proposals, iterations, generated inputs         (plan)
   vision/             advisory explanatory design synthesis                     (explanation)
   process/            how work gets done; policies                             (process)
  research/           evidence: lane reports, fact-sheets, crate surveys       (research, current-state)
  generated/          derived status; regenerate only                          (generated)
  templates/          authoring templates
  meta/               this model, the authority page, the audit
  archive/            historical; provenance only
```

Existing authoritative files were not moved to fit this layout; the navigation layer and the
index pages do the organising. [Original authority map](original-authority-map.md) remains a
retained reference.

## Page types

One page, one type. Split a page that grows a second purpose.

| Kind | Answers | Examples |
|---|---|---|
| `concept` | Why does this exist and how does it behave in the design? | [reliability](../simulation/concepts/reliability.md), [storming](../simulation/planned-economy/storming.md) |
| `architecture` | How do the parts fit, now and as targeted? | [simulation phases](../architecture/simulation-phases.md) |
| `current-state` | What does the repository implement today, cited? | [current substrate](../architecture/current-substrate.md) |
| `specification` | What MUST a subsystem do? | `reference/specifications/*.md` |
| `decision` | What was decided, why, and what it replaced? | `decisions/` |
| `standard` | What must or should new code and docs do? | `engineering/*.md` |
| `guide` | How do I perform this task? | `developer/*.md` |
| `reference` | Precise facts, tables, indexes | glossary, mechanics index |
| `research` | What does the evidence say, with confidence? | lane reports, fact-sheets |
| `plan` | What is the sequence or the scope? | charter, migration sequence |
| `index` | What belongs in this section and how do I read it? | every `index.md` |

Four reader intents stay separate: explanation, reference, how-to, tutorial. A `concept` page
does not carry a how-to; a `guide` does not re-explain the design.

## Metadata header

Directly under the H1:

```markdown
**Kind:** concept
**Authority:** advisory
**Status:** draft | active | accepted | superseded | archived
**Owner:** <code area, process, or role>
**Last verified:** YYYY-MM-DD
```

Add `**Verified-at:** <commit>` on any page with implementation claims. Add `**Scope:**` only
when it says something the section does not already say. Do not invent owners.

`CONFIRMED`, `PLAUSIBLE`, `HYPOTHESIS`, `UNSUPPORTED`, `OBSERVED`, `INFERRED`, `SPECULATIVE`,
`OURS` are evidence labels inside a page; they never replace the header.

## Section labels for the five states

Where a page holds more than one state of knowledge, use these H2s:

```text
## What this is
## 1.0 requirement
## Target design
## Current substrate
## Research basis
## Future direction
## Open questions
## Related
```

## Linking

- Relative links only inside the repository.
- Explain a cross-cutting idea once, on its concept page; every other page links to it and adds
  only the domain-specific mechanics.
- Link to a specification anchor (`logistics.md#spec-logistics-005`), not to a line number.
- End substantial pages with `## Related` — four to eight links, curated.
- No line numbers in conceptual pages. Line numbers live in `current-state` and `research` pages
  and rot with the code; paths and symbol names age better.

## Filenames

Stable nouns: `enterprise-behavior.md`, `physical-causality.md`. No versions, no dates in
conceptual filenames. Dates belong to research reports, handoffs and snapshots.

## Validation

`python3 scripts/check_docs.py` checks broken relative links, `SUMMARY.md` targets that do not
exist, active wiki pages not reachable from `SUMMARY.md`, duplicate H1 titles, and metadata on
specifications and wiki pages. `mdbook build` must succeed. CI runs both
(`.github/workflows/docs.yml`).

## Related

- [Document authority](document-authority.md)
- [Documentation audit](documentation-audit.md)
- [Documentation standard](../engineering/documentation.md) — the engineering rule for keeping docs in step with code
- [How to read the docs](../developer/how-to-read-the-docs.md)
