# Conversation mining 2026-08-28 — synthesis and reconciliation

**Kind:** explanation
**Authority:** explanatory
**Status:** draft
**Owner:** project lead
**Verified-at:** `4e9e930b2a73` (committed tree; the working copy carried an uncommitted `sov-ahw` diff to `market.rs` that this document does not cite)
**Last verified:** 2026-08-28

## 0. What this is

Two GPT documents describe this project's design at length:

- the **export** (`docs/archive/raw-sessions/gpt-vision-export-2026-08-28.md`, 1,210 lines) — six deep-dive passes and a closing thesis, with section 1 explicitly *reconstructed* rather than verbatim;
- the **bible** (`docs/archive/raw-sessions/gpt-master-bible-2026-08-28.md`, 3,194 lines) — GPT's later consolidation of the same thread into 26 sections.

Ten Opus lanes mined the export in parallel (lanes A, B1, B2, C1, C2, D, E, F, G, H; reports in this directory; session log at `docs/archive/raw-sessions/claude-mining-session-2026-08-28.txt`). The bible arrived after the lanes finished; no lane saw it. This document reconciles all three sources — export, bible, lanes — against the committed code and the specification corpus, adds the lead's own verification, and records where every idea was consolidated.

The user's decisions that shaped this pass:

1. The charter does not filter ideas. Post-1.0 material is recorded, labelled, not rejected.
2. Export and bible are peers; this synthesis reconciles them.
3. The bible's un-audited sections (§8, §14, §20, §21, §23, §24) were audited by the lead in-session, not by a new lane.
4. Where the bible and a lane's verified finding conflict, **both are recorded and the decision is left open** (§6 below).
5. The bible is archived verbatim and a curated, corrected form becomes the single vision document: `docs/vision/design-bible.md`.

Evidence labels used below: **CONFIRMED** (external source or code proves it), **PLAUSIBLE** (consistent, unproven), **UNSUPPORTED** (no source), **WRONG** (contradicted), **ALREADY-EXISTS** (in code or a ratified/draft spec), **ABSENT** (no substrate), **CONTRADICTED** (code does the opposite).

## 1. Verdict in one page

1. **The design thesis is sound and largely historically grounded.** The control-loop model (plan → adaptation → physical flows → imperfect reports → next plan), the reliability→buffer spiral, the ratchet, storming, priority-as-relocation-of-scarcity, social reproduction, time as a resource, housing as labour infrastructure, blat as a zero-sum allocation topology, and network-specific inertia all survived validation (lanes A, B1, B2, D; bible §5, §7, §10, §11). Sixteen of twenty CIA-derived claims were matched to specific declassified documents by ID (B2).

2. **Almost none of it exists in code.** Lane E audited 136 claims: 16 EXISTS, 26 PARTIAL, 89 ABSENT, 5 CONTRADICTED. The one thing that *does* exist and matters — the dishonest enterprise — is wired end-to-end (`request_multiplier` → `recipe_init` → `set_requested`, `goods_company.rs:22-26`) but invisible to the Planner: no UI reads `Market::requested()`.

3. **The export presented targets as conclusions; the bible mostly fixed that.** The bible dropped every fabricated number the lanes flagged (the 72/141/24/18 freight table, the 61/67/82/91 credibility table), labels the phase order a "target", the citizen split "conceptual layers", the code standards "not automatically binding", and it uses the charter's single-rouble language. It still overstates in four places (§2.3 below).

4. **Five code paths contradict the pillars and block building on them:** export sell-side teleport (`market.rs:774`), domestic money debited for buildings and roads (`world_command.rs:225`) and for workers per minute (`economy/mod.rs:54`), auto-generated roadside lots (`map/map.rs:682-720`), and a static rather than adaptive `request_multiplier`. Verified by the lead (§4).

5. **The architecture proposals form a dependency chain, and the chain's first links are missing:** keyed randomness → typed contexts → phase labels → parallelism; save-migration seam before any structural change; `PlannerSnapshot` independent but with the largest migration surface (~40 UI call sites). The `ParCommandBuffer::exec_ent` closure channel (`FnOnce(&mut Simulation)`) is the single biggest obstacle to typed contexts and to parallelism; neither GPT document mentions it (C2 §4.3–4.5).

6. **Both GPT documents are silent on things the lanes found central:** the `networking/` lockstep crate; the storage-capacity floor on hoarding that already exists in `recipe_should_produce`; propiska, alcohol, kommunalka tiers, pensioner queue labour; ministries as inflating intermediaries; the soft-budget-constraint's physical analogue; assortment plans; OTK quality attestation; junction deadlock; rail signalling as the real capacity limit; the tutorial problem; scenario-vs-mode; `libm` for float determinism; FxHasher non-portability; unpinned `egui`/`yakui` git deps.

7. **Consolidation happened.** §8 maps every idea to where it now lives: one vision document, one game-modes document, three proposals, one research note, eight glossary terms, one bd-draft file, and four stale-documentation fixes.

## 2. Export versus bible

### 2.1 Same thread, different quality

| Property | Export | Bible |
|---|---|---|
| Length | 1,210 lines | 3,194 lines |
| Provenance | Six passes + reconstructed context | Consolidated synthesis, authority note in §1 |
| Fabricated numbers | Two tables (lines 219–224, 261–265), "145 t" | None; metrics listed without values; "135 t" as illustration |
| Phase order | "Deterministic phase order" as a *conclusion* (line 41), ten phases | "Target phase architecture" (§13.11), eleven phases (adds REPORTING) |
| Citizen split | "Split persistent CitizenRecord from active CitizenBody" as done | Five "conceptual layers" L0–L4 (§7.1) |
| Money | "Border roubles" (plural), dual-circuit echoes | "The rouble is border foreign currency only" (law 5) — matches charter:33 |
| CIA sourcing | "CIA-only pass" with zero citations | 15 URLs in §25.2, none tied to a specific claim |
| Code standards | Absent | §14 (fifteen standards), §15 (crate decisions) |
| Labour institutions | Absent | §8 in full |
| Implementation order | Absent | §20 (Phases 0–10) |
| Missing specs | Absent | §21 |
| Mechanic template | Absent | §23 |

### 2.2 What the bible adds that no lane audited

§5.8 priority inflation; §5.13 tolkachi (Lane A's M-01, now present); §6.5 finite dock rates and §6.7 deadhead metrics (Lane D's "loading is the real bottleneck", now present); §6.8 handling classes; §6.13 construction phases; §8 entire (differentiated labour, tenure ramp, labour hoarding, work collectives, trade unions, safety inspection, local Soviets as a sensor network, representation error, institutional confidence, "who decides" questions for alternative rulesets); §13.15 idempotent authoritative transitions; §14 standards; §16.6 determinism bisection by phase digest; §17 validation strategy; §20 sequence; §21 missing specs; §23 taxonomy; §24 sketches. The lead's audit of these is in §3.5, §3.10, §3.11, §3.14.

### 2.3 Where the bible still overstates

| Bible claim | Reality | Evidence |
|---|---|---|
| §9.6 "the binding spec already uses EWMA load, BPR cost, and Gawron damping" | The spec *prescribes* it and is **draft**; nothing is implemented | `traffic.md` header `Status: draft`; `pathfinding.rs:234-239` cost is `length/speed_limit + noise` (D-08) |
| §10.8 "1.0 binding model `G + D = V + C + L`" | `SPEC-ELECTRICITY-002` says exactly this and is **draft**; code is a binary blackout over a road-adjacency union-find | `electricity.md:18-19`; `electricity_cache.rs:244-279`, `electricity.rs:43-93` |
| §6.1 "the 1.0 catalogue remains exactly fifteen domestic resources" | Charter target; `base_mod/items.lua` has 21 labelled items with no mass/volume/class metadata | `grep -c 'label' base_mod/items.lua` → 21 (E-003) |
| §16.1 "the headless whole-game benchmark is the final gate" | No benchmark exists; `sov-1ae` (250k benchmark contract) was cancelled 2026-08-27 | `.beads/issues.jsonl`; F-CONTRADICTS-01 |
| §25.2 fifteen CIA URLs | All 403 from this machine (same as Lane B2). Only two IDs (`CIA-RDP08S01350R000602150001-3` consumer frustrations; `CIA-RDP87T00787R000200200003-0` alcohol) overlap B2's 48 verified document IDs. The other thirteen, including the "Plant 393" report, are **unverified** | Lead fetched three: HTTP 403 |
| All 22 specifications | Every one is `Status: draft`. The bible's "binding" wording should read "draft specification" | `head -6 docs/reference/specifications/*.md` |

