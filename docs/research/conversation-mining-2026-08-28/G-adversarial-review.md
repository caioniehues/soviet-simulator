# G — Adversarial review: what GPT got wrong, glossed, invented, or contradicted

**Kind:** research
**Authority:** research
**Status:** active
**Owner:** project lead
**Last verified:** 2026-09-03
**Source:** GPT conversation export `gpt-vision-export-2026-08-28.md` (2026-08-28), validated against code paths cited inline; synthesis 2026-08-28

## 0. Summary (ten most important findings)

1. **G-01** — The ten-phase deterministic schedule (`COMMAND → TOPOLOGY → … → ACCOUNTING`) does not exist in the code; the actual schedule is flat registration-order serial systems. Presenting it as an "architecture conclusion" implies existing infrastructure that does not exist.
2. **G-02** — Every typed newtype the export proposes (`Currency`, `OnHandQty`, `ReservedQty`, `CustodyQty`, `HaulId`, `ConsumptionId`, `PlannerSnapshot`) is absent from the codebase. These are design aspirations presented as architecture conclusions.
3. **G-03** — `CitizenRecord`/`CitizenBody` split, `CitizenStore`, `CitizenCore` — none exist. The actual citizen is `HumanEnt`, an indivisible struct with physics, routing, food-buying, and personal info fused together. The claimed architecture is a wholesale rewrite, not an incremental improvement.
4. **G-04** — The numbers at lines 219–224 (72%/141%/24%/18%) and 261–265 (61%/67%/82%/91%) are unsourced illustrative fabrications. No citation, no model, no calibration. If taken as parameters they imply a fidelity the game cannot deliver or test.
5. **G-05** — "Do not store a hidden dishonest flag" (line 48) directly contradicts the need for per-enterprise intent state to make the core loop work. `ECO-SUB-005` confirms the dishonest enterprise is test-only and unobservable in gameplay — but the export never acknowledges that the observable-discrepancy approach has no implementation path described.
6. **G-06** — 250,000 citizens with households, social graphs, biography, expectations memory, time budgets, qualification history, and informal-network ties would exceed any plausible per-tick CPU budget at 60 fps. The export never does the arithmetic.
7. **G-07** — The export proposes Salsa for the derived/incremental world and Differential Dataflow as an advanced option, but never grapples with the fact that neither is used in the codebase, Salsa's programming model (queries over immutable inputs) does not map to a mutable physical simulation, and Differential Dataflow is a research system with no game-engine integration story.
8. **G-08** — The multiplayer crate (`networking/`, ~1,100 lines, client/server/auth/catchup) is never mentioned anywhere in the 1,210-line export. This is a real, wired-up crate with active code, not dead weight.
9. **G-09** — "Never game over" combined with the reflexive shortage spiral (lines 178–198) has no floor described. The spiral is self-reinforcing with no named recovery mechanism, equilibrium, or player intervention that breaks it. As designed, it is a death spiral with extra steps.
10. **G-10** — The export treats the whole design as an integrated system but never confronts the bootstrap problem: most of these mechanisms (social reproduction, enterprise trust, informal economy, causal inspector, material balance UI) are interdependent. Nothing is playable until a critical mass of them works together. No staging plan, no vertical slice, no minimum viable loop.

## 1. Extracted items

