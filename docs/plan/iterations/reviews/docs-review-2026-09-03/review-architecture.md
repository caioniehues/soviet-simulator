# Architecture drift review

## Summary

- The current/target split is consistently labelled across the eight architecture pages, and the workspace count plus the `Arc<RwLock<Simulation>>` boundary are accurate; the main problems are stale evidence rows and over-strong claims about tests and runners.
- The determinism narrative is not safe to use as a gate: the replay test uses an empty schedule and `Simulation::is_equal` omits the ECS `World`, so it cannot detect system reorders or world-only divergence.
- The active economy fact-sheet still presents pre-`sov-abs` import behavior and pre-`sov-lpj` request reachability as current, despite drift notes and code proving the opposite; this creates competing physical-custody and domain-model truths.
- The active performance page still treats fixed `sov-bo3` as a blocker, cites an archived `perf-engineer.md`, and hides the existing renderer GPU baseline; the architecture also conflates the server-only headless binary with replay and a finite benchmark runner.
- The migration DAG needs one terminology/ordering correction: inert phase labels and typed contexts are different prerequisites, and a future scale gate must be a finite benchmark rather than the infinite lockstep server loop.

## Findings

### 1. Severity: med — Qualify the replay test's determinism coverage

**Evidence:** `docs/architecture/determinism.md:14-20` describes `test_world_survives_serde` as replaying twice and comparing, while `docs/architecture/simulation-phases.md:36-39` says it will detect any system reorder. The actual test constructs `SeqSchedule::default()` at `simulation/src/tests/test_iso.rs:247`, so no registered system executes. Its comparisons call `Simulation::is_equal`, whose implementation only iterates serialized resources and never compares `self.world` (`simulation/src/lib.rs:214-233`). The separate `hashes()` helper does hash `world` (`simulation/src/lib.rs:267-276`), but the replay test does not use it. `docs/reference/architecture/substrate.md:31-38` also has a blanket “no repeat-run determinism” row that is now wrong for the resource half even though it remains true for full system/world coverage.

**Impact:** A system reorder or an ECS-only mutation can leave `test_world_survives_serde` green, so the current pages make the replay test look like a phase/replay gate when it is only a command/resource comparison over an empty schedule. This directly undermines the target's stated requirement that a repeat-run test exist before reorders (`docs/architecture/target-architecture.md:61-67`).

**Proposed fix:** Make one canonical coverage table in `docs/architecture/determinism.md` and link to it from `current-substrate.md`, `simulation-phases.md`, and `reference/architecture/substrate.md`: `TestCtx::check_determinism` is serialization round-trip coverage including the world; `test_world_survives_serde` is a two-run resource-only check over an empty schedule; full system/world repeat-run coverage is absent. Remove the claim that the current test detects reorders until that contract is actually implemented.

### 2. Severity: med — Document the native single-player instant-command path

**Evidence:** `docs/architecture/simulation-phases.md:31-39` presents `COMMAND` as `WorldCommand::apply` before `SeqSchedule::execute`, but the native single-player path first applies every command for which `WorldCommand::is_instant` is true and then clears the commands (`native_app/src/network.rs:47-56`). The four bypass variants are defined at `simulation/src/world_command.rs:212-224` (`MapBuildHouse`, `MapUpdateIntersectionPolicy`, `UpdateZone`, and `SetGameTime`). Only non-instant commands are handed to `Simulation::tick`, where the schedule runs. `current-substrate.md:35-37` mentions that four instant commands bypass ticking, but the phase page does not scope its command seam to `Simulation::tick` versus the application fast path.

**Impact:** The phase/replay description implies that all command application participates in the tick schedule, while paused native single-player map/zone/intersection/time commands mutate the simulation without a schedule pass. A migration that labels only `SeqSchedule` or assumes one command entry path can therefore report the wrong phase/tick semantics and replay behavior.

**Proposed fix:** In the current section, explicitly distinguish `Simulation::tick` ordering from the native fast path: list the four instant variants, say they call `apply` directly and are cleared without a schedule tick, and state that non-instant commands force one tick while paused. Link the same wording from `current-substrate.md` so the command contract has one authority.

