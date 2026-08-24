## F1 — Road/infrastructure construction phase pipeline entirely uncovered
- classification: omission
- source: spec/construction.md:44-46
- verbatim: "**Road/infrastructure phases (OURS — W&R computes road cost natively with no phases):** earthworks → sub-base (gravel) → paving (concrete) → surfacing (asphalt) → markings → open. Worked by `GROUNDWORKS`→`ASPHALT_LAYING`→`ROLLING` vehicles, consuming gravel then asphalt. This is the mechanic the project is named for — a highway visibly progressing grading → laying → surfacing."
- what should have covered it: no story / no scenario
- why it matters: this is called out as "the mechanic the project is named for" yet no story or AC anywhere addresses roads going through phased physical construction (as opposed to instant Road::make); every extracted story/scenario for phased construction is building-only.

## F2 — TUNNELING earthworks variant not covered
- classification: omission
- source: spec/construction.md:47
- verbatim: "(`TUNNELING` is a special earthworks variant for underground work.)"
- what should have covered it: no story / no scenario
- why it matters: names a distinct construction sub-mode with no representation, even as a minor/deferred item.

## F3 — "no-worker" bottleneck value never exercised by any scenario
- classification: missing-scenario
- source: spec/construction.md:114
- verbatim: "bottleneck                             // no-material | no-machine | no-worker  → player UI (shared w/ production.md)"
- what should have covered it: story "Progress a construction project through ordered, stallable phases" AC-4 mentions the bottleneck field generically ("no-material / no-machine / no-worker") but no scenario ever drives a phase into the no-worker state; both stall scenarios ("stalls without required material", "stalls without matching vehicle skill") only exercise no-material and no-machine.
- why it matters: worker sourcing is a named, distinct failure mode (separate from vehicle/machine assignment) and is left with zero proof obligation.

