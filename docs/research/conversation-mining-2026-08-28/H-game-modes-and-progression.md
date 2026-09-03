# Lane H — Game modes, campaigns, national projects, progression and the player loop

## 0. Summary (top ten findings)

1. **H-01** The fifteen named game modes are really organizational-parameter presets over one physical simulation, not separate games. Each changes ~4–8 institutional parameters (quota regime, reserve rules, priority classes, information quality, housing allocator). This is architecturally cheap once the parameters exist — but none of those parameters exist in today's code.
2. **H-09** Enterprise Director is structurally a different game: the player *is* the dishonest enterprise, optimizing against the Plan rather than writing it. It inverts the core loop and requires an AI Planner that does not exist. Risk: building it teaches nothing reusable for the base game.
3. **H-14** The progression ladder (fragile → integrated → sophisticated → national project) has no code substrate. The `Government` struct holds only `money: Money`. There are no quota periods, no plan periods, no credibility tracking, no institutional memory — the entire planner-side state machine is absent.
4. **H-03** National Projects (housing campaign, space, mobilization) are well-grounded historically. Each maps to a real Soviet program with documented logistics, but all three require systems that do not exist: construction material competition (housing), multi-tier priority freight (space), factory conversion and rationing (mobilization).
5. **H-06** The Sovnarkhoz mode is the most historically interesting organizational variant: 105→47 regional councils replacing branch ministries (1957–65). It reverses the information topology: local information improves, cross-regional coordination degrades. This is a clean parameter change if territorial vs. branch allocation exists.
6. **H-10** Scenario vs. mode is an unresolved architectural question. A *scenario* is a fixed starting state (map + buildings + resources + brief). A *mode* is a rule variant (changed parameters). The conversation conflates them. W&R uses scenarios (campaign1/2/3 with script.ini files) over one rule set; Victoria 3 uses a single rule set with country-specific starting states. The game should separate these cleanly.
7. **H-15** The multiplayer crate (`networking/`) is structurally complete: client/server, authentication, frame-synchronized inputs, world-state catch-up. A Gosplan-vs-Ministry multiplayer mode (one player sets quotas, others run enterprises) is technically feasible with the existing networking substrate — but the simulation has no ministry/enterprise role separation to exploit.
8. **H-12** Frostpunk *does* have game over: the city falls, the captain is banished. The conversation's "never game over" pillar is this project's own design decision, not borrowed from Frostpunk. This distinction matters: Frostpunk's pressure comes from potential termination; this game's pressure must come from degradation quality, which requires visible suffering the player cannot ignore.
9. **H-07** The "cheapest three modes to ship first" given current code are: (a) The Taut Plan (just quota targets + the existing market, since over-requesting already exists in `request_multiplier`), (b) Frontier Corridor (a scenario on a linear map with the existing rail + freight station), (c) Everyday Socialism (a scenario focused on the retail/citizen loop that already exists in `BuyFood`/`Home`/`Work`).
10. **H-16** The conversation missed the tutorial problem entirely. The charter requires "the First Plan alone must teach a new player to play for two hours without outside help." That is a mode design problem: the first plan *is* a scenario, and it must introduce the dishonest enterprise loop through play, not exposition.

## 1. Extracted items

| ID | Statement | Source line(s) | Verdict |
|---|---|---|---|
| H-01 | Organizational modes (branch ministries, territorial planning, self-management, reform-socialist autonomy, danwei, rationing) as rule variants over one physical economy | 482–484 | PLAUSIBLE — historically grounded, architecturally sound, no code substrate exists |
| H-02 | Fifteen named game modes listed | 488–503 | PLAUSIBLE — each has a historical basis; none has implementation substrate |
| H-03 | National Projects as temporary nationwide distortions of the material economy | 49–51 | PLAUSIBLE — matches Soviet megaprojects; needs priority freight, construction competition, factory conversion |
| H-04 | Space as an industrial/logistics national project, not KSP-style flight | 50, 200–216 | CONFIRMED — the electronics cascade (lines 200–216) is the mechanism; no orbital mechanics needed |
| H-05 | Mobilization as home-front economy with physical guns-vs-butter tradeoffs | 51, 67–68 | PLAUSIBLE — matches 1941 industrial evacuation; needs factory conversion, rationing, priority classes |
| H-06 | Sovnarkhoz 1957–65 as a territorial vs. branch planning mode | 493 | CONFIRMED by history — 105→47 regional councils, reversed by Brezhnev/Kosygin 1965 |
| H-07 | The Reform (Kosygin 1965) as a mode with enterprise profit autonomy | 494 (implied via "Reform") | CONFIRMED by history — 4–8 plan indicators, profit retention, limited implementation |
| H-08 | Self-Management (Yugoslav workers' councils) as a mode | 500 | CONFIRMED by history — 1950–1991, workers elect managers, allocate surplus |
| H-09 | Enterprise Director mode: player experiences why subordinates hoard and bargain | 502–504 | PLAUSIBLE — inverts the core loop; architecturally risky (needs AI Planner) |
| H-10 | Fragile → integrated → sophisticated → national project progression | 77–79 | PLAUSIBLE — no code substrate; `Government` has only `money` |
| H-11 | The plan as a sequence of quota periods on one continuous save | 24 | ALREADY-EXISTS in glossary; no code implements quota periods |
| H-12 | Dwarf Fortress-like persistence and causality | 19 | PLAUSIBLE — DF's Legends viewer records achievements across modes; could map to a causal history/chronicle |
| H-13 | CS2-quality urban design and scale | 16 | UNSUPPORTED as a mode claim — CS2 is a presentation target, not a mode |
| H-14 | W&R-style physical planned-economy causality | 17 | CONFIRMED — W&R is the reference implementation; installed locally with campaigns, scenarios, research tech tree |
| H-15 | "Automate execution, not decisions" (line 32) as a mode design constraint | 32 | CONFIRMED by design pillar — modes must change what the player decides, not automate decisions away |
| H-16 | Science cities / closed cities / priority geography | 464–466 | CONFIRMED by history — Akademgorodok (1958), Baikonur/Leninsk, numerous nuclear cities |
| H-17 | The danwei (Chinese work unit) as a mode | 484 | CONFIRMED by history — 1950s–1990s; enterprise-as-welfare-state is the mechanic |
| H-18 | Rationing regimes as a mode | 484 | CONFIRMED by history — wartime 1941 rationing by labor category |
| H-19 | Frontier Corridor as a mode | 498 | CONFIRMED by history — BAM railway 1974–1991, construction along a linear transport axis |
| H-20 | Late-System Maintenance as a mode | 499 | CONFIRMED by history — 1980s capital-stock ageing, maintenance costs rising from 10% to 20% of investment |

## 2. Validation detail

### H-01: Organizational modes as parameter presets

The conversation lists six organizational archetypes at lines 482–484. Each changes how allocation, information, and authority flow through the same physical economy. The key insight is that these are not separate simulations — they are configurations of institutional parameters:

| Parameter | Branch Ministry (default) | Sovnarkhoz | Self-Management | Danwei | Rationing |
|---|---|---|---|---|---|
| Allocation authority | Central by commodity | Regional by territory | Enterprise council | Work unit | Central by category |
| Information flow | Bottom-up reports to ministry | Bottom-up to regional council | Internal to enterprise | Internal to unit | Top-down ration cards |
| Reserve rules | State + enterprise | Regional + enterprise | Enterprise only | Unit only | State only |
| Priority classes | By commodity importance | By regional need | By worker vote | By unit leadership | By labor category |
| Housing allocator | Enterprise + district | Regional council | Enterprise council | Work unit (bundled) | State assignment |

**Code status:** `Government` at `simulation/src/economy/government.rs:9` has only `money: Money`. No allocation authority, no information topology, no reserve rules, no priority classes. The `Market` at `simulation/src/economy/market.rs:93` is a single global market with `BTreeMap<ItemID, SingleMarket>`. There is no concept of regional vs. branch vs. enterprise-level allocation.

### H-06: Sovnarkhoz reform 1957–65

The 1957 reform replaced ~30 central industrial ministries with 105 regional economic councils (sovnarkhozy), later consolidated to 47. Khrushchev's goal: combat departmentalism (*vedomstvennost'*) where ministries hoarded resources within their sector regardless of regional need.

