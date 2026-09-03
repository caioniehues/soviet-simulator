# Soviet Simulator — design bible

**Kind:** explanation
**Authority:** explanatory — this document binds nothing. The charter binds scope, the glossary binds terms, ratified specifications bind mechanism, accepted decisions bind architecture, `bd` binds task state, and source code and tests establish what exists. Fact-sheets provide evidence. Where this document and a higher authority disagree, the authority wins.
**Status:** draft
**Owner:** project lead
**Verified-at:** `4e9e930b2a73`
**Last verified:** 2026-09-03
**Provenance:** curated from the GPT design thread (`docs/archive/raw-sessions/gpt-master-bible-2026-08-28.md`, `gpt-vision-export-2026-08-28.md`) after ten-lane validation and the lead's reconciliation (`docs/research/conversation-mining-2026-08-28/SYNTHESIS.md`). Every section carries its evidence label and its current substrate status.

**How to read the labels.** `CONFIRMED` — a cited source or the code proves it. `PLAUSIBLE` — consistent with evidence, unproven. `HYPOTHESIS` — design proposal with no historical or technical ratification. `SPEC` — already written into a draft specification (all 22 are `Status: draft`). `CODE` — exists in the committed tree at the cited line. `ABSENT` — no substrate. Anything marked **Post-1.0** is direction, not a 1.0 requirement (charter §Explicit cuts).

---

## 1. Thesis

The subject of this game is **coordination under physical scarcity**. The player is THE PLANNER. The Planner does not buy domestic goods through a price-clearing market; the Planner sets quotas, priorities, allocation policies, construction programmes, reserves and institutional rules, and then real physical and institutional actors must produce, store, load, move, unload, deliver and consume.

```text
PLAN
  ↓ quotas / priorities / policies
enterprises and institutions adapt
  ↓ requests / buffers / labour decisions / dispatch pressure
physical production and logistics
  ↓ queues / shortages / surplus / delay / quality / congestion
household and workplace experience
  ↓ reports / complaints / institutional information
PLANNER KNOWLEDGE
  ↓
next PLAN
```

The planned economy generates its own gameplay. No crisis dice are needed: a taut plan, a bad reserve policy, an overconfident rail programme, a housing lag, a storming cycle or an unreliable allocation system is enough.

Two rules govern everything below.

> **Use the cheapest representation that preserves every causal distinction the game cares about.**

> **Every important macroeconomic number must eventually resolve into physical or institutional state.**

The player fantasy (the clearest statement in any project document, per Lane G):

> Turn a fragile, shortage-prone, buffer-hoarding industrial system into a calm, predictable, sophisticated republic capable of executing immense national projects without tearing ordinary society apart.

Success is **coordination quality**, not gigantism: smaller emergency reserves, fewer emergency dispatches, shorter queues, lower plan-period variance, more reliable deliveries, lower turnover, less overtime, more accurate reports, more household discretionary time.

## 2. Design laws

Laws 1–3, 5 and the never-game-over rule are charter pillars (`charter-1.0.md:24-35`). The rest are new to the corpus from the design thread and are recorded as principles, not binding scope.

### Physical causality
1. **Goods move physically or do not move.** Allocation, matching, payment, route creation and reservation never teleport stock. *(charter)*
2. **Request, allocation, reservation, pickup, custody, delivery, on-hand and consumption are separate states.** *(SPEC-PRODUCTION-003; glossary "Request")*
3. **Failure persists.** A missing stock, vehicle, route, dock, worker, watt, litre, dwelling, school place or clinic slot creates a visible waiting, partial, stalled, substitution or going-without state. It never ends the game. *(charter)*
4. **No silent deletion.** Goods, demand, citizens, vehicles, queues and sites do not vanish because a transaction failed. *(new; the code violates it — see §4 of the synthesis: `ECO-SUB-001`)*
5. **No domestic price clearing.** Scarcity resolves by policy, queue, priority, substitution, rationing, reserve, adaptation or going without. The rouble is border foreign currency only. *(charter:32-33)*
6. **Physical opportunity cost must be visible.** Prioritising one use removes actual capacity, materials, labour, transport, housing or service access from another. *(new)*

### Information causality
7. **Reports are not truth.** Reported demand, plan fulfilment, institutional reports and citizen knowledge are distinct from physical state. *(new; the four-realities model, §4)*
8. **Information is a resource.** Better reporting, monitoring, representation and reliable institutions improve planning quality without magically improving supply. *(new)*
9. **No omniscient player UI.** Player-facing data comes through Planner-visible snapshots and institutional observation, never unrestricted access to `Simulation`. *(new; CODE: the UI holds `Arc<RwLock<Simulation>>` and reads ~40 resources directly — `native_app/src/game_loop.rs:33`)*
10. **No hidden honesty flag.** Strategic behaviour is inferred from discrepancies: request, receipt, consumption, on-hand, surplus, queue age, declared capacity, physical output. *(SPEC-PRODUCTION-009 verbatim; CODE: no `dishonest` identifier exists.)* **Clarification from Lane G:** "no hidden *verdict*" is not "no hidden *state*". An enterprise that inflates needs reasons — remembered reliability, reserve targets, bargaining history — and those live in the simulation, hidden from the Planner's view, not absent. The Planner sees the discrepancy, never the reason.

### Social causality
11. **Citizens persist as identities.** Embodiment may be bounded; the record is not disposable. *(SPEC-CITIZENS-001)*
12. **Households are first-class actors.** Residence, pantry, care obligations, housing queues, adaptation, family history. *(SPEC-HOUSEHOLDS-004; ABSENT in code — `grep -r Household simulation/` is empty)*
13. **Social reproduction is physical.** Workers must be housed, fed, heated, educated, transported, kept healthy and given time. *(CONFIRMED — Zaslavskaya; Lane B1)*
14. **Citizens adapt.** They search, queue, substitute, buffer, reschedule, use plots, use contacts, relocate, change jobs or go without. *(CONFIRMED — CIA 1979, 1982; Lane B2)*
15. **No single happiness scalar.** Preserve causes: queue burden, crowding, warmth, health, time pressure, access, commute, career, household reliability. *(new)*

### Technical causality
16. **Stable things sleep; pressure wakes them.** *(HYPOTHESIS; the architecture path to 250k)*
17. **Compute → deterministic merge → commit.** Parallel workers calculate intents; only ordered commits mutate truth. *(HYPOTHESIS; CODE: `ParCommandBuffer` is an intent buffer but systems run serially)*
18. **One authority per state transition.** Cross-domain code references IDs and results; it does not mutate another domain's ledger. *(pattern already in the utility specs)*
19. **Every replayable transaction is idempotent** under an immutable ID. *(SPEC-WATER-006, SPEC-ELECTRICITY-002 already require this)*
20. **No generalised abstraction before shared invariants are proven.** Share topology, scheduling, IDs, journals; do not force water, power, traffic, sewage, heat and gas through one solver. *(new)*