None of these are errors of substance — the bible reads the specs correctly — but a reader who trusts the word "binding" would believe mechanisms are ratified that are not.

### 2.4 What the bible gets right that the export got wrong

- Numbers: none invented.
- Authority: §1 note, §14 "not automatically binding", §24 "illustrative shapes, not ratified Rust definitions".
- Money: single rouble, border only.
- Research caution (§3.2) matches Lane B2 §4: emigre bias, era flattening, Moscow bias, "the game must permit good planning to work".
- §11.5 gives the *positive* reliability spiral, which the export's spiral (G-11 "death spiral with no floor") lacked.

## 3. Reconciliation ledger by theme

Each subsection: what the sources claim; what the lanes found; what the lead verified; verdict; where it landed.

### 3.1 Design laws and pillars

**Sources.** Export lines 21–32 (ten principles); bible §2 (twenty laws in four groups).

**Lanes.** F-01…F-11 mapped the export's principles to the charter and glossary: player-as-Planner, quota periods, non-price clearing, physical movement, degradation-not-termination all ALREADY-EXIST in `charter-1.0.md:24-35`. Three are new to the corpus: "automate execution, not decisions" (F-10), "industrial logistics as the central pleasure loop" (F-09), "public transit dominant, private cars emergent" (F-08). F-04 flagged "border roubles" as a contradiction with the single-rouble charter; the bible fixed it.

**Lead audit of bible §2.** Laws 1–6 (physical) restate charter pillars plus "no silent deletion" (law 4) and "opportunity cost must be visible" (law 6) — both new, both good. Laws 7–10 (information) are the four-realities model as rules; law 10 "no hidden honesty flag" is verbatim `SPEC-PRODUCTION-009` (`production.md:54`). Laws 11–15 (social) are the social-reproduction thesis as rules. Laws 16–20 (technical) are architecture principles; law 19 (idempotent transactions) is new and matches the pattern already in `SPEC-WATER-006`, `SPEC-ELECTRICITY-002` ("one `ElectricityAllocationID` applies once").

**Tension the lead found.** Law 9 ("no omniscient player UI") versus bible §18.4's discrepancy inspector, which shows `consumed 91 t` and `on hand surplus 35 t` as bare facts. If reports are not truth (law 7), the Planner cannot simply *know* consumption. §13.20 resolves it — every Planner-visible value must say how it is known — but §18.4 does not apply its own rule. The vision document adds a provenance column to every inspector line.

**Verdict.** CONFIRMED as design laws; five are new to the corpus (no silent deletion, visible opportunity cost, information is a resource, one authority per transition, idempotent transitions).

**Landed.** `docs/vision/design-bible.md` §2.

### 3.2 The planned economy as a control system

**Sources.** Export lines 135–318; bible §5.