**What changed:** Resource allocation shifted from vertical (ministry controls all steel plants nationwide) to horizontal (regional council controls all industry in its territory). This improved local coordination (a factory could get supplies from a nearby plant in another sector) but created *mestnichestvo* (localism) — regions hoarded resources just as ministries had.

**Game mechanic:** A Sovnarkhoz mode swaps the allocation topology. Intra-regional allocation improves (shorter transport, better local information). Inter-regional allocation degrades (regions protect local supplies). The player manages a different shape of the same hoarding problem.

**Source:** Kibita, *Soviet Economic Management Under Khrushchev: The Sovnarkhoz Reform* (Routledge, 2013); Wikipedia, "1957 Soviet economic reform"; Encyclopedia.com, "Sovnarkhozy."

### H-07: Kosygin reform 1965

In September 1965, Brezhnev and Kosygin abolished the sovnarkhozy and restored central ministries, but with a twist: enterprises gained limited autonomy. The number of obligatory plan indicators was reduced from dozens to 4–8 (output volume, assortment, profit, quality). Enterprises could retain profit for self-financing investment funds. Managerial bonuses were tied to profitability and sales.

**What failed:** Ministries reasserted control. "Changing the names on doors." Prices remained fixed, so "profit" was meaningless as a signal. The reform was quietly abandoned by the early 1970s.

**Game mechanic:** A Reform mode reduces the Planner's control surface (fewer quota levers) while giving enterprises profit-retention autonomy. The player discovers that less control can produce better or worse outcomes depending on whether price signals carry information (they cannot in this game, by pillar — clearing is by queue, never price). This mode teaches *why* the Kosygin reform failed.

**Source:** Encyclopedia.com, "Kosygin Reforms"; Wikipedia, "1965 Soviet economic reform"; Britannica, "Aleksey Nikolayevich Kosygin."

### H-08: Yugoslav self-management

Yugoslavia's 1950 Basic Law gave workers' councils control over enterprise decisions: production targets, surplus distribution, manager appointment. Unlike Soviet enterprises (which received commands), Yugoslav enterprises were self-governing but still operated within a social-ownership framework.

**What changed:** No central plan dictated output. Enterprises competed in markets but could not be privately owned. Workers voted on investment, wages, and hiring. This produced high growth until the late 1970s, then inflation, inefficiency, and inter-republican inequality.

**Game mechanic:** A Self-Management mode removes the Planner's quota authority over individual enterprises. The player sets infrastructure and social policy; enterprises make production decisions through worker councils (AI). The player's challenge shifts from "what to produce" to "how to create the conditions under which self-managing enterprises produce what society needs."

**Source:** Britannica, "Socialist self-management"; AEA, "Yugoslavia: The Case of Self-Managing Market Socialism" (JEP 5:4, 1991); Jacobin, "The Life and Death of Yugoslav Socialism."

### H-12: Frostpunk does have game over

The conversation cites Frostpunk as an influence (line 19), and the project's "never game over" pillar appears at lines 28, 337. However, Frostpunk 1 and 2 both have explicit game-over states: the city falls, the captain is banished, the population dies. Frostpunk 2 adds political endings (banishment, dictatorship, reconciliation).

**This matters for mode design:** Frostpunk's pressure comes from the threat of termination. This game explicitly rejects that source. The pressure must come from *visible degradation* — queues the player sees, citizens whose names they know suffering, production cascading into shortage spirals. This is a harder design problem. Each mode card must define its pressure source carefully, and "never game over" means the pressure source must be *qualitative* (things getting worse in ways the player cares about) rather than *terminal* (the game ends).

**Source:** Frostpunk Wiki, "The End (Arc)"; ProGameGuides, "All endings in Frostpunk 2."

### H-14: What W&R actually implements

The local W&R install (`~/.local/share/Steam/steamapps/common/SovietRepublic/`) reveals:
- **Campaigns:** `campaign1/`, `campaign2/` (binary save files with `script.ini` triggers), plus `campaign3/` under `scenarios/`.
- **Scenarios:** `scenarios/` directory with sub-scenarios (clothesexport, goods, steelexport) each with `script.ini`.
- **Tutorial maps:** 18 separate tutorial maps (`tutorial_map1` through `tutorial_map18`).
- **Research:** A tech tree with ~200+ research items (PNG icons in `research/`): from `bauxite_study.png` to `broadcasting_radio.png` to `big_monuments.png`.
- **Ministry characters:** `people.ini` names 13 ministry officials (Dunya Orlova for clothes, Stefan Kovalev for industry, etc.).
- **No organizational modes:** W&R has one rule set. Campaigns are scenarios (starting states + objectives), not rule variants. There is no "Sovnarkhoz mode" or "Reform mode" in W&R.

