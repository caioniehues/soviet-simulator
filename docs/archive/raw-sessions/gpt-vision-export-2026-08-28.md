# Soviet Simulator — Conversation Export

**Export date:** 2026-08-28  
**Format:** Markdown

> **Note on completeness:** This export contains the user/assistant project discussion available in the current conversation context. Hidden system/developer instructions and private reasoning are excluded. Some earlier portions are available to me only as retained conversation context rather than a verbatim transcript; those sections are explicitly labeled **reconstructed context** rather than presented as exact quotations.

---

# 1. Earlier project discussion — reconstructed context

The project is `caioniehues/soviet-simulator`, a long-term Rust city/infrastructure/logistics/society simulation centered on a socialist planned economy. The current mainline is a hard fork of Egregoria after a discarded Bevy prototype.

The long-term vision is:

- Cities: Skylines II-quality urban design and scale.
- Workers & Resources: Soviet Republic-style physical planned-economy causality.
- Dwarf Fortress-like persistence, causality, and simulation depth.
- Persistent citizen identities at very large scale.

Important design principles already established:

- Player is **THE PLANNER**.
- The Plan is a sequence of quota periods on one continuous save.
- Domestic allocation is non-price-based.
- Border roubles exist only for foreign trade/customs.
- Goods move physically or do not move.
- Failure appears as queues, shortages, substitution, cold homes, missed service, and going without.
- Buildings are planned, physically constructed, and do not auto-spawn from zones.
- Public transit is culturally/economically dominant; private cars emerge from citizen/household mobility needs rather than being automatic.
- Industrial logistics is a central pleasure loop and visual spectacle.
- Core rule: **Automate execution, not decisions.**

Binding 1.0 ambition includes 250,000 persistent citizen identities at 60 fps on the development machine.

Major architecture conclusions from earlier discussion:

- Split persistent `CitizenRecord` from active `CitizenBody`.
- Use cadence bands rather than one universal tick frequency.
- Deterministic phase order:
  `COMMAND → TOPOLOGY → ALLOCATION → DECISION → ROUTING → MOVEMENT → ARRIVAL → PRODUCTION → UTILITIES → ACCOUNTING`.
- Parallelize inside phases using deterministic intent buffers and merge/commit.
- Decompose inherited Market responsibilities into demand, allocation, inventory reservation, logistics/fulfillment, retail provisioning, and border trade.
- Use hierarchical routing and topology/traffic caches.
- Share topology infrastructure across utilities, but keep domain physics distinct.
- Build a first-class causal inspector: **STATUS / CAUSE / TREND / POLICY / PHYSICAL CHAIN**.
- Preserve physical enterprise request/receipt/consumption/surplus distinctions.
- Do not store a hidden `dishonest` enterprise flag; let the Planner infer strategic behavior from observable discrepancies.
- Treat National Projects as temporary nationwide distortions of the material economy.
- Space exploration fits best as an industrial/logistics national project, not as a KSP-style flight simulator.
- Mobilization/war production fits best as a home-front material economy with physical guns-vs-butter tradeoffs, not tactical combat.

Major historical/gameplay mechanisms already identified:

- Material-balance planning.
- Enterprise bargaining and concealment of capacity.
- Over-requesting and precautionary hoarding.
- “Shortages amid plenty” from delivery unreliability.
- End-period storming / *shturmovshchina*.
- Assortment versus aggregate output targets.
- Quality lots and rework.
- Strategic reserves.
- Physical opportunity-cost reporting.
- Approved substitutions.
- Industrial ecology and byproduct chains.
- Human industrial chains: education → housing → transport → labor stability → production.
- Space logistics, cosmodrome settlements, project cargo, orbital resupply.
- Mobilization economy and factory conversion.
- Housing as labor infrastructure.
- Mikrorayon completeness.
- Household time as a scarce resource.
- Household plots as a food-system buffer.
- Informal networks/blat as an alternative allocation topology.
- Education → qualification → assignment → relocation.
- Social reproduction as an economic system.

Long-term progression concept:

> **fragile local economy → integrated industrial republic → sophisticated planned economy → state capable of executing enormous national projects**

---

# 2. Current conversation transcript/context

## User

> very good!! I really loved how you went deep into material historial data, citing papers and cia. LETS GOO EVEN DEEPER NOW: how granular and detailed and accurate and polished should our resources, logistics, supply chains, etc, be? as much as our own game engine allows? also do a deep dive on this core concept more electronics requested
>         ↓
> electronics plant storms
>         ↓
> more copper/component demand
>         ↓
> rail dispatch increases
>         ↓
> consumer radio/appliance output falls
>         ↓
> other factories lose components
>         ↓
> rail congestion increases
>         ↓
> precision components arrive late
>         ↓
> space programme storms even harder

## Assistant

The key correction was that **“as much detail as the engine allows” is the wrong upper bound**.

The recommended rule became **causal distinctness**:

> Split a resource, lot, vehicle operation, production stage, quality class, or institutional state only when the distinction can change routing, storage, substitution, bottlenecks, allocation priority, quality, timing, or visible consequences.

This lets the game be more physically accurate than W&R where the distinctions matter, without becoming “Copper Bolt Type 7 Simulator.”

---

## User

> I mean how the planned economy would generate and manifest its own challenges and mechanics as well integrated into all our systems

## Assistant

The planned economy should not sit above the game as a quota UI. It should **deform the physical systems themselves**.

Factories alter requests. Managers protect reserves. Ministries push priority freight. Workers get reassigned. Maintenance slips. Construction competes for materials. Rail demand bunches near deadlines. Consumer shortages become the downstream shadow of strategic choices.

> **Planning is not a layer on top of logistics. Planning is one of the forces that deforms logistics.**

---

## User

> proceed ! finish your great response

## Assistant — planned economy as endogenous gameplay

The deeper thesis became:

> **The Planner changes incentives and priorities. Enterprises react. Those reactions change physical demand. Physical demand changes logistics. Logistics changes reliability. Reliability changes enterprise behavior. Enterprise behavior changes the information on which the next Plan is built.**

This closes the loop.

### Planned economy as a control system

```text
                         ┌──────── reports ─────────┐
                         │                           │
                         ↓                           │
                     THE PLAN                        │
                         │                           │
              quotas / allocations                  │
                         ↓                           │
                    enterprises                     │
                  ↙      ↓       ↘                  │
              request   produce   hoard              │
                 ↓        ↓        ↓                 │
                 └──── logistics ──┘                 │
                         ↓                           │
                    actual flows                     │
                         ↓                           │
              shortages / surplus / queues          │
                         ↓                           │
                   observed results ─────────────────┘
```

The critical mechanic is that **reported need is not true need**.

A plant technically requiring 100 t of copper might request 145 t because of:

- reliability buffer,
- expected allocation cuts,
- plan-risk buffer,
- reserve building,
- bargaining slack.

This can create a self-generated shortage spiral:

```text
SUPPLY IS UNRELIABLE
        ↓
enterprises request extra stock
        ↓
enterprises receive precautionary reserves
        ↓
available central stock falls
        ↓
other enterprises experience shortages
        ↓
those enterprises increase requests
        ↓
dispatch pressure rises
        ↓
rail / docks / trucks overload
        ↓
deliveries become less reliable
        ↓
SUPPLY IS EVEN MORE UNRELIABLE
```

### Electronics / space cascade

The Space Programme can be behind schedule. The electronics plant responds through overtime, maintenance deferral, larger batches, reserve releases, substitutions, and larger input requests.

Upstream suppliers then receive a demand shock.

A core rule emerged:

> **Priority cannot solve scarcity. Priority only decides where scarcity appears.**

Strategic copper going to space electronics means less copper for consumer electrical production, machine tools, telecommunications, or construction.

The railway then feels the Plan. More emergency freight consumes finite locomotives, wagons, loading tracks, yards, dispatch windows, and unloading docks.

### Freight-plan stability

