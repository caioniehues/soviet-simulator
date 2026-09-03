# Soviet Simulator — Simulation, Mechanics, and Architecture Bible

**Version:** 0.1 consolidated synthesis  
**Date:** 2026-08-28  
**Project:** `caioniehues/soviet-simulator`  
**Purpose:** single project-facing synthesis of the mechanics, simulation architecture, implementation standards, research conclusions, scope boundaries, and open questions developed through the current design/research thread.

> **Authority note.** This document is a synthesis and design bible. It does **not** override the project's actual authority chain. The 1.0 charter binds scope. The glossary binds terminology. Ratified specifications bind mechanisms. Only accepted decisions bind architectural choices. `bd` binds task state. Current source and substrate fact sheets bind claims about what exists today. When this document conflicts with one of those authorities, the authority wins.

---

## 1. Executive thesis

Soviet Simulator should not be a conventional city-builder with a socialist skin, nor a generic economy simulator with quotas added on top. Its defining subject is **coordination under physical scarcity**.

The player is **THE PLANNER**. The player does not buy domestic goods through a price-clearing market. The player sets quotas, priorities, allocation policies, construction programs, reserves, and institutional rules. Goods then have to be produced, stored, loaded, transported, unloaded, delivered, and consumed by real physical and institutional actors.

The central simulation loop is:

```text
PLAN
  ↓
quotas / priorities / policies
  ↓
enterprises and institutions adapt
  ↓
requests / buffers / labor decisions / dispatch pressure
  ↓
physical production and logistics
  ↓
queues / shortages / surplus / delay / quality / congestion
  ↓
household and workplace experience
  ↓
reports / complaints / institutional information
  ↓
PLANNER KNOWLEDGE
  ↓
next PLAN
```

The planned economy itself should therefore generate gameplay. The game should not need arbitrary “economic crisis” dice rolls to create pressure. A taut plan, a bad reserve policy, an overconfident rail program, a housing lag, a storming production cycle, or an unreliable allocation system should be enough to create difficult but understandable situations.

The defining implementation rule is:

> **Use the cheapest representation that preserves every causal distinction the game cares about.**

That means deep simulation where distinctions change decisions, and aggressive aggregation where they do not.

The defining design rule is:

> **Every important macroeconomic number must eventually resolve into physical or institutional state.**

A steel shortfall should resolve into specific missing inputs, labor, power, water, storage, routes, or work. A cold apartment should resolve into a heating chain. A worker shortage should resolve into people, qualifications, housing, commute, care obligations, illness, turnover, or assignment. A national-project crisis should resolve into actual materials, rail slots, workers, construction sites, power, and social opportunity costs.

---

## 2. Non-negotiable design laws

### 2.1 Physical causality

1. **Goods move physically or do not move.** Allocation, matching, payment, route creation, and reservation never teleport stock.
2. **Request, allocation, reservation, pickup, custody, delivery, on-hand stock, and consumption are separate states.**
3. **Failure persists.** Missing stock, vehicle, route, dock, labor, power, water, housing, school capacity, or clinic capacity creates a visible waiting, partial, stalled, substitution, or going-without state.
4. **No silent deletion.** Goods, demand, citizens, vehicles, queues, and sites do not vanish because a transaction failed.
5. **No domestic price clearing.** Domestic scarcity is resolved by policy, queue, priority, substitution, rationing, reserve, adaptation, or going without. The rouble is border foreign currency only.
6. **Physical opportunity cost must be visible.** Prioritizing one use removes actual capacity, materials, labor, transport, housing, or service access from another use.

### 2.2 Information causality

7. **Reports are not truth.** Reported enterprise demand, plan fulfillment, institutional reports, and citizen knowledge are distinct from physical state.
8. **Information is a resource.** Better reporting, monitoring, social representation, and reliable institutions improve planning quality without magically improving physical supply.
9. **No omniscient player UI.** Player-facing data should be supplied through Planner-visible snapshots and institutional observation, not unrestricted access to `Simulation` truth.
10. **No hidden honesty flag.** Strategic behavior is inferred from discrepancies: request, receipt, consumption, on-hand, surplus, queue age, declared capacity, and physical output.

### 2.3 Social causality

11. **Citizens persist as identities.** Rendering or physical embodiment may be bounded, but the citizen record is not disposable.
12. **Households are first-class actors.** Residence, food/meat pantry, care obligations, housing queues, adaptation, and family history belong to the household layer.
13. **Social reproduction is physical.** Workers must be housed, fed, heated, educated, transported, kept healthy, and given enough time to perform work and household life.
14. **Citizens adapt.** They search, queue, substitute, hold buffers, change schedules, use household plots, rely on social contacts, relocate, transfer jobs, or go without.
15. **Do not reduce lived outcomes to one happiness scalar.** Preserve causes such as queue burden, crowding, warmth, health, time pressure, access, commute, career progress, and household reliability.

### 2.4 Technical causality

16. **Stable things sleep. Pressure wakes them.** Avoid per-frame AI for slow social and economic state.
17. **Compute → deterministic merge → commit.** Parallel workers may calculate intents; only deterministic ordered commits mutate authoritative state.
18. **One authority per state transition.** Cross-domain modules reference IDs/results; they do not copy or mutate another domain’s ledger.
19. **Every authoritative transaction is idempotent where replay is possible.** Immutable transaction IDs must make duplicate application a no-op.
20. **No generalized abstraction before shared invariants are proven.** Reuse topology, scheduling, IDs, journals, and transaction patterns; do not force water, power, traffic, sewage, heat, and gas through one generic flow solver.

---

## 3. Evidence and historical-method standard

Historical research is used to discover **mechanisms**, not to script an ideological verdict.

### 3.1 Evidence classes

- **Project binding evidence:** charter, glossary, specifications, accepted decisions.
- **Current-substrate evidence:** current repository source and fact sheets.
- **Primary/archival historical evidence:** declassified reports, translated technical material, constitutions, statistics, enterprise reports, technical manuals.
- **Serious secondary research:** economic history, social history, labor history, engineering literature.
- **Comparison evidence:** other simulation games and modern engineering software.
- **Design hypothesis:** proposed mechanic that is plausible but not yet historically or technically ratified.

### 3.2 Research caution

The CIA Reading Room is extremely valuable but heterogeneous. A CIA analytic assessment, a translated Soviet publication, an intercepted press item, and an anecdotal field report are not equal evidence. Formal Soviet legal texts describe institutional design, not necessarily lived power. Late-Soviet behavior should not be projected unchanged into the 1930s, 1950s, or 1960s. Yugoslav self-management, Hungarian reform socialism, Polish workplace politics, Soviet ministries, and syndicalist theories are different institutions and must not be collapsed into one generic “socialist behavior” model.

The game must remain capable of representing **excellent coordination as well as failure**. Historical problems such as storming, hoarding, shortages, reporting distortion, or turnover should arise from incentives and physical constraints, not from a hidden rule that central planning must fail.

---

## 4. The four realities

A defining architectural model is to separate four representations of the same republic.

### 4.1 Physical World

What actually exists:

- stocks;
- vehicles and cargo custody;
- citizens and households;
- buildings and sites;
- routes and queues;
- production runs;
- utility flows;
- actual attendance, time use, illness, shortages, and deliveries.

### 4.2 Institutional World

What organizations declare, request, record, or believe:

- enterprise capacity reports;
- material requirements;
- plan fulfillment reports;
- union/work-collective issues;
- local Soviet requests;
- housing and service reports;
- administrative statistics.

### 4.3 Planner World

What reaches THE PLANNER through reports, dashboards, inspections, measurements, and observation.

The Planner should not automatically receive every hidden warehouse quantity, every informal diversion, every citizen belief, or every unreported capacity reserve.

### 4.4 Lived World

What people actually experience:

- store search and queues;
- food availability;
- warmth;
- commute;
- crowding;
- childcare;
- health access;
- fatigue;
- workplace pressure;
- informal access;
- career and housing opportunity.

A formal Plan can report success while lived conditions deteriorate. Conversely, household plots, informal exchange, local adaptation, and enterprise welfare can make life more tolerable while formal administrative metrics look weak.

This separation is a major gameplay opportunity and should be enforced in code. The normal game UI should consume `PlannerSnapshot`, not `&Simulation`.

---

## 5. Planned economy as endogenous gameplay

### 5.1 The Plan is a control system

The Planner sends policy and allocation signals into a system with:

- delayed information;
- incomplete knowledge;
- physical lead times;
- subordinate organizations with local incentives;
- finite reserves;
- transport bottlenecks;
- social reproduction constraints.

The consequence is that a plan is never merely a list of quotas. It creates behavior.

### 5.2 Requests are strategic statements

An enterprise may technically consume 100 t of copper but report a requirement of 135 t because of:

- expected allocation cuts;
- unreliable rail delivery;
- future plan risk;
- safety stock;
- maintenance reserve;
- expected storming;
- fear of future shortage.

The simulation should distinguish:

```text
technical forecast
reported requirement
allocated amount
reserved amount
received amount
consumed amount
surplus/on-hand
outstanding request age
```

The Planner detects strategic over-requesting through discrepancies. No `dishonest: bool` is authoritative.

### 5.3 Generalized reliability → defensive buffering

This is one of the strongest unifying mechanics.

```text
unreliable delivery
      ↓
actors increase buffers / requests
      ↓
central availability falls
      ↓
other actors face shortage
      ↓
more defensive buffering
      ↓
logistics pressure rises
      ↓
delivery becomes less reliable
```

This applies to:

- enterprise material stocks;
- hospital medicine;
- household food reserves;
- rail wagon reserves;
- construction-site material reserves;
- potentially local social/informal networks.

