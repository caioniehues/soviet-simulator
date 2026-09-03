# Product / vision / scope review

## Summary

- The binding charter and the 1.0 portal agree on the release boundary at the level of their nine scope rows; the design bible repeats the major boundary, while the operational index is a deliberately compressed view.
- The index's final sentence nevertheless turns omissions from its short 1.0 summary into an inaccurate blanket Post-1.0 classification.
- The design bible has one substantive scope-status conflict: its Water row presents pressure/head/tank storage as a 1.0 draft while the product Post-1.0 direction explicitly defers those mechanics.
- The design bible's thesis, laws, observability, and implementation sequence duplicate material already present in `docs/product/`; most of its architecture subsections are useful synthesis, but should point to the canonical product/spec/proposal page rather than restate it.
- The generated roadmap is reproducible (`build_roadmap.py --check` validated 21 requirements); the docs checker found no errors, only the two existing duplicate-H1 warnings.

## Findings

### 1. medium — Remove the blanket Post-1.0 classification from the entrypoint

**Evidence:** `docs/index.md:45-52` presents only a compressed subset of the 1.0 boundary and then says “Everything else is Post-1.0.” The binding charter separately commits agriculture and services, all Planner interaction, shell and comfort, presentation and audio, and distribution at `docs/plan/charter-1.0.md:44-52`; the 1.0 portal repeats those rows at `docs/product/scope-1.0.md:16-26`. These omitted rows are not in the Post-1.0 list (`docs/product/post-1.0.md:14-51`). A contributor using the operational front door can therefore classify, for example, the main menu/autosaves, pollution coupling, or English/Linux/Windows distribution as deferred even though they are binding 1.0 commitments.

**Proposed fix:** Replace the final sentence with a pointer that distinguishes omitted details from future scope, such as “The full 1.0 boundary is the [1.0 portal] and [charter]; only the charter cuts and the linked Post-1.0 page are deferred.” Alternatively, include every charter row in the index summary.

### 2. medium — Reconcile Water pressure and tank storage with the 1.0 cut line

**Evidence:** The design bible’s Water row says the network’s 1.0 draft model is “pressure and tank storage” and specifies static head, pump power, and tank drain at `docs/vision/design-bible.md:264-267`. Its scope summary then lists “water pressure” among “Hooks now, mechanics later” at `docs/vision/design-bible.md:333-335`. The product Post-1.0 page explicitly places “Water pressure and head, tank storage” under Infrastructure at `docs/product/post-1.0.md:45-51`, while the binding charter only commits Water as a utility/no-cargo and reservoir/hydro scope (`docs/plan/charter-1.0.md:44,47-48`). The sibling Water concept also labels the pressure model “1.0 binding” (`docs/simulation/infrastructure/water.md:9-47`), so this is a live competing scope signal rather than a wording difference.

**Proposed fix:** Mark pressure/head/tanks as Post-1.0 in the design bible and Water concept, and leave the 1.0 row to describe the charter’s utility, reservoir/hydro, finite transfer, and no-cargo boundary. If pressure is intended to be 1.0, revise the binding charter and Post-1.0 page together instead of leaving the conflict for implementers to resolve.

### 3. medium — Repair the design bible’s dead source paths

**Evidence:** The design bible directs readers to `docs/vision/game-modes-post-1p0.md` at `docs/vision/design-bible.md:242,292,369`, but that file does not exist; the current page is `docs/product/game-modes.md:1-8`, which is explicitly the Post-1.0 direction. It likewise names `docs/research/rust-architecture-proposals-2026-08-28.md` at `docs/vision/design-bible.md:296,371`, but the current research page is `docs/research/engineering/rust-architecture-crates.md:1-8`. These are inline code paths rather than Markdown links, so the checker cannot catch them: `scripts/check_docs.py:38-43` defines the Markdown-link check and `scripts/check_docs.py:54-68` only resolves parsed link targets. Following the design bible’s architecture or mode pointers therefore stops at nonexistent files even though `python3 scripts/check_docs.py` passes.

