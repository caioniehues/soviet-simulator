# Documentation standard

**Kind:** standard
**Authority:** operational
**Status:** active
**Owner:** project lead
**Last verified:** 2026-08-28

This project's signature failure is documents asserting things the code does not do
(`development-cycle.md` lists the incidents). These rules exist to stop it.

## Rules

Enforcement: rules marked **mechanical** are enforced by `python3 scripts/check_docs.py`
(same command CI runs) or `mdbook build`; rules marked **review-only** need a human
reviewer — the checker does not enforce them. Mechanical: 6 (metadata header,
`Verified-at` via the `Implementation claims` marker), 8 (orphan / `SUMMARY.md`
reachability), 9 (checker + build pass). Review-only: 1–5, 7, 10–11.

1. **Must — inspect before asserting** *(review-only)*. "The simulation uses X" requires opening the source. "This
   is tested" requires opening the test. "This is complete" requires implementation, evidence,
   the specification's acceptance rows and `bd`.
2. **Must — keep the five states apart** *(review-only)*. What the game is; what 1.0 requires; what the target
   architecture proposes; what the code implements; what research suggests. Label sections when a
   page holds more than one ([document authority](../meta/document-authority.md)).
3. **Must — update the current-substrate page** *(review-only)* ([`architecture/current-substrate.md`](../architecture/current-substrate.md))
   in the same change that materially alters a documented contract: a system added or reordered,
   a resource added, an authority moved, a save layout changed, a pillar violation fixed. Bump its
   `Verified-at` and `Last verified`.
4. **Must — cite paths and symbols** *(review-only)* in conceptual pages; line numbers only in current-state and
   research pages, and only ones you read in the session you wrote them.
5. **Must — MUST/SHALL/SHOULD only in specifications and standards** *(review-only)*. Design and research pages say
   "proposes", "suggests", "indicates".
6. **Must — every substantial page has the metadata header** *(mechanical)* (Kind, Authority, Status, Owner, Last
   verified) and, when it makes implementation claims, `Verified-at`. The checker requires the
   five-field header on every specification, every page under the wiki sections (including
   `research/` and `explanation/`), and the root entry points (`README.md`, `AGENTS.md`,
   `CLAUDE.md`, `CONTEXT.md`). A page declares implementation claims with an
   `**Implementation claims:** yes` line in its first 30 lines; such a page must also carry a
   non-empty `**Verified-at:**` line (commit sha, optionally with a scope note) in its first
   30 lines.
7. **Must — links are relative** *(review-only: link existence is mechanical, single-canonical-page
   judgement is review-only)*; one canonical page per cross-cutting idea; every other page
   links rather than re-explains.
8. **Must — `SUMMARY.md` is curated** *(mechanical: orphan reachability; review-only: placement)*. A new page is added where a reader would look for it; not
   every artefact belongs in the primary path (archive and research indexes exist for the rest).
   Every active page under the wiki sections (except specifications, which are navigated
   through their own index) must be reachable from `SUMMARY.md` — unreachable pages fail the checker.
9. **Must — `python3 scripts/check_docs.py` and `mdbook build` pass** *(mechanical)* before a documentation change
   lands (CI runs both).
10. **Should — a Phase 6 `doc-reality-auditor` sweep** *(review-only)* follows any substantive wave; its findings
    are dispositioned, not archived.
11. **Must not — fossilise intent as fact** *(review-only)*. A target described in the present tense without a
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
