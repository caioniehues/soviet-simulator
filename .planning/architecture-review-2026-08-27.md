# Architecture review — whole-repo deepening survey (sov-00c)

**Date:** 2026-08-27 · **Verified at:** HEAD `8531d3c` + dirty tree (sov-lpj in flight)
**Method:** 7 explorer lenses (sonnet cartographers) → dedupe → 7 adversarial verifiers (opus reviewer lane + `ledger-invariant-checker` for the economy cluster). Every verdict below required the verifier to re-read the cited source; two claims were settled by executed probes and one by a gold-standard mutation.
**Tally:** ~41 claim groups — 38 CONFIRMED (several strengthened), 1 PLAUSIBLE, 2 sub-claims REFUTED, 3 new findings surfaced by the verify wave itself.
**HTML report (visual, richer):** `/tmp/architecture-review-2026-08-27.html` (ephemeral).

Vocabulary per `compass:deep-modules`: module / interface / depth / seam / adapter / leverage / locality. Deletion test throughout.

---

## Top recommendation

**Give player commands a Verdict** (`verdict-refusal-seam`). Three independent lenses converged on it; the glossary ratifies `Verdict` and `Refusal` as binding vocabulary, and the code contains neither type (0 grep hits workspace-wide). `WorldCommand::apply` returns `()`; road placement computes real refusal reasons (`PointGenerateError::{TooSteep, OutsideOfMap}`, road.rs:65-68) then discards them at road.rs:85; the UI re-derives validity for a preview only (roadbuild.rs:284-294) — the exact shape SPEC-CONSTRUCTION-001 forbids. Deepening here unlocks the charter's whole Planner-interaction row (ghost, verdict, refusal, rescind) AND gives every tool test an interface to assert against. Prerequisite: decide what refusal means for road *splitting* (the invariant problem the comment at road.rs:72 is dodging).

**Bundle-first quick win:** `lua-validation` (below) — one probe-proven sim-killing defect, fixable inside the existing `validate()`.

---

## Confirmed candidates by theme

Strength: **S** = Strong, **W** = Worth exploring, **P** = Speculative. All CONFIRMED unless marked.

### A. The command seam
| id | finding | strength |
|---|---|---|
| verdict-refusal-seam | Ratified vocabulary, zero implementation. Silent no-ops (world_command.rs:238-243, :291-299), dead `AddTrain` arm (:302-306), refusal reasons computed then discarded (road.rs:85). Verifier correction: "only refuse uses" is true of the simulation crate; networking/ has 3 unrelated ones. | **S** |

### B. Economy custody (verified by ledger-invariant-checker; ledger verdict: CONSERVED)
| id | finding | strength |
|---|---|---|
| haul-out-of-market | Market fuses ledger + haul state machine: `advance_dispatches` ~338 lines/7 params (7th contingent on sov-lpj), `Market::remove` 6 args, dead-buyer logic duplicated (:318-369 vs :876-928). SPEC-LOGISTICS-001 (logistics.md:29-32) names the split; spec's Haul fields (attempts, age, recovery reason) mostly absent. | **S** |
| sold-shadow-ledger | `CompanyEnt::sold` pushed per trade (economy/mod.rs:89-93), popped only by factory drivers; 6/26 store companies never pop → serialized unbounded growth. Fires on every retail bread sale. Goods ARE conserved — the driver trip is cosmetic motion beside the authoritative Dispatch. Deletion candidate. | **S** |
| ecostats-counts-matches | Stats booked at match time (mod.rs:76), custody moves at :111; lost trucks stay counted. SPEC-TRADE-003 (trade.md:37-39) requires settlement paired with custody transition. Deepen: feed stats from custody events. | **S** |
| dispatcher-reservation-lifetime | 15 `free` sites vs 1 acquire; freed truck invisible until next tick. Severity corrected: latency not loss, self-healing — but two historical bugs (market.rs:392-397, :1008-1028) justify the consuming-token deepening ParkingManagement already models (parking.rs:10, 27-32). | **S** |
| matching-has-no-queue | Distance-greedy per-tick re-sort, no age/priority; correction: sell orders DO partial-fill (:579), missing piece is multi-seller fill per buyer; human orders now persist (extract_if :661-663). Closer to spec implementation (logistics.md:91-92) than refactor — scope deliberately. | **W** |
| money-gate-unchecked | Facts confirmed: unconditional debit (world_command.rs:224-225), only balance-vs-cost comparison in the codebase is a tooltip colour (hud.rs:55). Decision needed before Border/tranche work. | flag |