**Proposed fix:** Replace every mode reference with `docs/product/game-modes.md` and every Rust research reference with `docs/research/engineering/rust-architecture-crates.md`; use real Markdown links or add a narrowly scoped code-path check if code spans remain.

### 4. low — Refresh line-numbered CODE evidence in the design bible

**Evidence:** `docs/vision/design-bible.md:169` cites `freight_station.rs:139` as the cargo counter, but the current source has the counters at `simulation/src/souls/freight_station.rs:33-40` and line 139 is only the `waiting_cargo + wanted_cargo < 10` threshold (`simulation/src/souls/freight_station.rs:136-142`). `docs/vision/design-bible.md:341` cites `market.rs:774` for the export-physicality work, but the current external export debit/trade append is at `simulation/src/economy/market.rs:717-740`, while `simulation/src/economy/market.rs:766-779` is `release_tosource_truck`. The page labels these as current `CODE` evidence (`docs/vision/design-bible.md:9-11`) but is verified at the older commit shown at `docs/vision/design-bible.md:7`.

**Proposed fix:** Re-verify the page against the current commit and update the anchors, preferably replacing mutable line numbers with symbol/file references as recommended by `docs/meta/documentation-model.md:101-105`.

### 5. low — Correct the ledger-test count used as current evidence

**Evidence:** The design bible claims “13 ledger tests, 14 retail tests” at `docs/vision/design-bible.md:167` (and repeats “13 ledger tests” at `docs/vision/design-bible.md:327`). The current ledger scenario file contains 12 `#[test]` declarations at `simulation/src/tests/scenarios/ledger.rs:64,90,134,222,266,333,393,492,595,722,881,1046`; the retail file does have 14 declarations, including the last at `simulation/src/tests/scenarios/retail.rs:949-950`. This makes the evidence-strength statement factually stale even though the broader claim that the truck leg is tested remains true.

**Proposed fix:** Change the ledger count to 12 in both locations, or avoid embedding volatile test counts and link to the scenario module instead.

### 6. low — Point the game-mode provenance at an existing section

**Evidence:** `docs/product/game-modes.md:8` says the “who decides” questions come from design bible §8.14. The current design bible has only `## 8. Labour, workplace, unions, representation` followed by bullets, with no `§8.14` subsection (`docs/vision/design-bible.md:230-242`). A reader cannot navigate to the cited subsection to recover the provenance or verify the claim.

**Proposed fix:** Cite the actual Lane H report section or the existing `design-bible.md#8-labour-workplace-unions-representation--lead-audited-synthesis-35` heading, and identify the specific bullet if one is intended.

### 7. low — Replace the nonexistent Design Laws §22 citation

**Evidence:** `docs/product/simulation-philosophy.md:47-49` links to the current Design Laws page and attributes the physical-chain anti-pattern to “§22 of the design thread.” The linked page defines laws 1–20 and then “Two rules above the laws,” ending at `docs/product/design-laws.md:37-49`; it has no §22. The citation therefore sends readers to a section that is absent from the linked document, and the current page has no stable target for the asserted anti-pattern.

**Proposed fix:** Link to an existing heading or anchor that contains the anti-pattern, or add a named section to `design-laws.md` and cite that stable heading rather than an unresolvable thread section number.

## Scope-claim table

Legend: **A** = the claim is stated with the same meaning; **N** = the page does not repeat that detail (an omission, not a contradiction); **P** = partially represented; **D** = contradictory status/wording. The charter/portal are the scope authorities; the design bible and index are explanatory/operational summaries, so omissions only become defects when they are presented as an exhaustive classification.