A mature republic can become physically calmer because reliable institutions need less defensive stock.

### 5.4 Ratchet effect

Exceptional performance can be interpreted as normal capacity.

```text
heroic overfulfillment
      ↓
future quota raised
      ↓
revealed slack becomes obligation
      ↓
enterprise learns to conceal capacity
```

A player who automatically converts every successful surge into the next baseline can teach enterprises not to reveal their true capabilities.

### 5.5 Storming / plan-period pulses

When inputs arrive late, quotas remain taut, or management delays effort, production can become concentrated near period end.

```text
late components
   ↓
early idle capacity
   ↓
period end approaches
   ↓
overtime / maintenance deferral / rushed inspection
   ↓
output pulse
```

Storming is not just a production modifier. It should generate:

- freight pulses;
- worker overtime and fatigue;
- rail congestion;
- loading-dock queues;
- quality/rework risk where quality is later modeled;
- service and household time pressure;
- distorted reporting.

### 5.6 Freight-plan stability

Two plans with the same annual tonnage may require different railway fleets and infrastructure if one is smooth and one is pulsed.

Track:

- mean corridor load;
- peak load;
- plan-period variance;
- emergency dispatch share;
- empty repositioning;
- dock waiting;
- missed loading windows.

The right fix may be changing the Plan rather than building more track.

### 5.7 Priority does not solve scarcity

Priority determines **where scarcity appears**.

A Space Programme that receives copper can create shortage in radio production, machine tools, consumer appliances, or construction. Strategic rail slots can delay coal. Skilled workers assigned to a national project can weaken another plant.

The Planner must see the displaced use.

### 5.8 Priority inflation

If every ministry or enterprise can label its request “critical,” priority loses meaning. A future institutional system should constrain who can assign priority classes and expose the share of national activity operating under emergency/priority status.

### 5.9 Taut plan vs resilient plan

A plan operating at 99% nominal capacity with two days of reserve may produce less realized output than a plan at 88% capacity with stable logistics and twelve days of reserve.

Reserve and slack are not generic inefficiency. They are physical resilience.

### 5.10 Specialization vs self-supply

A highly specialized national plant can be efficient but brittle. A local enterprise may propose a less efficient auxiliary workshop to reduce dependence on unreliable national supply.

This creates a real planning choice:

- centralized specialization;
- local redundancy;
- standardization;
- reserve stock;
- transport investment.

### 5.11 Indicator design

Whatever the Plan measures can become what the enterprise optimizes.

Potential future examples:

- gross tonnes encourages heavy/easy output;
- unit counts encourage low-complexity variants;
- plan fulfillment encourages end-period storming;
- rail tonnage can neglect awkward consignments;
- quality indicators can increase inspection/rework burden.

The game should introduce alternative plan indicators only when their physical consequences are represented.

### 5.12 Planning credibility

Track practical institutional reliability rather than an abstract “trust” score.

Examples:

```text
requested inputs delivered on time
promised allocations honored
emergency reallocations frequency
mid-period quota changes
reported capacity later ratcheted
formal complaints acted upon
```

Actors adapt their buffers and behavior from experienced reliability.

### 5.13 Tolkatshi / procurement effort — future

Persistent supply trouble can cause enterprises to devote staff or effort to chasing inputs, negotiating reallocations, or finding substitutes. The mechanic should redistribute access or information, never create goods. It belongs after the basic dishonest-enterprise and institutional-information loops are proven.

### 5.14 Material-balance UI

Every important resource should eventually have a national and local balance:

```text
COPPER

Physical stock
  producer stock
  enterprise operating stock
  safety/reserve stock
  state/project reserve
  in transit

Period flow
  produced
  imported
  consumed
  allocated not delivered

Information
  technical forecast demand
  reported demand
  outstanding request age
  suspected discrepancy
```

Every line must be drillable into physical holders, hauls, consumers, or reports.

---

## 6. Resource, production, logistics, and construction model

### 6.1 Resource granularity rule

Split resources only when the distinction changes at least one of:

- handling;
- storage;
- carrier compatibility;
- recipe substitution;
- allocation priority;
- routing;
- quality/acceptance;
- physical bottleneck;
- strategic planning decision.

Do not create detail merely because the real material had many grades.

The 1.0 catalogue remains exactly fifteen domestic resources plus import-only Medicine. Water is a utility, never cargo.

### 6.2 Stock semantics

Authoritative quantity states:

```text
on hand
reserved
in custody
embedded in construction
consumed
```

Reservations are encumbrances, not additive physical quantity.

### 6.3 Logistics authority

One authoritative haul owns domestic fulfillment transitions:

```text
demand referenced
→ allocated
→ vehicle reserved
→ route to source
→ pickup
→ in custody
→ route to destination
→ delivery
→ release/recovery
```

Consumption remains external to the haul.

### 6.4 Custody conservation

Pickup and delivery are physical transitions.

For pickup quantity `x`:

```text
H_source -= x
R_source -= x
C_haul   += x
```

Post-pickup cancellation cannot simply “release” the goods. Cargo remains in custody until return, reassignment, or delivery.

### 6.5 Finite loading and unloading

Docks have:

- compatibility;
- occupancy;
- loading/unloading rate;
- power requirement;
- finite space.

A 12 t truck at a 2 t/min dock needs physical time to transfer. Partial transfer changes custody only by the transferred amount.

### 6.6 Target-stock policy

Planner-authored min/max stock targets can create demand and supply signals.

Candidate dispatch priority:

1. largest normalized deficit;
2. meaningful physical route distance;
3. stable deterministic ID tie-break.

No domestic price participates.

### 6.7 Empty movement and deadhead

A freight vehicle's productivity should eventually expose:

- loaded distance;
- empty distance;
- loading wait;
- unloading wait;
- road/rail wait;
- recovery/depot time;
- utilization.

This turns fleet planning into logistics rather than simply vehicle count.

### 6.8 Cargo/handling classes

Future physical classes can differentiate without exploding the resource catalogue:

- bulk mineral;
- aggregate;
- grain/food bulk;
- pallet/general cargo;
- machinery/project cargo;
- medicine/special cargo;
- waste.

A handling class can affect storage, dock, wagon/truck compatibility, transfer rate, and future visual representation.

### 6.9 Production

A run is bounded by:

```text
recipe
available delivered input
labor
power
water
process capacity
output storage
```

Always record the **binding constraint**.

A useful generic production bound is:

```text
run = min(
  recipe_capacity,
  input_capacity,
  labor_capacity,
  power_capacity,
  water_capacity,
  output_space_capacity
)
```

The exact formulas may differ by recipe, but the inspector must identify which gate limited realized output.

### 6.10 Production atomicity

A recipe run should be keyed by immutable `ProductionRunId`. Inputs are debited and outputs/byproducts credited atomically. Failure applies none. Replay applies once.

### 6.11 Surplus must remain visible

If an enterprise requested 140 but consumed 100, the remaining 40 stays physically on hand. Never delete excess merely to make a report look consistent.

### 6.12 Construction

Construction is production spread over time and space.

Lifecycle:

```text
Ghost
→ Verdict
→ approved Site
→ material/work gates
→ ground broken
→ completion result
→ building activation
```

The Ghost shows footprint, full material bill, and refusal. Commit revalidates the same proposal.

A Site is not an operational building. Elapsed time alone never completes a gate.

### 6.13 Construction phases — future depth

The binding 1.0 spec needs only physical material/work gates. A future deeper visual/logistics model can use phases such as:

```text
earthworks
foundations
structure
utilities
finishing
commissioning
```

Only add phases if they create distinct materials, equipment, workers, access, or planning decisions.

### 6.14 Construction-office principle

When deeper construction logistics arrive, keep physical offices, equipment, crews, depots, and staging. Reduce repetitive clicking through jurisdiction, autosearch, templates, and dispatch policy.

> **Automate execution, not decisions.**

---

## 7. Citizen and household simulation

### 7.1 Citizen architecture

A citizen is not a continuously running AI object. It is a persistent identity with compact state and scheduled transitions.

Conceptual layers:

```text
L0 — CitizenRecord
persistent biography / membership / core state

L1 — Scheduled social agent
next relevant event/activity

L2 — Active activity or trip
service/queue/route interaction

L3 — CitizenBody
physical movement state

L4 — RenderInstance
visible GPU representation
```

A citizen can move between levels without losing identity.

### 7.2 Household-first social reproduction

A household owns:

- persistent membership;
- residence assignment or housing queue;
- Food and Meat pantry point of use;
- care obligations;
- household adaptation state;
- long-term family history.

Households should make many consumption and care decisions rather than duplicating them individually for every member.

### 7.3 Time is a resource

A citizen's day is allocated among:

```text
sleep
formal work
commute
shopping/search
queues
childcare
other household care
domestic work
healthcare
education
household plot work
leisure
informal activity
```

A shortage can be costly even when the good is eventually acquired because it consumes time.

### 7.4 Social time loss

District and national observatory metrics should aggregate time spent on scarcity and access problems:

```text
food search time
queue time
long commute time
health-service wait
childcare deficit time
```

A useful social-reproduction balance:

```text
Potential adult hours
- formal employment
- commuting
- household care
- shopping/search/queues
- household production
- illness/absence
= discretionary/reserve time
```

This gives the Planner a non-monetary measure of social efficiency.

### 7.5 Activity scheduling

Do not update every citizen every tick.

Example:

```text
07:10 depart home
07:48 arrive work
16:00 shift ends
16:40 shop
17:25 return
23:00 sleep
```

Only a relevant event or invalidation wakes the citizen.

### 7.6 Household scheduling

Household-level scheduled reviews include:

- pantry threshold crossed;
- shopping responsibility;
- childcare assignment;
- household plot work;
- housing reconsideration;
- family lifecycle event;
- relocation decision.

### 7.7 Citizen knowledge and search

Citizens should not know every store's stock.

They can hold limited beliefs such as:

```text
Store A: usually reliable, last success 2 days ago
Store B: friend reported meat today
Store C: farther but historically reliable
```

A delivery can cause information to spread through local/social links, creating emergent rushes and queues.

### 7.8 Need satisfaction

Food and Meat remain separate 1.0 dwelling needs. Satisfaction occurs only through authoritative consumption after physical receipt.

A future adaptive sequence can be:

```text
preferred good available
→ acquire

not available
→ approved substitute search

still unavailable
→ household reserve / informal route / plot

still unavailable
→ going without
```

### 7.9 Household plots / dachas — Post-1.0 candidate

A household plot can produce food from:

- land;
- household time;
- seed/feed/tools;
- season;
- transport.

It acts as a household buffer but consumes time and may depend on formal-sector inputs.

### 7.10 Informal economy / blat — Post-1.0

Represent informality as an **alternative allocation topology**.

A favor or contact may help a household locate or acquire a scarce good, repair, or service. Goods still leave a physical holder. Informal access redistributes scarcity; it never creates bonus resources.

### 7.11 Sparse social graph

Do not store heap-allocated `Vec<Friend>` on every citizen.

Use packed sparse relationships with small relationship categories:

- close kin;
- close friends;
- workplace ties;
- neighborhood ties;
- favor/exchange ties.

Most social computation sleeps until a need, information event, relocation, or institutional event activates it.

### 7.12 Expectations and cohorts

Store slow expectations rather than one happiness value:

```text
expected housing standard
expected food reliability
expected mobility opportunity
expected leisure access
```

Expectations update slowly from personal/cohort experience. This permits generational differences without personality-heavy AI.

### 7.13 Housing

Housing is non-price allocation.

A persistent household queue can consider Planner-authored attributes such as:

- queue age;
- displacement;
- household size;
- overcrowding;
- dwelling fit;
- strategic labor recruitment;
- local policy.

Housing shortage retains the household and its members.

### 7.14 Housing as labor infrastructure

A workplace can recruit or retain workers partly through:

- enterprise housing;
- dormitories;
- commute;
- childcare;
- canteen/service access;
- local quality of everyday provision.

An industrial project therefore requires a social footprint, not merely direct headcount.

### 7.15 Mikrorayon completeness

A numerically completed housing district can still fail everyday life if the following lag:

- schools;
- clinics;
- shops;
- childcare;
- heating;
- transit;
- pedestrian access.

Never reduce this to one service-radius aura if actual capacity and physical travel can be represented.

### 7.16 Fertility and household lifecycle — gradual depth

Do not use `if happiness > X then birth`.

Family formation can eventually depend slowly on:

- age and partnership;
- existing children;
- dwelling crowding;
- care capacity;
- household time burden;
- health;
- long-run provision reliability;
- cohort expectations.

This allows a housing/service shortage to leave demographic effects decades later.

### 7.17 Persistent biographies

Keep permanent high-value events:

- birth;
- education milestones;
- qualification;
- major employment changes;
- household formation;
- children;
- major moves;
- death.

Keep recent detailed events for diagnosis; compress old routine shopping/travel history into aggregates.

---

## 8. Labor, workplace, unions, representation, and elections

### 8.1 Labor is differentiated capacity

Workers are not fungible headcount.

An enterprise may require:

```text
20 operators
2 maintenance technicians
1 inspector
```

If only one technician is present, effective capacity may be 50% even with excess operators.

Qualification categories should exist only where they change allocation or production.

### 8.2 Tenure and adaptation

A newly transferred worker should not instantly equal an experienced worker. A small tenure/familiarity state can model:

- orientation;
- team familiarity;
- equipment familiarity;
- effective productivity;
- error/absence risk where later needed.

High turnover can therefore reduce output without changing nominal headcount.

### 8.3 Labor hoarding

Enterprises can retain workers above technical minimum because of:

- absence risk;
- turnover;
- expected expansion;
- recruitment uncertainty;
- future quotas;
- skill scarcity.

This mirrors material hoarding.

### 8.4 Childcare and labor

Childcare does not provide a generic labor-force bonus. A physical childcare place externalizes specific household care hours and releases an adult's time for work or other activity.

Kindergarten is explicitly Post-1.0 in the charter, but the architecture should not prevent later childcare capacity from participating in household scheduling.

### 8.5 Education → qualification → assignment → settlement

Education should be a physical service with attendance, staff, and capacity.

Future deeper pipeline:

```text
school
→ technical institute
→ qualification
→ employment/assignment
→ relocation if needed
→ housing
→ workplace adaptation
```

The 1.0 scope remains two education tiers.

### 8.6 Workplace as primary social-economic institution

A significant enterprise can have a `WorkCollectiveId` that aggregates the lived conditions of its members without owning production stock.

Potential facts:

```text
overtime
lost time from material shortage
machine downtime
illness/absence
housing queue among workers
childcare constraints
commute
safety issues
canteen/provision issues
```

### 8.7 Work-collective issue aggregation

Do not store `grievance = 82`.

Periodic or threshold-triggered meetings derive salient issues from physical facts:

```text
repeated month-end overtime
unsafe machine condition
housing allocation delay
canteen shortage
transport congestion
```

Each issue retains causal fact references.

### 8.8 Trade unions — Post-1.0 institutional layer

Historically Soviet unions were mass, system-integrated organizations with functions that included labor protection, welfare/social insurance, workplace participation, norms/remuneration discussions, and grievance channels. They should not be modeled as a generic Western collective-bargaining organization nor as a “union strength” meter.

A future `UnionCommittee` can own:

- membership/representation;
- active safety/welfare cases;
- proposals;
- meeting cadence;
- institutional memory.

It does not own production or resources.

### 8.9 Safety inspection mechanic

Physical inputs:

```text
maintenance backlog
fatigue
overtime
machine condition
minor incidents
```

Union/workplace safety body can raise a case.

Planner/enterprise choices:

- stop line for repair;
- defer repair;
- reallocate maintenance labor/material;
- lower short-term quota pressure.

Consequences remain physical: output, backlog, absence, fatigue, equipment condition.

### 8.10 Workplace welfare

Future unions/workplace bodies can mediate:

- housing recommendations;
- rest/sanatorium allocation;
- canteen/service proposals;
- safety;
- worker complaints.

Social provision is intertemporal production: immediate labor absence or construction/material cost can improve future labor capacity and retention.

### 8.11 Local Soviets and elections — Post-1.0

For the fixed 1950s–60s Soviet setting, do **not** model elections as modern multiparty competition.

The useful mechanic is representation and information:

```text
citizens / workplaces / neighborhoods
→ nomination and institutional filtering
→ representatives/delegates
→ local Soviet agenda
→ requests/reports
→ Planner information and allocation
```

Possible state:

- representative citizen identity;
- constituency;
- occupation/workplace provenance;
- term;
- active local issues;
- petitions/mandates;
- institutional report.

### 8.12 Representation error

A powerful future diagnostic is to compare:

```text
physical reality
lived experience
enterprise report
union/workplace report
local Soviet report
Planner belief
```

Political institutions become a **sensor network** rather than a popularity minigame.

### 8.13 Institutional confidence

Do not model loyalty in 1.0. A future system can track empirical channel effectiveness:

```text
complaint submitted
→ action taken or not
```

Repeatedly effective channels encourage formal reporting. Ineffective channels can encourage exit, absenteeism, informal adaptation, or reduced reporting. This is institutional reliability, not ideological mind reading.

### 8.14 Alternative socialist organization — future rulesets

The same physical economy can later support institutional variants by changing authority:

- Soviet directive planning;
- territorial/Sovnarkhoz planning;
- stronger work-collective participation;
- Yugoslav-style worker self-management;
- reform-socialist mixed autonomy.

The important questions are concrete:

```text
Who proposes quotas?
Who sets reserves?
Who selects management?
Who allocates enterprise surplus?
Who controls housing/welfare?
Who approves overtime/norms?
Who coordinates inter-enterprise inputs?
```

Do not implement these as government-type bonuses.

---

## 9. Vehicles, road traffic, transit, and rail

### 9.1 Specialized vehicle physics

Do not use a generic rigid-body physics engine as the traffic authority.

Road vehicles are constrained mostly to lane coordinates. Authoritative state can include:

```text
lane
longitudinal position
speed
acceleration
length
mass
max power
tractive force
braking capability
cargo load
route
```

### 9.2 Load and grade

Approximate longitudinal physics:

```text
F_resist = F_roll + F_aero + F_grade
F_grade ≈ m g sin(theta)
a = (F_tractive - F_resist) / m
```

The same truck should climb differently when loaded and empty. Terrain becomes economically meaningful.

### 9.3 Power and traction

At low speed, a vehicle can be traction-limited. At higher speed, power becomes limiting:

```text
F_tractive ≈ P / v
```

subject to a maximum tractive force.

### 9.4 Braking and jerk

Represent safe braking envelope and limit change in acceleration (jerk) for convincing motion.

No need for suspension, tire temperature, per-wheel contact, or destructive collision physics.

### 9.5 Collision avoidance

Use compact following/lane-change behavior inspired by IDM/MOBIL concepts:

- desired headway;
- leader gap;
- relative speed;
- comfortable acceleration/deceleration;
- safe lane-change effect on followers;
- mandatory route lane changes.

A physical overlap should be treated as an emergency obstruction/invariant problem, not as a random accident generator.

### 9.6 Traffic state