### 3. Severity: med — Refresh the import side of ECO-SUB-002

**Evidence:** The active table still says “Imports credit buyers directly” (`docs/research/fact-sheets/wave1-economy.md:24`) and ECO-SUB-002 repeats the old import-credit/teleport behavior (`docs/research/fact-sheets/wave1-economy.md:34-41`), even though its own drift note says the import side became physical (`docs/research/fact-sheets/wave1-economy.md:43-47`). Current code pushes an external buy into `all_trades` before the dispatch loop and explicitly performs no capital move there (`simulation/src/economy/market.rs:618-652`); the resulting dispatch starts in `ToSource` (`simulation/src/economy/market.rs:695-702`), and the implementation documents seller debit/buyer credit at physical endpoints (`simulation/src/economy/market.rs:755-760`).

**Impact:** An active “current reality” row still violates the core physical-goods rule for imports and conflicts with `current-substrate.md:51-57` and the physical import scenario. The remaining nuance is separate: `economy::market_update` still applies the external `money_delta` to `Government.money` at match time (`simulation/src/economy/mod.rs:95-104`), so the fact-sheet must distinguish physical goods movement from border-rouble settlement rather than retaining the obsolete all-in-one claim.

**Proposed fix:** Rewrite the table and ECO-SUB-002 as: import goods use a freight-station `Dispatch` and settle buyer/seller stock at endpoints; external roubles are still applied through `trade.money_delta` at match time; export goods still debit seller stock at match time and create no dispatch. Update the citations, classification, commit/verification date, and leave the historical pre-`sov-abs` wording only in the drift history.

### 4. Severity: med — Reclassify ECO-SUB-005 and its verification boundary

**Evidence:** `docs/research/fact-sheets/wave1-economy.md:64-72` still titles the behavior “test-only,” says `set_requested` has no non-test caller, and classifies it `UNREACHABLE AND UNOBSERVABLE`. The later drift note admits reachability is stale and requests `REACHABLE, UNOBSERVABLE` (`docs/research/fact-sheets/wave1-economy.md:73-79`), but the active Verification boundary still says searches found no non-test caller (`docs/research/fact-sheets/wave1-economy.md:100-103`). Production `recipe_init` calls `market.set_requested` with `amount * request_multiplier` (`simulation/src/souls/goods_company.rs:16-27`), and the setter is production API code (`simulation/src/economy/market.rs:454-461`).

**Impact:** The active fact-sheet presents two mutually exclusive domain models for the dishonest-enterprise mechanic, and its verification paragraph tells a future auditor the opposite of the current source. The current-substrate page has already corrected the reachability statement (`docs/architecture/current-substrate.md:51-55`), so the fact-sheet is now the competing stale authority in the consolidation trail.

**Proposed fix:** Change the ECO-SUB-005 title/body/classification and Verification boundary to `REACHABLE, UNOBSERVABLE`, cite `recipe_init` and the production scenario, and update `Last verified`/`Commit`. Keep the old no-caller claim only as a dated drift note, not as the active current row.

### 5. Severity: med — Remove the fixed `sov-bo3` blocker and archived evidence citation

**Evidence:** `docs/architecture/performance.md:12-16` says `sov-bo3` still causes a 17.6 GB OOM and blocks construction of a 250k-building city, then cites a bare `perf-engineer.md`. The current implementation bounds `LAV::iter_keys` to `vs.len() + 1` and marks a corrupt cycle instead of collecting forever (`geom/src/skeleton.rs:738-754`); `skeleton` then refuses the corrupt polygon (`geom/src/skeleton.rs:910-912`). The named `perf-engineer.md` is under the historical tree (`docs/archive/agents-2026-09-02/perf-engineer.md:1-3`), not an active authority. The fix and refusal test are in current history (`4d1d18b`, `ca3ef2e`).

**Impact:** The active performance plan says a resolved defect still blocks the scale path and relies on an archive file for evidence. That can misorder migration work and violates the rule that archive material is provenance, not active authority.

**Proposed fix:** Replace the blocker sentence with the current status of `sov-bo3` (fixed; any remaining scale gate is unbuilt rather than blocked by that defect), cite `geom/src/skeleton.rs` and the current tests/issue, and remove the bare archive citation. Do not edit the archived file.