Two republics can move the same annual tonnage while requiring radically different transport capacity if one flows steadily and one storms at period end.

```text
Average corridor utilization      72%
Peak utilization                 141%
Plan-period variance             HIGH
Emergency dispatch share          24%
Empty repositioning               18%
```

A player may think “I need another railway,” while the real problem is:

> **Your industrial plan produces terrible temporal demand.**

Storming can recursively propagate upstream:

```text
Space programme storming
        ↓
electronics storming
        ↓
wire plant storming
        ↓
copper processing storming
        ↓
mine dispatch pressure
        ↓
rail congestion
```

The congestion propagates forward again as late precision inputs.

Quality can also deteriorate under storming, creating rework and extra input demand.

### Ratchet effect

If exceptional performance becomes the basis for the next quota, enterprises learn not to reveal spare capacity.

The player can accidentally train enterprises not to tell the truth.

### Planning credibility

Instead of a generic loyalty stat, institutions can remember practical reliability:

```text
Requested inputs usually received:          61%
Promised delivery dates met:                67%
Emergency requisitions honored:             82%
Overfulfillment translated into new quota:  91%
Plan changed mid-period:                    frequent
```

If the Planner always cuts requests, institutions inflate them.
If reserves are repeatedly confiscated, institutions hide reserves.
If exceptional performance raises future quotas, managers conceal capacity.
If delivery becomes reliable, safety stocks shrink.

The player can therefore build **institutional trust through physical competence**.

### Slack as resilience

A nominally less aggressive Plan can outperform a taut one because it has room to absorb shocks.

### Multiple reserve purposes

- operating stock,
- safety stock,
- enterprise reserve,
- state reserve,
- project reserve.

### Specialization vs resilience

A locomotive works with unreliable bearing deliveries may create an inefficient local bearing workshop because reliability has value.

### Indicator design

“What you measure becomes what enterprises optimize.”

Gross tonnage, assortment, quality, freight tonnage, priority classes, etc. can each produce different distortions.

### Construction opportunity cost

Prioritizing housing can reduce machine-building capacity later. Prioritizing heavy industry can worsen housing and labor availability.

### Citizens deepen the loop

Strategic projects pulling engineers from other sectors also create demand for housing, food, heating, schools, health care, transit, and service workers.

The true labor footprint of a project is much larger than its direct technical workforce.

### Material-balance UI

A national material balance should show physical stock, reserves, in-transit quantities, production, consumption, allocation, and requested demand.

Every aggregate number should be clickable down to real trains, yards, warehouses, factories, and households.

### Golden rule

> **Every macroeconomic number must eventually resolve into physical or institutional state.**

The late game becomes optimization of **coordination quality**, not merely bigger production numbers.

---

## User

> cool!! now do a dedicated deep research on socialist/soviet things related/pheasable technically in our game and expand them in detail

## User

> cool research wide aspects and characteristics of theirs societies, then explore new, possible and not so clear on first view game modes

## User

> also citizen simulation deep dive granular machanica

## Assistant — socialist society / citizen simulation research

The major new conclusion was that the game can simulate **social reproduction**, not only industrial production.

```text
PLAN
 ↓
INDUSTRIAL PRODUCTION
 ↓
goods + housing + services + infrastructure
 ↓
HOUSEHOLD LIFE
 ↓
health + education + time + migration + family formation
 ↓
LABOUR FORCE
 ↓
enterprise capacity
 ↓
PLAN
```

### Enterprise as miniature welfare state

An enterprise can include production plus labor plus welfare infrastructure:

```text
FACTORY
│
├ PRODUCTION
│  machines / materials / power / logistics
│
├ LABOUR
│  workers / skills / shifts
│
└ WELFARE
   housing / dormitory / childcare / clinic
   canteen / culture / transport
```

Housing, childcare, transit, education, and healthcare therefore become industrial infrastructure.

### Social cost of industrialization

A plant employing 8,000 workers may require a settlement of ~20,000 people after households and supporting services are counted.

### Monotown gameplay

