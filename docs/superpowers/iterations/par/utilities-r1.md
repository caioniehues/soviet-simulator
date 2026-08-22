## F1 — Border trade for electricity is entirely unrepresented
- classification: missing-scenario
- source: spec/electricity.md:38-40
- verbatim: "### Border trade\n\nElectricity crosses the border as a wired utility link via dedicated import/export transformer buildings (§C4) — pricing/currency owned by spec/trade.md."
- what should have covered it: no story / no scenario
- why it matters: import/export transformer buildings are a named, CONFIRMED-grammar mechanism with cross-domain coupling to trade.md; no AC or scenario tests that electricity can be imported/exported across a border link.

## F2 — Idle/lighting fallback draw is asserted in the data model but never tested
- classification: thin-AC
- source: spec/electricity.md:32
- verbatim: "Consumers draw `$CONSUMPTION_PER_SECOND eletric` continuously, with idle/lighting fallback draws (research/production.md §A4 — not re-derived)."
- what should have covered it: story "Power plants are ordinary recipe buildings" (adjacent) / no scenario
- why it matters: the draft data model explicitly adds `idleDraw` to Consumer, but no AC or scenario distinguishes idle/lighting baseline draw from full operating draw — a consumer with zero production activity should still draw the idle amount, which is unverified.

## F3 — Solver amortisation/tick-budget requirement has no proof obligation
- classification: omission
- source: spec/electricity.md:34-36
- verbatim: "### Solver performance (CS1 pattern — CONFIRMED, adopted as implementation note)\n\nCS1 amortises its grid solve over a 256-frame cycle (§A3). Whatever our graph solver is, it must be budgeted per tick the same way (§H)."
- what should have covered it: no story / no scenario
- why it matters: this is a stated performance/behavioral constraint on the solver ("must be budgeted per tick") with no AC that falsifies an unbudgeted, all-at-once solve; extraction skipped it entirely (arguably borderline non-functional, but the spec frames it as an adopted requirement, not a note).

## F4 — Per-substation consumer-class reservation policy (residential vs industrial) has no story
- classification: omission
- source: spec/water.md:32-34
- verbatim: "### Routing policy\n\nW&R exposes routing-policy flags (`$WATER_NOT_USE_FOR_INDUSTRY_SUBSTATIONS` — INFERRED: reserve residential substations from industrial draw, §D4). Adopted as planner policy: per-substation consumer-class reservation."
- what should have covered it: no story / no scenario
- why it matters: this is an explicitly "Adopted" planner-policy mechanism (a substation can be reserved for residential-only draw, excluding industrial consumers) with zero AC or scenario coverage — industrial consumers could incorrectly draw from a reserved substation with no test to catch it.

## F5 — Downstream consequence of mixed vs sorted waste collection is asserted but untested
- classification: thin-AC
- source: spec/waste.md:22
- verbatim: "Sorting at source (separate bins) vs mixed collection is a planner choice with downstream consequences (mixed needs a separation plant first)."
- what should have covered it: story "Buildings deposit typed waste into containers" (AC-1 covers deposit/sorting bins but not downstream routing) / no scenario
- why it matters: the spec states a concrete behavioral consequence — mixed-collected waste must pass through a separation plant before it can reach type-specific recycling plants, while sorted waste can presumably skip that hop — but no AC or scenario tests this routing difference.

## F6 — Landfill cannot be bulldozed while full
- classification: omission
- source: spec/waste.md:32
- verbatim: "landfills are plain `$TYPE_STORAGE` holding waste forever. CS1 agrees: a landfill never empties and can't be bulldozed while full (§F1)."
- what should have covered it: story "Waste processes to recycled material, energy, or landfill" AC-3 (covers "never empties, no output" but omits the bulldoze-while-full constraint) / no scenario
- why it matters: "can't be bulldozed while full" is a distinct, testable constraint (an attempted demolition/removal action must be rejected) that the extracted AC-3 does not mention or cover.

## F7 — Toxic waste treatment (chemical neutralisation, high pollution) is a distinct fate not separately covered
- classification: thin-AC
- source: spec/waste.md:32
- verbatim: "**Treat toxic / landfill** — toxic neutralised with chemicals (high pollution); landfills are plain `$TYPE_STORAGE` holding waste forever."
- what should have covered it: story "Waste processes to recycled material, energy, or landfill" (covers separation, incineration, landfill but not toxic-chemical-neutralisation as its own fate) / no scenario
- why it matters: toxic treatment is named as one of "three fates" alongside recycling and incineration, with its own inputs (chemicals) and output (high pollution) distinct from plain landfill storage; the extracted story's three ACs cover separation/incinerate/landfill but never toxic neutralisation, so that fate has zero proof obligation.

## F8 — Heat pumping stations' active-pump requirement (ENGINE_SPEED) is not tested as a distinct failure mode
- classification: thin-AC
- source: spec/heating.md:22
- verbatim: "Pumping stations carry `$ENGINE_SPEED` — heat must be actively pumped, hinting at distance loss (INFERRED; solver native)."
- what should have covered it: story "Third pipe network: trunk-and-branch district heating" AC-2 (covers length-proportional loss but not an inoperative/unpumped pumping station) / no scenario
- why it matters: the spec calls out that heat "must be actively pumped" (implying a pumping station is itself a distinct point of failure, not just a passive conduit like the loss model assumes) — no AC tests what happens when a pumping station is unpowered or absent from the path.

## F9 — CS1 landfill's "consumes power/material" behavior when processing (vs plain storage) is not distinguished
- classification: intentional-exclusion
- source: spec/waste.md (evidence log row: "CS1: landfill stores forever, incinerator consumes (+power/material)")
- verbatim: "CS1: landfill stores forever, incinerator consumes (+power/material) | CONFIRMED | `LandfillSiteAI.cs:26-38, 399-430` | §F1"
- what should have covered it: n/a
- why it matters: this is an evidence-log citation about CS1's source code, not a normative statement about the new spec's model — correctly excluded from stories.

TOTAL FINDINGS: 8
