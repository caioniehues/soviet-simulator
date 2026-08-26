# Resume — controlled documentation rewrite, Wave 3 cutover

**Kind:** plan handoff
**Authority:** operational handoff only; `br` remains task-state authority
**Status:** active — final cutover in progress
**Owner:** project lead
**Last verified:** 2026-08-24

## Verified state

Read this after the charter and development cycle, then confirm live state with `br` before taking
work. The parent rewrite issue `sov-docs-controlled-rewrite-m3u` is `in_progress`. Its Wave 3
mapping, requirements, and evidence/roadmap children (`.12`, `.13`, `.14`) are closed at commit
`942c25e`; canonical discovery cutover (`.15`) is the remaining in-progress child.

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
26 passed, 0 failed, 0 filtered; doc-tests 0 (Wave 3 evidence/reviewer gate record on `br` child
`.14`). This is a substrate-regression check, not proof of the 107 target rows.

## Next work

1. Finish issue `.15`: remove old discovery paths, retain the legacy corpus only beneath archive,
   and verify direct local links and active-document metadata.
2. Run Wave 3 final gates in the ordered development cycle: evidence, wiring, conditional ledger,
   reviewer/domain sign-off, doc-reality, and scribe.
3. Re-run the exact serial simulation suite and update `br` with the real gate outputs before
   closing the rewrite parent.
