# Docs review synthesis — 2026-09-03

Seven read-only reviewers, one slice each. Per-slice reports: `local://review-{authority,product,economy,society,architecture,engineering,research}.md`.
Totals: high 9, medium 49, low 22 (some overlap across slices; the cross-cutting items below de-duplicate them).

## Cross-cutting themes (ranked)

### 1. The border-rouble rule is still broken in code, and the docs say it is half-fixed
- `market.rs:640-653` attaches `money_delta` to external imports/exports; `economy/mod.rs:103-104` applies it at match time, before `advance_dispatches` (`:124-133`). Only buyer *stock* waits for the truck.
- `reference/architecture/substrate.md:63-65` and `mechanics-index.md:30-31` now say "import half fixed, export-only violation". That is wrong: money clears at match for both.
- Import also has no accountable source stock: the external seller is a FreightStation with only waiting/wanted counters; `ledger.rs:639-710` checks timing, not border conservation. Mechanics-index row 31 "EXISTS" should be PARTIAL.
- Bounded Loading/Returning route failures remove the dispatch after the seller was debited (`retail.rs` scenario asserts 5 units → 0 on both sides). `invariants.md:14-15` claims conservation; no loss sink exists.
- Job-opening bypasses the physical path entirely (`market.rs:599-606`) while `physical-economy/index.md:20-42` says every good traverses all 13 states.
(economy H×3, research H×1, architecture M)

### 2. The determinism story is overstated everywhere it is told
- `test_world_survives_serde` (`test_iso.rs:247`) uses `SeqSchedule::default()` → zero systems run. `is_equal` (`lib.rs:214-232`) compares registered resources only, never `World`. A system reorder or ECS-only divergence stays green.
- Three mismatch branches (sim vs sim2, deser vs each); "failure means replay diverged" in `developer/debugging-determinism.md:15-20` is false for the serde branches. Only ticks where `tick % check_size == 0` are compared; tail unobserved. `.max(3)` floor is load-bearing (SimulationOptions absent before tick 3).
- `common::rand` is not stateless: `RandGen` LCG used in `terrain.rs:243-251`. `RandProvider` is a live registered resource, not retired. Fixture Init omits `seed` (default 123).
- `TestCtx::check_determinism` *does* hash World; docs say resource-only. Documented in `architecture/determinism.md`, `simulation-phases.md`, `engineering/determinism.md`, `developer/debugging-determinism.md` — four pages, all wrong the same way.
(engineering H×2 + M×5, architecture M)

### 3. Documented test commands run zero tests
- `cargo test -p simulation sentinel`: `scenarios/mod.rs:5-12` lists six sentinel IDs, no `fn` contains `sentinel`. Exits green with 0 tests, violating `testing.md:16-17` MUST.
- `cargo test -p simulation evid_logistics` (in `writing-evidence-tests.md:57-59` and AGENTS.md): no `fn evid_` exists. Prefix census: evid 0 / scenario 30 / sov 12 / other 3 = 45. Only 8/30 scenario tests carry the required `<nnnn>` ID.
- `sov_ahw_*` cited as model comment does not exist. Helpers documented in `scenarios/mod.rs` actually live in `hoarding.rs`/`inflation.rs`. Scenario count 43 vs actual 42.
(engineering H×1, M×4)

### 4. `process/development-cycle.md` was not updated for the b25a04b roster cut
- Phase 0 dispatches archived `substrate-cartographer` (`:76,95`); Phase 2 lists seven archived implementer lanes (`:148-153`); Phase 6/7 rows name archived doc-reality/release/perf.
- Gate formula `.beads/formulas/gate-chain.formula.toml:17-38` runs wiring→ledger→domain→review; process says wiring→ledger→review→domain. Formula labels review "opus"; reviewer is fable/medium.
- Advisor sign-off: process makes it conditional on Phase-0 divergence; kornai/logistics bodies say unconditional hard sign-off. Ledger trigger: process "economy" vs role "economy, market, dispatch, storage, trade".
- `dev-cycle` skill mandates `bd show`, `bd mol pour`, scale announcement, end-run sweep — none in the process doc it calls SSoT.
- Global `~/.claude/agents/reviewer.md` does not include SHARED.md; renderer ownership ambiguous (UI row vs archived engine lane).
- Linked "unresolved" swarmforge review (`:218-219`) describes the pre-cut roster. `Last verified` predates the cut.
(engineering H×1, M×6, L×2)

### 5. Binding 1.0 scope is expanded outside the charter
- `charter-1.0.md:42-52` has nine rows; no Households/citizens row, no Utilities row. Yet `scope-1.0.md:25-26`, `design-bible.md:335`, `index.md:48` claim households/housing shortage and full electricity/water/sewage/heating/waste as 1.0. The portal says it must summarize the charter (`:4-10`).
- `index.md:52` "Everything else is Post-1.0" misclassifies charter-committed agriculture, shell, presentation, distribution.
- Water: `design-bible.md:267` and `infrastructure/water.md:9-47` treat pressure/head/tank as 1.0 binding; `product/post-1.0.md:47` and `design-bible.md:335` defer them.
- `planned-economy/reliability-and-buffering.md:9-10` says "1.0 binding", `reserves.md:9` "1.0 candidate", while `post-1.0.md:15-18` defers adaptive request inflation/reserve classes.
- Design-bible: dead paths (`docs/vision/game-modes-post-1p0.md`, `docs/research/rust-architecture-proposals-2026-08-28.md`), stale CODE anchors, §5.7 "Planner computes hidden reserve from physical stock" vs no-omniscience law.
(product M×3 L×4, society L)

