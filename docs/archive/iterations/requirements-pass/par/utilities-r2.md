## F1 — Border electricity trade via import/export transformers has no story
- classification: omission
- source: spec/electricity.md:38-40
- verbatim: "### Border trade\n\nElectricity crosses the border as a wired utility link via dedicated import/export transformer buildings (§C4) — pricing/currency owned by spec/trade.md."
- what should have covered it: no story
- why it matters: this is a CONFIRMED cross-domain coupling (utility ↔ trade) naming a concrete building type (import/export transformer); none of the 5 electricity stories or their ACs mention cross-border power flow at all.

## F2 — Solver amortization budget (256-frame-equivalent per-tick cycle) is unrepresented
- classification: omission
- source: spec/electricity.md:36
- verbatim: "CS1 amortises its grid solve over a 256-frame cycle (§A3). Whatever our graph solver is, it must be budgeted per tick the same way (§H). See `architecture/simulation-clock.md`."
- what should have covered it: no story / no scenario
- why it matters: this is a stated performance requirement/constraint on the electricity (and by extension water/sewage/heat) solver — a proof obligation about tick-budget behavior under load that no AC falsifies.

## F3 — Idle/lighting fallback draw for consumers not covered by any AC
- classification: thin-AC
- source: spec/electricity.md:32
- verbatim: "Consumers draw `$CONSUMPTION_PER_SECOND eletric` continuously, with idle/lighting fallback draws (research/production.md §A4 — not re-derived)."
- what should have covered it: "Power plants are ordinary recipe buildings" story covers generation-side draw, but no story addresses consumer-side idle/lighting fallback draw as a distinct observable (a building with zero active recipe still drawing a baseline "idleDraw", per the Data draft's `idleDraw` field)
- why it matters: idleDraw is a named field in the spec's own data draft but never appears in an AC — a consumer that goes fully idle vs. one still drawing baseline power is an unverified distinction.

## F4 — Reservoirs/switches as in-line storage and junction hardware are absent from any AC
- classification: omission
- source: spec/water.md:18
- verbatim: "reservoirs/switches as in-line storage and junctions, substations (`$TYPE_WATER_ENDSTATION`) as leaf buffers. Adopted: a real hydraulic topology with explicit storage/junction/leaf hardware."
- what should have covered it: "Water sourced, treated, and piped with a quality grade" story covers source/treatment/pipe/consumer-gating but no AC mentions reservoirs as storage or switches as junction points
- why it matters: the spec explicitly adopts "explicit storage/junction/leaf hardware" as part of the hydraulic topology; storage buffering behavior (e.g. a reservoir smoothing supply/demand mismatch) has no proof obligation anywhere.

## F5 — Per-substation consumer-class routing policy (reserve residential from industrial draw) has no story
- classification: omission
- source: spec/water.md:32-34
- verbatim: "### Routing policy\n\nW&R exposes routing-policy flags (`$WATER_NOT_USE_FOR_INDUSTRY_SUBSTATIONS` — INFERRED: reserve residential substations from industrial draw, §D4). Adopted as planner policy: per-substation consumer-class reservation."
- what should have covered it: no story / no scenario
- why it matters: this is explicitly "Adopted as planner policy" (not an open question) — a distinct behavioral rule (a substation can reserve capacity for one consumer class over another) with zero coverage, unlike electricity's parallel priority-class brownout mechanic which IS covered (AC-1/AC-2 of "Priority-class brownout before blackout").

## F6 — Heat must be actively pumped (ENGINE_SPEED-driven distance loss) is not distinguished from static per-km loss in any AC
- classification: thin-AC
- source: spec/heating.md:22
- verbatim: "Pumping stations carry `$ENGINE_SPEED` — heat must be actively pumped, hinting at distance loss (INFERRED; solver native)."
- what should have covered it: "Third pipe network: trunk-and-branch district heating" AC-2 covers proportional-to-length loss, but no AC covers the active-pumping requirement itself (i.e., a pumping station being offline/unpowered/understaffed causing heat delivery to fail even with capacity available downstream)
- why it matters: "must be actively pumped" implies pump stations are themselves a failure point (analogous to transformers in the electricity model, which DO get an AC for capacity-limited conversion) — heating's pumping stations get no equivalent failure-mode AC.

## F7 — Sorting-bin vs. mixed-collection downstream consequence ("mixed needs a separation plant first") is unfalsified
- classification: thin-AC
- source: spec/waste.md:22
- verbatim: "Sorting at source (separate bins) vs mixed collection is a planner choice with downstream consequences (mixed needs a separation plant first)."
- what should have covered it: "Buildings deposit typed waste into containers" AC-1 covers that unsorted deposit goes to waste_mixed, and "Waste processes to recycled material, energy, or landfill" AC-1 covers separation-plant extraction yields — but no AC or scenario ties the two together to prove the actual claimed consequence: that already-sorted waste can skip the separation plant and route directly to recycling, while mixed waste cannot
- why it matters: the spec names a specific downstream branching behavior (sorted bypasses separation, mixed requires it) that is never stated as an observable outcome anywhere in the extracted ACs.

## F8 — Consumer required-quality numeric thresholds (0.93 animal farm, 0.97 food factory, 0.60 nuclear cooling) absent from every AC
- classification: thin-AC
- source: spec/water.md:20
- verbatim: "Adopted verbatim: quality degrades through use, is recovered at a cost, and gates sensitive consumers (food, hospitals). CS1's single pollution byte (§B2) only gestures at this." (thresholds themselves named in the Evidence log row, sourced from research/utilities.md §D2: animal farm 0.93, food factory 0.97, nuclear cooling 0.60)
- what should have covered it: "Water sourced, treated, and piped with a quality grade" AC-3 gates a "declared required quality" generically but names no concrete threshold; the paired scenario "Food factory rejects sub-threshold quality water" uses an invented 0.97/0.80 pair rather than citing the spec's own numbers
- why it matters: the spec's per-consumer-class thresholds (0.93/0.97/0.60) are load-bearing constants for gating behavior differentiation across consumer types, and no AC distinguishes multiple consumer classes with different thresholds — only a single generic "sensitive consumer" is proven.

## F9 — Heat plant output numeric constant (350 heat from 0.28 coal, 30 workers) absent from any AC
- classification: thin-AC
- source: spec/heating.md (Evidence log row): "W&R: heat is a produced resource from coal + workers, base-game | CONFIRMED | `heating_plant_big.ini:3-25`" — body text: "Heat is a produced resource: `heating_plant_big.ini` — 30 workers, `$CONSUMPTION coal 0.28` → `$PRODUCTION heat 350`, medium pollution (research/utilities.md §E2)."
- what should have covered it: "Heat plants burn fuel into district heat" AC-1 only proves the zero/nonzero boundary (no fuel or no workers ⇒ zero heat); no AC proves a proportional or rated-output relationship
- why it matters: same thin-AC pattern as F8 — the spec states a concrete recipe ratio but the extraction only ever tests the on/off boundary case, never the magnitude.

## F10 — Incinerator per-type burn ratios and dual-mode output magnitudes (waste 3.0→electricity 33; waste 2.5→heat 450) absent from any AC
- classification: thin-AC
- source: spec/waste.md: "**Incinerate** — for electricity (`incinerator_powerplant`: waste 3.0 → eletric 33, per-type burn ratios, ash + high pollution) or district heat (`incinerator_heat`: waste 2.5 → heat 450) — couplings to spec/electricity.md / spec/heating.md."
- what should have covered it: "Waste processes to recycled material, energy, or landfill" AC-2 covers that incineration produces electricity-or-heat plus ash and pollution, generically; the "Waste incinerator feeds either the electricity or heat network" scenario likewise stays generic ("sufficient waste input" / "outputs electricity")
- why it matters: per-waste-type burn ratios are named as a distinct mechanic ("per-type burn ratios") in the spec but no AC or scenario tests that different waste types yield different output amounts — only that incineration produces *some* output.

## F11 — Elevation/pressure model open question wrongly treatable as coverage gap
- classification: intentional-exclusion
- source: spec/water.md:44
- verbatim: "Elevation/pressure: model real head (pumps required uphill) or abstract pump-hop capacity? Lean abstract in v1."
- what should have covered it: n/a — explicitly an open design question with a stated lean toward the simpler v1 model, not a settled requirement
- why it matters: correctly excluded; flagged only to confirm it was checked and is not a missed AC.

## F12 — Storm/rain sewage load and CHP (combined heat-and-power) open questions correctly unmodeled
- classification: intentional-exclusion
- source: spec/sewage.md:53 ("Storm/rain load in scope? (Neither game models it; probably out for v1.)"); spec/heating.md:49 ("Combined heat-and-power (CHP): neither game has it ... Very period-authentic — add as OURS building type?")
- verbatim: "Storm/rain load in scope? (Neither game models it; probably out for v1.)" / "Combined heat-and-power (CHP): neither game has it (CS1's incinerator makes power *or* the heat variant makes heat; W&R splits plants too). Very period-authentic — add as OURS building type?"
- what should have covered it: n/a — both are open questions in "Open questions" sections, not adopted requirements
- why it matters: correctly excluded; noted to show these were checked, not silently skipped.

TOTAL FINDINGS: 10
