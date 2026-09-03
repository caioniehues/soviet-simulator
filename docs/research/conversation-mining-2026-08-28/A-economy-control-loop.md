# Lane A — The planned economy as a control system

**Kind:** research
**Authority:** research
**Status:** active
**Owner:** project lead
**Last verified:** 2026-09-03
**Source:** GPT conversation export `gpt-vision-export-2026-08-28.md` (2026-08-28), validated against code paths cited inline; synthesis 2026-08-28

## 0. Summary (top ten findings)

1. **A-01 Causal-distinctness rule**: CONFIRMED as the correct design heuristic. Splits only where
   routing, bottleneck, or consequence changes — validated by Kornai's framework and by the game's
   existing resource list.
2. **A-02 Self-generated shortage spiral**: CONFIRMED by Kornai (1980). The feedback loop
   (unreliable → hoard → scarce → less reliable) is the foundational mechanism of the shortage
   economy. The code has a partial prototype (`request_multiplier`) but not the adaptive spiral.
3. **A-05 Ratchet effect**: CONFIRMED by Weitzman (1980). The conversation's description is accurate
   to the literature. No code substrate exists.
4. **A-06 Planning credibility**: PLAUSIBLE but the specific percentages (61%, 67%, 82%, 91%) are
   invented illustrations, not sourced data. The concept of institutional memory for practical
   reliability is well-attested by Gregory & Harrison.
5. **A-07 Five reserve classes**: PLAUSIBLE as game design. Soviet reserve categories did exist
   (state reserve, enterprise buffer stocks, operational inventory), but the exact five-class
   taxonomy is the conversation's invention. No code substrate.
6. **A-08 Freight-plan stability numbers** (72%/141%/24%/18%): UNSUPPORTED. These are illustrative
   numbers with no citation. Soviet rail statistics exist but at aggregate national level; these
   corridor-level figures are fabricated examples.
7. **A-10 Material-balance identity**: CONFIRMED by Gosplan practice. The identity
   `stock + production + arrivals − consumption − departures = Δstock` is a correct physical
   restatement of the standard material balance equation.
8. **A-12 Priority cannot solve scarcity**: CONFIRMED by Kornai and Nove. Priority reallocates
   shortage, it does not eliminate it. This is a first-principles truth of a finite-resource system.
9. **A-15 Tolkachi/expediters MISSED**: The conversation never mentions tolkachi — the semi-official
   supply expediters who were central to Soviet enterprise operations. This is a significant gap
   for the game's economy model.
10. **A-16 Ministry as intermediate aggregator MISSED**: The conversation treats planning as a
    two-level system (Planner → enterprise). Real Soviet planning had ministries that themselves
    inflated aggregated requests. This intermediate layer is absent.

---

## 1. Extracted items

