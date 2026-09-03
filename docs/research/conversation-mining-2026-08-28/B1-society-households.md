# B1 — Society, households, time, housing, migration, informal networks

## 0. Summary (top-ten findings)

1. **B1-01** The social reproduction loop is the conversation's strongest original contribution — it closes an economic circuit through citizens that no existing city-builder models. CONFIRMED by Soviet demographic and economic literature.
2. **B1-05** Housing as a persistent non-price queue is historically accurate and mechanically novel for the genre. The conversation proposes it; the code has nothing — current `HumanEnt` stores one `Home(BuildingID)` with no queue, no household, no eligibility.
3. **B1-07** Time as a citizen resource is well-supported: Soviet time-budget studies document 28+ hours/week of domestic work for women, with shopping/queueing alone at 6 hours/week. The conversation names the categories; it does not build the conserved-hours accounting identity.
4. **B1-10** Blat as allocation topology is the conversation's most ambitious social mechanic. Ledeneva (1998) confirms it redistributes real stock and is zero-sum. Section 3 works out the graph structure the conversation left abstract.
5. **B1-03** The 8,000→20,000 multiplier is PLAUSIBLE but the conversation cites no source. Soviet monotown literature confirms the ratio is in the right range for a large single-enterprise city with full welfare provision.
6. **B1-14** The code has no household entity at all. `HumanEnt` is individual; `spawn_human` creates one person per house. Every household mechanic in the conversation is a green-field build.
7. **B1-20** Cohort expectation memory is a novel mechanic with no direct academic model but is consistent with sociological literature on generational adaptation (Zaslavskaya). Section 3 works out a minimal update rule.
8. **B1-21** The Social Reproduction Balance as an accounting identity is the conversation's most concrete UI proposal. Section 3 derives the exact balance equation.
9. **B1-MISSED-01** Propiska (residence permits) is entirely absent from the conversation. It was THE eligibility gate for housing and employment in Soviet cities and is a natural Planner policy lever.
10. **B1-MISSED-05** Alcohol as a time, health, and productivity sink is absent. It was quantitatively the largest single cause of Soviet male mortality and a major barter medium.

## 1. Extracted items

| ID | Statement | Source lines | Verdict |
|---|---|---|---|
| B1-01 | Social reproduction loop: plan → industrial production → goods/housing/services → household life → health/education/time/migration → labour force → enterprise capacity → plan | 335–353 | CONFIRMED |
| B1-02 | Enterprise as miniature welfare state: production + labour + welfare (housing, dormitory, childcare, clinic, canteen, culture, transport) | 355–373 | CONFIRMED |
| B1-03 | A plant employing 8,000 workers requires a settlement of ~20,000 people | 376–377 | PLAUSIBLE |
| B1-04 | Monotown feedback: housing shortage → overcrowding → turnover → vacancies → production shortfall → delayed housing | 379–383 | CONFIRMED |
| B1-05 | Housing as persistent non-price queue; housing as labour-recruitment instrument | 385–389 | CONFIRMED |
| B1-06 | Mikrorayon completeness: housing plan succeeds numerically while daily life fails (schools, shops, transit, clinics, heating lag) | 391–393 | CONFIRMED |
| B1-07 | Time as citizen resource: finite hours on sleep, work, commute, shopping, queueing, childcare, domestic work, healthcare, household food production, leisure | 395–411 | CONFIRMED |
| B1-08 | Childcare capacity releases real adult labour-hours | 413–417 | CONFIRMED |
| B1-09 | Household plots buffer formal food failures at cost of labour, transport, tools, seed/feed, leisure | 419–421 | CONFIRMED |
| B1-10 | Blat: sparse social ties as alternate allocation topology; moves physical goods; displaces someone else's access | 427–429 | CONFIRMED |
| B1-11 | Inequality without wealth classes: workplace, housing allocation, geography, service access, social contacts, qualifications, institutional privilege | 431–441 | CONFIRMED |
| B1-12 | Labour shortage: socialist economy faces scarcity of workers, not jobs; solutions include education, housing, migration, transit, childcare, mechanization | 443–447 | CONFIRMED |
| B1-13 | Citizens retain meaningful life histories: birth, school, technical education, qualification, employment, household formation, housing queues, relocation, children, death | 449–451 | CONFIRMED |
| B1-14 | Citizen architecture: persistent CitizenRecord; expensive CitizenBody only for physical movement | 453–455 | PLAUSIBLE |
| B1-15 | Queues as first-class scarcity objects sharing the same primitive at different time scales | 457–459 | PLAUSIBLE |
| B1-16 | Queue burden: measure human time lost to scarcity instead of collapsing into "happiness" | 461–462 | CONFIRMED |
| B1-17 | Science cities / closed cities / priority geography | 464–467 | CONFIRMED |
| B1-18 | Migration driven by housing opportunities, jobs, climate, services, administrative eligibility | 469–471 | CONFIRMED |
| B1-19 | Capital dilution: too many construction projects lock stocks inside unfinished assets | 473–475 | CONFIRMED |
| B1-20 | Different generations slowly learn different expectations regarding housing, food reliability, mobility, leisure | 1082–1084 | PLAUSIBLE |
| B1-21 | Social Reproduction Balance: district-level account of potential adult hours, formal employment, commuting, household care, shopping/queues, household production, illness/absence, residual discretionary time | 1086–1101 | PLAUSIBLE |
| B1-22 | Time poverty chain: retail supply failure → longer search → longer queues → household time burden → fatigue → late arrivals → labour performance | 955–972 | CONFIRMED |
| B1-23 | Household scheduling: households solve work, shopping, childcare, domestic labour, allotment work, healthcare, leisure using actual schedules and family structure | 974–977 | CONFIRMED |
| B1-24 | Long-term fertility echo: housing crowding and care burden affect family formation; shortage today echoes in labour force decades later | 987–991 | CONFIRMED |
| B1-25 | Private plots as adaptive production: formal food unreliability increases household production value; plots consume time and depend on state-sector inputs | 993–995 | CONFIRMED |
| B1-26 | Sparse social graph: citizens maintain small number of durable kin/work/neighborhood/favour ties; unmet needs wake the social network selectively | 1002–1007 | PLAUSIBLE |
| B1-27 | Access privilege as actual access channels rather than single social-class variable | 1009–1011 | CONFIRMED |
| B1-28 | Graduate assignment creates physical migration and household tradeoffs | 1013–1014 | CONFIRMED |
| B1-29 | Labour adaptation costs: replacement workers need time to become fully productive | 1016–1019 | CONFIRMED |
| B1-30 | Labour hoarding: enterprises hold surplus workers like surplus materials | 1021–1023 | CONFIRMED |
| B1-31 | Storming enters worker life: overtime, sleep loss, fatigue, absenteeism, quality problems, family time loss, future turnover | 1025–1027 | CONFIRMED |
| B1-32 | Health as production of future capacity, not generic welfare bonus | 1029–1031 | CONFIRMED |
| B1-33 | Formal vs lived economy: maintain separate views; plan may look good while households spend huge time searching/queueing | 1033–1047 | CONFIRMED |
| B1-34 | Four realities: actual physical, reported institutional, Planner knowledge, household lived experience | 1049–1059 | PLAUSIBLE |
| B1-35 | Citizen knowledge: citizens should not know every shop's inventory; act from remembered/local/social information | 1061–1079 | PLAUSIBLE |
| B1-36 | Rust implementation: dense CitizenCore, sparse side stores, materialization levels, separate scheduling, exhaustive enums, typed IDs, change journal, bitset filtering, biography layers, slow expectation memory, institutional reliability estimates | 1103–1115 | PLAUSIBLE |
| B1-37 | Citizens as agents of adaptation: factories, households, workers, young people, managers, networks each adapt; the state adapts to their adaptations | 1145–1165 | CONFIRMED |
| B1-38 | Stable things sleep; pressures wake them — computational mirror of adaptation | 1167–1171 | PLAUSIBLE |