**Lanes.** A validated the mechanisms against Kornai (1980), Berliner (1957), Weitzman (1980), Gregory & Harrison (2005), Nove: shortage spiral CONFIRMED, ratchet CONFIRMED, "priority cannot solve scarcity" CONFIRMED, material-balance identity CONFIRMED (equivalent to the Gosplan equation), credibility *concept* CONFIRMED with the numbers UNSUPPORTED, five reserve classes PLAUSIBLE (the taxonomy is the conversation's invention), freight numbers UNSUPPORTED. G-32 added a periodisation caveat: the ratchet was strongest under Stalinist planning and weakened after 1965; the bible's §3.2 "late-Soviet behaviour should not be projected unchanged" covers this.

**Code.** `request_multiplier` is a static per-prototype `i32` (`prototypes/src/types/recipe.rs:52`), set to 4 for `flour-factory` and 3 for `slaughterhouse` (`base_mod/companies.lua:40,582`), wired in `recipe_init` (`goods_company.rs:22-26`), proven by `SCENARIO-0151` and `sov-lpj`. `Government` holds only `money` (`government.rs:9-11`). No quota, period, credibility, reserve classes, storming, or material balance. Lane A §3 designed each of these to data-structure depth with the test that proves it; those designs are carried into the vision document.

**Lead's finding neither source has: the spiral already has a physical floor.** `recipe_should_produce` (`goods_company.rs:45-46`) refuses to buy when `capital - reserved >= amount × (storage_multiplier + 1)`. An enterprise cannot request more than its storage holds. Lane G called the spiral "a death spiral with no floor"; the bible answers only with the positive spiral (§11.5). The real damping is warehouse capacity, and it is already in the code. Design consequence: *storage construction is a Planner-visible hoarding signal* — an enterprise that keeps enlarging its warehouse while reporting shortage is telling on itself. Recorded as design-bible §5.3a.

**Lead audit of bible §5.8 (priority inflation) and §5.13 (tolkachi).** Both are historically grounded: priority classes proliferated until Gosplan maintained lists of "especially important" consignments (Nove); tolkachi are documented from 1937 (Berliner; Lane A M-01). Both PLAUSIBLE as mechanics, ABSENT in code, Post-1.0.

**Lead audit of Lane A's missed list against the bible.** Of A's nine missed mechanisms, the bible contains tolkachi (M-01) and loading bottlenecks; it still lacks the soft-budget-constraint physical analogue (M-02: the Planner rescuing failing enterprises by taking from performing ones), plan-fulfilment falsification (M-03: reported output decoupled from physical stock change — the inspector must compare the two), assortment plans (M-04), the plan-correction cycle (M-05), investment hunger (M-06), forced-substitution chains (M-07), ministries as inflating aggregators (M-08), and OTK quality attestation (M-09). All nine are in the vision document.

**Verdict.** Mechanisms CONFIRMED; taxonomy details (five reserve classes, specific stats) are design proposals; code has the seed and nothing else.

**Landed.** Vision §5; `bd` drafts 1–3 (Planner observability of requested-vs-consumed, adaptive multiplier, plan periods).

### 3.3 Resources, production, logistics, construction

**Sources.** Bible §6 (the export had only the causal-distinctness rule, line 111).

**Lanes.** A-01 causal-distinctness CONFIRMED as a heuristic (Gosplan tracked ~1,943 product categories, not SKUs). E-003: 21 items, metadata is `id`, `label`, `optout_exttrade` only. E-128: dispatch is a real truck (ToSource → Loading → ToDestination → Unloading; 13 ledger tests, 14 retail tests) — the code's actual strength, which the fact-sheets under-credit post-`sov-abs`/`sov-6qx`. D §4.3: loading/unloading time is absent; `freight_station.rs:139` cargo is a counter.

**Lead audit of bible §6.** §6.2 stock semantics (on hand / reserved / in custody / embedded / consumed) — `SingleMarket` has `capital`, `reserved`, `requested`; no custody quantity on the vehicle (`LOG-SUB-005`), no "embedded in construction" (construction is instant, E-015). §6.3 haul lifecycle matches `DispatchState` minus "release/recovery" (the `sov-ahw` work in flight adds a ToSource timeout). §6.4 custody conservation is `SPEC-LOGISTICS` territory and the `ledger-invariant-checker`'s job. §6.6 target-stock dispatch priority (deficit → distance → stable ID) is verbatim `SPEC-LOGISTICS-005`/line 62 — ALREADY-EXISTS in draft spec. §6.8 handling classes: new, PLAUSIBLE, needs the item metadata that E-003 says is missing. §6.9 `run = min(recipe, input, labour, power, water, process, output space)` with the binding constraint recorded — `recipe_should_produce` already gates on input, storage and workforce; the *record* of which gate bound is absent (E-016). §6.10 `ProductionRunId` atomicity — new; matches the transition pattern in the utility specs. §6.11 "surplus must remain visible" is `SPEC-PRODUCTION-003`. §6.12 construction lifecycle (Ghost → Verdict → Site → gates → ground broken → activation) is the glossary's own vocabulary and `construction.md`; ALREADY-EXISTS as draft spec, ABSENT in code.

**Verdict.** Bible §6 is a faithful restatement of the draft specs plus three genuine additions (handling classes, run atomicity by ID, binding-constraint record).

**Landed.** Vision §6.

### 3.4 Citizens, households, time, housing, informal economy

**Sources.** Export lines 320–516 and 898–1171; bible §7.

**Lanes.** B1 validated 38 items: social reproduction loop CONFIRMED (Zaslavskaya); housing queue CONFIRMED with hard numbers (12–36% of families on lists, 10+ year waits in Moscow; three channels: enterprise, municipal, cooperative); time budgets CONFIRMED (Gordon & Klopov: 28 h/week domestic work for women vs 12 for men); private plots CONFIRMED (3% of land, 64% of potatoes, 40% of meat, 1966); blat CONFIRMED against Ledeneva (1998) as zero-sum, sparse, reciprocal, invisible to the state; the 8,000→20,000 settlement multiplier PLAUSIBLE (labour-force participation 50–55% gives 1.8–2.0×; 2.5× is high but in range). B2 matched sixteen of twenty CIA-derived observations to documents by ID and built a 27-row calibration table (33.65 h/week for a 1954 Moscow food basket; 13 m² per capita by 1980; ~17%/yr turnover; 88% female participation ages 20–54; alcohol ≈10% of national income; male life expectancy 67→62 between 1964 and 1980). B1 §3 designed the household time budget as a conservation law, the housing queue as a `BTreeMap` keyed by (channel, priority, registration tick), the blat graph as degree-bounded (MAX_TIES 5–8, ~24 MB at 250k), the retail information model, the productivity ramp, and cohort expectation memory with age-dependent learning rates.

**Code.** `HumanEnt` is one monolithic struct (`world.rs:87-105`); `PersonalInfo` is `{name, age, gender}`; `Home` is a bare `BuildingID` scored 0.2; one need (bread via `BuyFood`); age never increments; no household entity anywhere (`grep -r Household simulation/` is empty); `spawn_human` creates one person per empty house. Everything in this theme is greenfield (E-018…E-035, E-080…E-108).

**Lead audit of bible §7.** §7.1 L0–L4 layers are a cleaner statement of the export's record/body split; L4 "bounded visible citizens" matches the charter. §7.2 household ownership list matches `SPEC-HOUSEHOLDS-004` (shared pantry). §7.3–7.4 time as resource and the social-reproduction balance are B1-21 with B1's identity. §7.7 citizen knowledge is B1 §3d. §7.8 the adaptive need sequence (preferred → substitute → reserve/informal/plot → going without) extends `SPEC-NEEDS-003/004`. §7.10 blat "never creates bonus resources" resolves G-17's "nothing teleports" worry only if the physical custody chain is spelled out — B1 §3c does: the contact must have physical access at a building with stock, the debit is a real retail debit, and the next formal-queue citizen goes without. §7.12 expectations and cohorts is B1 §3g. §7.16 fertility "not `if happiness > X`" matches B1-24; births are still an open question in `citizens.md:117`. §7.17 biography retention classes are new and sound.

**Tension G-15 (sleep vs stale memory)** — the bible's §7.5 wake-on-event answers it only if memory is refreshed on wake through the change journal (§13.16), which is the "option (b)" G identified. The citizen-architecture proposal makes this explicit.

**What both GPT documents still lack (B1 §4).** Propiska (the eligibility gate for housing and employment — a natural Planner lever), limitchiki (temporary-registration workers in dormitories), trade unions as housing/sanatoria allocators (the bible §8.8 has unions but not this function), komandirovki as an informal goods channel, alcohol (the largest single cause of Soviet male mortality, a barter medium, and a policy lever — B2 documented it from CIA-RDP87T00787R000200200003-0), kommunalka→separate-flat as a two-tier housing queue, kitchen-vs-canteen time split, pensioners as queue-standing labour, two-shift schools, rumour as a lossy information channel, and a cycling deficit-goods list. All eleven are in the vision document's §7 with B1's reasons.

**Verdict.** CONFIRMED historically; ABSENT in code; the household entity is the first thing to build.

**Landed.** Vision §7; `docs/plan/proposals/citizen-architecture.md`; glossary terms (mikrorayon, blat, monotown, propiska, kommunalka).

### 3.5 Labour, workplace, unions, representation — lead's in-session audit of bible §8

No lane covered this; the export did not contain it. The lead checked it against secondary literature reachable this session.

- **§8.1 differentiated labour** (operators vs technicians vs inspectors; capacity gated by the scarcest role): PLAUSIBLE and matches the Liebig-bottleneck framing already in the charter's binding-constraint vocabulary. Code: `raw_productivity = workers.len() / n_workers` — one undifferentiated headcount (`goods_company.rs:85`). The qualification taxonomy is bible §21.4's own open question.
- **§8.2 tenure/adaptation ramp**: CONFIRMED by the turnover literature (Feshbach & Rapawy 1973; ~17%/yr turnover; replacement productivity ramps of months). B1 §3e already designed the ramp. Code: every worker contributes fully from tick 1 (E-100).
- **§8.3 labour hoarding**: CONFIRMED — Kornai ch. 11; enterprises carried 10–20% surplus workers against absence and storming (Filtzer; Kornai). Code: none (E-101).
- **§8.4 childcare as care-hours released, not a bonus**: CONFIRMED by B2-04 (day nurseries attached to factories, an 86% increase in places 1961–70). Kindergarten is a charter Post-1.0 cut; the bible correctly says "hook, not implementation".
- **§8.5 education → qualification → assignment → relocation**: CONFIRMED (raspredelenie, 1933 onward; B1-28, B2-13).
- **§8.6–8.7 work collectives and issue aggregation from physical facts** ("do not store `grievance = 82`"): design hypothesis, consistent with law 15 and the no-flag rule. No historical claim to check; the *work collective* as a legal actor is a 1983 institution (Law on Labour Collectives), later than the fixed 1950s–60s era — the vision document notes the anachronism.
- **§8.8 trade unions**: CONFIRMED that Soviet unions administered social insurance (from 1933), labour protection and technical inspection, and held a joint role in housing allocation and sanatoria vouchers (Deutscher 1950; the JSTOR "Trade Union in Soviet Social Insurance"; B1-MISSED-03 documents putyovki at 20% free). The bible's "not a Western bargaining organisation, not a strength meter" is the right framing. Post-1.0.
- **§8.9 safety inspection from physical inputs** (maintenance backlog, fatigue, overtime, machine condition): PLAUSIBLE; the ILO repository has Semenov (1983) on workers' participation in OSH in the USSR and the "technical inspector" institution. B2 §5.6 cites the CIA "Labor Safety in Soviet Industry" (1965, ID 2114). Storming raising accident rates is consistent with both.
- **§8.11 local Soviets and elections**: CONFIRMED that the useful mechanic is *nakazy* (electors' mandates — specific constituent requests a deputy is bound to pursue) and standing commissions (planning-and-budget, working and living conditions for women, mother-and-child welfare). Single-candidate elections with party-controlled nomination are the historical form; the bible's refusal to model multiparty competition is correct for the era. Post-1.0.
- **§8.12 representation error** (compare physical, lived, enterprise report, union report, local-Soviet report, Planner belief): this is the four-realities model applied to institutions and is the strongest new idea in §8. It gives the sensor-network framing a testable form: each channel has a measurable bias. Design hypothesis.
- **§8.13 institutional confidence as channel effectiveness** ("complaint submitted → action taken or not"): consistent with Lane A §3d's credibility record; a generalisation of it to citizens. Design hypothesis.
- **§8.14 alternative rulesets as "who decides" questions**: the same conclusion Lane H reached independently (modes are institutional-parameter presets). The bible's seven questions are a better spec skeleton than H's parameter table; H's scenario-vs-mode distinction (H §4.1) is what the bible still lacks.

**Verdict.** §8 is historically defensible and correctly labelled Post-1.0 throughout; two era caveats (labour collectives 1983; late-Soviet union functions) are added in the vision document.

**Landed.** Vision §8.

### 3.6 Vehicles, road traffic, transit, rail

**Sources.** Export lines 700–790; bible §9.

**Lanes.** D audited the substrate: `Vehicle` has no mass, cargo, power, owner or capacity (`vehicle.rs:26-45`); physics is `speed += clamp(desired - speed, -decel, accel)` (`road.rs:141-183`); collision is a spatial-grid cone check, not IDM (`road.rs:186-407`); pathfinding cost is `length/speed_limit + noise` with no BPR/Gawron (`pathfinding.rs:234-239`); rail is the most developed system (consist mass/length/power/braking from prototypes, intersection reservation, look-ahead braking, `train.rs:58-78,388-475`) but rolling-stock speeds are placeholders (locomotive 200 m/s = 720 km/h, `rollingstock.lua`). D §3 gave the cheapest adequate model for each: grade physics from the lane polyline's Z derivative (one `sin()` per vehicle per tick), IDM parameters per kind, BPR+Gawron on a per-lane EMA in `transport_grid_synchronize`, a CTM cell layer for meso spillback with identity-carrying freight promoted to micro.

**Lead audit of bible §9.** §9.2–9.4 longitudinal physics matches D §3.1 exactly (F_grade = m g sin θ; F_tractive ≈ P/v capped by traction; jerk clamp). §9.5 IDM/MOBIL "inspired" — D-06 warns the current code is *not* IDM and that MOBIL needs multi-lane roads the substrate lacks. §9.6 "queue storage and spillback" is D-09's CTM/LTM. §9.8 industrial gates is D-10. §9.9 shift waves: E-066 found work intervals already carry random offsets (`desire/work.rs:32-37`), so staggering is a cheap PARTIAL→EXISTS move. §9.11 minimal freight rail "preserve fields for wagon capacity, cargo custody" — D §4.4 found `RailWagon` has no cargo type or capacity field; the hook the bible asks for is absent. §9.13 wagon balance and §9.14 yards are D-03's missing half. §9.15 winter roads is D §4.9 in part.

**Missed by both GPT documents (D §4).** Junction deadlock is "resolved" by a random wait (`road.rs:217-225`, `wait_time = fract(pos.x*1000)*0.5`) — two vehicles nose-to-nose deadlock forever; rail uses intersection reservation, not signal blocks, so two trains may share a segment — signalling is the true capacity constraint (charter: signals are Post-1.0); the 60-second stuck-train creep (`train.rs:379-383`) phases trains through blocked junctions; lane-change determinism under parallelism; road wear from axle load; wagon↔cargo compatibility.

**Verdict.** Design PLAUSIBLE and well-sourced (Treiber 2000, Kesting 2007, Daganzo 1994, Yperman 2007, BPR 1964, Gawron 1998); substrate has ~4 of 11 proposed vehicle fields; rail is the best-developed subsystem.

**Landed.** Vision §9; `bd` draft 7 (placeholder rolling-stock speeds); open questions §7.

### 3.7 Utilities and physical inertia

**Sources.** Export lines 792–890; bible §10.

**Lanes.** D-10: electricity is a union-find over road adjacency (`electricity_cache.rs:244-279`), binary blackout when consumed > produced (`electricity.rs:43-93`), houses draw a fixed 100 W; `SPEC-ELECTRICITY-001` forbids exactly this ("a road … MUST NOT itself be an electrical connection"). D-14…D-17: water, sewage, heating, gas have zero substrate — no building kind, no system, no data structure. D §3 designed the cheapest adequate solver for each: tree-based static head for water (GGA/Newton is overkill; "floor 9 has no pressure" is a lookup), gravity DAG with junction buffers and backpressure for sewage, pipe FIFO delay line plus first-order building thermal ODE for heating, one linepack integrator per segment for gas, priority-sorted load shedding for electricity. D verified the W&R reference install: W&R models water/sewage/heating by binary connectivity with quality thresholds, no pressure.

**Lead audit of bible §10.** §10.1 "shared topology, separate physics" and §13.22 network kernel (typed nodes/edges, attachments, components, topology revision, CSR adjacency) — the only shared topology today is the road graph itself (C2-12). §10.2 water principles are `SPEC-WATER-001…006` verbatim (finite-rate transfer, quality, border meter, never cargo, idempotent `WaterTransferID`) — ALREADY-EXISTS as draft spec. §10.6 heating "no electric fallback" is `SPEC-HEATING-001` and `EVID-HEATING-002`. §10.8 `G + D = V + C + L` is `SPEC-ELECTRICITY-002`; "continuous non-price priority shedding" is `-003`. §10.11 hydro `P = ρ g Q H η` and reservoir mass balance are in the charter's breadth exception; no spec or code. §10.12 weather through explicit interfaces — `heating.md` already requires "a ratified Weather interface" and no weather spec exists (bible §21.1 lists it as missing; D §4.9 shows why it matters: one weather state stresses every network at once). §10.13 network-reserves table is D-18/D-21, called "a powerful unifying Planner concept" by D. §10.14 phase lag is the design consequence of inertia.

**Missed by both.** Water-main freeze-up coupling water to weather and heating (D §4.7); coal calorific grades (D §4.8); the fact that replacing the union-find is a *full replacement* of the connectivity model, not an increment (D §4.6).

**Verdict.** CONFIRMED that the specs already say what the bible says; ABSENT in code except binary electricity; D's solver sketches are the cheapest known adequate models.

**Landed.** Vision §10; open questions §7 (pressure/head in the first water implementation? gas in scope?).

### 3.8 Cross-system causal loops

**Sources.** Export lines 87–103 (the electronics cascade), 230–244; bible §11.

**Lanes.** A-09 CONFIRMED storming's recursive upstream propagation as a valid construction from the documented three-phase monthly cycle (*spyachka*, *goryachka*, *likhoradka*). B1-22 CONFIRMED the retail-shortage → time-poverty → labour loop. B1-04 CONFIRMED the monotown housing → turnover → production loop (Feshbach; tekuchest').

**Lead audit of bible §11.** §11.1 extends the export's cascade with a quality/rework branch (Lane A M-09 OTK) and a worker layer (overtime → fatigue → safety complaints → institutional pressure → maintenance/welfare trade-off) — the latter is §8.9 applied. §11.2 coal → electricity → water → sewage → heat is D's cross-lane hook list as a single chain; "different systems respond at different speeds — that delay is the mechanic" is the inertia thesis stated as gameplay. §11.6 national-project privilege loop replaces any "national project penalty" modifier — consistent with §22.2's anti-pattern rule.

**Verdict.** CONFIRMED as design; every link is either a spec'd mechanism or a Lane-validated one. Nothing in code links any two of these systems except electricity → productivity (`goods_company.rs:104-108`).

**Landed.** Vision §11.

### 3.9 Game modes, national projects, progression

**Sources.** Export lines 482–504; bible §12.

**Lanes.** H: the fifteen modes are institutional-parameter presets over one simulation (~4–8 parameters each); Sovnarkhoz (1957–65, 105→47 councils) and the Kosygin reform (1965, 4–8 indicators, profit retention, "changing the names on doors") are the historically richest; Enterprise Director inverts the core loop and needs an AI Planner — "structurally a different game"; Frostpunk *does* have game over, so "never game over" is this project's own choice and its pressure must be qualitative; the cheapest three to ship are the Taut Plan (quota targets over the existing `request_multiplier`), Frontier Corridor (a scenario on a linear map with existing rail), and Everyday Socialism (a scenario over the existing `BuyFood`/`Home`/`Work` loop). H §3 wrote a mode card for each of sixteen modes (premise, start state, rule changes, pressure source, win-less success, 10-minute loop, 10-hour arc, what it teaches, dependencies) and a dependency graph over base systems. H §4 added what the export missed: scenario-vs-mode as separate concepts (`Scenario` = starting state, `Mode` = rule preset; combinable), mid-save mode switching with a transition cost, multiplayer as a Gosplan-vs-ministry mode over the existing `networking/` crate (`WorldCommand` has no role filter), chronicle mode as a feature not a mode, the tutorial problem (the charter's First Plan must teach the dishonest-enterprise loop through play), and unlocking (the three authored plans are the progression).