## 3. Evidence standard

Historical research discovers **mechanisms**, never an ideological verdict. Evidence classes, strongest first: project binding evidence; current-substrate evidence; primary/archival (declassified reports, translated Soviet material, statutes, enterprise reports, technical manuals); serious secondary research; comparison evidence (other games, engineering software); design hypothesis.

Cautions, all confirmed by Lane B2 §4: the CIA record is heterogeneous (an analytic assessment, a translated Soviet article and an emigre anecdote are not equal); it is Moscow/Leningrad-heavy and emigre-biased; it was written to find weakness; the 1960s — the decade of greatest domestic success — is thinly covered. Formal Soviet law describes institutional design, not lived power. Late-Soviet behaviour must not be projected onto the 1930s–60s. Yugoslav self-management, Hungarian reform, Polish workplace politics and Soviet ministries are different institutions.

**The game must be able to show excellent coordination as well as failure.** The baseline is "functional but constrained" (jobs, small flats, monotonous food, basic healthcare, good schools, adequate transit), not perpetual emergency. Crises emerge from plan failures. *(B2 §4.2)*

**Number hygiene.** The design thread once carried illustrative tables (72 %/141 %/24 %/18 % corridor utilisation; 61 %/67 %/82 %/91 % credibility). They have no source (Lanes A, G) and were removed. Use Lane B2's calibration table (`B2-cia-sources.md` §3) — 27 parameters with document IDs — as the only numeric baseline, and treat 1954 numbers as bounds, not constants.

## 4. The four realities

| Reality | Holds | Example |
|---|---|---|
| **Physical** | stocks, custody, citizens, households, buildings, sites, routes, queues, runs, utility flows, actual attendance and illness | 35 t of copper sitting in a plant's yard |
| **Institutional** | what organisations declare, request, record or believe | the plant's reported requirement of 140 t |
| **Planner** | what reaches THE PLANNER through reports, dashboards, inspection, measurement | "requested 140, received 126, discrepancy flagged" |
| **Lived** | what people experience: search, queues, warmth, commute, crowding, childcare, health access, fatigue, workplace pressure, informal access, opportunity | 33 h/week to fill a food basket (CIA, 1954 Moscow) |

A formal Plan can report success while lived conditions deteriorate; plots, blat, local adaptation and enterprise welfare can make life tolerable while the metrics look weak. **Enforce this in code**: the normal UI consumes a `PlannerSnapshot`, and every value in it says *how it is known* — measured, reported, aggregated, observed via an institution, estimated, or unknown. *(ABSENT; see `docs/plan/proposals/causal-inspector.md`.)*

Institutional extension (bible §8.12, HYPOTHESIS): compare physical reality, lived experience, enterprise report, union report, local-Soviet report and Planner belief as six channels each with a measurable **representation error**. Political institutions become a sensor network, not a popularity minigame.

## 5. The planned economy as a control system

### 5.1 Requests are strategic statements — CONFIRMED (Kornai 1980; Berliner 1957)
An enterprise consuming 100 t may report 135 t for: expected allocation cuts, unreliable delivery, future plan risk, safety stock, maintenance reserve, expected storming, fear of shortage. Distinguish: technical forecast · reported requirement · allocated · reserved · received · consumed · surplus/on-hand · outstanding request age.

**CODE:** the seed exists and is wired end-to-end — `request_multiplier` (`prototypes/src/types/recipe.rs:52`; `flour-factory` 4, `slaughterhouse` 3 in `base_mod/companies.lua:40,582`) → `recipe_init` → `market.set_requested` (`souls/goods_company.rs:22-26`); proven by `SCENARIO-0151` and `sov-lpj`. It is static: no reliability memory in `GoodsCompanyState`. **The Planner cannot see it**: `Market::requested()` is public and unread by `native_app/`. Wiring it into `inspect_building.rs` is ~30 lines and is the single cheapest high-value change in the project (Lane E §3).

### 5.2 Reliability → defensive buffering — CONFIRMED (Kornai)
```text
unreliable delivery → actors raise buffers/requests → central availability falls
→ others face shortage → more buffering → dispatch pressure → delivery less reliable
```
Applies to enterprise stock, hospital medicine, household pantries, wagon reserves, construction-site materials, informal networks. The positive spiral is equally real: reliable delivery → buffers shrink → stock released → emergency dispatch falls → congestion falls → reliability improves. **A mature republic is physically calmer.**

### 5.3 The spiral has a physical floor — CONFIRMED, CODE (lead's finding)
`recipe_should_produce` refuses to buy above `amount × (storage_multiplier + 1)` (`goods_company.rs:45-46`). **An enterprise cannot hoard what it cannot store.** Lane G's "death spiral with no floor" is bounded by warehouse capacity. Consequence: storage construction is a Planner-visible hoarding signal — an enterprise that keeps enlarging its warehouse while reporting shortage is telling on itself. Design the reserve mechanics (5.7) so that hidden reserves compete for the same finite storage as operating stock.

### 5.4 Ratchet — CONFIRMED (Weitzman 1980; Berliner)
Heroic overfulfilment → next quota raised → revealed slack becomes obligation → enterprises conceal capacity. A Planner who converts every surge into the next baseline trains enterprises to lie. Era note (Lane G-32): strongest under 1930s–50s planning, weakened after 1965 — strong in this game's fixed era. **Mechanism sketch (Lane A §3b):** per enterprise per period `(quota, actual)` in a ring buffer; auto-quota `max(quota, actual) × growth`; the staircase is visible in a timeline; the Planner can override below the ratcheted level to rebuild trust at the cost of output. Test: A overfulfils twice, B produces at quota; A's quota ends strictly higher though true capacity is equal.

### 5.5 Storming / shturmovshchina — CONFIRMED (three-phase monthly cycle: spyachka, goryachka, likhoradka)
Late inputs, taut quotas or delayed effort concentrate output near period end. Storming generates freight pulses, overtime and fatigue, rail congestion, dock queues, quality/rework risk, household time pressure and distorted reports; it propagates **upstream recursively** (space → electronics → wire → copper → mine → rail). **Sketch (A §3c):** `storming_state`, `storming_multiplier` up to 1.5× productivity *and* input draw when `period_remaining/period < threshold` and `output/target < shortfall`. Test: the downstream enterprise, previously adequately supplied, lags on its own quota after the upstream spike. **CODE:** production is continuous; no plan period exists anywhere (`Government` holds only `money`, `government.rs:9-11`).

### 5.6 Freight-plan stability — PLAUSIBLE
Two plans with equal annual tonnage need different fleets if one is smooth and one is pulsed. Track mean corridor load, peak load, period variance, emergency-dispatch share, empty repositioning, dock waiting, missed loading windows — as metrics, without invented values. The right fix may be the Plan, not more track.