## 2. Validation detail

### B1-01 Social reproduction loop — CONFIRMED

The loop (plan → production → goods/services → household life → labour force → plan) is a standard framework in Soviet economic sociology. Zaslavskaya's work on social stratification (1988) explicitly models the feedback between industrial production and social reproduction. The conversation's innovation is making it a game loop rather than an analytical framework.

**Code state:** No social reproduction exists in code. `HumanEnt` (`simulation/src/world.rs:88-105`) stores location, pedestrian physics, decision state, one home, one food desire, optional work, and `PersonalInfo` (name, age 20–50, gender). There is no household, no education, no health, no housing queue, no lifecycle beyond the initial spawn.

### B1-03 The 8,000→20,000 multiplier — PLAUSIBLE

The conversation provides no citation. Soviet monotown literature (Monotown: Urban Dreams, Brutal Imperatives, Clayton Strange) discusses settlements built around single enterprises. The 2.5× multiplier (8,000 workers → 20,000 total population) is reasonable for a large factory with full welfare provision. Soviet demographic data shows labour force participation rates of 50–55% of the total population in industrial cities, which would give a multiplier of ~1.8–2.0×. With dependents, retirees, and service workers, 2.5× is in the right range but on the high side. The exact number depends on assumptions about household size, age structure, and the extent of enterprise welfare.

### B1-04 Monotown feedback loop — CONFIRMED

The housing→turnover→production feedback loop is well-documented. Feshbach's work on Soviet labour turnover (Slavic Review, 1970s) identifies housing as the primary cause of voluntary turnover (tekuchest'). The Cambridge Core article "Labor Turnover in the Soviet Union" confirms that enterprises competed for workers through housing allocation. The feedback loop (inadequate housing → worker departure → production shortfall → inability to build housing) is a classic monotown failure mode documented in Russian monotown studies.

### B1-05 Housing queue — CONFIRMED against code: DOES NOT EXIST

**Historical validation:** Soviet housing allocation used persistent non-price queues. Key facts:
- Waiting lists: 12–36% of families registered by city (1988 UPI data)
- 18% of all Soviet families had been on waiting lists for 10+ years
- Enterprise housing: ~2-year wait; municipal housing: 10+ years in Moscow/Leningrad
- Eligibility threshold: living below the sanitary norm (typically 9 sq m per person, varying by city)
- Three channels: enterprise (fastest), municipal (longest), cooperative (partial self-financing, Khrushchev-era onward)
- Housing was allocated by workplace committees, trade unions, or municipal soviets — never by price

