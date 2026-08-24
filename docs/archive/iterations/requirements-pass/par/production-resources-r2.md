## F1 — Water-quality gate has no dedicated AC or scenario
- classification: omission
- source: spec/production.md:82
- verbatim: "| **Water quality** | `$CONSUMPTION_WATER_REQUIRED_QUALITY` | below threshold → recipe blocked | W&R CONFIRMED |"
- what should have covered it: no story / no scenario — `f_water_quality` only appears folded into the generic multiplicative-composition AC ("Combine all production factors multiplicatively so the scarcest factor wins", AC-1) which treats it as one anonymous factor among six, never independently tested
- why it matters: water quality is a distinct binary-threshold gate (below required purity ⇒ blocked), unlike the other five continuous factors; nothing falsifies that a sub-threshold water quality actually blocks the recipe.

## F2 — Recipe.waterQualityMin data field never exercised
- classification: omission
- source: spec/production.md:122-127
- verbatim: "waterQualityMin?                               // from $CONSUMPTION_WATER_REQUIRED_QUALITY"
- what should have covered it: no story / no scenario
- why it matters: the draft data schema names a field with no AC establishing it must exist or be enforced, unlike storageClass/transportClass/tier which each got a dedicated story in the resources epic.

## F3 — Sewage byproduct routing via network connection, distinct from bucket storage
- classification: omission
- source: spec/production.md:92
- verbatim: "**Sewage** — `$PRODUCTION_SEWAGE_POLLUTION <rate>` (numeric); routed via `$CONNECTION_SEWAGE_OUTPUT`. `$WATER_NOT_PRODUCE_SEWAGE_FROM_PRODUCTION` opts out."
- what should have covered it: "Let recipes emit byproducts alongside their primary outputs" / "Require waste to be physically stored and hauled, not vanish" — but both stories and their scenario ("A combustion recipe emits ash that fills its own output bucket…") only model byproducts as an output-storage bucket (ash-shaped). Sewage's stated mechanism is a network connection (`$CONNECTION_SEWAGE_OUTPUT`), not a storable/haulable bucket, and the opt-out flag is unaddressed by any AC.
- why it matters: the spec explicitly distinguishes two byproduct disposal mechanisms (bucket vs. network route) but the extraction collapses both into one bucket model, silently dropping the sewage-specific routing and its opt-out.

## F4 — Renewable output modulation ($PRODUCTION_CONNECT_TO_WIND/_TO_SUN) unaddressed
- classification: omission
- source: spec/production.md:37,59,107
- verbatim: "$PRODUCTION_CONNECT_TO_WIND / _TO_SUN    // renewable output modulated by weather/day-cycle"
- what should have covered it: no story / no scenario
- why it matters: named as a confirmed W&R token and listed under "Open questions" as something the spec leans toward adopting ("Both fit the physical-causality theme"), yet no AC captures it as even a deferred/flagged requirement.

## F5 — Depletion/ageing drift ($PRODUCTION_DECREASE_ACCORDING_YEAR) unaddressed
- classification: omission
- source: spec/production.md:60,107
- verbatim: "$PRODUCTION_DECREASE_ACCORDING_YEAR      // output drifts over the calendar (depletion/ageing)"
- what should have covered it: no story / no scenario
- why it matters: a distinct time-based production factor (output declining over calendar time, e.g. mine depletion) is named as a confirmed token the spec leans toward adopting; entirely absent from the extraction even as a stub.

## F6 — Idle-vs-lighting power fallback ($ELETRIC_WITHOUT_LIGHTING_FACTOR) not covered
- classification: omission
- source: spec/production.md:58
- verbatim: "$ELETRIC_WITHOUT_LIGHTING_FACTOR <0..1>  // power fraction with lighting off"
- what should have covered it: "Throttle output continuously with available power instead of a binary blackout gate" AC-3 covers only `$ELETRIC_WITHOUT_WORKING_FACTOR` (idle-draw fallback); the sibling lighting-off fallback token is a separate named draw-reduction mode never mentioned.
- why it matters: it's a second, distinct confirmed power-draw-reduction mechanism alongside the one AC-3 already covers; dropping it silently narrows the power-factor requirement.

## F7 — Waste recycling recovery yield ($WASTE_EXTRACTION) not covered
- classification: omission
- source: spec/production.md:63,94
- verbatim: "$WASTE_EXTRACTION <wasteClass> <yield>   // recycling recovers a fraction, e.g. waste_steel 0.98"
- what should have covered it: no story / no scenario — "Let recipes emit byproducts alongside their primary outputs" only covers waste as a byproduct output, not the reverse recycling-recovery-yield mechanic (waste as an input recovered at a fractional yield)
- why it matters: this is the other half of the confirmed waste chain (production.md explicitly calls it "the reverse") — recycling plants consuming waste and recovering only a stated fraction (e.g. 0.98, 0.9) is a distinct, numerically-specific behavior with zero AC coverage.