**Implication:** This game's organizational modes would be genuinely novel — no existing city builder does this. But the novelty also means there is no reference implementation to learn from.

### H-17: Chinese danwei

The danwei (work unit) was the foundational cell of urban Chinese life from the 1950s through the 1990s. Each unit provided employment, housing, ration coupons, medical care, pensions, childcare, and permission to marry or travel. The danwei was a "small society" (*xiaoshehui*) with little need for inter-unit exchange.

**Game mechanic:** A Danwei mode bundles welfare provision into enterprises. The player does not allocate housing, healthcare, or childcare separately — each enterprise provides its own. This simplifies some planning (no district-level service coverage) but creates new problems: large enterprises become welfare monopolies; closing a factory displaces not just workers but their entire social infrastructure; enterprise inequality becomes lived inequality.

**Source:** Lvivcenter, "The Transition from the Work Unit System"; MDPI Sustainability 12:4, "Corporate-Run Society: The Practice of the Danwei System"; Grokipedia, "Work unit."

### H-18: Wartime rationing

From July 1941, Soviet rationing was tiered by labor category: defense workers received the most, followed by industrial workers, then office workers, then dependents. This was not an abstract welfare system — it was a physical allocation mechanism that determined who ate and who did not.

In three months (July–October 1941), GOSPLAN relocated 1,360 factories: 455 to the Urals, 210 to Western Siberia, 250 to Central Asia. Over 10 million people were evacuated. Tank production went from 6,274 in 1941 to 24,639 in 1942.

**Game mechanic:** A Mobilization mode introduces factory conversion (civilian → military output), tiered rationing (priority classes for food/goods by labor category), and industrial evacuation (disassemble and relocate a factory — a physical logistics project). The pressure source is external: a material quota that increases each period, representing war demand.

**Source:** GlobalSecurity.org, "Military Industry Under Stalin - Evacuation"; Soviet History MSU, "Wartime Evacuation"; Left-Horizons, "Eighty years ago: evacuation of Soviet war factories."

### H-20: Late-system maintenance crisis

By the 1980s, the Soviet Union's capital stock was aging. Capital repairs as a percentage of investment rose from ~10% in the 1930s to ~20% in the 1980s. Equipment efficiency declined steadily from the 1950s. The western areas occupied in 1941 had been rebuilt with relatively modern equipment; the eastern areas (built under evacuation pressure) were wearing out. Road infrastructure remained underdeveloped.

**Game mechanic:** A Late-System Maintenance mode starts with a large, functioning but aging republic. Buildings and equipment degrade over time. The player must allocate scarce construction/maintenance capacity between new projects and upkeep. The pressure source is entropy: everything decays simultaneously, and the player cannot fix everything at once.

**Source:** CIA Paper 251 (history.state.gov/historicaldocuments/frus1981-88v03/d251); Wikipedia, "Transport in the Soviet Union"; Kukić (2024), "Technical change and the postwar slowdown."

## 3. Deeper mechanics — mode cards

### 3.1 Mode card template

Each card follows: premise, starting state, rule changes (which institutional parameters change), pressure source, win-less success condition, 10-minute loop, 10-hour arc, what it teaches, base-system dependencies.

---

### MODE 01: The Housing Campaign

**Premise:** It is 1957. The Party has pledged to end the housing shortage within twelve years. You must house 84 million people in standardized mikrorayon blocks while keeping the industrial economy running.

**Starting state:** A medium-sized industrial republic with severe housing overcrowding. Construction capacity exists but is committed to industrial projects.

**Rule changes:** Construction material allocation gains a new priority class: Housing Campaign. Housing quotas are period-binding (must build N dwelling units per plan period). Standardized housing types (Khrushchyovka) are cheaper and faster but lower quality. Allocation of construction workers, concrete, steel, and timber competes directly with industrial construction.

**Pressure source:** Housing quotas ratchet upward each period. Failure to meet housing quotas increases worker turnover (enterprise labor instability). But diverting construction materials from industrial expansion reduces productive capacity.

**Win-less success condition:** Never game over. "Done" looks like: housing queue median wait < 2 years, overcrowding rate < 15%, and industrial output has not collapsed. The tension never fully resolves — there is always a queue.

**10-minute loop:** Place Khrushchyovka blocks, allocate construction materials between housing and factories, check the housing queue, check industrial output, adjust the balance.

**10-hour arc:** Expand from acute overcrowding through mass construction to a functioning mikrorayon system with schools, shops, and transit. Late game: the buildings are aging, quality problems emerge, citizens want better housing.

**What it teaches:** Construction competes with production for the same physical materials. Housing is industrial infrastructure (workers need homes to be productive). Quality vs. quantity is a real tradeoff.

**Dependencies:** Construction system (material bill, site, ground broken — glossary terms exist). Housing allocation queue. Worker turnover linked to housing satisfaction. Mikrorayon completeness (school, shop, clinic coverage). Resource competition (concrete, steel, timber). **Requires lanes A (material allocation) and B1 (citizen housing queue).**

---

### MODE 02: The Monotown

**Premise:** Build and sustain a single-enterprise settlement around a new factory in undeveloped territory.

**Starting state:** Empty terrain with a rail connection. One large factory to build. No housing, no services, no infrastructure.

**Rule changes:** The factory IS the settlement. Enterprise welfare (housing, canteen, clinic, childcare) is bundled with production. Worker recruitment depends on housing availability. No external labor market — workers must be attracted from elsewhere.

**Pressure source:** The factory has production quotas, but it cannot produce without workers, and workers will not come without housing, and housing cannot be built without construction materials the factory does not produce. Circular dependency.

**Win-less success condition:** Factory reaches 80% capacity with stable workforce (turnover < 10%/year) and settlement has basic services.

**10-minute loop:** Balance construction priorities (factory vs. housing vs. services), recruit workers, manage supply trains, check production.

**10-hour arc:** From a construction site to a functioning monotown. Late game: the factory's product becomes less needed, diversification is needed, but the town has no other purpose.

**What it teaches:** The true cost of an industrial project includes its entire settlement. Housing shortage → overcrowding → turnover → production shortfall → housing delay (the monotown death spiral).

**Dependencies:** Construction, housing allocation, worker recruitment/turnover, enterprise welfare, rail freight. **Requires lanes A (production), B1 (settlement), D (rail freight).**

