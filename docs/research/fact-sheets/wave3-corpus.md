# Wave 3 plan-corpus and discovery fact-sheet

**Kind:** reference
**Authority:** reference
**Status:** superseded — the rewrite it describes has shipped, cutover commit `b6381a5`; see `docs/plan/iterations/RESUME.md`
**Owner:** architecture
**Last verified:** 2026-08-24
**Commit:** `15b97ca`

This Phase 0 fact-sheet records the current derived planning corpus and discovery paths. It does
not preserve a legacy claim, ratify a specification, promote a scenario, or establish target
behaviour. Wave 3 must re-derive the active corpus from the authority direction below.

## Authority direction

```text
binding charter + binding glossary + current code/fact-sheets
                         ↓
                   draft/ratified SPEC anchors
                         ↓
                  rewritten requirements and evidence
                         ↓
              validated extract and generated roadmap
```

The charter binds only scope; current code and fact-sheets describe present behaviour; ratified
specifications bind an in-scope mechanism; `bd` owns task state
(`docs/plan/charter-1.0.md:9-19`, `docs/reference/specifications/README.md:15-18`). A draft does
not establish completion or override the charter (`docs/reference/specifications/README.md:9-11`).
Requirements and scenarios must cite stable `SPEC-*` anchors rather than mutable lines
(`docs/reference/specifications/README.md:49-56`). Generated output and RESUME are downstream
reporting surfaces, never authority upstream of requirements.

## Current corpus inventory

| Surface | Current count/state | Direct evidence | Wave 3 disposition |
|---|---|---|---|
| Legacy requirements | 36 EPIC files, **149** STORY headings, **370** `[SUBSTRATE:]` tags | Archived generator reads old STORY blocks and scenario tags (`docs/archive/iterations/legacy/corpus/build_roadmap.py:78-91`) | Re-derive every story against charter scope and stable SPEC claims. Retain an ID only if its meaning survives; map every old ID to retained, rewritten, split, deferred, or retired. |
| Extract | 8 JSON inputs; validator checks structural fields and legacy tag syntax | `docs/archive/iterations/legacy/corpus/extract/validate.py:9-57` | Re-extract from rewritten requirements. Schema success is not semantic, command, or cross-file corpus-ID proof. |
| Scenarios and corpus | **153** rows: 1 JOURNEY and 152 SCENARIO entries; every command is **TBD** | `docs/archive/iterations/legacy/corpus/behavior-scenarios.md:3-9`; `docs/archive/iterations/legacy/corpus/behavior-corpus.md:3-18` | Rebuild cards, corpus, and coverage with executable bindings. Do not move the old corpus as evidence. |
| Sentinels | **6** promoted sentinels, all `TBD` | `docs/archive/iterations/legacy/corpus/behavior-corpus.md:11-18` | No sentinel is promotable until its command runs at least one test. |
| Coverage ledger | 23 rows: 22 `covered`, 1 `non-normative`; all cite root `spec/` paths | `docs/archive/iterations/legacy/corpus/coverage-ledger.md:6-32` | Rebuild from rewritten requirement-to-SPEC-to-evidence links; labels are not proof of current coverage. |
| Roadmap/generator | Parser recognizes deferred markers but emitted header is hard-coded to “plus 1 deferred” | `docs/archive/iterations/legacy/corpus/build_roadmap.py:82-99`, `docs/archive/iterations/legacy/corpus/build_roadmap.py:188-194`; false archived header at `docs/archive/iterations/legacy/corpus/roadmap.md:1-3` | Replace generator around explicit requirement metadata; validate totals and reproduce the new roadmap byte-for-byte. |
| RESUME | Reports 151 scenarios/5 sentinels despite the 153-row/6-sentinel corpus; calls truck work in flight despite current SmallTruck registration and real dispatch | stale counts at `docs/archive/iterations/legacy/corpus/RESUME.md:12-20`; stale truck narrative at `docs/archive/iterations/legacy/corpus/RESUME.md:73-85`; current code at `simulation/src/map_dynamic/dispatch.rs:95-104`, `simulation/src/economy/market.rs:462-609` | Reconstruct from current `bd`, verified commits, executed commands, and regenerated counts; copy no narrative status forward. |

