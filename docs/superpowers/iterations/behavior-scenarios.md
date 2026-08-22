# Behavior Scenarios

## Journey Scenarios

## JOURNEY-0001 — A mill hoards coal and the planner catches it from observable state alone

**Kind:** journey
**Proof seam:** e2e
**Owning stories:** STORY-0105, STORY-0106, STORY-0107, STORY-0093

**Preconditions:**
- A deterministic TestCtx world with one coal mine, one steel mill, one road connecting them, and one truck
- The mill runs a recipe consuming coal and producing steel
- The mill is configured to request more coal than its recipe strictly needs
- No money, wages or foreign trade machinery is required for this journey
- Coal and steel item prototypes are authored with optout_exttrade = true — otherwise economy/market.rs:284-296 credits any unmet buy order to the buyer unconditionally, and does so BEFORE calling find_external, so the mill would receive coal even with no external partner and the journey's "nothing teleports" observable would fail at the first market pass

**Steps:**
1. Tick the world until the mine has produced coal into its export bucket
   → Coal stock at the mine is greater than zero
   → No coal has appeared at the mill — nothing teleports
2. Let the dispatcher assign the truck to carry coal from mine to mill
   → The truck traverses travel -> load -> travel -> unload in that order
   → Coal leaves the mine bucket only at load and appears at the mill only at unload
   → At no tick does coal exist in both places or neither
3. Tick the world while the mill produces steel
   → Steel stock at the mill rises
   → Coal is consumed at the recipe's true rate, not the requested rate
4. Read the mill's production ledger
   → Requested coal quantity is strictly greater than consumed coal quantity
   → The surplus accumulates in the mill's input store rather than vanishing
5. Open the mill's inspection panel as the planner
   → The panel exposes requested-versus-consumed for the mill
   → A hoarding enterprise is distinguishable from an honest one using only state the player can see
   → No hidden debug-only field is required to reach the conclusion

**Final observables:**
- The mill holds more coal than its production consumed, sourced entirely by physical vehicle delivery
- The planner can identify the mill as hoarding from the inspection panel alone
- Running an honest mill through the identical journey produces different requested-versus-consumed figures than the hoarding mill — the signal discriminates, it is not a shared flag or threshold
- The run is deterministic: identical seed and inputs yield identical ledger figures

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/production.md:84-86`
- `spec/logistics.md:24-31`

## Surface Scenarios

## SCENARIO-0001 — Building a road creates no automatic lots

**Kind:** contract
**Proof seam:** integration
**Owning stories:** STORY-0013

**Preconditions:**
- Empty map with no lots
- Lot::generate_along_road disabled per 2026-08-22 decision

**Action:**
- Player draws and builds a road segment through empty land

**Expected observables:**
- Road entity is created
- No Lot entity is created
- Total Lot count in the map is unchanged from before the road was built

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/zoning.md:24-27`

## SCENARIO-0002 — Blueprint placement defers all capability activation

**Kind:** surface
**Proof seam:** integration
**Owning stories:** STORY-0001

**Preconditions:**
- Player has authorised, validated land for a residential building
- Player has sufficient plan allocation (not money)

**Action:**
- Player places a residential building blueprint
- Query the building entity for housing allocation before any phase completes

**Expected observables:**
- A ConstructionProject is created at Planned/Under-construction state
- The building's dwelling slots report zero available capacity
- No Money balance changes
- Housing allocation system finds zero usable dwelling slots at this building
- Building capabilities remain inactive until the final construction phase completes

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/construction.md:1-24`

## SCENARIO-0003 — Construction phase stalls without required material, resumes on delivery

**Kind:** failure-recovery
**Proof seam:** integration
**Owning stories:** STORY-0003

**Preconditions:**
- A ConstructionProject exists in its foundations phase requiring 28 tonnes of concrete
- No concrete has been delivered to the site
- A crane vehicle is already assigned

**Action:**
- Advance simulation ticks with zero concrete delivered
- Deliver 21 of the 28 tonnes of concrete to the site
- Deliver the remaining 7 tonnes to complete the full concrete bill

**Expected observables:**
- Phase work-progress stays at 0
- Project.bottleneck reports no-material
- Phase work-progress begins advancing proportionally to the 21/28 tonnes delivered
- Project.bottleneck still reports no-material for the missing 7 tonnes
- Phase work-progress advances at full rate
- Project.bottleneck clears
- The foundations phase eventually completes once material and machine are both present; partial delivery produced partial, proportional progress rather than an all-or-nothing wait

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/construction.md:37-56`

## SCENARIO-0004 — Construction phase stalls without matching vehicle skill

**Kind:** failure-recovery
**Proof seam:** integration
**Owning stories:** STORY-0003, STORY-0008

**Preconditions:**
- A ConstructionProject's structure phase has its full material bill delivered
- No CRANE-skilled vehicle is assigned to the phase's station slot

**Action:**
- Advance simulation ticks with materials present but no crane assigned
- Assign a crane vehicle and let it travel to and park in the station slot

**Expected observables:**
- Phase work-progress stays at 0
- Project.bottleneck reports no-machine
- Phase work-progress begins advancing once the vehicle is parked
- A phase never completes purely from having materials without a matching machine

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/construction.md:37-56`
- `spec/construction.md:68-74`

## SCENARIO-0005 — Second crane assigned shortens phase completion time

**Kind:** surface
**Proof seam:** unit
**Owning stories:** STORY-0004

**Preconditions:**
- A structure phase has its full material bill delivered
- One crane (skill throughput X) is assigned and parked

**Action:**
- Record remaining ticks-to-completion with one crane
- Assign a second matching crane (skill throughput Y) and park it

**Expected observables:**
- A finite remaining duration D1 is computed as phase_work / X
- Remaining duration recomputes to phase_work / (X + Y)
- Remaining duration with two cranes is strictly less than with one, in proportion to added throughput

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/construction.md:57-66`

## SCENARIO-0006 — Demolition emits sorted rubble, never a money refund

**Kind:** contract
**Proof seam:** integration
**Owning stories:** STORY-0009

**Preconditions:**
- An operating building exists with known material composition
- A demolition office is dispatched to it

**Action:**
- Demolition office assigns explosives and machine-work to the site over several ticks
- Demolition process completes

**Expected observables:**
- Building remains present and operating/inert until the process completes
- No Money is credited to the player treasury during or after
- Building entity is removed
- Typed rubble items (e.g. waste_gravel, waste_steel) appear in the logistics system near the site
- Treasury balance is unchanged by the demolition; rubble items exist and are haulable

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/construction.md:68-74`

## SCENARIO-0007 — Renovation adds capacity without evicting occupants

**Kind:** surface
**Proof seam:** integration
**Owning stories:** STORY-0010

**Preconditions:**
- An operating residential building is fully occupied
- Player starts a renovation project on it

**Action:**
- Renovation project begins and progresses through its phases
- Renovation completes

**Expected observables:**
- All existing occupants remain assigned to the building throughout
- Building continues operating (still habitable) during renovation
- Declared dwelling capacity increases
- Existing occupants are still assigned; new slots are available for allocation
- No occupant was relocated or evicted at any point in the process

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/buildings.md:34-42`

## SCENARIO-0008 — Building condition decays under starved maintenance input and cannot be auto-demolished

**Kind:** failure-recovery
**Proof seam:** integration
**Owning stories:** STORY-0011

**Preconditions:**
- An operating residential building has a declared heat maintenance requirement
- Heat supply to the building is cut

**Action:**
- Advance simulation N ticks with heat supply absent
- Continue until condition reaches zero
- Restore heat supply without repairing

**Expected observables:**
- Building condition value decreases from full toward zero
- Building capacity (dwelling/workplace) is withdrawn — becomes unusable
- Building entity is NOT removed or auto-demolished
- Further decay halts
- Condition value does not increase on its own
- A zero-condition building persists, uninhabitable, until an explicit repair process runs

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/buildings.md:34-49`

## SCENARIO-0009 — Zoning polygon paint alone creates no building

**Kind:** contract
**Proof seam:** e2e
**Owning stories:** STORY-0014

**Preconditions:**
- Empty, unzoned land with room for four separate districts

**Action:**
- Player paints a residential land-use polygon over one area
- Player paints industrial, agricultural, and mixed polygons over the three remaining areas
- Advance simulation many ticks with all four districts left empty

**Expected observables:**
- A zoning district record with category=residential is created
- No building or ConstructionProject entity is created
- Three further district records are created with category=industrial, agricultural, mixed respectively
- No building or ConstructionProject entity is created for any of them
- No building or ConstructionProject appears in any of the four districts at any point
- All four districts remain zoned-but-empty indefinitely — a visible plan backlog, never a spawn

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/zoning.md:28-34`

## SCENARIO-0010 — Placement rejected on land failing the physical siting checklist

**Kind:** failure-recovery
**Proof seam:** integration
**Owning stories:** STORY-0015

**Preconditions:**
- A steeply sloped or already-occupied tile exists on the map
- A flat, unoccupied tile with no road adjacency exists on the map
- A flat, unoccupied, road-adjacent tile outside utility (electricity) reach exists on the map

**Action:**
- Player attempts to place a building blueprint on the invalid (sloped/occupied) tile
- Player attempts to place the blueprint on the flat, unoccupied tile with no network adjacency
- Player attempts to place the blueprint on the flat, unoccupied, road-adjacent tile outside utility reach
- Player attempts to place the same blueprint on a valid, flat, unoccupied, network-adjacent, in-utility-reach tile

**Expected observables:**
- Placement is rejected
- No ConstructionProject is created
- No Lot or building entity is created
- Placement is rejected for failing network-adjacency
- No ConstructionProject is created
- Placement is rejected for failing utility-reach
- No ConstructionProject is created
- Placement succeeds
- A ConstructionProject is created
- Only the fully valid placement produced a construction project; each of the four checklist criteria independently rejected placement when it alone was violated

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/zoning.md:16-23`

## SCENARIO-0011 — Construction office dispatches idle vehicle to nearest stalled site

**Kind:** surface
**Proof seam:** integration
**Owning stories:** STORY-0007

**Preconditions:**
- A construction office has stocked concrete and owns an idle crane
- Two construction sites are stalled: one on no-material, one on no-machine, both within range

**Action:**
- Office evaluates dispatch on its tick

**Expected observables:**
- Office sends a delivery vehicle carrying concrete toward the no-material site
- Office sends the idle crane toward the no-machine site
- Both previously-stalled sites receive the resource they were missing without player intervention

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/construction.md:68-74`

## SCENARIO-0012 — Construction phase stalls on no-worker distinct from no-machine

**Kind:** failure-recovery
**Proof seam:** integration
**Owning stories:** STORY-0003

**Preconditions:**
- A ConstructionProject's utilities phase has its full material bill delivered
- A technician-skilled vehicle is assigned and parked
- The office's WORKERS pool has zero workers available to assign to this phase

**Action:**
- Advance simulation ticks with materials and machine present but zero workers assigned
- Office's WORKERS pool gains enough workers to fill the phase requirement

**Expected observables:**
- Phase work-progress stays at 0
- Project.bottleneck reports no-worker, not no-machine
- Phase work-progress begins advancing
- Project.bottleneck clears
- A phase never completes purely from having materials and a machine without sufficient sourced workers

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/construction.md:72-74`
- `spec/construction.md:114`

## SCENARIO-0013 — Demolition stalls without explosives, resumes on delivery

**Kind:** failure-recovery
**Proof seam:** integration
**Owning stories:** STORY-0009

**Preconditions:**
- A demolition office has dispatched machine-work to a site
- No explosives have been delivered to the site

**Action:**
- Advance simulation ticks with zero explosives delivered
- Deliver the required explosives to the site

**Expected observables:**
- Demolition progress stays at 0
- Building entity remains present, unremoved
- Demolition progress begins advancing on the next tick
- Demolition never completes purely from machine-work without explosives on site

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/construction.md:73`

## SCENARIO-0014 — Road construction progresses through its own phase pipeline and stalls without gravel or asphalt

**Kind:** failure-recovery
**Proof seam:** integration
**Owning stories:** STORY-0006

**Preconditions:**
- A road segment blueprint has been placed and authorised as a RoadConstructionProject
- GROUNDWORKS and ASPHALT_LAYING/ROLLING vehicles are assigned to their respective phase station slots