| ID | Statement | Source line(s) | Verdict |
|---|---|---|---|
| G-01 | Ten-phase deterministic schedule exists as architecture conclusion | 41 | WRONG — code has flat serial systems in registration order |
| G-02 | Typed newtypes (Currency, OnHandQty, etc.) exist as architecture | 548–564 | WRONG — none exist in codebase |
| G-03 | CitizenRecord/CitizenBody split as concluded architecture | 38, 454–455, 1104 | WRONG — actual struct is monolithic `HumanEnt` |
| G-04 | Freight-plan utilization numbers (72%/141%/24%/18%) | 219–224 | UNSUPPORTED — no source, no model |
| G-05 | Planning credibility numbers (61%/67%/82%/91%) | 261–265 | UNSUPPORTED — no source, no model |
| G-06 | 145 t request for 100 t true need | 168 | UNSUPPORTED — plausible ratio but no source |
| G-07 | 8,000 workers → 20,000 settlement | 377 | PLAUSIBLE — consistent with Soviet new-town literature (Bater 1980) but unsourced |
| G-08 | 250,000 at 60 fps binding | 34 | CONFIRMED — charter line 56 states this |
| G-09 | "Do not store a hidden dishonest flag" | 48 | CONTRADICTS internal requirement for enterprise intent state |
| G-10 | "Reported need is not true need" requiring per-enterprise intent | 166 | CONFIRMED as design goal; ABSENT in code (ECO-SUB-005) |
| G-11 | Shortage spiral has no floor | 178–198 | UNSUPPORTED — no equilibrium mechanism named |
| G-12 | "Priority cannot solve scarcity" vs National Projects as priority | 208 | CONTRADICTS own proposal at line 49 |
| G-13 | "Money is not a gate" vs Currency newtype and border roubles | 25, 558 | CONTRADICTS — export proposes Currency type while asserting money is not a gate |
| G-14 | Enterprise Director mode vs Planner fantasy | 503–504 | CONTRADICTS — player-as-enterprise contradicts player-as-Planner pillar |
| G-15 | "Stable things sleep" vs "citizens act from remembered/local information" | 1169, 1063–1064 | CONTRADICTS — who updates citizen memory while they sleep? |
| G-16 | "Citizens adapt" via job-seeking vs graduate assignment as administered placement | 423, 74, 1013 | CONTRADICTS — free agent vs administered allocation |
| G-17 | Blat as physical goods movement through social graph | 427–429 | CONTRADICTS "nothing teleports" unless physical custody chain exists |
| G-18 | Salsa for derived/incremental world | 652 | UNSUPPORTED — no codebase presence, unclear fit for mutable sim |
| G-19 | Differential Dataflow for economic analysis | 654 | UNSUPPORTED — research system, no game integration precedent |
| G-20 | Shadow simulation / Gosplan Computer from determinism | 663–665 | PLAUSIBLE but UNSUPPORTED — branching a full sim state is expensive; no cost estimate |
| G-21 | LP/MILP feasibility analysis for the Plan | 669–671 | PLAUSIBLE — well-known technique; no crate evaluated, no perf estimate |
| G-22 | Ten-phase order is parallelizable with intent buffers | 42, 604–612 | WRONG — `ParCommandBuffer` exists but is entity-spawn/despawn only, not an intent-merge system |
| G-23 | "Every aggregate number clickable down to real trains" | 311 | UNSUPPORTED — requires an index structure never costed |
| G-24 | Keyed randomness from seed+domain+entity+event | 617–620 | PLAUSIBLE — standard technique; code uses a single `RandProvider` instead |
| G-25 | EPANET-like water network | 796–806 | PLAUSIBLE as design; no code exists; EPANET is EPA public domain |
| G-26 | Gas linepack as hidden-then-sudden failure | 847–849 | PLAUSIBLE — real physics; charter explicitly cuts gas/pipelines to Post-1.0 |
| G-27 | Bitset society for cohort filtering | 622–624 | PLAUSIBLE — standard ECS technique; no code exists |
| G-28 | Multiplayer crate existence | (never mentioned) | WRONG (by omission) — `networking/` is a live crate (~1,100 LOC) |
| G-29 | Electricity as "near-instant balancing" | 839 | PARTIAL — code has `ElectricityCache` as a connectivity graph (BTreeMap-based network sets), not a power-balance solver |
| G-30 | Cadence bands replace universal tick | 39 | WRONG — code runs all systems every tick at 50 Hz; no band/wake system exists |
| G-31 | Households as shared-pantry units | (implied throughout) | CONFIRMED as ratified spec (SPEC-HOUSEHOLDS-004); not yet implemented |
| G-32 | Ratchet effect as universal mechanism | 251–254 | PLAUSIBLE but overstated — post-1965 reforms weakened the ratchet significantly (Berliner 1976, Gregory & Stuart 2001) |
| G-33 | Private plots as 1–3% of land, ~25–30% of some outputs | 72, 419–421 | CONFIRMED — CIA SOV-85-10126 and Wädekin data; but "some outputs" needs specification (potatoes, vegetables, eggs, milk — not grain or industrial crops) |

## 2. Validation detail

### G-01: Ten-phase schedule

The export (line 41) presents `COMMAND → TOPOLOGY → ALLOCATION → DECISION → ROUTING → MOVEMENT → ARRIVAL → PRODUCTION → UTILITIES → ACCOUNTING` as a "major architecture conclusion from earlier discussion." The actual code in `simulation/src/init.rs:52-109` registers 16 named systems in flat order: `electricity_flow_system`, `dispatch_system`, `update_decision_system`, `company_system`, `pedestrian_decision_system`, `transport_grid_synchronize`, `locomotive_system`, `vehicle_decision_system`, `vehicle_state_update_system`, `routing_changed_system`, `routing_update_system`, `itinerary_update`, `market_update`, `train_reservations_update`, `freight_station`, `random_vehicles`. No phase enum, no phase barrier, no multi-pass structure. The "architecture conclusion" is a target, not a conclusion.