Housing shortage → overcrowding → turnover → vacancies → production shortfall.

Factory underperformance can also delay housing construction, which worsens worker retention.

### Housing allocation

Housing should be a persistent non-price queue. Households retain identity through shortage, displacement, or overcrowding.

Housing can be used as a labor-recruitment instrument.

### Mikrorayon completeness

A housing Plan can succeed numerically while daily life fails because schools, shops, transit, clinics, and heating lag.

### Time as a citizen resource

Citizens and households spend finite time on:

- sleep,
- work,
- commute,
- shopping,
- queueing,
- childcare,
- domestic work,
- healthcare,
- household food production,
- leisure.

Shortage therefore creates time costs even when a good is eventually obtained.

### Household labor and childcare

Care and domestic work should follow household structure, schedules, period-specific norms, and actual availability.

Childcare capacity can release real adult labor-hours.

### Household plots

Allotments/dachas can buffer formal food failures at the cost of household labor, transport, tools, seed/feed, and leisure.

### Citizens adapt

Citizens should seek substitutes, alternate stores, queues, social contacts, walking routes, household sharing, temporary overcrowding, or new employers rather than merely accumulating red need bars.

### Informal networks / blat

Sparse social ties can become an alternate allocation topology. Informal access moves actual physical goods and therefore displaces someone else's access.

### Inequality without wealth classes

Different lived outcomes can emerge from:

- workplace,
- housing allocation,
- geography,
- service access,
- useful social contacts,
- scarce qualifications,
- institutional privilege.

### Labor shortage

A socialist economy can face scarcity of workers rather than scarcity of jobs.

Solutions include education, housing, migration, transit, childcare, mechanization, reallocation, and retention.

### Qualifications and life course

Citizens retain meaningful histories: birth, school, technical education, qualification, employment, household formation, housing queues, relocation, children, death.

### Citizen architecture

Persistent `CitizenRecord`; expensive `CitizenBody` only when physical movement requires it.

### Queues as first-class scarcity objects

Food, clinics, schools, and housing can share the same conceptual queue primitive at different time scales.

### Queue burden

Measure human time lost to scarcity instead of collapsing everything into “happiness.”

### Science cities / closed cities / priority geography

Strategic settlements can concentrate scarce specialists, housing, service provision, consumer allocation, and infrastructure at physical opportunity cost elsewhere.

### Migration

Citizens can respond to housing opportunities, jobs, climate, services, and administrative eligibility.

### Capital dilution

Too many construction projects can lock huge stocks of steel, concrete, machinery, and labor inside unfinished assets.

### Innovation conflict

A better machine may be technically desirable but risky to install under a taut current quota.

The Planner may need to create deliberate innovation windows.

### Organizational modes

Future campaigns/systems can explore branch ministries, territorial planning, self-management, reform-socialist enterprise autonomy, danwei-like work units, and rationing regimes over the same physical economy.

### Non-obvious future game modes

- The Housing Campaign
- The Monotown
- Science City
- Shortages Amid Plenty
- The Taut Plan
- The Reform
- Sovnarkhoz
- Everyday Socialism
- National Project
- Frontier Corridor
- Closed City
- Late-System Maintenance
- Self-Management
- International Plan
- Enterprise Director

The Enterprise Director mode is especially interesting because it lets the player experience why subordinate managers hoard and bargain.

### Citizen-to-nation causal ladder

A national steel shortfall can resolve into worker turnover, housing delays, construction-material delays, rail congestion, and National Project freight priority.

### Core conclusion

> **Physical social reproduction** can be as important as physical industrial logistics.

Workers must exist, be born, educated, housed, fed, heated, transported, kept healthy, and given enough time to live.

---

## User

> nice !! now do a deep research on rust language features/architectures/crates/modules that will allow us to simulate all of this in a really unique and surprising manner

## Assistant — Rust architecture research

The key question became:

> **How can Rust let us represent different parts of society at radically different levels of cost, while keeping one coherent deterministic physical reality?**

Recommended direction: a **deterministic, multi-resolution, causal simulation architecture** with spatial, temporal, semantic, historical, and informational LOD.

