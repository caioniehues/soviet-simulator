# How to read the docs

**Kind:** guide
**Authority:** operational
**Status:** active
**Owner:** project lead
**Last verified:** 2026-08-28

## The one rule

Five statements look alike and mean different things. Every page here labels which one it is
making; read the label before the sentence.

| The sentence says | It is a claim about | Trust it if |
|---|---|---|
| "The game is about coordination under scarcity" | what the game **is** | it is in `product/` or a concept page — it is direction, not scope |
| "1.0 includes fifteen resources" | what 1.0 **requires** | the charter says it; a `## 1.0 requirement` section cites a `SPEC-*` anchor |
| "Citizens are stored as a dense `CitizenCore`" | what the target architecture **proposes** | it is under `## Target design` or in `architecture/` not marked current — it is not built |
| "`recipe_init` calls `set_requested`" | what the code **implements** | it is under `## Current substrate`, in `architecture/current-substrate.md`, or a fact-sheet, with a path — and check `Last verified` |
| "Soviet unions administered social insurance from 1933" | what research **suggests** | it is under `## Research basis` or in `research/`, with a source and a confidence label |

## Authority in one glance

Charter → glossary → *active* specifications → *accepted* decisions → code and tests → `bd` →
research → synthesis. Higher wins ([document authority](../meta/document-authority.md)). Today
every specification is `draft` and no decision is `accepted`; the binding layer is the charter,
the glossary, the code, and `bd`.

## Where to start for a question

- *Why does this mechanic exist?* → [simulation tree](../simulation/index.md); the concept pages first.
- *What must it do?* → the specification; then its `## Current substrate` for the gap.
- *What does the code do?* → [current substrate](../architecture/current-substrate.md); then the source.
- *What should I build first?* → [migration sequence](../architecture/migration-sequence.md); then `bd ready`.
- *Which mechanic lives where?* → [mechanics index](../reference/mechanics-index.md).
- *Who owns this state?* → [authority index](../reference/authority-index.md).
- *Which rule does this test protect?* → [invariants](../reference/invariants.md).
- *Is this number real?* → only numbers with a source are real; the design thread's illustrative
  tables were removed ([research methodology](../research/methodology.md)).

## Reading a specification

Purpose → scope → exclusions → `SPEC-*` claims → authoritative state → transitions → failure →
observability → acceptance evidence (`EVID-*` rows name the test and the wrong implementation it
must reject) → current substrate → deferred → open questions. A `draft` spec is a proposal for
review; it binds nothing yet.

## Reading an archive page

It is historical. Not current architecture. Not mechanism authority. It may cite paths that no
longer exist; that is evidence of what was, not a broken link to fix.

## Rendering

`mdbook serve` from the repository root renders `docs/` with the curated navigation in
`SUMMARY.md`. GitHub renders the same files. The Markdown is the product; the book is a view.

## Related

- [Front door](../index.md)
- [Documentation model](../meta/documentation-model.md)
- [Documentation standard](../engineering/documentation.md)
