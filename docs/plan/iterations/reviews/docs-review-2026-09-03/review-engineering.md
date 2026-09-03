# Engineering-slice documentation review

## Summary (5 bullets max)

- The active simulation test tree has **45** `#[test]` functions: `evid_` 0, `scenario_` 30, `sov_` 12, and 3 other names. The seven `scenarios` modules contain 42 tests; the getting-started page's "43 scenario tests" is stale.
- The documented sentinel and evidence filters are both vacuous. `sentinel` matches no active test, and `evid_logistics` matches no active `evid_` test.
- The determinism guide overstates `test_world_survives_serde`: it uses an empty schedule, compares only registered-resource serialisations, and never compares `World`; `eed5ead` made the test fail after narrowing, but the docs do not describe that behavior.
- The dependency command/source allowlist is aligned, but the transitive-dependency inventory and source-enforcement wording drift from `Cargo.toml`/`deny.toml`; several deny semantics and all lifecycle/design rules remain manual and unlabeled.
- The new eight-agent roster is not applied consistently through the cycle, two tooling references disagree about subagent capabilities, and the Beads survey contains contradictory (and now stale) provenance/hook/telemetry snapshots.

## Test-prefix count table

| Prefix among active `#[test]` functions under `simulation/src/tests/**` | Count | Evidence / notes |
|---|---:|---|
| `evid_` | 0 | No active `fn evid_...`; `docs/engineering/testing.md:18-20` acknowledges the 107 planned rows are not implemented. |
| `scenario_` | 30 | Numeric examples are in `scenarios/hoarding.rs:138,186,231` and `recipe_provided.rs:44,117,148,195,231`; the remaining 22 names are listed under Findings 4. |
| `sov_` | 12 | `inflation.rs:83,143`, `ledger.rs:65,90,595,722,1046`, and `validation.rs:68,79,90,103,114`. |
| Other active names | 3 | `test_iso.rs:66,242` and `vehicles.rs:13`; the rest of `vehicles.rs:75-219` is inside a block comment. |
| **Total** | **45** | 42 tests in the seven `scenarios` modules plus two `test_iso` tests and one active vehicle test. |

## Findings

### 1. high — Make the sentinel filter select a real test

**Evidence:** `docs/engineering/testing.md:16-17` makes a non-zero filter a MUST and says the zero-test sentinel run is the failure the rule prevents. `simulation/src/tests/scenarios/mod.rs:5-12` still advertises six sentinel IDs and `cargo test -p simulation sentinel`, but no active test declaration contains `sentinel` (the only `sentinel` matches are those comments). The same module's only local test is `scenario_harness_smoke` at `:28`, and it is explicitly not corpus-numbered. The filter therefore selects zero tests and exits green, exactly the vacuous check the standard says is forbidden.

**Proposed fix:** Add/rename the six live sentinel tests and keep the command, or remove the sentinel command and mark the set as not yet implemented until those tests exist. Run the filter and record the `running N tests` line with `N >= 1`.

### 2. medium — Do not present the future evidence filter as executable now

**Evidence:** `docs/developer/writing-evidence-tests.md:57-59` presents `cargo test -p simulation evid_logistics -- --nocapture` and requires `N >= 1`, while the source count has zero active `fn evid_...` declarations. The `evid_logistics_...` in the guide's Rust block at `:26-28` is only a Markdown example. A contributor following the guide gets a green zero-test result.

**Proposed fix:** Use an existing filter (for example `scenario_0082` or `sov_lpj`) for the guide's smoke command, and label the `evid_logistics` command as a future command until its first evidence test lands; alternatively land the evidence test before documenting the command.

### 3. medium — Point the helper documentation at the modules that define the helpers

**Evidence:** `docs/engineering/testing.md:39-41` says `build_company_at`, `setup_seller_buyer`, `drain_dispatches`, and `remove_default_freight_station` live in `tests/scenarios/mod.rs`. `simulation/src/tests/scenarios/mod.rs:17-22` only declares the child modules and contains the harness smoke test. The shared helpers are in `simulation/src/tests/scenarios/hoarding.rs:24-25,51-52,92-93`; `remove_default_freight_station` is in `inflation.rs:39-40`; `ledger.rs:14-18` imports them from those modules. The guide's example comment also calls them "scenarios/mod.rs helpers" at `docs/developer/writing-evidence-tests.md:26-28`.

