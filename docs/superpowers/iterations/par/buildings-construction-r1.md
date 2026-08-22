# Omission Review R1 — Buildings & Construction

## F1 — Road/infrastructure construction phase pipeline is entirely uncovered
- classification: omission
- source: spec/construction.md:53
- verbatim: "**Road/infrastructure phases (OURS — W&R computes road cost natively with no phases):** earthworks → sub-base (gravel) → paving (concrete) → surfacing (asphalt) → markings → open. Worked by `GROUNDWORKS`→`ASPHALT_LAYING`→`ROLLING` vehicles, consuming gravel then asphalt."
- what should have covered it: no story / no scenario
- why it matters: every extracted story/scenario about phased construction is framed around buildings only (earthworks/foundations/structure/utilities/finishing); the parallel, distinct road-phase sequence (earthworks→sub-base→paving→surfacing→markings→open) and its vehicle-skill chain is a separately named mechanic ("the mechanic the project is named for") with zero proof obligations.

## F2 — Named W&R phase list and craft-order constraint not tested
- classification: thin-AC
- source: spec/construction.md:30
- verbatim: "`SOVIET_CONSTRUCTION_GROUNDWORKS` (475, always mult 0.0 — pure earthmoving) → `SKELETON_CASTING` (concrete) / `STEEL_LAYING` / `BRICKS_LAYING` / `PANELS_LAYING` (prefab `panelák`) / `BOARDS_LAYING` → `WIRE_LAYING` (only 3 files!) → `ROOFTOP_BUILDING`, plus `TUNNELING`. A building lists only the phases its construction method uses, in craft order."
- what should have covered it: "Progress a construction project through ordered, stallable phases" AC-1 (generic "predecessor phase" check only)
- why it matters: the AC tests only that phase N+1 can't start before phase N completes in the abstract; it never asserts that a building declares only the subset of phases its construction method actually uses, nor that GROUNDWORKS always has zero work-multiplier (pure earthmoving, no material stall possible by design) — a materially different acceptance property than "gated on material delivery."

## F3 — TUNNELING as a special earthworks variant is unrepresented
- classification: omission
- source: spec/construction.md:51
- verbatim: "(`TUNNELING` is a special earthworks variant for underground work.)"
- what should have covered it: no story / no scenario
- why it matters: named alternate phase type for underground construction has no proof obligation anywhere, even as a stub/deferred note.

## F4 — Vehicle skill throughput numeric ranges never asserted
- classification: thin-AC
- source: spec/construction.md:59
- verbatim: "construction vehicles carry a `$SKILL_CONSTRUCTION_*` tag with a numeric **work throughput** (cranes 21–95, groundworks 15–37, rolling 18–27)."
- what should have covered it: "Derive construction phase duration from assigned machine throughput, never a fixed timer" AC-1 / scenario "Second crane assigned shortens phase completion time"
- why it matters: the AC and scenario prove the *proportionality law* (phase_work / Σskill) but never anchor it to any concrete magnitude or range — a duration formula that silently used, say, throughput=1 for every vehicle would still pass every extracted AC.

## F5 — Construction-office fleet sizing by tier (8–24 vehicles) not covered
- classification: omission
- source: spec/construction.md:70
- verbatim: "**owns a vehicle fleet** (`$WORKING_VEHICLES_NEEDED`, 8–24 by office tier)."
- what should have covered it: "Stand up a construction office that stocks materials and dispatches its own vehicle fleet" AC-2 (only asserts "a fleet sized by a declared vehicle count", no tiering or range)
- why it matters: the notion that office *tier* determines fleet size (8–24) is a distinct declared-data requirement from "a fleet exists with some count" — no AC or scenario would fail if all offices had an identical, untiered fleet size.

## F6 — Full `$RESOURCE_SOURCE_*` material-class list not exercised beyond generic mention
- classification: thin-AC
- source: spec/construction.md:70
- verbatim: "via `$RESOURCE_SOURCE_*` flags (WORKERS, GRAVEL, ASPHALT, CONCRETE, OPEN[steel/boards], OPEN_BRICKS, OPEN_PANELS, OPEN_BOARDS, COVERED, COVERED_ELECTRO)"
- what should have covered it: "Stand up a construction office..." AC-1 (paraphrases as "workers, gravel, asphalt, concrete, steel/brick/panel/board classes" — drops COVERED/COVERED_ELECTRO distinction entirely)
- why it matters: the spec distinguishes open-air vs. covered storage classes (COVERED, COVERED_ELECTRO) as separate flags; no AC or scenario tests that covered/weather-sensitive materials (e.g. electrical components) require covered storage as opposed to open stockpiling.

## F7 — Simulation-clock frequency requirement for construction (medium accounting / high-frequency vehicle movement) uncovered
- classification: omission
- source: spec/construction.md:119
- verbatim: "Construction progress runs at **medium** frequency (material accounting) with **high**-frequency vehicle movement to/from site (`architecture/simulation-clock.md`)."
- what should have covered it: no story / no scenario
- why it matters: this is a concrete, falsifiable tick-cadence requirement (two different update frequencies for two different subsystems of the same feature) with no AC anywhere checking it — a construction system that recomputes material accounting every single high-frequency tick, or vice-versa, would violate the spec undetected.