---

### MODE 03: Science City

**Premise:** Build an Akademgorodok: a closed scientific community with priority resource access, attracting the nation's best researchers.

**Starting state:** Undeveloped forest/steppe near a major city. Budget for housing and research institutes. Priority access to consumer goods.

**Rule changes:** Priority allocation class for the settlement (better food, goods, housing). Education requirements for residents (only qualified researchers and support staff). Research output mechanic (not in base game — would need a research/tech system). Intellectual freedom modifier (not directly simulatable — could be abstracted as researcher satisfaction affecting output and retention).

**Pressure source:** The Party demands research output. Researchers leave if living conditions deteriorate or intellectual conditions become restrictive. Priority access creates resentment elsewhere (opportunity cost).

**Win-less success condition:** Research output sustained, researcher retention stable, settlement self-sustaining.

**10-minute loop:** Manage research institute staffing, housing allocation, consumer supply, education pipeline.

**10-hour arc:** Build the settlement, attract researchers, achieve research milestones, manage political pressure to reduce privileges.

**What it teaches:** Priority geography — concentrating resources creates a pocket of abundance at the cost of scarcity elsewhere. Human capital is the scarce resource, not materials.

**Dependencies:** Education system (qualification tiers), housing allocation, priority consumer allocation, worker retention. Requires a research/tech system that does NOT exist and is NOT in the 1.0 charter. **Most expensive mode to build.**

---

### MODE 04: Shortages Amid Plenty

**Premise:** A paradox scenario. The republic produces enough of everything in aggregate, but citizens experience constant shortages due to distribution failures, hoarding, and information problems.

**Starting state:** A large, mature republic with adequate total production. But dispatch is unreliable, enterprises hoard, information is delayed, and retail distribution is poor.

**Rule changes:** None — this is the BASE GAME in steady state. The "mode" is really a scenario that puts the player into the middle of the hoarding/information loop rather than building from scratch.

**Pressure source:** Citizens experience shortages despite adequate production. The player must find and fix the distribution failures, the hoarding enterprises, and the information gaps.

**Win-less success condition:** Reduce queue times, reduce enterprise safety stocks, improve dispatch reliability, increase citizen satisfaction — without reducing production.

**10-minute loop:** Inspect enterprise stock levels, check dispatch queues, identify hoarding, adjust allocation priorities, inspect citizen shopping time.

**10-hour arc:** Systematically improve distribution from chaotic to reliable. The core learning: the problem is coordination, not production.

**What it teaches:** THE CORE GAME LOOP. This is the dishonest enterprise in its purest form. Priority cannot solve scarcity — it only decides where scarcity appears.

**Dependencies:** Enterprise request/stock observation, dispatch system, citizen shopping/queue time, hoarding detection. **This IS lanes A + B1 + D.**

---

### MODE 05: The Taut Plan

**Premise:** Every resource is allocated to the last unit. No slack, no reserves, no buffers. One disruption cascades through the entire economy.

**Starting state:** A functioning republic with very tight quotas and zero strategic reserves. Everything works — barely.

**Rule changes:** Reserve caps set to zero. Quotas set to 100% of capacity. No surplus allocation.

**Pressure source:** Any disruption (weather, equipment failure, transport delay) cascades. The player must manage the cascade in real time.

**Win-less success condition:** Survive a plan period without total collapse. Gradually build reserves and slack into the system.

**10-minute loop:** Monitor all production chains, respond to disruptions, reallocate in real time.

**10-hour arc:** Transform a taut plan into a resilient one by building buffers, improving reliability, and accepting lower peak output for stability.

**What it teaches:** Slack is resilience. A nominally less aggressive plan can outperform a taut one.

**Dependencies:** Reserve system, quota system, disruption/failure events, cascade propagation. Needs `request_multiplier` (exists in code) and quota targets (do not exist). **Cheapest mode after "Shortages Amid Plenty" because it needs minimal new systems — mainly quota enforcement.**

---

### MODE 06: The Reform (Kosygin)

**Premise:** It is 1965. The Party has authorized limited enterprise autonomy. Enterprises retain profit, manage their own investment, and respond to 4–8 plan indicators instead of dozens.

**Rule changes:** Reduce Planner's quota levers from full control to 4–8 aggregate indicators. Enterprises gain profit-retention (a money pool they invest autonomously). Quality becomes a plan indicator alongside volume.

**Pressure source:** Enterprises optimize for the indicators you set. If you measure gross tonnage, they produce heavy things. If you measure profit (but prices are fixed), profit is meaningless. The player discovers that fewer controls can mean less control OR better control depending on institutional design.

**Win-less success condition:** Economy grows while enterprise autonomy increases. But growth is hard to sustain because the player cannot micromanage distribution.

**What it teaches:** Why the Kosygin reform failed — and whether it could have succeeded with better indicator design.

**Dependencies:** Indicator/quota system, enterprise autonomy parameters, quality tracking. **Requires the full indicator system from lane A.**

---

### MODE 07: Sovnarkhoz

**Premise:** It is 1957. Replace branch ministries with regional economic councils. Allocation shifts from vertical (by commodity) to horizontal (by territory).

**Rule changes:** Allocation topology changes. Intra-regional allocation is faster and better-informed. Inter-regional allocation requires explicit coordination (slower, more loss). Each region can see its own enterprises clearly but has poor information about other regions.

**Pressure source:** *Mestnichestvo* (localism) — regions hoard resources for local use. The player must coordinate between regions without the old ministry system.

**Win-less success condition:** Inter-regional trade flows efficiently despite decentralized authority.

**What it teaches:** Decentralization trades one kind of hoarding for another. The problem is the topology of information, not the level of centralization.

**Dependencies:** Regional/territorial subdivision of the map, allocation topology, information delay by distance. **Requires spatial economic subdivision that does not exist in code.**

---

### MODE 08: Everyday Socialism

**Premise:** A slice-of-life mode focused on the lived experience of citizens. The player manages a district, not a nation. Shopping queues, childcare, housing allocation, commute times, and household time budgets are the gameplay.

**Starting state:** A functioning mikrorayon with shops, schools, clinics, transit, and housing. Citizens have names, households, schedules.

**Rule changes:** None — this is a scenario zoom into the citizen simulation. Industrial production is assumed; the player manages distribution, services, and daily life.

**Pressure source:** Citizens' daily lives degrade when services fail. Queue times rise, commutes lengthen, shopping takes longer, childcare gaps appear.