### G-03: Citizen data model

The export repeatedly references `CitizenRecord` (line 38, 454), `CitizenBody` (454), `CitizenCore` (1104), and `CitizenStore` (573) as architectural patterns. The actual citizen is `HumanEnt` (`world.rs:88-105`): `Transform + Speed + Location + Pedestrian + Transporter + Router + Itinerary + HumanDecision + Home + BuyFood + Bought + Work + PersonalInfo`. Every citizen has physics, routing, and rendering data fused. There is no persistent/ephemeral split. The current `PersonalInfo` is `{name: String, age: u8, gender: Gender}` — no household, no qualification, no biography, no expectations, no social ties.

### G-04/G-05: Invented numbers

Lines 219–224 present freight utilization statistics: `Average corridor utilization 72%`, `Peak utilization 141%`, `Emergency dispatch share 24%`, `Empty repositioning 18%`. Lines 261–265 present planning credibility statistics: `Requested inputs usually received: 61%`, `Promised delivery dates met: 67%`, `Emergency requisitions honored: 82%`, `Overfulfillment translated into new quota: 91%`. None cite a source. They are not from CIA documents, Kornai, Berliner, Nove, or any named reference. They read as LLM-generated plausible-looking numbers. The specific danger: if used as calibration targets, they commit the game to a fidelity no model supports. The 141% peak utilization is especially dangerous — it implies a measurement system and a capacity definition that the game does not have.

### G-06: 250k citizen compute budget

The export's citizen model proposes per-citizen state including: biography (birth, education, employment history, household formation, housing queues, relocation, children, death), expectations memory (generational, slowly learned), social graph (small number of durable ties), time budget (sleep, work, commute, shopping, queueing, childcare, domestic work, healthcare, household production, leisure), health state, qualification, current activity, and institutional reliability memory.

Conservative per-citizen estimate:
- Biography/lifecycle: ~128 bytes (compact enums, timestamps)
- Social graph (8 ties × 8 bytes): ~64 bytes
- Time budget (10 slots × 4 bytes): ~40 bytes
- Expectations/memory (4 dimensions × 4 bytes): ~16 bytes
- Health/qualification/location: ~32 bytes
- Household ref, work ref, itinerary ref: ~24 bytes
- Name (interned): ~16 bytes
- **Minimum total: ~320 bytes/citizen**

250,000 × 320 = 80 MB. This fits in RAM but not in L2 (typically 1–16 MB) or L3 (typically 32–96 MB on consumer chips). A single pass over all citizens touches 80 MB of scattered data.

At 60 fps with 50 ticks/second, each tick has 20 ms. A naïve per-citizen pass of 250,000 entities touching 320 bytes each requires sequential reads of ~80 MB. On a modern CPU with ~30 GB/s memory bandwidth, that is ~2.7 ms per full scan — but only if perfectly sequential. With scattered SoA access across multiple arrays, expect 3–8× worse. Multiple systems scanning all citizens per tick would consume the entire frame budget on memory access alone.

The export's answer is "stable things sleep; pressures wake them" (line 1169), but it never defines what fraction of citizens are awake per tick, what the wake cost is, or what the steady-state active set looks like. If 10% are awake, that is 25,000 active entities per tick — plausible but still tight with the proposed per-entity complexity.

### G-09: "Do not store a hidden dishonest flag" vs observable discrepancy

The export (line 48) says: "Do not store a hidden `dishonest` enterprise flag; let the Planner infer strategic behavior from observable discrepancies." This is the core design: the player catches cheating enterprises by noticing that `requested ≠ consumed`.

But the current code (`ECO-SUB-005`) shows that `Market.requested` exists but `set_requested` has no non-test caller. The hoarding scenario manually configures request inflation. No UI exposes requested, received, consumed, reserved, in-transit, or surplus state.

The contradiction: if there is no hidden flag, where does the enterprise's *decision* to inflate live? The export says "reported need is not true need" (line 166) and enterprises "request extra stock" (line 169) for five named reasons. Each of those reasons requires per-enterprise state: reliability memory, plan-risk assessment, reserve targets, bargaining history. That is an enterprise intent model — not a flag, but something far more complex than a flag. The export proposes both "no hidden state" and "rich hidden behavior," which are incompatible unless "hidden" means only "hidden from the UI" rather than "absent from the simulation."