**Lead audit of bible §12.** The bible's list is the export's list with one-paragraph premises and the same conflation of scenario and mode; §12.8/12.9 correctly say "the physical economy stays the same; authority changes". §8.14's "who decides" questions are the parameter set H's table approximates. The bible does not mention the tutorial, the chronicle, multiplayer, or mid-save transitions.

**Verdict.** CONFIRMED historically (Kibita 2013; Britannica; AEA JEP 5:4); all Post-1.0 except that the three authored plans *are* the progression ladder and the First Plan *is* a mode-design problem (H-16).

**Landed.** `docs/vision/game-modes-post-1p0.md` (H's cards, H's additions, the bible's "who decides" questions).

### 3.10 Rust architecture — sources versus code

**Sources.** Export lines 522–693, 1102–1116; bible §13–§16.

**Lanes.** C2 traced every proposal to the code: the actual schedule is 18 registered systems in flat registration order (`init.rs:54-114`; registration count 18) with electricity *first* and map update second-last — the opposite of the proposed order; `PlannerSnapshot` ABSENT, the UI holds `Arc<RwLock<Simulation>>` with ~40 direct resource reads; `Market` has six responsibilities but not the six the export named (order book, matching, dispatch lifecycle, retail claims, external trade, price calculation); randomness is one global Xorshift128 drawn sequentially (`rand_provider.rs`) plus an ad-hoc positional hash (`common::rand::rand2`); every system runs every 20 ms tick; `rerun.rs` is 48 lines of dead code; the multiplayer crate assumes lockstep (`Frame(u64)`, `assert_eq!(frame, tick+1)` in `headless/src/main.rs:65-69` and `native_app/src/network.rs:199`). C2 §3.1 gave the dependency DAG (keyed randomness and typed contexts → phase order → parallelism; record/body → SoA → bitsets; snapshot → four snapshots → Gosplan computer; journal → inspector) and a first-commit migration sketch per proposal. C2 §4 found the risks neither GPT document names: no save-migration mechanism (`lib.rs:389-441` only defaults missing resources; version mismatch warns), replay-test brittleness (`test_iso.rs` proves round-trip, not repeat-run; every reorder regenerates `world_replay.json`), the `exec_ent` closure channel, `f32` positions everywhere.

C1 verified every crate on crates.io and GitHub: `egui` and `yakui` are git deps with **no rev pin** (`Cargo.toml:22-33`; lockfile pins `d4e8966a` and `6c6982ff`; 13 packages from 2 git sources); `slotmapd` is Uriopass's own 0-star fork of `slotmap` for serialisation-cycle determinism and *is* the entity store — the export's hint at hecs/legion/bevy_ecs is WRONG for this codebase; `FxHasher` is used for all maps and `hash_u64` and is not portable — canonical digests need `xxhash-rust` (or `blake3`); float determinism is UNADDRESSED (`geom/` calls `.sin()/.cos()/.sqrt()` as intrinsics; no `libm`); Salsa v0.28.2 is viable for a per-tick derived layer (~200 µs for 1,000 inputs + 100 queries) but a hand-rolled dirty-flag layer is cheaper first; Differential Dataflow has no shipped-game precedent; LP maps to `good_lp` + `microlp` (pure Rust, MILP since 0.6), `minilp` is abandoned; `rayon` is used once (`terrain.rs:66`); `ParCommandBuffer` is an intent buffer but sequential; `enum-map` and `bytemuck` are already transitive deps.

**Lead audit of bible §13–§16.** §13.1 "do not put ECS at the centre; identity, time, authority, transactions, change propagation, determinism, information boundaries" — consistent with C1's finding that the world is a hand-rolled typed store, not an ECS, and with §14.15 "do not migrate to Bevy/hecs/Shipyard". §13.1's module shape (`core/ stores/ physical/ society/ institutions/ observatory/ forecast/ snapshot/`) is a reorganisation of today's `economy/ souls/ map_dynamic/ transportation/ map/`; the bible rightly says modules, not crates. §13.2 two identity families (append-only dense for citizens/households; generational slotmap for bodies/vehicles) — matches C2-08 and the archived Bevy ADR-0004; a dead Citizen #N stays #N. §13.5 non-zero IDs for `Option` niche — correct Rust. §13.6 fixed resource arrays — C1 §4.2 names `enum-map`, already transitive. §13.7 bitset society — C1: `fixedbitset`/`roaring`; needs dense citizen IDs, which slotmap keys are not. §13.9 deterministic calendar — C1: `hierarchical_hash_wheel_timer` exists; C2-17 ABSENT. §13.11 eleven phases with REPORTING — see the sim-tick-phases proposal for the reconciliation with the actual order. §13.12 "do not make correctness depend on `DashMap`, lock order, or Rayon scheduling" — C1 §3.3 confirms `ParCommandBuffer`'s `Mutex<Vec>` would be non-deterministic under parallel insertion; commands must carry a source key and be sorted. §13.13 keyed random `(seed, domain, entity, ordinal)` — C2-09 says this is the MUST-DO-FIRST. §13.14 typed contexts — **the bible does not address the `exec_ent` closure channel** (C2 §4.3). §13.16–13.18 change journal, observatory, causal facts with retention classes — C2-11 ABSENT; `EcoStats` is ring-buffer trade history only; the proposal starts with a `ChangeJournal` resource and one event kind. §13.19 four snapshots via `ArcSwap` — `arc-swap` 1.7.1 is already a dep (C1-21). §13.20 "identify how the Planner knows a value" — the rule §18.4 forgets. §13.21 hierarchical routing — C2-10: flat A* per request; `fast_paths` or similar. §13.23 shadow sim from *reported* state — C2 §2.10: `Simulation` is Serialize+Deserialize and headless ticks, so `fork()` is possible but ~100 ms per clone. §13.24 `good_lp + HiGHS` — C1 prefers `microlp`; see §6. §13.25 GPU boundary with validated POD — `bytemuck` already present.

§14 standards: the lead finds them consistent with the draft specs' own transition patterns (`WaterTransferID`, `ElectricityAllocationID`) and with the charter's save rule ("explicit version-gated hard breaks during development; compatible from RC"). §14.12's envelope (magic, format version, schema version, codec, sizes, checksum, payload) is what C1-22 recommends first — but **no migration function**; C2 §4.1's `SaveMigration` seam must accompany it or Phase 1 breaks every save. §14.13 BLAKE3 vs C1's xxhash — open conflict. §14.7 float standard omits the `libm` prerequisite for cross-platform determinism (C1-14). §14.5 "no `String` in hot core records" — today `PersonalInfo.name: String` is boxed on every `HumanEnt`.

§15 crate decisions, checked by the lead this session: `typed-index-collections` 3.5.0 (Jan 2026, 5.0M downloads) exists; `shuttle` 0.9.3 (Aug 2026, awslabs) exists and is the concurrency tester the bible means (Context7 resolves the name to shuttle.dev, a different project — cite `awslabs/shuttle` explicitly); `iai-callgrind` 0.16.1 (Jul 2025) exists and needs valgrind installed; `good_lp` selects its default solver by feature flag (coin_cbc > highs > lpsolve > microlp > …) and HiGHS "is written in C++, needs a C compiler, no additional libraries typically required on Linux" (good_lp README) — so the C1/bible disagreement is about build-toolchain risk on the Windows CI target, not availability. `proptest` vs `quickcheck` (1.0.3 already a dev-dep) — duplication, open. `postcard + zstd` vs bincode + envelope — open. `faer`, `uom`, `rkyv` — prototype-only in both sources.

§16.2 optimisation hierarchy (representation → cadence → locality → incremental → hierarchy → parallelism → SIMD) is sound and matches G-06's arithmetic: 250k × ~320 B = 80 MB of scattered citizen state; a naïve full pass costs ~2.7 ms sequential and 3–8× worse scattered; the active fraction per tick is the number nobody has stated (G's open question 5). §16.6 phase digests for determinism bisection — new, and the answer to C2 §4.2's "the test detects divergence but cannot say which system".