| Atomic 1.0 commitment | Binding charter / 1.0 portal | Design bible | `docs/index.md` | Review note |
|---|---|---|---|---|
| 15 domestic resources; 12 new recipe buildings | A — `docs/plan/charter-1.0.md:44`; `docs/product/scope-1.0.md:16` | P — 15 + Medicine is repeated, but the 12-building count is not (`docs/vision/design-bible.md:165,335`) | P — 15 + Medicine (`docs/index.md:47`) | Current Lua has 21 items including `job-opening` and no Medicine (`base_mod/items.lua:1-109`); the portal explicitly records this as an unimplemented gap (`docs/product/scope-1.0.md:54-59`). |
| Food and Meat are separate dwelling needs | A — `charter-1.0.md:44`; `scope-1.0.md:16` | A — `design-bible.md:197,335` | A — `index.md:48` | No drift. |
| Water is a utility and never cargo | A — `charter-1.0.md:44,48`; `scope-1.0.md:16` | A — `design-bible.md:165,267,335` | N — not in the short 1.0 paragraph | The separate pressure/head/tank status is the finding above, not a disagreement about no-cargo. |
| Medicine is the 16th, import-only resource | A — `charter-1.0.md:44,48`; `scope-1.0.md:16,20` | A — `design-bible.md:165,335` | A — `index.md:47` | Current source has no Medicine; `docs/simulation/physical-economy/resources.md:19-29` explicitly labels that as substrate gap. |
| Field-cycle farming and livestock conversion | A — `charter-1.0.md:45`; `scope-1.0.md:17` | N — not named in the bible’s scope roll-up | N — omitted | The omission is covered by the portal’s linked domain pages; it must not be read as Post-1.0. |
| Demographics including death | A — `charter-1.0.md:45`; `scope-1.0.md:17` | P — social/lifecycle material, but not the complete commitment (`design-bible.md:179,228,335`) | A — `index.md:48` | No direct contradiction. |
| Two education tiers; healthcare | A — `charter-1.0.md:45`; `scope-1.0.md:17` | A — `design-bible.md:236,335` | A — `index.md:48` | No direct contradiction. |
| Landfill and incinerator | A — `charter-1.0.md:45`; `scope-1.0.md:17` | A — `design-bible.md:335` | N — omitted | Omission is caught by the blanket sentence (Finding 1). |
| Placement snapping and rotation | A — `charter-1.0.md:46`; `scope-1.0.md:18` | N — not stated as a scope commitment | N — omitted | Construction chain is present, but these interaction requirements are not (`design-bible.md:175`). |
| Ghost exposes footprint, material bill, refusal; one coherent verdict | A — `charter-1.0.md:46`; `scope-1.0.md:18` | P — Ghost → Verdict → Site and material/work gates (`design-bible.md:175`) | N — omitted | No direct contradiction. |
| Rescind before ground breaks; selection/inspect depth; tooltips/icons/camera; no general undo | A — `charter-1.0.md:46`; `scope-1.0.md:18` | N — omitted | N — omitted | These need links or an explicit note, not a Post-1.0 classification. |
| Procedural seed maps and heightfield terrain | A — `charter-1.0.md:47`; `scope-1.0.md:19` | P — terrain is named but the generation/heightfield details are not (`design-bible.md:335`) | N — omitted | No direct contradiction. |
| Reservoir-graph water and hydro dams | A — `charter-1.0.md:47`; `scope-1.0.md:19` | A — `design-bible.md:335` | A — reservoir/hydro (`index.md:49-50`) | No direct contradiction. |
| Ore siting and minimal bridges | A — `charter-1.0.md:47`; `scope-1.0.md:19` | N — not named separately | N — omitted | No direct contradiction. |
| Pollution coupled to sickness, crop yield, basin water | A — `charter-1.0.md:47`; `scope-1.0.md:19` | P — pollution named, couplings not spelled out (`design-bible.md:335`) | N — omitted | No direct contradiction. |
| Minimal freight rail; 3 buildings; 1 locomotive; 1 wagon | A — `charter-1.0.md:48`; `scope-1.0.md:20` | A — `design-bible.md:254` | A — minimal rail (`index.md:49-50`) | No direct contradiction; the bible records missing cargo custody as absent substrate. |
| Fixed-consist border purchase; multiple customs offices | A — `charter-1.0.md:48`; `scope-1.0.md:20` | P — border/rail are named, office count and fixed-consist detail are not (`design-bible.md:254,335`) | P — one-rouble border trade only (`index.md:50`) | No direct contradiction. |
| All 16 resources tradeable at fixed per-kind prices in one rouble; Water never cargo; ratified trade mechanism | A — `charter-1.0.md:48`; `scope-1.0.md:20` | P — one rouble/no-cargo appears, but all details are not repeated (`design-bible.md:165,335`) | P — one-rouble border trade (`index.md:50`) | The trade specification resolves Water as a utility transfer; portal text should continue pointing to charter/spec, not imply cargo. |
| One fixed 1950s–60s era and flat catalogue | A — `charter-1.0.md:48`; `scope-1.0.md:20` | N — not explicit in roll-up | N — omitted | No direct contradiction. |
| Three authored Plans on one continuous save; procedural endless afterward | A — `charter-1.0.md:49`; `scope-1.0.md:21` | P — three Plans named and endless/sequence discussed (`design-bible.md:335,348-349`) | P — three Plans named (`index.md:50`) | The index omits “procedural endless,” so do not read omission as a cut. |
| First Plan teaches two hours without outside help | A — `charter-1.0.md:49`; `scope-1.0.md:21` | A — tutorial/First Plan (`design-bible.md:349`) | N — omitted | No direct contradiction. |
| Main menu; named manual saves; three rotating period-end autosaves | A — `charter-1.0.md:50`; `scope-1.0.md:22` | N — omitted | N — omitted | Included in Finding 1’s blanket misclassification. |
| Pause/date/speed; minimal settings; notifications/event log; HUD onboarding | A — `charter-1.0.md:50`; `scope-1.0.md:22` | P — notifications/HUD are in the sequence, other shell details omitted (`design-bible.md:339,349`) | N — omitted | No direct contradiction. |
| Camera polish; panic log; autosave on crash | A — `charter-1.0.md:50`; `scope-1.0.md:22` | P — polish and shell/save/crash are named as missing specs (`design-bible.md:339,349`) | N — omitted | No direct contradiction. |
| Zero-spend art/audio; grounded palette; bounded visible citizens; day/night/seasons | A — `charter-1.0.md:51`; `scope-1.0.md:23` | P — bounded citizens and day/night/seasons in the roll-up; art/audio/palette are omitted (`design-bible.md:335`) | N — omitted | No direct contradiction. |
| Legible state/refusal feedback; UI feedback; ambience; optional menu music | A — `charter-1.0.md:51`; `scope-1.0.md:23` | N — omitted | N — omitted | No direct contradiction. |
| English-only; fixed keybindings; no accessibility line item; no telemetry | A — `charter-1.0.md:52`; `scope-1.0.md:24` | N — omitted | N — omitted | Included in Finding 1’s blanket misclassification. |
| CI Linux/Windows binaries; unlisted itch build; friends-grade shell; stranger-grade visuals/feel | A — `charter-1.0.md:52`; `scope-1.0.md:24` | P — Linux/Windows named, the rest omitted (`design-bible.md:335`) | N — omitted | No direct contradiction. |
| Households/housing shortage; persistent individual identities | A — `charter-1.0.md:53` (identity pillar + scope row); `scope-1.0.md:25` | A — household/housing/identity sections (`design-bible.md:179,212,335`) | A — households/housing and identities (`index.md:48,50`) | No direct contradiction; all are explicitly absent in current substrate. |
| Electricity, water, sewage, heating, waste | A — `charter-1.0.md:44,47`; `scope-1.0.md:26` | A — utility table and roll-up (`design-bible.md:266-269,335`) | N — omitted | Water pressure status is the separate finding. |
| 250,000 citizen identities at 60 fps on development machine; target has no gate yet | A — `charter-1.0.md:54-58`; `scope-1.0.md:28-31` | A — target and no-gate posture (`design-bible.md:335,340`) | A — target (`index.md:50-51`) | Current benchmark lane was cancelled and no benchmark exists (`scope-1.0.md:28-31`); this is correctly labelled target, not completion. |
| Version-gated hard breaks during development; released saves compatible from 1.0 RC | A — `charter-1.0.md:59-61`; `scope-1.0.md:33-34` | P — migration seam is named, but compatibility policy is not repeated (`design-bible.md:339`) | N — omitted | No direct contradiction. |
| Completion requires current implementation evidence plus required acceptance evidence/capture | A — `charter-1.0.md:62-64`; `scope-1.0.md:50-52` | P — evidence sequence is described, but not the full charter sentence (`design-bible.md:339-342`) | N — omitted | Roadmap/status cannot replace this boundary. |
| Explicit Post-1.0 cuts and Never list | A — `charter-1.0.md:66-79`; `scope-1.0.md:36-49` | P — individual cuts are labelled (unions, gas, passenger rail, kindergarten), and §17 says charter cuts/Never are absolute (`design-bible.md:235,238,253,270,335`) | D/ambiguous — blanket `index.md:52` overstates all omissions as Post-1.0 | The index sentence is the only status error; the charter/portal cut lists agree. |

