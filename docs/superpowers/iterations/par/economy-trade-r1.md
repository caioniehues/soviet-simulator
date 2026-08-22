## F1 — Utility border trade (electricity/pipeline import-export) entirely absent
- classification: omission
- source: spec/trade.md:21
- verbatim: "**Utility border buildings** — electricity import/export transformers and border pipelines are separate typed endpoints (§A3). The transport medium of a good decides *which* border building it crosses at, same as domestically (INFERRED)."
- what should have covered it: no story / no scenario
- why it matters: this is a distinct trade mechanism (wire/pipe endpoints, not vehicle-hauled cargo) called out explicitly in the spec's own `BorderCrossing { mode: road|rail|air|wire|pipe; ... }` data sketch (line 55), and no AC or scenario anywhere in the extraction addresses electricity or pipeline import/export at all — every AC assumes a vehicle trip.

## F2 — Per-currency loans (interest, borrowing caps) dropped, not captured-and-deferred
- classification: omission
- source: spec/trade.md:29
- verbatim: "Separate loans per currency with own interest and borrowing caps (§C2 — CONFIRMED from UI strings)."
- what should have covered it: no story / no scenario (the "Track two hard-currency ledgers" story only covers the two balances and no-free-conversion, not loans)
- why it matters: this is marked CONFIRMED and adopted in the source (not an open question — the loans-as-mechanic open question at spec/trade.md:63 only asks whether to adopt penalty/cap details "as-is"), and it is asserted as a core reason dual-currency matters ("having money is insufficient in a second, sharper way", spec/trade.md:31); the extraction should have either produced a loan story/AC or explicitly marked it deferred, per the domain brief's rule that dropped dual-currency-adjacent requirements must be captured-and-deferred, not silently omitted.

## F3 — No scenario for "Give households a cash balance (nal)"
- classification: missing-scenario
- source: docs/superpowers/iterations/extract/economy-trade.json (story "Give households a cash balance (nal)")
- verbatim: "Every human entity has a nal (cash) balance field that persists across save/load."
- what should have covered it: story exists, no scenario owns it
- why it matters: this is the foundational fact the entire nal/beznal split depends on (save/load persistence of a new field type), yet no scenario in the file exercises it — the closest scenario ("Household cash cannot be spent using enterprise accounting roubles") only tests the *rejection* boundary, never that nal itself round-trips through save/load.

## F4 — No scenario for "Pay wages from employer to worker in nal"
- classification: missing-scenario
- source: docs/superpowers/iterations/extract/economy-trade.json (story "Pay wages from employer to worker in nal")
- verbatim: "A working human's workplace debits its own account and credits the worker's nal balance on a defined wage interval."
- what should have covered it: story exists, no scenario owns it
- why it matters: wages are the only documented legal bridge between beznal and nal (per the story "Forbid beznal from ever buying a consumer good", AC-2: "the documented wage-payment path ... is the only legal bridge between circuits") — yet the bridge itself has zero scenario proving it actually fires (debit employer, credit worker, on interval), only the negative case (no bypass) is scenario-covered.

## F5 — No scenario for "Give enterprises a separate accounting-rouble (beznal) settlement account"
- classification: missing-scenario
- source: docs/superpowers/iterations/extract/economy-trade.json (story "Give enterprises a separate accounting-rouble (beznal) settlement account")
- verbatim: "Internal (domestic, non-border) trades between enterprises settle in beznal with money_delta no longer hardcoded to zero, replacing today's price-free barter clearing."
- what should have covered it: story exists, no scenario owns it
- why it matters: AC-2 directly conflicts with the substrate ("every internal trade is money_delta: Money::ZERO today, economy/market.rs:226") — a substrate-conflicting AC with no scenario is unproven that domestic enterprise-to-enterprise trade actually moves beznal at all.

## F6 — No scenario for "Build customs houses as typed border-crossing buildings"
- classification: missing-scenario
- source: docs/superpowers/iterations/extract/economy-trade.json (story "Build customs houses as typed border-crossing buildings")
- verbatim: "A customs house is a placeable building entity distinct from ordinary production buildings, with fields for mode, per-transport-class buffer (1-unit buffer per cargo class), bay list, domestic edge, and border edge."
- what should have covered it: story exists, no scenario owns it
- why it matters: no scenario ever exercises actually placing/constructing a customs house or checking its fields exist — the other border scenarios all assume a customs house already exists and test order flow through it, never the building's own placement/shape.