**Win-less success condition:** Median citizen discretionary time increases. Queue burden decreases. Service coverage improves.

**10-minute loop:** Check queue lengths at shops, adjust delivery schedules, inspect citizen daily schedules, manage transit.

**10-hour arc:** Transform a struggling district into a smoothly functioning mikrorayon.

**What it teaches:** Social reproduction is as important as industrial production. Time poverty is a real constraint.

**Dependencies:** Citizen daily schedule (`Work`, `BuyFood`, `Home` desires exist in code), retail delivery, housing, transit. **Cheapest citizen-focused mode because `BuyFood`/`Home`/`Work` already exist at `simulation/src/souls/desire/`.**

---

### MODE 09: National Project — Space

**Premise:** The Party orders a cosmodrome and space program. This is an industrial/logistics megaproject: build a remote settlement, establish supply chains for electronics/precision components/fuel, and meet escalating material targets.

**Starting state:** A mature industrial republic. A remote location designated for the cosmodrome. The electronics cascade from lines 200–216 is the core mechanic.

**Rule changes:** Priority freight class for space materials. Emergency dispatch authority. The cosmodrome is a closed settlement (priority consumer allocation). Space quotas escalate each period.

**Pressure source:** The electronics/space cascade: space programme behind schedule → electronics plant storms → more copper/component demand → rail dispatch increases → consumer output falls → other factories lose components → rail congestion increases → precision components arrive late → space programme storms even harder.

**Win-less success condition:** Space targets met without collapsing the civilian economy.

**What it teaches:** Priority cannot solve scarcity. The cascade. Temporal demand management (storming vs. steady flow).

**Dependencies:** Multi-tier priority freight, electronics/component supply chain, closed settlement, construction at remote site. **Requires the full dispatch priority system, the extended resource tree, and remote settlement logistics.**

---

### MODE 10: National Project — Mobilization

**Premise:** External threat. Convert civilian industry to military production while maintaining the home front.

**Starting state:** A functioning peacetime republic. Military quotas arrive and escalate.

**Rule changes:** Factory conversion (civilian recipes → military recipes at reduced efficiency during transition). Tiered rationing by labor category. Industrial evacuation (disassemble, transport, reassemble a factory). Priority classes reversed (military > civilian).

**Pressure source:** Military quotas increase while civilian capacity shrinks. Rationing creates civilian hardship. Worker fatigue from overtime. The player manages guns vs. butter as a physical tradeoff.

**Win-less success condition:** Military quotas met, civilian population survives (not thrives). Never game over — but quality of life can degrade severely.

**What it teaches:** The real cost of mobilization is civilian welfare. Factory conversion is a logistics project, not a button press.

**Dependencies:** Factory conversion mechanic, tiered rationing, industrial relocation/evacuation logistics, military production recipes. **Among the most expensive modes.**

---

### MODE 11: National Project — Housing Campaign

**(See Mode 01 above — The Housing Campaign IS the housing national project.)**

---

### MODE 12: Frontier Corridor

**Premise:** Build the BAM: a railway through remote territory, establishing settlements along the route.

**Starting state:** One end of a long, narrow, undeveloped map with a rail connection. No infrastructure between.

**Rule changes:** Construction is the primary activity, not production. Workers are recruited from elsewhere and housed in temporary settlements. Supply chains run along the partially-completed railway itself.

**Pressure source:** The railway must advance on schedule. But advancing requires construction materials, which must be transported along the incomplete railway. A logistics bootstrap problem.

**Win-less success condition:** Railway connects both ends. Settlements along the route are self-sustaining.

**What it teaches:** Infrastructure as a logistics bootstrap. The supply chain for building the supply chain.

**Dependencies:** Rail construction, temporary/permanent settlement, long-distance freight, construction material supply. **Cheap as a scenario because it uses existing rail + freight + construction; the linear map constraint is the mode.**

---

### MODE 13: Closed City

**Premise:** Build and manage a secret settlement (Baikonur/Leninsk, a nuclear city). Priority resource access, closed perimeter, no information leakage.

**Starting state:** Empty site near a strategic asset. Budget and priority access.

**Rule changes:** Priority allocation class. Closed perimeter (no spontaneous migration). All workers assigned, not recruited. Consumer allocation by state, not by queue.

**Pressure source:** The strategic asset demands sustained output. Workers cannot leave but also cannot be replaced easily. Morale matters because there is no exit valve.

**Win-less success condition:** Strategic output targets met, worker retention stable, settlement self-sustaining.

**What it teaches:** Priority allocation creates bubbles of abundance that are fragile to disruption.

**Dependencies:** Priority allocation, worker assignment (not market-based), closed settlement logic. **Medium cost — mainly needs priority allocation parameters.**

---

### MODE 14: Late-System Maintenance

**(See H-20 validation above.)**

**Rule changes:** All buildings and infrastructure have age and condition. Maintenance requires workers, materials, and equipment. Deferred maintenance accelerates decay.

**Pressure source:** Everything ages simultaneously. The player cannot maintain everything. Triage is the gameplay.

**Win-less success condition:** System does not collapse. Critical infrastructure maintained. Graceful degradation of non-critical systems.

**What it teaches:** The 1980s Soviet maintenance crisis. Growth is easy; sustaining is hard.

**Dependencies:** Building/infrastructure age and condition system, maintenance resource requirements, decay mechanics. **Does not exist in code — buildings have no age or condition.**

---

### MODE 15: International Plan

**Premise:** Coordinate economic plans across multiple socialist republics. Trade agreements, specialization, and mutual dependency.

**Rule changes:** Multiple planning authorities with distinct interests. Trade agreements as quota commitments. Specialization creates mutual dependency.

**Pressure source:** Other republics may not deliver on commitments. The player must build resilience against unreliable partners while maintaining the benefits of specialization.

**Dependencies:** Multiple economic actors with autonomous planning, trade agreements, specialization mechanics. **Very expensive — essentially requires AI-driven neighboring economies.**

---

### MODE 16: Enterprise Director

**Premise:** You ARE the enterprise director, not the Planner. You receive quotas from above and must meet them while managing workers, supplies, and the enterprise's welfare obligations. Your goal is to hoard, bargain, and survive — the exact behavior the Planner is trying to catch.

**Rule changes:** The core loop is INVERTED. The player optimizes enterprise behavior against Plan constraints. An AI Planner issues quotas, allocates inputs, and inspects results.