**Proposed fix:** Link to the defining modules, or deliberately centralize/re-export the helpers in `scenarios/mod.rs` and update all callers. Also distinguish the private duplicate `build_company_at` helpers in `inflation.rs:56-59` and `recipe_provided.rs:18-18`.

### 4. medium — Reconcile the required test-name shape with the live names

**Evidence:** `docs/developer/writing-evidence-tests.md:18-19` requires `scenario_<nnnn>_<behaviour>`. Only eight `scenario_` names have the numeric shape: hoarding (`simulation/src/tests/scenarios/hoarding.rs:138,186,231`) and recipe-provided (`recipe_provided.rs:44,117,148,195,231`). Twenty-two active names do not: `scenarios/mod.rs:28`; `ledger.rs:135,223,267,334,394,493,881`; and `retail.rs:53,112,146,241,324,370,459,527,575,617,685,750,832,950`. Three active tests also have no documented `evid_`/`scenario_`/`sov_` prefix (`test_iso.rs:66,242`; `vehicles.rs:13`).

**Proposed fix:** Rename issue/story tests to include their corpus IDs, and document explicit exceptions for harness/unit tests; otherwise weaken the guide's contract to state that the prefix alone is the supported convention. Keep the generated evidence mapping and filters in the same change.

### 5. low — Document the final-tick determinism check

**Evidence:** Both `docs/engineering/testing.md:37-39` and `docs/developer/writing-evidence-tests.md:37-39` describe `advance_ticks(n)` as checking every 25 ticks. `simulation/src/tests/mod.rs:86-101` checks every 25th tick **and** `i + 1 == n`, so a short `advance_ticks(5)` still performs one round-trip check and a run ending between 25-tick boundaries is checked at its final tick.

**Proposed fix:** State "every 25 ticks and on the final tick" in both guides.

### 6. low — Replace the nonexistent `sov_ahw_*` model reference

**Evidence:** `docs/developer/writing-evidence-tests.md:20-21` tells authors to use the `sov_ahw_*` test comment as the model, but no current `simulation/src/tests/**` function has that name. A real causal-chain comment exists beside `sov_jcl_outbound_loading_route_failure` in `simulation/src/tests/scenarios/ledger.rs:700-727`.

**Proposed fix:** Point the guide at the existing `sov_jcl...` example (or another live test), or label `sov_ahw_*` as a planned example instead of an existing model.

### 7. high — Describe the replay test as a limited resource/command check, not full determinism

**Evidence:** `docs/developer/debugging-determinism.md:15-20` says `test_world_survives_serde` means two replays produced different state. The test constructs `let mut s = SeqSchedule::default()` at `simulation/src/tests/test_iso.rs:247`; `SeqSchedule::default()` has an empty `systems` vector (`simulation/src/utils/scheduler.rs:27-29`), and `execute` only runs that vector (`:39-44`). The populated schedule is `Simulation::schedule()` (`simulation/src/lib.rs:138-145`). Thus this test does not execute the simulation systems, including scheduler ordering and system-side RNG/ECS work.

The test's three comparisons at `simulation/src/tests/test_iso.rs:277-302` call `Simulation::is_equal`; that method only counts resources and compares `saveload_funcs()` bytes (`simulation/src/lib.rs:214-232`) and never reads `World`. `Simulation::hashes()` does hash `World` (`simulation/src/lib.rs:268-275`), but this test never calls it. An entity/component divergence can therefore pass. `docs/engineering/determinism.md:53-55` also calls this a replay-based check without stating these limits.

**Proposed fix:** State that the current fixture exercises replay command application, time/replay bookkeeping, and registered-resource serialisation only. Do not cite it as system, scheduler, ECS-World, or full repeat-run determinism evidence until the test uses a populated schedule and compares authoritative world plus resources.

### 8. medium — Distinguish replay-path divergence from serialisation mismatch

**Evidence:** The test compares `sim` against `sim2`, then the decoded `deser` against `sim`, and then `deser` against `sim2` (`simulation/src/tests/test_iso.rs:274-303`). A decode mismatch can fail even when both replay paths agree. `Simulation::is_equal` writes the differing registered-resource JSON files before returning false (`simulation/src/lib.rs:219-228`). The guide currently collapses every failure to "the replay diverged" at `docs/developer/debugging-determinism.md:12-17`.