**Action:**
- Advance simulation ticks through the earthworks phase
- Advance ticks into the sub-base phase with zero gravel delivered
- Deliver gravel, letting sub-base and paving (concrete) phases complete, then advance into the surfacing phase with zero asphalt delivered
- Deliver asphalt to the site

**Expected observables:**
- Earthworks phase completes using only GROUNDWORKS vehicle work, no material bill required
- Sub-base phase work-progress stays at 0
- Project.bottleneck reports no-material
- Surfacing phase work-progress stays at 0
- Project.bottleneck reports no-material for asphalt
- Surfacing phase work-progress resumes and completes
- Markings phase begins
- The road reaches the open state only after passing through earthworks, sub-base, paving, surfacing, and markings in order, stalling exactly where gravel or asphalt was missing

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/construction.md:53`

## SCENARIO-0015 — New game starts with a stocked starter warehouse

**Kind:** surface
**Proof seam:** e2e
**Owning stories:** STORY-0046

**Preconditions:**
- Fresh new-game start, no player actions taken yet

**Action:**
- Load a new game
- Inspect the starter warehouse's stock

**Expected observables:**
- Simulation initializes without error
- Warehouse holds the configured non-zero starting quantities of bootstrap goods
- Starter warehouse stock matches the configured seed quantities before any tick advances

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `docs/egregoria-substrate-audit.md:159-161`

## SCENARIO-0016 — Deadlocked domestic chain is broken by a marked-up customs import

**Kind:** failure-recovery
**Proof seam:** integration
**Owning stories:** STORY-0047, STORY-0050

**Preconditions:**
- A production chain has zero domestic supply of a required input and no path to acquire it internally (simulated deadlock)

**Action:**
- Player places a customs import order for the deficit good
- Advance simulation until a vehicle completes the border-to-inland haul
- Advance the deadlocked chain

**Expected observables:**
- Order is accepted and priced above the domestic administered/shadow price
- Treasury's foreign-currency balance is debited only at the moment of physical clearance, not at order time
- Chain resumes production using the imported goods
- Deadlocked chain produces output again
- Treasury foreign-currency balance decreased by the markup price
- No enterprise beznal or household nal balance changed

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `docs/egregoria-substrate-audit.md:130-135,159-161`

## SCENARIO-0017 — Foreign trade order stalls with no vehicle capacity, no money moves

**Kind:** failure-recovery
**Proof seam:** integration
**Owning stories:** STORY-0048

**Preconditions:**
- A foreign trade order is placed
- No freight vehicle/fleet capacity exists to service the customs house

**Action:**
- Advance the simulation for many ticks with no freight capacity available

**Expected observables:**
- Order remains in the ordered or atCustoms state, never reaches cleared
- No capital/goods quantity changed for either party
- No treasury balance changed
- Order is still visibly queued to the player, not silently dropped

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `docs/egregoria-substrate-audit.md:136-141`

## SCENARIO-0018 — Household cash cannot be spent using enterprise accounting roubles

**Kind:** contract
**Proof seam:** unit
**Owning stories:** STORY-0043

**Preconditions:**
- An enterprise holds a beznal balance sufficient to cover a retail good's price
- A citizen's nal balance is zero

**Action:**
- Attempt to fund the citizen's retail purchase directly from the enterprise's beznal account (bypassing wages)

**Expected observables:**
- Transaction is rejected
- Citizen does not receive the good
- Enterprise beznal balance is unchanged
- No implicit beznal-to-nal conversion occurred

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `docs/egregoria-substrate-audit.md:154-158`

## SCENARIO-0019 — Excess retail demand lengthens the queue instead of raising the price

**Kind:** surface
**Proof seam:** integration
**Owning stories:** STORY-0044

**Preconditions:**
- Administered retail price for a good is set and held fixed
- Household demand for the good exceeds current shelf stock

**Action:**
- Advance simulation with demand continuing to exceed stock

**Expected observables:**
- Retail price for the good is unchanged tick over tick
- Queue length and/or wait time for the good has increased
- Administered price is bit-for-bit identical to before the demand spike

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `docs/egregoria-substrate-audit.md:121-127`
- `spec/trade.md:1-6`

## SCENARIO-0020 — Treasury and simulation stay consistent across a mid-transit save/load

**Kind:** contract
**Proof seam:** process-level
**Owning stories:** STORY-0050

**Preconditions:**
- A foreign trade order has been matched and is in the atCustoms (in-transit) state, not yet cleared

**Action:**
- Save the game
- Reload the save
- Advance simulation until the vehicle clears the border

**Expected observables:**
- Save completes without error
- Order is restored in the atCustoms state
- Treasury is credited/debited exactly once, at clearance
- Treasury balance after clearance equals pre-save balance plus/minus exactly one trade's value
- No double-settlement and no lost settlement occurred

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/trade.md:27-29`

## SCENARIO-0021 — Dollar-only good cannot be bought with a rouble-only balance

**Kind:** contract
**Proof seam:** integration
**Owning stories:** STORY-0053

**Preconditions:**
- Treasury holds a large rouble balance and zero dollars
- Target import good is tagged as a hard-currency (dollar) good

**Action:**
- Attempt to place an import order for the dollar-tagged good

**Expected observables:**
- Order is rejected or blocked pending dollar funds
- Rouble balance is unchanged
- No goods are imported

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/trade.md:18-24`

## SCENARIO-0022 — Household nal balance persists across save and load

**Kind:** contract
**Proof seam:** process-level
**Owning stories:** STORY-0040

**Preconditions:**
- A citizen holds a non-zero nal balance

**Action:**
- Save the game
- Reload the save

**Expected observables:**
- Save completes without error
- Citizen's nal balance is restored to the exact pre-save value
- Citizen nal balance after load equals nal balance before save

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `docs/egregoria-substrate-audit.md:37-38`

## SCENARIO-0023 — Wage payment debits employer beznal and credits worker nal on interval

**Kind:** surface
**Proof seam:** integration
**Owning stories:** STORY-0041

**Preconditions:**
- A human is bound to a workplace via Work.workplace
- Both the workplace's beznal balance and the worker's nal balance are known before the wage interval elapses

**Action:**
- Advance simulation past one wage interval

**Expected observables:**
- Workplace beznal balance decreases by the wage amount
- Worker nal balance increases by the same wage amount
- Worker nal balance increased exactly by the wage amount
- Workplace beznal balance decreased by the same amount
- No other account's balance changed

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `docs/egregoria-substrate-audit.md:17-19,154-158`

## SCENARIO-0024 — Domestic enterprise-to-enterprise trade settles in beznal with a nonzero delta

**Kind:** surface
**Proof seam:** integration
**Owning stories:** STORY-0042

**Preconditions:**
- Two domestic enterprises complete an internal (non-border) trade of goods

**Action:**
- Advance simulation until the internal trade resolves

**Expected observables:**
- Buyer's beznal balance decreases by a nonzero money_delta
- Seller's beznal balance increases by the same nonzero money_delta
- money_delta for the trade is nonzero, not the current Money::ZERO barter clearing
- Neither party's nal balance changed

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `docs/egregoria-substrate-audit.md:29-31`

## SCENARIO-0025 — Building a customs house creates a placeable border-crossing entity

**Kind:** surface
**Proof seam:** app-level
**Owning stories:** STORY-0049

**Preconditions:**
- Player has access to the build menu

**Action:**
- Place a customs house building

**Expected observables:**
- Building entity is created with mode, per-class buffer, bay list, domestic edge, and border edge fields populated
- A customs house entity exists distinct from ordinary production buildings

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/trade.md:9-16`

## SCENARIO-0026 — Foreign trade order cannot resolve with no customs house built

**Kind:** failure-recovery
**Proof seam:** integration
**Owning stories:** STORY-0049

**Preconditions:**
- No customs house has been built on the map
- A foreign trade order is placed

**Action:**
- Advance simulation for many ticks with no customs house built

**Expected observables:**
- Order never reaches cleared
- No goods or money move for either party
- No infinite external partner services the order
- Order remains unfulfilled until a customs house is built

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/trade.md:16-17`

## SCENARIO-0027 — Import order outside its era/bloc window is rejected at customs

**Kind:** failure-recovery
**Proof seam:** integration
**Owning stories:** STORY-0056

**Preconditions:**
- An item's availability window and bloc tag are set
- Current game era or player's bloc alignment falls outside that item's window/tag

**Action:**
- Place an import/export order for the out-of-window or out-of-bloc item

**Expected observables:**
- Order is rejected at customs
- No goods or money moved for the rejected order
- Item remains unavailable until era/bloc conditions are met

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/trade.md:42-44`

## SCENARIO-0028 — Used vehicle export fetches less hard currency than an equivalent new vehicle

**Kind:** contract
**Proof seam:** integration
**Owning stories:** STORY-0057

**Preconditions:**
- Two otherwise-identical vehicles exist, one new (full condition) and one worn (reduced condition)

**Action:**
- Export the new vehicle
- Export the worn vehicle

**Expected observables:**
- Hard currency credited equals the vehicle's full base price
- Hard currency credited is strictly less than the full base price
- Worn vehicle's export payout is strictly less than the new vehicle's export payout for the same prototype

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/trade.md:46-48`

## SCENARIO-0029 — Player-facing view shows a price move and its driver

**Kind:** surface
**Proof seam:** app-level
**Owning stories:** STORY-0045

**Preconditions:**
- A traded good's border price changes due to a modelled driver (e.g. cumulative export volume)

**Action:**
- Trigger the driver (e.g. export enough volume to move the price)
- Open the player-facing price view for that good

**Expected observables:**
- Border price for the good changes
- View shows the new price and names the driver that changed it
- Displayed price matches the current border price
- At least one named driver is shown alongside the price

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/trade.md:30-33`

## SCENARIO-0030 — A* refuses a route through a banned turn at an intersection

**Kind:** contract
**Proof seam:** integration
**Owning stories:** STORY-0058

**Preconditions:**
- A road network with an intersection where a specific turn (e.g. left turn from lane A to lane B) is not permitted

**Action:**
- Request a path whose shortest geometric route would cross the banned turn

**Expected observables:**
- The returned lane-hop chain never traverses the banned turn
- An alternative legal route is returned if one exists
- No segment of the computed route uses the banned turn

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/roads.md:16-21`

## SCENARIO-0031 — Route cost prefers the faster lane absent congestion

**Kind:** surface
**Proof seam:** unit
**Owning stories:** STORY-0058

**Preconditions:**
- Two parallel lanes connecting the same origin and destination with equal length but different speed limits, zero load on both

**Action:**
- Compute route cost for both lanes

**Expected observables:**
- The higher-speed-limit lane has strictly lower cost
- A* selects the higher-speed-limit lane
- Chosen route uses the faster lane

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/pathfinding.md:26-29`

## SCENARIO-0032 — Two identical trip requests each allocate independent fresh paths

**Kind:** contract
**Proof seam:** integration
**Owning stories:** STORY-0058

**Preconditions:**
- Two agents request a path between the same origin/destination pair in the same tick

**Action:**
- Issue both requests

**Expected observables:**
- Both requests are solved independently
- No shared cached route object is returned to both agents
- No general origin-destination path cache exists that both requests hit

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/pathfinding.md:47-48`

## SCENARIO-0033 — Paved road yields higher effective speed than dirt for the same vehicle

**Kind:** surface
**Proof seam:** unit
**Owning stories:** STORY-0059

**Preconditions:**
- A dirt-class road segment and a paved-class road segment of identical length, curvature and terrain

**Action:**
- Compute effective speed for the same vehicle type on both segments

**Expected observables:**
- Effective speed on paved is strictly greater than on dirt
- The delta matches the authored road-class modifier
- Travel time over the paved segment is lower for the same vehicle

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/roads.md:23-27`

## SCENARIO-0034 — Per-lane EMA rises under sustained load and decays once load stops

**Kind:** surface
**Proof seam:** unit
**Owning stories:** STORY-0065

**Preconditions:**
- A single lane with its EMA counter initialized at zero load

**Action:**
- Drive a sustained stream of vehicles through the lane for several in-game minutes
- Stop all vehicle flow through the lane and advance several more in-game minutes