The binding spec already uses EWMA load, BPR cost, and Gawron damping for route cost. Preserve that role.

But ultimate traffic simulation should also represent **queue storage and spillback**. A saturated downstream link can prevent upstream entry and block intersections.

### 9.7 Hybrid micro/mesoscopic traffic

Use different fidelity by relevance:

- authoritative corridor/lane-cell density and queue propagation at network scale;
- microscopic car-following near camera, bottlenecks, and important freight;
- persistent important vehicle identities mapped into the mesoscopic flow.

### 9.8 Industrial gates

Factories and warehouses create real traffic through finite gates/docks.

```text
unloading delay
→ yard fills
→ gate queue
→ public road spillback
→ buses/freight delayed
```

### 9.9 Shift waves

Large workplaces create discrete passenger demand pulses.

Shift staggering becomes a planner tool:

```text
Plant A 07–15
Plant B 08–16
Plant C 09–17
```

Same infrastructure, lower peak load.

### 9.10 Bus/tram/trolleybus depth — mostly Post-1.0

Future transit vehicles can have:

- seats/standing capacity;
- boarding/alighting throughput;
- dwell time;
- headway;
- crowding;
- bunching.

Tram/trolleybus service can later couple to electricity.

Passenger rail, signals, and electrification remain Post-1.0.

### 9.11 Freight rail 1.0

1.0 contains minimal freight rail: three buildings, one locomotive type, one wagon type.

The same no-teleport custody rules apply.

Even in minimal form, preserve architectural fields for:

- locomotive identity;
- wagon identity;
- consist length/mass;
- wagon capacity;
- source/destination;
- cargo custody;
- train route/progress.

### 9.12 Future rail physics

Train behavior can derive from:

- consist mass;
- locomotive power/tractive effort;
- brake capability;
- train length;
- gradient;
- speed;
- siding/track length.

Train length becomes infrastructure compatibility. Braking/headway/blocks become capacity.

### 9.13 Wagon balance and empty repositioning

A region can have locomotives but no suitable empty wagons. Empty movements consume track, yard, locomotive, and time capacity.

### 9.14 Yards as logistics processors — future

A yard can process:

```text
arrival
→ classification
→ storage tracks
→ consist assembly
→ departure
```

Each stage has finite track/shunter/time capacity.

### 9.15 Winter roads — future weather coupling

Road surface states can modify:

- safe acceleration;
- braking;
- headway;
- effective capacity.

Snow clearing should use physical vehicles from depots and Planner-authored road priorities.

---

## 10. Utilities and physical inertia

### 10.1 Shared topology, separate physics

Water, sewage, heating, electricity, and future gas may share a network topology kernel:

```text
NodeId
EdgeId
attachments
connected components
topology revision
compact adjacency
```

They must not share one generic solver.

### 10.2 Water

1.0 binding principles:

- separate Water authority;
- connected finite-rate transfer;
- quantity and quality;
- buffers;
- border meter;
- Water never cargo.

Recommended deeper hydraulic model where feasible:

- elevation/head;
- node pressure;
- pipe resistance;
- pump head/capacity;
- tanks/reservoirs;
- pressure-dependent delivery.

Connection does not guarantee adequate pressure.

### 10.3 Water storage and delayed failure

Tanks create resilience:

```text
pump loses power
→ tank drains
→ pressure later falls
→ service later degrades
```

The time delay is gameplay.

### 10.4 Sewage

Separate graph and authority.

Key physical concepts:

- source buffers;
- gravity/slope where modeled;
- finite pipes;
- pumps;
- treatment;
- discharge;
- backpressure;
- residue.

A downstream full buffer can restrict upstream flow. Backlog persists.

### 10.5 Sewage-water-power coupling

A sewage pump losing electricity can fill buffers and eventually force a declared service restriction or future emergency discharge. The utility interaction must occur through explicit service results, not cross-module state mutation.

### 10.6 Heating

1.0 requires a distinct thermal network with conservation and no electric fallback.

A deeper model should include:

- generated thermal rate;
- pipe/pump capacity;
- distribution loss;
- network buffer;
- supply transport delay;
- building thermal mass.

A heating failure should take time to reach apartments.

### 10.7 Building thermal inertia

A simple first-order model is enough:

```text
indoor heat change
= supplied heat
- envelope loss(outdoor temperature, building state)
```

Do not simulate room CFD.

### 10.8 Electricity

Electricity has very little inherent network storage compared with water, heat, or gas.

1.0 binding model:

```text
G + D = V + C + L
```

with continuous non-price priority shedding.

Future generator depth can add:

- min/max output;
- ramp up/down;
- startup state;
- operating reserve;
- generator class.

Do not introduce full AC-grid physics until the game has a demonstrated need.

### 10.9 Future grid frequency/inertia

Possible advanced extension only after basic grid behavior works. Frequency response and synchronous inertia can create fast stability behavior, but this is not a 1.0 need.

### 10.10 Gas and pipelines — Post-1.0

Gas is valuable because the pipeline itself stores gas as **linepack**.

Potential state:

- pipe pressure;
- stored gas/linepack;
- compressor capacity;
- source flow;
- endpoint minimum pressure;
- storage.

A supply shortfall can remain temporarily hidden while linepack drains. This creates a slow-motion crisis unlike electricity.

### 10.11 Reservoirs and hydro

1.0 includes reservoir-graph water and hydro dams.

Use mass-balance representation rather than full CFD:

```text
V_next = V + inflow - outflow - named losses
```

Hydropower approximately:

```text
P = rho * g * Q * H * efficiency
```

Reservoir level affects available head and future reserve.

### 10.12 Weather and seasons

Weather should alter physical demands/capacities through explicit interfaces:

- heating demand;
- hydrology/reservoir inflow;
- crop cycles;
- road surface;
- possibly utility demand.

Do not fake weather effects through generic city-wide multipliers where a physical interface can exist.

### 10.13 Network reserves

Different systems have different physical reserve forms:

| System | Physical reserve |
|---|---|
| Roads | unused storage/capacity |
| Rail | headway/timetable slack, spare vehicles/wagons |
| Logistics | inventory and fleet slack |
| Water | tank volume, pressure/pump headroom |
| Sewage | empty buffer, treatment headroom |
| Heating | hot network water and building thermal mass |
| Electricity | ramping/generation/storage reserve |
| Gas | linepack/storage |
| Reservoir | stored water/head |
| Society | discretionary time, spare housing/service capacity |

A republic can be nominally functioning while reserve is dangerously low.

### 10.14 Physical momentum and phase lag

The republic has memory.

A disruption may take time to become visible because stocks, water, heat, linepack, rail slack, and household buffers carry the system.

A repair may also take time to propagate.

The expert Planner manages trajectories:

```text
current stock
consumption rate
incoming ETA
time to depletion
```

rather than responding only to red icons.

---

## 11. Cross-system causal loops

### 11.1 Space electronics cascade

```text
Space programme quota risk
→ emergency electronics requirement
→ electronics plants over-request / storm
→ copper and precision-component demand rises
→ rail dispatch pressure rises
→ consumer/appliance production loses inputs
→ rail congestion rises
→ precision components arrive later
→ more storming/overtime
→ quality/rework risk rises
→ replacement demand rises
→ Space programme risk worsens
```

Citizen/workplace layer:

```text
storming
→ overtime
→ fatigue
→ safety/workplace complaints
→ institutional pressure
→ maintenance or welfare allocation
→ short-term production trade-off
```

### 11.2 Coal → electricity → water → sewage → heat cascade

```text
coal train delayed
→ thermal generation reserve falls
→ electricity curtailment
→ pump service constrained
→ water tank drains
→ sewage pump buffer fills
→ district service restrictions
→ heating circulation may degrade
→ household time/health/warmth consequences
```

Different systems respond at different speeds. That delay is the mechanic.

### 11.3 Housing → labor → production loop

```text
industrial expansion
→ labor demand
→ housing queue / long commute
→ turnover / recruitment difficulty
→ lower effective staffing
→ production shortfall
→ construction-material availability worsens
→ housing construction slows
```

### 11.4 Retail shortage → labor loop

```text
store shortage
→ household search/queue time rises
→ sleep/discretionary time falls
→ lateness/fatigue/absence increases
→ workplace effective capacity falls
```

Consumer logistics is therefore an industrial input.

### 11.5 Reliability spiral

```text
unreliable supply
→ larger enterprise/household buffers
→ central availability falls
→ shortage spreads
→ emergency dispatch and queueing rises
→ supply becomes less reliable
```

The positive version is equally important:

```text
reliable delivery
→ buffers shrink
→ stock released
→ emergency dispatch falls
→ congestion falls
→ reliability improves
```

### 11.6 National-project privilege loop

```text
strategic project receives housing + specialists + freight priority
→ staffing/project performance improves
→ other districts lose those resources
→ queues / service strain / labor shortage elsewhere
→ local/institutional pressure rises
```

No generic “national project penalty” is needed.

---

## 12. National projects and scenario opportunities

The same engine can support varied gameplay without separate minigames.

### 12.1 National Project

A temporary nationwide material and labor distortion with explicit project phases, reserved materials, project cargo, construction sites, and priority rules.

### 12.2 Space Programme

Ultimate coordination challenge:

- precision industry;
- electronics;
- chemicals/materials;
- skilled labor;
- project cargo;
- test/assembly facilities;
- infrastructure;
- national priority opportunity costs.

The Space Programme should stress the ordinary economy, not live in a separate tech tree.

### 12.3 Housing Campaign

Goal is not just dwelling count. The player must coordinate prefabrication, cement/steel, transport, sites, utilities, schools, clinics, shops, transit, and labor retention.