**Proposed fix:** Document the three failure branches and direct debugging to the resource dumps; call the first branch replay-path divergence and the latter two serialisation/equality mismatches.

### 9. medium — Narrow the claim that the bisection finds the first divergent tick

**Evidence:** `docs/developer/debugging-determinism.md:15-17,24-26` promises that the loop bisects to the first divergent tick. The loop checks only `tick % check_size == 0` (`simulation/src/tests/test_iso.rs:263-265`), starts at `check_size = 1024` (`:249`), and the loader stops at its recorded end (`simulation/src/utils/replay.rs:69-78`). There is no forced final checkpoint. A divergence between checkpoints that resolves before the next checkpoint, or in the unobserved tail after the last multiple of 1024, is not observed; no phase/system attribution exists either.

**Proposed fix:** Say that the loop narrows a detected checkpoint window and does not guarantee the first divergent tick. Either add a final/targeted checkpoint if that guarantee is required, or keep the weaker wording.

### 10. medium — Record the post-`eed5ead` failure behavior and dump path

**Evidence:** Before `eed5ead`, each mismatch arm could halve `check_size` until the loop returned normally. Current code records `divergence_tick`, dumps `world`/`world2`, narrows with `(tick - check_size).max(3)`, and continues (`simulation/src/tests/test_iso.rs:277-302`), then panics after narrowing at `:311-312`. The commit is `eed5ead` ("fail determinism divergence after narrowing"). Neither `docs/developer/debugging-determinism.md:12-20` nor `docs/engineering/determinism.md:51-55` says that a detected mismatch now fails the test after the diagnostic narrowing, or that the dumps are produced.

**Proposed fix:** Add the armed-after-narrowing failure and dump behavior to the guide, and preserve the `.max(3)` floor when describing or changing this test.

### 11. low — Do not regenerate the command fixture merely to bless changed output

**Evidence:** `docs/developer/debugging-determinism.md:35-36` says to regenerate `world_replay.json` after an intended behavior change. The fixture is an included command log (`simulation/src/tests/test_iso.rs:10`); `Replay` stores only `enabled`, commands, and `last_tick_recorded` (`simulation/src/utils/replay.rs:8-11`), and the test has no golden digest/output. Regenerating it does not update expected state; it changes the input command stream. `Simulation::from_replay` reconstructs a fresh sim from registry init (`simulation/src/lib.rs:154-174`).

**Proposed fix:** Say to change the fixture only for an intentional command-stream/schema change. For behavior changes, keep the command log and update the test's actual determinism expectations or add a real golden digest.

### 12. medium — Correct the transitive-dependency inventory

**Evidence:** `docs/engineering/dependencies.md:17-19` asserts that `enum-map`, `bytemuck`, `arc-swap`, `tracing`, `quickcheck`, and `fixedbitset` are transitive. `arc-swap` is declared directly in `simulation/Cargo.toml:26`; `quickcheck` is a direct simulation dev-dependency at `:36`; `bytemuck` is direct in `engine/Cargo.toml:24` and `native_app/Cargo.toml:9`. `enum-map` and `tracing` do have transitive paths (`Cargo.lock:875-878,1663-1672`), but no `fixedbitset` package is present in the current lockfile. The list is therefore not a reliable "already transitive" inventory.

**Proposed fix:** Re-run and date the inventory, separating direct, dev-direct, and transitive dependencies; remove `fixedbitset` unless it returns to the graph. Or change the rule to require `cargo tree -i` verification without asserting these six current states.

### 13. medium — Separate local path dependencies from externally allowed sources

**Evidence:** `docs/engineering/dependencies.md:13-14` says "only crates.io and the two allowed git sources" and says no new Git **or path** source may be added without a policy change. `deny.toml:82-89` does enforce unknown registries/Git and allows only the two Git URLs, but `deny.toml:70-80` sets `bans.allow-wildcard-paths = true` specifically to allow versionless path/workspace dependencies. Existing local paths are normal, for example `native_app/Cargo.toml:10-17`. The workflow only runs `cargo-deny check` (`.github/workflows/dependency-policy.yml:50-57`); it has no path-origin policy check. A new local/vendor path can therefore pass the stated gate without the policy change the prose requires.