**Expected observables:**
- The EMA value increases monotonically toward a steady state proportional to throughput
- The EMA value decays back toward zero at the same time-constant
- EMA reflects recent load, not an instantaneous or permanently sticky value

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/traffic.md:16-24`

## SCENARIO-0035 — BPR cost multiplier matches the formula at known v/c ratios

**Kind:** contract
**Proof seam:** unit
**Owning stories:** STORY-0066

**Preconditions:**
- A lane with known freeflow cost t0 and known capacity c

**Action:**
- Set v/c = 0 (no load) and compute cost
- Set v/c = 1 (at capacity) and compute cost
- Set v/c = 2 (double capacity) and compute cost

**Expected observables:**
- Cost equals t0 (multiplier is 1.0)
- Cost equals t0 * 1.15, matching t0*(1+0.15*(v/c)^4)
- Cost equals t0 * (1 + 0.15*16) = t0 * 3.4
- Computed multiplier matches the BPR formula exactly at each test point

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/pathfinding.md:26-33`

## SCENARIO-0036 — Damped remembered cost blends across successive updates

**Kind:** contract
**Proof seam:** unit
**Owning stories:** STORY-0067

**Preconditions:**
- A lane with remembered cost R0 and a newly observed cost O that differs sharply from R0

**Action:**
- Apply one damping update
- Apply the same observed cost O repeatedly across several updates

**Expected observables:**
- New remembered cost equals 0.3*O + 0.7*R0, not O itself
- Remembered cost converges toward O asymptotically rather than jumping to it in one step
- Remembered cost never equals the raw observed cost after a single update when they differ

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/traffic.md:36-38`

## SCENARIO-0037 — Two equal-cost parallel corridors do not flap under repeated re-routing

**Kind:** failure-recovery
**Proof seam:** process-level
**Owning stories:** STORY-0067

**Preconditions:**
- Two parallel corridors of equal base cost between the same origin/destination
- A population of agents re-routing periodically with Gawron damping enabled

**Action:**
- Introduce a one-time load perturbation making corridor A briefly cheaper
- Let the simulation run for N further re-route cycles with damping active

**Expected observables:**
- Some agents shift from B to A on the next re-route cycle
- The fraction of agents switching corridors per cycle is non-increasing and trends to zero
- No sustained oscillation (ping-pong) between A and B is observed
- Corridor split stabilizes; switch-fraction time series does not oscillate at a persistent non-zero amplitude

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/traffic.md:36-38`

## SCENARIO-0038 — Gridlocked vehicle persists in the simulation indefinitely

**Kind:** failure-recovery
**Proof seam:** integration
**Owning stories:** STORY-0070

**Preconditions:**
- A vehicle boxed in by a permanent gridlock with no possible movement

**Action:**
- Advance the simulation far past 200 seconds of the vehicle being fully blocked

**Expected observables:**
- The vehicle entity still exists in the world
- The vehicle has transitioned to Panicking state, not been removed
- Vehicle entity count for this vehicle's ID is unchanged from before the block began

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/traffic.md:26-34`

## SCENARIO-0039 — Stalled vehicle with an alternative route re-routes instead of waiting forever

**Kind:** failure-recovery
**Proof seam:** integration
**Owning stories:** STORY-0071

**Preconditions:**
- A vehicle stalled past the stall threshold on a segment with a viable alternative path to its destination

**Action:**
- Advance the simulation past the stall threshold

**Expected observables:**
- The vehicle issues a new path request
- The new route avoids the stalled segment
- Vehicle's active route no longer includes the originally stalled segment

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/traffic.md:26-34`

## SCENARIO-0040 — Stalled vehicle with no alternative registers a planner-visible bottleneck event

**Kind:** failure-recovery
**Proof seam:** app-level
**Owning stories:** STORY-0071

**Preconditions:**
- A vehicle stalled past the stall threshold with no viable alternative route (e.g. single road in and out)

**Action:**
- Advance the simulation past the stall threshold

**Expected observables:**
- No re-route occurs, since none is possible
- A bottleneck/stall event is emitted
- A stall event referencing this vehicle/segment is present in the planner-visible event feed

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/traffic.md:26-34`

## SCENARIO-0041 — Corridor utilisation readout matches the internal EMA value exactly

**Kind:** contract
**Proof seam:** app-level
**Owning stories:** STORY-0068

**Preconditions:**
- A corridor with a known, directly-set EMA load value

**Action:**
- Read the planner-facing corridor utilisation readout for that corridor

**Expected observables:**
- The displayed value is derived from the same EMA counter, with no independent recomputation
- Readout value and internal EMA value are consistent (same source, not two divergent trackers)

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/roads.md:29-33`

## SCENARIO-0042 — Vehicle brakes to avoid rear-ending a slower vehicle ahead

**Kind:** surface
**Proof seam:** integration
**Owning stories:** STORY-0069

**Preconditions:**
- Two vehicles on the same lane, the lead vehicle traveling slower than the trailing vehicle

**Action:**
- Advance the simulation as the trailing vehicle closes the gap

**Expected observables:**
- The trailing vehicle reduces speed as it approaches the lookahead threshold
- No collision (overlap) occurs between the two vehicles
- Trailing vehicle's speed is bounded by the lead vehicle's position and speed at all times

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/traffic.md:16-24`

## SCENARIO-0043 — Heavy vehicle request is excluded from a pedestrian-only lane

**Kind:** contract
**Proof seam:** integration
**Owning stories:** STORY-0060

**Preconditions:**
- A route between origin and destination whose shortest geometric path crosses a pedestrian-only lane
- A path request flagged as a heavy vehicle

**Action:**
- Solve the path request with the vehicle-type mask excluding pedestrian lanes

**Expected observables:**
- The returned route does not use the pedestrian-only lane
- A longer legal route is returned instead, or no route if none exists
- No segment of the computed route is a pedestrian-only lane

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/pathfinding.md:20-24`

## SCENARIO-0044 — Hard modifiers apply the exact car-ban, transit-lane and closed-segment costs

**Kind:** contract
**Proof seam:** unit
**Owning stories:** STORY-0060

**Preconditions:**
- Three lanes of identical base cost: one flagged car-banned, one flagged transit-lane, one flagged closed

**Action:**
- Compute cost for a car request crossing the car-banned lane
- Compute cost for a transit-vehicle request crossing the transit lane
- Compute cost for any request crossing the closed segment

**Expected observables:**
- Cost equals base_cost * 7.5
- Cost equals base_cost * 0.95
- The segment is rejected (infinite cost); A* never selects it
- Each computed multiplier or rejection matches the spec value exactly

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/pathfinding.md:28-31`

## SCENARIO-0045 — Vehicle continues its original route despite rising ambient congestion

**Kind:** contract
**Proof seam:** integration
**Owning stories:** STORY-0058

**Preconditions:**
- A vehicle mid-route on a lane whose EMA/BPR cost rises significantly after the route was computed
- No blockage occurs on the route

**Action:**
- Advance the simulation while congestion increases on the vehicle's route ahead

**Expected observables:**
- The vehicle does not issue a new path request
- The vehicle's route remains identical to the originally computed route
- Vehicle's active route is unchanged despite the congestion increase

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/pathfinding.md:31`

## SCENARIO-0046 — Redrawing a road invalidates routes that reference the changed segment

**Kind:** failure-recovery
**Proof seam:** integration
**Owning stories:** STORY-0058

**Preconditions:**
- A vehicle holds an active route through a segment
- That segment is redrawn by a completed construction/upgrade project mid-trip

**Action:**
- Trigger the road redraw (construction/upgrade completes)

**Expected observables:**
- Any route referencing the changed segment is invalidated
- The vehicle requests a fresh path
- Vehicle's active route no longer references the stale segment id

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/roads.md:21`

## SCENARIO-0047 — A burst of simultaneous path requests is solved across multiple ticks without blocking the frame

**Kind:** contract
**Proof seam:** integration
**Owning stories:** STORY-0061

**Preconditions:**
- More path requests are issued in a single tick than the configured per-tick solver budget

**Action:**
- Issue the burst of requests in one tick
- Advance to the next tick(s)

**Expected observables:**
- Requesting agents' tick does not stall waiting for their own path result
- Only up to the per-tick budget of requests are resolved that tick; the remainder queue
- Queued requests continue resolving until all are solved
- All requests eventually resolve; no single tick's frame time is blocked on solving all of them synchronously

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/pathfinding.md:22-23`

## SCENARIO-0048 — A ninth road segment cannot attach to an already-full intersection

**Kind:** contract
**Proof seam:** integration
**Owning stories:** STORY-0062

**Preconditions:**
- A RoadNode with exactly 8 attached RoadSegments

**Action:**
- Attempt to deliver a construction project attaching a 9th segment to that node

**Expected observables:**
- The attachment is rejected
- The node's segment count remains 8
- RoadNode segment count never exceeds 8

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/roads.md:18`

## SCENARIO-0049 — A road segment does not exist until its construction project completes

**Kind:** contract
**Proof seam:** integration
**Owning stories:** STORY-0063

**Preconditions:**
- A planned road segment with a construction project not yet complete

**Action:**
- Query the lane graph before project completion
- Advance the simulation until the construction project completes

**Expected observables:**
- No segment or lanes exist for the planned road
- The segment now exists in the lane graph with routable lanes
- The lane graph's segment count only increases at project completion, never before

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/roads.md:12`

## SCENARIO-0050 — Upgrading dirt to paved requires materials and labour, not a money payment

**Kind:** contract
**Proof seam:** integration
**Owning stories:** STORY-0063

**Preconditions:**
- A dirt-class road segment
- A treasury with sufficient money but the settlement's asphalt/gravel stockpile empty

**Action:**
- Attempt to trigger the upgrade with money alone
- Supply asphalt/gravel and labour and advance the construction project to completion

**Expected observables:**
- The upgrade does not proceed while asphalt/gravel and labour are unavailable
- The segment's surface class changes to paved only once the construction project completes
- Road surface class changes only after material+labour construction completes, never from a money transaction alone

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/roads.md:27`

## SCENARIO-0051 — A trip entirely inside a large compound routes over its internal connection graph

**Kind:** surface
**Proof seam:** integration
**Owning stories:** STORY-0064

**Preconditions:**
- A large compound with an authored internal connection graph between two internal points not connected by any public road segment

**Action:**
- Request a path between the two internal points

**Expected observables:**
- The route uses the internal connection graph's edges
- Computed route includes internal-compound connection edges rather than failing or routing over external public roads

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/roads.md:25`

## SCENARIO-0052 — Stall response waits before attempting re-route, and re-routes before registering a stall event

**Kind:** contract
**Proof seam:** integration
**Owning stories:** STORY-0071

**Preconditions:**
- A vehicle that becomes blocked, with a viable alternative route available only after the stall threshold

**Action:**
- Observe the vehicle immediately after becoming blocked, before the stall threshold
- Advance to the stall threshold

**Expected observables:**
- The vehicle waits in place
- No re-route request is issued
- No stall event is emitted
- The vehicle issues a re-route request before any stall event is emitted
- Observed event order is wait, then re-route attempt, then (only if re-route fails) stall event

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/traffic.md:30-32`

## SCENARIO-0053 — Heavy truck loses proportionally more speed on dirt than a light vehicle

**Kind:** surface
**Proof seam:** unit
**Owning stories:** STORY-0059

**Preconditions:**
- A dirt-class road segment
- A heavy truck and a light vehicle with equal paved-road top speed

**Action:**
- Compute effective speed for both vehicle classes on the dirt segment

**Expected observables:**
- The heavy truck's speed-reduction factor is strictly larger than the light vehicle's
- Heavy truck's effective-speed-to-top-speed ratio on dirt is lower than the light vehicle's ratio

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/pathfinding.md:28`

## SCENARIO-0054 — Road prefab lane template names each lane's type from the fixed taxonomy

**Kind:** contract
**Proof seam:** unit
**Owning stories:** STORY-0059

**Preconditions:**
- A road-type prefab definition with two lanes

**Action:**
- Read the prefab's lane template

**Expected observables:**
- Each lane's type is one of Vehicle, Pedestrian, Parking, PublicTransport, Cargo, with a defined speed limit and direction
- Instantiated live lanes match the prefab's declared type, speed limit and direction exactly

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/roads.md:20`

## SCENARIO-0055 — Reserved braking-distance space matches half-v-squared-over-a plus half-length at known speed and deceleration

**Kind:** contract
**Proof seam:** unit
**Owning stories:** STORY-0069

**Preconditions:**
- A vehicle of known half-length L, speed v, and braking deceleration a

**Action:**
- Compute the reserved forward space using the target formula

**Expected observables:**
- Reserved distance equals (v*v)/(2*a) + L
- Computed reservation matches the formula exactly at multiple (v, a, L) test points

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/traffic.md:22`