### C. Test surfaces (the interface is the test surface)
| id | finding | strength |
|---|---|---|
| determinism-test-no-teeth | **Proven by mutation**: injected divergence at tick 5000 → 11 "not equal" reports → `test result: ok`. Every mismatch branch halves `check_size` and continues; exhaustion breaks green (test_iso.rs:253, :276, :284, :292). `check_determinism` is round-trip only. Adjudicated: substrate.md:37 is CORRECT as written — status Absent stands. New finding: early-onset divergence crashes the diagnostic path via `SimulationOptions` noinit unwrap (resources.rs:80 via init.rs:232). Keep the bisection, make exhaustion fail. | **S** |
| testctx-observation | TestCtx has drivers, zero observation methods; scenarios reach past it (55× `read::<Market>`, 4× `world_mut_unchecked`, counts re-derived); cannot make an empty world (hardcoded START_COMMANDS, lib.rs:443); `build_company_at` duplicated ×3 (verifier found the third in hoarding.rs). | **S** |
| ui-intent-testable | native_app: 10,085 lines (not 3.6k), zero tests, no dev-deps. All 10 tool systems are `(sim, uiworld)` pure decision logic; bulldozer's ExternalTrading refusal (bulldozer.rs:67) is testable today. Standing rule should be narrowed: frames prove RENDERING; commands/refusals are assertable. | **S** |
| sim-to-frame-no-surface | headless/ has no engine dep (not a second adapter); engine+native_app tests = 26 (25 inherited earcut). Verified: instance building threads GfxContext through every call (map_mesh.rs:198-227) — "decide what to draw" is welded to "upload it". | **S** |

### D. Persistence
| id | finding | strength |
|---|---|---|
| silent-decode-default | 16 resources cross the save seam; on decode failure 15 silently revert to Default (init.rs:233-240); loader always returns Ok (lib.rs:439); save side panics, load side swallows. One guard exists (map, lib.rs:289) and only on the disk path. | **S** |
| version-gate | Warn-only, patch-blind (0.6.0 and 0.6.9 load silently into 0.6.1 — hand-evaluated in an extracted binary); at 1.x only MAJOR warns (1.0.0→1.6.1 silent — proven). Panic narrowed: save version `"0"` (major match, minor missing) or a truncated VERSION file. Decide mismatch policy before RC. | **S** |
| save-dir/is_equal | `world/` hardcoded cwd-relative; `is_equal` (pub, returns bool) writes per-resource dump pairs to cwd and unwraps — observed live during the mutation runs (litter deleted). | **W** |
| netcode-dup | worldsend/catchup re-implement save/replay for the wire; `cargo check --features multiplayer` clean. No 1.0 multiplayer commitment — don't invest, don't let it rot silently. | **W** |
| rerun-dead | rerun.rs is 49 lines of comment imported by `mod rerun;`. Delete. | **W** |

### E. Map model
| id | finding | strength |
|---|---|---|
| terrain-clamp | Full heightfield stack EXISTS and is live (gradient fn, MAX_SLOPE heightfinder ×3 sites, pylons via `true_height`); a 3-line clamp (terrain.rs:266-268, :82-84) is the only thing between the code and real relief. 1.0 terrain = generator change + consumer wiring, NOT a new subsystem. Probe: 36,242 land samples, all height 0. | **S** |
| auto-lot-seam | Strengthened: auto-lots have ZERO production consumers — every `build_house_near` caller is test code (freight_station.rs:179 is inside `#[cfg(test)]`). Migration to `build_house_at` already half-done. SPEC-ROADS-005 conflict named by the spec itself. | **S** |
| no-topology-revision | Strengthened: the notification channel exists but every subscriber renders pixels; zero subscribers in map_dynamic/. Deleted lane reads as passable (traversable.rs:57-65). SPEC-ROADS-003. | **S** |
| map-mutation-wide-bag | 32 public methods (counted); remove_building hand-maintains 6 side effects; electricity graph maintained twice with a coherency checker that exists because they can disagree; check_invariants is a release no-op. | **W** |
| road-capacity-absent | grep: 10 hits, all `Vec::with_capacity`. SPEC-ROADS-004 unimplemented. | **W** |
| freight-cargo-unitless | u32 counters with no ItemID; producers +1, train −100 `saturating_sub` (silent flooring); the two counters summed as one unit (:139). "Nothing teleports" unauditable at the station. Load-bearing for the 1.0 rail commitment. | **S** |