### 6. Severity: med — Separate headless server, replay, and finite benchmark roles

**Evidence:** `docs/architecture/current-substrate.md:20-22` labels `headless` a “server/replay runner,” while `docs/architecture/migration-sequence.md:39-42` calls for a “headless 250k benchmark”; `docs/architecture/performance.md:50-55` repeats the headless benchmark as the final gate. The actual binary loads or creates a `Simulation`, starts `networking::Server`, and enters an unbounded poll/sleep loop that ticks only merged server inputs (`headless/src/main.rs:33-71`); it contains no replay loader. Replay advancement is in the native app (`native_app/src/network.rs:82-95`) and the simulation replay loader (`simulation/src/lib.rs:154-171`).

**Impact:** Implementers can extend the infinite multiplayer server loop for a benchmark or assume the headless binary provides replay execution, contrary to the finite-runner boundary recorded for the cancelled `sov-1ae` lane. This also makes the current-substrate workspace map and the migration deliverable describe different binaries for the same role.

**Proposed fix:** Name `headless` only as the lockstep server runner; name `SimulationReplayLoader`/native replay as replay support; and call any future scale gate a finite, fixed-seed simulation benchmark (not “headless” unless a finite mode is deliberately designed). Record whether the cancelled `sov-1ae` contract is replaced before retaining it as an M1 milestone.

### 7. Severity: low — Distinguish inert phase labels from typed phase contexts

**Evidence:** The migration dependency graph places `typed system contexts` before `labelled phases` before deterministic parallelism (`docs/architecture/migration-sequence.md:13-18`), but Milestone 0 schedules “phase labels without reorder” without a typed-context step (`docs/architecture/migration-sequence.md:32-37`). The parallelism page separately requires keyed randomness, typed contexts, labelled phases, and repeat-run/per-phase digests before parallel workers (`docs/architecture/parallelism.md:27-38`).

**Impact:** The graph and milestone text can be followed in two incompatible ways: add harmless telemetry labels first, or treat labels as semantic contexts that require typed access boundaries first. That ambiguity matters because the target claims labels are prerequisites for parallelism, while the first migration milestone explicitly promises no behavioral change.

**Proposed fix:** Rename the M0 item to “inert phase metadata/labels without reorder,” and add a later explicit “typed phase contexts/barriers” prerequisite before deterministic parallelism; alternatively move typed contexts into M0 if labels are intended to carry access semantics.

### 8. Severity: low — Scope the “no profiling baseline” statement

**Evidence:** `docs/architecture/performance.md:18-22` says no profiling baseline is recorded in the repository. The repository contains a renderer GPU timing baseline with adapter, scene, per-pass medians, tolerance, and rank order (`engine_demo/gpu_timing_baselines/radv-navi3x/baseline.json:1-25`, committed by `2fcf527`).

**Impact:** Readers cannot tell whether the page means “no simulation CPU/whole-world baseline” or “no profiling evidence at all”; the latter is false and hides the existing renderer evidence from the architecture inventory.

**Proposed fix:** Change the sentence to “No simulation CPU or whole-world baseline is recorded,” and link the renderer-only GPU baseline with its adapter/scope caveat.

### 9. Severity: low — Correct the scenario-test count

**Evidence:** `docs/architecture/current-substrate.md:116-119` says there are 43 scenario tests. The seven listed scenario modules currently contain 42 `#[test]` functions: `hoarding.rs:137,185,230` (3), `inflation.rs:82,142` (2), `ledger.rs:64,89,134,222,266,333,393,492,594,721,880,1045` (12), `mod.rs:27` (1), `recipe_provided.rs:43,116,147,194,230` (5), `retail.rs:52,111,145,240,323,369,458,526,574,616,684,749,831,949` (14), and `validation.rs:67,78,89,102,113` (5).

**Impact:** The current test inventory overstates scenario coverage by one and makes the active map disagree with the source it purports to inventory.

**Proposed fix:** Change 43 to 42 (or generate the inventory count from the source), and state whether the non-corpus `scenario_harness_smoke` is included so the counting rule remains stable.