## SCENARIO-0056 — Save/load roundtrip preserves citizen identity

**Kind:** surface
**Proof seam:** integration
**Owning stories:** STORY-0072

**Preconditions:**
- Running simulation with N citizens having distinct PersonalInfo values

**Action:**
- Serialize the simulation to a save
- Deserialize the save into a fresh simulation
- Compare each loaded HumanEnt's PersonalInfo against its pre-save counterpart

**Expected observables:**
- Save completes without error
- Human count matches pre-save count
- Every field is byte-identical
- entity-to-soul mapping is unchanged
- No citizen's identity fields differ from before the save

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `docs/egregoria-substrate-audit.md:10-16`

## SCENARIO-0057 — Determinism replay detects accidental citizen-state divergence

**Kind:** contract
**Proof seam:** integration
**Owning stories:** STORY-0072

**Preconditions:**
- TestCtx harness available, per simulation/src/tests/mod.rs

**Action:**
- Run a recorded session twice independently through TestCtx::tick
- Hash-compare per-tick state between the two runs

**Expected observables:**
- Both runs round-trip through bincode each tick
- Hashes match at every tick, including citizen fields
- No divergence tick is found for citizen-related state

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `docs/egregoria-substrate-audit.md:10-16`

## SCENARIO-0058 — Workplace binding survives a full workday cycle without silent reassignment

**Kind:** surface
**Proof seam:** integration
**Owning stories:** STORY-0076

**Preconditions:**
- A citizen with an assigned Work.workplace
- No reassignment event triggered

**Action:**
- Advance the simulation through a full day/night cycle
- Read Work.workplace after the cycle

**Expected observables:**
- Citizen commutes to and from the same workplace
- Value is unchanged from the start of the cycle
- Work.workplace equals its pre-cycle value

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/citizens.md:37-46`

## SCENARIO-0059 — Buy order match does not move goods before physical arrival

**Kind:** failure-recovery
**Proof seam:** integration
**Owning stories:** STORY-0081

**Preconditions:**
- A citizen's buy order has been matched to a seller
- Citizen has not yet reached the shop

**Action:**
- Advance ticks up to but not including the arrival tick
- Advance to the arrival tick

**Expected observables:**
- Shop stock is unchanged
- Citizen/household inventory is unchanged
- Shop stock debits by the traded amount
- Citizen/household inventory credits by the traded amount, in the same tick
- No goods movement occurred before physical arrival; movement is atomic on arrival

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/citizens.md:63-66`

## SCENARIO-0060 — Household pantry is shared, not per-member

**Kind:** surface
**Proof seam:** integration
**Owning stories:** STORY-0084

**Preconditions:**
- A household with two or more members and one shared pantry

**Action:**
- Each member consumes their per-step food draw
- Inspect any individual member's inventory

**Expected observables:**
- The single household pantry level drops by the combined draw of all members
- No member holds a private food stock separate from the household pantry
- Pantry level reflects combined household consumption, never a per-citizen stock

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/households.md:24-29`

## SCENARIO-0061 — Housing queue grows visibly when no vacancy exists

**Kind:** surface
**Proof seam:** app-level
**Owning stories:** STORY-0085

**Preconditions:**
- No vacant flat available in the simulation

**Action:**
- A new couple forms (queue-entry source: new couples)
- Read the player-visible queue length

**Expected observables:**
- A new household is created with dwellingRef = 0
- Queue length increments by one
- The new household appears in the queue and is not silently dropped or auto-housed

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/households.md:31-40`

## SCENARIO-0062 — Eviction returns a household to the queue instead of deleting it

**Kind:** failure-recovery
**Proof seam:** integration
**Owning stories:** STORY-0085

**Preconditions:**
- A housed household occupies a flat that is condemned/destroyed

**Action:**
- Trigger the building-loss event
- Check the simulation's household count and the queue

**Expected observables:**
- Household dwellingRef resets to 0
- Household count is unchanged (no deletion)
- The household appears in the housing queue
- The displaced household is present in the queue with all members intact

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/households.md:36-40`

## SCENARIO-0063 — Job assignment respects minimum education tier over raw proximity

**Kind:** contract
**Proof seam:** integration
**Owning stories:** STORY-0077

**Preconditions:**
- Two open vacancies: a near tier-0 slot and a far tier-1 slot
- One candidate citizen who meets only the tier-1 minimum education requirement

**Action:**
- Run the labour allocation pass for the candidate
- Check the far tier-1 slot

**Expected observables:**
- Candidate is not assigned to the near tier-0 slot despite shorter distance
- Candidate is assigned there if it also satisfies commute feasibility
- Assignment outcome is governed by education-tier eligibility, not distance alone

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/citizens.md:37-46`

## SCENARIO-0064 — Sick citizen's economic activity freezes until hospital capacity resolves it

**Kind:** surface
**Proof seam:** integration
**Owning stories:** STORY-0073

**Preconditions:**
- A citizen has sustained a low-health streak and rolled sick
- A hospital-transport need has been posted

**Action:**
- Advance ticks while hospital capacity is unavailable
- Free hospital capacity and let the need resolve

**Expected observables:**
- Citizen does not commute to work
- Citizen does not execute shopping trips
- Sickness clears
- Work/shopping behaviour resumes on the next eligible decision tick
- No economic activity is recorded for the citizen during the sick window; activity resumes only after resolution

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/citizens.md:48-52`

## SCENARIO-0065 — Citizen death frees a household slot without dissolving the household

**Kind:** surface
**Proof seam:** integration
**Owning stories:** STORY-0074

**Preconditions:**
- A household with a member near the upper end of the death age window and poor health

**Action:**
- Advance ticks until the death condition triggers
- Inspect the household afterward

**Expected observables:**
- Member is removed from the simulation
- Household entity still exists
- Remaining members are unaffected
- The freed member slot is available for a future member
- Household persists post-death with one fewer member and no cascading deletion

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/households.md:72-76`

## SCENARIO-0066 — Unmet need surfaces as a visible waiting or going-without state

**Kind:** surface
**Proof seam:** app-level
**Owning stories:** STORY-0083

**Preconditions:**
- A citizen's buy order cannot currently be matched (no seller/stock available)

**Action:**
- Advance ticks while the order remains unmatched
- Continue advancing until urgency crosses the going-without threshold with no substitute available

**Expected observables:**
- The citizen is shown in a queued/waiting state to the player
- Felt urgency rises using the elapsed-since-satisfied clock
- Citizen enters an observable going-without state, distinct from queued-waiting
- At every tick the citizen's need state (satisfied / waiting / going-without) is player-observable, never a silent unmatched order

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/needs.md:1-11`

## SCENARIO-0067 — Full-population tick time stays within budget at the profiled ceiling

**Kind:** surface
**Proof seam:** process-level
**Owning stories:** STORY-0087

**Preconditions:**
- Simulation seeded with the profiled ceiling population

**Action:**
- Run the simulation for a representative number of ticks under profiling
- Compare measured duration against the stated frame budget

**Expected observables:**
- Per-tick human-update pass duration is measured
- Duration stays under budget for the sampled ticks
- No tick in the sample exceeds the documented budget for the human-update pass

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `docs/egregoria-substrate-audit.md:119-135`

## SCENARIO-0068 — Childcare access raises the observed birth rate

**Kind:** surface
**Proof seam:** integration
**Owning stories:** STORY-0075

**Preconditions:**
- A household with a present couple, both adults, and a free member slot
- No childcare access initially

**Action:**
- Advance many steps and sample the birth roll outcome distribution without childcare access
- Grant the household childcare access and repeat the sampling

**Expected observables:**
- Observed birth frequency is consistent with a 1/12 per-step chance
- Observed birth frequency rises to be consistent with a 1/8 per-step chance
- Birth frequency measurably increases when childcare access is present

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/households.md:54`

## SCENARIO-0069 — Adult child leaving home enters the housing queue

**Kind:** surface
**Proof seam:** integration
**Owning stories:** STORY-0085

**Preconditions:**
- A household with an adult child member eligible to leave

**Action:**
- Trigger the adult-child-leaving-home event
- Read the player-visible queue length

**Expected observables:**
- A new household is created for the departing member with dwellingRef = 0
- Queue length increments by one
- The fission-created household appears in the queue

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/households.md:40`

## SCENARIO-0070 — Plan-recruited immigrant household enters the housing queue

**Kind:** surface
**Proof seam:** integration
**Owning stories:** STORY-0085

**Preconditions:**
- The plan recruits a new immigrant household (not demand-gated auto-fabrication at the map edge)

**Action:**
- Recruit an immigrant household via the plan
- Read the player-visible queue length

**Expected observables:**
- A new household is created with dwellingRef = 0
- Queue length increments by one
- The immigrant household appears in the queue, sourced only from an explicit plan decision, never automatic map-edge fabrication

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/households.md:41`

## SCENARIO-0071 — Student seat allocation is capacity-limited and required for credentials

**Kind:** contract
**Proof seam:** integration
**Owning stories:** STORY-0078

**Preconditions:**
- A school at its authored throughput ceiling
- A student with no assigned seat

**Action:**
- Attempt to allocate the student a seat at the full school
- Advance time with the student unseated
- Free a seat and allocate the student

**Expected observables:**
- Allocation is refused; student remains unseated
- The student does not gain that tier's education credential
- Student occupies the seat, commute/capacity accounting updates like a job assignment
- Credential is granted only after attended capacity was actually occupied, never by ambient coverage

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/citizens.md:51-52`

## SCENARIO-0072 — Labour-pool dispatch staffs a construction surge without moving fixed workers

**Kind:** surface
**Proof seam:** integration
**Owning stories:** STORY-0079

**Preconditions:**
- A construction site posts a short-lived labour demand
- A set of citizens hold fixed Work.workplace bindings

**Action:**
- Dispatch unassigned/temporary workers from the labour-pool office to the site
- Read Work.workplace for every citizen holding a fixed binding before dispatch

**Expected observables:**
- The site's labour demand is filled
- All fixed Work.workplace values are unchanged
- Surge demand is staffed with zero fixed-binding citizens reassigned

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/citizens.md:47`

## SCENARIO-0073 — Commute departure times follow a time-of-day curve, not a uniform draw

**Kind:** surface
**Proof seam:** integration
**Owning stories:** STORY-0077

**Preconditions:**
- A population of citizens with assigned workplaces over multiple simulated days

**Action:**
- Sample commute departure times across many citizens and days

**Expected observables:**
- Departure times cluster around the authored time-of-day curve's peaks rather than being uniformly distributed across the day
- Departure-time distribution matches the time-of-day probability curve

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/citizens.md:65`

## SCENARIO-0074 — Sustained low wellbeing raises crime contribution and lowers attendance probability

**Kind:** surface
**Proof seam:** integration
**Owning stories:** STORY-0077

**Preconditions:**
- A citizen with sustained low wellbeing

**Action:**
- Advance ticks while wellbeing stays low
- Raise the citizen's wellbeing and advance further

**Expected observables:**
- The citizen's work-attendance probability is measurably lower than a high-wellbeing citizen's
- The citizen's contribution to the local crime rate is measurably higher
- Attendance probability rises
- Crime-rate contribution falls
- Attendance probability and crime contribution both track wellbeing

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/needs.md:65`

## SCENARIO-0075 — Work efficiency scales continuously with health

**Kind:** surface
**Proof seam:** integration
**Owning stories:** STORY-0073

**Preconditions:**
- A citizen with a fixed workplace slot

**Action:**
- Vary the citizen's health across the 0-100 range while not sick

**Expected observables:**
- Measured work efficiency moves continuously within the 10-100% range, tracking health rather than jumping only at the sick/not-sick boundary
- Efficiency curve is continuous in health, not a step function

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/citizens.md:61`

## SCENARIO-0076 — Citizen graduates lifecycle stages at age thresholds

**Kind:** surface
**Proof seam:** integration
**Owning stories:** STORY-0074

**Preconditions:**
- A citizen aging from childhood toward pension age

**Action:**
- Advance the citizen's age past the child->student threshold
- Advance past the student->worker threshold
- Advance past the worker->pensioner threshold

**Expected observables:**
- Citizen becomes eligible for a student seat and ineligible for a job vacancy
- Citizen becomes eligible for job vacancies and loses student-seat eligibility
- Citizen loses job-vacancy eligibility
- Life-stage-gated eligibility changes at each named age threshold rather than staying fixed

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/citizens.md:70`