## Proposal review matrix

| Proposal | Authority/status check | Alignment with `docs/architecture/target-architecture.md` | Result |
|---|---|---|---|
| `docs/plan/proposals/citizen-architecture.md:1-8` | Clearly advisory; “decision (draft)” and “binds nothing until accepted” | Matches append-only dense citizen/household IDs, save migration seam, scheduled actors and benchmark path (`target-architecture.md:28-44`) | No conflict. |
| `docs/plan/proposals/sim-tick-phases.md:1-8` | Clearly advisory; “decision (draft)” and “binds nothing until accepted” | Matches labelled phases, keyed randomness, deterministic merge/commit, and lockstep constraints (`target-architecture.md:20-44`) | No conflict. |
| `docs/plan/proposals/causal-inspector.md:1-8` | Clearly advisory; “decision (draft)” and “binds nothing until accepted” | Matches immutable Planner snapshots, provenance, observatory, and causal facts (`target-architecture.md:35-44`) | No conflict. |
| `docs/plan/proposals/gosplan.md:1-12` | Clearly non-binding as written: “proposed — binds nothing until ... ADR-0001”; it supersedes the current process only **on ratification** | Process proposal, not runtime architecture; no target-architecture conflict | No authority conflict. Its current roster/scripts assumptions are noted below as consolidation/open items. |