**Proposed fix:** Rewrite the rule as "crates.io and the two Git sources for external dependencies; local workspace/path dependencies are allowed under the private-workspace rule; new external/vendored paths require review," or add a dedicated path-origin check.

### 14. medium — Document the deny settings that define the actual dependency gate

**Evidence:** The process/engineering pages describe the checker and source/license allowlists (`docs/process/dependency-policy.md:27-59`, `docs/engineering/dependencies.md:11-29`) but omit several active settings: `deny.toml:3-4` enables `graph.all-features`; `:33-35` sets `yanked = "deny"`, `unmaintained = "all"`, and `unsound = "all"`; and `:37-38` sets the license confidence threshold to `0.8`. These settings change what `cargo-deny check` evaluates, yet a reader cannot derive them from the policy prose.

**Proposed fix:** Add a compact config-to-policy table, or explicitly label these as implementation details outside the promised baseline.

### 15. medium — Mark lockfile freshness, exception expiry, and design bans as manual controls

**Evidence:** The docs require re-recording after every `Cargo.lock` change (`docs/engineering/dependencies.md:11-12`; `docs/process/dependency-policy.md:47-52`) and renewal/removal by `2026-11-25` (`docs/process/dependency-policy.md:142-176`), but the workflow only installs, version-checks, and runs cargo-deny (`.github/workflows/dependency-policy.yml:40-57`). Likewise, the ECS/async/concurrent-map/nightly-SIMD bans at `docs/engineering/dependencies.md:22-23` have no corresponding deny entries (`deny.toml:70-80`). These are valid process controls only if they are explicitly manual; currently the pages present them beside the enforced baseline without an enforcement boundary.

**Proposed fix:** Add a checker for lockfile baseline/exception expiry and a static/design review gate for architecture bans, or label each rule `manual review` with owner, evidence, and check procedure.

### 16. high — Route every cycle phase to an available roster agent

**Evidence:** The new roster says there are eight agents, seven in `.claude/agents/` plus the global reviewer (`docs/process/development-cycle.md:35-47`), and the archive policy says the other fifteen definitions are in `docs/archive/agents-2026-09-02/` (`:49-53`). However, Phase 0 still requires `substrate-cartographer` (`:76-96`), which is only in that archive; Phase 2 lists `data-implementer`, `engine-implementer`, `geom-implementer`, `widget-implementer`, `net-implementer`, and `common-implementer` (`:147-153`), also archive-only. Phase 6 requires `doc-reality-auditor` (`:236-237`), Phase 7 lists `release-engineer` and `perf-engineer` (`:263-264`), and the advisor table names archive-only `utilities-modeller`, `settlement-modeller`, and `soviet-authenticity` (`:277-281`). The current `.claude/agents/` directory contains only logistics-modeller, kornai-economist, evidence-auditor, ledger-invariant-checker, wiring-auditor, ui-implementer, sim-implementer, and SHARED.md. Following the process literally attempts unavailable dispatches in required phases.

**Proposed fix:** Make the phase tables use the seven live definitions plus the global reviewer, and state an explicit "restore with `git mv` before dispatch" step for optional archive lanes. Do not leave archive-only names in mandatory phase instructions without that step.

### 17. medium — Make cycle tier labels agree with agent frontmatter

**Evidence:** Phase 3 labels `evidence-auditor` as `sonnet` at `docs/process/development-cycle.md:171-174`, but `.claude/agents/evidence-auditor.md:5-7` declares `model: fable` and `effort: medium`. The cycle itself says frontmatter is authoritative at `:45-47` and `:57-58`, so the phase label contradicts its own authority rule.

**Proposed fix:** Replace `sonnet` with the frontmatter tier (`fable@medium`) or remove model labels from prose and generate them from the agent metadata.

### 18. low — Put the gate-chain command in the cycle's canonical process

**Evidence:** The dev-cycle skill requires the poured gate chain `bd mol pour gate-chain --var story=<id> --var scope=<range>` at `.claude/skills/dev-cycle/SKILL.md:26-28`. `docs/process/development-cycle.md:186-218` describes the ordered gates but has no `bd mol pour`/`gate-chain` instruction. The same command is separately recorded in the adopted Beads conventions at `docs/reference/bd-capability-survey.md:224-232`. The skill says the process document is the single source of truth (`.claude/skills/dev-cycle/SKILL.md:6-8`), so following the process page alone omits the required gate-chain instantiation.