## F8 — Visual 0→255 render-ramp requirement (driven by real state, never a timer) uncovered
- classification: omission
- source: spec/construction.md:119
- verbatim: "Visual progress can reuse a CS1-style 0→255 render ramp, but it is *driven by* real material/machine state, never a timer."
- what should have covered it: no story / no scenario
- why it matters: this is an explicit anti-regression requirement (the visual layer must not silently become the CS1 timer anti-pattern it borrows the ramp shape from) — no AC asserts that the render value tracks actual phase progress rather than elapsed time.

## F9 — Incremental "N of M tonnes delivered" partial-delivery accounting not tested
- classification: thin-AC
- source: spec/construction.md:95
- verbatim: "**Material delivery model.** Incremental (build proceeds as materials arrive) or full-bill-first per phase? Lean incremental — matches the transcript's \"21 of 28 tonnes arrived, waits for 7 more.\""
- what should have covered it: scenario "Construction phase stalls without required material, resumes on delivery" (only tests the two boundary states: zero delivered vs. full bill delivered)
- why it matters: the spec's own worked example is a *partial* delivery (21 of 28 tonnes) implying work can proceed proportionally to what has arrived so far, not just gated as a binary present/absent — no scenario proves partial-delivery behavior (does 21/28 tonnes allow 21/28 of the phase's work-consuming input, or does the phase still wait for the last 7?). This is explicitly flagged as still-open in the source, so at minimum it's a gap worth flagging even if not yet a hard AC.

## F10 — Degrading-quality precedent grounding ($QUALITY_OF_LIVING) not represented
- classification: thin-AC
- source: spec/buildings.md:39
- verbatim: "loses quality (W&R `$QUALITY_OF_LIVING` as the residential precedent) and can become uninhabitable"
- what should have covered it: "Degrade an operating building's condition when maintenance inputs are starved" AC-1/AC-2 (uses a generic "condition value" with no tie to the residential quality-of-living precedent or non-residential analog)
- why it matters: minor but the extracted story's ACs are residential-only ("A residential building..."); the spec's degrading lifecycle stage applies to buildings generally ("a building starved of maintenance inputs (heat, repairs, materials)") — no AC or scenario covers degradation of a non-residential (e.g. industrial) building.

## F11 — "Frees land" on demolition completion not asserted
- classification: omission
- source: spec/buildings.md:41
- verbatim: "**Demolition** — a planned physical act requiring crews and hauling (W&R `$TYPE_DEMOLITION_OFFICE`); reclaims some materials, frees land."
- what should have covered it: "Demolish and repair buildings as separate physical office-dispatched processes" AC-1/AC-2, scenario "Demolition emits sorted rubble, never a money refund"
- why it matters: the extracted ACs/scenario prove rubble emission and no-money-refund, but never assert that the land becomes available for new siting/construction after demolition completes — a demolition that removed the building entity but left the lot permanently blocked would pass every extracted check.

## F12 — Explosives-consumption quantity/mechanism not represented beyond generic mention
- classification: thin-AC
- source: spec/construction.md:74
- verbatim: "Demolition consumes **explosives** + machine-work and **emits sorted rubble** (`waste_gravel`/`waste_steel`/`waste_toxic`) back into logistics — no money refund, no instant deletion."
- what should have covered it: "Demolish and repair buildings as separate physical office-dispatched processes" AC-1 (says "consumes explosives plus machine-work" but no scenario checks that missing explosives stalls demolition, analogous to material stalls elsewhere)
- why it matters: every other physical-consumption mechanic in this domain (construction phases) has an explicit stall/failure-mode scenario when its input is missing; demolition's explosives-as-input has no equivalent "demolition stalls with no explosives on site" scenario, despite the spec describing explosives as a required input, not a formality.

## F13 — Zone-mismatch enforcement policy left fully open, no AC even for the "advisory" default
- classification: thin-AC
- source: spec/zoning.md:38
- verbatim: (open question, not a requirement) "Does zone mismatch ever force anything (e.g. rezoning an occupied district), or is it advisory only? (CS1's auto-despawn on mismatch is rejected — research §G7.)"
- what should have covered it: "Author land-use zoning as a planning overlay, never a spawn trigger" AC-3 (correctly marks this UNAUDITED/open)
- why it matters: not a full omission — the extraction correctly flags this as an open policy question — but no scenario exists even for the one thing that IS settled: that CS1's auto-despawn-on-mismatch must never happen. The rejection is stated as CONFIRMED-settled ("CS1's auto-despawn on mismatch is rejected") while the AC treats the entire mismatch question as open; the narrower, already-decided negative claim (mismatch never causes auto-demolition) deserves its own falsifiable check independent of the broader open policy question.

## F14 — Multi-building enterprise composition question is an open design question, correctly unaddressed
- classification: intentional-exclusion
- source: spec/buildings.md:49
- verbatim: "Multi-building enterprises (factory + attached hostel + siding) — one asset or a composition? "
- what should have covered it: n/a
- why it matters: explicitly listed under "Open questions" as an unresolved design fork with no chosen model yet; correctly excluded from ACs since there is nothing normative to test.

## F15 — Ruin-state / self-completing degradation is an open design question, correctly unaddressed
- classification: intentional-exclusion
- source: spec/buildings.md:48
- verbatim: "Does degradation ever complete on its own (ruin state) or always stop at uninhabitable-until-renovated?"
- what should have covered it: n/a
- why it matters: explicitly an open question in the source with no decided behavior; the extracted AC-2 ("Restoring the maintenance input halts further condition decay... never automatically demolished") already covers the settled half (never auto-demolished); the ruin-state question itself remains genuinely undecided and is rightly not turned into an AC.

TOTAL FINDINGS: 13
