# Documentation standard

**Kind:** standard
**Authority:** operational
**Status:** active
**Owner:** project lead
**Last verified:** 2026-08-28

This project's signature failure is documents asserting things the code does not do
(`development-cycle.md` lists the incidents). These rules exist to stop it.

## Rules

1. **Must — inspect before asserting.** "The simulation uses X" requires opening the source. "This
   is tested" requires opening the test. "This is complete" requires implementation, evidence,
   the specification's acceptance rows and `bd`.
2. **Must — keep the five states apart.** What the game is; what 1.0 requires; what the target
   architecture proposes; what the code implements; what research suggests. Label sections when a
   page holds more than one ([document authority](../meta/document-authority.md)).
3. **Must — update the current-substrate page** ([`architecture/current-substrate.md`](../architecture/current-substrate.md))
   in the same change that materially alters a documented contract: a system added or reordered,
   a resource added, an authority moved, a save layout changed, a pillar violation fixed. Bump its
   `Verified-at` and `Last verified`.
4. **Must — cite paths and symbols** in conceptual pages; line numbers only in current-state and
   research pages, and only ones you read in the session you wrote them.
5. **Must — MUST/SHALL/SHOULD only in specifications and standards.** Design and research pages say
   "proposes", "suggests", "indicates".
6. **Must — every substantial page has the metadata header** (Kind, Authority, Status, Owner, Last
   verified) and, when it makes implementation claims, `Verified-at`.
7. **Must — links are relative**; one canonical page per cross-cutting idea; every other page
   links rather than re-explains.
8. **Must — `SUMMARY.md` is curated.** A new page is added where a reader would look for it; not
   every artefact belongs in the primary path (archive and research indexes exist for the rest).
9. **Must — `python3 scripts/check_docs.py` and `mdbook build` pass** before a documentation change
   lands (CI runs both).
10. **Should — a Phase 6 `doc-reality-auditor` sweep** follows any substantive wave; its findings
    are dispositioned, not archived.
11. **Must not — fossilise intent as fact.** A target described in the present tense without a
    "target" label is a defect.

## Where things go

| You are writing | Put it in |
|---|---|
| Why a mechanic exists | `docs/simulation/<domain>/` (concept) |
| A contract a subsystem must meet | `docs/reference/specifications/` (use the template) |
| How the software is or should be built | `docs/architecture/` |
| A rule for new code | `docs/engineering/` |
| A how-to | `docs/developer/` |
| Evidence from history or measurement | `docs/research/` with the evidence classes |
| A decision awaiting the Planner | `docs/plan/proposals/`; once accepted, `docs/decisions/` |
| A superseded document | `docs/archive/` with the historical banner |

## Related

- [Documentation model](../meta/documentation-model.md)
- [Document authority](../meta/document-authority.md)
- [Adding a specification (guide)](../developer/adding-a-specification.md)
- [Development cycle — Phase 6](../process/development-cycle.md)