### Three worlds

```text
PHYSICAL WORLD
 actual truth
    ↓ observation
INSTITUTIONAL WORLD
 enterprise reports / requests
    ↓ reporting
PLANNER WORLD
 what the Planner can know
```

The UI should receive a `PlannerSnapshot`, not unrestricted access to `Simulation`.

Rust module privacy can enforce imperfect information.

### Type system as physics enforcement

Use typed IDs and unit/state newtypes where they prevent real bugs:

```text
CitizenId
HouseholdId
HaulId
ConsumptionId
Mass
Volume
Energy
Power
Currency
OnHandQty
ReservedQty
CustodyQty
```

### Persistent vs ephemeral identity

Persistent social identity should use stable dense typed indexes.

Ephemeral physical embodiments/vehicles can use generational slot-map identities.

### SoA citizens

A future `CitizenStore` should use SoA or equivalent dense columnar storage for large hot state.

### Temporal LOD

Nothing relevant changed → do not simulate it.

Use scheduled wake-ups rather than evaluating every citizen every frame.

### Deterministic event calendar

A custom timing wheel / monotone queue is attractive.

Avoid async-task-per-citizen architecture because explicit state machines are easier to serialize, inspect, replay, hash, and version.

### Semantic LOD

Keep resources aggregated until a causal distinction such as quality, reserve status, custody, or destination actually matters.

### Fixed resource arrays

With a small fixed resource catalogue, dense arrays can replace per-holder hash maps.

### Integer/fixed-point authoritative state

Use deterministic conserved quantities where appropriate.

### Deterministic parallelism

Global deterministic phases; parallel compute inside each phase.

Pattern:

```text
parallel compute
→ intent buffers
→ deterministic merge
→ stable sort
→ authoritative commit
```

### Typed system contexts

Replace `&mut Simulation` with narrow capability contexts that expose only the domains a system is allowed to mutate.

### Keyed randomness

Use stable random keys derived from seed + domain + entity + event index rather than one global mutable RNG stream.

### Bitset society

Large cohort queries become intersections over dense/sparse bitsets before expensive per-citizen logic is run.

### Heterogeneous data structures

```text
dense arrays        authoritative state
bitsets             cohort membership
spatial grids       physical proximity
graphs              infrastructure
event calendar      future activity
causal DAG          explanation
incremental queries Planner knowledge
```

### Incremental Observatory

The authoritative simulation emits a Change Journal.

A derived/incremental layer maintains:

- material balances,
- labor balance,
- shortage propagation,
- queue statistics,
- service coverage,
- enterprise discrepancy analysis,
- causal explanations.

Salsa was identified as an interesting prototype for the derived world, not the physical simulation core.

Differential Dataflow was identified as a more experimental option for advanced incremental relational/economic analysis.

### Causal history

Important transitions can emit facts with parent-cause links.

Recent history remains detailed; old routine history compresses.

### Shadow simulation / Gosplan Computer

Determinism enables branching headless forecasts from Planner-visible information.

Forecasts can be wrong because they use reported state rather than hidden physical truth.

### Mathematical feasibility analysis

LP/MILP can analyze whether the reported Plan is materially feasible, while the physical simulation still produces actual outcomes with logistics, information, labor, and timing problems.

### Network architecture

Share compact topology machinery; keep Water, Sewage, Heat, Electricity, Gas, etc. as distinct solvers.

### Snapshots

Publish separate immutable Planner, Render, Audio, and Debug snapshots.

### SIMD / memory discipline / save format / determinism testing

Use contiguous layouts first, explicit SIMD only after profiling.

Compile-time size assertions can protect hot structs from accidental bloat.

Use stable versioned release saves separately from fast internal snapshots.

Use canonical deterministic hashes and property-based testing for conservation/state-machine correctness.

### Final architecture rule

> **What is the cheapest representation that preserves every causal distinction the game cares about?**

---

## User