## F7 — No scenario for "Gate the import/export catalogue by era and bloc"
- classification: missing-scenario
- source: docs/superpowers/iterations/extract/economy-trade.json (story "Gate the import/export catalogue by era and bloc"); spec/trade.md:42-44
- verbatim: "Goods and vehicles carry availability era + origin country (`$AVAILABLE 1969 1987`, `$COUNTRY` — §C1). The trade catalogue changes across the campaign timeline and with bloc alignment — trade as a geopolitical lever."
- what should have covered it: story exists, no scenario owns it
- why it matters: the AC states an order outside the availability window or bloc should be "rejected at customs" but nothing ever simulates placing such an out-of-window/out-of-bloc order to observe the rejection.

## F8 — No scenario for "Depreciate exported vehicles by condition on resale"
- classification: missing-scenario
- source: docs/superpowers/iterations/extract/economy-trade.json (story "Depreciate exported vehicles by condition on resale"); spec/trade.md:46-48
- verbatim: "Exported vehicles fetch full price only when new; condition discounts resale (§C3). Trade value = f(condition, market, era) — no churning new vehicles across the border for free money."
- what should have covered it: story exists, no scenario owns it
- why it matters: this is explicitly an anti-exploit rule ("no churning new vehicles across the border for free money") with no scenario ever exporting a used vs. new vehicle to confirm the discount is real.

## F9 — No scenario for "Publish the world-market price model to the player"
- classification: missing-scenario
- source: docs/superpowers/iterations/extract/economy-trade.json (story "Publish the world-market price model to the player"); spec/trade.md:30-33
- verbatim: "**OURS:** adopt the moving market (reacts to era, world events, and our own export volume) but **publish the model** — expose the price curve and its drivers to the player, fixing W&R's black box (§G)."
- what should have covered it: story exists, no scenario owns it
- why it matters: "published (non-opaque) market" is called out as this domain's own design fix (OURS, not inherited from W&R) yet nothing ever simulates a price move and checks the player-facing view actually surfaces the driver that caused it.

## F10 — CS1 rejected shop-of-last-resort contrast is prose context, correctly unextracted directly, but its positive obligation ("no map-edge partner reachable without a built customs house") is only asserted, never scenario-tested
- classification: missing-scenario
- source: spec/trade.md:24
- verbatim: "Contrast dropped: CS1's map edge as `OutsideConnectionAI` posting **unlimited priority-0 offers** — an infinite fixed-price shop-of-last-resort with no building to construct (§D1). That is the abstraction we reject."
- what should have covered it: story "Build customs houses as typed border-crossing buildings" AC-2 states the rule but no scenario proves an order placed with zero customs houses built is rejected/unfulfillable
- why it matters: this is the concrete falsifiable form of "no infinite external partner" — without a scenario that tries to trade with no customs house built and observes failure, the rejection of CS1's model is asserted but not provable.

## F11 — Price-at-order vs price-at-clearance question and `priceAtOrder?` field
- classification: intentional-exclusion
- source: spec/trade.md:57, 63
- verbatim: "Price at order time or at clearance time? (Moving market makes this a real hedging question.)"
- what should have covered it: n/a — this is an explicitly open, undecided design question in the source, not a settled requirement
- why it matters: correctly excluded; nothing to extract until the question is resolved, though a future iteration should flag it as still open rather than silently forgotten.

## F12 — Foreign labour deferral question
- classification: intentional-exclusion
- source: spec/trade.md:64
- verbatim: "Foreign labour (W&R has it) — defer to a later spec batch?"
- what should have covered it: n/a — explicitly an open question about deferring to a future spec batch, not a decided requirement
- why it matters: correctly excluded; this is unlike the dual-currency situation (where charter-1.0.md overrode a CONFIRMED/adopted line) — here the source itself never asserts foreign labour as adopted, it only poses the question.

TOTAL FINDINGS: 10