## SCENARIO-0077 — Wants satisfaction is spatially quality-weighted

**Kind:** surface
**Proof seam:** integration
**Owning stories:** STORY-0082

**Preconditions:**
- Two otherwise-identical want-serving buildings (e.g. two parks/culture venues), one in a polluted area and one near nature/water

**Action:**
- Have citizens visit the polluted-area building
- Have equivalent citizens visit the nature/water-adjacent building

**Expected observables:**
- The relevant want field's satisfaction gain is measurably lower
- The relevant want field's satisfaction gain is measurably higher
- Satisfaction gain differs by the building's pollution/nature/water context

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/needs.md:30-36`

## SCENARIO-0078 — Fixing a mobility gap lowers car aspiration pressure below its demand threshold

**Kind:** contract
**Proof seam:** integration
**Owning stories:** STORY-0082

**Preconditions:**
- A citizen with car-aspiration pressure already above the demand threshold, driven mainly by a large mobility gap

**Action:**
- Improve transit quality (e.g. new tram line) reducing the citizen's mobility gap
- Continue advancing aspiration ticks

**Expected observables:**
- Desire_car pressure recomputes lower
- If pressure drops back below threshold, the car-buying demand signal is withdrawn; while pressure stayed above threshold it had produced an active demand signal (e.g. a $TYPE_CAR_DEALER visit/queue entry)
- Car-desire pressure and its resulting demand signal move with transit quality, not by scripted fiat

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/needs.md:39-47`

## SCENARIO-0079 — Rest satisfaction improves with a shorter commute

**Kind:** surface
**Proof seam:** integration
**Owning stories:** STORY-0082

**Preconditions:**
- Two otherwise-identical citizens whose schedules allow rest, one with a long commute and one with a short commute

**Action:**
- Advance both citizens through equivalent schedule/rest windows

**Expected observables:**
- The short-commute citizen's rest satisfaction recovers faster or higher than the long-commute citizen's
- Rest satisfaction is coupled to commute length, not a flat schedule-only refill

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/needs.md:28`

## SCENARIO-0080 — Dwelling quality and default heat/electricity requirement feed housing need and reject silent opt-out

**Kind:** contract
**Proof seam:** integration
**Owning stories:** STORY-0086

**Preconditions:**
- Two dwelling prefabs with different authored qualityOfLiving values
- A residential prefab with no heating/electricity declaration

**Action:**
- House equivalent households in each dwelling prefab
- Inspect the residential prefab's service requirements

**Expected observables:**
- housingQuality need satisfaction differs, tracking each prefab's authored qualityOfLiving
- Heat and electricity are required by default; no opt-out flag is present
- Housing quality varies by prefab and residential buildings never silently skip heat/electricity

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/households.md:48`

## SCENARIO-0081 — Refinery-style recipe consumes one input and yields two co-products

**Kind:** surface
**Proof seam:** unit
**Owning stories:** STORY-0093

**Preconditions:**
- A recipe is defined with 1 input item and 2 output items, plus a nonzero duration

**Action:**
- Run the recipe to completion once with input fully supplied

**Expected observables:**
- Both output items are added to the building's output storage in their declared quantities
- The input item is consumed in its declared quantity
- Both co-products exist in storage after one completed cycle
- Consumed input quantity matches the recipe's declared amount exactly

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/production.md:39-71`

## SCENARIO-0082 — Mine produces raw ore with zero declared inputs

**Kind:** surface
**Proof seam:** unit
**Owning stories:** STORY-0094

**Preconditions:**
- A recipe is defined with an empty inputs list and one output

**Action:**
- Run the recipe with full labour and power available

**Expected observables:**
- Production proceeds without any input-availability check ever being evaluated
- Output is added to storage at the declared rate
- Output quantity matches base_rate with no input items consumed

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/production.md:60-64`

## SCENARIO-0083 — Full output warehouse halts a producing factory, then resumes when space clears

**Kind:** failure-recovery
**Proof seam:** integration
**Owning stories:** STORY-0095

**Preconditions:**
- A factory is actively producing
- Its output storage bucket is filled to capacity

**Action:**
- Advance the sim while output storage stays full
- Clear the output storage (simulate freight pickup)

**Expected observables:**
- Production rate drops to 0
- No further output is added past the storage cap
- Production resumes automatically on a subsequent tick without manual intervention
- Output storage never exceeds capacity
- Production resumes once space is available

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/production.md:73-87`

## SCENARIO-0084 — Linear staffing factor understates the intended curve's penalty at partial staffing

**Kind:** contract
**Proof seam:** unit
**Owning stories:** STORY-0098

**Preconditions:**
- staffFrac = 0.5, worker efficiency e = 1.0

**Action:**
- Compute today's linear factor (staffFrac) and the target curve factor (2e − 200e/(staffFrac+100))

**Expected observables:**
- The linear result (0.5) is strictly greater than the curve result
- The curve result is the value the acceptance criteria require after the fix
- Linear model output ≠ curve model output at staffFrac=0.5, proving today's implementation does not yet match the spec

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/production.md:28-38`

## SCENARIO-0085 — Partial power availability throttles output instead of an all-or-nothing blackout

**Kind:** failure-recovery
**Proof seam:** integration
**Owning stories:** STORY-0099

**Preconditions:**
- A factory is fully staffed and fully supplied on inputs
- Its power network delivers 50% of required electricity, not zero

**Action:**
- Advance the sim one production cycle under 50% power availability

**Expected observables:**
- Output rate is reduced proportionally toward 50%, not left at full rate and not dropped to zero
- The factory is not marked as blacked out
- Realized output is strictly between 0 and full rate under partial power — proving the gate is no longer binary

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/production.md:28-38`

## SCENARIO-0086 — Scarcest of two under-supplied inputs sets the throttled rate

**Kind:** contract
**Proof seam:** unit
**Owning stories:** STORY-0100

**Preconditions:**
- A recipe has two inputs; input A is at 80% of required stock, input B is at 30%

**Action:**
- Compute f_inputs = min(available/required) across both inputs

**Expected observables:**
- f_inputs equals 0.3 (input B's fraction), not 0.8, not an average of the two
- f_inputs == min(0.8, 0.3) == 0.3

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/production.md:28-38`

## SCENARIO-0087 — Two independently-deficient factors compound multiplicatively, not by minimum

**Kind:** contract
**Proof seam:** unit
**Owning stories:** STORY-0101

**Preconditions:**
- f_labour = 0.5 (understaffed)
- f_power = 1.0 (full power)
- f_inputs = 0.3 (scarce input)
- all other factors = 1.0

**Action:**
- Compute output_rate = base_rate × f_labour × f_power × f_inputs × f_machinery × f_water_quality × f_output_space

**Expected observables:**
- output_rate == base_rate × 0.5 × 0.3 == base_rate × 0.15
- output_rate is NOT equal to base_rate × min(0.5, 1.0, 0.3) == base_rate × 0.3
- Composed rate equals the product of all factors (0.15×base_rate), distinguishing multiplicative composition from a min()-only composition

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/production.md:14-27`

## SCENARIO-0088 — Player inspects a stalled factory and sees which input is missing

**Kind:** surface
**Proof seam:** app-level
**Owning stories:** STORY-0102

**Preconditions:**
- A factory is stalled because its steel input reached zero while labour and power remain fully available

**Action:**
- Player selects the stalled building in the UI

**Expected observables:**
- A panel or overlay names the bottleneck as the specific missing input (steel), not a generic 'stopped' or 'idle' state
- The bottleneck reason shown to the player matches the actual limiting factor (NoResources(steel))

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/production.md:66-87`

## SCENARIO-0089 — An enterprise with an inflated request accumulates surplus stock a matched honest enterprise does not

**Kind:** surface
**Proof seam:** integration
**Owning stories:** STORY-0105, STORY-0106

**Preconditions:**
- Two identical buildings of the same recipe type exist
- Building A's declared request quantity for its input equals the recipe's true per-cycle consumption (honest)
- Building B's declared request quantity for the same input is set to 2x the recipe's true per-cycle consumption (inflated)
- The market has enough supply to fill both requests each cycle

**Action:**
- Advance the sim through several production cycles

**Expected observables:**
- Building A's input stock oscillates near zero between deliveries (consumes what it requests)
- Building B's input stock grows cycle over cycle, since it receives more than the recipe consumes
- Building B holds a strictly larger surplus of the input than Building A after N cycles, proving the requested/consumed gap is now mechanically real

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/production.md:1-9`

## SCENARIO-0090 — Player detects a hoarding enterprise from its inspection panel alone

**Kind:** surface
**Proof seam:** e2e
**Owning stories:** STORY-0107

**Preconditions:**
- Building B (from the hoarding scenario above) has accumulated visible surplus stock over several cycles

**Action:**
- Player opens Building B's inspection panel
- Player compares Building B's panel to Building A's (honest) panel

**Expected observables:**
- The panel shows both the requested/received input quantity and the recipe's true consumption for at least one input
- The two numbers visibly differ
- Building A's requested and consumed figures match; Building B's do not
- The player can, from UI alone with no source access, identify which of the two buildings is hoarding

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/production.md:1-9`

## SCENARIO-0091 — Steel and fuel resources declare incompatible transport classes

**Kind:** contract
**Proof seam:** unit
**Owning stories:** STORY-0108

**Preconditions:**
- Steel item prototype has transportClass = OPEN (flatbed)
- Fuel item prototype has transportClass = OIL (tanker)

**Action:**
- Query whether steel and fuel share a compatible transportClass
- Attempt to load steel onto a tanker vehicle instance (transportClass = OIL)

**Expected observables:**
- The compatibility check returns false — no shared transport class
- The load is rejected
- Steel cannot be assigned to OIL-class transport; the compatibility contract is enforced, not merely documented

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/resources.md:48-90`

## SCENARIO-0092 — Meat left outside cold storage decays past its shelf life

**Kind:** failure-recovery
**Proof seam:** integration
**Owning stories:** STORY-0110

**Preconditions:**
- A meat item prototype declares shelfLife = N ticks and storageClass = cooled as its compatible class
- A quantity of meat is stored in an uncooled (open) storage bucket

**Action:**
- Advance the sim past N ticks with meat continuously in uncooled storage
- Repeat with meat stored in a cooled bucket for the same duration

**Expected observables:**
- The stored meat quantity decreases or is zeroed once shelfLife elapses
- The stored meat quantity is unaffected (or decays at a slower/no rate)
- Uncooled meat quantity after N+1 ticks is strictly less than cooled meat quantity under identical elapsed time

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/resources.md:48-78`
- `spec/resources.md:116-126`

## SCENARIO-0093 — A combustion recipe emits ash that fills its own output bucket and can halt the recipe

**Kind:** surface
**Proof seam:** integration
**Owning stories:** STORY-0113, STORY-0114

**Preconditions:**
- A recipe declares a primary output plus a waste_ash byproduct
- The building's ash output bucket has finite capacity and is nearly full

**Action:**
- Run the recipe to completion enough times to fill the ash bucket
- Attempt one more production cycle once the ash bucket is full

**Expected observables:**
- Ash accumulates in a dedicated output-space bucket, consuming capacity like any other output
- The recipe halts (output-space backpressure applies to the byproduct exactly as it does to the primary output)
- Production is blocked by a full byproduct bucket even though the primary output has free space, proving byproducts are first-class outputs subject to the same physical rules

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/production.md:90-95`

## SCENARIO-0094 — Cooled-class storage bucket rejects an incompatible open-class resource

**Kind:** contract
**Proof seam:** unit
**Owning stories:** STORY-0108

**Preconditions:**
- A storage bucket declares storageClass = cooled
- An item prototype (e.g. steel) declares storageClass = open

**Action:**
- Attempt to place steel into the cooled-class bucket
- Place a cooled-class resource (e.g. meat) into the same bucket

**Expected observables:**
- The placement is rejected — the bucket accepts only cooled-class resources
- The placement succeeds
- Steel is never accepted by the cooled bucket while a cooled-class resource is, proving storage-class compatibility is enforced independently of transport-class compatibility

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/resources.md:120`

## SCENARIO-0095 — Electricity is never returned by a query for vehicle-transportable resources

**Kind:** contract
**Proof seam:** integration
**Owning stories:** STORY-0108

