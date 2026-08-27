# Memory Index

- [Six "empirical failures" — forensic verdicts](false-claims-failure-inventory.md) — 2 overstated, 1 REFUTED (time inverted), 1 still live in RESUME.md:84; verified at fdfabca
- [STORY-0107 hoarding-panel seam](seam-hoard-panel-story0107.md) — PARTLY SUPERSEDED: set_requested IS now production-called via request_multiplier; rest holds
- [Sim-wide structure](seam-simwide-structure-2026-08-27.md) — Refusal/Verdict types DO NOT EXIST; Dispatcher leaks 15 free() sites into market.rs; 8531d3c
- [Map model](seam-map-model-2026-08-27.md) — heightfields EXIST but land is clamped flat; Road::make discards TooSteep; no topology revision; 8531d3c
- [Persistence + determinism](seam-persistence-determinism-2026-08-27.md) — two-run determinism check EXISTS but can never fail; 1 of 16 resources guarded; 8531d3c
- [Inherited perimeter](seam-perimeter-native-app.md) — request_multiplier PROVEN to silently default on typo; native_app has ZERO tests; 8531d3c+dirty
- [Render/engine seam](seam-render-engine-2026-08-27.md) — colors.lua IS a palette authority; seasons ABSENT; headless is NOT a render adapter; 8531d3c
- [External-drive / MCP seam](seam-external-drive-mcp-2026-08-27.md) — new crate CAN own a Simulation; apply() returns () so failures are silent; AddTrain is a no-op; 0aa5c35
- [LSP read-guard relent](gotcha-lsp-read-guard-relent.md) — retry the same file 3x; batch files to pay 3 rounds total
- [Economy/logistics depth map](seam-economy-logistics-2026-08-27.md) — Market is two modules; sold.0 leaks for 6 stores; EcoStats counts matches not deliveries; 8531d3c
- [Data-layer seam](seam-data-layer-2026-08-27.md) — one Lua digit (amount=0) panics via unguarded Money div; leisure+road-vehicles dead; 8531d3c
- [W&R buildings .ini grammar](wr-buildings-ini-grammar.md) — verified directive counts; "1472 .ini" claim is false (only 488 are .ini); ported from prototype-researcher
- [Factorio recipe/machine binding](factorio-recipe-machine-binding.md) — crafting_categories many-to-many; field renamed plural in 2.1; ported from prototype-researcher
- [Neither source hot-reloads prototypes](hot-reload-neither-source-has-it.md) — Factorio confirmed no; W&R unknown; ported from prototype-researcher