**Proposed fix:** Add the command and skip-if-existing rule to Phase 4 in `development-cycle.md`, or remove it from the skill and keep one canonical operational location.

### 19. medium — Fix the getting-started test count

**Evidence:** `docs/developer/getting-started.md:29-36` says "43 scenario tests plus the serialisation round-trip test." The current source has 42 active tests in the seven `scenarios` modules: hoarding 3, inflation 2, ledger 12, module smoke 1, recipe-provided 5, retail 14, validation 5 (`simulation/src/tests/scenarios/{hoarding,inflation,ledger,mod,recipe_provided,retail,validation}.rs`, declarations cited in the count table). It also has `test_iso.rs:66,242` and the one active vehicle test at `vehicles.rs:13`, for 45 total. The `scenario_0151` filter itself is valid at `hoarding.rs:230-231`; only the count sentence is wrong.

**Proposed fix:** Replace 43 with 42 and call out the two `test_iso` tests plus the active vehicle test, or avoid a hard-coded count and point to the named test inventory.

### 20. medium — Enforce the metadata contract on all substantial active pages

**Evidence:** `docs/engineering/documentation.md:28-29` requires Kind, Authority, Status, Owner, Last verified on every substantial page and `Verified-at` when a page makes implementation claims. `scripts/check_docs.py:133-145` checks only specification pages and the eight `WIKI_SECTIONS`, and checks only the five `META_FIELDS`; it never checks `Verified-at`. `docs/process/mutation-trial-sov-mwy.md:3-9` is an active process page with no `Last verified`, but it is outside `WIKI_SECTIONS` (`scripts/check_docs.py:18-28`) and passes. `docs/engineering/testing.md:36-41` makes current `TestCtx` implementation claims but has no `Verified-at`; it is checked for the five fields only. The one allowed checker run reported `228 active files checked; 0 error(s), 2 warning(s)`, demonstrating this gap.

**Proposed fix:** Expand the checker to process/engineering/developer/reference active pages and enforce `Verified-at` where implementation claims are marked, or narrow the documentation standard to the exact checker scope. Add a test/fixture for a process page missing `Last verified`.

### 21. medium — Consolidate the contradictory subagent capability truth

**Evidence:** `docs/reference/code-intelligence.md:116-121` says subagents lack `Agent` and `WebFetch`. The later, dedicated reference says the opposite: `docs/reference/subagent-tooling.md:31-45` records a control probe with full `Agent,WebFetch` schemas and marks both `yes` for unrestricted arms; `:61-64` explains only a `tools:` allowlist removes them. `.claude/agents/SHARED.md:32-45` also tells workers they may spawn `Agent` helpers and that `Agent`/`WebFetch` are reachable. These competing instructions change how a worker is briefed and whether it can fan out.

**Proposed fix:** Keep the later settled probe and SHARED rule as the canonical truth, update `code-intelligence.md`'s toolset table, and add a cross-link/date so future probe updates change one record.

### 22. low — Update the stale `set_requested` source citation

**Evidence:** `docs/reference/code-intelligence.md:79-80` cites `simulation/src/economy/market.rs:441` for `Market::set_requested`. The current declaration is at `simulation/src/economy/market.rs:453-455`, and the warmed language-server lookup resolves four references at `hoarding.rs:269-270` and `goods_company.rs:24`. A reader sent to line 441 lands in the preceding `reserved` method rather than the symbol discussed.

**Proposed fix:** Update the citation to the current declaration and either record a commit or avoid a brittle line number by citing the symbol plus file.

### 23. medium — Remove the obsolete Beads provenance instructions

**Evidence:** The survey's capability map tells workers to use `--actor`/`$BEADS_ACTOR` and says `prepare-commit-msg` adds an `Executed-By:` trailer (`docs/reference/bd-capability-survey.md:94-98`); its recommendations repeat that workers should set `BEADS_ACTOR` (`:149-157`). The adopted-conventions section says the opposite: the convention was deleted, the hook is inert, and workers must not set it (`:213-216`). Current `CLAUDE.md:77-87` repeats `--author` and says not to set `BEADS_ACTOR`. This contradiction can produce the provenance behavior the project explicitly removed.