Sources: Andrusz, *Housing and Urban Development in the USSR* (1984); Morton, "Who Gets What, When and How? Housing in the Soviet Union" (1980); UPI Archives (1988).

**Code state:** `Home` (`simulation/src/souls/desire/home.rs:7-28`) is a single `BuildingID` with a constant score of `0.2`. `spawn_human` (`simulation/src/souls/human.rs:237-278`) creates one human per empty house and immediately assigns ownership. There is no queue, no waiting, no eligibility, no household. The households specification (`docs/reference/specifications/households.md`) documents this gap explicitly at lines 113–119.

### B1-07 Time as citizen resource — CONFIRMED

Soviet time-budget studies provide hard numbers:
- **Women's domestic work:** 28 hours/week (1965 study), vs 12 hours for men
- **Food preparation:** 10–12 hours/week (women), 1.5–2 hours (men)
- **Grocery shopping:** ~6 hours/week (women), ~3 hours (men)
- **Laundry:** ~6 hours/week (women), 20–30 minutes (men)

Key scholars: Gordon L.A. & Klopov E.V., *Man After Work* (1972); Patrushev V.D., *Time Budget of the Working Population* (various Soviet-era publications); Szalai A., "Women's Time" (time-budget journal, 1970s).

The conversation names the right categories (sleep, work, commute, shopping, queueing, childcare, domestic work, healthcare, household food production, leisure). The numbers support time as a scarce, gendered, and measurable resource.

### B1-09 Household plots — CONFIRMED with hard numbers

Private plots produced a disproportionate share of Soviet food:
- **1966 data** (Cambridge Core, "The Private Sector in Soviet Agriculture"): 64% of potatoes, 43% of vegetables, 40% of meat, 39% of milk, 66% of eggs — from approximately 3% of agricultural land
- By the late Soviet period, private plots produced up to 90% of vegetables (Jamestown Foundation)
- Post-Soviet (2000): 92% of potatoes, 77% of vegetables, 87% of berries/fruit (Russian government statistics)

Lovell's *Summerfolk: A History of the Dacha* (2003) documents the cultural and economic function. Shmelev's work on the informal economy confirms private plots as a critical food buffer. The conversation correctly identifies the costs: labour, transport, tools, seed/feed, and leisure time.

### B1-10 Blat — CONFIRMED

Ledeneva (1998), *Russia's Economy of Favours: Blat, Networking and Informal Exchange*, Cambridge University Press:
- Blat = exchange of "favours of access" in conditions of shortages and a state system of privileges
- Blat IS NOT corruption or bribery — it operates through personal relationships and reciprocity, not money
- Blat IS zero-sum for physical goods: getting something through connections means someone else in the formal queue does not get it
- Blat networks were sparse (small number of trusted contacts per person), durable, and reciprocal
- Blat was pervasive: it was not deviant behaviour but a normal survival mechanism

The conversation's proposal (B1-10, B1-26) to model blat as a sparse social graph that redistributes real stock and displaces formal queue access is faithful to Ledeneva's analysis.

### B1-11 Inequality without wealth classes — CONFIRMED

Zaslavskaya (1988) identified social stratification in the USSR through access rather than wealth: workplace privileges, housing allocation priority, geographic location (Moscow vs province), special distribution channels (zakrytye raspredeliteli — closed distributors), educational credentials, party membership, and informal connections. The Wilson Center paper "The Soviet Worker: Social Stratification and Political Perceptions" (Connor, 1977) corroborates access-based inequality.

The conversation's seven sources of inequality (workplace, housing allocation, geography, service access, social contacts, qualifications, institutional privilege) map directly onto the academic literature.

### B1-28 Graduate assignment — CONFIRMED

Raspredeleniye (job by distribution): mandatory job placement for graduates, established 1933. Graduates were assigned by commission to positions anywhere in the USSR for 3–4 years. Young specialists received special status (could not be fired) and housing benefits. This created forced migration and household disruption. Source: Wikipedia "Job by distribution"; Belarusian continuation of the system documented in Equal Times (2022).

### B1-29, B1-30 Labour turnover and hoarding — CONFIRMED