**Verdict.** The architecture is a coherent target; every element is ABSENT or PARTIAL; the ordering is the deliverable, and C2's DAG plus the bible's §20 agree on it except that the bible's Phase 1 omits the save-migration seam and keyed randomness (it lists "keyed RNG" and "world digest" — good — but not migration).

**Landed.** `docs/plan/proposals/sim-tick-phases.md`, `citizen-architecture.md`, `causal-inspector.md`; `docs/research/rust-architecture-proposals-2026-08-28.md`; §6 open conflicts; `bd` drafts 4–6 (pin git deps, save envelope + migration seam, repeat-run determinism test).

### 3.11 Validation and tests — lead's audit of bible §17

§17.1 conservation property (source + destination + custody + embedded + consumed = initial + sources) is the `ledger-invariant-checker`'s standing question and is partly covered by the 13 `ledger.rs` scenario tests (E-136). §17.2 idempotency "apply twice, second is a no-op" matches every utility spec's EVID row. §17.3 mutation tests "name the deliberately wrong implementation that must fail" is exactly the `evidence-auditor`'s method and the cargo-mutants ADOPT decision (`sov-mwy`, commit `339edec`); the bible's examples (credit at reservation, satisfy at arrival, activate a Site early, double-apply a meter delta) are the EVID tables' "mutation" columns. §17.4 repeat-run determinism: **absent** — `TestCtx::check_determinism` (`tests/mod.rs:106-121`) proves serialize→deserialize hash equality, not same-inputs→same-state (E-088, C2 §4.2, substrate.md row 5). §17.5 reference oracles (EPANET, SWMM, HEC, IDM, CTM) are D §2's sources used as tests — sound. §17.6 eight benchmarks: none exist; `sov-1ae` cancelled; `perf-engineer.md` already says no bench runner exists.