### 12.4 Monotown

One city-forming enterprise dominates employment and welfare. Expansion can outrun housing/services; factory decline can destabilize the settlement.

### 12.5 Science City / Closed City

Scarce specialists, priority provision, specialized construction, high service expectations, and opportunity cost in the rest of the republic.

### 12.6 Shortages Amid Plenty

National stocks look adequate, but enterprise reserves, transport variance, wrong location, and retail bottlenecks produce lived scarcity.

### 12.7 The Taut Plan

High utilization, small reserves, and narrow margins. The challenge is not total capacity but variance and phase lag.

### 12.8 Sovnarkhoz / territorial planning — future

Change administrative topology from branch-ministry control toward territorial councils. The physical economy stays the same; reporting, allocation authority, and optimization locality change.

### 12.9 Reform / self-management — future

Change enterprise authority, worker participation, retained surplus/investment decisions, and proposal formation while preserving the same physical logistics and citizen engine.

### 12.10 Everyday Socialism

Focus on lived provision: housing, queues, transport, household time, childcare, health, retail reliability, and local services rather than maximum heavy-industry expansion.

### 12.11 Frontier Corridor

New extraction/industrial corridor where rail, housing, utilities, labor migration, climate, and construction sequencing dominate.

### 12.12 Late-System Maintenance

Large inherited industrial base, aging infrastructure, high maintenance burden, weak reserves, and pressure to preserve throughput without overbuilding new capacity.

### 12.13 Enterprise Director — future alternate perspective

A scenario that lets the player feel local enterprise incentives: over-requesting, buffer protection, labor retention, storming, quota bargaining, maintenance deferral, and reporting.

---

## 13. Rust simulation architecture

### 13.1 Architectural center

Do not put `ECS` at the conceptual center.

The center is:

```text
identity
time
authority
transactions
change propagation
determinism
information boundaries
```

A proposed module shape:

```text
simulation/
  core/
    time
    ids
    units
    scheduler
    random
    transition
    change_journal

  stores/
    citizens
    households
    enterprises
    stocks
    bodies
    vehicles

  physical/
    resources
    logistics
    production
    construction
    roads
    traffic
    rail
    networks

  society/
    households
    needs
    employment
    education
    healthcare
    demography

  institutions/
    enterprises
    allocation
    reporting
    plan
    reserves
    work_collectives      # hook / post-1.0
    unions                # post-1.0
    representation        # post-1.0

  observatory/
    material_balance
    labor_balance
    service_balance
    discrepancy
    causality
    indexes

  forecast/
    feasibility
    shadow_sim
    plan_compare

  snapshot/
    planner
    render
    audio
    debug
```

Initially these should usually be modules inside the existing `simulation` crate, not dozens of new Cargo crates.

### 13.2 Identity strategy

Use two different identity families.

**Persistent append-only dense typed IDs** for things whose historical identity must not be reused:

- CitizenId;
- HouseholdId;
- potentially major institution records.

**Generational/slot-map handles** for reusable live entities:

- CitizenBody;
- vehicle live entities where current substrate already benefits;
- temporary route/itinerary;
- temporary haul or active movement objects where reuse is safe.

A dead Citizen #N stays Citizen #N.

### 13.3 Dense stores and SoA

Hot large-scale state should be contiguous.

Avoid:

```rust
struct Citizen {
    name: String,
    friends: Vec<CitizenId>,
    needs: HashMap<...>,
    ...
}
```

Prefer a compact dense core plus sparse side stores:

```text
CitizenCore[]
CitizenActivity[]
CitizenResidence[]

PregnancyStore
IllnessEpisodeStore
EducationEnrollmentStore
HousingQueueStore
SocialEdgeStore
CitizenBodyStore
```

Benchmark hand-written SoA against candidate crates; do not adopt a SoA library simply for elegance.

### 13.4 Typed IDs and units

Use newtypes aggressively at authority boundaries:

```rust
CitizenId
HouseholdId
EnterpriseId
HaulId
ProductionRunId
DeliveryId
WaterTransferId
```

Use fixed/integer authoritative quantities for conservation-sensitive state.

Selective `uom`-style dimensional typing can be valuable at engineering/physics boundaries, but avoid making the entire hot loop type-heavy if profiling or ergonomics suffer.

### 13.5 Option-sized IDs

For compact dense persistent IDs, non-zero integer IDs can allow `Option<Id>` to retain integer-sized niche layout. Verify with compile-time size assertions.

### 13.6 Fixed resource arrays

The 1.0 catalogue is small and fixed. Prefer dense arrays indexed by typed resource index over HashMaps for per-holder stock vectors.

Benefits:

- deterministic iteration;
- cache locality;
- no hashing/allocation;
- compact serialization.

### 13.7 Bitset society

Maintain derived membership/cohort indexes.

Example:

```text
working_age
∩ technical_qualified
∩ district_7
∩ available
∩ reachable
```

Then only the narrowed candidates run expensive household or employment evaluation.

> **Filter cheaply. Think expensively.**

Use dense bitsets for dense populations and roaring/sparse structures where membership is sparse.

### 13.8 Event-driven simulation

Do not iterate over every citizen and institution every tick.

Core pattern:

```text
Sleep
→ Wake on scheduled/event condition
→ Decide
→ Emit intent
→ Commit
→ Schedule next wake
→ Sleep
```

Use explicit state-machine enums, not suspended async futures for authoritative citizen simulation.

### 13.9 Deterministic event calendar

Use a custom deterministic calendar/timing-wheel architecture optimized for monotonic simulation time.

Requirements:

- serializable;
- stable ordering for equal times;
- domain/entity tie-break;
- efficient bulk scheduling;
- no dependence on thread wake order.

### 13.10 Cadence bands

Different domains update at natural rates:

```text
visible movement       high cadence
traffic aggregation    medium/high
logistics dispatch     seconds/minutes
production             minutes
utilities              seconds/minutes by domain
household reviews      event/daily
education/demography   daily/monthly/yearly
plan reporting         period boundaries
```

No universal “everything thinks every frame” tick.

### 13.11 Deterministic phases

Target phase architecture:

```text
COMMAND
TOPOLOGY
ALLOCATION
DECISION
ROUTING
MOVEMENT
ARRIVAL
PRODUCTION
UTILITIES
ACCOUNTING
REPORTING
```

A system can be split across phases if its lifecycle requires it.

### 13.12 Parallel compute → deterministic commit

Within a phase:

```text
parallel read-only compute
→ thread/local intent buffers
→ concatenate
→ stable deterministic sort
→ serial/partitioned authoritative commit
```

Do not make authoritative correctness depend on `DashMap`, lock acquisition order, or Rayon scheduling.

### 13.13 Keyed deterministic random

Never rely on one global sequential RNG for outcomes that can be parallelized.

Derive randomness from stable keys:

```text
master_seed
domain
entity_id
event_ordinal
```

The same simulation state must yield the same result independent of thread scheduling.

### 13.14 Typed system contexts

Systems should not receive unrestricted `&mut Simulation`.

Example:

```rust
struct ProductionContext<'a> {
    resources: &'a ResourcesRead,
    labor: &'a LaborRead,
    power: &'a PowerServiceRead,
    out: &'a mut ProductionIntentBuffer,
}
```

This is the code-level form of authority ownership.

### 13.15 Authoritative transitions

Use immutable IDs and explicit application results for once-only changes.

Good examples:

- DeliveryId;
- EmbedId;
- ProductionRunId;
- ElectricityAllocationId;
- HeatAllocationId;
- DeathResultId.

The shared concept is **authoritative transition**, not one giant generic transaction framework.

### 13.16 Change Journal

Authoritative commits emit compact changes:

```text
StockChanged
HaulStateChanged
HouseholdMoved
EmploymentChanged
QualificationChanged
NetworkTopologyChanged
PlanChanged
CollectiveIssueRaised
```

Consumers:

- Observatory;
- indexes;
- notifications;
- causal history;
- snapshots;
- debugging.

The world should propagate **what changed**, not rescan the civilization whenever possible.

### 13.17 Incremental observatory

The physical simulation owns truth. A derived observatory maintains:

- material balances;
- labor balances;
- service pressure;
- queue metrics;
- enterprise discrepancy;
- exposure indexes;
- causal explanations;
- planner indicators.

Salsa is a prototype candidate for pure derived queries. Differential Dataflow is a research prototype for advanced relational/incremental diagnostics, not a required core dependency.

### 13.18 Causal facts

Important transitions may emit:

```text
FactId
tick
subject
kind
causes[]
```

Retention classes:

- active/recent failures: detailed;
- routine old history: compact/aggregate;
- major lifecycle/plan events: persistent.

Do not full-event-source the entire universe indefinitely.

### 13.19 Snapshot architecture

Publish immutable read views:

```text
PlannerSnapshot
RenderSnapshot
AudioSnapshot
DebugSnapshot
```

The simulation writes the next snapshot; UI/render/audio can continue reading the previous immutable snapshot.

`ArcSwap` is a strong fit for snapshot publication; upgrade only when repository dependency policy permits.

### 13.20 Planner information boundary

`PlannerSnapshot` must never expose hidden physical truth merely because it is convenient for UI.

If the Planner can know a value, the simulation should identify **how**:

- measured directly;
- reported by enterprise;
- aggregated statistically;
- observed via institution;
- estimated/forecast;
- unknown.

### 13.21 Hierarchical routing

Direct A* per request will not scale to late-game movement.

Use hierarchy and cache by dimensions such as:

```text
origin region
destination region
mode
vehicle/access class
topology revision
traffic epoch
```

Keep exact local routing where needed.

### 13.22 Network kernel

Shared network topology can own:

- typed nodes/edges;
- attachments;
- connected components;
- topology revision;
- compact CSR adjacency.

Each domain keeps separate solver state.

### 13.23 Shadow simulation / Gosplan computer

Determinism enables forecast branches from snapshots.

Crucial rule:

> Forecasts should consume Planner-visible/reported state, not secret PhysicalWorld truth.

A mathematically feasible plan can fail physically because of hidden capacity, freight variance, labor shortage, or storming.

### 13.24 LP/MILP feasibility — optional analysis tool

Use an optimizer as an instrument, not as the player.

Possible question:

```text
Given reported capacities, recipes, stocks, and declared transport limits,
is this plan materially feasible?
```

`good_lp` + HiGHS are candidates for a future Gosplan analysis tool.

### 13.25 GPU boundary

Keep authoritative economy/social decisions on CPU.

GPU is appropriate for:

- culling/LOD;
- citizen/vehicle interpolation;
- crowds;
- heatmaps;
- smoke/stockpiles/lights/snow;
- visual state.

Use validated POD structures at the GPU boundary; do not make arbitrary authoritative structs `Pod` for convenience.

---

## 14. Code and architecture standards

These are recommended engineering standards for future specifications/implementation. They are not automatically binding until ratified where the project process requires it.

### 14.1 Authority standard

Every mutable authoritative field must have exactly one owning module.

Cross-domain code passes:

- typed IDs;
- immutable results;
- service views;
- intents.

Never duplicate another module's ledger for convenience.

### 14.2 Transaction standard

Any transition that can be retried/replayed must have:

- immutable transaction/result ID;
- named source/destination/subject;
- explicit quantity/state delta;
- atomic validation/commit where conservation requires it;
- replay no-op semantics.

### 14.3 Failure standard

Failure must answer:

```text
what is waiting?
why?
since when?
what physical/institutional object owns the problem?
what can recover it?
```

Do not use a generic `failed: bool` when a recoverable state exists.

### 14.4 Deterministic ordering standard

All equal-priority/equal-cost authoritative choices must have a stable tie-break based on immutable identity or declared ordering.

### 14.5 Data-layout standard

For hot 250k-scale records:

- no heap allocation per citizen by default;
- no `String` in hot core records;
- no HashMap where fixed dense indexing works;
- sparse side stores for rare states;
- size assertions for critical structs.

### 14.6 String/name standard

Citizen names and verbose presentation metadata should live in cold/presentation stores or interned tables, not the hot simulation core.

### 14.7 Float standard

Use integer/fixed-point state for conserved/accounting quantities where determinism matters.

Floating point is acceptable for bounded physical/presentation calculations when:

- order is controlled;
- drift does not violate conservation;
- authoritative outcomes are tested for repeatability.

### 14.8 Randomness standard

Randomness must be reproducible and domain-keyed. Never let unrelated system iteration order change outcomes.

### 14.9 Scheduling standard

Slow state uses scheduled events or change-driven invalidation. A system must justify any full-population per-tick scan.

### 14.10 Snapshot standard

UI/render/audio do not hold broad simulation locks during normal operation. They read immutable snapshots.

### 14.11 Cache invalidation standard

Caches must be keyed by explicit revision/epoch where possible:

- topology revision;
- traffic epoch;
- policy version;
- data generation.

Do not rely on “probably still valid.”

### 14.12 Serialization standard

Released saves need an explicit envelope:

```text
magic
format version
schema/game version
codec
uncompressed size
compressed size
checksum
payload
```

From 1.0 release candidate onward, released saves remain compatible per charter.

Postcard + zstd are strong candidates for stable released payloads. `rkyv` is a candidate for internal snapshots/caches, not necessarily the public save contract.

### 14.13 Hash/digest standard

Maintain a canonical authoritative-state digest for determinism tests and replay comparison. BLAKE3 is a strong candidate.

### 14.14 Documentation standard

Every mechanism document should state:

- authority;
- state;
- invariants;
- failure behavior;
- observability;
- acceptance evidence;
- current substrate gap;
- deferred behavior;
- open questions.

Every current-reality claim should include a verified commit or source location where practical.

### 14.15 Dependency standard

Prefer standard library/custom compact code for core invariants. Add crates when they buy proven correctness, performance, or development leverage.

Do not migrate to Bevy/hecs/Shipyard merely because they are popular. Do not introduce Tokio-per-citizen, DashMap authoritative state, or nightly `std::simd` as foundational architecture.

---

## 15. Recommended Rust ecosystem decisions

### Adopt / strong candidates

- **Rayon**: parallel compute phases, with deterministic commit discipline.
- **ArcSwap**: immutable snapshot publication.
- **FixedBitSet**: dense cohort indexes.
- **Roaring**: sparse large-set indexes where appropriate.
- **SmallVec**: small causal-parent lists and truly small variable collections.
- **BLAKE3**: authoritative digests/checksums.
- **bytemuck**: validated POD at render/GPU boundaries.
- **proptest**: conservation and state-machine property tests.
- **Shuttle**: reproducible concurrency testing for infrastructure primitives.
- **iai-callgrind**: stable instruction/cache regression benchmarks.

### Prototype before adopting

- **typed-index-collections**: typed dense vectors/slices.
- **fixed**: fixed-point authoritative quantities.
- **soa-rs / soa_derive**: compare against hand-written SoA.
- **Salsa**: derived Observatory queries.
- **good_lp + HiGHS**: plan feasibility instrument.
- **rkyv**: internal snapshots/caches.
- **postcard + zstd**: released save payload/envelope combination.

### Research/algorithm references

- hierarchical timing wheel / radix-heap ideas;
- EPANET water solver behavior as validation oracle;
- SWMM for sewage/stormwater reference cases;
- HEC-RAS/ResSim for hydrology/reservoir reference cases;
- IDM/MOBIL and cell-transmission traffic literature;
- Rapier/Parry for geometry/collision validation, not authoritative road traffic.

### Avoid as authoritative core defaults

- general ECS rewrite;
- async future per citizen;
- concurrent hash-map mutation of truth;
- generic rigid-body traffic;
- full numerical engineering solvers where a compact causal approximation is enough;
- dependency churn without benchmark/evidence.

---

## 16. Performance, memory, and determinism strategy

### 16.1 Primary contract

The actual product contract remains:

> **250,000 persistent citizen identities at 60 fps on the development machine.**

A microbenchmark is not proof. A green test suite is not proof. The headless whole-game benchmark is the final gate.

### 16.2 Performance hierarchy

Optimize in this order:

1. representation/algorithm;
2. update cadence;
3. locality/SoA;
4. incremental propagation;
5. hierarchy/cache;
6. parallelism;
7. SIMD.

Do not jump to SIMD before fixing per-citizen scanning or cache-unfriendly structures.

### 16.3 Memory budgets

Set explicit budgets for hot records and assert them at compile time.

Candidate budget categories:

- CitizenCore;
- HouseholdCore;
- Vehicle hot state;
- Haul;
- scheduled event;
- causal fact;
- route-cache entry.

The exact byte budgets require profiling and should become an accepted decision later.

### 16.4 Citizen scale

250k citizens should not imply 250k:

- heap strings;
- vectors;
- behavior trees;
- individual timers allocated separately;
- route searches;
- rendered bodies.

### 16.5 Rendering scale

Use spatial render cells/chunks, frustum/distance culling, citizen/vehicle LOD, and dirty/partial instance-buffer updates.

The current substrate's “render every human/vehicle” pattern must not define the target architecture.

### 16.6 Determinism bisection

At checkpoints:

```text
tick digest
phase digest
```

If two runs diverge:

1. binary-search first divergent checkpoint;
2. identify first divergent phase;
3. inspect transition journal around the difference.

This can make parallel deterministic bugs debuggable.

---

## 17. Validation and test strategy

### 17.1 Conservation properties

Property-test sequences such as:

```text
request
reserve
pickup
cancel
reroute
return
deliver
consume
```

Invariant:

```text
source on-hand
+ destination on-hand
+ in-custody
+ embedded
+ declared consumed/sinks
```

must equal initial + declared sources.

### 17.2 Idempotency

Every replayable immutable transition must be tested twice. The second application is a no-op.

### 17.3 Mutation tests

Each specification acceptance test should name a deliberately wrong implementation that must make it fail.

Examples:

- credit destination at reservation;
- satisfy need at route arrival;
- let Traffic release parking;
- allow a construction Site to activate before completion;
- double-apply Water meter delta.

### 17.4 Deterministic repeat-run tests

Identical initial state and commands must produce identical authoritative digests.

### 17.5 Scientific/reference oracles

For simplified engineering solvers, compare representative cases with known tools or analytic results:

- water vs EPANET;
- future sewage/stormwater vs SWMM cases;
- reservoir/hydro vs mass-balance/HEC reference cases;
- vehicle following vs IDM reference scenarios;
- traffic queue wave vs cell-transmission analytic cases.

The external solver is a validation oracle, not necessarily a runtime dependency.

### 17.6 Benchmarks

Maintain:

- 250k whole-world headless benchmark;
- routing benchmark;
- citizen daily-event benchmark;
- material-allocation benchmark;
- logistics transfer benchmark;
- snapshot publication benchmark;
- utility solver benchmarks;
- render-instance preparation benchmark.

Use instruction-level benchmarks for hot kernels only after whole-world bottlenecks are known.

---

## 18. UI, observability, and causal inspector

### 18.1 The inspector contract

Every significant object should eventually answer:

```text
STATUS
CAUSE
TREND
POLICY
PHYSICAL CHAIN
```

### 18.2 Example: cold apartment

