# Soviet Simulator documentation

**Kind:** reference
**Authority:** operational
**Status:** active
**Owner:** project lead
**Last verified:** 2026-08-24

This directory is the canonical home for human project documentation. It does not make a
claim true merely by containing it. Use the authority map below before relying on a document.

## Authority map

| Question | Canonical source | What it may establish |
|---|---|---|
| What 1.0 includes or excludes | [1.0 charter](plan/charter-1.0.md) | Binding scope and cut line |
| What project words mean | [Glossary](reference/glossary.md) | Terminology only |
| What the current fork actually provides | [Substrate architecture](reference/architecture/substrate.md) and its cited fact-sheets | Observed implementation reality |
| How an in-scope system must behave | `reference/specifications/` | Binding mechanisms after ratification |
| What is being worked on now | `br` | Task state only |

Authority flows downward: charter, glossary, and observed substrate constrain specifications;
specifications constrain requirements; requirements constrain scenarios and generated status.
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

The controlled rewrite is still in progress. New canonical files may coexist temporarily with
legacy paths until the final discovery-path cutover; the cutover must name one source for each
question in this map.