**Landed.** `bd` draft 6; vision §17.

### 3.12 UI, observability, the causal inspector

**Sources.** Export line 46 and 308–316; bible §18.

**Lanes.** E-105/E-131: the building inspector shows workers, productivity, power, network health, progress, recipe, storage per item (`inspect_building.rs:150-267`); the human inspector shows location, destination, house, last-ate, work (`inspect_human.rs:17-80`); no STATUS/CAUSE/TREND anywhere; `Market::requested()` is public and unread by `native_app/`. G-23: "every aggregate clickable down to real trains" requires an index whose cost scales with entities × stats and must be budgeted. F-OVERLAP-01: no spec exists for the inspector.

**Lead audit of bible §18.** §18.1 five-line contract; §18.2–18.4 worked examples (cold apartment chain, worker-shortage drivers, enterprise discrepancy) are excellent and become the proposal's acceptance examples — with the provenance column added (§3.1 tension). §18.5 pressure maps and §18.6 reserves in natural units ("coal bunker 18 h at current burn") are the network-reserves table made visible. §18.7 notifications from causal state match the charter's "action-needed notifications and event log" shell commitment.

**Landed.** `docs/plan/proposals/causal-inspector.md`; `bd` draft 1.

### 3.13 Scope

**Lanes.** F-CHARTER-SCOPE: of ~90 export ideas, ~35 are 1.0, ~25 Post-1.0, ~15 architecture, ~15 vision. F-CONTRADICTS-01…05: 250k is a target with no gate; "border roubles" retired; auto-lots still exist; the phase order is unanchored; the no-flag rule already exists in `SPEC-PRODUCTION-009`.

**Lead audit of bible §19.** §19.1's 1.0 list is charter-faithful line by line (checked against `charter-1.0.md:36-58`). §19.2 "hooks now, mechanics later — a hook means avoiding an architectural dead end, not implementing dormant complexity" is the right discipline and is applied in the proposals. §19.3 restates the charter's cuts exactly.

### 3.14 Implementation sequence — bible §20 versus the lanes

The bible's Phases 0–10: architecture contract and missing specs → prove 250k representation → one physical chain → dishonest-enterprise loop and observatory → construction → households → movement scale → utilities → labour/services/demography → Plan/Quota/Tranche → authored plans and polish.

Reconciled with C2's dependency DAG, E's "ten cheapest PARTIAL→EXISTS moves", H's "cheapest three modes", and G-10's bootstrap warning:

- **Phase 0 must add** the save-migration seam (C2 §4.1) and pinning the git deps (C1-01/02) — both are prerequisites for everything after.
- **Phase 1 must start with keyed randomness** (C2: MUST-DO-FIRST) and a repeat-run determinism test (E-088) — otherwise 250k SoA work cannot be proven deterministic.
- **Phase 2's "one complete physical chain" already exists for the truck leg** (E-128; 13 ledger tests) — the missing pieces are export-side physicality (`market.rs:774`), loading/unloading time, and custody on the vehicle. The bible's "disable conflicting inherited fulfillment paths" is E's five CONTRADICTED rows.
- **Phase 3 is the cheapest high-value step in the whole plan**: wire `Market::requested()` into `inspect_building.rs` (~30 lines, E §3 item 1) and the Planner can catch the first dishonest enterprise *today*. G-10's bootstrap problem is answered by this: the minimum viable loop is request-vs-consumed on screen.
- **Phase 5 households** is the first greenfield entity and the gate for everything in §3.4.
- **Phase 9 Plan/Quota/Tranche "only after physical economy and lived scarcity are meaningful"** — H-07 disagrees in one respect: the Taut Plan needs only quota targets over the existing multiplier and is the cheapest mode; a minimal quota period could come earlier as the plan-period clock that storming, ratchet and credibility all depend on (Lane A open question 2).
- **The bible has no tutorial phase.** H-16: the First Plan is a mode-design problem the charter binds; it belongs in Phase 10's "authored plans" explicitly.

**Landed.** Vision §20 (reconciled sequence).

## 4. Code reality — verified by the lead

Commands run on `4e9e930b2a73` this session (output abbreviated):

