## F1 — Per-currency loans (interest, penalty rate, borrowing caps) entirely absent
- classification: omission
- source: spec/trade.md:22-23 (Dual currency section) and evidence log row
- verbatim: "Separate loans per currency with own interest and borrowing caps (§C2 — CONFIRMED from UI strings)."
- what should have covered it: no story / no scenario
- why it matters: the draft data model even has a `loans: [{currency, principal, rate, penaltyRate}]` field on `Treasury` (spec/trade.md Data block) — a whole mechanic with a data shape is missing from every extracted story, not just thin.

## F2 — Utility border buildings (electricity transformers, border pipelines) not extracted
- classification: omission
- source: spec/trade.md:14-15
- verbatim: "**Utility border buildings** — electricity import/export transformers and border pipelines are separate typed endpoints (§A3). The transport medium of a good decides *which* border building it crosses at, same as domestically (INFERRED)."
- what should have covered it: no story / no scenario (the extraction's "Build customs houses as typed border-crossing buildings" story only covers road/rail/air customs, not electricity/pipe endpoints)
- why it matters: this is a distinct border-building typology called out in its own evidence-log row (`eletric_transformator_custom*.ini`, `foreign_pipe_player.ini`); silently folding it into "customs house" loses a requirement.

## F3 — Player-buildable customs variant (`$SUBTYPE_OWN_CUSTOM`) not extracted
- classification: omission
- source: spec/trade.md:13
- verbatim: "Road, rail, and air variants; player-buildable variants exist (`$SUBTYPE_OWN_CUSTOM`)."
- what should have covered it: "Build customs houses as typed border-crossing buildings" story has no AC distinguishing a player-built customs house from a pre-placed/map one
- why it matters: the distinction (can the player construct their own customs house vs only use existing ones) is a placement/ownership rule with no AC or scenario proving it.

## F4 — Standing plan import/export contracts (quota mechanism) not extracted
- classification: omission
- source: spec/trade.md:60-61 (Open questions)
- verbatim: "Import contracts planned or reactive? → partially open: orders are placed at customs (W&R shape), but *who* places them — plan quotas, or deficit-driven like spec/logistics.md dispatch? Lean: plan sets standing import/export contracts; logistics fulfils."
- what should have covered it: no story / no scenario
- why it matters: this is exactly the "planning artifacts — quotas, plan-driven contracts" category the brief flags as high-value; the spec's own working lean ("plan sets standing import/export contracts") never became a testable AC anywhere, even as a deferred/open placeholder.

## F5 — Price-at-order-time vs price-at-clearance-time (hedging question) not extracted
- classification: omission
- source: spec/trade.md:62
- verbatim: "Price at order time or at clearance time? (Moving market makes this a real hedging question.)"
- what should have covered it: no story / no scenario ("Settle foreign trade only on physical border clearance" covers *when the treasury debit happens*, but not *which price snapshot* is used — the TradeOrder data model has a distinct `priceAtOrder?` field for exactly this)
- why it matters: given a moving/published market, whether the order locks in the price at placement or floats to the clearance-time price is an observable, player-facing rule with zero AC coverage.

## F6 — Foreign labour explicitly flagged for later, correctly unextracted
- classification: intentional-exclusion
- source: spec/trade.md:63
- verbatim: "Foreign labour (W&R has it) — defer to a later spec batch?"
- what should have covered it: n/a
- why it matters: the spec itself marks this as open/deferred to a future batch, so its absence from this extraction is correct, not an omission.

## F7 — Seven of fourteen stories have zero scenario coverage
- classification: missing-scenario
- source: docs/superpowers/iterations/extract/economy-trade.json (stories vs scenarios arrays)
- verbatim: "\"scenarios\": [ ... 7 entries ... ]" against 14 entries in "\"stories\": [ ... ]"
- what should have covered it: the following stories own no scenario at all: "Give households a cash balance (nal)", "Pay wages from employer to worker in nal", "Give enterprises a separate accounting-rouble (beznal) settlement account", "Build customs houses as typed border-crossing buildings", "Gate the import/export catalogue by era and bloc", "Depreciate exported vehicles by condition on resale", "Publish the world-market price model to the player"
- why it matters: each of these has behavior-changing ACs (e.g. nal balance persistence, wage debit/credit, catalogue rejection at customs, condition-based resale value, published price curve) with no scenario proving the observable end-to-end behavior — this is exactly the thin ratio the brief calls out.

## F8 — Settlement "tag import/export" requirement not represented as an AC
- classification: thin-AC
- source: spec/trade.md:26
- verbatim: "Book trade as an explicit paired ledger entry (CS1's `EconomyManager` credit-seller/debit-buyer at delivery is a clean pattern — §D3), but: split by currency, tag import/export, and settle **only on physical border clearance** — treasury and simulation never diverge (§G)."
- what should have covered it: "Settle foreign trade only on physical border clearance" story covers the clearance-timing half and "Track two hard-currency ledgers" covers the currency-split half, but no AC anywhere requires a settled trade to be tagged/recorded as import vs export
- why it matters: without an import/export tag on the ledger entry, per-direction reporting (e.g. trade balance, plan fulfilment on exports) has no substrate requirement to point to.

## F9 — CS1 "unlimited priority-0 offer" contrast is correctly excluded as rejected-design narrative
- classification: intentional-exclusion
- source: spec/trade.md:16-17
- verbatim: "Contrast dropped: CS1's map edge as `OutsideConnectionAI` posting **unlimited priority-0 offers** — an infinite fixed-price shop-of-last-resort with no building to construct (§D1). That is the abstraction we reject."
- what should have covered it: n/a — covered indirectly by "Build customs houses" AC-2 ("no map-edge 'infinite external partner'")
- why it matters: this is explanatory rationale for a decision already captured by an AC; re-extracting it as a separate requirement would be redundant, not missing.

## F10 — Citation imbalance: spec/trade.md under-mined relative to the substrate audit
- classification: omission
- source: docs/superpowers/iterations/extract/economy-trade.json (sources arrays, counted)
- verbatim: "\"sources\": [{\"file\": \"docs/egregoria-substrate-audit.md\", ...}]" appears as the sole or first source on 9 of 14 stories, while spec/trade.md appears alone as source for only 5 stories ("Settle foreign trade only on physical border clearance", "Track two hard-currency ledgers, roubles and dollars", "Gate the import/export catalogue by era and bloc", "Depreciate exported vehicles by condition on resale", "Publish the world-market price model to the player")
- what should have covered it: cross-reference F1–F5 above — every one of those gaps is content that lives only in spec/trade.md (loans §C2, utility border buildings §A3, player-buildable customs §A2, plan-contract open question, price-timing open question) and none of it made it into a story
- why it matters: confirms the brief's flagged pattern — the extraction leaned on the substrate audit (what code exists/is absent) more than on the design spec (what the design actually requires), so spec-only requirements with no corresponding audit line got dropped.

TOTAL FINDINGS: 8