The exact 149/370/153/6/TBD inventory was counted from the current files on 2026-08-24. The
legacy RESUME's 130-scheduled/19-deferred statement conflicts with the generator output's
hard-coded one-deferred header (`docs/archive/iterations/legacy/corpus/RESUME.md:14-20`,
`docs/archive/iterations/legacy/corpus/roadmap.md:1-3`). This is a generator defect, not a license to treat
RESUME as scope authority.

## Runnable-test identity and corpus collisions

Phase 0's serial test-list observation contains **26** runnable `simulation` tests. It is a
substrate inventory, not a runnable corpus: the scenario module claims function names carry stable
corpus IDs and describes six sentinels (`simulation/src/tests/scenarios/mod.rs:1-13`), but none of
the six legacy sentinel IDs names a corresponding executable sentinel function. Legacy IDs also
collide with tests that prove different behaviour:

| Legacy corpus ID/meaning | Current test identity/meaning | Consequence |
|---|---|---|
| 0082 mine extraction; 0083 output storage | `scenario_0082_dispatch_gates_stock_not_match`; `scenario_0083_zero_trucks_blocks_delivery` | Same ID, different contract (`docs/archive/iterations/legacy/corpus/behavior-corpus.md:98-99`, `simulation/src/tests/scenarios/hoarding.rs:96-145`). |
| 0093–0097 combustion/storage/transport/water/bottleneck | Recipe tests cover different multi-input, extraction, storage, workforce, and treasury guards | Same IDs cannot be imported as proof (`docs/archive/iterations/legacy/corpus/behavior-corpus.md:108-112`, `simulation/src/tests/scenarios/recipe_provided.rs:43-230`). |
| 0151 four dispatch states | `scenario_0151_inflated_request_hoards_honest_does_not` | Same ID, different contract (`docs/archive/iterations/legacy/corpus/behavior-corpus.md:164`, `simulation/src/tests/scenarios/hoarding.rs:189-190`). |

Existing tests may later be retained as named legacy guards only after binding to rewritten
requirements, SPEC anchors, and mutation-proven evidence. `TestCtx`'s encode/decode-and-hash check
is serialization round-trip stability, not repeat-run determinism (`simulation/src/tests/mod.rs:87-121`).

## Discovery and migration obligations

The Phase 0 stale-discovery finding is resolved by canonical discovery: AGENTS points to the
current process and RESUME (`AGENTS.md:11-15`); CLAUDE points to the current process
(`CLAUDE.md:21-26`); and the process document points to the charter and current RESUME
(`docs/process/development-cycle.md:13-14`, `docs/process/development-cycle.md:58-63`). The
archived legacy corpus remains provenance, not an alternate authority.

Wave 3 must proceed in this order:

1. Write story migration and re-derived requirements before extract, evidence, or roadmap.
2. Re-extract and validate structured data, then rebuild executable evidence and a total-validating generator.
3. Reconstruct RESUME from `bd`, verified commits, executed commands, and regenerated counts.
4. Cut root discovery, process, art-direction, agent definitions, and approved research/plan paths together.
5. Prove no active old-path reference remains, every live requirement resolves to charter and SPEC, every promoted scenario runs a nonzero test, and generated output reproduces byte-for-byte.

These obligations and final-gate order are binding operational work (`docs/plan/controlled-documentation-rewrite.md:133-169`). The migration manifest is superseded but records intended destinations and the required roadmap regeneration (`docs/plan/documentation-migration.md:7-15`, `docs/plan/documentation-migration.md:41-52`). Operational directories `.claude/**`, `.codex/**`, and `.beads/**` remain in place (`docs/plan/controlled-documentation-rewrite.md:146-152`).

## Evidence boundary

No legacy requirement, extract row, coverage classification, RESUME completion statement,
scenario ID, roadmap status, or `SUBSTRATE` label is promotable evidence. Current code remains the
only substrate authority; a ratified specification plus executable or player-visible evidence is
required before a target claim can become active. This fact-sheet is read-only Phase 0 mapping:
no simulation suite was run for it.