```
$ grep -n 'cap -= qty_sell' simulation/src/economy/market.rs
774:                    *cap -= qty_sell;
$ grep -rn 'set_requested' simulation/src --include=*.rs | grep -v tests/
simulation/src/economy/market.rs:495:    pub fn set_requested(...)
simulation/src/souls/goods_company.rs:24:        market.set_requested(soul, item.id, qty);
$ grep -n 'money -=' simulation/src/economy/mod.rs simulation/src/world_command.rs
simulation/src/world_command.rs:225:        sim.write::<Government>().money -= cost;
simulation/src/economy/mod.rs:54:        gvt.money -= n_workers as i64 * WORKER_CONSUMPTION_PER_MINUTE;
$ rg -n '^\s*register_system(?:\(|_sim\()' simulation/src/init.rs | wc -l
18
$ grep -rn 'dishonest' simulation/src --include=*.rs | grep -v tests/
(empty)
$ grep -n 'static mut\|OnceLock' simulation/src/init.rs
168:static REGISTRY: std::sync::OnceLock<Registry> = ...
$ grep -n 'git =' Cargo.toml
22:egui = { git = "https://github.com/emilk/egui" }        (no rev)
29:yakui = { git = "https://github.com/Uriopass/yakui", branch = "dev" }   (no rev)
$ grep -c 'label' base_mod/items.lua
21
```

| Claim | Status | Evidence |
|---|---|---|
| Export sell side teleports | **CONTRADICTED pillar** | `market.rs:774` debits seller capital at match time; no `Dispatch` is created for the export trade (pushed after the dispatch loop, lines 777–784). Import side was fixed by `sov-abs`. |
| Domestic money gates actions | **CONTRADICTED pillar** | `world_command.rs:225` (buildings, roads, trains), `economy/mod.rs:54` (10 cents per worker per minute). Money can go negative (not a hard gate) but it is a price in a non-price domain. |
| Auto-lots | **CONTRADICTED design target** | `map/map.rs:682-720` generates roadside lots on road construction (`MAP-SUB-002`); `SPEC-ZONING-003` forbids spawning from intent. |
| `request_multiplier` static | PARTIAL | Per-prototype constant; no reliability memory in `GoodsCompanyState` (`goods_company.rs:69-78`). |
| Dishonest enterprise wired, unobservable | PARTIAL | Production caller exists; `Market::requested()` unread by UI. `wave1-economy.md` ECO-SUB-005 ("no non-test caller") is stale since `0caee71`. |
| No hidden flag | ALREADY-EXISTS | No `dishonest` identifier in non-test code; `SPEC-PRODUCTION-009`. |
| `static mut` registries | STALE claim in fact-sheets | `init.rs:168` is `OnceLock`; fixed 2026-08-26. |
| Save version | PARTIAL | `lib.rs:378-412`: version string stored; major mismatch only warns; no migration. C1's "no envelope" was wrong on the version field, right on migration. |
| 18 systems, flat order | CONFIRMED | `init.rs`; `update_map` is second-last and `add_souls_to_empty_buildings` is last. |
| Hoarding has a storage floor | CONFIRMED (new) | `goods_company.rs:45-46` `storage_multiplier + 1` cap. |

## 5. What both GPT documents miss — consolidated

| # | Missed item | Found by | Why it matters |
|---|---|---|---|
| 1 | `networking/` lockstep crate (~1,100 LOC) | G-28, C2-14, H-15 | Every parallelism proposal must produce bit-identical frames or multiplayer breaks; alternatively decide to drop the crate |
| 2 | Storage capacity already floors the hoarding spiral | lead | Turns "death spiral" into a bounded mechanic and makes warehouse building a hoarding signal |
| 3 | `ParCommandBuffer::exec_ent` `FnOnce(&mut Simulation)` closures | C2 §4.3 | Blocks typed contexts and parallelism; the main cross-system mutation channel |
| 4 | No save-migration mechanism | C2 §4.1 | Every structural proposal invalidates saves; "one continuous save" is a charter value |
| 5 | Replay test proves round-trip, not repeat-run | E-088, C2 §4.2 | Determinism is asserted, not proven; reorders silently regenerate the baseline |
| 6 | `libm` for cross-platform floats; `FxHasher` not portable | C1-10, C1-14 | Any cross-machine replay or multiplayer claim is false today |
| 7 | `egui`/`yakui` git deps unpinned | C1-01/02 | Build not reproducible; `cargo update` advances silently |
| 8 | Ministries as inflating intermediaries | A M-08 | Real planning was three-level; a "dishonest ministry" aggregates and is harder to catch |
| 9 | Soft budget constraint's physical analogue | A M-02 | Never-game-over forces the Planner to rescue failing enterprises from performing ones — a perverse incentive to model explicitly |
| 10 | Plan-fulfilment falsification; assortment plans; investment hunger; substitution chains; OTK quality gate | A M-03…M-09 | Each is a distinct dishonest-enterprise behaviour with its own observable |
| 11 | Propiska, limitchiki, kommunalki, pensioner queue labour, canteen split, two-shift schools, rumour, komandirovki, alcohol, deficit-goods list | B1 §4 | Central to Soviet daily life; each yields distinct gameplay; alcohol is CIA-documented at ~10% of national income |
| 12 | Emigre/Moscow/Cold-War bias in CIA sources; "the normal experience was functional but constrained" | B2 §4 (bible §3.2 has this) | Baseline state must be functional-but-taut, not perpetual emergency |
| 13 | Junction deadlock is a random-wait hack; 60 s stuck-train creep | D §4.1, §4.12 | Real deadlock resolution is missing |
| 14 | Rail signalling as the true capacity constraint; wagon cargo capacity absent | D §4.2, §4.4 | Two trains can share a segment today |
| 15 | Rolling-stock speeds are placeholders (720 km/h) | D §4.11 | Data bug |
| 16 | Water-main freeze; coal grades; unified weather stress | D §4.7–4.9 | Weather is a missing spec (bible §21.1 agrees) |
| 17 | Scenario vs mode; mid-save transitions; chronicle; tutorial | H §4 | The First Plan is a charter-bound mode-design problem |
| 18 | Enterprise Director contradicts the Planner information model | G-14, H-09 | Standalone expansion, not a base-game mode |
| 19 | Active-fraction number for 250k | G open Q5 | The one number that decides feasibility |
| 20 | Inspector lines need provenance | lead | Otherwise the four-realities model collapses in the first panel |
| 21 | Work collectives are a 1983 institution | lead | Era caveat for the fixed 1950s–60s setting |

## 6. Open conflicts (both sides recorded; decision left to the Planner)

| Topic | Bible says | Lane says | What would decide it |
|---|---|---|---|
| Canonical state digest | BLAKE3 (§14.13) | `xxhash-rust` XXH3: fast, portable, BSL-1.0; BLAKE3 is overkill unless adversarial inputs matter (C1-10) | Whether digests must survive adversarial inputs (multiplayer anti-cheat) |
| LP backend | `good_lp + HiGHS` (§13.24) | `good_lp + microlp`: pure Rust, MILP since 0.6; HiGHS is C++ (C1-20). Lead: good_lp README confirms HiGHS needs a C compiler, no extra libs on Linux | Windows CI build cost vs solver speed on large plans |
| Property testing | `proptest` (§15) | `quickcheck` 1.0.3 is already a dev-dep; `proptest` duplicates (C1-18; prior survey) | Whether shrinking quality justifies two frameworks |
| Released save codec | `postcard + zstd` (§14.12) | Envelope around existing bincode first; format switch premature (C1-22) | Whether a codec change is worth breaking saves now |
| Determinism digest hashing of `f32` | `fixed` for conserved quantities (§14.7) | Same, plus `libm` for transcendentals (C1-14) | Whether cross-platform replay is a 1.0 goal (C1 open Q1, C2 open Q6) |
| Phase count | Eleven phases incl. REPORTING (§13.11) | Actual order is electricity-first, map-last; label before reorder (C2 §3.2) | Whether replay compatibility across versions matters |
| Snapshot library | `ArcSwap` "upgrade only when dependency policy permits" | `arc-swap` 1.7.1 already a dep; 1.9.2 available (C1-21) | Dependency policy |
| Salsa | "prototype candidate" (§13.17) | Hand-rolled dirty flags first; Salsa only past ~20 query types (C1 §3.1) | Query-graph complexity when the observatory exists |
| Enterprise intent state | "No hidden honesty flag" (law 10) | G-09: inflation needs per-enterprise state (reliability memory, reserve targets); "hidden from UI" ≠ "absent from sim". A §3a: `reliability_memory` EMA | Adopt A's model: rich hidden *state*, no hidden *verdict* |
| Ratchet universality | Always-on (§5.4) | Weakened post-1965 (G-32); bible §3.2 acknowledges era | Era is fixed 1950s–60s: ratchet is strong in-era; note the caveat |