> cool now do a deep dive on the vehicle, collision, physics, inertia, transportation traffic, water, fluids, sewage, electricity, gas , etc etc it’s unique possibilities and insights deep research

## Assistant — vehicles, traffic, utilities, physical inertia

The major conclusion became:

> **Do not model infrastructure as generic capacity networks. Model the different forms of inertia that make each network behave differently.**

A truck has mechanical inertia.
A road has queue inertia.
A railway has braking-distance/timetable inertia.
Water has pressure and tank storage.
Sewage has gravity/backpressure.
Heating has transport delay and thermal mass.
Electricity has near-instant balancing needs.
Gas has linepack.
Reservoirs have long-term stored-water memory.

### Vehicles

Use specialized lane-constrained physics instead of generic rigid-body simulation.

Model:

- lane,
- longitudinal position,
- speed,
- acceleration,
- length,
- mass,
- power,
- traction,
- braking capability,
- cargo,
- route.

Loaded and empty trucks behave differently on slopes.

Road grade becomes an industrial logistics variable.

Jerk limits improve polish cheaply.

### Collision

Collision should primarily mean collision avoidance.

IDM/MOBIL-style models are good conceptual references.

A true overlap becomes a recoverable obstruction/invariant failure rather than a random destructive accident system.

### Traffic

BPR/Gawron routing cost is useful, but ultimate traffic should also represent real queues and **spillback**.

Downstream blockage can physically occupy links and block upstream junctions.

Industrial gates and docks can create public-road congestion.

### Hybrid micro/meso traffic

Use mesoscopic lane flow/cell models for cheap network-scale queue propagation and richer microscopic motion where individual movement matters.

Important freight vehicles keep exact identity/custody.

### Shift changes

Factory shifts create passenger waves.

Shift staggering becomes a planning mechanic that can reduce peak demand without new infrastructure.

### Public transport

Trams/trolleybuses/buses should have boarding throughput, dwell time, crowding, headways, bunching, and eventually power-system coupling.

### Rail

Rail should model:

- consist mass,
- length,
- locomotive power,
- traction,
- braking,
- grade,
- track occupancy.

Train length affects siding/loading compatibility.

Braking/headway/junction conflicts determine capacity.

Empty-wagon repositioning is real traffic.

Rail yards can become logistics processors.

### Water

Do not use Navier–Stokes.

Use an EPANET-like network abstraction:

- nodes,
- pipes,
- pumps,
- tanks,
- reservoirs,
- pressure/head,
- finite demand,
- optional quality.

Connection is not the same as adequate pressure.

High-rise floors can experience different service.

Water tanks create delayed resilience.

Pump power couples Water and Electricity.

### Sewage

Model:

- gravity,
- slope,
- finite conduits,
- buffers,
- pumps,
- treatment,
- backpressure.

Downstream saturation can propagate upstream similarly to traffic spillback.

### Heating

Heat has transport delay.

Pipes/network water and buildings themselves store thermal energy.

A coal supply disruption can take hours before apartments get cold because reserves and thermal inertia absorb it first.

### Electricity

Electricity has the opposite behavior: imbalance matters quickly.

Generators can eventually have min/max output, ramp rates, startup state, and reserve contribution.

Explicit priority load shedding is preferable to generic blackout modifiers.

### Gas

Gas pipelines are compelling because of **linepack**: the pressurized pipeline itself stores gas.

A supply shortage can remain hidden while pressure falls, then produce a delayed service collapse.

Compressor stations become strategic energy-consuming infrastructure.

### Reservoirs / hydro

Use basin/reach/reservoir mass balance rather than full CFD.

Hydro depends on flow × head × efficiency.

Reservoir releases trade current power against future stored water.

### Weather

Snow/ice can change safe acceleration/braking/headway and therefore road capacity.

Snow clearing can become real vehicle logistics with priority corridors.

### Universal concept: network reserves

```text
Road         unused capacity
Rail         headway slack + spare wagons
Water        tank volume + pressure/pump reserve
Sewage       empty buffer + treatment headroom
Heating      network/building thermal reserve
Electricity  ramping/spinning reserve
Gas          linepack
Reservoir    stored water
Logistics    inventories + vehicles
Citizens     discretionary time
```