**Preconditions:**
- Electricity item prototype declares no vehicle-compatible transportClass (network-only)

**Action:**
- Query the logistics vehicle scheduler for resources eligible to be assigned to a truck/rail transport instance
- Attempt to directly assign electricity to a vehicle transport instance

**Expected observables:**
- Electricity does not appear in the eligible set at all
- The assignment is rejected
- Electricity is categorically absent from vehicle-transportable resources, not merely incompatible with one specific transport class

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/resources.md:125`

## SCENARIO-0096 — Sub-threshold water quality blocks the recipe outright despite full staffing and power

**Kind:** failure-recovery
**Proof seam:** unit
**Owning stories:** STORY-0103

**Preconditions:**
- A recipe declares waterQualityMin = 0.6
- f_labour = 1.0 and f_power = 1.0
- Supplied water quality = 0.4

**Action:**
- Compute f_water_quality and the composed output_rate
- Raise supplied water quality to 0.6 and recompute

**Expected observables:**
- f_water_quality == 0
- output_rate == 0 regardless of f_labour and f_power both being 1.0
- f_water_quality == 1
- output_rate is no longer forced to 0 by the water-quality factor
- The recipe is blocked below the threshold and unblocked at or above it, proving water quality is a binary gate distinct from the continuous factors

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/production.md:82`

## SCENARIO-0097 — Bottleneck reason updates immediately when an input stock is depleted mid-cycle

**Kind:** failure-recovery
**Proof seam:** integration
**Owning stories:** STORY-0104

**Preconditions:**
- A factory is producing at full rate with all factors satisfied
- Its input stock for one item drops to zero mid-tick, before the next fixed production pass boundary

**Action:**
- Advance the sim by a fraction of the production-frequency period after the stock change

**Expected observables:**
- The factory's bottleneck field updates to name the depleted input before the next fixed production pass boundary is reached
- Bottleneck reason reflects the depleted input before the next scheduled production pass, proving recomputation is change-triggered, not only tick-scheduled

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/production.md:131`

## SCENARIO-0098 — Recycling waste_steel at its confirmed 0.98 yield recovers 98 of 100 units consumed

**Kind:** contract
**Proof seam:** unit
**Owning stories:** STORY-0115

**Preconditions:**
- A recycling recipe declares waste input class = waste_steel with recovery yield = 0.98

**Action:**
- Run the recipe consuming exactly 100 units of waste_steel

**Expected observables:**
- Recovered output quantity equals 98 units exactly, not 100
- Recovered output == input_quantity × yield (100 × 0.98 == 98)

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/production.md:63`

## SCENARIO-0099 — A professor shortfall throttles output multiplicatively alongside a simultaneous labour shortfall

**Kind:** contract
**Proof seam:** unit
**Owning stories:** STORY-0117

**Preconditions:**
- A recipe declares workersNeeded and professorsNeeded
- workers present / workersNeeded staffing fraction yields f_labour = 0.8
- professors present / professorsNeeded yields f_professors = 0.5
- f_power = f_inputs = 1.0

**Action:**
- Compute the composed output_rate including f_professors as an independent multiplicative factor

**Expected observables:**
- output_rate == base_rate × f_labour × f_professors == base_rate × 0.8 × 0.5 == base_rate × 0.4
- output_rate is NOT equal to base_rate × min(f_labour, f_professors) == base_rate × 0.5
- Composed rate equals the product of the labour and professor factors (0.4×base_rate), proving professorsNeeded throttles independently and multiplicatively rather than by minimum

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/production.md:96-108`

## SCENARIO-0100 — Child cannot enrol when all schools are at capacity

**Kind:** surface
**Proof seam:** integration
**Owning stories:** STORY-0121

**Preconditions:**
- One school exists with studentCapacity = 2 and 2 citizens already enrolled
- A third school-age citizen with no work or education binding exists within reach

**Action:**
- Simulate ticks while the third citizen's desire scoring runs

**Expected observables:**
- The citizen's enrolment desire scores non-zero utility
- No enrolment binding is created because the school is at capacity
- The third citizen remains unenrolled
- school.enrolled.len() == 2 (unchanged)
- No panic, error, or stalled tick

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/education.md:16-23`

## SCENARIO-0101 — Serviceable seats shrink below raw capacity when the school underproduces

**Kind:** contract
**Proof seam:** integration
**Owning stories:** STORY-0121

**Preconditions:**
- A school with studentCapacity = 10 (StudentCount * 5/4), enrolled.len() = 4
- The school is running at 50% production rate due to partial staffing

**Action:**
- Attempt to enrol a 5th citizen

**Expected observables:**
- Enrolment is refused because serviceable seats (throttled by production rate) are below capacity, despite enrolled.len() < studentCapacity
- enrolled.len() stays at 4
- Citizen remains unenrolled despite raw capacity headroom

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/education.md:20`

## SCENARIO-0102 — School operating rate is proportional to its two-tier staff composition, not just staff presence

**Kind:** failure-recovery
**Proof seam:** integration
**Owning stories:** STORY-0121

**Preconditions:**
- A school has its full worker complement (10) staffed but only half its profesor complement (7 of 15)

**Action:**
- Simulate ticks under this partial profesor staffing

**Expected observables:**
- Seat-time accrual rate is reduced proportionally to the profesor shortfall rather than accruing at full rate or dropping to zero
- Seat-time accrual rate at partial staffing is strictly between zero and the full-staffing rate

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/education.md:31`

## SCENARIO-0103 — Enrolled citizen cannot simultaneously hold a job

**Kind:** contract
**Proof seam:** integration
**Owning stories:** STORY-0121

**Preconditions:**
- A citizen is enrolled at a school (holds a Student/education binding)

**Action:**
- Run the work-desire scoring/assignment pass for that citizen

**Expected observables:**
- No Work.workplace binding is created while the education binding is active
- Citizen has exactly one of {education binding, work binding} active, never both

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/education.md:16-23`

## SCENARIO-0104 — Seat-time only accrues while the school is staffed and operating

**Kind:** failure-recovery
**Proof seam:** integration
**Owning stories:** STORY-0122

**Preconditions:**
- A citizen is enrolled at a school
- The school's staff count drops to zero (e.g. all workers/profesors reassigned or absent)

**Action:**
- Simulate N ticks with zero staff present
- Restore staff to the school and simulate further ticks

**Expected observables:**
- seatTimeMonths does not increment during this period
- seatTimeMonths resumes incrementing
- Total seatTimeMonths equals only the ticks during which the school was staffed

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/education.md:37-40`

## SCENARIO-0105 — University enrolment is capped below school-tier enrolment

**Kind:** contract
**Proof seam:** integration
**Owning stories:** STORY-0122

**Preconditions:**
- A university with capacity for 3 simultaneous students
- 4 school-graduated citizens seeking university enrolment

**Action:**
- Run enrolment assignment for all 4 candidates

**Expected observables:**
- Exactly 3 are enrolled
- The 4th remains unenrolled and is not silently dropped or errored
- university.enrolled.len() <= 3
- 4th citizen's education-seeking desire remains active next tick

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/education.md:26-35`

## SCENARIO-0106 — School-tier throughput is capped at 12 per cycle, distinct from kindergarten's 10

**Kind:** contract
**Proof seam:** integration
**Owning stories:** STORY-0122

**Preconditions:**
- A school with a throughput ceiling of 12 per cycle and 15 candidates seeking school-tier processing
- A kindergarten with a throughput ceiling of 10 per cycle and 15 candidates seeking kindergarten-tier processing

**Action:**
- Run one cycle of throughput processing for both tiers

**Expected observables:**
- The school processes at most 12 candidates
- The kindergarten processes at most 10 candidates, independently of the school's cap
- School-tier processed count <= 12 this cycle
- Kindergarten-tier processed count <= 10 this cycle

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/education.md:26`

## SCENARIO-0107 — Kindergarten operates fully staffed by workers alone, no profesor tier required

**Kind:** surface
**Proof seam:** integration
**Owning stories:** STORY-0122

**Preconditions:**
- A kindergarten-tier school is staffed with workers only and zero profesors

**Action:**
- Simulate ticks with enrolled kindergarten-age citizens present

**Expected observables:**
- seatTimeMonths increments normally for kindergarten enrollees despite profesor count == 0
- Kindergarten operates at full rate with zero profesors, unlike school and university tiers which require a profesor tier
- No degraded or blocked state caused by the missing profesor tier at kindergarten

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/education.md:30`

## SCENARIO-0108 — Medical university graduate can staff a hospital profesor slot; technical graduate cannot

**Kind:** contract
**Proof seam:** integration
**Owning stories:** STORY-0123

**Preconditions:**
- One citizen graduated with specialisation = medical
- One citizen graduated with specialisation = technical
- A hospital has an open profesor-tier staff slot

**Action:**
- Attempt to assign the medical graduate to the hospital profesor slot
- Attempt to assign the technical graduate to the same slot

**Expected observables:**
- Assignment succeeds
- Assignment is rejected
- Hospital profesor slot filled only by the medical-specialised citizen

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/education.md:41-44`
- `spec/healthcare.md:40-43`

## SCENARIO-0109 — Citizen near an idle hospital with satisfied needs does not sicken

**Kind:** surface
**Proof seam:** integration
**Owning stories:** STORY-0124

**Preconditions:**
- A citizen's food/warmth/water need-satisfaction values are all at maximum
- A hospital is placed adjacent to the citizen's home but never dispatches to or receives them

**Action:**
- Simulate a long run (e.g. equivalent of several in-game months)

**Expected observables:**
- The citizen's sickness probability stays at its needs-derived baseline regardless of hospital proximity
- sick == false throughout
- No health field changed due to hospital adjacency alone

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/healthcare.md:16-21`

## SCENARIO-0110 — Sick citizen is cured only proportional to hospital staffing

**Kind:** failure-recovery
**Proof seam:** integration
**Owning stories:** STORY-0125

**Preconditions:**
- A sick citizen occupies a hospital bed
- The hospital currently has zero staff present

**Action:**
- Simulate ticks with zero staff
- Staff the hospital to full required ratio and simulate further ticks

**Expected observables:**
- Cure progress remains at zero
- Cure progress begins advancing
- Citizen's sick flag clears only after cure progress accumulated during staffed ticks reaches the cure threshold

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/healthcare.md:22-34`

## SCENARIO-0111 — Cure rate is reduced, not zero, when only one hospital staff tier is present

**Kind:** failure-recovery
**Proof seam:** integration
**Owning stories:** STORY-0125

**Preconditions:**
- A hospital is staffed with 50 workers and 0 profesors
- A patient occupies a bed

**Action:**
- Simulate ticks with worker-only staffing
- Staff the hospital with the required 50 profesors and simulate further ticks

**Expected observables:**
- Cure progress advances, but at a reduced rate below the full two-tier rate
- Cure progress rate rises to the full two-tier rate
- Cure rate at worker-only staffing is strictly between zero and the full-staffing rate

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/healthcare.md:42`

## SCENARIO-0112 — Hospital treatment throughput is capped below its bed count

**Kind:** contract
**Proof seam:** integration
**Owning stories:** STORY-0125

**Preconditions:**
- A hospital with beds = 100 and 10 occupied beds holding sick citizens
- Serve-rate cap of 3 patients per cycle

**Action:**
- Simulate one cycle

**Expected observables:**
- At most 3 of the 10 occupied patients receive cure progress this cycle
- The other 7 occupied patients' cure progress is unchanged this cycle
- Patients advanced per cycle <= 3 regardless of free bed count
- occupied.len() unaffected by the serve-rate cap

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/healthcare.md:34`

## SCENARIO-0113 — Unfuelled hospital dispatches no ambulance; citizen falls back to self-travel

**Kind:** failure-recovery
**Proof seam:** integration
**Owning stories:** STORY-0125

**Preconditions:**
- A hospital has fuelStore == 0 and a free bed
- A sick citizen posts a treatment request within the hospital's catchment

**Action:**
- Advance the tick where dispatch would normally trigger
- Continue simulating