**Pressure source:** Quota ratcheting. Input unreliability. Quality demands. Worker needs. The player must decide how much to request, how much to hoard, when to storm, and what to report.

**Win-less success condition:** Enterprise survives and workers are reasonably well-off. But "winning" means the system as a whole is worse — the player learns why the hoarding spiral exists.

**What it teaches:** Why enterprises behave dishonestly. This is the most pedagogically valuable mode — but also the most architecturally risky.

**Dependencies:** AI Planner, quota/allocation system from the Planner's side, enterprise-level management UI, worker management. **Structurally a different game. The AI Planner is the hardest piece — it must be good enough that gaming it feels like gaming a real bureaucracy, not exploiting bad AI.**

---

### 3.2 Mode dependency graph over base systems

```
Base systems (lanes A/B1/D mechanisms):
  [A] Material allocation, quota/plan system, enterprise request/stock
  [B1] Citizens, housing queue, labor, daily schedule, needs
  [D] Freight dispatch, rail, truck, routing, congestion

Mode dependencies:

  Taut Plan ────────── [A] only (+ quota enforcement)
  Shortages Amid Plenty ── [A] + [B1] + [D]
  Everyday Socialism ─── [B1] (+ retail delivery from [D])
  Frontier Corridor ──── [D] + construction + [B1-lite]
  Monotown ────────────── [A] + [B1] + [D] + enterprise welfare
  Housing Campaign ────── [A] + [B1] + construction competition
  Sovnarkhoz ──────────── [A] + territorial subdivision (NEW)
  The Reform ──────────── [A] + indicator system (NEW)
  Self-Management ─────── [A-modified] + enterprise autonomy (NEW)
  Closed City ─────────── [A] + [B1] + priority allocation (NEW)
  Late-System Maintenance ── [A] + [D] + building age/condition (NEW)
  Frontier Corridor ────── [D] + construction + temporary settlement
  Science City ────────── [B1] + education + research system (NEW, EXPENSIVE)
  Space (National Project) ── [A] + [D] + priority freight + extended resources
  Mobilization ────────── [A] + factory conversion + rationing (NEW)
  International Plan ──── [A] + AI foreign economies (NEW, VERY EXPENSIVE)
  Enterprise Director ──── AI Planner + enterprise management UI (NEW, DIFFERENT GAME)
```

### 3.3 Cheapest three modes to ship first

Given what the code has today (`Market` with `request_multiplier`, `BuyFood`/`Home`/`Work` desires, freight dispatch, rail, truck routing):

1. **The Taut Plan** — Cheapest. Requires only quota targets (a number per resource per plan period) and enforcement (compare production to quota). The `request_multiplier` in `recipe_init` (`goods_company.rs:24`) already enables enterprises to over-request. Setting it to 1.0 and quotas to 100% of capacity creates the mode.

2. **Frontier Corridor** — A scenario, not a rule change. A long narrow map with rail at one end, construction objectives along the route. Uses existing rail, freight station, construction, and settlement systems. The mode IS the map.

3. **Everyday Socialism** — A scenario zoom into the citizen loop. Uses existing `BuyFood`, `Home`, `Work` desires (`simulation/src/souls/desire/`). Needs retail delivery to work well (it partially does — the retail two-leg model is ratified). The mode IS the starting state: a functioning mikrorayon where the player manages daily life.

## 4. Missed / not apparent

### 4.1 Scenario vs. mode distinction

The conversation lists fifteen "game modes" without distinguishing between a *scenario* (fixed starting state + map + brief) and a *mode* (rule variant with changed institutional parameters). This matters architecturally:

- "Frontier Corridor" is a **scenario**: same rules, different starting state and map.
- "Sovnarkhoz" is a **mode**: different rules (territorial allocation topology), applicable to any save.
- "The Housing Campaign" is **both**: a mode (housing quota mechanics) AND a scenario (specific starting state with overcrowding).

The codebase should have separate concepts: `Scenario` (starting state) and `Mode` (institutional parameter preset). A game can combine them: "Sovnarkhoz + Frontier Corridor" is a territorial planning mode on a linear map.

### 4.2 Mode transitions within a save

The conversation hints (lines 482–484) that the same save could transition through organizational modes. This is historically accurate — the USSR went from branch ministries (pre-1957) to sovnarkhozy (1957–65) to reformed branch ministries (1965+). But it creates a design question: are modes selected at game start, or can the player switch mid-save? If switchable, the parameter change is a gameplay event with consequences (institutional disruption, adaptation period). If fixed at start, each mode is a separate playthrough. The conversation does not resolve this. **Recommendation:** Modes should be switchable mid-save with a transition cost (institutional disruption period where allocation efficiency drops). This makes mode selection a strategic decision, not a menu choice.

### 4.3 Enterprise Director is a different game

Enterprise Director (H-09) inverts the core loop. The player IS the dishonest enterprise, not the Planner catching it. This requires:
- An AI Planner that issues quotas and inspects results
- An enterprise-level management UI (workers, supplies, welfare, reporting)
- A reward function that is orthogonal to the base game's (survive as enterprise vs. optimize the republic)

This is structurally closer to *Papers, Please* (a bureaucrat navigating an impersonal system) than to a city builder. Building it teaches nothing reusable for the base game — the AI Planner is the expensive piece, and it must be good enough to feel like a real bureaucracy. **Recommendation:** Enterprise Director should be a standalone expansion, not a base-game mode.

### 4.4 Multiplayer as organizational mode

The `networking/` crate at `/home/caio/soviet-simulator/networking/src/` is structurally complete: `client/`, `server/`, `authent.rs`, `catchup.rs`, `connections.rs`, `packets.rs`, `worldsend.rs`. Frame-synchronized inputs via `Frame(u64)`, `PlayerInput`, `MergedInputs`. Authentication via `AuthentID`.

A multiplayer organizational mode is conceivable: one player is Gosplan (sets quotas, allocates resources), another is a ministry (manages enterprises in one sector), another is a factory director. This maps to the conversation's organizational modes — but requires role-separated authority over the same simulation, which the `WorldCommand` enum does not currently support (all commands are equivalent; there is no "this player can only issue commands to enterprises in the steel sector").

**Recommendation:** Add a `Role` field to `WorldCommand` or filter commands by role at the server. This is a natural extension of the multiplayer crate and creates a unique game mode no competitor offers.

### 4.5 Chronicle mode (Dwarf Fortress Legends)