A system can be functioning but dangerously taut.

### Physical momentum

A good republic keeps working for some time after disruption because it has real buffers.

Fixes also propagate with delay.

The expert Planner manages **phase lag** and trajectories, not merely red icons.

---

## User

> nice! now do a deep research only on cia sources, use as many and quote ad many as possible. go deep into citizen and society simulation, do a deep dive on how socialist society behaved and go deep on how rust can simulate this amazingly in amazing mechanics and systems

## Assistant — CIA-only citizen/society research

This pass sharpened the citizen thesis dramatically.

The declassified CIA material repeatedly describes shortages and social conditions changing:

- time use,
- household labor,
- migration,
- fertility,
- job choice,
- worker turnover,
- informal exchange,
- health,
- social status,
- trust/reliability,
- production quality.

These effects feed back into the economy.

Important CIA-derived observations discussed included:

- consumer scarcity as daily-life management rather than an abstract need score;
- large shopping/search-time burdens;
- high female labor-force participation together with a disproportionate household workload;
- childcare tied to workplaces/residential districts;
- housing being used to attract workers and reduce turnover;
- priority projects using housing/services as recruitment privileges;
- living conditions influencing migration and possibly fertility;
- private household plots forming a major food-system buffer;
- private agriculture still depending on socialized-sector inputs;
- second-economy/informal access filling genuine provisioning gaps;
- informal/corrupt systems simultaneously undermining formal allocation and making daily life more workable;
- elite/priority access operating through housing, clinics, stores, sanatoria, etc.;
- education and graduate assignment tied to manpower planning;
- production norms being ratcheted upward after persistent overfulfillment;
- end-period storming creating overtime and quantity/quality distortions;
- reporting pressure creating incentives for false reporting;
- health policy being understood as economically productive;
- alcohol affecting productivity, absenteeism, health, family stability, and turnover;
- demographic cohort changes affecting future manpower;
- generational expectations shifting with urbanization, education, and higher living-standard aspirations.

### Citizen simulation thesis

Instead of one “happiness” number, citizens/households should be modeled through:

```text
LIFE
HOUSEHOLD
TIME
ACCESS
EXPECTATIONS
HISTORY
```

### Time poverty

Consumer shortages should cause search, travel, queueing, substitution, and lost household time.

```text
RETAIL SUPPLY FAILURE
        ↓
longer search
        ↓
longer queues
        ↓
household time burden
        ↓
fatigue / reduced discretionary time
        ↓
late arrivals / lower resilience
        ↓
labor performance
```

### Household scheduling

Households solve work, shopping, childcare, domestic labor, allotment work, health care, and leisure using actual schedules and family structure.

### Childcare as labor-supply transformer

Physical childcare attendance can release adult care time into formal labor.

### Housing as labor routing

Enterprises can compete for workers through housing eligibility, dormitories, transit, childcare, canteens, and cultural facilities even without a conventional labor market.

### Long-term fertility / labor consequences

Housing crowding, care burden, and living conditions can affect family formation slowly.

A shortage today can echo in the labor force decades later.

### Private plots as adaptive production

Formal food unreliability increases the value of household production, but household plots consume time and may depend on state-sector inputs.

### Informal economy as alternate allocation topology

Scarce goods acquired informally must come from real stock.

Informal networks redistribute shortages rather than creating resources.

### Sparse social graph / reciprocity

Citizens can maintain a small number of durable kin/work/neighborhood/favor ties.

Unmet needs wake the social network selectively.

### Access privilege / non-monetary inequality

Practical privilege can be represented as actual access channels rather than a single social-class variable.

### Education → qualification → assignment → relocation

Graduate assignments can create physical migration and household tradeoffs.

### Labor adaptation costs

Turnover matters because replacement workers need time to become fully productive.

### Labor hoarding

Enterprises can protect themselves against uncertainty by holding surplus workers, just as they hold surplus materials.

