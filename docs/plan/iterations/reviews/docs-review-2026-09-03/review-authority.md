# Review: authority

Verdict: incorrect (confidence 0.97)

The authority rewrite preserves the four model pillars and all SUMMARY targets resolve, but it leaves a CI false negative for active research reports, removes AGENTS.md metadata, omits a new research page from navigation, and keeps conflicting authority/lifecycle conventions in templates and substrate maps. Dated active snapshots still name the deleted docs/README path; archive-only references are correctly historical.

## Findings

### [high] Extend metadata enforcement to the active research corpus
`docs/research/conversation-mining-2026-08-28/A-economy-control-loop.md:1-3`

The ten lane reports linked from docs/SUMMARY.md begin with an H1 and no Kind/Authority/Status/Owner/Last verified block (for example docs/research/conversation-mining-2026-08-28/A-economy-control-loop.md:1-3 and B1-society-households.md:1-3). `scripts/check_docs.py:27-38,133-143` excludes research from its required set, so its observed run reports 228 active files and 0 errors despite these active pages violating docs/meta/documentation-model.md:64-70 and docs/meta/original-authority-map.md:42-44. Add headers to the active reports and include research in the checker, or move them to an explicitly historical/private location.

### [medium] Restore the metadata block on the AGENTS entrypoint
`AGENTS.md:1-8`

The rewritten AGENTS.md starts with `# Repository Guidelines` and immediately enters `## Project Overview` at lines 1-8, so the operational root entrypoint has no Kind/Authority/Status/Owner/Last verified fields. This contradicts docs/meta/document-authority.md:32-34 and docs/meta/documentation-model.md:64-70, while docs/index.md:68 sends readers to AGENTS.md; the checker includes root files only for links/titles (`scripts/check_docs.py:29-38,133-143`) and cannot catch the regression. Restore the five-line operational header and check root entrypoints.

### [medium] Add the new Beads research snapshot to the curated tree
`docs/explanation/research/beads-oh-my-pi-integration-2026-08-30.md:1-8`

The substantive non-underscore page docs/explanation/research/beads-oh-my-pi-integration-2026-08-30.md:1-8 is absent from the Research entries in docs/SUMMARY.md:219-245. `SUMMARY.md` is the curated navigation layer (`docs/meta/documentation-model.md:17-18`), but `scripts/check_docs.py:27-38,104-112` does not inspect explanation/ for orphans, so the checker does not report this omission. Add the page to the Research tree (or move it under docs/research/) and extend orphan checking to the chosen active explanation/research roots.

### [medium] Align the research template with the authority taxonomy
`docs/templates/research.md:1-7`

The changed docs/templates/research.md:1-7 prescribes Kind: explanation and Authority: explanatory, but docs/meta/document-authority.md:30-38 defines binding/operational/advisory/reference/observational/research/historical/derived and no explanatory label. The page-type model has research as its own kind (`docs/meta/documentation-model.md:49-56`) and the writer brief prescribes concept/current-state/research/index (`docs/meta/_writer-brief-common.md:62-71`); active pages split between explanation/explanatory (`docs/research/conversation-mining-2026-08-28/SYNTHESIS.md:3-8`) and research/research (`docs/research/engineering/rust-architecture-crates.md:3-8`). Choose one supported taxonomy and update the template, model, and pages together.

### [medium] Make generated-page metadata agree with its template
`docs/templates/generated.md:1-7`

The changed docs/templates/generated.md:1-7 prescribes Kind: generated, Authority: derived, Status: active, but the maintained outputs say Kind: generated roadmap/evidence coverage, Authority: reporting only, Status: draft (`docs/generated/roadmap.md:3-8`, `docs/generated/evidence/coverage.md:3-8`), and build_roadmap.py emits the former values at lines 60-65. The authority model only defines derived for generated files (`docs/meta/document-authority.md:37-38`), so copying the template and regenerating produce competing lifecycle/authority claims. Make the template, generator, and authority model use one set of values.

### [medium] Place the current-substrate page explicitly in the hierarchy
`docs/meta/document-authority.md:16-23`

The hierarchy names source/tests at rank 5, research plus reference/architecture/substrate.md at rank 7, and target/design pages including architecture/target-architecture.md and siblings at rank 8 (`docs/meta/document-authority.md:16-23`), but omits architecture/current-substrate.md even though it is explicitly the current-code state at `docs/meta/document-authority.md:47-49,71-73` and declares Authority: observational (`docs/architecture/current-substrate.md:3-8`). CLAUDE.md points to the reference map at line 36 while lines 42-48 and docs/index.md:58-60 point to current-substrate, and AGENTS.md:82-84 points only to the reference map. Add both paths and their precedence/division to the hierarchy so current evidence cannot be mistaken for advisory target design.

### [low] Retire or update active snapshots that name deleted docs/README.md
`docs/SUMMARY.md:196-202`

The deleted path is still named in pages surfaced in the active tree: doc-audit calls docs/README.md the entrypoint (`docs/process/doc-audit-2026-08-26.md:42-60,186-190`), doc-survey lists it as the active docs root (`docs/process/doc-survey-2026-08-26.md:75-80`), and the F report calls it the metadata source (`docs/research/conversation-mining-2026-08-28/F-doc-overlap-and-consolidation.md:199-209,440-444`); SUMMARY links those pages at lines 196-202 and 230-243. These are prose references, not broken Markdown links, while superseded/archived matches are intentionally historical. Mark the dated snapshots superseded/historical or replace live pointers with docs/index.md and docs/meta/document-authority.md without editing archive bodies.