| ID | Statement | Source line(s) | Verdict |
|---|---|---|---|
| A-01 | Split a resource/operation only when the distinction changes routing, storage, substitution, bottlenecks, allocation, quality, timing, or visible consequences ("causal distinctness") | 107–113 | CONFIRMED |
| A-02 | Self-generated shortage spiral: unreliable supply → enterprises request extra → available stock falls → other enterprises experience shortages → requests increase → dispatch overloads → deliveries less reliable | 177–198 | CONFIRMED |
| A-03 | Reported need ≠ true need: a plant requiring 100 t of copper might request 145 t for five named reasons (reliability buffer, expected cuts, plan-risk buffer, reserve building, bargaining slack) | 166–175 | CONFIRMED |
| A-04 | Electronics/space cascade: a priority programme propagates demand shocks upstream through overtime, maintenance deferral, larger batches, reserve releases, substitutions, larger requests | 200–211 | PLAUSIBLE |
| A-05 | Priority cannot solve scarcity; priority only decides where scarcity appears | 207–208 | CONFIRMED |
| A-06 | Planning credibility as institutional memory: six specific stats (61% inputs received, 67% dates met, 82% emergency honored, 91% overfulfillment → new quota, "frequent" plan changes) | 257–274 | PLAUSIBLE (concept) / UNSUPPORTED (numbers) |
| A-07 | Five reserve classes: operating stock, safety stock, enterprise reserve, state reserve, project reserve | 279–285 | PLAUSIBLE |
| A-08 | Freight-plan stability numbers: 72% avg utilization, 141% peak, 24% emergency dispatch share, 18% empty repositioning | 218–224 | UNSUPPORTED |
| A-09 | Storming propagates upstream recursively: space → electronics → wire → copper → mine → rail congestion | 230–244 | CONFIRMED |
| A-10 | Material-balance identity: stock + production + arrivals − consumption − departures = Δstock | 308–311 | CONFIRMED |
| A-11 | "Every macroeconomic number must eventually resolve into physical or institutional state" (golden rule) | 315 | CONFIRMED (design principle) |
| A-12 | Ratchet effect: if exceptional performance becomes basis for next quota, enterprises conceal capacity | 250–255 | CONFIRMED |
| A-13 | Slack as resilience: a nominally less aggressive plan can outperform a taut one by absorbing shocks | 277 | CONFIRMED |
| A-14 | Construction opportunity cost: prioritizing housing reduces machine-building later; prioritizing heavy industry worsens housing and labor | 298–299 | CONFIRMED |
| A-15 | Specialization vs resilience: an enterprise with unreliable bearing deliveries may create an inefficient local bearing workshop because reliability has value | 288–289 | PLAUSIBLE |
| A-16 | Indicator design: "what you measure becomes what enterprises optimize" — gross tonnage, assortment, quality, etc. each produce different distortions | 292–295 | CONFIRMED |
| A-17 | Generalized reliability→reserve mechanism: factories, households, railways, hospitals all build buffers when they distrust supply | 1117–1128 | CONFIRMED |
| A-18 | Mature system as physical calm: fewer emergency requests, smaller safety stocks, shorter queues, predictable trains, lower turnover, more accurate reporting, less storming | 1130–1141 | PLAUSIBLE |
| A-19 | The Plan as a force that deforms physical systems, not a quota UI sitting on top | 123–127 | CONFIRMED (design principle) |
| A-20 | The control-system feedback loop: Plan → quotas → enterprises → request/produce/hoard → logistics → actual flows → shortages/surplus/queues → observed results → Plan | 145–163 | CONFIRMED |
| A-21 | Citizens deepen the loop: strategic projects pulling engineers create demand for housing, food, heating, schools, health, transit; true labor footprint is much larger than direct workforce | 301–305 | CONFIRMED |
| A-22 | Closing thesis: physically simulated planned society in which material flows, institutions, information, infrastructure, households, persistent citizens, and long-term social reproduction continuously reshape one another | 1197–1198 | CONFIRMED (design thesis) |

---

## 2. Validation detail