## Consolidation proposals

1. **Make the charter the sole scope table.** Keep `docs/product/scope-1.0.md` as the navigable portal, but make `docs/index.md` a pointer rather than a second compressed classification. In `docs/vision/design-bible.md`, replace the repeated thesis (§1) with a short explanation plus links to `docs/product/vision.md`; replace §2 with a link to `docs/product/design-laws.md`; replace the long §17 roll-up with a charter link and a compact “what this page adds” list.
2. **Split the design bible by reader intent.** §4’s four realities and §16’s observability should point to `docs/simulation/concepts/information.md`, `docs/architecture/snapshots.md`, `docs/architecture/observatory.md`, and the causal-inspector proposal. §§5–6 should link to planned-economy/resource/logistics pages and retain only cross-domain synthesis; §7–8 should link to product/player-role and society concepts; §9–10 should link to transport/infrastructure concepts; §19 should link to design laws/simulation philosophy. This removes competing prose while preserving the bible as an evidence-labelled map.
3. **Resolve status labels before implementation dispatch.** Decide that pressure/head/tanks are Post-1.0 (consistent with charter/product portal) or move the Post-1.0 bullet; do not leave the design bible and Water concept with a 1.0-binding row while the product page defers it. Similarly, mark reserves/ratchet/adaptive inflation as Post-1.0 or explicitly label them 1.0 candidates: `docs/product/post-1.0.md:15-18` currently defers them, while `docs/simulation/planned-economy/reliability-and-buffering.md:9-10` and `docs/simulation/planned-economy/reserves.md:9-18` present them as 1.0 binding/candidate.
4. **Keep the four target architecture proposals as proposals.** Their current headers are unambiguous, and all four align with target architecture. If any is accepted, move the numbered decision into `docs/decisions/` and leave the proposal as a linked rationale; do not let “Decision proposed” headings become mechanism authority while all specs are draft.
5. **Treat Gosplan as a stale proposed process, not current authority.** It is marked non-binding, but its 20-agent claim and migration table (`docs/plan/proposals/gosplan.md:34-45,490-504`) predate the current eight-agent process (`docs/process/development-cycle.md:29-49`). Its stage/table references to `scripts/doc-check.sh`, `bd-close.sh`, `plan-metrics.sh`, and `check_traceability.py` (`gosplan.md:232-242,277-290`) describe future installation (`gosplan.md:500-504`), not available tooling. Refresh the proposal or clearly keep it as historical future design; do not dispatch against it as if it replaced the current cycle.
6. **Preserve generated-roadmap provenance.** `docs/generated/roadmap.md:3-8,12-14` names the generator and reports 21 requirements / 107 planned / 0 implemented. `docs/plan/iterations/build_roadmap.py:17-57,91-123` derives the counts and `--check` passed (`validated roadmap from 21 requirements`), so this file is genuinely generated, not a hand-edited competing truth. Keep it reporting-only and link it from status pages.
7. **Archive or isolate superseded migration plans.** `docs/plan/controlled-documentation-rewrite.md:5-7` and `docs/plan/documentation-migration.md:9-20` clearly say superseded, and `docs/SUMMARY.md:206-217` labels them as such, so they are not authority defects. They still contain obsolete paths such as `docs/generated/iterations/roadmap.md` (`controlled-documentation-rewrite.md:137-144`; `documentation-migration.md:49-52`). Move them under the historical/archive navigation or retain them only in an explicitly “superseded plans” section, without rewriting archive bodies.
8. **Make section/path checking cover the remaining failure mode.** The checker catches Markdown links but not inline backtick paths or section numbers; dead design-bible paths and §8.14/§22 references passed the checker. A lightweight check for `docs/...` code spans and local heading anchors would prevent this class of drift without making conceptual pages carry mutable line references.

