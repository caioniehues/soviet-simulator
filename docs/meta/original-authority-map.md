# Original authority map

**Kind:** reference
**Authority:** operational
**Status:** active
**Owner:** project lead
**Last verified:** 2026-08-24

This retained map describes the documentation authority model. Start at the
[documentation home](../index.md) and navigate with [SUMMARY.md](../SUMMARY.md).
The full hierarchy, page types, and five states of knowledge are in
[Documentation authority](document-authority.md) and
[Documentation model](documentation-model.md).

## Authority map

| Question | Canonical source | What it may establish |
|---|---|---|
| What 1.0 includes or excludes | [1.0 charter](../plan/charter-1.0.md) | Binding scope and cut line |
| What project words mean | [Glossary](../reference/glossary.md) | Terminology only |
| What the current fork actually provides | Source code and tests; use [Current substrate](../architecture/current-substrate.md) as a cited guide | Implementation and test reality |
| How an in-scope system must behave | `reference/specifications/` | Binding mechanisms after ratification |
| What is being worked on now | `bd` | Task state only |

The charter constrains scope. The glossary constrains terminology. Active specifications constrain
mechanism within scope. Source and tests establish implementation reality. Research and fact-sheets
provide evidence but do not establish scope or mechanism.
Generated files, handoffs, archived material, and pre-fork ADRs cannot establish upstream scope or
mechanism.

## Reading paths

- Start a domain change with the glossary, the charter, and the applicable substrate fact-sheet.
- Read a specification only after confirming that its scope is in the charter.
- Treat `decisions/` as the register of ratified decisions; its index explains the pre-fork ADR
  archive.
- Use `process/` for repeatable work and `templates/` when authoring a document.
- Read `archive/` for provenance, never as current direction.

## Document metadata

Every active document declares its **Kind**, **Authority**, **Status**, **Owner**, and **Last
verified** date. `CONFIRMED`, `OBSERVED`, `INFERRED`, `SPECULATIVE`, and `OURS` remain
evidence labels inside a document; they do not replace the metadata above.

The canonical paths above are the only active discovery paths. The legacy corpus is retained under
[`archive/iterations/legacy/`](../archive/iterations/legacy/) for provenance and is not a second plan.
