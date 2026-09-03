# Document authority

**Kind:** standard
**Authority:** binding for documentation practice
**Status:** active
**Owner:** project lead
**Last verified:** 2026-08-28

A page's presence in this knowledge base does not make it true, binding, or implemented. This
page says which documents may establish what. Every other page defers to it.

## The hierarchy

| Rank | Source | Establishes | Cannot establish |
|---|---|---|---|
| 1 | [`docs/plan/charter-1.0.md`](../plan/charter-1.0.md) | Product scope: what 1.0 includes, the Post-1.0 cuts, the Never list, the 250k target | Mechanism, implementation status |
| 2 | [`docs/reference/glossary.md`](../reference/glossary.md) | Terminology: the one name for each thing and the words to avoid | Behaviour, acceptance criteria |
| 3 | [`docs/reference/specifications/`](../reference/specifications/README.md) with `Status: active` | Subsystem mechanism inside charter scope: `SPEC-*` claims, authoritative module, acceptance evidence | Scope; completion. **Every specification is currently `Status: draft`** — a draft records the proposed contract for review and binds nothing yet |
| 4 | [`docs/decisions/`](../decisions/README.md) with `Status: accepted` | Architectural and engineering decisions | Scope or mechanism outside the decision's stated question. **No accepted decision exists yet** |
| 5 | Source code and tests | What is implemented; what is tested | What is intended, in scope, or complete |
| 6 | `bd` (beads) | Task state: what is open, claimed, closed, blocked | Anything about design |
| 7 | Research and fact-sheets (`docs/research/`, `docs/reference/architecture/substrate.md`) | Observed evidence, cited to `path:line` or an external source at a stated date | Scope, mechanism, or completion — evidence is an input to those, never a substitute |
| 8 | Synthesis and design guidance (`docs/product/`, `docs/simulation/`, `docs/architecture/target-architecture.md` and its siblings) | Consolidated design direction and target architecture, labelled by evidence class | Anything binding. These pages are `Authority: advisory` |

A page never silently outranks a source above it. When a wiki page and a higher source disagree,
the higher source wins and the wiki page is wrong; fix the page.

## Authority labels on pages

Every substantial page carries an `Authority:` line:

- `binding` — charter, glossary, active specifications, accepted decisions, this page.
- `operational` — process and entry points (`CLAUDE.md`, `AGENTS.md`, `docs/process/`): how work is done here.
- `advisory` — design, architecture guidance, proposals: read, weigh, do not cite as a requirement.
- `reference` / `observational` — current-state and fact-sheet pages: true at `Last verified` for the cited paths.
- `research` — evidence and interpretation; never a mechanic by itself.
- `historical` — archive; not current architecture, not mechanism authority.
- `derived` — generated files; regenerate, never edit.

## The five states of knowledge

Documentation here keeps five statements apart, and labels sections when a page holds more than one:

```text
WHAT THE GAME IS                 → docs/product/, docs/simulation/concepts/
WHAT 1.0 REQUIRES                → the charter; "## 1.0 requirement" sections; docs/product/scope-1.0.md
WHAT THE TARGET ARCHITECTURE PROPOSES → docs/architecture/target-architecture.md and siblings; "## Target design"
WHAT THE CURRENT CODE IMPLEMENTS → docs/architecture/current-substrate.md; "## Current substrate"; fact-sheets
WHAT RESEARCH SUGGESTS           → docs/research/; "## Research basis"
```

A sentence of the form "the simulation uses X" is a current-substrate claim and needs a source
path. A sentence of the form "enterprises hoard" is either a research claim (cite the evidence) or
a design proposal (label it); it is never a rule unless a specification says so.

## Normative language

`MUST`, `SHALL`, `SHOULD`, `MAY` in capitals belong to specifications and engineering standards
that declare the RFC 8174 convention. Design and research pages do not use them. "The design
proposes", "the thread suggests", "historical research indicates" are the right registers.

## Archive policy

A document that no longer describes current authority moves under `docs/archive/` and keeps its
body unchanged. Its index entry says *Historical. Not current architecture. Not mechanism
authority.* Nothing is deleted for being obsolete; provenance is evidence.

## Freshness

Conceptual pages do not churn dates. Any page that makes an implementation claim carries
`Last verified` and, where practical, a `Verified-at:` commit. The
[current substrate](../architecture/current-substrate.md) page is the one whose date matters most;
the [documentation standard](../engineering/documentation.md) says who updates it and when.

## Related

- [Documentation model](documentation-model.md) — page types, metadata, layout
- [Documentation audit](documentation-audit.md) — scope and validation baseline
- [Original authority map](original-authority-map.md) — retained reference
- [Specification register](../reference/specifications/README.md)
- [Decision register](../decisions/README.md)