### Plan pressure enters worker life

Storming can produce overtime, sleep loss, fatigue, absenteeism, quality problems, family time loss, and future turnover.

### Health as production of future capacity

Healthcare, prevention, rest, childcare, housing, education, and transport can all be treated as investments in future human capability rather than generic welfare bonuses.

### Formal vs lived economy

Maintain separate views of:

```text
FORMAL PLAN
```

and

```text
LIVED ECONOMY
```

A Plan may look good administratively while households spend huge amounts of time searching and queueing.

### Four realities

```text
actual physical reality
reported institutional reality
Planner knowledge
household lived experience
```

These should not collapse into one database view.

Rust's module boundaries can enforce this.

### Citizen knowledge

Citizens should not know every shop's inventory. They act from remembered/local/social information.

A delivery can create:

```text
supply arrival
 ↓
information propagation
 ↓
crowd formation
 ↓
queue
 ↓
stock depletion
```

without scripted events.

### Cohort expectations

Different generations can slowly learn different expectations regarding housing, food reliability, mobility, and leisure.

### Social Reproduction Balance

A future district-level account can show:

```text
Potential adult hours
Formal employment
Commuting
Household care
Shopping/queues
Household production
Illness/absence
Residual discretionary time
```

This makes social policy materially measurable.

### Rust implementation model

- Dense persistent `CitizenCore`.
- Sparse side stores for rare states.
- Citizen materialization levels from identity record to rendered body.
- Separate citizen/household/institutional event scheduling.
- Exhaustive enums for state machines.
- Typed IDs for domain safety.
- Incremental Change Journal.
- Causal exposure indexes.
- Bitset filtering before expensive decision logic.
- Permanent major biography + recent detail + old aggregate history.
- Slow expectation memory.
- Institutional reliability estimates rather than generic loyalty.

### Generalized reliability → reserve mechanism

Factories, households, railways, and hospitals can all build buffers when they distrust future supply.

```text
unreliable system
→ defensive reserves
→ aggregate scarcity
→ worse reliability
```

Improving reliability can free huge quantities of physical stock and human time because actors no longer need to protect themselves so aggressively.

### Mature system as physical calm

A mature republic can have:

- fewer emergency requests,
- smaller safety stocks,
- shorter queues,
- predictable trains,
- lower turnover,
- more accurate reporting,
- less storming,
- more household discretionary time.

### Final CIA-derived principle

> **Citizens should be agents of adaptation.**

Factories adapt to unreliable supply.
Households adapt to unreliable provisioning.
Workers adapt to bad housing.
Young people adapt to perceived opportunity.
Managers adapt to quotas.
Informal networks adapt to allocation failures.
The state then adapts to their adaptations.

```text
PLAN
 ↓
society responds
 ↓
response changes reality
 ↓
Planner observes imperfectly
 ↓
new PLAN
```

The computational mirror of this principle is:

> **Stable things sleep; pressures wake them.**

That is the path to 250,000 genuinely persistent citizens without 250,000 expensive per-frame AI agents.

---

# 3. Current consolidated design thesis

The conversation has converged on a game where the Plan is not an abstract economic overlay.

It is a force that changes:

- enterprise requests,
- production timing,
- reserve behavior,
- freight demand,
- rail congestion,
- worker overtime,
- household time,
- migration,
- education,
- housing,
- social provision,
- informal behavior,
- information quality,
- future demographics.

The core simulation concept can be summarized as:

> **A physically simulated planned society in which material flows, institutions, information, infrastructure, households, persistent citizens, and long-term social reproduction continuously reshape one another.**

The core engine concept can be summarized as:

> **Use the cheapest deterministic representation that preserves every causal distinction the game cares about. Stable things sleep; changes propagate incrementally; important physical logistics remain explicit.**

The core player fantasy can be summarized as:

> **Turn a fragile, shortage-prone, buffer-hoarding industrial system into a calm, predictable, sophisticated republic capable of executing immense national projects without tearing ordinary society apart.**

---

# End of export