### 5.7 Reserves — PLAUSIBLE taxonomy (the five classes are the thread's invention; Soviet practice had state reserve, enterprise buffer stock, operational inventory)
Operating · safety · enterprise (hidden surplus) · state · project. Rules (A §3e): the five sum to physical stock; consumption draws operating then safety (with a recorded event); enterprise reserve is never drawn automatically — it *is* the hoard; state reserve moves only by Planner action; project reserve only by a national-project system. The Planner can *compute* the hidden reserve from physical stock minus the four declared classes if they inspect closely; the enterprise's report omits it. That is `SPEC-PRODUCTION-009` made concrete. Open: five classes may be too many for the player; two or three may carry the loop.

### 5.8 Priority does not solve scarcity — CONFIRMED (Kornai; Nove)
Priority decides **where** scarcity appears. Copper to the Space Programme is copper not in radios, machine tools or construction. The Planner must see the displaced use. **Priority inflation (HYPOTHESIS):** if everyone may label a request critical, priority means nothing; constrain who assigns classes and expose the share of activity running under emergency status.

### 5.9 Taut versus resilient — CONFIRMED
99 % nominal with two days of reserve can realise less than 88 % with twelve days. Slack is physical resilience, not inefficiency.

### 5.10 Specialisation versus self-supply — PLAUSIBLE
A locomotive works with unreliable bearing deliveries builds an inefficient bearing shop because reliability has value. Centralised specialisation, local redundancy, standardisation, reserve stock, transport investment are real choices.

### 5.11 Indicator design — CONFIRMED (Nove)
What the Plan measures becomes what enterprises optimise: tonnes → heavy goods; units → simple variants; fulfilment % → storming; rail tonnage → awkward consignments neglected. Introduce an indicator only when its physical consequence is represented.

### 5.12 Planning credibility — concept CONFIRMED (Gregory & Harrison 2005); numbers must be earned in play
Track practical reliability, not a trust score: requested inputs delivered on time; promised allocations honoured; emergency reallocations; mid-period quota changes; reported capacity later ratcheted; complaints acted upon. **Sketch (A §3d):** global `credibility` EMA in a `PlanningAuthority` struct; per-enterprise `trust_in_plan` blended from global credibility and own `reliability_memory`; low trust raises `effective_multiplier`, lowers reporting accuracy, raises the propensity for local workshops. Test: three confiscations measurably drop credibility and raise subsequent requests from the confiscated enterprises.

### 5.13 Mechanisms the thread missed — all CONFIRMED historically (Lane A §4), all ABSENT, all Post-1.0 unless noted
- **Tolkachi / expediters** (Berliner; from 1937): a worker leaves production to physically chase inputs through personal contacts — a second allocation topology and a visible cost. Redistributes access; never creates goods.
- **Ministries as inflating aggregators**: real planning was Gosplan → ministry → enterprise; the "dishonest ministry" aggregates and is harder to catch.
- **Soft budget constraint, physical form** (Kornai): never-game-over means the Planner rescues failing enterprises by taking from performing ones — the reliable are punished. Make the tension explicit.
- **Plan-fulfilment falsification** (Harrison 2011; CIA grain overstatement up to 53 %): reported output is itself a report. The inspector compares reported production against physical stock change.
- **Assortment plans**: aggregate quota met with the easy mix. Quotas need mix as well as volume.
- **Plan correction cycle**: every mid-period revision disrupts logistics and degrades credibility.
- **Investment hunger** (Kornai): enterprises request buildings they cannot staff — a distinct dishonest pattern; the physical form of capital dilution.
- **Forced-substitution chains**: A missing → substitute B → B's allottee substitutes C. Substitution is an explicit action with a penalty and a traceable chain.
- **OTK quality attestation**: a quality gate makes storming costly through rework that consumes more inputs.

### 5.14 Material balance — CONFIRMED (equivalent to the Gosplan identity)
```text
opening + production + arrivals − consumption − departures = closing
```
Per resource: physical stock (producer, operating, safety/reserve, state/project, in transit); period flow (produced, imported, consumed, allocated-not-delivered); information (technical forecast, reported demand, outstanding request age, suspected discrepancy). Every line drills to holders, hauls, consumers or reports. A non-zero residual means something teleported — a `ledger-invariant-checker` bug, and a visible one. **CODE:** `EcoStats` records trade volumes only; production, consumption and stock levels are not tracked (A §3f).

## 6. Resources, production, logistics, construction

