# Current roster and process audit

**Kind:** read-only audit
**Authority:** findings only; the designer dispositions
**Date:** 2026-08-28
**Sources:** 22 project agents, 4 global agents, development-cycle.md, gate-review.js,
gate-chain.formula.toml, 3 prior reviews, bd issues.jsonl (136 issues), git log,
RESUME.md, HANDOFF-2026-08-27, evidence-log.md, 15 codex .toml adapters, ROLE_ADAPTER.md,
5 skills, 2 policy docs

---

## 1. Roster table

26 agents total: 22 project (.claude/agents/), 4 global (~/.claude/agents/).

| # | Agent | Lines | Model | Effort | Memory | Phase | Lane | bd comments | bd assignee | git mentions |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | substrate-cartographer | 361 | opus | - | project | 0 | advisor | 10 | 0 | 5 |
| 2 | kornai-economist | 349 | claude-opus-4-6 | high | project | 0/4 | advisor | 4 | 0 | 5 |
| 3 | logistics-modeller | 357 | opus | - | project | 0/4 | advisor | 11 | 0 | 6 |
| 4 | utilities-modeller | 340 | opus | - | project | 0/4 | advisor | 7 | 0 | 1 |
| 5 | settlement-modeller | 334 | opus | - | project | 0/4 | advisor | 6 | 0 | 2 |
| 6 | soviet-authenticity | 347 | opus | - | project | 0/- | advisor | 1 | 0 | 4 |
| 7 | sim-implementer | 302 | opus | - | project | 2 | sim | 29 | 0 | 13 |
| 8 | ui-implementer | 285 | opus | - | project | 2 | ui | 2 | 0 | 6 |
| 9 | data-implementer | 312 | opus | - | project | 2 | data | 0 | 0 | 5 |
| 10 | engine-implementer | 261 | opus | - | project | 2 | engine | 9 | 0 | 5 |
| 11 | geom-implementer | 248 | opus | - | project | 2 | geom | 2 | 0 | 1 |
| 12 | widget-implementer | 255 | opus | - | project | 2 | widget | 0 | 0 | 1 |
| 13 | net-implementer | 244 | opus | - | project | 2 | net | 0 | 0 | 1 |
| 14 | common-implementer | 254 | sonnet | high | project | 2 | common | 0 | 0 | 2 |
| 15 | evidence-auditor | 330 | opus | - | project | 3 | gate | 15 | 0 | 7 |
| 16 | wiring-auditor | 329 | opus | - | project | 4 | gate | 18 | 1 | 11 |
| 17 | ledger-invariant-checker | 326 | opus | - | project | 4 | gate | 19 | 1 | 6 |
| 18 | debugger | 327 | opus | - | project | any | diag | 2 | 0 | 6 |
| 19 | doc-reality-auditor | 322 | opus | - | project | 6 | gate | 7 | 1 | 15 |
| 20 | perf-engineer | 318 | opus | - | project | 7 | gate | 1 | 0 | 3 |
| 21 | release-engineer | 302 | opus | - | project | 7 | gate | 6 | 0 | 4 |
| 22 | implementer (generic) | 201 | opus | - | user | 2 | fallback | 7 | 0 | 19 |
| 23 | reviewer (global) | 48 | opus | medium | project | 4 | gate | 18 | 0 | 35 |
| 24 | miner (global) | 28 | sonnet | high | user | any | extraction | 0 | 0 | 0 |
| 25 | researcher (global) | 39 | sonnet | high | user | any | research | 5 | 0 | 3 |
| 26 | team-lead (global) | 38 | opus | - | user | - | orchestration | 22 | 1 | 7 |

**bd comment counts** come from `bd comments list` on the live Dolt DB (comments are not in the JSONL export). JSONL-verifiable columns are assignee and git-log. The "any mention" column in JSONL (where the agent name appears in title/description/acceptance) confirms activity discussion but not dispatch.

