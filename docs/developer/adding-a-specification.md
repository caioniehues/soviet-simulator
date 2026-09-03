# Adding or revising a specification

**Kind:** guide
**Authority:** operational
**Status:** active
**Owner:** project lead
**Last verified:** 2026-08-28

## When a specification is the right document

You are stating what an in-scope subsystem **must** do. Not why (concept page), not how the
software is shaped (architecture), not what the code does (current substrate).

## Steps

1. **Confirm scope.** The charter must include the area; a Post-1.0 cut cannot receive a spec with
   1.0 acceptance criteria.
2. **Start from the template:** `docs/templates/specification.md`. Keep the structure the register
   requires: purpose, scope, exclusions, invariants, authoritative state, transitions, failure
   behaviour, observability, acceptance evidence, current substrate, deferred behaviour, open
   questions.
3. **Anchors.** Number claims `SPEC-<SUBSYSTEM>-NNN` monotonically; never reuse a retired id.
   Evidence rows are `EVID-<SUBSYSTEM>-NNN` and name the test (`cargo test -p simulation
   evid_<subsystem>_<claim>`) **and the deliberately wrong implementation the test must reject**.
4. **Authority.** Name the one module that owns each transition; reference other modules' IDs
   and results, never a parallel copy. Check the [authority index](../reference/authority-index.md).
5. **Current substrate.** Cite a fact-sheet anchor plus a source location for every present-tense
   claim. If the fact-sheet is stale, say so and cite the commit that changed things.
6. **Normative language.** `MUST`/`SHALL`/`MAY` in capitals, in this document type only.
7. **Status.** New specs are `draft`. `active` only after ordered review findings are fixed,
   accepted or filed. A replaced spec becomes `superseded` with a link to its successor.
8. **Register.** Add it to `docs/reference/specifications/README.md`, `docs/SUMMARY.md`, the
   [mechanics index](../reference/mechanics-index.md) and, if it introduces an invariant, the
   [invariants index](../reference/invariants.md).
9. **Validate.** `python3 scripts/check_docs.py` checks the metadata block and links; `mdbook build`.

## Missing 1.0 specifications (as of 2026-08-28)

Agriculture and livestock; terrain, geology and ore; weather and seasons; hydrology, reservoir and
hydro; pollution; the Plan / Quota / Tranche macro loop; authored Plans and onboarding;
notifications and event log; shell, save, autosave and crash recovery; presentation and audio
acceptance. Each is a scope-critical gap ([1.0 scope](../product/scope-1.0.md)).

## Related

- [Specification register](../reference/specifications/README.md)
- [Specification template](../templates/specification.md)
- [Documentation standard](../engineering/documentation.md)
- [Writing evidence tests](writing-evidence-tests.md)