### 10. Severity: low — Add source-commit metadata to current-bearing architecture pages

**Evidence:** The documentation standard requires a `Verified-at` field when a substantial page makes implementation claims (`docs/engineering/documentation.md:27-30`), and the authority guide says implementation claims should carry a practical commit (`docs/meta/document-authority.md:70-73`). `docs/architecture/current-substrate.md:1-8` has `Verified-at`, but current-bearing pages such as `docs/architecture/determinism.md:1-8`, `simulation-phases.md:1-8`, `performance.md:1-8`, and `routing.md:1-8` only carry `Last verified` while describing concrete implementation behavior below their headers.

**Impact:** A date alone does not identify which source tree was inspected, which makes the stale claims above difficult to reproduce and allows secondary pages to drift from the canonical current-substrate map.

**Proposed fix:** Add `Verified-at: <commit>` to each architecture page that retains implementation claims, including the reference substrate map, or remove those claims and link to the canonical current-substrate sections. Keep target-only pages date-scoped if they contain no implementation assertions.

## Consolidation proposals

1. **Make `current-substrate.md` the sole narrative current-state map.** Keep `docs/reference/architecture/substrate.md` as a compact evidence ledger and keep each fact-sheet as a source-specific ledger, but remove repeated full system/test lists from the secondary pages. In particular, have `simulation-phases.md` link to the current-substrate system order and only add phase labels/target-order analysis; this prevents the 18-system list and test semantics drifting independently.
2. **Introduce one “current truth matrix” for the economy seam.** For each trade path, record separately: matching time, goods custody location, seller debit, buyer credit, rouble delta, dispatch/claim object, failure recovery, and observability. Generate the current-substrate economy paragraph and fact-sheet rows from that matrix; this makes import goods physical while preserving the separate match-time border-rouble fact.
3. **Add a standard evidence header field for source scope.** `Last verified` and commit alone are not enough when a page cites a later drift note or an archive artifact. Record `source scope` (e.g. “simulation runtime,” “renderer GPU baseline,” “historical agent report”) and require active pages to link only active evidence except explicitly labelled provenance.
4. **Use a role glossary for binaries and test harnesses.** Define `native_app` (interactive single-player/multiplayer/replay UI), `headless` (lockstep server), `SimulationReplayLoader` (replay mechanism), `TestCtx` (round-trip hash harness), and “finite benchmark runner” as separate terms. Reuse those names in architecture, engineering, and developer docs.
5. **Make migration edges executable or issue-linked.** Keep the advisory sequence, but add a small “edge contract” table for each milestone: prerequisite, behavior preserved, digest/test gate, save impact, and replacement bead/decision. In particular, record a replacement for cancelled `sov-1ae` before M1 calls for a benchmark.
6. **Make draft-spec language uniform.** Where architecture pages quote a draft mechanism, say “draft SPEC proposes/requires” and link the register; do not let current-state sections read a draft as already binding.

## Out of slice

- `docs/engineering/benchmarking.md:29-31`, `docs/developer/benchmarking.md:29-30`, and `docs/developer/profiling.md:38-40` repeat the stale `sov-bo3` blocker; these should be handled in the engineering/process slice, not by editing archive material.
- No simulation-internal `Arc<RwLock<Simulation>>` leak was found: the lock is at the native application boundary (`native_app/src/game_loop.rs`), while `simulation/src` uses resource-level locks/command-buffer mutexes.
- `python3 scripts/check_docs.py` was already run for this review: 228 active files, 0 errors, 2 duplicate-H1 warnings; no dead architecture links were reported.

## Open questions

- Should the project revive a scale-proof benchmark after cancelling `sov-1ae`, and if so should its contract explicitly be a finite CPU simulation runner rather than the server binary?
- Is match-time `Government.money` movement for border trades intentional while goods/agent stock settle at physical endpoints, or should rouble settlement move to the customs event in a future contract?
- Is `test_world_survives_serde` intended to remain a cheap resource-only smoke test, or is it the intended place for full scheduled-world repeat-run coverage once `sov-n8v`/`sov-y66` are addressed?
- Are M0 phase labels deliberately telemetry-only, or are they meant to establish the typed access boundaries required for parallel phases?