## F4 — Workers pool sourcing and fixed lump-sum worker billing not covered
- classification: omission
- source: spec/construction.md:71
- verbatim: "**Workers** — from the office's `$RESOURCE_SOURCE_WORKERS` pool (`spec/citizens.md`); a few very large jobs also bill a fixed `$COST_RESOURCE workers <n>` lump."
- what should have covered it: no story / no scenario (the construction-office story only covers material resource-source flags: "workers, gravel, asphalt, concrete, steel/brick/panel/board classes" is *listed* in AC-1's text but no AC or scenario actually tests that a phase consumes workers from this pool or that a lump-sum worker bill gates a phase)
- why it matters: workers are named as a first-class construction input alongside materials and machines, but the extraction has no proof obligation for worker consumption/gating at all — ties directly to the untested no-worker bottleneck (F3).

## F5 — Building's own declared workforce fields ($WORKERS_NEEDED, $CITIZEN_ABLE_SERVE) not covered
- classification: omission
- source: spec/buildings.md:22
- verbatim: "**Workforce** — `$WORKERS_NEEDED n`, `$CITIZEN_ABLE_SERVE n`. Staffing is declared; workers are *sourced* at runtime by labour allocation ([spec/citizens.md](citizens.md))."
- what should have covered it: story "Declare a building as flat typed data: function type, capacities, connections" — its 3 ACs cover function type (AC-1), connections (AC-2), and material metadata (AC-3), but none address the building declaring workforce/staffing requirements.
- why it matters: workforce declaration is one of the four listed declaration axes ("Function type ... Capacities ... Workforce ... Connections") in the same paragraph the story is sourced from, yet it is the one axis with no AC.

## F6 — Building's own declared storage/dwelling capacities not covered
- classification: omission
- source: spec/buildings.md:21
- verbatim: "**Capacities** — storages per transport class (`$STORAGE <class> <n>`), consumption/supply as `$STORAGE_DEMAND_*`; dwelling capacity is a people-bucket ([research/households.md](../research/households.md) §D1)."
- what should have covered it: story "Declare a building as flat typed data" — AC-3 tests material metadata for construction bill-of-quantities items, not the building's own storage/dwelling capacity declaration.
- why it matters: capacity declaration (how much a building can store/house once operating) is a distinct axis from the construction-material metadata AC-3 actually tests; conflating them leaves the operating-capacity declaration itself unproven.

## F7 — Utility reach in the siting checklist not exercised by any scenario
- classification: thin-AC
- source: spec/zoning.md:20
- verbatim: "**Placement validity is a physical checklist, not a grid.** Adopt CS1's validator — flat, network-adjacent, in-bounds, unoccupied, within utility reach — evaluated at the planner's cursor at siting time."
- what should have covered it: scenario "Placement rejected on land failing the physical siting checklist" only exercises "steeply sloped or already-occupied tile" — flatness and occupancy, not network-adjacency or utility (electricity) reach specifically, though the owning story's AC-1 text does list all five checklist items.
- why it matters: "within utility reach" and "network-adjacent" are two of the five named checklist criteria with no scenario step ever failing/passing on them specifically.

## F8 — Construction office fleet size range (8–24 by tier) not represented
- classification: omission
- source: spec/construction.md:69
- verbatim: "and **owns a vehicle fleet** (`$WORKING_VEHICLES_NEEDED`, 8–24 by office tier)."
- what should have covered it: story "Stand up a construction office that stocks materials and dispatches its own vehicle fleet" AC-2 says "The office owns a fleet sized by a declared vehicle count" (generic), no AC/scenario tests tiering or the numeric range.
- why it matters: minor numeric/tiering detail; likely acceptable to abstract away, but flagged since the concrete number is asserted by the spec.

## F9 — Visual construction progress ramp (render state) not covered
- classification: omission
- source: spec/construction.md:117
- verbatim: "Visual progress can reuse a CS1-style 0→255 render ramp, but it is *driven by* real material/machine state, never a timer."
- what should have covered it: no story / no scenario
- why it matters: an explicit observable behavior (a rendered progress indicator whose value must be derived from real state rather than time) with no proof obligation anywhere; low priority but genuinely observable and testable ("render state never advances purely from elapsed ticks").

## F10 — Simulation-clock frequency requirement for construction not covered
- classification: intentional-exclusion
- source: spec/construction.md:118-119
- verbatim: "Construction progress runs at **medium** frequency (material accounting) with **high**-frequency vehicle movement to/from site (`architecture/simulation-clock.md`)."
- what should have covered it: n/a
- why it matters: this is an implementation/performance-tier assignment referencing a separate architecture doc, not itself a player-observable behavioral requirement — reasonable to exclude from behavior scenarios.

## F11 — Zoning land-use category set ("agricultural", "mixed") not distinctly tested
- classification: thin-AC
- source: spec/zoning.md:29
- verbatim: "the general plan (генплан) marks districts residential / industrial / agricultural / mixed."
- what should have covered it: story "Author land-use zoning as a planning overlay, never a spawn trigger" scenario only exercises "a residential land-use polygon"; industrial/agricultural/mixed categories are never instantiated in any scenario.
- why it matters: minor — one representative category is exercised generically, but the spec names four distinct categories and none besides residential appears anywhere in a scenario.

## F12 — Demolition office AND repair office named as two distinct offices; repair-office identity not covered
- classification: thin-AC
- source: spec/construction.md:73
- verbatim: "**Demolition** (`$TYPE_DEMOLITION_OFFICE`) and **repair** (`$TYPE_REPAIR_OFFICE`/`$REPAIR_AREA`) are **separate physical offices** — CONFIRMED."
- what should have covered it: story "Demolish and repair buildings as separate physical office-dispatched processes" AC-3 covers a repair *process* consuming a materials/work bill, but never asserts repair is dispatched from a distinct `$TYPE_REPAIR_OFFICE`-equivalent building entity (as opposed to just "a separate flow" in code).
- why it matters: the spec's claim is specifically about repair being sited as its own physical building type, which is a stronger/more concrete requirement than "a distinct code path" that the extracted AC settles for.

TOTAL FINDINGS: 11