### A-01 Causal-distinctness rule
The heuristic is sound design theory. Kornai's *Economics of Shortage* (1980) does not prescribe
granularity, but the principle that "a distinction matters only when it changes allocation behavior"
is implicit in material-balance planning practice: Gosplan tracked ~1,943 product categories in 1973
([Material balance planning, Wikipedia](https://en.wikipedia.org/wiki/Material_balance_planning)),
not the millions of individual SKUs. The game's existing 21 Lua items
(`base_mod/items.lua`) already live at approximately this level. The rule correctly prevents
"Copper Bolt Type 7 Simulator" without mandating a specific count.

**Code substrate**: The `ItemPrototype` type (`prototypes/src/prototypes/item.rs:6-25`) carries only
`id`, `label`, and `optout_exttrade`. No unit, mass, volume, storage class, or transport class
metadata exists — the resources spec (`docs/reference/specifications/resources.md` §Substrate)
already notes this gap.

### A-02 Self-generated shortage spiral
Kornai (1980) formalized this as the core mechanism of the shortage economy: "Firms hoarded inputs
and labor to buffer against supply uncertainties, exacerbating chronic shortages rather than resolving
them through efficiency gains"
([Shortage economy, Wikipedia](https://en.wikipedia.org/wiki/Shortage_economy)). The feedback loop
in the conversation (lines 177–198) accurately describes Kornai's analysis.

**Code substrate**: `request_multiplier` (`prototypes/src/types/recipe.rs:52`) is the static seed of
this spiral. It is wired end-to-end: `recipe_init` (`simulation/src/souls/goods_company.rs:23`)
calls `market.set_requested(soul, item.id, qty)` where `qty = item.amount * request_multiplier`.
Two Lua entries set it: `flour-factory` at 4, `slaughterhouse` at 3
(`base_mod/companies.lua:40,582`). The remaining ~24 companies default to 1.

The `SCENARIO-0151` test (`simulation/src/tests/scenarios/hoarding.rs:253`) proves that an inflated
requester accumulates surplus while an honest requester does not, and `sov-lpj`
(`simulation/src/tests/scenarios/inflation.rs:143`) proves the production path wires
`request_multiplier` end-to-end.

**What is missing from the spiral**: The multiplier is a *static per-prototype constant*, not a
dynamic function of experienced reliability. The conversation describes an adaptive spiral where
enterprises *learn* to inflate; the code has only a compile-time fixed inflation level. No
`reliability_memory`, `experienced_fulfillment_rate`, or equivalent state exists in `GoodsCompanyState`
(`simulation/src/souls/goods_company.rs:69-78`). The spiral is seeded but not dynamic.

### A-03 Reported need ≠ true need
Directly from Kornai. The five named reasons are accurate to the literature. Berliner (*Factory and
Manager in the USSR*, 1957) documents the same practice: "hoarding of plant capacity, labour and
inventories" driven by "chronic supply shortages"
([Berliner, Google Books](https://books.google.com/books/about/Factory_and_Manager_in_the_USSR.html?id=5ASaAAAAIAAJ)).

**Code substrate**: `SingleMarket::requested` (`market.rs:50`) stores the inflated request distinct
from `capital` (actual stock). `SPEC-PRODUCTION-003` in the production spec explicitly requires
"Requested, received, consumed, on-hand, reserved, in-custody, and surplus quantities are distinct."
`SPEC-PRODUCTION-009` further specifies that "an enterprise MAY report a requirement above the
recipe's actual consumption." The specification is aligned with the conversation. The current
code implements only two of these distinctions (requested vs capital).

### A-05 Priority cannot solve scarcity
This is a formal consequence of the no-teleport pillar and finite resources. Kornai and Nove both
document that priority sectors (defense, space) received preferential allocation at the direct
expense of consumer goods. Nove: "quality was often sacrificed to fulfill plans in quantitative terms"
([Nove, New Left Review](https://newleftreview.org/issues/i119/articles/alec-nove-problems-and-prospects-of-the-soviet-economy.pdf)).

### A-06 Planning credibility stats
**Concept**: Gregory & Harrison (2005) document from Stalin's archives that enterprise compliance was
shaped by the credibility of the planning system's promises
([Gregory & Harrison, JEL 2005](https://wrap.warwick.ac.uk/id/eprint/164/1/WRAP_Harrison_jel05.pdf)).
The concept of tracking reliability metrics as institutional memory is well-founded.

**Numbers**: The specific percentages (61%, 67%, 82%, 91%) appear in no source I can find. They are
illustrative examples crafted by the conversation, not sourced data. The conversation does not claim
them as historical; they function as a UI mockup. Verdict: concept CONFIRMED, numbers are
**invented illustrations**.

### A-08 Freight-plan stability numbers
The four statistics — 72% average corridor utilization, 141% peak utilization, 24% emergency dispatch
share, 18% empty repositioning — have no citation and do not appear in any source I checked. Soviet
rail statistics at the national level exist (e.g., the USSR handled roughly half the world's rail
freight tonne-km in its last decades
([Rail transport in Russia, Wikipedia](https://en.wikipedia.org/wiki/Rail_transport_in_Russia))),
but corridor-level utilization breakdowns are not publicly available in the literature I searched.
The 141% peak exceeding 100% is plausible if "utilization" means "demand relative to scheduled
capacity," but this specific number set is **fabricated for illustration**.

### A-09 Storming cascades upstream
Shturmovshchina is one of the best-documented phenomena of Soviet management. The three-phase
monthly cycle — *spyachka* (hibernation), *gorychka* (hot time), *likhoradka* (feverish frenzy) —
is described in multiple sources
([Shturmovshchina, Wikipedia](https://en.wikipedia.org/wiki/Shturmovshchina);
[GlobalSecurity.org](https://www.globalsecurity.org/military/world/russia/industry-storming.htm)).
The *recursive* upstream propagation (space → electronics → wire → copper → mine → rail) is the
conversation's own construction, but it follows logically from the documented cascade pattern and
physical supply-chain coupling. Confirmed as a valid mechanism.

### A-10 Material-balance identity
The standard Gosplan material balance equation is: `Q_t + Q_{t-1} + M_t = ID_t + FD_t + X_t`
(production + inventories + imports = inter-industry demand + final demand + exports)
([Material balance planning, Wikipedia](https://en.wikipedia.org/wiki/Material_balance_planning);
[Encyclopedia.com](https://www.encyclopedia.com/history/encyclopedias-almanacs-transcripts-and-maps/material-balances)).
The conversation's restatement `stock + production + arrivals − consumption − departures = Δstock`
is an equivalent physical-flow form. This is correct.

### A-12 Ratchet effect
Weitzman (1980) formalized this in "The 'Ratchet Principle' and Performance Incentives" (*Bell
Journal of Economics*, 11(1), pp. 302–308)
([Weitzman 1980, Harvard](https://scholar.harvard.edu/files/weitzman/files/rachetprincipleperformanceincentives.pdf)):
"higher rewards from better current performance must be weighed against the future assignment of
more ambitious targets." The conversation's description (lines 250–255) is accurate to this
literature. Berliner (1957) also documents capacity concealment as standard Soviet managerial
practice.

### A-16 Indicator design
Nove extensively documents how metric choice drives enterprise behavior: tonnage targets produce
excessive weight, ruble targets reward expensive inputs, assortment plans are neglected in favor of
the easiest-to-produce items. This is the "Goodhart's Law" of Soviet planning and is
comprehensively confirmed.

---

## 3. Deeper mechanics

For each mechanism the conversation sketched but did not design to implementation depth:

### 3a. Enterprise request-inflation function driven by remembered reliability

**Current state**: `request_multiplier` is a static `i32` in the `Recipe` prototype
(`prototypes/src/types/recipe.rs:52`), defaulting to 1. It is baked into Lua data and never changes
at runtime.

**Minimal data structure**:
```
per enterprise (in GoodsCompanyState or a new companion struct):
  reliability_memory: f32        // exponential moving average of fulfillment_rate
  fulfillment_rate: f32          // (received_qty / requested_qty) over the last N cycles
  effective_multiplier: f32      // computed from reliability_memory: low reliability → higher multiplier
  request_age: u32               // ticks since the last fulfilled delivery per input
```

**What ticks it**: After each recipe cycle completes (in `recipe_act`,
`simulation/src/souls/goods_company.rs:52`), update `fulfillment_rate` from the ratio of what was
actually received (capital gained during the cycle) to what was requested (`set_requested`). Blend
into `reliability_memory` with exponential decay:
`reliability_memory = α * fulfillment_rate + (1-α) * reliability_memory`.
Compute `effective_multiplier = base_multiplier / max(reliability_memory, floor)` where `floor`
prevents division by zero and `base_multiplier` is the Lua prototype's `request_multiplier`.

**The test that proves it**: A two-enterprise scenario (extending SCENARIO-0151) where both start
with `request_multiplier = 1`. One enterprise has its deliveries artificially delayed (by removing
and restoring its road connection). After N cycles, the delayed enterprise's
`effective_multiplier` should be strictly higher than the reliably-served one's, and its
accumulated stock should be higher even though both consume the same amount.

**Observable for the Planner**: The inspector shows `requested` vs `received` vs `consumed` per
input per cycle, plus `reliability_memory` as a bar or percentage. A discrepancy between `requested`
and `consumed` is the Planner's primary hoarding signal.

### 3b. Ratchet — how quota memory updates and how the player sees it

**Current state**: No quota mechanism exists. The `Government` struct
(`simulation/src/economy/government.rs:9-11`) holds only `money: Money`. No production quota, no
quota history, no performance tracking.

**Minimal data structure**:
```
per enterprise, per plan period:
  quota: u32                      // the Planner-set target output for this period
  actual_output: u32              // what was physically produced (counted by recipe_act)
  quota_history: RingBuffer<(u32, u32), 8>  // (quota, actual) for the last N periods
  visible_capacity_estimate: f32  // what the Planner believes this enterprise CAN produce
```

The ratchet manifests when the Planner (or an auto-planner system) uses `actual_output` from period
N to set `quota` for period N+1. If the enterprise overfulfilled, the new quota rises. The
enterprise's rational response: produce exactly at quota, never above, to avoid the ratchet raising
it further.

**What ticks it**: At plan-period boundaries (a new phase gate in the simulation tick), record
`(quota, actual_output)` into `quota_history`, reset `actual_output` to 0. The auto-quota
algorithm (if any) sets next-period quota as `max(quota, actual_output) * growth_factor`.

**The test that proves it**: Enterprise A overfulfills for two periods. Enterprise B produces
exactly at quota. After two periods, A's quota should be strictly higher than B's (ratcheted up),
even though A's true capacity is the same as B's. The Planner observes this through
`quota_history`: a pattern of "last period's actual became this period's quota" signals the ratchet
is active.

**Observable for the Planner**: A timeline view of `(quota, actual)` per period. The ratchet is
visible as a staircase where quota tracks previous actual. The player-as-Planner can manually
override quotas below ratcheted levels to restore trust, at the cost of lower immediate output.

### 3c. Storming as per-enterprise temporal demand profile

**Current state**: Production happens at a constant rate scaled by `productivity` (workforce and
electricity). `company_system` (`simulation/src/souls/goods_company.rs:192-218`) advances
`progress` by `productivity * DELTA / recipe.duration.seconds()`. There is no plan-period
awareness, no deadline behavior, no temporal bunching.

**Minimal data structure**:
```
per enterprise:
  plan_period_ticks_remaining: u32     // countdown to period end
  plan_period_target: u32              // output target for this period
  period_output_so_far: u32            // what has been produced so far this period
  storming_state: enum { Normal, Storming }
  storming_multiplier: f32             // 1.0 normal, up to 1.5 under storming
```

**What ticks it**: When `plan_period_ticks_remaining / total_period_ticks < threshold` AND
`period_output_so_far / plan_period_target < shortfall_threshold`, the enterprise enters storming.
Storming multiplies `productivity` by `storming_multiplier` but also multiplies input consumption
rate by the same factor (driving larger raw-material requests into the logistics system) and
degrades quality (if quality grades exist).

**The test that proves it**: An enterprise with a quota it cannot meet at normal rate enters
storming in the final third of the plan period. Its input request rate doubles. A downstream
enterprise that was previously adequately supplied now faces a demand spike and begins to lag on
its own quota. The test asserts the cascade by checking both enterprises' `period_output_so_far`
relative to target at period boundaries.

**Observable for the Planner**: The freight system shows a spike in dispatch requests in the final
third of each plan period. The material-balance UI shows elevated input requests against static
supply. A congestion indicator on rail/road corridors rises. The Planner can diagnose "your
industrial plan produces terrible temporal demand" (conversation line 228) from these observables.

### 3d. Planning-credibility record as institutional memory with decay

**Current state**: No institutional memory exists in the code. `Government` has only `money`.

**Minimal data structure**:
```
global (in Government or a new PlanningAuthority struct):
  credibility: f32                    // [0, 1], exponential moving average
  fulfilled_promise_count: u32        // promises kept this period
  broken_promise_count: u32           // promises broken this period
  confiscation_memory: f32            // how often the Planner seized reserves
  mid_period_revision_count: u32      // how many plan changes this period

per enterprise:
  trust_in_plan: f32                  // [0, 1], derived from credibility + own experience
```

**What ticks it**: At each plan-period boundary, compute `credibility` from the ratio of
fulfilled to total promises (delivery promises kept, quotas not unilaterally raised, reserves not
confiscated). Blend with decay: `credibility = α * (fulfilled / total) + (1-α) * credibility`.
Enterprise `trust_in_plan` is a weighted average of global `credibility` and the enterprise's own
`reliability_memory` (see 3a). Low `trust_in_plan` increases `effective_multiplier` (more hoarding),
decreases voluntary reporting accuracy, and increases propensity to create local workshops (3e/A-15).

**The test that proves it**: The Planner confiscates enterprise reserves (a planned mechanic) three
times. `credibility` should drop measurably. Enterprises that experienced confiscation should have
lower `trust_in_plan` than those that did not, and should subsequently request more inputs.

**Observable for the Planner**: A "Planning Authority Credibility" metric visible on the
material-balance UI. The four behavioral indicators the conversation lists (lines 267–270) — if
always cuts requests → they inflate; if confiscates reserves → they hide; if overfulfillment →
higher quota → they conceal capacity; if delivery reliable → safety stocks shrink — are exactly the
signals this metric tracks.

### 3e. Five reserve classes as custody states that never teleport

**Current state**: `SingleMarket` tracks `capital` (on-hand), `reserved` (matched but not yet
picked up), and `requested` (declared need). There is no distinction between types of on-hand stock.
A single `i32` represents all stock at an enterprise.

**Minimal data structure**:
```
per enterprise, per item:
  operating_stock: u32       // what the current recipe cycle is consuming from
  safety_stock: u32          // minimum buffer below which the enterprise increases requests
  enterprise_reserve: u32    // hidden surplus the enterprise does not report
  state_reserve: u32         // stock the Planner has explicitly allocated as strategic reserve
  project_reserve: u32       // stock earmarked for a specific national project

  // Physical constraint: sum of all five classes == the enterprise's real physical stock.
  // None of these can go negative. Transfer between classes is an explicit action, not a balance.
```

**What ticks it**: `recipe_act` consumes from `operating_stock` first. If `operating_stock` is
depleted, the enterprise draws from `safety_stock` (with a credibility penalty — this signals
unreliability). `enterprise_reserve` is never drawn automatically — it is the enterprise's hidden
surplus, the hoarding behavior the Planner must detect. `state_reserve` is drawn only by a
Planner action (confiscation or reallocation). `project_reserve` is drawn by a national project
system.

**The test that proves it**: Set up an enterprise with 100 units: 40 operating, 20 safety, 20
enterprise_reserve, 10 state_reserve, 10 project_reserve. Starve it of deliveries. It should
consume operating_stock first, then safety_stock (with a recorded event), but NEVER touch
enterprise_reserve, state_reserve, or project_reserve automatically. The sum of all five classes
should always equal the enterprise's physical stock (conservation invariant). The Planner can
observe `enterprise_reserve > 0` even while the enterprise reports "shortage."

**Observable for the Planner**: The inspector shows the five classes. The Planner can SEE
`enterprise_reserve` if they inspect closely enough (physical stock − operating − safety −
state − project = the hidden surplus), but the enterprise's own *report* omits it. This is the
dishonest-enterprise inference loop: `SPEC-PRODUCTION-009`'s "infer suspected deception from
inspectable request, receipt, consumption, on-hand, surplus, and outstanding-request-age
discrepancies" is exactly this.

### 3f. Material-balance identity and the UI that proves it

**Current state**: `EcoStats` (`simulation/src/economy/ecostats.rs`) tracks ring-buffered histories
of exports, imports, and internal trade volume per item. It does not track production, consumption,
or stock levels — only trade matches. The material balance cannot be computed from current state.

**Minimal data structure**:
```
per item, per period (extending EcoStats or a new MaterialBalance struct):
  opening_stock: i64          // total across all holders at period start
  domestic_production: i64    // recipe_act outputs this period
  arrivals: i64               // imports cleared this period
  consumption: i64            // recipe_act inputs this period + retail consumption
  departures: i64             // exports cleared this period
  closing_stock: i64          // total across all holders at period end

  // Invariant: opening_stock + domestic_production + arrivals
  //           − consumption − departures == closing_stock
```

**What ticks it**: Each `recipe_act` call updates `domestic_production` (output) and `consumption`
(input). Each trade clearance updates `arrivals` or `departures`. At period boundaries, compute
`closing_stock` from a sweep of all holders' `capital` and verify the identity. A discrepancy
is a ledger bug and should be caught by the `ledger-invariant-checker`.

**The test that proves it**: Run 100 ticks of a two-enterprise, one-item economy. At every period
boundary, assert `opening + production + arrivals − consumption − departures == closing`. Mutate
one side (e.g., silently create stock) and assert the identity breaks.

**Observable for the Planner**: The material-balance UI the conversation describes (lines 308–316):
"every aggregate number should be clickable down to real trains, yards, warehouses, factories,
and households." The identity's terms are the drill-down roots. A non-zero discrepancy
(opening + production + arrivals − consumption − departures − closing ≠ 0) would be a visible
bug indicator for the Planner — it means something teleported.

---

## 4. Missed / not apparent

### M-01 Tolkachi / expediters
The conversation never mentions tolkachi — the semi-official supply expediters who were one of
the most distinctive features of Soviet enterprise management. Berliner (1957) describes them as
occupying "a key position mediating between the enterprises and the commissariat" by 1937
([Tolkach, Wikipedia](https://en.wikipedia.org/wiki/Tolkach)). They were "premier practitioners
of blat" — using personal networks to procure scarce inputs. For the game, this is a natural
emergent mechanic: when the official allocation system fails, enterprises should be able to
assign a worker as an expediter who physically travels to supplier enterprises and uses informal
relationships (a social stat) to jump the queue. This creates a player-visible signal (workers
disappearing from production into procurement roles) and a meaningful cost (reduced labor at
the enterprise). It also creates a second allocation topology alongside the planned one — exactly
what the game's core loop requires.

### M-02 Soft budget constraint's physical analogue
Kornai's soft budget constraint is fundamentally monetary: the state bails out failing enterprises.
Since this game has no domestic money, the physical analogue is more important: the Planner cannot
let an enterprise fail (never game over), so they must physically reallocate scarce inputs from
performing enterprises to underperforming ones. This creates a perverse incentive: enterprises that
*underperform* receive emergency allocations, while enterprises that perform well have their
surplus confiscated. The conversation touches this indirectly (reserve confiscation, line 269) but
never names it as the soft budget constraint or works out the physical mechanism. The game should
make this an explicit tension: the Planner's rescue of failing enterprises degrades the reliable
ones.

### M-03 Plan-fulfilment percent as reported figure decoupling from physical output
The conversation discusses the ratchet (A-12) but does not address a related problem:
plan-fulfilment percent itself is a reported number that enterprises manipulate. Soviet managers
inflated reported output ("forging success" — see Harrison, *Forging Success: Soviet Managers
and False Accounting, 1943 to 1962*
([ResearchGate](https://www.researchgate.net/publication/46454796_Forging_Success_Soviet_Managers_and_False_Accounting_1943_to_1962))).
For the game, this means the Planner should not automatically trust reported output: the inspector
should compare *reported* production against *physical stock changes* and flag discrepancies.

### M-04 Assortment plans
The conversation mentions assortment (line 295) but does not develop it. In Soviet planning, an
aggregate output target (e.g., "100 tonnes of metal goods") could be met by producing only the
easiest items while neglecting harder-to-produce items the economy actually needed. Gosplan used
assortment plans to specify the mix, but enterprises evaded them. For the game, this means the
quota system should specify both aggregate output AND mix. An enterprise that meets aggregate
quota with the wrong mix should be detectable by the Planner.

### M-05 Plan correction cycle
The conversation notes that plan changes were "frequent" (line 266) but does not design the
correction cycle. In practice, annual plans were continuously revised through quarterly and monthly
operational plans, each revision propagating new supply-demand imbalances. For the game, this
means the Planner's own plan changes create turbulence: every mid-period revision disrupts
logistics, invalidates enterprise safety stocks, and degrades planning credibility (3d).

### M-06 Investment hunger
Kornai identified "investment hunger" — the insatiable demand for new fixed capital in a shortage
economy where the budget constraint is soft. Enterprises always request more investment than they
can absorb, leading to chronic construction-in-progress backlogs. For the game, this manifests as
the construction opportunity cost the conversation mentions (A-14), but the underlying mechanism
(enterprises requesting buildings they cannot staff or supply) is a separate dishonest-enterprise
behavior pattern the Planner must detect.

### M-07 Forced-substitution chain
The conversation mentions "approved substitutions" in the reconstructed context (line 59) and
"specialization vs resilience" (A-15) but does not design how substitution cascades through the
economy. When input A is unavailable, the enterprise substitutes B. But B was allocated to a
different enterprise, which now faces its own shortage and substitutes C. The chain propagates.
For the game, substitution should be an explicit action with a quality/efficiency penalty, and the
chain should be traceable in the inspector.

### M-08 Ministry as intermediate aggregator
The conversation models planning as Planner → enterprise. Real Soviet planning had a
three-level hierarchy: Gosplan → ministries → enterprises. Ministries themselves inflated requests
(aggregating enterprise requests and adding their own buffers) and concealed capacity from Gosplan.
For the game, if ministry-level aggregation is ever introduced, it adds a second layer of
dishonest-enterprise behavior — the "dishonest ministry" is harder to detect because it
aggregates multiple enterprises' data.

### M-09 Quality attestation
The conversation mentions quality degradation under storming (line 248) but does not design a
quality attestation system. In Soviet practice, state quality inspectors (OTK — Otdel
Tekhnicheskogo Kontrolya) could reject output that failed standards, but managers pressured them
to pass defective goods near plan deadlines. For the game, a quality gate on production output
would make storming costly: rushed output fails quality checks, requires rework, and consumes
additional inputs — deepening the shortage spiral.

---

## 5. Cross-lane hooks

| What | Lane(s) that must know |
|---|---|
| Storming creates a temporal demand spike that overloads the freight system | Lane D (vehicles/logistics) — storming bunches dispatch requests |
| Planning credibility affects citizen behavior (migration, morale, informal economy) | Lane B (society/citizens) — `trust_in_plan` drives household decisions |
| Reserve confiscation is a Planner action with social consequences | Lane B — confiscation degrades enterprise worker morale |
| Tolkachi as worker assignment diverts labor from production | Lane B — labor allocation, household time |
| Material-balance UI requires clickable drill-down to physical entities | Lane C (Rust architecture) — UI architecture, inspector design |
| Construction opportunity cost competes for the same resource pool | Lane D — construction logistics, material transport |
| Electricity blackout already stops production (`CompanyEnt::productivity`, `goods_company.rs:104-108`) | Lane E (if utilities lane exists) — electricity ties directly into production |
| Quality under storming creates rework demand | Lane B — worker time, Lane D — logistics for rework inputs |

---

## 6. Open questions for the user

1. **Adaptive multiplier or Planner-set?** Should enterprises dynamically adjust their request
   inflation based on experienced reliability (section 3a), or should the Planner manually set
   per-enterprise request limits? The first is more realistic; the second gives the player more
   direct control.

2. **Plan periods as game structure?** The conversation implies periodic plan cycles but does not
   specify whether the player defines plan-period boundaries or whether they emerge from the
   simulation. The quota, ratchet, storming, and credibility mechanisms all depend on a defined
   period structure.

3. **Tolkachi: emergent or designed?** Should expediter behavior emerge from a worker-assignment
   system (worker travels physically to another enterprise to negotiate), or should it be a
   button the enterprise presses? The physical travel version is more consistent with "nothing
   teleports."

4. **How many reserve classes?** Five classes (section 3e) may be too many for the player to
   manage. Two or three (operating, reserve, state) might deliver the same gameplay loop with
   less cognitive load. The hidden enterprise reserve is the essential one for the
   dishonest-enterprise detection loop.

5. **Ministry layer?** Is a ministry (intermediate aggregator between Planner and enterprise)
   in scope for any iteration? It would add strategic depth but also complexity.

---

## 7. Sources

### Academic and reference works
- Kornai, J. (1980). *Economics of Shortage*. Amsterdam: North-Holland. [Google Books](https://books.google.com/books/about/Economics_of_shortage.html?id=zi3sAAAAMAAJ)
- Berliner, J.S. (1957). *Factory and Manager in the USSR*. Harvard University Press. [Google Books](https://books.google.com/books/about/Factory_and_Manager_in_the_USSR.html?id=5ASaAAAAIAAJ)
- Weitzman, M.L. (1980). "The 'Ratchet Principle' and Performance Incentives." *Bell Journal of Economics*, 11(1), 302–308. [Harvard PDF](https://scholar.harvard.edu/files/weitzman/files/rachetprincipleperformanceincentives.pdf)
- Gregory, P.R. & Harrison, M. (2005). "Allocation Under Dictatorship: Research in Stalin's Archives." *Journal of Economic Literature*, 43(3), 721–761. [Warwick PDF](https://wrap.warwick.ac.uk/id/eprint/164/1/WRAP_Harrison_jel05.pdf)
- Nove, A. (1980). "Problems and Prospects of the Soviet Economy." *New Left Review*, I/119. [NLR PDF](https://newleftreview.org/issues/i119/articles/alec-nove-problems-and-prospects-of-the-soviet-economy.pdf)
- Harrison, M. "Forging Success: Soviet Managers and False Accounting, 1943 to 1962." [ResearchGate](https://www.researchgate.net/publication/46454796_Forging_Success_Soviet_Managers_and_False_Accounting_1943_to_1962)

### Wikipedia and reference
- [Shortage economy](https://en.wikipedia.org/wiki/Shortage_economy)
- [Material balance planning](https://en.wikipedia.org/wiki/Material_balance_planning)
- [Shturmovshchina](https://en.wikipedia.org/wiki/Shturmovshchina)
- [Tolkach](https://en.wikipedia.org/wiki/Tolkach)
- [Soviet-type economic planning](https://en.wikipedia.org/wiki/Soviet-type_economic_planning)
- [Rail transport in Russia](https://en.wikipedia.org/wiki/Rail_transport_in_Russia)

### Other web sources
- [GlobalSecurity.org — Soviet Defense Industry Storming](https://www.globalsecurity.org/military/world/russia/industry-storming.htm)
- [P2P Foundation — Material Balance Planning](https://wiki.p2pfoundation.net/Material_Balance_Planning_in_the_Central_Planning_System_of_the_Soviet_Union)
- [Encyclopedia.com — Material Balances](https://www.encyclopedia.com/history/encyclopedias-almanacs-transcripts-and-maps/material-balances)
- [Ratchet Effect — NBER Working Paper 16325](https://www.nber.org/system/files/working_papers/w16325/w16325.pdf)

### Codebase files
- `simulation/src/economy/market.rs` — Market, SingleMarket, Dispatch, make_trades, advance_dispatches
- `simulation/src/souls/goods_company.rs` — recipe_init, recipe_act, recipe_should_produce, company_system, GoodsCompanyState
- `simulation/src/economy/government.rs` — Government (money only)
- `simulation/src/economy/ecostats.rs` — EcoStats, ItemHistories (trade volumes)
- `simulation/src/map_dynamic/dispatch.rs` — Dispatcher (vehicle-to-buyer routing)
- `simulation/src/tests/scenarios/hoarding.rs` — SCENARIO-0082, SCENARIO-0083, SCENARIO-0151
- `simulation/src/tests/scenarios/inflation.rs` — sov-lpj (request_multiplier end-to-end proof)
- `simulation/src/tests/scenarios/validation.rs` — sov-k3w (negative/zero request_multiplier refusal)
- `prototypes/src/types/recipe.rs:52` — `request_multiplier: i32`
- `prototypes/src/validation.rs:65-74` — request_multiplier validation
- `base_mod/companies.lua:40,582` — flour-factory (4), slaughterhouse (3)
- `docs/reference/specifications/production.md` — SPEC-PRODUCTION-003, SPEC-PRODUCTION-009
- `docs/reference/specifications/logistics.md` — SPEC-LOGISTICS-001 through -011
- `docs/reference/specifications/resources.md` — SPEC-RESOURCES-001 through -006
- `docs/reference/specifications/trade.md` — SPEC-TRADE-001 through -008
- `docs/plan/proposals/gosplan.md` — GOSPLAN proposal (process, not economy)