## F8 — Water-well "whole not per-worker" exception to the labour-scaling rule
- classification: omission
- source: spec/production.md:32
- verbatim: "`water_well_*.ini`: `//production for water well is not per worker, but whole` marks the *exception*"
- what should have covered it: "Verify declared workforce is sourced live from present population, not stockpiled" / "Scale output continuously with staffing fraction, not linearly" — both assume the per-worker rate model universally; the spec explicitly flags a named exception class (water wells) where production is NOT per-worker, and no AC or scenario tests that this exception is representable/respected.
- why it matters: an extraction that only encodes the general per-worker rule silently loses the one documented counter-example, which is exactly the kind of edge case a recipe schema needs to support.

## F9 — containerClass field (typed containers) has no AC
- classification: omission
- source: spec/resources.md:67,86
- verbatim: "containerClass?    // typed container where applicable (steel/plastic/bio/toxic/aluminium/open)" ... "**`containerClass`** ← typed container models: `container_big_{aluminium,bio,construction,plastic,steel,toxic}` + small variants — a container's material encodes what it may legally carry."
- what should have covered it: "Give every resource item physical handling metadata" covers mass/volume/storageClass/transportClass (AC-1–3) but never mentions containerClass, despite it being named in the same Resource{} schema block and given its own confirmed-token bullet.
- why it matters: containerClass is presented as a third handling-compatibility axis (alongside storage/transport) with its own confirmed W&R evidence; it is dropped without even being marked as an open question in the extraction.

## F10 — $STORAGE_IMPORT_SPECIAL and $STORAGE_DEMAND_* bucket types not covered
- classification: omission
- source: spec/resources.md:84
- verbatim: "`$STORAGE_IMPORT_SPECIAL <class> <cap> <resource>` (a bucket pinned to one named resource), `$STORAGE_DEMAND_BASIC/_ADVANCED/_HOTEL/_PRISON` (consumer-demand buffers in shops)."
- what should have covered it: no story / no scenario — "Give every resource item physical handling metadata" only requires a general storageClass enum, not these specific confirmed bucket-shape variants (a bucket pinned to a single named resource, or consumer-demand buffers)
- why it matters: these are two structurally distinct storage-bucket behaviors (resource-pinned import bucket; demand buffer tiers) called out as confirmed W&R tokens, with no requirement that the storage model support either shape.

## F11 — Energy/heat explicitly excluded from the vehicle/logistics scheduler
- classification: omission
- source: spec/resources.md:125
- verbatim: "Energy (electricity, heat) never rides a vehicle — it flows on its own network, so it's modelled but excluded from the logistics vehicle scheduler."
- what should have covered it: no story / no scenario — the transport-compatibility contract story/scenario ("Steel and fuel resources declare incompatible transport classes") only tests that two physical goods have disjoint transport classes; it never tests that network-only resources (electricity/heat) are excluded from vehicle assignment altogether, a different and stronger constraint than "incompatible class."
- why it matters: this is an explicit design decision with a concrete consequence (electricity/heat can never appear in a vehicle-transport query), distinct from ordinary storageClass/transportClass mismatch, that no AC falsifies.

## F12 — Production re-evaluation cadence and tick frequency unaddressed
- classification: omission
- source: spec/production.md:131
- verbatim: "Production runs at **medium** frequency (see `architecture/simulation-clock.md`); factor re-evaluation on input/power/staffing change."
- what should have covered it: no story / no scenario
- why it matters: this states an observable timing/update-cadence behavior (factors re-evaluate on change, not just every tick) that governs when a bottleneck reason updates; none of the bottleneck-surfacing ACs test *when* the recomputation happens, only that a value exists.

## F13 — Under-qualification throttle-vs-warn decision for professorsNeeded
- classification: thin-AC
- source: spec/production.md:99
- verbatim: "Should under-qualification **throttle** output (OURS) or merely warn (CS1 base)? Lean throttle."
- what should have covered it: "Gate advanced recipes on skilled/educated labour separately from general labour" AC-2 says a shortfall "throttles output independently... and multiplicatively" — this does state the throttle behavior, but no scenario exercises it (only the two Liebig-factor scenarios cover labour/power/inputs numerically; professorsNeeded's multiplicative interaction is asserted in prose only, never in a scenario with concrete numbers as the other factors get)
- why it matters: every other multiplicative factor (labour curve, power, inputs) got a dedicated numeric contract scenario; professorsNeeded is the one factor left as an AC assertion with no falsifying scenario.

## F14 — TOC / status headers / evidence-log tables
- classification: intentional-exclusion
- source: spec/production.md:1-9, spec/resources.md:1-13
- verbatim: "**Status:** draft model (grounded in research)" / "## Evidence log"
- what should have covered it: n/a — non-normative metadata and citation tables, not behavior
- why it matters: correctly excluded; these carry no testable requirement.

TOTAL FINDINGS: 13