**Expected observables:**
- No ambulance trip is created
- The citizen either self-travels to a hospital with a free bed and fuel, or remains untreated without any simulation error
- No ambulance entity spawned from the unfuelled hospital
- No panic or stalled tick

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/healthcare.md:27-29`

## SCENARIO-0114 — Cure rate degrades smoothly as medicine stock depletes, never halting outright

**Kind:** surface
**Proof seam:** integration
**Owning stories:** STORY-0126

**Preconditions:**
- A fully staffed hospital with a patient in a bed
- medicineStore starts full and is not resupplied

**Action:**
- Simulate ticks as medicineStore depletes toward zero from consumption
- Continue simulating after medicineStore reaches zero

**Expected observables:**
- Cure rate per tick decreases proportionally as medicineStore falls
- Cure rate approaches but does not become negative; patient does not instantly worsen or die from medicine shortage alone
- Cure progress accrual rate is a monotonic function of medicineStore level
- No hard stop or error state on medicine exhaustion

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/healthcare.md:36-38`

## SCENARIO-0115 — Citizen death from prolonged untreated sickness never surfaces as a game-over state

**Kind:** failure-recovery
**Proof seam:** e2e
**Owning stories:** STORY-0126

**Preconditions:**
- A citizen is sick with no reachable hospital and remains untreated past the death threshold duration

**Action:**
- Simulate through the death threshold
- Continue simulating the rest of the population

**Expected observables:**
- The citizen entity is removed from the simulation
- The simulation continues running normally with no pause, no failure screen, no termination signal
- Population count decreases by one
- Simulation tick loop continues uninterrupted
- No game-over UI or state is presented

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/healthcare.md:44-47`
- `CLAUDE.md:1-20`

## SCENARIO-0116 — Crime buffer accrues from turn one with zero police buildings placed

**Kind:** surface
**Proof seam:** integration
**Owning stories:** STORY-0089

**Preconditions:**
- A fresh simulation with occupied buildings and no PoliceStation anywhere on the map
- At least one occupant has nonzero unemployment duration

**Action:**
- Simulate ticks from game start

**Expected observables:**
- The occupied building's crimeBuffer becomes nonzero purely from occupant crimePropensity, with no police-unlock gate blocking accrual
- crimeBuffer > 0 despite zero PoliceStation buildings existing

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/crime.md:16-30`

## SCENARIO-0117 — Crime buffer never exceeds the occupant-count cap

**Kind:** contract
**Proof seam:** unit
**Owning stories:** STORY-0089

**Preconditions:**
- A building with occupantCount = 5

**Action:**
- Force maximum crimePropensity on all occupants and simulate many ticks including night multiplier windows

**Expected observables:**
- crimeBuffer growth stops at 5 * 100 = 500
- crimeBuffer <= occupantCount * 100 at all times

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/crime.md:16-21`

## SCENARIO-0118 — Police officer arrests a specific named citizen, not a radius debit

**Kind:** surface
**Proof seam:** e2e
**Owning stories:** STORY-0090

**Preconditions:**
- A building's crimeBuffer exceeds the arrest threshold with 3 identifiable occupants
- A staffed PoliceStation with an available officer/vehicle exists in reach

**Action:**
- Trigger the dispatch and arrest cycle

**Expected observables:**
- An officer entity travels to the building
- Exactly one specific occupant citizen is flagged arrested: true
- Other occupants remain unarrested and unaffected
- One citizen entity has arrested == true and is en route/transported to Court for sentencing
- crimeBuffer at the building is not simply decremented by mere officer presence without a named arrest

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/crime.md:31-34`

## SCENARIO-0119 — Unfuelled police station dispatches no officer

**Kind:** failure-recovery
**Proof seam:** integration
**Owning stories:** STORY-0090

**Preconditions:**
- A PoliceStation has zero fuel in its vehicle fuel store
- A building in its catchment has crimeBuffer above the arrest threshold

**Action:**
- Advance the tick where dispatch would normally trigger
- Continue simulating

**Expected observables:**
- No officer trip is created
- crimeBuffer at the affected building keeps accruing undisturbed
- No arrest occurs until the station is refuelled
- No officer entity spawned from the unfuelled station
- No panic or stalled tick

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/crime.md:52`

## SCENARIO-0120 — Arrests back up in a queue when court capacity is exceeded

**Kind:** failure-recovery
**Proof seam:** integration
**Owning stories:** STORY-0091

**Preconditions:**
- A staffed Court with caseThroughput = 2 cases per cycle
- 5 citizens are arrested and awaiting sentencing this cycle

**Action:**
- Simulate one cycle of court processing
- Simulate a further cycle with no new arrests

**Expected observables:**
- Exactly 2 citizens are sentenced and transported onward to Prison
- The remaining 3 stay queued awaiting sentencing
- The court processes up to 2 more of the queued citizens
- Pending-sentencing queue count decreases only by caseThroughput per cycle
- No simulation error or dropped arrest while the queue is nonzero

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/crime.md:41-53`

## SCENARIO-0121 — Arrest is deferred, not dropped, when the prison is full

**Kind:** failure-recovery
**Proof seam:** integration
**Owning stories:** STORY-0090

**Preconditions:**
- A Prison with cells == occupied.len() (full)
- A new arrest is triggered elsewhere

**Action:**
- Attempt to complete the arrest transport to the full prison
- Free a cell (e.g. release or transfer an inmate) and continue simulating

**Expected observables:**
- The citizen stays flagged arrested: true but is not force-inserted beyond capacity
- The pending arrested citizen is admitted to the now-free cell
- prison.occupied.len() never exceeds cells
- No simulation error on a full prison

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/crime.md:31-34`

## SCENARIO-0122 — Unsupplied prison degrades inmate quality-of-living without killing or releasing inmates

**Kind:** failure-recovery
**Proof seam:** integration
**Owning stories:** STORY-0090

**Preconditions:**
- A Prison holds inmates and its foodDemand goes unsupplied for an extended period

**Action:**
- Simulate ticks with zero food deliveries

**Expected observables:**
- Inmate quality-of-living/health value decreases over time
- Inmates remain incarcerated (not auto-released, not removed)
- No simulation failure or game-over triggered by prison starvation alone

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/crime.md:35-38`

## SCENARIO-0123 — Black-market leak rate rises with shortage severity and falls with enforcement

**Kind:** surface
**Proof seam:** integration
**Owning stories:** STORY-0092

**Preconditions:**
- A district with a warehouse holding surplus stock of an item citizens have unmet demand for
- Baseline enforcement presence in that district

**Action:**
- Increase local unmet demand (shortage severity) while holding enforcement constant
- Increase enforcement presence while holding shortage constant

**Expected observables:**
- leakRate for that district increases
- leakRate for that district decreases
- leakRate is monotonic in shortage severity and inversely monotonic in enforcement, per the specified formula shape

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/crime.md:43-46`

## SCENARIO-0124 — Black-market goods are drawn from real inventory, never conjured

**Kind:** contract
**Proof seam:** integration
**Owning stories:** STORY-0092

**Preconditions:**
- A warehouse holds a known stock quantity of an item
- A citizen has an unmet need for that item and the district's leakRate is nonzero

**Action:**
- Simulate a black-market leak event satisfying the citizen's need

**Expected observables:**
- The warehouse's stock quantity decreases by exactly the leaked amount
- sum(warehouse stock decrease) == sum(citizen need satisfied via black market)
- No item quantity appears from nothing

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/crime.md:43-46`
- `CLAUDE.md:1-6`

## SCENARIO-0125 — Building on a powered road but without a laid wire stays dark

**Kind:** surface
**Proof seam:** integration
**Owning stories:** STORY-0018

**Preconditions:**
- A producer with spare capacity exists on the map
- A road connects the producer's tile to a building's tile
- No wire hop has been placed between the road/producer and the building

**Action:**
- Advance simulation ticks with the building requesting power

**Expected observables:**
- Building reports unpowered
- ElectricityCache-style road-adjacency reachability is not consulted for power state
- Building's power-consuming recipe does not run for lack of electricity input

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/electricity.md:10-24`

## SCENARIO-0126 — Deficit subnetwork brownouts industry before housing goes dark

**Kind:** failure-recovery
**Proof seam:** integration
**Owning stories:** STORY-0021

**Preconditions:**
- One subnetwork with a hospital, a housing block, and an industrial consumer, all wired to the same underpowered producer
- Total demand exceeds available supply

**Action:**
- Reduce plant fuel input so supply falls below total demand

**Expected observables:**
- Industrial consumer's draw is reduced (brownout) first
- Housing draw reduced only if industry brownout alone doesn't close the deficit
- Hospital retains full power longest
- No consumer goes to full blackout while a lower-priority consumer still has full power

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/electricity.md:30-33`

## SCENARIO-0127 — Food factory rejects sub-threshold quality water

**Kind:** surface
**Proof seam:** integration
**Owning stories:** STORY-0127

**Preconditions:**
- A treatment plant delivering water at quality 0.80 to a food-factory recipe building requiring quality >= 0.97
- Flow rate is otherwise sufficient

**Action:**
- Run a tick with the food factory attempting to consume water

**Expected observables:**
- Water input is rejected/gated despite sufficient volume
- Recipe does not advance for lack of usable water input
- Factory output remains at pre-tick level; no water quality bypass occurs

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/water.md:20-23`

## SCENARIO-0128 — Sewage treatment recovers second-grade water, never food-grade

**Kind:** contract
**Proof seam:** unit
**Owning stories:** STORY-0132

**Preconditions:**
- A sewage treatment plant configured per spec (chemicals+power+workers -> water)

**Action:**
- Run treatment with full chemical/power/worker input

**Expected observables:**
- Output water quality is capped at 0.85
- Output quality never exceeds fresh treatment's 0.99 ceiling
- Recovered water cannot satisfy a consumer requiring quality > 0.85 (e.g. a 0.97-threshold food factory)

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/sewage.md:20-29`

## SCENARIO-0129 — Sewage backup with no discharge point gates the producer's tap

**Kind:** failure-recovery
**Proof seam:** integration
**Owning stories:** STORY-0133

**Preconditions:**
- A producer generating sewage into a local buffer
- Its sewage network has no reachable discharge point and treatment is at capacity

**Action:**
- Continue producing sewage past local buffer capacity

**Expected observables:**
- Producer's water consumption is gated (blocked)
- No sewage silently disappears
- Producer's recipe requiring water halts until buffer space frees via eventual treatment capacity

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/sewage.md:35-37`

## SCENARIO-0130 — Cold snap without weather system cannot compute heat demand

**Kind:** contract
**Proof seam:** integration
**Owning stories:** STORY-0030

**Preconditions:**
- A building with temperature-driven heat demand wired into the district heat network
- No weather/climate system providing T(t) exists yet

**Action:**
- Attempt to evaluate clamp((20-T) x k, 0, 4) for the building's demand

**Expected observables:**
- No valid T(t) is available
- Story is blocked pending the weather/climate prerequisite landing
- Heat demand story cannot be implemented standalone; roadmap must sequence weather/climate first

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/heating.md:26-28`

## SCENARIO-0131 — Heat shortfall draws extra electricity before a home goes cold

**Kind:** surface
**Proof seam:** integration
**Owning stories:** STORY-0031

**Preconditions:**
- A building's heat pipe delivery is below its (mocked/fixed) temperature-driven demand
- The building's electricity connection has spare capacity

**Action:**
- Run a tick with the heat deficit present

**Expected observables:**
- Building's electricity draw increases by the heat shortfall amount
- Building's warmth need remains satisfied via the electric fallback
- Building only registers unmet warmth need once electricity is also unavailable

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/heating.md:30-32`

## SCENARIO-0132 — Waste incinerator feeds either the electricity or heat network

**Kind:** contract
**Proof seam:** integration
**Owning stories:** STORY-0136, STORY-0028

**Preconditions:**
- An incinerator building configured to consume waste_burnable
- Wired into either the electricity network or the heat network

**Action:**
- Run incineration with sufficient waste input

**Expected observables:**
- Electricity-mode incinerator converts 3.0 units waste into 33 units electricity onto its wired electricity network
- Heat-mode incinerator converts 2.5 units waste into 450 units heat onto its wired heat network
- Ash byproduct is produced in both modes
- Downstream network (electricity or heat) receives the incinerator's output like any other generator

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/waste.md:28-33`
- `spec/heating.md:16-18`

## SCENARIO-0133 — Full waste container becomes a dispatcher source-job

**Kind:** surface
**Proof seam:** integration
**Owning stories:** STORY-0135

**Preconditions:**
- A garbage office with an available truck
- A container at or above its collection fill threshold

**Action:**
- Advance ticks so the dispatcher evaluates jobs