```text
Apartment 41
Indoor temperature: 15.8 C

Why?
served heat 61%
→ district supply temperature low
→ Heat Plant 3 constrained
→ coal bunker shortage
→ coal delivery 3h41 late
→ rail corridor congestion
→ strategic freight surge
```

### 18.3 Example: worker shortage

```text
Factory 41
Required: 1,480
Present: 1,291

Drivers:
transfers          -72
illness/absence    -31
turnover           -46
unfilled positions -40
```

Click turnover:

```text
recent departures correlated with:
housing queue
commute > 60 min
childcare deficit
repeated overtime
```

### 18.4 Example: enterprise discrepancy

```text
Copper wire plant
reported need:      140 t
received:           126 t
consumed:            91 t
on hand surplus:     35 t
oldest open request: 11 d
```

The UI can flag “discrepancy worth inspection” without asserting fraud.

### 18.5 Pressure maps

Useful overlays:

- material balance;
- freight queue and dock wait;
- road spillback;
- rail utilization;
- housing queue;
- service queue burden;
- water pressure/tank reserve;
- sewage buffer/backpressure;
- heating reserve;
- electricity curtailment;
- social time loss;
- information/report confidence.

### 18.6 Reserve view

Expose reserves in natural units rather than one generic percentage:

```text
Coal bunker:       18 h at current burn
Water district:     6 h at current draw
Heat buffer:        4 h equivalent
Rail corridor:     13% headway slack
Housing:            1.8% spare fit capacity
Household time:    47 min/day median reserve
```

### 18.7 Notifications

Notifications should be generated from causal state, not arbitrary events.

Examples:

- request inflation rising;
- repeated period-end storming;
- rail peak/mean divergence worsening;
- household queue burden threshold crossed;
- water tank depletion trajectory critical;
- work collective repeatedly reporting the same unresolved issue.

---

## 19. 1.0 scope, hooks, and Post-1.0 discipline

### 19.1 Required 1.0 commitments

Keep aligned with the current charter:

- 15 domestic resources + import-only Medicine;
- Food and Meat as separate dwelling needs;
- Water as utility, never cargo;
- physical production and logistics;
- dishonest-enterprise inspectability;
- construction Sites with material/work causality;
- households and housing shortage;
- demographics including death;
- two education tiers;
- healthcare;
- landfill/incineration;
- terrain, reservoir graph, hydro, ore, pollution;
- minimal freight rail;
- border trade with one rouble;
- three authored Plans on one continuous save;
- day/night/seasons;
- bounded visible citizens;
- Linux/Windows build;
- 250k persistent citizen target.

### 19.2 Architecture hooks now, mechanics later

Design data/interfaces so future work can add:

- childcare/kindergarten;
- worker collectives;
- union committees;
- local representation;
- household plots;
- informal networks;
- quality lots;
- vehicle wear/fuel/manufacture;
- rail signaling/electrification/passengers;
- deeper water pressure/quality;
- CHP;
- gas pipelines/linepack;
- more detailed stormwater;
- advanced grid physics.

A hook means avoiding an architectural dead end. It does **not** mean implementing dormant complexity in 1.0.

### 19.3 Explicit Post-1.0 / future

Respect the charter cuts:

- loyalty/legitimacy;
- broadcast/monuments/crime;
- vehicle manufacture and fuel lifecycle;
- voltage tiers/transformers/grid depth;
- CHP;
- electric-heating fallback;
- passenger rail/signals/electrification;
- ships/docks/pipelines/cableways/containers/aircraft/petrochemicals;
- era calendar/dual currency;
- kindergarten/deathcare/epidemics;
- perishables/refrigerated transport;
- Steam/marketing.

Tourism and random fires/disasters remain out of scope.

---

## 20. Recommended implementation sequence

### Phase 0 — architecture contract and scope closure

Before large rewrites:

- ratify missing core specs;
- resolve resource catalogue/units/handling;
- define IDs and transition conventions;
- define event-calendar/determinism contract;
- define benchmark harness;
- close missing 1.0 specs for agriculture, terrain/geology, weather/seasons, hydrology/hydro, pollution, macro Plan/Quota/Tranche, authored Plans/onboarding, notifications/save/crash-recovery, presentation/audio.

### Phase 1 — prove 250k representation

Implement/benchmark:

- compact CitizenStore;
- persistent IDs;
- event calendar;
- snapshots;
- keyed RNG;
- world digest;
- headless 250k benchmark.

Do this before layering rich citizen mechanics.

### Phase 2 — prove one complete physical industrial chain

A canonical vertical slice:

```text
coal source
→ request
→ allocation
→ reservation
→ finite truck
→ route
→ pickup custody
→ delivery
→ producer on-hand
→ production
```

Disable conflicting inherited fulfillment paths.

### Phase 3 — dishonest-enterprise loop and Observatory

Implement:

- reported requirement;
- actual consumption;
- surplus;
- discrepancy inspector;
- material balance;
- causal journal.

This proves the game's economic thesis early.

### Phase 4 — construction

One building from Ghost through physical Site material/work to activation.

### Phase 5 — households and lived scarcity

Implement:

- household identity;
- residence/housing queue;
- Food/Meat pantry;
- consumption;
- explicit going without;
- minimal household activity scheduling.

### Phase 6 — movement and traffic scale

Implement:

- hierarchical routing/cache;
- durable traffic state;
- queue/spillback where feasible;
- vehicle movement improvements;
- render culling/LOD.

### Phase 7 — utilities

Introduce separate domain solvers with explicit service results:

- electricity;
- water;
- sewage;
- heating;
- waste;
- reservoir/hydro as required by charter.

### Phase 8 — labor/services/demography

- employment/qualification;
- education;
- healthcare;
- death/demography;
- richer household time effects if budget allows.

### Phase 9 — Plan/Quota/Tranche macro loop

Only after physical economy and lived scarcity are meaningful.

### Phase 10 — authored Plans and polish

The three Plans teach and stress the same simulation rather than introduce scripted exceptions.

Graphics/UI/audio polish remains interleaved throughout development rather than postponed to the end.

---

## 21. Missing specifications and unresolved design questions

### 21.1 Scope-critical missing specifications

The following need explicit current specs before design freeze:

- agriculture/livestock;
- terrain/geology/ore;
- weather/seasons;
- hydrology/reservoir/hydro;
- pollution;
- Plan / Quota / Tranche macro loop;
- authored Plans/onboarding;
- notifications/event log;
- shell/save/autosave/crash recovery;
- presentation/audio acceptance.

### 21.2 Resource questions

- exact 15-resource catalogue;
- units;
- storage/handling classes;
- permitted substitutions;
- whether any 1.0 resource requires quality differentiation.

### 21.3 Household questions

- overcrowding semantics;
- queue policy attributes/tie-breaks;
- household formation/split/merge lifecycle;
- minimum 1.0 time/activity model.

### 21.4 Labor questions

- smallest useful qualification taxonomy;
- adaptation/tenure effect in 1.0 or later;
- how workplace-worker matching is allocated;
- whether enterprises have explicit labor reserve targets.

### 21.5 Traffic questions

- authoritative lane/corridor capacity measure;
- spillback detail required for 1.0;
- wait→reroute→stall thresholds;
- topology-change behavior for in-progress trips.

### 21.6 Rail questions

- exact 1.0 station trio;
- wagon capacity/compatibility;
- train consist representation;
- minimal yard/loading model;
- whether grade/mass physics is 1.0 or only a data hook.

### 21.7 Utility questions

- minimum Water quality classes/endpoints;
- whether pressure/head belongs in first ratified Water implementation;
- Sewage generation endpoints and treatment outputs;
- Electricity priority categories/storage requirement;
- Heating source classes and minimum thermal-inertia fidelity;
- reservoir operational rules for 1.0.

### 21.8 Institutional questions — Post-1.0

- workplace collective granularity;
- union authority by historical era;
- formal vs actual election/nomination mechanics;
- local Soviet information powers;
- alternative socialist-system rulesets;
- which channels remain player-visible without loyalty/legitimacy mechanics.

---

## 22. Anti-patterns and risks

### 22.1 Simulation maximalism

Risk: “simulate everything the engine can afford.”

Rule: detail must preserve a causal distinction or player decision.

### 22.2 Generic percentage modifiers

Avoid:

```text
kindergarten +5% workforce
bad housing -10% productivity
union +10% happiness
snow -20% traffic
```

Prefer physical chains:

```text
childcare place
→ care hours released
→ adult can attend shift
```

### 22.3 Omniscient UI

If UI reads PhysicalWorld directly, information-as-resource gameplay collapses.

### 22.4 Hidden personality flags

Avoid hidden `honesty`, `loyalty`, `corruption`, or `morale` flags as substitutes for observed behavior. Use state/history/reliability unless a future design proves a compact latent trait is necessary.

### 22.5 Over-generalized shared solver

Electricity, water, sewage, heat, traffic, and gas have different storage/inertia/failure behavior. Share topology and tooling, not fake physics.

### 22.6 Per-frame citizens

250k citizens with per-frame decision code is architectural failure, not a problem to solve with more threads.

### 22.7 Parallel mutable truth

Avoid making determinism depend on concurrent mutation order.

### 22.8 Full event sourcing

Do not retain every routine historical transition forever. Preserve what enables diagnosis and biography; aggregate the rest.

### 22.9 Crate-driven architecture

Do not redesign the engine around a new ECS, async runtime, concurrent map, or numerical library because it is fashionable.

### 22.10 Historical flattening

Do not create one timeless “Soviet citizen” behavior. Era, region, institution, cohort, and system type matter.

### 22.11 Predetermined ideological failure

The game must permit good planning to work. Mechanisms create trade-offs and information problems, not a hard-coded collapse trajectory.