### G-15: "Stable things sleep" vs "citizens act from remembered/local information"

Line 1169: "Stable things sleep; pressures wake them." Line 1063–1064: "Citizens should not know every shop's inventory. They act from remembered/local/social information."

The contradiction: if a citizen is sleeping (not simulated this tick), their memory is frozen. But the world around them changes — a shop restocks, a neighbor gets a delivery, a factory shift changes. When the citizen wakes, their memory is stale. Either:
(a) Memory is updated while sleeping (violating the cost savings of sleeping), or
(b) Memory is updated on wake (requiring a "catch-up" pass that itself costs time and needs to know what changed since last wake — requiring a per-citizen change log or subscription), or
(c) Citizens simply act on stale information (which is historically accurate but needs explicit design — and the export treats citizen knowledge as mostly current).

Option (b) is the only viable path, but it requires exactly the Change Journal infrastructure (line 640) that the export proposes but never costs.

### G-17: Blat and "nothing teleports"

The export (lines 427–429): "Informal access moves actual physical goods and therefore displaces someone else's access." This is correct in principle, but the export also proposes blat operating "through the social graph" (line 73). A social-graph edge is not a physical transport route. For blat to satisfy "nothing teleports," every informal transfer must go through the same physical custody chain as formal allocation — a citizen must physically visit a contact, receive goods at a physical location, and carry them home. The export never describes this physical chain. If blat moves goods through graph edges without physical movement, it teleports goods.

### G-28: Multiplayer crate

The `networking/` crate has ~1,100 lines across 10 source files: authentication (`authent.rs`, 235 lines), client/server architecture, connection management, packet framing, catch-up state synchronization, and world-state replication. The example code shows a working client-server loop. This is a substantial existing subsystem that the export's architecture discussion — which proposes deterministic snapshots, typed system contexts, and parallel compute — never acknowledges. Any architecture change must either preserve or deliberately remove this crate.

### G-32: Ratchet effect as universal

The export (lines 251–254) presents the ratchet as a core, always-on mechanism. Historically, the ratchet was strongest under Stalinist planning (1930s–1950s). The 1965 Kosygin reforms introduced profit-based indicators and tried to weaken the ratchet (Berliner 1976, *The Innovation Decision in Soviet Industry*). Post-1965, enterprises had more formal channels to negotiate targets. By the 1980s, the ratchet was one of several distortions, not the dominant one (Gregory & Stuart, *Russian and Soviet Economic Performance and Structure*, various editions). The export treats it as universal and always strong, which oversimplifies.

## 3. The twelve claims most likely to mislead consolidation

Ranked by damage if copied as-is:

### 1. (G-01) "Deterministic phase order: COMMAND → TOPOLOGY → … → ACCOUNTING"
**Problem:** Implies existing infrastructure. **Corrected:** "Target phase order for a future rewrite. The current schedule is flat serial registration-order systems with no phase barriers."

### 2. (G-03) "Split persistent CitizenRecord from active CitizenBody"
**Problem:** Implies a partially-done architecture. **Corrected:** "The current citizen is a monolithic `HumanEnt` struct with physics, routing, food, and identity fused. A record/body split is a future rewrite target, not an existing pattern."

### 3. (G-04/G-05) Freight utilization and planning credibility numbers
**Problem:** Look like calibration targets or empirical data. **Corrected:** "Illustrative numbers with no source. Do not use as calibration targets."

### 4. (G-09) "Do not store a hidden dishonest flag; let the Planner infer from observable discrepancies"
**Problem:** Sounds like a solved design; actually an unsolved problem. **Corrected:** "Enterprises need per-entity intent state (reliability memory, reserve targets, bargaining history) that drives inflation. The *Planner's view* should not expose this directly, but the simulation must model it. The current code has no non-test inflation path."

### 5. (G-22) "Parallel compute → intent buffers → deterministic merge → stable sort → commit"
**Problem:** Implies `ParCommandBuffer` is this system. **Corrected:** "`ParCommandBuffer` handles entity spawn/despawn only. No intent-buffer-merge system exists. This is a future architecture pattern."