## Out of slice

- `docs/simulation/infrastructure/water.md:9-47` and `docs/reference/specifications/water.md:9-17,69-83` independently call the pressure/head/tank model 1.0; the product/economy slice should reconcile this with Post-1.0.
- `docs/simulation/planned-economy/reliability-and-buffering.md:9-10` and `docs/simulation/planned-economy/reserves.md:9-18` conflict with the product Post-1.0 list for adaptive inflation, ratchet, credibility, and reserve classes; economy slice should decide status.
- The active docs checker run was `python3 scripts/check_docs.py`: 228 active files, 0 errors, warnings for duplicate H1 `Soviet Simulator` (`README.md` / `docs/SUMMARY.md`) and duplicate H1 `Allocation` (two physical-economy allocation pages). These are navigation/title warnings, not scope defects.
- Current source still has a physical-export gap documented as target work (`simulation/src/economy/market.rs:717-740`; `docs/vision/design-bible.md:341`); this review treats the model rule as design direction, not as a claim that all current code already satisfies it.

## Open questions

- Is the product intent that Water pressure/head/tank storage is Post-1.0, as the charter/portal and Post-1.0 page imply, or should the binding scope be revised to include it?
- Is the Planner allowed to infer hidden enterprise reserve from measured physical stock minus declared classes (`docs/vision/design-bible.md:128-129`; `docs/product/design-laws.md:24-27`; `docs/product/player-role.md:20-32`), or must the inspector expose only provenance-labelled discrepancy evidence? The current pages support both readings; no formal defect is assigned without this decision.
- Should the 20-agent Gosplan proposal be refreshed to the current eight-agent process before any ADR ratifies it?
- Should superseded plans remain in the active tree with explicit labels, or move to historical navigation under the archive policy?
- Which omitted charter rows should be added to the generated requirements/evidence graph (shell, presentation/audio, distribution, Plan/onboarding, agriculture, terrain/pollution)? The current generated roadmap intentionally reports 21 requirement contracts, while the charter has additional rows without stable specifications.