---

## 23. Canonical mechanic taxonomy

For future specifications and implementation tickets, classify every mechanic using this template.

```text
Mechanic:
Historical/technical evidence:
Scope: 1.0 / hook / Post-1.0
Authority:
Authoritative state:
Inputs:
Transition cadence:
Physical conservation/invariants:
Failure behavior:
Player decisions:
Planner-visible information:
Hidden/institutional information:
Causal facts emitted:
Computational representation:
Benchmark/test:
Open questions:
```

This should stop mechanics from being described only as prose fantasies.

---

## 24. Proposed core data/state sketches

These are illustrative shapes, not ratified Rust definitions.

### 24.1 Citizen

```rust
struct CitizenCore {
    household: HouseholdId,
    birth_day: Day,
    workplace: Option<WorkplaceId>,
    qualification: QualificationCode,
    activity: ActivityCode,
    next_event: SimTime,
    flags: CitizenFlags,
}
```

### 24.2 Household

```rust
struct HouseholdCore {
    members: MemberRange,
    residence: ResidenceState,
    housing_queue: Option<HousingQueueId>,
    food_pantry: StockHolderId,
    meat_pantry: StockHolderId,
    next_review: SimTime,
}
```

### 24.3 Haul

```rust
struct Haul {
    demand: DemandId,
    item: ResourceId,
    quantity: Quantity,
    source: StockHolderId,
    destination: StockHolderId,
    vehicle: Option<VehicleId>,
    custody: Quantity,
    state: HaulState,
    age: SimDuration,
    recovery_reason: Option<RecoveryReason>,
}
```

### 24.4 Enterprise requirement view

```rust
struct EnterpriseRequirementRecord {
    enterprise: EnterpriseId,
    resource: ResourceId,
    technical_forecast: Quantity,
    reported_requirement: Quantity,
    received: Quantity,
    consumed: Quantity,
    on_hand: Quantity,
    outstanding_age: SimDuration,
}
```

### 24.5 Causal fact

```rust
struct Fact {
    id: FactId,
    tick: Tick,
    subject: SubjectId,
    kind: FactKind,
    causes: SmallCauseList,
}
```

### 24.6 Work collective — future

```rust
struct WorkCollectiveRecord {
    enterprise: EnterpriseId,
    membership: MembershipRange,
    union: Option<UnionCommitteeId>,
    issue_summary: CollectiveIssueSummary,
    next_review: SimTime,
}
```

### 24.7 Planner snapshot

```rust
struct PlannerSnapshot {
    period: PlanPeriod,
    material_balances: Arc<MaterialBalanceView>,
    institutions: Arc<InstitutionReportView>,
    shortages: Arc<ShortageView>,
    services: Arc<ServicePressureView>,
    causal_alerts: Arc<CausalAlertView>,
}
```

No raw `Simulation` reference is exposed.

---

## 25. Source and reference bibliography

This bibliography records the major source families used to develop the mechanics in this design thread. It is not a substitute for source notes in future ratified specifications.

### 25.1 Project authority

- `docs/plan/charter-1.0.md`
- `docs/reference/glossary.md`
- `docs/decisions/README.md`
- `docs/reference/specifications/resources.md`
- `docs/reference/specifications/production.md`
- `docs/reference/specifications/logistics.md`
- `docs/reference/specifications/construction.md`
- `docs/reference/specifications/households.md`
- `docs/reference/specifications/education.md`
- `docs/reference/specifications/healthcare.md`
- `docs/reference/specifications/vehicles.md`
- `docs/reference/specifications/roads.md`
- `docs/reference/specifications/traffic.md`
- `docs/reference/specifications/pathfinding.md`
- `docs/reference/specifications/water.md`
- `docs/reference/specifications/sewage.md`
- `docs/reference/specifications/electricity.md`
- `docs/reference/specifications/heating.md`

### 25.2 CIA Reading Room / declassified material used in citizen, labor, economy, transport research

- CIA, Soviet consumer/living-standard analyses: `https://www.cia.gov/readingroom/docs/19831201.pdf`
- CIA, living standards / social pressures: `https://www.cia.gov/readingroom/docs/CIA-RDP08S01350R000602150001-3.pdf`
- CIA, female labor-force/manpower reporting: `https://www.cia.gov/readingroom/document/cia-rdp65b00383r000200020002-8`
- CIA, housing and labor turnover material: `https://www.cia.gov/readingroom/docs/CIA-RDP78-03061A000200050003-3.pdf`
- CIA, National Intelligence Survey: USSR social/labor institutions: `https://www.cia.gov/readingroom/document/cia-rdp01-00707r000200090031-2`
- CIA, private agriculture/plots: `https://www.cia.gov/readingroom/docs/DOC_0000499547.pdf`
- CIA, second economy/corruption/informal allocation: `https://www.cia.gov/readingroom/docs/DOC_0000681980.pdf`
- CIA, Soviet labor market/manpower allocation: `https://www.cia.gov/readingroom/docs/DOC_0000498182.pdf`
- CIA, Plant 393 report on norms/storming/reporting: `https://www.cia.gov/readingroom/document/cia-rdp80-00810a001700780006-2`
- CIA-translated Soviet transport material: `https://www.cia.gov/readingroom/document/cia-rdp82-00850r000300080044-2`
- CIA-held Soviet public-health material: `https://www.cia.gov/readingroom/document/cia-rdp80t00246a001600750002-0`
- CIA, alcohol/social productivity analysis: `https://www.cia.gov/readingroom/document/cia-rdp87t00787r000200200003-0`
- CIA, manpower/demographic assessment: `https://www.cia.gov/readingroom/docs/CIA-RDP85T00875R001900030121-1.pdf`
- CIA, late-Soviet social expectations/stratification analysis: `https://www.cia.gov/readingroom/document/cia-rdp87t00495r000700720002-3`
- CIA, Gorbachev worker participation analysis: `https://www.cia.gov/readingroom/docs/DOC_0000499321.pdf`

### 25.3 Labor institutions and workplace governance

- ILO Research Repository, Soviet labor inspection and union/workplace safety material.
- ILO Research Repository, workers' participation in occupational safety and health matters in the USSR.
- ILO Research Repository, labor administration by the state and trade unions in the USSR.
- ILO Research Repository, role of management/workers in norms, remuneration, productivity, and production conferences.
- Soviet constitutions (1936, 1977) for formal institutional/electoral design; use with caution as formal rather than lived-practice evidence.
- Library of Congress country studies for comparative Yugoslav, Hungarian, East German, and other socialist institutional contexts.

### 25.4 Traffic and vehicle engineering references

- Martin Treiber et al., Intelligent Driver Model (IDM) reference material: `https://mtreiber.de/MicroApplet/IDM.html`
- MOBIL lane-changing model: `https://mtreiber.de/MicroApplet/MOBIL.html`
- Daganzo, Cell Transmission Model literature.
- FHWA traffic-flow/queue/spillback guidance.
- FRA freight-train operating/braking reference material.

### 25.5 Water, sewage, hydrology, and energy engineering references

- US EPA EPANET: `https://www.epa.gov/water-research/epanet`
- US EPA SWMM: `https://www.epa.gov/water-research/storm-water-management-model-swmm`
- US Army Corps HEC-RAS / HEC-ResSim documentation.
- District-heating literature on transport delay, pipeline thermal storage, and building thermal inertia.
- NREL/NERC literature on generation reserve, ramping, and frequency response.
- Gas-network literature on linepack as distributed storage.

### 25.6 Rust/library references discussed

- Rust 2024 Edition / Rust 1.85+ official documentation.
- Rayon.
- arc-swap.
- typed-index-collections.
- fixed / uom.
- FixedBitSet / roaring.
- smallvec.
- Salsa.
- differential-dataflow.
- good_lp / HiGHS bindings.
- faer.
- rkyv.
- postcard.
- zstd.
- blake3.
- proptest.
- Shuttle / loom for focused concurrency primitives.
- iai-callgrind / Criterion.
- bytemuck.
- wide / pulp only after profiling.

---

## 26. Final synthesis

The most powerful ideas in this project are not individual features. They are a small set of rules that make many systems behave coherently.

### Coupling

Every important system participates in other systems. Freight affects production. Production affects electricity. Electricity affects pumps. Water affects sewage. Housing affects labor. Retail affects household time. Household time affects work. Work affects production. Institutional reports affect the next Plan.

### Physical social reproduction

The economy must reproduce not only goods but the human capability required to continue producing them.

### Physical momentum

Stocks, reserves, water, thermal mass, rail slack, and household buffers let the republic continue after a disruption and delay the effects of both failure and recovery.

### Phase lag

The Planner must learn to act before visible failure. A cold apartment may have been caused by a rail decision many hours earlier. A labor shortage may have been caused by housing decisions years earlier.

### Adaptive actors

Enterprises, households, workers, institutions, and future informal networks should respond to the reliability and incentives they experience.

### Information as a resource

The Planner does not merely need more steel. The Planner needs to know which steel report is credible, why workers are leaving, whether a shortage is physical or informational, and whether a formal metric corresponds to lived reality.

### Calmness as success

A mature planned economy should not only produce more. It should become calmer:

```text
smaller emergency reserves
fewer emergency dispatches
shorter queues
lower plan-period variance
more reliable deliveries
lower turnover
less overtime
more accurate reports
more household discretionary time
```

The long-term victory fantasy is therefore not only gigantism.

It is **coordination quality**.

The player starts with a fragile physical economy and gradually builds a republic capable of executing complex national Plans without hiding their cost in warehouses, queues, overtime, cold homes, late trains, or exhausted people.

That is the identity the architecture should protect.
