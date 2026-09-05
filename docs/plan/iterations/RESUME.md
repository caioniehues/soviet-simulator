# Resume — controlled documentation rewrite, Wave 3 cutover

**Kind:** plan handoff
**Authority:** operational handoff only; `bd` (beads, replaced `br` 2026-08-26) remains task-state authority
**Status:** cutover complete — commit `b6381a5`; parent closure recorded in `bd`
**Owner:** project lead
**Last verified:** 2026-09-05

> **2026-08-27 — a separate wave is in progress and is NOT part of this documentation plan.**
> The `sov-m0q` tooling evidence epic was worked and cut short. Its handoff, including a P1 crash
> bug (`sov-bo3`) and an ungated branch (`wip/sov-m0q-wave1`), is in
> [`HANDOFF-2026-08-27-tooling-wave.md`](HANDOFF-2026-08-27-tooling-wave.md). Read that before
> taking any tooling, CI, benchmark or renderer work.

> **2026-09-03 — documentation review complete; 16 follow-up beads filed.** Seven-slice review of
> the docs rewrite against code, decomposed into parallel-safe `bd` tasks (label
> `docs-review-2026-09-03`) plus one lead decision (`sov-m7r`). Handoff:
> [`HANDOFF-2026-09-03-docs-review.md`](HANDOFF-2026-09-03-docs-review.md). Read it before taking
> any documentation task.
> **2026-09-04 — border settlement wave 1 built; `sov-5ut` ready.** Doc-sweep wave,
> process reconciliation, checker widening, border grill (ADR-0003), spec `sov-o1w`,
> and the four-bead settlement build all landed. Handoff:
> [`HANDOFF-2026-09-04-border-settlement.md`](HANDOFF-2026-09-04-border-settlement.md).
> Read it before taking any economy, process, or documentation task.

> **2026-09-04 — empty-board run complete.** All 171 beads closed across five parallel waves
> (26 commits, ~7.8k insertions). The durable record: `bd list --status=closed`, the wave
> commits (`git log --grep="wave"`), `docs/generated/evidence/`, and the session retrospectives
> at [`.planning/empty-board-reflection.md`](../../../.planning/empty-board-reflection.md).

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
| `generated/roadmap.md` | 21 requirements, 107 planned targets, 0 implemented | Reporting only; cannot establish scope or task completion |

The legacy 149-story corpus, including stale `RESUME`, scenarios, roadmaps, requirements, extracts,
and generators, is retained unchanged at [`../../archive/iterations/legacy/corpus/`](../../archive/iterations/legacy/corpus/).
It is provenance, not an input to current execution except for identity/title migration accounting.

## Reproduction commands

Run from the repository root. Each `--check` regenerates or validates the stated derived result;
none validates target implementation by itself.

```bash
python3 docs/plan/iterations/requirements/build_requirements.py --check
python3 docs/plan/iterations/extract/build_extract.py --requirements docs/plan/iterations/requirements --specifications docs/reference/specifications --output docs/plan/iterations/extract/requirements.json --check
python3 docs/plan/iterations/evidence/build_evidence.py --extract docs/plan/iterations/extract/requirements.json --specifications docs/reference/specifications --bindings docs/plan/iterations/evidence/evid-spec-bindings.json --output-dir docs/generated/evidence --check
python3 docs/plan/iterations/build_roadmap.py --requirements-dir docs/plan/iterations/requirements --extract docs/plan/iterations/extract/requirements.json --evidence docs/generated/evidence/target-scenarios.json --output docs/generated/roadmap.md --check
```

The most recent recorded exact serial suite is `cargo test -p simulation -- --test-threads=1`:
45 passed, 0 failed, 0 filtered in 43.23s; doc-tests 0. Run at commit `91d9cb4` on 2026-08-27.
This is a substrate-regression check, not proof of the 107 target rows.

The evidence regeneration owed since `7e4b82f` is DONE (2026-08-27). All four generated evidence
artifacts now live together under `docs/generated/evidence/` — `target-scenarios.json`,
`current-regression-inventory.json`, `current-regression-inventory.md` and `coverage.md` — because
`build_evidence.py` writes all four to a single `--output-dir`. Only the two Markdown files had
been moved, which made the documented `--check` above exit 1. The inventory now reports 45
regressions, matching the live suite. Every command in the block above exits 0.

Inputs stay where they are: `build_evidence.py` and `evid-spec-bindings.json` remain under
`docs/plan/iterations/evidence/`. Generated artifacts live under `docs/generated/`; generators and
their inputs live under `docs/plan/`.

## Next work

Live work is tracked in `bd`. **Re-derive the queue with `bd ready`** rather than trusting this
section — it went stale the day it was written last time.

**Historical note (2026-08-27, all items since closed):** `sov-lpj` landed the core loop
(`0caee71`), the hoard panel followed (`sov-hoard-panel-mko`, commit `1051f65`), the five
dispatch-wedge bugs (`sov-jcl`, `sov-xyx`, `sov-abs`, `sov-dii`, `sov-6qx`) and `sov-361`
(warnings fixed — build is clean) are all closed. **`bd ready` is now the only live queue.**