## 7. Open questions for the Planner — consolidated

Economy (A): adaptive multiplier or Planner-set request limits? Are plan periods player-defined or emergent (everything temporal depends on it)? Tolkachi as physical worker travel or a button? Five reserve classes or three? Ministry layer ever?
Society (B1, B2): household composition at spawn (historical mean 3.5–4.0)? Propiska as a direct lever? Alcohol in scope? Kommunalka tier? Blat visible individually or only as aggregate anomalies? Gendered time budgets? Fertility in 1.0? Which era for the calibration table (1954 vs 1980s numbers)? Regional supply tiers?
Architecture (C1, C2, G): cross-platform determinism a 1.0 requirement? Save migration before any structural refactor? Which resources may the Planner *not* see? Keep lockstep multiplayer? Replay compatibility across versions? Active fraction per tick at 250k? What is the enterprise intent model?
Physics (D): grade physics in 1.0? IDM or incremental? Meso CTM in 1.0 or BPR/Gawron first? Gas in scope for design? Fix rolling-stock speeds now? Unified weather authority in 1.0?
Modes (H): modes switchable mid-save? Pursue Enterprise Director at all? Do the three authored plans teach three modes? Is the charter's three-plan ladder the progression? How much enterprise AI? Multiplayer worth the investment?
Bible §21: agriculture, terrain/geology, weather, hydrology, pollution, Plan/Quota/Tranche, authored plans, notifications, shell/save/crash, presentation/audio all lack specs; resource units and handling classes; qualification taxonomy; lane capacity measure; the 1.0 station trio; water pressure in the first implementation.

## 8. Consolidation map — where everything landed

| Content | Destination | Notes |
|---|---|---|
| Raw export, raw bible, session transcript | `docs/archive/raw-sessions/` + `INDEX.md` table | Provenance only |
| This reconciliation | `docs/research/conversation-mining-2026-08-28/SYNTHESIS.md` | You are here |
| Lane reports A–H | same directory, unchanged | Research record |
| Design laws, four realities, control loop, resources/production, citizens/households, labour/institutions, vehicles, utilities, cross-system loops, architecture principles, standards, sequence, anti-patterns, taxonomy | `docs/vision/design-bible.md` | Curated bible + every lane correction and addition, each labelled; replaces the three separate vision files Lane F proposed |
| Sixteen mode cards, scenario-vs-mode, tutorial, chronicle, multiplayer-as-mode, "who decides" questions | `docs/vision/game-modes-post-1p0.md` | Post-1.0 direction |
| STATUS/CAUSE/TREND/POLICY/PHYSICAL CHAIN, material-balance drill-down, provenance column, worked examples | `docs/plan/proposals/causal-inspector.md` | Advisory until ratified |
| Actual vs target phase order, label-then-reorder, keyed randomness, intent-buffer determinism, lockstep constraint, `exec_ent` obstacle | `docs/plan/proposals/sim-tick-phases.md` | Advisory |
| L0–L4 layers, record/body split, SoA, dense IDs, event calendar, cadence bands, bitsets, wake-and-refresh, save migration | `docs/plan/proposals/citizen-architecture.md` | Advisory |
| Crate-by-crate verified findings, the open conflicts, the cheapest path per technique | `docs/research/rust-architecture-proposals-2026-08-28.md` | Research snapshot |
| Storming, ratchet, mikrorayon, blat, capital dilution, monotown, tolkach, propiska, kommunalka | `docs/reference/glossary.md` | Terminology only |
| Actionable findings | `docs/research/conversation-mining-2026-08-28/bd-drafts.md` | Drafts; not filed |
| Stale claims | fixed in place (§9) | |

## 9. Stale documentation fixed in this pass

1. `docs/research/fact-sheets/wave1-economy.md` ECO-SUB-005 and its verification boundary: added a dated drift note — `set_requested` has a production caller since `0caee71` (`goods_company.rs:24`); the observability half of the finding still stands.
2. `docs/reference/architecture/substrate.md`: initialization row — `static mut` replaced by `OnceLock` (`init.rs:168`, 2026-08-26); ECO-SUB-002 row — import side physical since `sov-abs`, export side (`market.rs:774`) still debits at match time.
3. `docs/process/development-cycle.md:57`: `perf-engineer` row no longer claims "five bench gates at 250k" (the charter names none; `sov-1ae` cancelled).
4. `docs/SUMMARY.md`: new vision, proposal, research and archive entries added.

Not changed: the lane reports (research record), the specifications (draft, untouched), the charter.

## 10. Sources

Primary: the three archived raw files; lane reports A, B1, B2, C1, C2, D, E, F, G, H in this directory; `docs/plan/charter-1.0.md`; `docs/reference/glossary.md`; all 22 files under `docs/reference/specifications/`; `docs/reference/architecture/substrate.md`; `docs/research/fact-sheets/wave1-*.md`; `docs/plan/proposals/gosplan.md`; `.planning/architecture-review-2026-08-27.md`, `.planning/agent-roster-review-2026-08-27.md`.

Code cited: `simulation/src/{init,lib,world,world_command}.rs`, `economy/{market,mod,government,ecostats}.rs`, `souls/{goods_company,human,freight_station}.rs`, `souls/desire/{buyfood,home,work}.rs`, `map/{pathfinding,electricity_cache,map}.rs`, `map_dynamic/{electricity,dispatch}.rs`, `transportation/{vehicle,road,train}.rs`, `utils/{scheduler,par_command_buffer,rand_provider}.rs`, `tests/{mod,test_iso}.rs`, `tests/scenarios/*.rs`; `common/src/{hash,rand,saveload}.rs`; `networking/src/`; `native_app/src/{game_loop,network}.rs`, `gui/inspect/*.rs`; `prototypes/src/types/recipe.rs`; `base_mod/{items,companies,rollingstock}.lua`; `Cargo.toml`, `Cargo.lock`.

External (verified by the lanes; URLs in their §7 sections): Kornai 1980; Berliner 1957, 1976; Weitzman 1980; Gregory & Harrison 2005; Nove 1977/1980; Harrison 2011; Ledeneva 1998; Andrusz 1984; Morton 1980; Feshbach & Rapawy 1973; Gordon & Klopov 1972; Zaslavskaya 1988; Filtzer 1994; Bater 1980; Wädekin 1973; Lovell 2003; Kibita 2013; Treiber et al. 2000; Kesting et al. 2007; Daganzo 1994; Yperman 2007; Gawron 1998; BPR 1964; EPA EPANET/SWMM; 48 CIA/DI documents and 86 JEC papers listed in B2 §8; crates.io API records listed in C1 §7. Lead this session: crates.io records for `typed-index-collections` 3.5.0, `shuttle` 0.9.3 (awslabs), `iai-callgrind` 0.16.1; `good_lp` README via Context7 (`/rust-or/good_lp`); three `cia.gov` fetches (HTTP 403); four web searches on Soviet unions, local Soviets/nakazy, labour hoarding/turnover, ILO OSH participation.
