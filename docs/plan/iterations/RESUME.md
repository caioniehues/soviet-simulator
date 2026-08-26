# Resume — controlled documentation rewrite, Wave 3 cutover

**Kind:** plan handoff
**Authority:** operational handoff only; `bd` (beads, replaced `br` 2026-08-26) remains task-state authority
**Status:** cutover complete — commit `b6381a5`; parent closure recorded in `bd`
**Owner:** project lead
**Last verified:** 2026-08-26

## Verified state

Read this after the charter and development cycle, then confirm live state with `bd` before taking
work. The Wave 3 cutover landed at commit `b6381a5` (documentation tree migrated, nine agent
definitions repointed). One latent gap was found and fixed 2026-08-26: `target-scenarios.json` —
a required input of the evidence `--check` and `build_roadmap` — was silently ignored by the
inherited `target*` gitignore glob and never committed; the glob is now dir-only (`target/`) and
the file is versioned. Task history lives on `sov-docs-controlled-rewrite-m3u` and its children.

Wave 1 and Wave 2 are closed documentation waves. Their specifications remain draft and unratified:
the re-derived target guards are deliberately `UNIMPLEMENTED`, so no documentation status claims
target runtime behavior is shipped.

## Re-derived planning corpus

Commit `942c25e` established the current planning inputs and reporting output:

| Artifact | Current verified result | Authority boundary |
|---|---|---|
| `requirements/` plus `story-migration.md` | 149 migrated STORY rows; 21 live re-derived requirements | Requirements derive scope from the charter and mechanism references from stable `SPEC-*` anchors |
| `extract/requirements.json` | 21 extracted requirements | Structured input only; schema validity is not target proof |
| `evidence/` | 107 planned `EVID-*` targets, 0 implemented; 26 current regressions separate | Target evidence is not current code evidence until its guard exists and mutation proof is recorded |
| `generated/iterations/roadmap.md` | 21 requirements, 107 planned targets, 0 implemented | Reporting only; cannot establish scope or task completion |

The legacy 149-story corpus, including stale `RESUME`, scenarios, roadmaps, requirements, extracts,
and generators, is retained unchanged at [`../../archive/iterations/legacy/corpus/`](../../archive/iterations/legacy/corpus/).
It is provenance, not an input to current execution except for identity/title migration accounting.

## Reproduction commands

Run from the repository root. Each `--check` regenerates or validates the stated derived result;
none validates target implementation by itself.

```bash
python3 docs/plan/iterations/requirements/build_requirements.py --check
python3 docs/plan/iterations/extract/build_extract.py --requirements docs/plan/iterations/requirements --specifications docs/reference/specifications --output docs/plan/iterations/extract/requirements.json --check
python3 docs/plan/iterations/evidence/build_evidence.py --extract docs/plan/iterations/extract/requirements.json --specifications docs/reference/specifications --bindings docs/plan/iterations/evidence/evid-spec-bindings.json --output-dir docs/plan/iterations/evidence --check
python3 docs/plan/iterations/build_roadmap.py --requirements-dir docs/plan/iterations/requirements --extract docs/plan/iterations/extract/requirements.json --evidence docs/plan/iterations/evidence/target-scenarios.json --output docs/generated/iterations/roadmap.md --check
```

The most recent recorded exact serial suite is `cargo test -p simulation -- --test-threads=1`:
26 passed, 0 failed, 0 filtered; doc-tests 0 (Wave 3 evidence/reviewer gate record on `bd` child
`.14`; re-confirmed at clean HEAD `f89bc3b` on 2026-08-26 — see the parent close reason for the
real output). This is a substrate-regression check, not proof of the 107 target rows.

Note: while `sov-dispatch-wedge-ab4` work is uncommitted in the working tree, the evidence
`--check` diverges there by design (the wedge adds simulation tests); regeneration of
`current-regression-inventory.json` and the roadmap is owed at that issue's Phase 6 wrap.

## Next work

Wave 3 and the rewrite parent are closed. Live work is tracked in `bd` (`bd ready`); as of
2026-08-26 the front of the queue is `sov-dispatch-wedge-ab4` (P1, round 4 in flight) and
`sov-hoard-panel-mko` (Phase 0 fact-sheet in flight).