### 6. (G-06) 250k citizens with full social/biographical state at 60 fps
**Problem:** No arithmetic to support feasibility. **Corrected:** "250k identities at 60 fps is a charter commitment. The proposed per-citizen state budget (~320+ bytes minimum) requires SoA layout, cadence-band sleeping, and <10% active fraction per tick to fit in the frame budget. None of these systems exist. The 250k target needs a benchmark gate and staged implementation."

### 7. (G-11) Shortage spiral without equilibrium
**Problem:** As described, the spiral has no floor. **Corrected:** "The shortage spiral is self-reinforcing on paper. In practice, physical limits (finite enterprises, finite goods, floor-zero inventories) provide a trivial floor, but that floor is economic collapse. A designed equilibrium — where reduced output eventually reduces demand pressure — must be explicitly modeled, or the game plays as a death spiral."

### 8. (G-30) "Cadence bands rather than one universal tick frequency"
**Problem:** Implies an existing system. **Corrected:** "All systems run at 50 Hz on every tick. Cadence bands are a future optimization target."

### 9. (G-14) Enterprise Director mode
**Problem:** Contradicts the player-as-Planner pillar. **Corrected:** "Enterprise Director mode is an interesting design idea for a future expansion/scenario. It requires a separate camera, information model, and goal structure. It cannot share the same Planner UI or information contract."

### 10. (G-23) "Every aggregate number clickable down to real trains"
**Problem:** Requires a drill-down index from every macro stat to every physical entity — an unbounded UI and data-structure commitment. **Corrected:** "Material-balance aggregates should link to representative physical examples. Full drill-down to every entity requires an index whose cost scales with entity count × stat count and must be budgeted."

### 11. (G-17) Blat through the social graph
**Problem:** Risks teleportation. **Corrected:** "Informal exchange must use the same physical custody chain as formal allocation. A citizen requesting goods through blat must physically visit a contact, receive goods at a building, and transport them home. The social graph identifies *who* to ask, not a transport mechanism."

### 12. (G-18/G-19) Salsa and Differential Dataflow
**Problem:** Names real projects in a way that implies fit. **Corrected:** "Salsa is an incremental computation framework designed for compiler queries over immutable inputs; adapting it for a mutable physical simulation is unproven. Differential Dataflow is a research streaming-dataflow system with no game-engine integration precedent. Evaluate concrete alternatives (e.g., custom change-journal + derived queries) before committing to either."

## 4. What the export gets right that the repo docs get wrong

### 4a. "Planning is not a layer on top of logistics. Planning is one of the forces that deforms logistics." (line 127)
The repo's specifications treat the economy, logistics, and citizens as separate domains with clean interfaces. The export correctly identifies that plan pressure creates emergent cross-domain effects (storming → overtime → fatigue → turnover → production shortfall) that the specifications do not model. The specifications are correct about their boundaries but silent about the feedback loops that cross them.

### 4b. The four-reality model (lines 1049–1058)
The repo has no concept of information asymmetry. The export's model — actual physical reality, reported institutional reality, Planner knowledge, household lived experience — is a genuine design insight not captured in any ratified specification. `SPEC-CITIZENS-002` says citizens reference modules; it does not say citizens have imperfect information about those modules.

### 4c. Enterprise as miniature welfare state (lines 358–371)
The repo's specifications treat enterprises as production units with workers. The export correctly identifies that Soviet enterprises provided housing, childcare, canteens, clinics, and cultural facilities — making the enterprise a welfare institution, not just a production function. This is well-documented (Filtzer 1994, *Soviet Workers and De-Stalinization*) and has real gameplay implications: closing a factory affects not just production but the social infrastructure of its settlement.

### 4d. Housing as labor infrastructure (line 389)
The repo's household specification treats housing as a residence assignment. The export correctly frames housing as a labor-recruitment instrument — enterprises compete for workers through housing allocation. This is historically accurate (Bater 1980, *The Soviet City*) and adds a genuine planning mechanic missing from the specifications.

### 4e. Time as a citizen resource (lines 397–410)
The repo's needs specification uses satisfaction states (met/unmet/going-without). The export's model of citizen time as a finite, observable, measurable resource — where shortage creates time costs even when goods are eventually obtained — is a stronger design that makes queuing legible as a gameplay surface rather than an invisible penalty.

### 4f. The core player fantasy statement (lines 1205–1207)
"Turn a fragile, shortage-prone, buffer-hoarding industrial system into a calm, predictable, sophisticated republic capable of executing immense national projects without tearing ordinary society apart." This is the clearest statement of the player fantasy in any project document. The charter identifies the player as the Planner but does not describe the emotional arc of play.