The conversation mentions DF-like persistence (line 19) and causal history (lines 656–658). DF's Legends Viewer lets the player browse the world's history after (or during) play: civilizations rising and falling, notable figures, battles, constructions.

A "Chronicle" mode for this game would let the player browse the republic's causal history: which enterprises were dishonest, which shortages cascaded from what, which citizens moved where and why, which plan periods succeeded or failed. This requires the causal history system described at lines 656–658 (important transitions emit facts with parent-cause links). It is not a separate mode — it is a view into the simulation's history, available from any mode.

**Recommendation:** The chronicle is not a mode; it is a feature. But it is the feature that makes the modes narratively meaningful — without it, each mode is a puzzle, not a story.

### 4.6 The tutorial problem

The charter (line 49 of `charter-1.0.md`) requires: "Three authored plans on one continuous save, then procedural endless mode; the First Plan alone must teach a new player to play for two hours without outside help."

This is a mode design problem. The First Plan is a scenario (specific starting state) with a tutorial function (teach the dishonest enterprise loop through play). It must:
1. Start simple enough that a new player can act without instruction
2. Introduce the core loop (enterprise requests more than it needs) through observable consequences
3. Escalate naturally so the player discovers dispatch, allocation, and citizen needs
4. Transition smoothly into the Second Plan (which is authored but not tutorial)

W&R solves this with 18 separate tutorial maps, each teaching one mechanic. This is expensive and fragile (tutorials break when mechanics change). **Recommendation:** The First Plan should be a single continuous scenario where the tutorial is emergent: the player places buildings, and the dishonest enterprise behavior emerges naturally from the simulation. The HUD strip (charter: "an in-HUD onboarding strip") points the player at observable discrepancies.

### 4.7 Unlocking modes vs. all-available

Should modes be unlocked through progression (complete the base game → unlock Sovnarkhoz), or all available from the start? Progression creates motivation but gates content. All-available gives freedom but risks overwhelming new players.

**Recommendation:** The three authored plans are the progression. After completing them, all modes and endless mode unlock. This matches the charter's "three authored plans on one continuous save, then procedural endless mode." The authored plans teach the base game; modes are the replayability layer.

### 4.8 Victoria 3's pops are the closest analog to this game's citizens

Victoria 3 models pops (population groups) as the center of the economy: they work in buildings, consume goods, and their Standard of Living determines their needs. But V3 pops are statistical groups, not individuals. This game's citizens are persistent individuals with names, households, and histories. The individual-level simulation enables mechanics V3 cannot do: the player can follow one citizen through a shortage spiral, watch their commute lengthen, see their children enter school.

**Game mode implication:** Any mode that touches citizens (Everyday Socialism, Monotown, Closed City) benefits from this individual-level persistence in ways no competitor can match. This is the game's competitive advantage.

### 4.9 Factorio's logistics vs. this game's logistics

Factorio abstracts logistics into belt/bot/train networks with perfect information and no human labor. This game's logistics are physically simulated with imperfect information, human drivers, and congestion. The difference matters for modes: Factorio-style optimization ("find the optimal ratio") is not the gameplay. This game's optimization is "find the coordination failure" — the information gap between what enterprises report and what is physically happening.

### 4.10 Crusader Kings' dynasty persistence as a model for plan persistence

CK3 tracks dynasties across generations: legacies accumulate, succession changes the player character, history is persistent. This maps to this game's plan periods: each plan builds on the previous one's decisions, infrastructure, and mistakes. A "dynasty" in this game is the republic's institutional memory — the accumulated trust/distrust between the Planner and enterprises.

## 5. Cross-lane hooks

| Finding | Lane | What they must know |
|---|---|---|
| Modes are parameter presets over one simulation | **A (economy)** | The institutional parameters (quota regime, reserve rules, priority classes, information quality) must be exposed as configurable state, not hardcoded behavior |
| Scenario vs. mode separation | **E (code audit)** | `SimulationOptions` at `lib.rs:116` needs extension for mode parameters |
| Enterprise Director requires AI Planner | **A (economy)** | The dishonest enterprise loop must work in both directions: player-as-Planner catching enterprises, AND enterprise logic sophisticated enough to be the AI Planner |
| Tutorial is a mode design problem | **F (doc overlap)** | The charter's tutorial requirement is a game mode requirement |
| Multiplayer-as-mode | **C2 (architecture)** | The `networking/` crate supports frame-sync but `WorldCommand` has no role filtering |
| National Projects need priority freight | **D (physics)** | Dispatch priority classes do not exist |
| Citizen individual persistence enables unique modes | **B1 (society)** | The `PersonalInfo` struct (`human.rs:36`) has name, age, gender but no history/biography |
| Housing Campaign needs construction competition | **A (economy)** | Construction and production compete for the same materials — this requires material-bill tracking |
| Progression ladder has no substrate | **E (code audit)** | `Government` has only `money`; no plan periods, no institutional memory |
| Wartime rationing needs tiered allocation | **A (economy)** | Allocation by labor category, not by enterprise request |

## 6. Open questions for the user

1. **Should modes be switchable mid-save?** Historically they were (Sovnarkhoz → branch ministries). But mid-save switching requires institutional transition mechanics. The alternative is modes-at-start, which is simpler but less interesting.

2. **Should Enterprise Director be pursued at all?** It is the most pedagogically interesting mode but inverts the core loop and requires an AI Planner. Is this a base-game expansion or a standalone project?

3. **Should the three authored plans teach three different modes?** e.g., First Plan = base game, Second Plan = Taut Plan, Third Plan = Housing Campaign. This introduces modes through progression rather than menu selection.

4. **Is the 1.0 charter's "three authored plans" the progression ladder?** The conversation's fragile → integrated → sophisticated → national project maps naturally to Plan 1 → 2 → 3 → endless, but the charter does not specify what the plans contain.

5. **How much AI is acceptable?** Enterprise Director and Self-Management modes require enterprises with autonomous decision-making beyond the current `recipe_should_produce` / `recipe_act` loop. How sophisticated should enterprise AI be?

6. **Multiplayer: worth the investment?** The networking crate exists. A Gosplan-vs-Ministry multiplayer mode would be unique in the genre. But multiplayer adds testing, balancing, and maintenance burden.

## 7. Sources

