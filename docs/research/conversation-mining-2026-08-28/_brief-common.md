# Common brief — conversation mining 2026-08-28

You are one of ten Opus miners working in parallel on the same source. Read this file fully, then your lane brief, then the source.

## The source

`/home/caio/Downloads/soviet_simulator_conversation_export.md` — 1,210 lines. A GPT conversation export about this project. Section 1 is *reconstructed* context (not verbatim); section 2 holds six deep-dive passes (economy control loop, society/citizens, Rust architecture, vehicles/utilities, CIA-only society); section 3 is the closing thesis. Read all of it. It is the object of study, not an authority: every claim in it is a hypothesis until you validate it.

## The project

`/home/caio/soviet-simulator` — Rust, a hard fork of Egregoria (2026-08-22). Read `CLAUDE.md` at the repo root first. Key pointers:

- Simulation crate: `simulation/src/` — `economy/{market,government,ecostats}.rs`, `souls/{goods_company,human,freight_station}.rs`, `souls/desire/`, `map_dynamic/{dispatch,electricity,itinerary,router,parking}.rs`, `transportation/{vehicle,train,road,pedestrian}.rs`, `map/{electricity_cache,pathfinding,traffic_control}.rs`, `world.rs`, `tests/`.
- Data layer: `base_mod/*.lua`, `prototypes/`.
- UI: `native_app/`.
- Substrate map (cited fact-sheet): `docs/reference/architecture/substrate.md`; fact-sheets under `docs/research/fact-sheets/`.
- Ratified specifications: `docs/reference/specifications/*.md` (buildings, citizens, construction, education, electricity, healthcare, heating, households, logistics, needs, pathfinding, production, resources, roads, sewage, trade, traffic, vehicles, waste, water, zoning, crime).
- Charter: `docs/plan/charter-1.0.md`. **For this pass the charter does NOT filter ideas.** The user chose to ignore 1.0 scope. Do not reject or downgrade an idea because it is Post-1.0. You may note overlap with existing documents for deduplication only.
- Prior research you must not duplicate: `docs/research/architecture-review-2026-08-27.md`, `docs/research/agent-roster-review-2026-08-27.md`, `docs/research/research-synthesis-2026-08-27.html`, `docs/research/awesome-rust-project-fit.md`, `docs/plan/proposals/gosplan.md`, `docs/reference/glossary.md`, `docs/vision/`.
- Tests: `cargo test -p simulation` (parallel-safe).
- Knowledge graph MCP tools (`mcp__code-review-graph__*`) are available: `semantic_search_nodes_tool` for behaviour-described searches (misses ~34% — empty means unknown), `query_graph_tool` for callers/callees. You have no LSP; cite `file:line` from what you actually read.
- Workers & Resources: Soviet Republic is installed locally as a reference implementation; `docs/archive/agents-2026-09-02/substrate-cartographer.md` line 74 onward says where and how to read it. Use it when your lane compares mechanics to W&R.

## Design pillars that are not negotiable (from CLAUDE.md)

- Nothing teleports. Goods move physically or do not move.
- Never game over. Failure degrades into queues, shortages, colder homes.
- Clearing by queue, substitution and going without — never by price.
- The player is THE PLANNER. The core loop is the dishonest enterprise caught from observable state.

## What "deep mining" means here

1. **Extract** every distinct idea, mechanism, claim, number, rule, and named reference in your lane. Give each a stable id (`<LANE>-NN`) and a one-line statement.
2. **Validate** each: against the codebase (does a substrate exist? file:line), against external truth (crates.io, GitHub, papers, CIA FOIA reading room, W&R behaviour — use WebSearch/WebFetch; cite URLs), and against internal consistency (does it contradict another part of the conversation or a pillar?).
3. **Go deeper than the conversation did.** The conversation summarises; you work the mechanism. For each important idea: what state does it need, what wakes it, what does the player see, how does it fail, what is the cheapest representation, what is the test that proves it.
4. **Find what was missed or is not apparent.** Second-order effects, hidden coupling with other lanes, historical mechanisms the conversation skipped, assumptions that do not survive contact with the code, ideas that contradict each other, numbers pulled from nowhere.
5. **Verdict per item**: `CONFIRMED` / `PLAUSIBLE` / `UNSUPPORTED` / `WRONG` / `ALREADY-EXISTS` / `CONTRADICTS-PILLAR`, with the evidence.

## Output

Write your full report to `docs/research/conversation-mining-2026-08-28/<lane-file>.md` (the lane brief names the file). Length: as long as the evidence needs — 400 to 1,200 lines is normal. Structure:

```
# <Lane title>
## 0. Summary (≤ 30 lines: the ten most important findings, each with an id)
## 1. Extracted items (table: id | statement | source line(s) | verdict)
## 2. Validation detail (one subsection per item that needed real work; evidence, URLs, file:line)
## 3. Deeper mechanics (the design work the conversation did not do)
## 4. Missed / not apparent (numbered, each with why it matters)
## 5. Cross-lane hooks (what other lanes must know; name the lane)
## 6. Open questions for the user
## 7. Sources (every URL and file you relied on)
```

Do not commit. Do not modify any file outside your own report. Do not run `bd` mutations. Do not create bd issues.

Your final message to the lead must be **≤ 40 lines**: the report path, the ten headline findings by id, and anything the lead must know that is not in the report. The lead reads the report itself; do not paste it back.