## 5. Cross-lane hooks

- **Lane A (economy):** G-04/G-05 invented numbers must not be treated as calibration targets. G-09 enterprise intent model is unsolved. G-11 shortage spiral needs an equilibrium mechanism. G-12 priority vs scarcity is a real tension in the design.
- **Lane B1 (society):** G-06 citizen compute budget is the binding constraint on everything B1 proposes. G-15 sleep/wake vs memory staleness needs explicit design. G-16 adaptation vs administered placement is a fundamental tension.
- **Lane B2 (CIA):** G-04/G-05 numbers are not from CIA sources. G-32 ratchet needs historical periodization. G-33 private-plot outputs need commodity-level specificity.
- **Lane C1 (crates):** G-18/G-19 Salsa and Differential Dataflow need crate evaluation, not endorsement. G-07 proposed types do not exist.
- **Lane C2 (architecture):** G-01 phase order is aspirational. G-02 typed newtypes are aspirational. G-03 citizen split is aspirational. G-22 intent buffers are not what `ParCommandBuffer` does. G-28 networking crate must be acknowledged.
- **Lane D (physics):** G-25/G-26 EPANET/gas systems are Post-1.0. G-29 electricity is connectivity, not power balance.
- **Lane E (code audit):** G-28 networking crate. G-30 cadence bands do not exist. ECO-SUB-005 is the critical gap for the core loop.
- **Lane F (doc overlap):** G-01/G-02/G-03/G-30 — the export presents aspirational architecture as conclusions. Every such claim needs a label.

## 6. Open questions for the user

1. **Enterprise intent model:** The core loop requires enterprises to have reasons to inflate requests. "No hidden flag" is not "no hidden state." What is the actual design for enterprise decision-making — a rule-based heuristic, a learned behavior, or something else?
2. **Shortage spiral equilibrium:** What prevents the reflexive shortage spiral from converging on total collapse? Is the player the only equilibrium mechanism, or should the simulation have natural damping?
3. **Multiplayer intent:** The networking crate is substantial. Is multiplayer a future direction, or should the crate be removed to reduce maintenance surface?
4. **Enterprise Director mode:** Is this a serious future direction? If so, it needs its own design document because it contradicts the Planner information model.
5. **Wake fraction:** For 250k citizens, what steady-state fraction should be active per tick? This number determines whether the architecture is feasible.

## 7. Sources

### Codebase files
- `simulation/src/init.rs:52-109` — system registration order
- `simulation/src/world.rs:26-105` — entity types, `HumanEnt` struct
- `simulation/src/souls/human.rs:21-80` — `HumanDecision`, `PersonalInfo`
- `simulation/src/economy/market.rs` — market matching, dispatch
- `simulation/src/map/electricity_cache.rs:53-63` — `ElectricityCache` structure
- `networking/src/` — multiplayer crate (~1,100 LOC)
- `docs/plan/charter-1.0.md:56` — 250k commitment
- `docs/reference/architecture/substrate.md` — substrate classifications
- `docs/research/fact-sheets/wave1-economy.md` — ECO-SUB-001 through ECO-SUB-006
- `docs/research/fact-sheets/wave1-logistics.md` — LOG-SUB-001 through LOG-SUB-009
- `docs/reference/specifications/households.md` — SPEC-HOUSEHOLDS-001 through -008
- `docs/reference/specifications/citizens.md` — SPEC-CITIZENS-001 through -007

### External references
- Berliner, J. (1976). *The Innovation Decision in Soviet Industry*. MIT Press. — ratchet effect post-1965
- Gregory, P. & Stuart, R. (various editions). *Russian and Soviet Economic Performance and Structure*. — ratchet weakening
- Bater, J. (1980). *The Soviet City*. Edward Arnold. — new-town settlement ratios, housing as labor infrastructure
- Filtzer, D. (1994). *Soviet Workers and De-Stalinization*. Cambridge University Press. — enterprise welfare
- Kornai, J. (1980). *Economics of Shortage*. North-Holland. — shortage spiral dynamics, soft budget constraint
- Ledeneva, A. (1998). *Russia's Economy of Favours*. Cambridge University Press. — blat as physical exchange
- Nove, A. (1977, rev. 1992). *The Soviet Economic System*. Routledge. — planning credibility, reporting distortions
- Wädekin, K.-E. (1973). *The Private Sector in Soviet Agriculture*. University of California Press. — private plot outputs
- CIA SOV-85-10126 — private agricultural production