### Web sources (verified 2026-08-28)
- [Sovnarkhoz — Wikipedia](https://en.wikipedia.org/wiki/Sovnarkhoz)
- [1957 Soviet economic reform — Wikipedia](https://en.wikipedia.org/wiki/1957_Soviet_economic_reform)
- [1965 Soviet economic reform — Wikipedia](https://en.wikipedia.org/wiki/1965_Soviet_economic_reform)
- [Kosygin Reforms — Encyclopedia.com](https://www.encyclopedia.com/history/encyclopedias-almanacs-transcripts-and-maps/kosygin-reforms)
- [Aleksey Kosygin — Britannica](https://www.britannica.com/biography/Aleksey-Nikolayevich-Kosygin)
- [Socialist self-management — Britannica](https://www.britannica.com/topic/socialist-self-management)
- [Yugoslavia: The Case of Self-Managing Market Socialism — AEA JEP 5:4](https://www.aeaweb.org/articles?id=10.1257%2Fjep.5.4.187)
- [The Life and Death of Yugoslav Socialism — Jacobin](https://jacobin.com/2017/07/yugoslav-socialism-tito-self-management-serbia-balkans)
- [Danwei system — Lvivcenter](https://www.lvivcenter.org/en/discussions/chinese-socialism/)
- [Corporate-Run Society: The Danwei — MDPI Sustainability](https://www.mdpi.com/2071-1050/12/4/1338)
- [Work unit — Grokipedia](https://grokipedia.com/page/Work_unit)
- [Soviet wartime factory evacuation — GlobalSecurity.org](https://www.globalsecurity.org/military/world/russia/industry-stalin-evacuation.htm)
- [Wartime Evacuation — Soviet History MSU](https://soviethistory.msu.edu/1943-2/wartime-evacuation/)
- [Baikonur Cosmodrome — Britannica](https://www.britannica.com/place/Baikonur)
- [Russia's Cosmos Town — Moscow Times](https://www.themoscowtimes.com/2021/12/09/russias-cosmos-town-an-isolated-relic-of-soviet-glory-a75772)
- [Akademgorodok — Britannica](https://www.britannica.com/place/Akademgorodok)
- [Akademgorodok — Wikipedia](https://en.wikipedia.org/wiki/Akademgorodok)
- [BAM railway — Rusmania](https://rusmania.com/history-baikal-amur-railway-bam)
- [BAM — Soviet History MSU](https://soviethistory.msu.edu/1980/bam/)
- [BAM — Encyclopedia.com](https://www.encyclopedia.com/history/encyclopedias-almanacs-transcripts-and-maps/baikal-amur-magistral-railway)
- [Khrushchyovka housing — GW2RU](https://www.gw2ru.com/history/2866-khrushchyovka-apartment-building)
- [Khrushchevka — Grokipedia](https://grokipedia.com/page/Khrushchevka)
- [Khrushchyovka — HiSoUR](https://www.hisour.com/data/khrushchyovka/)
- [CIA Paper 251: Soviet economic outlook](https://history.state.gov/historicaldocuments/frus1981-88v03/d251)
- [Transport in the Soviet Union — Wikipedia](https://en.wikipedia.org/wiki/Transport_in_the_Soviet_Union)
- [Victoria 3 pops — Medium](https://medium.com/@pabloGordilloSanchez/victoria-3-a-game-of-pops-eada5ea3a750)
- [Victoria 3 Market — Paradox Wiki](https://vic3.paradoxwikis.com/Market)
- [Deep Dive: Victoria 3 economy — Game Developer](https://www.gamedeveloper.com/design/deep-dive-modeling-the-global-economy-in-victoria-3)
- [Anno 1800 production chains — Fandom](https://anno1800.fandom.com/wiki/Production_chains)
- [Anno 1800 supply chains — Chill Place Gaming](https://chillplacegaming.com/anno-1800-supply-chains/)
- [Factorio logistics — Shapes](https://shapes.inc/fandom/factorio/automation-logistics)
- [Dwarf Fortress modes — The Gamer](https://www.thegamer.com/dwarf-fortress-mode-differences/)
- [Dwarf Fortress emergent narrative — ResearchGate](https://www.researchgate.net/publication/356686095_Characterization_and_Emergent_Narrative_in_Dwarf_Fortress)
- [Legends — Dwarf Fortress Wiki](http://dwarffortresswiki.org/legends)
- [Frostpunk 2 endings — ProGameGuides](https://progameguides.com/frostpunk-2/all-endings-in-frostpunk-2/)
- [CK3 Dynasty Legacies — The Gamer](https://www.thegamer.com/crusader-kings-3-best-strongest-dynasty-legacies/)
- [W&R: Soviet Republic — Wikipedia](https://en.wikipedia.org/wiki/Workers_%26_Resources:_Soviet_Republic)

### Code files referenced
- `simulation/src/lib.rs` — `Simulation` struct, `SimulationOptions`, `SoulID` enum
- `simulation/src/economy/government.rs` — `Government` struct (only `money: Money`)
- `simulation/src/economy/market.rs` — `Market`, `SingleMarket`, `Dispatch`
- `simulation/src/economy/mod.rs` — `market_update`, trade matching
- `simulation/src/souls/goods_company.rs` — `recipe_init`, `recipe_should_produce`, `recipe_act`, `request_multiplier`
- `simulation/src/souls/human.rs` — `PersonalInfo`, `HumanDecision`, `HumanDecisionKind`
- `simulation/src/souls/desire/` — `BuyFood`, `Home`, `Work`
- `simulation/src/world_command.rs` — `WorldCommand` enum (no role filtering)
- `simulation/src/multiplayer/mod.rs` — `MultiplayerState`
- `networking/src/lib.rs` — `Frame`, `PlayerInput`, `MergedInputs`, client/server
- `native_app/src/gui/mod.rs` — `Tool` enum (8 tools: Hand, RoadbuildStraight/Curved, RoadEditor, Bulldozer, LotBrush, SpecialBuilding, Train, Terraforming)
- `native_app/src/gui/hud/windows/mod.rs` — `GUIWindows` (Economy, Settings, Load, Network)
- `native_app/src/gui/inspect/mod.rs` — inspect building, human, vehicle, train
- `docs/plan/charter-1.0.md` — 1.0 scope and cuts
- `docs/reference/glossary.md` — binding definitions
- `docs/plan/proposals/gosplan.md` — process framework proposal

### W&R reference install
- `~/.local/share/Steam/steamapps/common/SovietRepublic/media_soviet/` — campaigns (1–3), scenarios, 18 tutorial maps, research tech tree (~200 items), people.ini (13 ministry officials)