**Proposed fix:** Delete or clearly mark the old capability-map/recommendation paragraphs as historical, leaving `--author <roster-name>` as the one active convention.

### 24. medium — Reconcile the Beads hook-fix claim with the checked-in hook

**Evidence:** The survey says the duplicate `bd prime` was fixed by deleting the project hook, leaving the plugin as the sole owner (`docs/reference/bd-capability-survey.md:234-254`). The current working tree adds a `SessionStart` hook that runs `bd prime` in `.claude/settings.json:3-11` (the diff adds it), while the survey itself says the plugin is installed and was the other owner (`:239-252`). This restores the exact two-hook configuration the survey calls a duplicate, so sessions can receive `bd prime` twice again.

**Proposed fix:** Either remove the project `bd prime` hook and keep the plugin-owned hook, or update the survey to the deliberate two-hook design and document why duplicate injection is acceptable. Verify a session receives the context once.

### 25. medium — Refresh the Beads telemetry status in one place

**Evidence:** The survey's verified local-state table says telemetry is ON and `metrics.disabled = false` (`docs/reference/bd-capability-survey.md:39-42`), and its recommendation repeats "Currently ON" (`:163-165`). The adopted-conventions section later says telemetry is disabled (`:231-232`). The current effective `bd config show` reports `metrics.disabled = true` from `/home/caio/.config/bd/config.yaml`. The same page therefore gives opposite operational advice and a stale snapshot.

**Proposed fix:** Keep one current-state entry with its verification date/source, move old ON data into a clearly historical note, and make the recommendation conditional on the effective user-level setting.

## Consolidation proposals

1. Make `docs/process/development-cycle.md` the sole process authority and generate its roster/tier tables from `.claude/agents` metadata where practical. Keep archive-only roles in an explicitly optional restoration section. Add the skill's `bd mol pour` command to Phase 4.
2. Create one test-inventory source (or a tiny checked-in report) that records 45 active tests and prefix counts. Have getting-started/testing/current-substrate link to it instead of embedding divergent 43/45/107 numbers.
3. Split `docs/engineering/documentation.md` into a written-contract table and a checker-enforcement table. Include active scope, warning-vs-error behavior, metadata fields, `Verified-at`, archive treatment, SUMMARY targets, duplicate H1s, and fragment handling.
4. Keep one determinism page for current evidence and one for target rules. Put the exact limits of `test_world_survives_serde` and `TestCtx` in the current-evidence section, and link the `eed5ead` failure semantics.
5. Split dependency policy into machine-enforced (`deny.toml` + CI) and manual (`Cargo.lock` re-record, rev pinning, architecture bans, rationale, expiry) sections. Re-run the transitive inventory and date it.
6. Treat `docs/reference/subagent-tooling.md` and `.claude/agents/SHARED.md` as the settled probe record; update `code-intelligence.md` rather than maintaining two tool matrices. Treat Beads §5 as the only active convention and mark §§1–3 historical where they conflict.

## Out of slice

- `docs/architecture/current-substrate.md:116-118` independently repeats "43 scenario tests"; update it with the shared test inventory (owned by the broader architecture/current-substrate review).
- `simulation/src/souls/freight_station.rs:173-176` contains an active unprefixed `TestCtx` test outside `simulation/src/tests/**`; it is excluded from the 45-test count and needs a broader test-convention decision.
- The active checker run emitted duplicate H1 warnings for `README.md`/`docs/SUMMARY.md` and the two `Allocation` pages; no docs rule currently says duplicate H1s are errors.

## Open questions

- Should sentinel tests be real aliases/renamed corpus tests, or is the six-ID set itself stale and to be retired?
- Are process pages intentionally exempt from the metadata checker, or should `WIKI_SECTIONS` include `process` and enforce the documented contract?
- Should the cycle restore archive-only lanes automatically per ticket, or should Phase 0/2/6/7 be reduced to the live seven-agent roster?
- Is the current two-hook `bd prime` setup deliberate after the 2026-09-02 settings change, or is it an accidental reintroduction of the documented duplicate?