### F. Presentation
| id | finding | strength |
|---|---|---|
| palette-coverage | Authority EXISTS (`simulation::colors()`, 16 Lua fields, 9 consumers — procgen reads it too, correction). art-direction.md:52-53 denies it — doc wrong. Every non-road tint is `LinearColor::WHITE` literal (5 sites); lot colours are lawn green vs the doc's "never lawn green". Extend coverage, don't build a second palette. Two things named "palette" — name which. | **S** |
| seasons-absent | One grep hit: a surname. Charter stake has zero substrate; any "extend the season system" brief is wrong. Sun/season input lands in RenderParams. | **S** (design first) |
| sun-triplication | `sun_col` expression character-identical in 3 crates (3 real adapters at the framework State seam); 8-hour offset magic. Correction: shadowmap matrix is NOT duplicated (shared Camera method) — strike that half. Deepen: one time→sun module; natural season hook. | **W** |
| entity-render-full-scan | All vehicles/wagons/humans iterated per frame; humans twice; Outside filter is at draw not scan. Siblings in the same directory cull (trees/map_mesh/signals). 250k cost stays INFERRED — no measurement. | **S** |
| renderparams-mirror | Worse than claimed: WGSL is 16 fields vs Rust 19; layouts agree only because two padding conventions coincide. Every seasons/palette field addition lands here; mismatch is silent. | **W** |