**Expected observables:**
- Container is queued as a WASTE-class source-job
- A garbage-office truck is dispatched to collect it via the same dispatcher logic as ordinary goods jobs
- Container fill level drops after truck collection; no separate garbage-only scheduling path was used

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/waste.md:24-27`

## SCENARIO-0134 — Import transformer feeds the local grid across the border

**Kind:** surface
**Proof seam:** integration
**Owning stories:** STORY-0023

**Preconditions:**
- An import transformer built at a border tile
- The transformer is wire-connected to the local network

**Action:**
- Advance ticks with local demand exceeding local generation

**Expected observables:**
- Import transformer draws power from across the border
- Imported power is injected into the local grid like any other producer
- Local consumers downstream of the import transformer receive power sourced across the border, priced per spec/trade.md

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/electricity.md:38-40`

## SCENARIO-0135 — Idle building still draws baseline lighting power

**Kind:** contract
**Proof seam:** unit
**Owning stories:** STORY-0024

**Preconditions:**
- A building wired to a powered network with no active recipe running

**Action:**
- Advance a tick with the building idle

**Expected observables:**
- Building's electricity draw equals its idleDraw baseline, not zero
- idleDraw is strictly less than the building's full operating draw
- Network demand accounting includes the idle building's baseline draw

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/electricity.md:32`

## SCENARIO-0136 — Large grid solve stays within its per-tick time budget

**Kind:** contract
**Proof seam:** integration
**Owning stories:** STORY-0025

**Preconditions:**
- A wire network large enough that a full resolve cannot complete within one tick's budget

**Action:**
- Advance ticks while the solver amortises the resolve

**Expected observables:**
- No single tick's solver time exceeds the configured per-tick budget
- The full network resolve completes across multiple ticks
- Power state converges to the correct resolved values after the amortised solve finishes, without a single-tick spike

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/electricity.md:34-36`

## SCENARIO-0137 — Industrial consumer cannot draw from a residential-reserved substation

**Kind:** failure-recovery
**Proof seam:** integration
**Owning stories:** STORY-0130

**Preconditions:**
- A substation flagged residential-only with spare capacity
- An industrial consumer wired only to that substation

**Action:**
- Advance a tick with the industrial consumer requesting water

**Expected observables:**
- Draw request from the industrial consumer is rejected at the reserved substation
- Residential consumers on the same substation are unaffected
- Industrial consumer's water need goes unmet or is routed to a different, non-reserved substation

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/water.md:32-34`

## SCENARIO-0138 — Sorted waste bypasses the separation plant; mixed waste does not

**Kind:** contract
**Proof seam:** integration
**Owning stories:** STORY-0136

**Preconditions:**
- One container with per-type sorting bins in use
- One container collecting only mixed waste
- Both feed the same downstream recycling plants

**Action:**
- Dispatch collection from both containers to the recycling network

**Expected observables:**
- Sorted waste reaches its type-specific recycling plant directly, with no separation-plant hop
- Mixed waste is routed through a separation plant before reaching any type-specific recycling plant
- Sorted waste's recycling throughput is unblocked by separation-plant capacity; mixed waste's is not

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/waste.md:22`

## SCENARIO-0139 — Offline pumping station stops district heat delivery past it

**Kind:** failure-recovery
**Proof seam:** integration
**Owning stories:** STORY-0029

**Preconditions:**
- A district heat trunk line running through a pumping station to downstream buildings
- Downstream pipe capacity is otherwise sufficient

**Action:**
- Remove power or workers from the pumping station and advance ticks

**Expected observables:**
- No heat is delivered past the offline pumping station
- Downstream buildings register unmet heat demand despite sufficient pipe capacity
- Restoring the pumping station's power/workers resumes heat delivery past it

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/heating.md:22`

## SCENARIO-0140 — Consumer classes gate at different water-quality thresholds

**Kind:** contract
**Proof seam:** integration
**Owning stories:** STORY-0127

**Preconditions:**
- An animal farm, a food factory, and a nuclear cooling consumer each wired to the same network
- Delivered water quality is 0.90 (below food factory's 0.97 and animal farm's 0.93, above nuclear cooling's 0.60)

**Action:**
- Run a tick with all three consumers attempting to draw water

**Expected observables:**
- Food factory rejects the water (0.90 < 0.97)
- Animal farm rejects the water (0.90 < 0.93)
- Nuclear cooling accepts the water (0.90 >= 0.60)
- Only the nuclear cooling consumer's recipe advances this tick; the other two remain gated

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/water.md:20`

## SCENARIO-0141 — Outdoor temperature is reproducible across a save/load round-trip

**Kind:** contract
**Proof seam:** integration
**Owning stories:** STORY-0138

**Preconditions:**
- A deterministic TestCtx world with a fixed seed and weather enabled
- No other systems need to be active for this scenario

**Action:**
- Tick the world to an arbitrary point mid-cycle and record T(t)
- Save the world, then load it back into a fresh TestCtx
- Tick the loaded world by zero ticks and read T(t) again
- Tick both the original (had it kept running) and the loaded world forward by the same number of ticks and compare

**Expected observables:**
- T(t) is a finite value consistent with the annual/diurnal cycle
- The load succeeds with no error
- T(t) immediately after load equals T(t) immediately before save, bit-for-bit
- The two T(t) trajectories are identical
- The per-tick state hash is identical between the two runs
- T(t) is bit-for-bit reproducible across a save/load round-trip
- No nondeterminism is introduced by the weather subsystem

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/heating.md:26-36`

## SCENARIO-0142 — Delivery waits when no compatible vehicle is idle

**Kind:** surface
**Proof seam:** integration
**Owning stories:** STORY-0140

**Preconditions:**
- A buy order and matching sell order exist for a resource
- No idle vehicle with matching cargoClass and available capacity exists at any depot

**Action:**
- Run make_trades / dispatch tick with zero idle compatible vehicles

**Expected observables:**
- No Trade is produced for that pair
- The buy and sell orders remain queued (not removed) after the tick
- Buy order still present in market.buy_orders
- Sell order still present in market.sell_orders
- No vehicle position changed

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/vehicles.md:95-97`

## SCENARIO-0143 — External buy order does not resolve instantly once queue requirement is in place

**Kind:** contract
**Proof seam:** integration
**Owning stories:** STORY-0051

**Preconditions:**
- A buy order is unmet by any internal seller
- find_external returns Some(customs_partner)
- The new queue/vehicle-trip requirement is implemented

**Action:**
- Call make_trades on the tick the buy order is posted
- Advance ticks until a customs vehicle trip would physically complete

**Expected observables:**
- No Trade referencing this buyer and the external partner appears in all_trades on this same tick
- A Trade referencing this buyer and the external partner appears only after the simulated travel/loading time has elapsed
- Buyer's capital increases only after the simulated trip completes, not on the posting tick

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/logistics.md:48-58`

## SCENARIO-0144 — External customs partner has a throughput cap

**Kind:** failure-recovery
**Proof seam:** integration
**Owning stories:** STORY-0051

**Preconditions:**
- N buy orders exceed the customs partner's per-tick throughput limit
- All N orders are otherwise eligible for external fulfilment

**Action:**
- Run make_trades for a tick where demand exceeds customs capacity

**Expected observables:**
- Only up-to-capacity orders are resolved into trades
- Remaining orders stay queued, unresolved, for a later tick
- Count of resolved external trades this tick <= customs capacity
- Unresolved buy orders remain in market.buy_orders

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/logistics.md:48-58,95-100`

## SCENARIO-0145 — External buy fails cleanly when no freight station exists

**Kind:** failure-recovery
**Proof seam:** unit
**Owning stories:** STORY-0052

**Preconditions:**
- An unmet buy order exists
- find_external(order.pos) is stubbed to return None

**Action:**
- Call make_trades with a find_external closure returning None for this order's position

**Expected observables:**
- capital[buyer] is unchanged from its pre-call value
- No Trade is pushed to all_trades for this buyer
- capital.get(&buyer) equals the pre-call value
- all_trades contains no entry with this buyer

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/logistics.md:95-100`

## SCENARIO-0146 — External sell fails cleanly when no freight station exists

**Kind:** failure-recovery
**Proof seam:** unit
**Owning stories:** STORY-0052

**Preconditions:**
- A sell order has surplus qty_sell > 0
- find_external(order.pos) is stubbed to return None

**Action:**
- Call make_trades with a find_external closure returning None for this order's position

**Expected observables:**
- capital[seller] is unchanged from its pre-call value
- order.qty for the seller is unchanged from its pre-call value
- No Trade is pushed to all_trades for this seller
- capital.get(&seller) equals the pre-call value
- sell_orders[seller].qty equals the pre-call value

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/logistics.md:95-100`

## SCENARIO-0147 — Incompatible cargo class is never matched

**Kind:** contract
**Proof seam:** unit
**Owning stories:** STORY-0035

**Preconditions:**
- A sell order for gravel and a buy order for bagged goods exist at the same position (distance2 = 0)
- Only a gravel-class vehicle is idle

**Action:**
- Run the matcher over this mismatched pair

**Expected observables:**
- No candidate trade is generated despite zero distance
- The matcher does not fall back to matching on proximity alone
- all_trades contains no trade between this seller and buyer

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/logistics.md:31-40`

## SCENARIO-0148 — Deficit priority outranks pure distance

**Kind:** surface
**Proof seam:** unit
**Owning stories:** STORY-0146

**Preconditions:**
- Seller with sufficient stock exists
- Buyer A: distance 10, deficit 90% below target
- Buyer B: distance 5, deficit 5% below target

**Action:**
- Run the deficit-aware matcher with both buyers competing for the same limited seller stock

**Expected observables:**
- Buyer A (larger deficit) is served first even though Buyer B is closer
- Trade list shows Buyer A resolved before/instead of Buyer B when stock is insufficient for both

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/logistics.md:60-72`

## SCENARIO-0149 — Empty fuel tank halts vehicle dispatch

**Kind:** surface
**Proof seam:** integration
**Owning stories:** STORY-0139

**Preconditions:**
- A Vehicle entity has fuel = 0
- A dispatch request targets this vehicle as the nearest idle candidate

**Action:**
- Run the dispatch selection pass

**Expected observables:**
- The empty-tank vehicle is skipped
- A different fueled vehicle is selected, or the request stays queued if none is available
- The empty-tank vehicle's position/state is unchanged after the tick

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/vehicles.md:56-58`

## SCENARIO-0150 — Depot cannot exceed its physical parking slot count

**Kind:** failure-recovery
**Proof seam:** integration
**Owning stories:** STORY-0141

**Preconditions:**
- A depot has N reserved parking slots, all currently occupied by N vehicles

**Action:**
- Attempt to assign vehicle N+1 to this depot

**Expected observables:**
- The assignment is rejected or the vehicle is redirected to a depot with a free slot
- Depot's owned-vehicle count never exceeds its parking-slot count

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/vehicles.md:22-24,64`

## SCENARIO-0151 — Dispatch progresses through all four states in order

**Kind:** surface
**Proof seam:** integration
**Owning stories:** STORY-0149

**Preconditions:**
- A vehicle is assigned a fresh dispatch for a buy/sell pair at different positions

**Action:**
- Advance ticks until the vehicle reaches the source
- Advance ticks through loading completion and travel to destination
- Advance ticks through unload completion

**Expected observables:**
- Dispatch state is travel-to-source before arrival, loading only after arrival
- Dispatch state moves to travel-to-destination only after loading completes
- Dispatch state moves to unloading only after the vehicle reaches the destination
- Vehicle becomes idle or receives a new assignment
- Cargo is cleared from the vehicle
- Dispatch state trace shows exactly travel-to-source -> loading -> travel-to-destination -> unloading in order
- Vehicle position at each state transition matches source/destination positions

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/logistics.md:32,111`

## SCENARIO-0152 — Vehicle at zero condition produces scrap materials, not deletion

**Kind:** surface
**Proof seam:** integration
**Owning stories:** STORY-0144

**Preconditions:**
- A Vehicle entity's condition has reached zero this tick

**Action:**
- Run the vehicle-lifecycle tick that processes zero-condition vehicles

**Expected observables:**
- The vehicle entity is removed from the active fleet
- waste_steel and waste_aluminium cargo quantities increase at the scrapyard by an amount > 0
- Vehicle id no longer appears in the fleet's dispatch-eligible set
- Scrapyard waste_steel/waste_aluminium stock increased

**Automation status:** pending
**Execution command:** TBD

**Sources:**
- `spec/vehicles.md:67`