**Turnover (tekuchest'):** Feshbach & Rapawy, "Labor constraints in the five-year plan" (JEC, 1973). Housing was the primary driver of voluntary turnover. Enterprises without housing could not retain workers. Replacement workers needed a productivity ramp of months to years depending on skill level. Soviet studies documented turnover rates of 20–30% annually in some industries.

**Labour hoarding:** Kornai, *Economics of Shortage* (1980), Chapter 11. Enterprises with soft budget constraints hoard both materials and workers as insurance against plan uncertainty. The mechanism is identical: unreliable supply → defensive reserves → aggregate scarcity → worse reliability. Labour hoarding manifests as enterprises carrying 10–20% more workers than their production technically requires.

### B1-14 CitizenRecord/CitizenBody split — PLAUSIBLE

The conversation proposes this architecture but does not validate it against performance constraints. The current `HumanEnt` (`simulation/src/world.rs:88-105`) is 13 fields including `Transform`, `Pedestrian`, `Itinerary`, `Router`, and `Collider` — all physical. The proposed split into persistent record (identity, assignments, queue state) vs active body (physics, rendering) is a standard ECS optimization. The 250k-citizen target makes it necessary: 250k × full `HumanEnt` is expensive; 250k × compact record + N active bodies is feasible.

## 3. Deeper mechanics

### 3a. Household time budget as conserved weekly account

The conversation names time as a resource (B1-07) but does not build the accounting identity. Here is the minimal mechanism:

**State per household:**
```
struct HouseholdTimeBudget {
    /// Total available adult-hours per week (sum of adult members × waking hours)
    total_hours: u16,
    /// Committed hours by category
    committed: TimeCommitments,
    /// Residual = total_hours - sum(committed)
    discretionary: u16,
}

struct TimeCommitments {
    formal_work: u16,      // from Work assignments
    commute: u16,          // from pathfinding distance
    shopping_queue: u16,   // from retail queue wait times
    childcare: u16,        // from household composition, childcare availability
    domestic: u16,         // base rate, reduced by service access
    household_production: u16, // from plot assignment
    healthcare: u16,       // from health state and queue times
}
```

**Conservation law:** `total_hours = sum(committed) + discretionary`. This is a hard constraint, never violated.

**What wakes it:** Any committed-hours change (job assignment, commute change, retail shortage, child birth/age-out, plot assignment, health event).

**Test:** Create a household with 2 adults × 112 waking hours = 224 total. Assign work (2 × 40h = 80h), commute (2 × 5h = 10h), base domestic (14h), shopping (6h). Discretionary = 224 - 110 = 114h. Then create a retail shortage that doubles shopping time to 12h. Discretionary drops to 108h. Assert the conservation law holds.

**Planner-visible observable:** The Social Reproduction Balance panel (B1-21) renders this directly. A district where average discretionary time is below a threshold signals time poverty.

**Performance:** One `u16` add per household per cadence tick (not every sim tick). At 250k citizens / ~100k households, this is ~100k additions per cadence — trivial.

### 3b. Housing queue as persistent ordered structure

**State:**
```
struct HousingQueue {
    entries: BTreeMap<HousingQueueKey, HouseholdID>,
    /// Indexed by household for O(1) position lookup
    positions: HashMap<HouseholdID, HousingQueueKey>,
}

struct HousingQueueKey {
    channel: HousingChannel,     // Enterprise, Municipal, Cooperative
    priority: HousingPriority,   // from policy: family size, displacement, queue age, veteran, etc.
    registration_tick: u64,       // tie-break: first registered wins
}

enum HousingChannel {
    Enterprise(CompanyID),  // fastest, tied to employment
    Municipal,              // slowest, administered by city
    Cooperative,            // partial self-build, Khrushchev-era
}
```

**Eligibility:** A household enters the queue when its per-capita living space falls below the sanitary norm (policy-set, historically 9 sq m/person). Propiska (see B1-MISSED-01) is an additional gate.

**Displacement:** When a household is displaced (building demolished, enterprise closure), it re-enters the queue with a displacement priority bonus but keeps its original registration tick.

**What wakes it:** Building completion (new dwelling capacity), household composition change (birth, death, marriage), displacement event, Planner policy change.

**Test:** Register 3 households at different ticks. Complete one dwelling. Assert the highest-priority household receives it. Assert the other two remain queued with accurate ages. Displace the assigned household. Assert it re-enters with displacement bonus.

**Planner-visible observable:** Queue length by channel, average wait time, number of households below sanitary norm, longest-waiting household.

### 3c. Blat graph — degree-bounded, favour-moving, physically grounded

**Data structure:**
```
struct BlatGraph {
    /// Adjacency: each citizen has at most MAX_TIES edges
    edges: Vec<SmallVec<[BlatEdge; MAX_TIES]>>,  // indexed by CitizenID
}

struct BlatEdge {
    other: CitizenID,
    relationship: BlatRelation,  // Kin, Coworker, Neighbor, Favour
    reciprocity_balance: i8,     // positive = they owe me; negative = I owe them
    last_activated: GameTick,
}
```

**Degree bound:** MAX_TIES = 5–8 per citizen. Ledeneva documents that blat networks were small and personal, not diffuse. This also controls performance: 250k citizens × 6 edges = 1.5M edges, each 16 bytes = 24 MB — affordable.

**How a favour moves real stock:** When a citizen has an unmet need and a blat edge to someone with access (e.g., a shop worker, a warehouse employee), the favour request checks:
1. Does the contact have physical access to the needed item? (They work at a place that has stock.)
2. Is the reciprocity balance within bounds? (Net favours owed < threshold.)
3. Is there physical stock available to divert?

If yes: The item is physically moved from the store's inventory to the requesting citizen. This is a real stock debit — the same `Resources` debit that a normal retail purchase would make. The next citizen in the formal queue who would have received that item does not get it.

**Displacement is the game mechanic:** The Planner sees that a store's inventory depleted faster than its throughput should allow. The causal chain — blat diversion → faster depletion → longer formal queue → more time poverty for non-connected citizens — is the observable consequence.

**What wakes it:** An unmet need with score above threshold + at least one blat edge to a contact with relevant access.

**Test:** Create two citizens, A and B. A has a blat edge to a shop worker. B is in the formal queue. Stock = 1 unit. A activates blat. Assert: stock debited, A's need met, B's queue position unchanged but B goes without. Assert: reciprocity_balance updated.

**Planner-visible observable:** Informal acquisition rate by district (aggregate, not individual — the Planner cannot see individual blat networks, matching Ledeneva's observation that blat is invisible to the state). Anomalous inventory depletion patterns.

### 3d. Retail information model

**What a household knows about a shop:**
```
struct ShopKnowledge {
    shop: BuildingID,
    last_seen_stock: Option<(ItemID, u16, GameTick)>,  // what, how much, when
    last_visit_tick: GameTick,
    heard_from: Option<CitizenID>,  // social information source
    estimated_queue_time: u16,       // in minutes, from last visit or rumour
}
```

A household maintains knowledge about at most N shops (N = 3–5, nearest + socially discovered).

**How it learns:**
1. **Direct visit:** On arrival at shop, updates stock knowledge to current actual.
2. **Neighbourhood rumour:** When a delivery arrives at a shop, citizens within visual range observe it. They may tell household members or blat contacts. Information propagation is physical: a citizen must be present to see the delivery.
3. **Social transmission:** A blat contact or household member who visited a shop shares their `ShopKnowledge` with a time decay (information is stale by the time it reaches you).

**How a delivery becomes a crowd:**
```
delivery arrives at shop
  → citizens in visual range observe
  → they tell household members (same tick, if at home)
  → household members tell blat contacts (next social tick)
  → informed citizens travel to shop
  → queue forms
  → stock depletes
```

This emerges without scripted events. The Planner sees: delivery → crowd → depletion → empty shelves → longer queues at other shops.

**Test:** Place a shop with zero stock. Deliver 10 units. Place 3 citizens within visual range. Assert: those citizens update ShopKnowledge within 1 tick. Assert: after social transmission tick, their contacts also have (stale) knowledge. Assert: informed citizens travel to shop. Assert: stock depletes in order of arrival.

### 3e. Turnover and productivity ramp

**State per worker:**
```
struct WorkerProductivity {
    base_skill: SkillLevel,         // from education
    workplace_experience: u16,       // ticks at current workplace
    ramp_complete_at: u16,           // ticks needed for full productivity
    current_effectiveness: f32,      // 0.0 to 1.0
}
```

**Ramp curve:** `effectiveness = min(1.0, workplace_experience / ramp_complete_at)`. Linear ramp is cheapest; a log curve is more realistic but harder to test.

**What wakes it:** Worker assignment change, tick increment while below full effectiveness.

**Planner-visible observable:** Enterprise productivity panel shows average worker effectiveness and number of workers below ramp. A high-turnover enterprise has persistently low average effectiveness — the Planner can diagnose why production is below quota without a hidden "turnover" stat.

**Test:** Assign a new worker to an enterprise. Assert effectiveness starts at the ramp floor (e.g., 0.3 for unskilled, 0.5 for qualified). Tick forward ramp_complete_at ticks. Assert effectiveness reaches 1.0. Replace worker. Assert new worker starts at ramp floor.

### 3f. Social Reproduction Balance as accounting identity

The conversation proposes this (B1-21, lines 1086–1101) but does not derive the identity. Here it is:

**For a district d in a period t:**
```
TotalAdultHours(d,t)
  = FormalEmployment(d,t)
  + Commuting(d,t)
  + HouseholdCare(d,t)
  + ShoppingQueues(d,t)
  + HouseholdProduction(d,t)
  + IllnessAbsence(d,t)
  + DiscretionaryTime(d,t)
```

This is a **conservation law**: `TotalAdultHours = sum of all categories`. It holds by construction because the household time budget (3a) is conserved.

**What the UI shows:** A stacked bar per district. The Planner compares districts: one with 40% discretionary time is thriving; one with 5% is in time poverty crisis. The Planner can see exactly which sink is consuming time — if ShoppingQueues jumped from 5% to 15%, the cause is a retail supply failure.

**The identity the UI can prove:** If the Planner improves retail supply (more shops, better logistics), ShoppingQueues shrinks and DiscretionaryTime grows. If the Planner builds a kindergarten, HouseholdCare shrinks and FormalEmployment + DiscretionaryTime grow. Every social investment is measurable in recovered hours.

### 3g. Cohort expectation memory and update rule

**State per citizen:**
```
struct ExpectationMemory {
    housing_expectation: f32,   // 0.0 = expects nothing, 1.0 = expects separate flat
    food_reliability: f32,      // 0.0 = expects famine, 1.0 = expects full shelves
    mobility_expectation: f32,  // transit access expectations
    leisure_expectation: f32,   // discretionary time expectations
}
```

**Update rule:** Exponential moving average with a slow decay:
```
expectation = expectation * (1 - alpha) + current_reality * alpha
```
where `alpha` is small (0.01–0.05 per period). Young citizens (age < 25) have higher alpha (learn faster from first experiences). Older citizens have lower alpha (expectations are sticky).

**What wakes it:** Period-end evaluation (once per plan period, not every tick). Cadence: monthly or quarterly.

**Planner-visible observable:** Average expectation by cohort (generation). A generation that grew up in housing shortage has low housing expectations and tolerates crowding. A generation that grew up with full shelves expects food reliability and reacts more strongly to shortage. This creates the "fertility echo" (B1-24): a generation with low expectations has more children despite crowding; a generation with high expectations defers family formation.

**Test:** Create two citizens: one with 50 ticks of housing shortage, one with 50 ticks of adequate housing. Assert their housing_expectation values diverge. Then give both adequate housing for 50 ticks. Assert the pessimist's expectation rises more slowly than the optimist's decays.

## 4. Missed / not apparent

### B1-MISSED-01 — Propiska (residence permits) as eligibility gate

The conversation never mentions propiska. In the Soviet system, the propiska (residence registration) was the administrative prerequisite for housing queue entry, employment, and access to city services. Without propiska, a citizen was invisible to the municipal housing queue. The propiska was tied to a specific address and city.

**Why it matters for the game:** Propiska is a natural Planner policy lever. The Planner can restrict or expand propiska to control migration, housing queue growth, and labour supply. It is the mechanism that makes limitchiki (B1-MISSED-02) possible.

**Minimal mechanic:** A boolean per citizen: `has_propiska: bool` with a city reference. Without propiska: cannot enter municipal housing queue, cannot access some services, cannot hold permanent employment (only temporary/limit contracts).

### B1-MISSED-02 — Limitchiki (limited-registration workers)

Workers recruited to big cities on temporary contracts with limited registration and collective housing (dormitories). They filled labour gaps but had restricted access to services and permanent housing. Source: Russiapedia/RT.

**Why it matters:** A Planner facing labour shortage in a city can recruit limitchiki as a fast but socially costly solution. They work but don't enter the permanent housing queue. They live in dormitories with poor conditions. They have high turnover. This creates a visible underclass the Planner must manage.

### B1-MISSED-03 — Trade unions as housing/vacation allocators

Trade unions (profsoiuz) were not just worker organizations — they administered significant welfare allocation:
- Distributed housing within enterprises
- Allocated sanatorium vouchers (putyovki): 20% free, rest at 20–30% of cost
- Managed rest homes and boarding houses
- In 1960, trade unions controlled almost all sanatoriums and rest resorts

**Why it matters:** The trade union is a second allocation channel parallel to the enterprise. If the game models enterprise welfare, the trade union is the mechanism that distributes it.

### B1-MISSED-04 — Komandirovki (business trips)

Business trips (komandirovki) served as an informal economy mechanism: travellers to better-supplied cities brought back deficit goods. A Moscow komandirovka was an opportunity to buy things unavailable in the provinces.

**Why it matters:** Komandirovki are a physical information and goods channel. A citizen on a business trip to Moscow can bring back deficit goods, updating their household's ShopKnowledge and buffering local shortages. This connects to the information model (3d) and blat (3c).

### B1-MISSED-05 — Alcohol as time, health, and productivity sink

Alcohol is entirely absent from the conversation. Quantitative impact:
- Heavy vodka consumption: 35% mortality risk at ages 35–54 for those consuming 3+ bottles/week (Lancet, 2014)
- Alcohol as barter currency: used for under-the-table transactions, ploughing, construction, repairs (Atlas Obscura)
- Productivity losses: absenteeism, reduced work quality, workplace accidents
- Gorbachev's anti-alcohol campaign (1985–1987) temporarily raised life expectancy and productivity but created massive sugar shortages (diverted to moonshine)

**Why it matters:** Alcohol is a triple-threat resource: it consumes time (drinking, recovery), damages health (reducing future labour capacity), and serves as informal currency. It is also a Planner policy lever (anti-alcohol campaign). Omitting it loses a major cause of the male mortality gap and a significant informal-economy mechanism.

### B1-MISSED-06 — Communal apartments (kommunalki) as queue transition

The conversation mentions housing queues but not the kommunalka → separate flat transition. Up to 80% of Moscow residents lived in communal setups until the mid-1960s. The transition from shared room in a kommunalka to a separate flat in a khrushchyovka was the single most important quality-of-life improvement for Soviet households.

**Why it matters:** The game's housing queue should model at least two tiers: shared housing (kommunalka / dormitory) and separate flat. Moving up is the queue transition. A household in a kommunalka has reduced privacy, shared kitchen, and conflict — modelled as reduced discretionary time and lower satisfaction.

### B1-MISSED-07 — Kitchen vs canteen split

Soviet domestic food preparation was split between the home kitchen and the enterprise/public canteen (stolovaya). Canteens provided cheap meals and reduced the household's food-preparation time burden. When canteen supply failed, the burden shifted entirely to the household kitchen.

**Why it matters:** The canteen is a service that converts enterprise food supply into recovered household time. It connects the enterprise welfare model (B1-02) to the household time budget (3a). Building a canteen at an enterprise is a Planner investment in time recovery.

### B1-MISSED-08 — Pensioners as queue-standing labour

Retired citizens (pensioners) served a specific household function: standing in queues while working-age adults were at work. This was a deliberate household scheduling strategy.

**Why it matters:** Household composition affects queue costs. A household with a pensioner can send them to queue during work hours, converting their "idle" time into shopping time and freeing working-age adults. This is a simple mechanism with large impact on the time budget.

### B1-MISSED-09 — School shifts (morning/afternoon)

Soviet schools commonly operated in two shifts due to building shortages:
- 1st shift: 8:30 AM – 2:30 PM (grades 1, 5–10)
- 2nd shift: 3:30 PM – 7:30 PM (grades 2–4)

**Why it matters:** School shifts affect household scheduling. A child in the second shift needs adult supervision in the morning. A child in the first shift needs supervision in the afternoon. Two children in different shifts can create an all-day supervision burden. This connects directly to childcare as labour transformer (B1-08) and the household time budget (3a).

### B1-MISSED-10 — Rumour as information channel

The conversation mentions citizen knowledge (B1-35) and information propagation from deliveries (lines 1065–1079) but does not name rumour as a distinct channel. Soviet citizens learned about deficit goods through word of mouth, workplace gossip, and neighbourhood observation — not through any formal information system.

**Why it matters:** Rumour is the only information channel that can propagate without physical proximity to a shop. It is lossy (information degrades), delayed (multiple social hops), and sometimes wrong (false rumours). Modelling rumour vs direct observation vs social transmission creates a realistic information landscape where better-connected citizens have better information.

### B1-MISSED-11 — Deficit goods list (defitsitnye tovary)

Certain categories of goods were persistently scarce in the Soviet Union:
- **Food:** meat, butter, coffee, exotic fruit (bananas, citrus), caviar
- **Consumer goods:** quality shoes, clothing, household appliances, electronics
- **Daily necessities:** toilet paper, laundry detergent, soap (late Soviet)
- **Luxury/prestige:** imported goods, records, books by popular authors

The deficit list varied by city (Moscow better supplied than provinces) and period (shortages worsened in the 1980s).

**Why it matters:** The game should model deficit not as "everything is equally scarce" but as specific goods cycling in and out of deficit status based on production, logistics, and priority allocation. A good's deficit status determines queue length and blat activation for that item.

## 5. Cross-lane hooks

| Item | Other lane | What they must know |
|---|---|---|
| B1-02 Enterprise welfare | A (Economy) | Enterprise cost structure must include welfare provision (housing, canteen, clinic, childcare, transport) as real material inputs, not abstract bonuses |
| B1-04 Monotown loop | A (Economy) | Production output depends on workforce stability; workforce stability depends on housing construction; housing construction depends on construction-material allocation — this is the feedback loop the economy must model |
| B1-09 Household plots | A (Economy) | Private plot food production enters the food supply outside the formal plan. It must be physically grown (seed + labour + time → food), not conjured |
| B1-12 Labour shortage | A (Economy) | Enterprise staffing is a binding constraint on production. The economy lane must treat unfilled positions as real capacity reduction, not abstract penalty |
| B1-19 Capital dilution | A (Economy) | Construction competes for the same materials as production. Too many active sites lock physical stock in unfinished assets |
| B1-22 Time poverty | Logistics (if separate lane) | Retail supply failure → longer shopping trips → time poverty. The logistics lane determines how reliably shops are stocked |
| B1-29, B1-30 Turnover/hoarding | A (Economy) | Labour hoarding and turnover affect enterprise efficiency. The economy must see these as physical states, not hidden parameters |
| B1-31 Storming | A (Economy) | End-of-period storming enters worker life as overtime/fatigue. The economy lane must propagate storming pressure into the time budget |
| B1-05, B1-MISSED-01 Housing/propiska | Governance (if any) | Housing queue and propiska are Planner policy levers that control migration and labour supply |
| B1-10 Blat | A (Economy) | Blat diverts stock from formal allocation. The economy lane must see inventory depletions that exceed legitimate throughput |

## 6. Open questions for the user

1. **Household size at spawn:** The conversation proposes households but does not say what the initial household composition should be. Historical Soviet households averaged 3.5–4.0 persons (2 adults + 1–2 children + possibly a grandparent). Should households spawn with realistic composition, or start as single adults and form households over time?

2. **Propiska as Planner policy:** Should the Planner have direct control over propiska (grant/deny residence permits), or should it be automatic based on housing availability?

3. **Alcohol:** The conversation omits alcohol entirely. It was historically the single largest cause of Soviet male excess mortality and a significant informal-economy medium. Is this in scope for the game, or is it too sensitive/complex for 1.0?

4. **Kommunalka tier:** Should the housing model distinguish between kommunalka (shared) and separate flat, with a queue transition between them? This adds significant gameplay depth but also data-structure complexity.

5. **Blat visibility to the Planner:** Ledeneva emphasizes that blat was invisible to the state. Should the Planner see individual blat transactions (unrealistic but gameplay-useful), or only aggregate anomalies (realistic but harder to act on)?

6. **Gender in the time budget:** Soviet time budgets were heavily gendered (women did 2–3× the domestic work). Should the game model gender-differentiated time burdens, or abstract household time as undifferentiated?

7. **Fertility:** The conversation proposes a fertility echo (B1-24) where housing conditions affect family formation. Is fertility/birth a 1.0 mechanism? The citizens spec leaves births as an open question (line 117).

## 7. Sources

### Academic / book sources
- Ledeneva, A.V. (1998). *Russia's Economy of Favours: Blat, Networking and Informal Exchange*. Cambridge University Press.
- Kornai, J. (1980). *Economics of Shortage*. North-Holland.
- Andrusz, G.D. (1984). *Housing and Urban Development in the USSR*. SUNY Press.
- Morton, H.W. (1980). "Who Gets What, When and How? Housing in the Soviet Union." *Soviet Studies*.
- Lovell, S. (2003). *Summerfolk: A History of the Dacha*. Cornell University Press.
- Feshbach, M. & Rapawy, S. (1973). "Labor constraints in the five-year plan." In JEC *Soviet Economic Prospects for the Seventies*.
- Gordon, L.A. & Klopov, E.V. (1972). *Man After Work* (Chelovek posle raboty). Moscow.
- Strange, C. (2021). *Monotown: Urban Dreams, Brutal Imperatives*. University of Toronto Press.
- Connor, W.D. (1977). "The Soviet Worker: Social Stratification and Political Perceptions." Wilson Center.
- Zaslavskaya, T.I. (1988). Social stratification framework (Novosibirsk school).
- Szalai, A. (1970s). "Women's Time." Time-budget studies.

### Web sources
- [UPI Archives — Soviet housing shortage (1988)](https://www.upi.com/Archives/1988/05/08/Soviet-housing-shortage-chronic-One-in-five-Soviets-still-waits-for-proper-housing/2689579067200/)
- [Encyclopedia.com — Administration for Organized Recruitment](https://www.encyclopedia.com/history/encyclopedias-almanacs-transcripts-and-maps/administration-organized-recruitment)
- [Russiapedia — Limitchik](https://russiapedia.rt.com/of-russian-origin/limitchik/index.html)
- [Jamestown Foundation — The Dacha: Russia's Retreat](https://jamestown.org/the-dacha-russias-retreat-soul-saver-and-key-food-supplier/)
- [Cambridge Core — Private Sector in Soviet Agriculture](https://www.cambridge.org/core/services/aop-cambridge-core/content/view/F3DB87C39F65AD129B8116622E368756/S0037677900132188a.pdf/)
- [P2P Foundation — Dacha Model](https://wiki.p2pfoundation.net/Dacha_Model_of_Familial_Food_Production_in_Russia)
- [Atlas Obscura — How Vodka Became Currency](https://www.atlasobscura.com/articles/vodka-currency-russia)
- [Wikipedia — Communal apartment](https://en.wikipedia.org/wiki/Communal_apartment)
- [Wikipedia — Job by distribution](https://en.wikipedia.org/wiki/Job_by_distribution)
- [Wikipedia — Monotown](https://en.wikipedia.org/wiki/Monotown)
- [Wikipedia — Consumer goods in the Soviet Union](https://en.wikipedia.org/wiki/Consumer_goods_in_the_Soviet_Union)
- [Qminder — The Art of Soviet Queues](https://www.qminder.com/blog/queue-management/queues-in-ussr/)
- [Lancet/BMJ — Alcohol and mortality in Russia (2014)](https://www.ncbi.nlm.nih.gov/pmc/articles/PMC4007591/)
- [Open Book Publishers — Holiday Convergences (putyovka)](https://books.openbookpublishers.com/10.11647/obp.0171/ch4.xhtml)
- [LSE Blog — Soviet inequality](https://blogs.lse.ac.uk/europpblog/2025/09/09/soviet-communism-was-no-more-successful-at-reducing-inequality-than-other-regimes/)
- [New East Archive — Soviet sanatoriums](https://www.new-east-archive.org/features/show/9100/holidays-in-soviet-sanatoriums-ussr-tourism-photography)
- [Seventeen Moments in Soviet History — Women in the Work Force](https://soviethistory.msu.edu/1980/moscow-doesnt-believe-in-tears/moscow-doesnt-believe-in-tears-texts/women-in-the-work-force/)
- [Cambridge Core — Labor Turnover in the Soviet Union](https://www.cambridge.org/core/journals/slavic-review/article/abs/labor-turnover-in-the-soviet-union/731B07BACB1812AB81E2925D985DFE6D)

### Codebase files read
- `simulation/src/souls/human.rs:1-278` — HumanEnt decision loop, PersonalInfo, spawn_human
- `simulation/src/souls/desire/home.rs:1-28` — Home desire (constant score 0.2, single BuildingID)
- `simulation/src/souls/desire/buyfood.rs:1-179` — BuyFood state machine (bread only)
- `simulation/src/souls/desire/work.rs:1-77` — Work desire (Worker/Driver, time interval scoring)
- `simulation/src/souls/mod.rs:1-56` — add_souls_to_empty_buildings (one human per empty house)
- `simulation/src/world.rs:1-120` — HumanEnt struct (13 fields, no household)
- `simulation/src/economy/government.rs:1-84` — Government (money only, no citizen creation)
- `simulation/src/map_dynamic/binfos.rs:1-73` — BuildingInfos (owner/inside tracking)
- `docs/reference/specifications/citizens.md` — Citizens spec (all evidence UNIMPLEMENTED)
- `docs/reference/specifications/households.md` — Households spec (all evidence UNIMPLEMENTED)
- `docs/reference/specifications/needs.md` — Needs spec (all evidence UNIMPLEMENTED)
- `docs/reference/specifications/education.md` — Education spec (all evidence UNIMPLEMENTED)
- `docs/reference/specifications/healthcare.md` — Healthcare spec (all evidence UNIMPLEMENTED)
- `docs/reference/specifications/buildings.md:1-80` — Buildings spec
- `docs/reference/specifications/zoning.md:1-80` — Zoning spec
- `docs/reference/glossary.md` — Project glossary
- `docs/archive/agents-2026-09-02/settlement-modeller.md` — Settlement modeller agent definition (households as shared-pantry units)