**Zero dispatch evidence (no bd assignee AND no bd comment authorship):** common-implementer, widget-implementer, net-implementer, miner (4 agents). data-implementer has 0 assignee and 0 authored comments but is mentioned in 3 issue descriptions (as a lane reference, not as the worker). All five exist as files with no evidence of ever running a task.

**Model discrepancy:** kornai-economist pins `claude-opus-4-6` (a specific version); common-implementer pins `sonnet`. All others pin `opus`. The development-cycle.md says "uniform opus/high across all 16 in-repo agents" — this is wrong for at least 2 of 22.

**Effort frontmatter:** only kornai-economist and common-implementer set `effort:` explicitly. The rest inherit the session default. The dev-cycle doc claims "opus/high" but most agents have no effort field.

---

## 2. Duplication

### 2.1 Shared sections — quantified

| Section | Files carrying it | Body identity | Lines per copy |
|---|---|---|---|
| `## Engineering practice — all lanes` | 22/22 | 2 variants (md5 `3e8ca8` for 9 implementers, `2909c5` for 13 gates/advisors) | 119-121 |
| `## Subagent tooling — settled 2026-08-28` | 22/22 | 100% identical (md5 `799add`) | 18 |
| `## How to judge — all gates` | 13/22 | 100% identical among the 13 | 30 |
| Tooling preamble (before first ##) | 18/22 have 31-35 line versions | Structurally identical, differ in 1-2 role-specific sentences | 31-35 |

The two `Engineering practice` variants differ in one paragraph: the implementer variant says "Restraint fires once, before you add anything the brief does not name"; the gate/advisor variant says "Restraint fires once, when you propose or accept a mechanism the brief does not already name."

### 2.2 Aggregate duplication

| Measure | Value |
|---|---|
| Total lines across 22 project agents | 6,704 |
| Shared boilerplate (ep-all + tooling + judge-all) per agent | 137 (implementers) or 169 (gates/advisors) |
| Total shared lines (sum across all files) | 3,386 |
| Total unique lines (sum across all files) | 3,318 |
| **Duplication ratio** | **50.5%** |

Half of every agent file is identical boilerplate. A single shared include would save ~3,200 lines.

### 2.3 Duplicated content blocks beyond the three sections

Identified from the prior roster review (B14) and confirmed:

| Content block | Copies | Drift? |
|---|---|---|
| `cargo test` + static-mut race + `init.rs:85-86` | 9 | none |
| W&R install path + grammar counts | 6 | drifted (5 say "1,472 .ini files"; real count is 1,472 total files, 488 .ini) |
| `optout_exttrade` 1-of-21 | 6 | none |
| TestCtx::tick() limits | 5 | **drifted** — 3 deny it proves determinism, 1 uses it AS determinism proof |
| Truck-vs-train substrate | 3 | line citation `goods_company.rs:129` vs real `:132` |

### 2.4 Unique content per agent (what this agent knows that nothing else records)

| Agent | Unique knowledge (3-5 lines) |
|---|---|
| substrate-cartographer | Three-source methodology (Rust, Lua, W&R reference). Returns cited fact-sheets. Memory persists fact-sheets across iterations. |
| kornai-economist | Four Kornai pillars (shortage normal, queue clearing, soft budget constraint, dishonest enterprise). Known live violations list. Five economy-mechanic questions. Rules against `request_multiplier` typo deleting core loop. |
| logistics-modeller | BPR volume-delay and Gawron blending models. Truck vs train substrate (parking/collider). Three open dispatch bugs (sov-jcl/xyx/abs). Five movement-mechanic questions. |
| utilities-modeller | Electricity is union-find over road adjacency (must be replaced by laid wire). Brownout-before-blackout rule. Weather is small and blocking. |
| settlement-modeller | Households are shared-pantry units. Needs clear by waiting/substituting/going-without. Performance is a design constraint (cited numbers stale). |
| soviet-authenticity | 1950s-60s fantasy. Standing playtest verdict: "looks like something done by a child." Fabricated art-direction quote (B13). |
| sim-implementer | Determinism harness details. Registration points. Five verified traps (trucks/trains, CompanyKind::Factory, Lua data, auto-lots, Market::remove). BTreeMap vs FastMap rule. |
| ui-implementer | Sim test harness cannot drive UI. Visual proof owed. Planner fantasy. Legibility-is-gameplay rule. |
| data-implementer | Lua data layer as smallest-surface/largest-failure. W&R reference install path. Prototype validation gaps. Thread-local market test fixtures. |
| engine-implementer | wgpu pipelines, render passes, drawables, shaders. Traps that cost time (unstated — section header exists but read was truncated). |
| geom-implementer | Vectors, matrices, splines. Determinism-sensitive float paths. Skeleton OOM bug context. |
| widget-implementer | yakui/egui boundary. Standing "child" quality bar. Stack trap (yakui layer). |
| net-implementer | Client/server connections, authentication, packet framing, catch-up replication. "Be careful" (fragile crate). |
| common-implementer | RNG draw order sensitivity. Timestep determinism. saveload bincode traps (FIXINT vs VARINT, positional schema). FxHasher version instability. `headless` is a server, not a benchmark. |
| evidence-auditor | Mutation testing methodology. Known exemplar: sentinel test ran 0 tests, exited 0. |
| wiring-auditor | Reachability-first gate. Proven: reproduced 2 opus findings + 1 new one on first run. Four-check sequence. |
| ledger-invariant-checker | Quantity/money conservation across seams. Method: concrete failing sequence or "none". |
| debugger | Three founding incidents (unstaffed factory, invisible freight station, dispatch wedge family). 3-strike rule. Mutation-based confirmation. |
| doc-reality-auditor | Sweep method against code, bd, and git. Phase-6 timing. |
| perf-engineer | Bench gates do not exist. 250k benchmark lane cancelled 2026-08-27. No bench runner. |
| release-engineer | Build not reproducible (egui tracks HEAD, yakui on fork's dev branch). Licence obligations. |
| implementer (generic) | Brief-following. Known stale briefs. Re-derive performance numbers. Reachability check. |
| reviewer (global) | Lean 48-line definition. Method: one pass, no subagents. Verdict schema (approve/approve-with-fixes/send-back). |
| team-lead (global) | Orchestration persona. initialPrompt with orientation sequence. Workers report to `main`. |
| miner (global) | Extraction worker. Refuse judgment briefs. Sonnet tier. |
| researcher (global) | Evidence hierarchy: ctx7 docs > upstream source > live behavior. Sonnet tier. |

---

## 3. Evidence backing

### Phase justifications

| Phase | Cited incident | Verifiable? |
|---|---|---|
| 0 GROUND | Commented-out truck registration; false "copy train pattern" brief; `optout_exttrade` 1-of-21 unread | **Measured.** All three appear in bd comments, git history, and agent-memory. The optout incident traced to `sov-dispatch-wedge-ab4` comments. |
| 0 GROUND | Inherited false claims (~20 dispatches poisoned) | **Asserted.** The "~20 dispatches" figure appears only in development-cycle.md and the swarmforge review. No bd/git artifact counts them. |
| 1 PLAN | Shared `scenarios/mod.rs` clobber near-miss | **Asserted.** "This nearly caused a clobber; only dispatch timing prevented it." No commit evidence; no bd id cited. |
| 2 BUILD | `sov-2c4`/`sov-7pg` truck-parking wedge from corner cut | **Measured.** Commit `e27a068`, bd issues exist, 106-line fix cited. |
| 3 PROVE | Sentinel test ran 0 tests, exited 0 | **Measured.** Reproduced in roster review B15: `cargo test -p simulation sentinel` → `running 0 tests`, exit 0. `sov-journey-sentinels-rxa` is OPEN. |
| 3 PROVE | `test_world_survives_serde` had no assert | **Measured.** `sov-myg` closed with red/green proof. |
| 4 GATE (ordering) | wiring-auditor reproduced 2 + found 1 missed by opus | **Measured.** Stated in development-cycle.md:211-214. The swarmforge review cites the same. |
| 4 GATE (ledger) | ledger found 5 vs reviewer's 2 on same seam | **Measured.** Token counts cited: 100.8k vs 112k. Appears in swarmforge review with bd trail. |
| 5 DISPOSITION | Agent rewrote 359 lines between gate and disposition | **Asserted.** No commit or bd id cited for the specific incident. |
| 6 WRAP | `bevy.md` reference to nonexistent file; 4 agents targeting deleted paths; open ticket for shipped work; stale RESUME counts | **Measured.** Each appears in doc-audit-2026-08-26 with specific file:line. |
| 7 SHIP | egui tracks upstream HEAD with no rev pin | **Measured.** Stated in release-engineer definition; verifiable in Cargo.toml. |

### Evidence-log cross-reference

The evidence log (`~/.claude/reference/evidence-log.md`) records 22 incidents across 6 dates (2026-08-20 through 2026-08-28). Of the 17 incidents cited in development-cycle.md, only 1 has a bd id (`sov-test-race-initfuncs-qt6`). The rest are narrative descriptions. All evidence-log entries are dated and verifiable through git or bd.

### Gate justification summary

- **Measured incidents (in development-cycle.md):** 9
- **Asserted without verifiable artifact:** 3 (the "~20 dispatches", the mod.rs clobber, the 359-line rewrite)

---

## 4. Cost

All token figures found in the source corpus:

| Source | Figure | Context |
|---|---|---|
| development-cycle.md:306-307 | 65-85k | miner/extraction |
| development-cycle.md:307 | 110-155k | sonnet implementer |
| development-cycle.md:308 | 105-113k | opus reviewer |
| development-cycle.md:309 | 15-30k | narrow sonnet auditor |
| development-cycle.md:314-316 | ~675k per iteration | Ground ~80k, Build ~360k, Prove ~40k, Gate ~165k, Wrap ~30k |
| roster-review-2026-08-27:8 | 170k | doc-reality-auditor (bodies, opus/high) |
| roster-review-2026-08-27:8 | 115k | claude-code-guide (frontmatter, opus) |
| roster-review-2026-08-27:8 | 40k | wiring-auditor (runtime probe, opus) |
| swarmforge-review:20 | 100.8k | ledger-invariant-checker (one story) |
| swarmforge-review:20 | 112k | opus reviewer (one story, same seam) |
| swarmforge-review:38 | ~2,900 lines | process documentation in one session |
| swarmforge-review:38 | ~400 lines | production logic in one session |

| progress.md (data-driven-buildings) | 1.39M | Gate 3+5 alone: 23 agents, 45 min |

### Contradictions in cost data

1. development-cycle.md says "sonnet implementer: 110-155k" but all implementers are now opus. The figures were measured at sonnet tier and have not been re-measured at opus.
2. development-cycle.md says "narrow sonnet auditor: 15-30k" but all auditors are now opus. Same staleness.
3. The ~675k per-iteration estimate was built from sonnet-tier measurements. At opus tier it is likely 2-3x higher, but no opus-tier iteration total has been recorded.
4. The roster review itself cost 170k+115k+40k = 325k tokens for a read-only audit — half a full iteration's budget — suggesting the 675k estimate is outdated.
5. The data-driven-buildings gate alone consumed 1.39M tokens (23 agents) — more than double the entire 675k iteration estimate. Either the estimate predates the multi-reviewer gate design or excludes gate-chain overhead.

---

## 5. Usage reality

### Full 8-phase cycle completions

**One story has evidence of the full cycle: `sov-dispatch-wedge-ab4`.**

It went through: Phase 0 (kornai-economist ruling), Phase 2 (implementer, 3 rounds), Phase 3 (implicit — tests exist), Phase 4 (ledger-invariant-checker 3 passes, reviewer 3 passes, codex cross-vendor review), Phase 5 (lead disposition of cross-vendor send-back), close. 12 bd comments, 4 send-backs, 8 gate findings.

**Evidence this is the ONLY full-cycle story:**
- Only 1 molecule exists in bd: `sov-mol-jdl` (gate-chain), status: closed.
- `sov-bo3` (the OOM crash) shows a 2-phase subset: implementer → wiring-auditor → evidence-auditor → reviewer, but no Phase 0 grounding and no domain gate. 6 bd comments.
- `sov-myg` (determinism guard) shows implementer + lead verification, no gates.
- All other closed issues (69 total) show 0-2 phases of evidence in their comments.

### Gate runs that caught real bugs

| Gate agent | Story | What it caught |
|---|---|---|
| ledger-invariant-checker | sov-dispatch-wedge-ab4 | Market::remove destroys goods when buyer removed mid-dispatch (BROKEN, send-back) |
| reviewer | sov-dispatch-wedge-ab4 | 2 blockers on 2nd pass (send-back); approve-with-fixes on 3rd with truck leak finding |
| codex-reviewer | sov-dispatch-wedge-ab4 | Cargo conservation on dead-seller dispatches (send-back) |
| wiring-auditor | sov-bo3 | Memory guard is unwired (nothing runs --ignored); reproduced independently |
| evidence-auditor | sov-bo3 | Refusal path (any_corrupt) had zero coverage (stubbing it left all 24 tests green) |
| reviewer | sov-1ae | send-back on benchmark chunk (on WIP branch, never reached main) |
| ledger-invariant-checker | sov-abs | Test defect: retail scenario assertion leak |

**Gates caught real bugs in at least 3 stories.** The `sov-dispatch-wedge-ab4` chain (implementer → 3 rounds of gates → 4 send-backs) is the strongest evidence the process works.

### Non-roster wave names

~55 unique comment authors exist in bd, vs 26 roster agents. Most dispatches used ad-hoc wave names (wave3-final-reviewer, wave3-evidence-roadmap-writer, wave2_utilities_writer, codex-lead, lens-perimeter, etc.) rather than roster agent names. This indicates the roster was not the primary dispatch surface for most of the project's history.

### Phases most commonly skipped

- **Phase 0 (GROUND):** Only sov-dispatch-wedge-ab4 shows a domain-advisor ruling before implementation. Most issues go straight to Phase 2.
- **Phase 3 (PROVE):** evidence-auditor has 15 bd comments across the whole history — concentrated on sov-bo3. Most stories skip mutation testing.
- **Phase 6 (WRAP):** doc-reality-auditor ran at least twice (the two dated audits). No evidence of routine Phase-6 runs.
- **Phase 7 (SHIP):** perf-engineer has 1 bd comment total; release-engineer has 6 (all from the dependency-policy work). No release has been cut.

---

## 6. Open findings from prior reviews

Consolidated from: agent-roster-review-2026-08-27, review-2026-08-26-vs-swarmforge, doc-audit-2026-08-26.

| # | Finding | Source | Status | Check |
|---|---|---|---|---|
| 1 | `development-cycle.md` says "16 agents" but the roster table lists 22 | roster-review B1 + count | **OPEN** | `ls .claude/agents/*.md | wc -l` = 22; heading still says "Sixteen" |
| 2 | `development-cycle.md:59` says "uniform opus/high across all 16" — common-implementer is sonnet, kornai-economist is claude-opus-4-6 | roster-review B1 | **OPEN** | `grep '^model:' .claude/agents/common-implementer.md` = sonnet |
| 3 | B5: lane collision on market.rs — three gates can each claim it | roster-review B5 | **FIXED** | development-cycle.md:197-205 now has an explicit split table |
| 4 | B6: engine/geom/net/common had no implementer lane | roster-review B6 | **FIXED** | 5 new implementers added (HANDOFF-2026-08-27) |
| 5 | B8: debugger and evidence-auditor can collide on mutation | roster-review B8 | **OPEN** | Neither definition mentions the other |
| 6 | B9: three implementers do not know debugger exists | roster-review B9 | **FIXED** | `grep -l debugger .claude/agents/{sim,ui,data}-implementer.md` = all three match (2 hits each) |
| 7 | B10: soviet-authenticity refuses to gate but is listed as gating | roster-review B10 | **OPEN** | `development-cycle.md:41` says "0 / —" but the domain-advisors table still lists it with Phase-4 sign-off |
| 8 | B13: soviet-authenticity has a fabricated art-direction quotation | roster-review B13 | **OPEN** | `grep "Gritty, weathered" docs/` = 0 hits |
| 9 | B14: TestCtx::tick() drift — logistics-modeller uses it as determinism proof while 3 others deny it proves determinism | roster-review B14 | **OPEN** | Not reconciled |
| 10 | B15: sentinel test still runs 0 tests, sov-journey-sentinels-rxa still OPEN | roster-review B15 | **OPEN** | `cargo test -p simulation sentinel` → 0 tests |
| 11 | F1: no agent uses `skills:` frontmatter for discipline preload | roster-review F1 | **OPEN** | Only `implementer.md` has `skills:` (compass playbooks) |
| 12 | F6: kornai-economist has invalid `color: magenta` | roster-review F6 | **OPEN** | Still present in frontmatter; should be purple |
| 13 | F1 swarmforge: advisor tier table vs frontmatter | swarmforge F1a | **PARTIALLY FIXED** | Dev-cycle table now says opus; most frontmatter says opus; but common-implementer is sonnet and kornai-economist is claude-opus-4-6 |
| 14 | F1 swarmforge: gate-chain formula missing ledger step | swarmforge F1b | **FIXED** | gate-chain.formula.toml now has a `ledger` step |
| 15 | F1 swarmforge: Executed-By trailer convention dead | swarmforge F1c | **FIXED** | CLAUDE.md §Adopted conventions now says "DELETED (2026-08-27, the hook is inert)" |
| 16 | F2 swarmforge: micro-layer comments only under wave pressure | swarmforge F2 | **OPEN** | 10 open issues still have 0 comments |
| 17 | F3 swarmforge: completion signalling is prompt-discipline, not mechanism | swarmforge F3 | **OPEN** | No mechanism added |
| 18 | F4 swarmforge: no documented light path | swarmforge F4 | **OPEN** | No light path documented |
| 19 | F5 swarmforge: no named dispute procedure | swarmforge F5 | **OPEN** | No dispute procedure documented |
| 20 | F6 swarmforge: infra can starve the fleet silently | swarmforge F6 | **PARTIALLY FIXED** | LSP issue diagnosed and documented; no general infra-audit mechanism |
| 21 | Doc-audit #1: charter-1.0.md still references `br` | doc-audit | **FIXED** | `grep -c 'br ' docs/plan/charter-1.0.md` = 0 |
| 22 | Doc-audit #4: roster tier table wrong | doc-audit | **FIXED** | Table now says opus |
| 23 | Doc-audit #5: gate-chain formula missing ledger | doc-audit | **FIXED** | Ledger step added |
| 24 | Doc-audit #6: Executed-By dead | doc-audit | **FIXED** | Convention marked deleted |
| 25 | Doc-audit #7: swarmforge review is unreachable | doc-audit | **FIXED** | development-cycle.md:219 now links it as "See also:" with relative path |
| 26 | Doc-audit #8: regression inventory stale at 26 vs 42/45 tests | doc-audit | **FIXED** | RESUME.md says regeneration done, now 45 |
| 27 | 0.1: LSP absent in all subagents; 22 agents carry LSP instructions they cannot follow | roster-review 0.1 | **OPEN** | `ToolSearch("select:LSP")` fails in any subagent; all 22 agent files reference LSP |
| 28 | 0.2: `memory: project` re-enables Write/Edit on 12 "read-only" agents | roster-review 0.2 | **OPEN** | Gate/advisor agents promise read-only but `memory: project` auto-grants Write+Edit |
| 29 | autoMode.environment false facts (trusted repo path wrong, "no remotes" is false) | roster-review | **OPEN** | `~/.claude/settings.json` still says "no remotes configured" |
| 30 | Doc-audit #9: ADR mechanism never used post-fork | doc-audit | **OPEN** | `docs/decisions/` has only README.md |

**Summary:** 17 open, 11 fixed, 2 partially fixed (30 total from 3 prior reviews + this audit's additions).

---

## 7. Structural gaps

Each gap is stated as a fact with the evidence it is missing, not a recommendation.

| # | Gap | Evidence it is missing |
|---|---|---|
| 1 | **No light path.** Every change, from a one-line typo to a multi-story epic, faces the same 8-phase cycle. | development-cycle.md has no conditional path; swarmforge F4 named this; still open. The lead improvises shortcuts with no documentation. |
| 2 | **No dispute procedure.** When two gates disagree, or a worker disputes a verdict, the resolution is "the lead decides." | swarmforge F5; no written path exists. The `sov-dispatch-wedge-ab4` codex send-back was adjudicated by lead with no documented rule. |
| 3 | **No completion mechanism.** Workers report via SendMessage or final message. Nothing structural confirms arrival. | swarmforge F3; evidence-log:79 records "thin idle summaries" 4× in one wave. Dead Executed-By trailer was the only attempt; it never worked. |
| 4 | **No retrospective.** No phase or process step examines what went wrong in the PROCESS after an iteration. | No "retro" or "retrospective" appears in any process doc. The swarmforge review is the closest thing and it was an ad-hoc audit, not a recurring phase. |
| 5 | **No sprint/iteration boundary.** The cycle describes phases but no concept of grouping stories into a bounded iteration. | RESUME.md tracks stories individually. `bd` has no sprint/milestone primitive. The ~675k budget is "per iteration" but nothing defines what an iteration contains. |
| 6 | **No document of what the human does.** The human's role (choosing stories, adjudicating disputes, accepting visual proof, approving commits/pushes) is implicit. | The user appears as a constraint ("explicit authority from the user") but their decision points are not enumerated. |
| 7 | **Single-threaded lead.** Every dispatch, every gate disposition, every synthesis goes through one entity (main/team-lead). | team-lead.md and development-cycle.md Phase 1/5 say "lead only". No deputy or delegation hierarchy exists. |
| 8 | **Memory strategy is fragmented.** 20 agents use `memory: project`, 2 use `memory: user`. Agent-memory directories exist but are not audited for staleness. | No memory rotation, no memory audit, no memory budget. The roster review (B14) found drifted content in memory-like sections. |
| 9 | **No cost tracking.** Token costs are recorded as one-time measurements in development-cycle.md. No per-session or per-story cost is logged. | The only cost data is the static table in development-cycle.md:304-316, measured at sonnet tier, now stale. |
| 10 | **Gate-review.js is gitignored.** `.gitignore:2` is `.claude/*`. The workflow exists only in the main checkout, not in worktrees or clones. | roster-review finding in "How to judge — all gates" section; the two-axes rule references gate-review.js but agents in worktrees cannot read it. |
| 11 | **Codex mirror has no reviewer or gate entry point.** 15 of 22 agents have .toml adapters; no `reviewer.toml`, no `debugger.toml`, no gate workflow. | development-cycle.md:65-68 confirms this. The cross-vendor gate is "planned, not built." |
| 12 | **Four skills are vestigial.** `debug-issue` (28 lines), `explore-codebase` (29), `refactor-safely` (29), `review-changes` (30) are thin wrappers that add nothing beyond what the agent definitions already contain. | Each is under 30 lines and references no unique content. `dev-cycle` (47 lines) is the only skill that adds value (the front-door routing). |

---

## 8. Contradictions between documents

| # | Contradiction | Documents |
|---|---|---|
| 1 | **Agent count: "Sixteen" vs 22.** development-cycle.md heading says "Sixteen agents in `.claude/agents/`"; the repo has 22 files; the roster table in the same document lists 22 rows (20 project + reviewer + debugger). | development-cycle.md:32 vs `ls .claude/agents/*.md` |
| 2 | **Uniform opus claim vs reality.** development-cycle.md:59 says "uniform opus/high across all 16 in-repo agents." common-implementer.md:4 says `model: sonnet`; kornai-economist.md:4 says `model: claude-opus-4-6`. | development-cycle.md:59 vs agent frontmatter |
| 3 | **Effort claim.** development-cycle.md:59 says "opus/high" but only 2 of 22 agents set `effort:` in frontmatter. The rest inherit session default (medium). | development-cycle.md vs agent frontmatter |
| 4 | **evidence-auditor tier.** development-cycle.md:172 says "evidence-auditor (sonnet)"; the agent file says `model: opus`. The development-cycle.md:59 uniform-opus sentence also contradicts the sonnet claim. The document contradicts itself. | development-cycle.md:172 vs :59 vs evidence-auditor.md:4 |
| 5 | **doc-reality-auditor tier.** development-cycle.md:237 says "doc-reality-auditor (sonnet)"; the agent file says `model: opus`. Same self-contradiction as #4. | development-cycle.md:237 vs doc-reality-auditor.md:4 |
| 6 | **perf-engineer bench gates.** development-cycle.md:57 says "The five bench gates at 250k"; the same doc at :265 says "none exist yet, and the charter names none of them" and "the lane is dropped." | development-cycle.md:57 vs :265 |
| 7 | **soviet-authenticity gating.** development-cycle.md:41 says phase "0 / —" (no gate). development-cycle.md:285 says advisors "hold a Phase-4 sign-off". soviet-authenticity.md says "You never gate a merge." | development-cycle.md:41 vs :285 vs soviet-authenticity.md |
| 8 | **TestCtx::tick() as determinism proof.** logistics-modeller.md:131 says "The sim bincode-round-trips and hash-compares every tick" as a criterion. debugger.md:104-106, evidence-auditor.md:69-70, sim-implementer.md:58-61 say it does NOT prove determinism. | logistics-modeller vs 3 other agents |
| 9 | **W&R .ini file count.** 5 agents say "1,472 `.ini` files". substrate-cartographer says "1,472 total files." Only the cartographer is right (1,472 total, 488 .ini). | 5 agent bodies vs substrate-cartographer |
| 10 | **Cost figures measured at wrong tier.** development-cycle.md:307 says "sonnet implementer: 110-155k" but all implementers are opus. The per-iteration estimate of ~675k is built from sonnet measurements. | development-cycle.md:306-316 vs current model assignments |
| 11 | **`implementer` (generic) status.** development-cycle.md:158 says "The global `implementer` is no longer the fallback for any crate in this repo." But `implementer.md` still exists with 201 lines and has 7 bd comments + 19 git mentions — more usage than most specialists. | development-cycle.md:158 vs actual usage |

---

## Summary statistics

| Metric | Value |
|---|---|
| Total agent definitions | 26 (22 project + 4 global) |
| Total agent lines | 6,704 (project) + 153 (global) = 6,857 |
| Duplicated lines | ~3,386 (50.5% of project total) |
| Unique lines per agent (average) | ~151 (project), ~39 (global) |
| Zero-dispatch agents | 5 (data-implementer, widget-implementer, net-implementer, common-implementer, miner) — 4 certain zeros + 1 borderline |
| Stories with full cycle evidence | 1 (sov-dispatch-wedge-ab4) |
| Gate send-backs that caught real bugs | 7 across 3 stories |
| Open findings from prior reviews | 17 open + 2 partially fixed (of 30 total) |
| Structural gaps | 12 |
| Document contradictions | 11 |
| Codex adapters | 15 .toml files + ROLE_ADAPTER.md; no reviewer or debugger adapter |
| Skills | 5 (1 useful: dev-cycle; 4 vestigial at <30 lines each) |
| Process docs | development-cycle.md (374 lines), mutation-policy.md (430), dependency-policy.md (314) = 1,118 lines |
| Gate workflow | gate-review.js (~250 lines), gitignored |
| Gate formula | gate-chain.formula.toml (39 lines, 4 steps) |
