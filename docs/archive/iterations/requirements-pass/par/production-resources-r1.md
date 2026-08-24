## F1 — Water-quality gate has no story or scenario at all
- classification: omission
- source: spec/production.md:45,82,119,143
- verbatim: "**Water quality** | `$CONSUMPTION_WATER_REQUIRED_QUALITY` | below threshold → recipe blocked | W&R CONFIRMED"
- what should have covered it: no story / no scenario
- why it matters: it is one of six named factor gates in the spec's own table (labour, power, inputs, water quality, machinery, output space); labour/power/inputs/machinery/output-space each got a dedicated story and at least one scenario, but water quality — a binary "below threshold → recipe blocked" gate distinct from the other continuous factors — has zero AC and zero scenario anywhere in the extraction, even though it appears in the multiplicative formula (`f_water_quality`) cited by AC-1 of "Combine all production factors multiplicatively."

## F2 — Production factor re-evaluation trigger and clock cadence unstated
- classification: omission
- source: spec/production.md:131
- verbatim: "Production runs at **medium** frequency (see `architecture/simulation-clock.md`); factor re-evaluation on input/power/staffing change."
- what should have covered it: no story / no scenario
- why it matters: this is an explicit, non-open-question behavioral claim (not listed under "Open questions") about when the rate recomputes — no AC or scenario asserts that a change in input/power/staffing triggers re-evaluation rather than only at fixed tick boundaries.

## F3 — containerClass field on Resource metadata is untested
- classification: thin-AC
- source: spec/resources.md:67,86
- verbatim: "`containerClass` ← typed container models: `container_big_{aluminium,bio,construction,plastic,steel,toxic}` + small variants — a container's material encodes what it may legally carry."
- what should have covered it: "Give every resource item physical handling metadata" (AC-3 only covers storageClass and transportClass, not containerClass)
- why it matters: containerClass is named as one of the ontology's handling-classification fields ("determine compatible storage & transport") with a legality claim ("what it may legally carry") but no AC requires the field to exist or enforces the legality check, unlike storageClass/transportClass which got an explicit AC and a scenario.

## F4 — category field on Resource metadata is untested
- classification: thin-AC
- source: spec/resources.md:56,93
- verbatim: "category           // raw / processed-material / construction / consumer-good / liquid / energy / waste  (see below)"
- what should have covered it: "Give every resource item physical handling metadata" / "Give every resource item an economic tier classification" (neither AC set mentions `category`, only `tier`)
- why it matters: category is a distinct, explicitly enumerated field in the Resource{} schema (7 named values) separate from tier; the extraction covers tier exhaustively but drops category with no AC requiring it to exist on every item prototype.

## F5 — Storage-class incompatibility rejection is asserted but not scenario-tested (only transport rejection is)
- classification: missing-scenario
- source: spec/resources.md:120
- verbatim: "> A resource can only sit in a storage of a compatible `storageClass`, and only move on a `transportClass` it belongs to."
- what should have covered it: "Give every resource item physical handling metadata" / scenario "Steel and fuel resources declare incompatible transport classes" (that scenario tests only the transport half — loading steel onto a tanker — never storage-bucket rejection)
- why it matters: the spec states two independent enforceable rules (storage compatibility AND transport compatibility); AC-3 of the owning story explicitly names "storage buildings can reject incompatible goods" in its own text, but no scenario exercises putting an incompatible resource into a storage bucket of the wrong storageClass — only the transport-loading half is falsified.

## F6 — Electricity/heat exclusion from the vehicle transport scheduler is unfalsified
- classification: omission
- source: spec/resources.md:125
- verbatim: "Energy (electricity, heat) never rides a vehicle — it flows on its own network, so it's modelled but excluded from the logistics vehicle scheduler."
- what should have covered it: no story / no scenario
- why it matters: this is a concrete, checkable exclusion rule (network-borne resources must never be assignable to vehicle transport) that follows directly from the transportClass mechanism the extraction otherwise tests (F5's sibling scenario), but no AC or scenario asserts electricity/heat are rejected by or absent from the vehicle scheduler.

## F7 — Sewage opt-out token has no coverage
- classification: intentional-exclusion
- source: spec/production.md:92
- verbatim: "`$WATER_NOT_PRODUCE_SEWAGE_FROM_PRODUCTION` opts out."
- what should have covered it: "Let recipes emit byproducts alongside their primary outputs"
- why it matters: this is a narrow W&R data-token detail (an opt-out flag on one byproduct channel) noted only in passing in the byproducts paragraph; the extraction reasonably curated it out as non-essential to the byproducts-are-first-class-outputs behavior the story and scenario do cover — flagged here for completeness, not as a required gap.

TOTAL FINDINGS: 6