### 6. The doc checker passes because it checks little; metadata is inconsistent
- `check_docs.py:133-145` requires metadata only for specs + 8 wiki sections, first 20 lines, five fields, never `Verified-at`. Research is excluded → all ten conversation-mining lanes (A,B1,B2,C2,D,E,F,G,H) have no metadata and pass. `explanation/` excluded from orphan check → `beads-oh-my-pi-integration-2026-08-30.md` not in SUMMARY.
- `AGENTS.md` rewrite dropped the five-line operational header required by `document-authority.md:32-34`.
- Taxonomy split: `templates/research.md` says Kind explanation/Authority explanatory; authority model has no "explanatory"; pages split between the two. `templates/generated.md` (generated/derived/active) vs generator output (generated roadmap/reporting only/draft).
- `architecture/current-substrate.md` absent from authority hierarchy while index points readers to it AND `reference/architecture/substrate.md` — two competing substrate maps, neither declared canonical.
- `engineering/documentation.md` rules 1-5, 7, 10-11 are semantic and unautomated; it does not say so.
(authority H×1 M×5, engineering M, architecture M)

### 7. `wave1-economy.md` fact-sheet is patched with drift notes rather than rewritten
- ECO-SUB-002 body still says imports credit buyers directly; ECO-SUB-005 heading/body still "test-only/unreachable" while `goods_company.rs:22-25` calls `set_requested` with the multiplier in production. Needs row 17 stale (eat now debits stock via `settle_retail`); ECO-SUB-001 says citizens don't repost (they do, `buyfood.rs:100-115`). Cited line numbers stale (`market.rs:441` → `455`; `market.rs:500-501` → `459-461`).
- `index.md:60` cites this sheet as the evidence for current substrate.
(research M×2, architecture M×2, economy L)

### 8. Simulation pages overstate implementation
- Society: `provisioning.md`/`healthcare.md` consumption claims stale; `workplaces.md` names wrong struct fields; mechanics-index has no Crime or Resources-catalogue row; infra index present-tense "solver" overclaim; `electricity.md:50-60` current binary blackout contradicts its own target.
- Economy: `enterprise-behavior.md:97-103` (and two other pages) attribute `request_multiplier 3` to slaughterhouse; Lua declares it on meat-facility; slaughterhouse defaults to 1. Reports page calls match-time EcoStats "Received/Consumed". Logistics page says trucks released unparked — they park first; says all failures retry — Loading/Returning are bounded and delete cargo.
- Architecture: `simulation-phases.md` omits native instant-command fast path (`native_app/src/network.rs:47-56` applies `is_instant` commands outside the schedule). `performance.md:12-16` still lists fixed sov-bo3 OOM as blocker and cites an archived agent report.
- Physical-economy `allocation.md` duplicates planned-economy `allocation.md`.
(society M×3 L×3, economy M×4, architecture M×3)

### 9. Dependency/engineering standards drift from config
- `engineering/dependencies.md:17-19` calls bytemuck/arc-swap/quickcheck transitive (they are direct); fixedbitset absent. Rule "no path deps" contradicts `deny.toml` `allow-wildcard-paths`. Architecture bans (rule 6) have no `[bans]` entries — not gate-enforced. `all-features=true`, yanked/unsound/unmaintained, confidence 0.8 undocumented.
- `code-intelligence.md:116-117` says Agent/WebFetch absent; `subagent-tooling.md` + SHARED say reachable. `bd-capability-survey.md` contradicts itself on BEADS_ACTOR and the bd prime hook (now duplicated by `.claude/settings.json:9`).
(engineering M×6 L×3)

## Suggested ordering for fixes
1. Code: border settlement (theme 1) — this is a model-rule violation, not a doc bug. File as defect; docs then follow.
2. Code: loss sink for bounded dispatch failure (theme 1).
3. Docs: rewrite four determinism pages from one truthful paragraph (theme 2); fix or remove sentinel/evid commands (theme 3).
4. Process: reconcile development-cycle.md ↔ gate formula ↔ agent frontmatter ↔ dev-cycle skill (theme 4). One owner, one pass.
5. Charter: add Households and Utilities rows or demote them everywhere (theme 5). Decide Water pressure/tank.
6. Checker: extend metadata/orphan scope to research + explanation + root entrypoints; pick one taxonomy (theme 6).
7. Rewrite wave1-economy in place; declare one substrate map canonical (themes 6-7).
8. Sweep simulation pages with the classification tables in review-economy/society (theme 8).