- **Granularity rule** (CONFIRMED heuristic — Gosplan balanced ~1,943 categories, not SKUs): split only when handling, storage, carrier compatibility, substitution, priority, routing, quality/acceptance, a physical bottleneck or a strategic decision changes. The 1.0 catalogue is fifteen domestic resources plus import-only Medicine; Water is a utility, never cargo (charter). **CODE:** 21 items with `id`, `label`, `optout_exttrade` only — no mass, volume, storage or transport class (`prototypes/src/prototypes/item.rs:6-25`, `base_mod/items.lua`).
- **Stock semantics:** on hand · reserved (an encumbrance, not additive) · in custody · embedded in construction · consumed. **CODE:** `capital`, `reserved`, `requested` in `SingleMarket`; no custody quantity on vehicles (`LOG-SUB-005`); no embedding — construction is instant.
- **One logistics authority** owns the haul: demand referenced → allocated → vehicle reserved → route to source → pickup → custody → route to destination → delivery → release/recovery. Consumption stays outside the haul. **CODE:** the truck leg is real and well-tested (`DispatchState` ToSource → Loading → ToDestination → Unloading; 13 ledger tests, 14 retail tests — the code's actual strength). Bounded return recovery is implemented. Source-wait recovery is worktree-only and requires committed verification. Missing custody on the vehicle and the **export side** (`market.rs:774` still debits at match time — a pillar violation).
- **Custody conservation:** pickup of `x` is `H_source −= x; R_source −= x; C_haul += x`. Post-pickup cancellation cannot "release"; cargo stays in custody until return, reassignment or delivery.
- **Finite loading/unloading** (SPEC-LOGISTICS-011; Lane D: "the real Soviet bottleneck"): docks have compatibility, occupancy, rate, power and space. A 12 t truck at a 2 t/min dock takes six minutes; partial transfer moves custody by the transferred amount only. **CODE:** cargo is a counter (`freight_station.rs:139`); no transfer time.
- **Target-stock dispatch priority** (SPEC-LOGISTICS-005): largest normalised deficit → meaningful route distance → stable ID tie-break. No domestic price participates. **CODE:** `make_trades` sorts by distance only.
- **Deadhead metrics:** loaded distance, empty distance, loading wait, unloading wait, road/rail wait, recovery time, utilisation — fleet planning becomes logistics rather than vehicle count.
- **Handling classes (HYPOTHESIS):** bulk mineral, aggregate, grain/food bulk, pallet/general, machinery/project, medicine/special, waste. Affects storage, dock, wagon/truck compatibility, transfer rate, visuals — without exploding the catalogue. Needs the item metadata the code lacks.
- **Production bound:** `run = min(recipe, delivered input, labour, power, water, process, output space)`, and **always record the binding constraint** (glossary term). Runs keyed by immutable `ProductionRunId`; inputs debited and outputs credited atomically; replay applies once. **CODE:** `recipe_should_produce` gates on input, storage cap and workforce; which gate bound is not recorded.
- **Surplus stays visible** (SPEC-PRODUCTION-003): requested 140, consumed 100 → 40 on hand, never deleted to tidy a report.
- **Construction** (SPEC-CONSTRUCTION; glossary): Ghost → Verdict → Site → material/work gates → ground broken → completion → activation. Elapsed time never completes a gate. Deeper phases (earthworks, foundations, structure, utilities, finishing, commissioning) only if each creates distinct materials, crews, access or decisions. Construction offices, depots and crews stay physical; repetitive clicking is removed by jurisdiction, templates and dispatch policy — **automate execution, not decisions.** **CODE:** placement is instant with a rouble cost (`world_command.rs:225`) — a pillar violation; roads auto-generate lots (`map/map.rs:682-720`), which `SPEC-ZONING-003` forbids.

## 7. Citizens and households

All of §7 is **ABSENT in code**: `HumanEnt` is one monolithic struct (`world.rs:87-105`); `PersonalInfo` is `{name, age, gender}`; age never increments; one need (bread); `Home` is a bare `BuildingID`; no household. The household entity is the first thing to build.

### 7.1 Layers — HYPOTHESIS
L0 `CitizenRecord` (biography, membership, core state) · L1 scheduled social agent (next event) · L2 active activity/trip · L3 `CitizenBody` (movement) · L4 render instance (bounded visible citizens, per charter). A citizen moves between layers without losing identity. Design detail: `docs/plan/proposals/citizen-architecture.md`.

### 7.2 Household first — SPEC-HOUSEHOLDS-004
A household owns membership, residence or queue position, the Food and Meat pantry (point of use), care obligations, adaptation state, family history. Households decide consumption and care; members do not duplicate it.

### 7.3 Time is a resource — CONFIRMED (Gordon & Klopov 1972; Szalai; CIA 1955: 33.65 h/week for a 1954 Moscow food basket; women 28 h/week domestic vs men 12)
A day divides among sleep, work, commute, shopping/search, queues, childcare, other care, domestic work, healthcare, education, plot work, leisure, informal activity. A shortage costs time even when the good is eventually obtained. **Conservation law (B1 §3a):** per household, `total_hours = Σ committed + discretionary`, never violated; one `u16` add per household per cadence tick — trivial at 100k households. The **social-reproduction balance** per district is the same identity summed: potential adult hours − employment − commuting − care − shopping/queues − household production − illness = discretionary. A stacked bar per district shows which sink is eating time; every social investment is measurable in recovered hours.

### 7.4 Scheduling — HYPOTHESIS
Not every citizen every tick. A day is a small schedule (07:10 depart, 07:48 arrive, 16:00 shift ends, 16:40 shop, 17:25 home, 23:00 sleep); only a relevant event or invalidation wakes the citizen. Household reviews: pantry threshold, shopping responsibility, childcare assignment, plot work, housing reconsideration, lifecycle event, relocation. **Lane G-15 tension resolved:** a sleeping citizen's memory goes stale; on wake it is refreshed from the change journal (§13), not by scanning the world.

### 7.5 Knowledge and search — PLAUSIBLE
Citizens hold a few beliefs ("Store A usually reliable, last success two days ago; Store B — friend reported meat today"), at most 3–5 shops. Learning: direct visit; neighbourhood observation of a delivery (physical presence required); social transmission with decay; **rumour** as a lossy, delayed, sometimes wrong channel (B1-MISSED-10). A delivery becomes a crowd without scripting: arrival → observers → household → contacts → travel → queue → depletion.

### 7.6 Need satisfaction — SPEC-NEEDS-003/004 extended
Preferred available → acquire; not available → approved substitute; still not → household reserve / informal route / plot; still not → going without. Food and Meat are separate 1.0 dwelling needs (charter). Only authoritative consumption after physical receipt satisfies.

### 7.7 Plots and dachas — CONFIRMED (3 % of land; 64 % potatoes, 43 % vegetables, 40 % meat, 39 % milk, 66 % eggs in 1966 — CIA DOC_0000496622; Wädekin 1973) — **Post-1.0**
Household buffer from land, time, seed/feed/tools, season, transport; depends on state-sector inputs; food must be physically grown.

### 7.8 Blat — CONFIRMED (Ledeneva 1998: zero-sum for goods, sparse, reciprocal, invisible to the state) — **Post-1.0**
An alternative allocation topology. A favour moves a real unit from a real holder; the next citizen in the formal queue goes without. **Physical chain (B1 §3c, answering Lane G-17):** the contact must have physical access at a building with stock; the debit is the same retail debit; the requester travels. The social graph says *whom to ask*, never *how goods move*. Degree-bounded (`MAX_TIES` 5–8; ~24 MB at 250k). The Planner sees aggregate anomalies (depletion faster than throughput), never individual favours. Also: **komandirovki** — a business trip to Moscow returns with deficit goods and fresh shop knowledge.

### 7.9 Non-monetary inequality — CONFIRMED (Zaslavskaya; CIA 1982)
Workplace, housing allocation, geography (Moscow ≫ province ≫ small town ≫ rural — B2 §4.2), service access, contacts, scarce qualifications, institutional privilege (closed distributors, clinics, sanatoria). Model access channels, not a class variable.

### 7.10 Expectations and cohorts — PLAUSIBLE (CIA 1982 on youth aspirations)
Slow EMA per citizen for housing standard, food reliability, mobility, leisure; young citizens learn faster. Generations differ without personality AI; the **fertility echo** (a generation with low expectations has children despite crowding) follows. Births remain an open question in `citizens.md:117`.

### 7.11 Housing — CONFIRMED (Andrusz 1984; Morton 1980; 12–36 % of families on lists, 10+ years in Moscow; three channels: enterprise ~2 yr, municipal 10+ yr, cooperative)
A persistent non-price queue keyed by (channel, priority, registration tick); eligibility below the sanitary norm (~9 m²/person); displacement re-enters with a bonus and the original tick. **Two tiers** (B1-MISSED-06): kommunalka/dormitory → separate flat was *the* quality-of-life transition (up to 80 % of Moscow communal until the mid-1960s). Housing shortage retains the household.

### 7.12 Housing as labour infrastructure — CONFIRMED (Bater 1980; Feshbach: housing the primary cause of tekuchest'; six CIA housing reports)
Enterprises recruit and retain through housing, dormitories, commute, childcare, canteen, provision. A plant of 8,000 needs a settlement of ~20,000 (PLAUSIBLE — 2.5× is high but in range). **Mikrorayon completeness:** a numerically complete district fails if schools, clinics, shops, childcare, heating, transit or pedestrian access lag; never one service-radius aura when capacity and travel can be represented.

### 7.13 Institutions of daily life the thread missed — CONFIRMED (Lane B1 §4), all Post-1.0 unless the household spec absorbs them
- **Propiska**: residence registration was *the* gate for housing-queue entry, permanent employment and services; a natural Planner lever over migration and labour supply.
- **Limitchiki**: temporary-registration recruits in dormitories — fast labour, restricted access, high turnover, a visible underclass.
- **Trade unions as allocators**: enterprise housing lists and sanatoria vouchers (putyovki, 20 % free).
- **Kitchen vs canteen**: the stolovaya converts enterprise food into recovered household time; when it fails the burden returns home.
- **Pensioners as queue labour**: a household with a pensioner queues during work hours.
- **Two-shift schools** (08:30–14:30; 15:30–19:30): supervision burden depends on which shift each child has.
- **Alcohol**: the largest single non-structural drag on labour (CIA 1986: ~10 % of national income; 37 % of the male workforce chronically drunk in 1982; absenteeism −33 % after the 1985 campaign, which also caused sugar shortages). A time sink, a health sink, a barter medium and a policy lever. Sensitive; the data justify a first-class system.
- **Deficit-goods list**: specific goods cycle in and out of deficit by city tier and period; deficit status drives queue length and blat activation.

### 7.14 Biographies — HYPOTHESIS
Permanent: birth, education milestones, qualification, major employment changes, household formation, children, major moves, death. Recent detail for diagnosis; old routine history compressed.

## 8. Labour, workplace, unions, representation — lead-audited (synthesis §3.5)

- **Labour is differentiated capacity** (PLAUSIBLE): 20 operators, 2 technicians, 1 inspector — one technician present may halve output despite spare operators. Qualification categories only where they change allocation or production. **CODE:** `productivity = workers.len() / n_workers` (`goods_company.rs:85`).
- **Tenure ramp** (CONFIRMED — ~17 %/yr turnover; replacement ramps of months): `effectiveness = min(1, experience / ramp)`; high turnover lowers output without changing headcount, and the inspector shows it without a hidden "turnover" stat (B1 §3e).
- **Labour hoarding** (CONFIRMED — Kornai ch. 11; 10–20 % surplus against absence, storming, recruitment risk): mirrors material hoarding.
- **Childcare** (CONFIRMED — factory nurseries; +86 % places 1961–70): a physical place releases specific care hours; never a workforce percentage. Kindergarten is a charter Post-1.0 cut; keep the household-scheduling hook.
- **Education → qualification → assignment → relocation** (CONFIRMED — raspredelenie, 1933–): 1.0 has two tiers.
- **Work collectives and issue aggregation** (HYPOTHESIS): no `grievance = 82`; issues derive from physical facts (repeated month-end overtime, unsafe machine, housing delay, canteen shortage, congestion) and keep causal references. **Era caveat:** the labour collective as a legal actor dates from 1983; in the 1950s–60s the enterprise trade-union committee and production conferences played this role.
- **Trade unions** (CONFIRMED — social insurance from 1933, labour protection and technical inspection, joint housing role, sanatoria; Deutscher 1950) — **Post-1.0.** Not a Western bargaining body, not a strength meter. A `UnionCommittee` owns membership, safety/welfare cases, proposals, meeting cadence, memory — never production or stock.
- **Safety inspection** (PLAUSIBLE — ILO/Semenov 1983; CIA "Labor Safety in Soviet Industry" 1965): inputs are maintenance backlog, fatigue, overtime, machine condition, minor incidents; choices are stop the line, defer, reallocate maintenance, ease quota pressure; consequences stay physical. Storming and alcohol both raise accident rates.
- **Local Soviets and elections** (CONFIRMED form — single-candidate, party-nominated; the useful mechanic is **nakazy**, electors' mandates a deputy is bound to pursue, and standing commissions on planning, women's working conditions, mother-and-child welfare) — **Post-1.0.** Model representation and information, never multiparty competition.
- **Institutional confidence** (HYPOTHESIS): complaint submitted → action taken or not; effective channels encourage reporting, ineffective ones encourage exit, absenteeism, informal adaptation. Institutional reliability, not mind-reading. Never loyalty in 1.0.
- **Alternative rulesets** as concrete questions (HYPOTHESIS; Lane H reached the same conclusion): who proposes quotas, sets reserves, selects management, allocates surplus, controls housing/welfare, approves overtime/norms, coordinates inputs. Never government-type bonuses. See `docs/vision/game-modes-post-1p0.md`.

## 9. Vehicles, road traffic, transit, rail

**CODE (Lane D):** `Vehicle` has no mass, cargo, power, owner or capacity (`vehicle.rs:26-45`); motion is `speed += clamp(desired − speed, −decel, accel)` (`road.rs:141-183`); avoidance is a spatial-grid cone check, not IDM (`road.rs:186-407`); pathfinding cost is `length/speed_limit + noise` (`pathfinding.rs:234-239`); rail is the most developed subsystem (consist mass/length/power/braking from prototypes, intersection reservation, look-ahead braking — `train.rs:58-78, 388-475`).

- **Specialised lane physics, not rigid bodies.** State: lane, s, v, a, length, mass, power, tractive force, braking, cargo load, route. `F_grade ≈ m g sin θ`; `F_tractive ≈ P/v` capped by traction; jerk-limited. Grade comes free from the lane polyline's Z derivative; one `sin()` per vehicle per tick (D §3.1). Loaded and empty trucks climb differently — terrain becomes economic. Open: 1.0 or hook?
- **Collision avoidance** inspired by IDM/MOBIL (Treiber 2000; Kesting 2007) — parameters per kind in D §3.2; MOBIL needs multi-lane roads the substrate lacks. Overlap is an invariant failure, not an accident generator. **Missed by the thread:** junction deadlock today is a random wait (`road.rs:217-225`); nose-to-nose vehicles deadlock forever. Real resolution is needed.
- **Traffic state** (SPEC-TRAFFIC-007/008, draft): EWMA load, BPR `1 + 0.15 (v/c)^4`, Gawron 0.3/0.7 — D §3.3 shows the cheapest integration on the existing substrate. Ultimate traffic also needs **queue storage and spillback**: a cell-transmission meso layer (Daganzo 1994; LTM, Yperman 2007) with identity-carrying freight promoted to micro (D §3.8). Open: meso in 1.0?
- **Industrial gates:** unloading delay → yard fills → gate queue → public-road spillback.
- **Shift waves:** large workplaces pulse demand; staggering (07–15 / 08–16 / 09–17) lowers peak load with the same infrastructure. **CODE:** work intervals already carry random offsets (`desire/work.rs:32-37`) — a cheap PARTIAL → EXISTS move.
- **Transit** (mostly Post-1.0): seats, boarding throughput, dwell, headway, crowding, bunching; trams/trolleybuses couple to electricity. Passenger rail, signals and electrification are Post-1.0 (charter).
- **Freight rail 1.0:** three buildings, one locomotive, one wagon (charter); same custody rules. Preserve fields for locomotive and wagon identity, consist length/mass, **wagon capacity and cargo custody** (ABSENT — `RailWagon` has no cargo type or capacity, D §4.4), source/destination, progress.
- **Future rail:** consist mass, tractive effort, braking, length, gradient, siding compatibility; **signalling is the true capacity constraint** — today two trains can occupy one segment because only intersections are reserved (D §4.2); empty-wagon repositioning is real traffic; yards as processors (arrival → classification → storage → assembly → departure). **Data bug:** rolling-stock `max_speed` 200 m/s (720 km/h) and 360 m/s are placeholders; ~30 m/s freight, ~44 m/s passenger are realistic.
- **Winter roads:** surface state modifies acceleration, braking, headway, capacity; snow clearing is physical vehicles from depots under Planner road priorities; road wear from axle load (Post-1.0).

## 10. Utilities and physical inertia

**CODE (Lane D):** electricity is a union-find over road adjacency (`electricity_cache.rs:244-279`) with a binary blackout when consumed > produced (`electricity.rs:43-93`); `SPEC-ELECTRICITY-001` forbids exactly this. Water, sewage, heating, gas: no building kind, no system, no data structure. Replacing the union-find is a **full replacement** of the connectivity model, not an increment.

> **Model the different forms of inertia that make each network behave differently.** Share topology and tooling; never one solver.

| Network | Inertia | 1.0 draft spec | Cheapest adequate model (D §3) |
|---|---|---|---|
| Electricity | near-instant balance | `G + D = V + C + L`, continuous non-price priority shedding, `B_next = B + C − D` (SPEC-ELECTRICITY-002/003) | sum generation per island; serve demands in priority order; per-building served/curtailed with reason. Future: min/max, ramp, startup, reserve, generator class. No AC physics until proven needed. |
| Water | pressure and tank storage | separate authority, finite-rate, quantity and quality, buffers, border meter, never cargo, idempotent `WaterTransferID` (SPEC-WATER-001…006); static head + tank in 1.0 ([ADR-0001](../decisions/0001-households-and-utilities-are-1.0-scope.md)) | tree-based static head: `H = H_src − loss·L − Δz`; floor-N pressure is a lookup; pump power couples to electricity. Full EPANET GGA is overkill; use EPANET as a validation oracle. Tank drains before service fails — the delay is gameplay. |
| Sewage — **Post-1.0** | gravity and backpressure | none (charter cut, [ADR-0001](../decisions/0001-households-and-utilities-are-1.0-scope.md)) | gravity DAG, per-pipe capacity, junction buffers, treatment; full downstream buffer restricts upstream; backlog persists; pump loses power → buffer fills → declared restriction. SWMM as oracle. |
| Heating | transport delay and thermal mass | distinct thermal network, conservation, **no electric fallback** (SPEC-HEATING-001; EVID-HEATING-002); needs a ratified Weather interface | pipe FIFO delay line `(T, volume)` + first-order building ODE `dT = (Q_in − U·A·(T − T_out)) / C`. Coal stops → hot water still arriving → slow cooling → cold flats hours later. |
| Gas — **Post-1.0** | linepack | none (charter cuts pipelines) | one integrator per segment: `mass += (in − out)·dt; p = f(mass)`; supply shortfall hides while linepack drains, then collapses; compressors consume electricity. |
| Reservoir/hydro | stored water and head | charter breadth exception; no spec | `V_next = V + inflow − outflow − losses`; `P = ρ g Q H η`; releases trade current power for future water. HEC-ResSim as oracle. |

**Weather** through explicit interfaces (heating demand, hydrology, crop cycles, road surface, utility demand) — never city-wide multipliers. **No weather spec exists** (bible §21.1), and one weather state stresses every network at once (D §4.9): roads, freeze risk on mains (D §4.7), heating demand, winter electricity peak, snowmelt into sewers, frozen switches. Coal grades (lignite ≈ half of anthracite) are a heat variable the thread skipped.

**Network reserves** — a unifying Planner view in natural units: roads (unused capacity), rail (headway slack, spare wagons), logistics (inventory, fleet slack), water (tank volume, pump headroom), sewage (empty buffer, treatment headroom), heating (hot network water, building thermal mass), electricity (ramping/generation reserve), gas (linepack), reservoir (stored head), society (discretionary time, spare housing and service capacity). "Coal bunker: 18 h at current burn." A republic can function while every reserve is dangerously low.

**Physical momentum and phase lag:** stocks, water, heat, linepack, rail slack and household buffers carry the system after a disruption and delay recovery too. The expert Planner manages trajectories — current stock, consumption rate, incoming ETA, time to depletion — not red icons.

## 11. Cross-system causal loops

All CONFIRMED as constructions from validated mechanisms; **none is wired in code** except electricity → productivity (`goods_company.rs:104-108`).

- **Space electronics cascade:** quota risk → emergency electronics → over-request/storm → copper and precision demand → rail pressure → consumer output loses inputs → congestion → components later → more storming → quality/rework → replacement demand → risk worsens. Worker layer: storming → overtime → fatigue → safety complaints → institutional pressure → maintenance/welfare trade-off.
- **Coal → electricity → water → sewage → heat:** coal train late → thermal reserve falls → curtailment → pumps constrained → tank drains → sewage buffer fills → district restriction → heating circulation degrades → time, health, warmth. Different speeds; the delay is the mechanic.
- **Housing → labour → production:** expansion → labour demand → queue/commute → turnover → understaffing → shortfall → construction materials scarcer → housing slows. (The monotown death spiral, CONFIRMED — Feshbach.)
- **Retail → labour:** store shortage → search/queue time → sleep and discretionary time fall → lateness, fatigue, absence → workplace capacity falls. Consumer logistics is an industrial input.
- **Reliability spiral,** both directions (§5.2).
- **National-project privilege:** a project receives housing, specialists and freight priority → performs → other districts lose exactly those → queues, strain, labour shortage elsewhere. No "national project penalty" modifier is ever needed.

## 12. National projects and scenarios

Post-1.0 direction. Sixteen mode cards, the scenario-vs-mode distinction, mid-save transitions, chronicle, the tutorial problem and multiplayer-as-mode are in `docs/vision/game-modes-post-1p0.md`. Two rules from that document bind here: a National Project is a temporary nationwide material and labour distortion with phases, reserved materials, project cargo and priority rules — it stresses the ordinary economy, never lives in a separate tech tree; and the Space Programme is an industrial/logistics project, not a flight simulator.

## 13. Simulation architecture — principles

Design detail is in three proposals (`docs/plan/proposals/sim-tick-phases.md`, `citizen-architecture.md`, `causal-inspector.md`) and the crate-level research note (`docs/research/rust-architecture-proposals-2026-08-28.md`). The principles, with their code status:

- **The centre is not ECS.** Identity, time, authority, transactions, change propagation, determinism, information boundaries. **CODE:** the world is a hand-rolled typed store of `HopSlotMap`s over `slotmapd` (Uriopass's determinism fork of `slotmap`) — not an ECS, and not to be replaced by one (C1-07).
- **Module shape** (modules inside `simulation`, not new crates): core (time, ids, units, scheduler, random, transition, change_journal) · stores · physical · society · institutions · observatory · forecast · snapshot.
- **Two identity families:** append-only dense typed IDs for citizens and households (a dead Citizen #N stays #N); generational slot-map handles for bodies, vehicles, itineraries, hauls. **CODE:** everything is generational (C2-08).
- **Dense stores, SoA where hot,** sparse side stores for rare states (pregnancy, illness, enrolment, queue membership, social edges). No heap allocation, `String` or `HashMap` in hot records. **CODE:** `PersonalInfo.name: String` boxed on every human.
- **Typed IDs and units** at authority boundaries; fixed/integer for conserved quantities. **CODE:** entity IDs typed; only `Money` and `Power` newtypes; quantities bare `i32`/`f32` (C2-07).
- **Fixed resource arrays** indexed by typed resource (`enum-map` is already a transitive dep — C1 §4.2). **CODE:** `BTreeMap<ItemID, SingleMarket>` with `BTreeMap<SoulID, i32>`.
- **Bitset society:** `working_age ∩ technical ∩ district_7 ∩ available ∩ reachable` before expensive evaluation — *filter cheaply, think expensively.* Needs dense citizen IDs.
- **Event-driven:** sleep → wake on condition → decide → emit intent → commit → schedule next → sleep. Explicit state-machine enums, never suspended futures. A deterministic calendar (serialisable, stable ties by domain/entity, no thread-order dependence). **CODE:** every system every 20 ms tick; the only wake-up is `HumanDecision.wait` (C2-05/16).
- **Cadence bands:** movement high; traffic medium; dispatch seconds–minutes; production minutes; utilities by domain; households event/daily; demography daily–yearly; plan reporting at period boundaries.
- **Deterministic phases:** COMMAND · TOPOLOGY · ALLOCATION · DECISION · ROUTING · MOVEMENT · ARRIVAL · PRODUCTION · UTILITIES · ACCOUNTING · REPORTING — a **target**. **CODE:** 18 systems in flat registration order with electricity *first* and map update second-last; reordering changes replay hashes. Label first, reorder later (C2 §3.2).
- **Parallel compute → deterministic commit:** read-only parallel compute → local intent buffers → concatenate → stable sort by immutable key → serial commit. Correctness never depends on `DashMap`, lock order or Rayon scheduling. **CODE:** `ParCommandBuffer` is a `Mutex<Vec>` applied in insertion order — non-deterministic if ever fed in parallel (C1 §3.3). **Constraint the thread missed:** `networking/` is lockstep (`Frame(u64)`; `assert_eq!(frame, tick+1)`) — parallelism must be bit-identical or multiplayer is dropped.
- **Keyed randomness** `(master_seed, domain, entity, ordinal)` — the MUST-DO-FIRST for any parallelism (C2-09). **CODE:** one global Xorshift128 drawn sequentially (`rand_provider.rs`); any insertion-order change reshuffles everything downstream.
- **Typed system contexts** instead of `&mut Simulation`. **Obstacle the thread missed:** `ParCommandBuffer::exec_ent` takes `FnOnce(&mut Simulation)` and is the main cross-system mutation channel (C2 §4.3); typed contexts must first give deferred callbacks a declared resource set.
- **Authoritative transitions** with immutable IDs: `DeliveryId`, `EmbedId`, `ProductionRunId`, `ElectricityAllocationId`, `HeatAllocationId`, `DeathResultId`. Not one giant transaction framework.
- **Change journal** → observatory, indexes, notifications, causal history, snapshots. Propagate what changed; do not rescan the civilisation. **CODE:** `rerun.rs` is dead; `EcoStats` is ring-buffer history (C2-11).
- **Causal facts** `(id, tick, subject, kind, causes[])` with retention classes: recent failures detailed, routine history aggregated, lifecycle and plan events permanent. Never full event-sourcing forever.
- **Four immutable snapshots** (Planner, Render, Audio, Debug) via `ArcSwap` (already a dep). **The Planner snapshot never exposes hidden physical truth for convenience;** every value declares how it is known.
- **Hierarchical routing** cached by (origin region, destination region, mode, class, topology revision, traffic epoch). **CODE:** fresh A* per request.
- **Network kernel:** typed nodes/edges, attachments, components, topology revision, CSR adjacency; each domain keeps its own solver state.
- **Shadow simulation / Gosplan computer** forecasts from *reported* state, so a feasible plan can still fail physically. **CODE:** `Simulation` is `Serialize + Deserialize` and headless ticks, so `fork()` is possible at ~100 ms per clone (C2 §2.10).
- **LP/MILP** as an instrument, never the player: "given reported capacities, recipes, stocks and declared transport limits, is this plan materially feasible?" Backend is an open conflict (synthesis §6).
- **GPU boundary:** authoritative decisions on CPU; GPU for culling, interpolation, crowds, heatmaps, effects; validated POD only (`bytemuck` present).

## 14. Engineering standards (recommended; not binding until ratified)

Authority: one owning module per mutable field. Transactions: immutable ID, named endpoints, explicit delta, atomic where conservation requires, replay no-op. Failure answers *what is waiting, why, since when, who owns it, what recovers it* — never a bare `failed: bool`. Deterministic tie-breaks on immutable identity. Data layout for 250k-scale records: no per-record heap, no `String`, no `HashMap` where dense indexing works, size assertions. Names in cold or interned stores. Floats only for bounded physical/presentation values with controlled order and tested repeatability; integers/fixed for conserved quantities — **and `libm` for transcendentals if cross-platform determinism is a goal** (C1-14; the thread omitted this). Randomness domain-keyed. Slow state event-driven; any full-population per-tick scan must be justified. UI/render/audio read immutable snapshots. Caches keyed by explicit revision or epoch. Released saves carry an envelope (magic, format version, schema version, codec, sizes, checksum, payload) **and a migration seam** — the thread specified the envelope and forgot the migration (C2 §4.1). Canonical state digest for determinism tests (hash choice is an open conflict). Every mechanism document states authority, state, invariants, failure behaviour, observability, acceptance evidence, current gap, deferred behaviour, open questions. Prefer std and compact code; no ECS rewrite, no async-per-citizen, no concurrent-map truth, no nightly SIMD as foundation.

## 15. Validation strategy

Conservation property: `source + destination + custody + embedded + declared sinks = initial + declared sources` over request → reserve → pickup → cancel → reroute → return → deliver → consume (the `ledger-invariant-checker`'s question; 13 ledger tests cover the truck leg today). Idempotency: apply every replayable transition twice. Mutation tests: every acceptance test names the wrong implementation that must fail (the `evidence-auditor` method; cargo-mutants ADOPTED, `sov-mwy`). **Repeat-run determinism** — same initial state and commands → identical digests — is **absent**: `TestCtx::check_determinism` proves serialize→deserialize round-trip only (`tests/mod.rs:106-121`). Per-phase digests make divergence bisectable. Reference oracles: EPANET, SWMM, HEC, IDM, CTM. Benchmarks: none exist; the 250k contract has no gate (`sov-1ae` cancelled). Optimise in order: representation → cadence → locality → incremental → hierarchy → parallelism → SIMD.

## 16. Observability

Every significant object answers STATUS / CAUSE / TREND / POLICY / PHYSICAL CHAIN, with a **provenance column per line** (measured / reported / aggregated / observed / estimated / unknown). Worked examples, pressure maps, reserves in natural units and causal notifications are in `docs/plan/proposals/causal-inspector.md`. **CODE:** the building inspector shows workers, productivity, power, progress, storage (`inspect_building.rs:150-267`); nothing causal; `Market::requested()` unread.

## 17. Scope discipline

1.0 is the charter table (`charter-1.0.md:42-54`): fifteen resources plus Medicine; Food and Meat as separate needs; Water a utility with static head and tank storage; electricity, heating and waste as utilities; households with a housing queue; physical production and logistics; dishonest-enterprise inspectability; construction Sites; household consumption with explicit going without and an observable housing shortage; demographics including death; two education tiers; healthcare; landfill and incinerator; terrain, reservoir graph, hydro, ore, pollution; minimal freight rail; border trade in one rouble; three authored Plans on one save; day/night and seasons; bounded visible citizens; Linux/Windows; 250k identities at 60 fps as a target with no gate yet ([ADR-0001](../decisions/0001-households-and-utilities-are-1.0-scope.md) added the Households and citizens and Utilities rows and cut sewage). **Hooks now, mechanics later:** childcare, work collectives, unions, representation, plots, informal networks, quality lots, vehicle wear/fuel, signalling/electrification/passengers, sewage, CHP, gas, stormwater, grid physics. A hook avoids an architectural dead end; it never implements dormant complexity. The charter's cuts and its Never list are absolute.

## 18. Implementation sequence — reconciled (synthesis §3.14)

0. **Architecture contract:** ratify missing specs (agriculture, terrain/geology, weather, hydrology, pollution, Plan/Quota/Tranche, authored plans, notifications, shell/save/crash, presentation/audio); resource units and handling classes; ID and transition conventions; calendar/determinism contract; benchmark harness; **pin the `egui`/`yakui` git deps; add the save envelope and migration seam.**
1. **Prove 250k representation:** keyed RNG first; repeat-run determinism test; compact `CitizenStore` with persistent IDs; event calendar; snapshots; world digest; headless 250k benchmark. State the active-fraction target.
2. **One complete physical chain:** the truck leg exists; make exports physical (`market.rs:774`), add loading/unloading time and vehicle custody; retire the five contradicted paths (export teleport, two domestic-money debits, auto-lots, static multiplier).
3. **Dishonest-enterprise loop and observatory:** requested vs consumed on screen (~30 lines), surplus, discrepancy, material balance, change journal. This proves the thesis earliest and answers the bootstrap problem: the minimum viable loop is one discrepancy on one panel.
4. **Construction:** Ghost → Site → activation for one building.
5. **Households and lived scarcity:** household identity, residence/queue, Food/Meat pantry, consumption, going without, minimal scheduling.
6. **Movement at scale:** hierarchical routing, durable traffic state, spillback where feasible, render culling/LOD.
7. **Utilities:** separate solvers with explicit service results — electricity (replacing the union-find), water, sewage, heating, waste, reservoir/hydro.
8. **Labour, services, demography:** employment/qualification, education, healthcare, death; richer time effects if budget allows.
9. **Plan/Quota/Tranche macro loop** — with one caveat from Lane H and Lane A: a minimal plan-period clock may need to arrive earlier, because storming, ratchet, credibility and the Taut Plan all hang from it.
10. **Authored Plans and polish,** including the tutorial as a mode-design problem (the First Plan teaches the loop through play; the HUD strip points at discrepancies). Polish is interleaved throughout, never postponed.

## 19. Anti-patterns

Simulation maximalism ("as much as the engine allows" is the wrong bound). Generic percentage modifiers (`kindergarten +5 %`, `snow −20 %`) — prefer physical chains. Omniscient UI. Hidden personality flags (honesty, loyalty, corruption, morale) as substitutes for observed behaviour. One shared solver for unlike networks. Per-frame citizens (a failure, not a threading problem). Parallel mutable truth. Full event-sourcing forever. Crate-driven architecture. Historical flattening (one timeless Soviet citizen). Predetermined ideological failure — good planning must be able to work.

## 20. Mechanic taxonomy

Every future specification or ticket classifies its mechanic:

```text
Mechanic · Historical/technical evidence · Scope (1.0 / hook / Post-1.0) · Authority ·
Authoritative state · Inputs · Transition cadence · Conservation/invariants · Failure behaviour ·
Player decisions · Planner-visible information (with provenance) · Hidden/institutional information ·
Causal facts emitted · Computational representation · Benchmark/test · Open questions
```

## 21. Related documents

- Reconciliation and evidence: `docs/research/conversation-mining-2026-08-28/SYNTHESIS.md` and lane reports A–H
- Game modes: `docs/vision/game-modes-post-1p0.md`
- Proposals: `docs/plan/proposals/{causal-inspector,sim-tick-phases,citizen-architecture}.md`
- Crates and architecture research: `docs/research/rust-architecture-proposals-2026-08-28.md`
- Binding: `docs/plan/charter-1.0.md`, `docs/reference/glossary.md`, `docs/reference/specifications/`
- Current reality: `docs/reference/architecture/substrate.md`, `docs/research/fact-sheets/`
- Raw provenance: `docs/archive/raw-sessions/INDEX.md`