### G. UI shell
| id | finding | strength |
|---|---|---|
| uiworld-any-bag | 37 registrations (counted; one cfg-gated → 36 in default builds), static-mut registries (same shape the sim crate already fixed via OnceLock), bag mutated mid-frame, 15 `sim.read().unwrap()` sites in game_loop with drop()-ordered locking. | **S** |
| multiplayer-self-chat | Strengthened: pressing T in a single-player build opens a working self-chat — the `SendMessage` handler is ungated (world_command.rs:364-367); message persists 5 game-minutes. Gate it like the 7 sibling sites, or remove; scope call is the user's. | **W** |
| economy-panel-prices | "$" axis, Expenses/Income, Money tab, whole-market `ext_value` price table — inherited capitalism UI contradicting pillars 3-4 on sight. Nuance: `ext_value` is border-adjacent, so a readout is legitimate; the panel just can't say which money surface it shows. Retheme vs remove = user decision (needs ratified trade spec). | **W** ⚠ |
| dead-code-cluster | 271 commented UI lines + rerun.rs + assets_gui editor block (references enum variants that DON'T EXIST — kept green by being commented) + leisure/road-vehicle prototypes with zero consumers ("loaded 1 leisure" logged every run). REFUTED sub-claim: MouseInfo exists (engine/src/input.rs:133); blocks actually rot from a mangled Tesselator generic, and reviving them would panic on an unregistered UiWorld resource. Delete-list, ~180+ lines. | **W** |
| dontclear-protocol | Narrowed: one writer, two readers, ordering in a comment; the entity half has NO writer — already dead. | **P** |

### H. Data layer
| id | finding | strength |
|---|---|---|
| lua-validation | **Probe-proven**: recipe amount 0 → validator passes, 20+ sim tests panic (divide-by-zero, money.rs:193); `get_lua().unwrap_or()` swallows TYPE errors at 6 sites (correct form exists 2 lines away); multiplier −3 → 4,294,967,293 standing request accepted as ordinary demand (proven end-to-end); ="not-a-number" → silently honest company (deletes the core loop). One fix: range checks in `validate()` + switch 6 sites to `get_lua_opt` + delete the `dbg!` at market.rs:1124 (fires per Market::default). Second zero source at market.rs:1118. | **S** |
| unknown-field-warn | Parser warns on unknown TYPE only; unknown field keys silently dropped (`max_power = "1kW"` consumed by nothing, no warning). Cheap: diff table keys vs consumed keys. | **S** |
| name-literal-goods | 11 literal sites (corrected from 13), two names — but blast radius WIDER: two missed native_app inspector sites mean a Lua rename silently breaks the UI too. | **W** |
| depth-measure | **Headline answered**: adding the charter's 15 resources + 12 recipe buildings is data-only, N=0 code edits (calculate_prices walks the recipe graph, no hardcoded table). What humans EAT is 4 bread edits. Medicine import-only has NO representation (only symmetric `optout_exttrade`) — design decision needed before resource 16. | fact |

### I. Sim structure
| id | finding | strength |
|---|---|---|
| exec-on-dead-entity | **Argument REFUTED, empirically**: ParCommandBuffer runs queued execs for entities killed in the SAME drain (probe: exec ran, entity dead). The `goods_company.rs:55` unwrap is safe only via one function's early return — and sov-lpj INTRODUCED the bare unwrap (replaced `.unwrap_or`). Warning filed on sov-lpj. Deepening: liveness-check the exec drain. | **W** ⚠ |
| desire-tournament | Hand-unrolled 3-way max-by-score, repeated blocks, 15-param fn, resources bound as ra..rf; Home desire is a constant 0.2. Adding desire #4 = 5 edit sites. | **W** |
| schedule-hardcoded | 6 ParCommandBuffer drains × 18 systems = 108 apply()/tick, two hand-kept lists; forgotten drain = commands silently never apply. | **W** |
| itinerary-writers | Verified by reading the module (ceiling lifted): plain value struct, no ownership token; economy writes vehicle itineraries at 4 sites. "Who decides where this goes" has no single answer. | **W** |
| resources-panic | read/write unwrap on missing TypeId; registration is a separate manual step. | **P** |
| freight-station-panic | **PLAUSIBLE** (only non-CONFIRMED candidate): demolish-external-station during a Loading train's 10s window → unwrap panic (freight_station.rs:109). Chain fully wired; scenario not built (registry file dirty with sov-lpj). "Never game over" pillar — settle with one scenario. | flag |

---

## Protected seams — deep modules; do not shallow them
- **MapSubscriber** chunk invalidation — 5 adapters incl. Map subscribing to itself
- **ImmediateDraw** (native_app/src/rendering/immediate.rs — path corrected) — 12 external consumers, commit-on-Drop
- **DebugObjs** — 9 uniform fn-pointer adapters
- **ParkingManagement** — the consuming-token model the Dispatcher should copy
- **SimDrop** — despawn cleanup concentrated in one place
- **Dispatcher's BFS core** — genuinely deep; only its reservation lifetime leaks
- **prototypes test_prototypes thread-local** — a real second adapter at the registry seam

## New findings from the verify wave itself
1. **Export debits before border check** (market.rs:702 vs :705, committed) — stock destroyed with no recorded sink. The import half was fixed at fdfabca; substrate.md:63 must be SPLIT, not marked fixed.
2. **Early-divergence diagnostic crash** — see determinism row.
3. **Mid-session test breakage**: test_iso.rs briefly didn't compile (Money::new_base) during the wave; "tests pass" reports from that window are suspect.

## Doc corrections queued (for doc-reality-auditor)
- substrate.md:60 (false since 7e4b82f), :63 (split import/export), :64 (humans persist now), :59 (still true — keep), :37 (KEEP as written per adjudication)
- art-direction.md:52-53 (palette authority exists), :21 vs colors.lua lawn-green lots
- Standing rule "UI proven by eyeballed frame" → narrow to rendering
- CLAUDE.md note: `simulation/src/multiplayer/` is chat, not netcode (briefs keep assuming otherwise)

## Decisions flagged for the project lead (not verdicts)
1. Economy panel: retheme (border-readout) vs remove — needs ratified trade spec
2. Multiplayer: target or not (gates chat panel + networking investment)
3. Leisure + road-vehicle prototypes: 1.0 scope or delete
4. Money gate: what SHOULD a negative balance mean (currently unobserved)
5. Medicine import-only representation (blocks resource 16)

## Next steps
Chosen candidate → `/grill` (constraints, dependencies, what sits behind the seam, surviving tests) → `/domain` for vocabulary/ADR → `/spec` for tracked work. Interface sketches on request via the design-it-twice pattern (parallel implementer sketches + opus adversarial pass).
