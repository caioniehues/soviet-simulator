# Behavior Corpus

Every scenario in the corpus, its proof seam, and how often it runs.
`Command` is filled in by the implementing iteration that first proves the scenario.

| Scenario ID | Title | Kind | Proof seam | Run cadence | Command | Owning stories |
|---|---|---|---|---|---|---|
| SCENARIO-0001 | Building a road creates no automatic lots | contract | integration | iteration | TBD | STORY-0013 |
| SCENARIO-0002 | Blueprint placement defers all capability activation | surface | integration | iteration | TBD | STORY-0001 |
| SCENARIO-0003 | Construction phase stalls without required material, resumes on delivery | failure-recovery | integration | iteration | TBD | STORY-0003 |
| SCENARIO-0004 | Construction phase stalls without matching vehicle skill | failure-recovery | integration | iteration | TBD | STORY-0003, STORY-0008 |
| SCENARIO-0005 | Second crane assigned shortens phase completion time | surface | unit | iteration | TBD | STORY-0004 |
| SCENARIO-0006 | Demolition emits sorted rubble, never a money refund | contract | integration | iteration | TBD | STORY-0009 |
| SCENARIO-0007 | Renovation adds capacity without evicting occupants | surface | integration | iteration | TBD | STORY-0010 |
| SCENARIO-0008 | Building condition decays under starved maintenance input and cannot be auto-demolished | failure-recovery | integration | iteration | TBD | STORY-0011 |
| SCENARIO-0009 | Zoning polygon paint alone creates no building | contract | e2e | sentinel | TBD | STORY-0014 |
| SCENARIO-0010 | Placement rejected on land failing the physical siting checklist | failure-recovery | integration | iteration | TBD | STORY-0015 |
| SCENARIO-0011 | Construction office dispatches idle vehicle to nearest stalled site | surface | integration | iteration | TBD | STORY-0007 |
| SCENARIO-0012 | Construction phase stalls on no-worker distinct from no-machine | failure-recovery | integration | iteration | TBD | STORY-0003 |
| SCENARIO-0013 | Demolition stalls without explosives, resumes on delivery | failure-recovery | integration | iteration | TBD | STORY-0009 |
| SCENARIO-0014 | Road construction progresses through its own phase pipeline and stalls without gravel or asphalt | failure-recovery | integration | iteration | TBD | STORY-0006 |
| SCENARIO-0015 | New game starts with a stocked starter warehouse | surface | e2e | sentinel | TBD | STORY-0023 |
| SCENARIO-0016 | Deadlocked domestic chain is broken by a marked-up customs import | failure-recovery | integration | iteration | TBD | STORY-0024, STORY-0027 |
| SCENARIO-0017 | Foreign trade order stalls with no vehicle capacity, no money moves | failure-recovery | integration | iteration | TBD | STORY-0025 |
| SCENARIO-0018 | Household cash cannot be spent using enterprise accounting roubles | contract | unit | iteration | TBD | STORY-0020 |
| SCENARIO-0019 | Excess retail demand lengthens the queue instead of raising the price | surface | integration | iteration | TBD | STORY-0021 |
| SCENARIO-0020 | Treasury and simulation stay consistent across a mid-transit save/load | contract | process-level | iteration | TBD | STORY-0027 |
| SCENARIO-0021 | Dollar-only good cannot be bought with a rouble-only balance | contract | integration | iteration | TBD | STORY-0030 |
| SCENARIO-0022 | Household nal balance persists across save and load | contract | process-level | iteration | TBD | STORY-0017 |
| SCENARIO-0023 | Wage payment debits employer beznal and credits worker nal on interval | surface | integration | iteration | TBD | STORY-0018 |
| SCENARIO-0024 | Domestic enterprise-to-enterprise trade settles in beznal with a nonzero delta | surface | integration | iteration | TBD | STORY-0019 |
| SCENARIO-0025 | Building a customs house creates a placeable border-crossing entity | surface | app-level | iteration | TBD | STORY-0026 |
| SCENARIO-0026 | Foreign trade order cannot resolve with no customs house built | failure-recovery | integration | iteration | TBD | STORY-0026 |
| SCENARIO-0027 | Import order outside its era/bloc window is rejected at customs | failure-recovery | integration | iteration | TBD | STORY-0033 |
| SCENARIO-0028 | Used vehicle export fetches less hard currency than an equivalent new vehicle | contract | integration | iteration | TBD | STORY-0034 |
| SCENARIO-0029 | Player-facing view shows a price move and its driver | surface | app-level | iteration | TBD | STORY-0022 |
| SCENARIO-0030 | A* refuses a route through a banned turn at an intersection | contract | integration | iteration | TBD | STORY-0035 |
| SCENARIO-0031 | Route cost prefers the faster lane absent congestion | surface | unit | iteration | TBD | STORY-0035 |
| SCENARIO-0032 | Two identical trip requests each allocate independent fresh paths | contract | integration | iteration | TBD | STORY-0035 |
| SCENARIO-0033 | Paved road yields higher effective speed than dirt for the same vehicle | surface | unit | iteration | TBD | STORY-0036 |
| SCENARIO-0034 | Per-lane EMA rises under sustained load and decays once load stops | surface | unit | iteration | TBD | STORY-0042 |
| SCENARIO-0035 | BPR cost multiplier matches the formula at known v/c ratios | contract | unit | iteration | TBD | STORY-0043 |
| SCENARIO-0036 | Damped remembered cost blends across successive updates | contract | unit | iteration | TBD | STORY-0044 |
| SCENARIO-0037 | Two equal-cost parallel corridors do not flap under repeated re-routing | failure-recovery | process-level | iteration | TBD | STORY-0044 |
| SCENARIO-0038 | Gridlocked vehicle persists in the simulation indefinitely | failure-recovery | integration | iteration | TBD | STORY-0047 |
| SCENARIO-0039 | Stalled vehicle with an alternative route re-routes instead of waiting forever | failure-recovery | integration | iteration | TBD | STORY-0048 |
| SCENARIO-0040 | Stalled vehicle with no alternative registers a planner-visible bottleneck event | failure-recovery | app-level | iteration | TBD | STORY-0048 |
| SCENARIO-0041 | Corridor utilisation readout matches the internal EMA value exactly | contract | app-level | iteration | TBD | STORY-0045 |
| SCENARIO-0042 | Vehicle brakes to avoid rear-ending a slower vehicle ahead | surface | integration | iteration | TBD | STORY-0046 |
| SCENARIO-0043 | Heavy vehicle request is excluded from a pedestrian-only lane | contract | integration | iteration | TBD | STORY-0037 |
| SCENARIO-0044 | Hard modifiers apply the exact car-ban, transit-lane and closed-segment costs | contract | unit | iteration | TBD | STORY-0037 |
| SCENARIO-0045 | Vehicle continues its original route despite rising ambient congestion | contract | integration | iteration | TBD | STORY-0035 |
| SCENARIO-0046 | Redrawing a road invalidates routes that reference the changed segment | failure-recovery | integration | iteration | TBD | STORY-0035 |
| SCENARIO-0047 | A burst of simultaneous path requests is solved across multiple ticks without blocking the frame | contract | integration | iteration | TBD | STORY-0038 |
| SCENARIO-0048 | A ninth road segment cannot attach to an already-full intersection | contract | integration | iteration | TBD | STORY-0039 |
| SCENARIO-0049 | A road segment does not exist until its construction project completes | contract | integration | iteration | TBD | STORY-0040 |
| SCENARIO-0050 | Upgrading dirt to paved requires materials and labour, not a money payment | contract | integration | iteration | TBD | STORY-0040 |
| SCENARIO-0051 | A trip entirely inside a large compound routes over its internal connection graph | surface | integration | iteration | TBD | STORY-0041 |
| SCENARIO-0052 | Stall response waits before attempting re-route, and re-routes before registering a stall event | contract | integration | iteration | TBD | STORY-0048 |
| SCENARIO-0053 | Heavy truck loses proportionally more speed on dirt than a light vehicle | surface | unit | iteration | TBD | STORY-0036 |
| SCENARIO-0054 | Road prefab lane template names each lane's type from the fixed taxonomy | contract | unit | iteration | TBD | STORY-0036 |
| SCENARIO-0055 | Reserved braking-distance space matches half-v-squared-over-a plus half-length at known speed and deceleration | contract | unit | iteration | TBD | STORY-0046 |
| SCENARIO-0056 | Save/load roundtrip preserves citizen identity | surface | integration | iteration | TBD | STORY-0049 |
| SCENARIO-0057 | Determinism replay detects accidental citizen-state divergence | contract | integration | iteration | TBD | STORY-0049 |
| SCENARIO-0058 | Workplace binding survives a full workday cycle without silent reassignment | surface | integration | iteration | TBD | STORY-0053 |
| SCENARIO-0059 | Buy order match does not move goods before physical arrival | failure-recovery | integration | iteration | TBD | STORY-0057 |
| SCENARIO-0060 | Household pantry is shared, not per-member | surface | integration | iteration | TBD | STORY-0060 |
| SCENARIO-0061 | Housing queue grows visibly when no vacancy exists | surface | app-level | iteration | TBD | STORY-0061 |
| SCENARIO-0062 | Eviction returns a household to the queue instead of deleting it | failure-recovery | integration | iteration | TBD | STORY-0061 |
| SCENARIO-0063 | Job assignment respects minimum education tier over raw proximity | contract | integration | iteration | TBD | STORY-0054 |
| SCENARIO-0064 | Sick citizen's economic activity freezes until hospital capacity resolves it | surface | integration | iteration | TBD | STORY-0050 |
| SCENARIO-0065 | Citizen death frees a household slot without dissolving the household | surface | integration | iteration | TBD | STORY-0051 |
| SCENARIO-0066 | Unmet need surfaces as a visible waiting or going-without state | surface | app-level | iteration | TBD | STORY-0059 |
| SCENARIO-0067 | Full-population tick time stays within budget at the profiled ceiling | surface | process-level | iteration | TBD | STORY-0063 |
| SCENARIO-0068 | Childcare access raises the observed birth rate | surface | integration | iteration | TBD | STORY-0052 |
| SCENARIO-0069 | Adult child leaving home enters the housing queue | surface | integration | iteration | TBD | STORY-0061 |
| SCENARIO-0070 | Plan-recruited immigrant household enters the housing queue | surface | integration | iteration | TBD | STORY-0061 |
| SCENARIO-0071 | Student seat allocation is capacity-limited and required for credentials | contract | integration | iteration | TBD | STORY-0055 |
| SCENARIO-0072 | Labour-pool dispatch staffs a construction surge without moving fixed workers | surface | integration | iteration | TBD | STORY-0056 |
| SCENARIO-0073 | Commute departure times follow a time-of-day curve, not a uniform draw | surface | integration | iteration | TBD | STORY-0054 |
| SCENARIO-0074 | Sustained low wellbeing raises crime contribution and lowers attendance probability | surface | integration | iteration | TBD | STORY-0054 |
| SCENARIO-0075 | Work efficiency scales continuously with health | surface | integration | iteration | TBD | STORY-0050 |
| SCENARIO-0076 | Citizen graduates lifecycle stages at age thresholds | surface | integration | iteration | TBD | STORY-0051 |
| SCENARIO-0077 | Wants satisfaction is spatially quality-weighted | surface | integration | iteration | TBD | STORY-0058 |
| SCENARIO-0078 | Fixing a mobility gap lowers car aspiration pressure below its demand threshold | contract | integration | iteration | TBD | STORY-0058 |
| SCENARIO-0079 | Rest satisfaction improves with a shorter commute | surface | integration | iteration | TBD | STORY-0058 |
| SCENARIO-0080 | Dwelling quality and default heat/electricity requirement feed housing need and reject silent opt-out | contract | integration | iteration | TBD | STORY-0062 |
| SCENARIO-0081 | Refinery-style recipe consumes one input and yields two co-products | surface | unit | iteration | TBD | STORY-0064 |
| SCENARIO-0082 | Mine produces raw ore with zero declared inputs | surface | unit | iteration | TBD | STORY-0065 |
| SCENARIO-0083 | Full output warehouse halts a producing factory, then resumes when space clears | failure-recovery | integration | iteration | TBD | STORY-0066 |
| SCENARIO-0084 | Linear staffing factor understates the intended curve's penalty at partial staffing | contract | unit | iteration | TBD | STORY-0069 |
| SCENARIO-0085 | Partial power availability throttles output instead of an all-or-nothing blackout | failure-recovery | integration | iteration | TBD | STORY-0070 |
| SCENARIO-0086 | Scarcest of two under-supplied inputs sets the throttled rate | contract | unit | iteration | TBD | STORY-0071 |
| SCENARIO-0087 | Two independently-deficient factors compound multiplicatively, not by minimum | contract | unit | iteration | TBD | STORY-0072 |
| SCENARIO-0088 | Player inspects a stalled factory and sees which input is missing | surface | app-level | iteration | TBD | STORY-0073 |
| SCENARIO-0089 | An enterprise with an inflated request accumulates surplus stock a matched honest enterprise does not | surface | integration | iteration | TBD | STORY-0076, STORY-0077 |
| SCENARIO-0090 | Player detects a hoarding enterprise from its inspection panel alone | surface | e2e | sentinel | TBD | STORY-0078 |
| SCENARIO-0091 | Steel and fuel resources declare incompatible transport classes | contract | unit | iteration | TBD | STORY-0079 |
| SCENARIO-0092 | Meat left outside cold storage decays past its shelf life | failure-recovery | integration | iteration | TBD | STORY-0081 |
| SCENARIO-0093 | A combustion recipe emits ash that fills its own output bucket and can halt the recipe | surface | integration | iteration | TBD | STORY-0084, STORY-0085 |
| SCENARIO-0094 | Cooled-class storage bucket rejects an incompatible open-class resource | contract | unit | iteration | TBD | STORY-0079 |
| SCENARIO-0095 | Electricity is never returned by a query for vehicle-transportable resources | contract | integration | iteration | TBD | STORY-0079 |
| SCENARIO-0096 | Sub-threshold water quality blocks the recipe outright despite full staffing and power | failure-recovery | unit | iteration | TBD | STORY-0074 |
| SCENARIO-0097 | Bottleneck reason updates immediately when an input stock is depleted mid-cycle | failure-recovery | integration | iteration | TBD | STORY-0075 |
| SCENARIO-0098 | Recycling waste_steel at its confirmed 0.98 yield recovers 98 of 100 units consumed | contract | unit | iteration | TBD | STORY-0086 |
| SCENARIO-0099 | A professor shortfall throttles output multiplicatively alongside a simultaneous labour shortfall | contract | unit | iteration | TBD | STORY-0088 |
| SCENARIO-0100 | Child cannot enrol when all schools are at capacity | surface | integration | iteration | TBD | STORY-0091 |
| SCENARIO-0101 | Serviceable seats shrink below raw capacity when the school underproduces | contract | integration | iteration | TBD | STORY-0091 |
| SCENARIO-0102 | School operating rate is proportional to its two-tier staff composition, not just staff presence | failure-recovery | integration | iteration | TBD | STORY-0091 |
| SCENARIO-0103 | Enrolled citizen cannot simultaneously hold a job | contract | integration | iteration | TBD | STORY-0091 |
| SCENARIO-0104 | Seat-time only accrues while the school is staffed and operating | failure-recovery | integration | iteration | TBD | STORY-0092 |
| SCENARIO-0105 | University enrolment is capped below school-tier enrolment | contract | integration | iteration | TBD | STORY-0092 |
| SCENARIO-0106 | School-tier throughput is capped at 12 per cycle, distinct from kindergarten's 10 | contract | integration | iteration | TBD | STORY-0092 |
| SCENARIO-0107 | Kindergarten operates fully staffed by workers alone, no profesor tier required | surface | integration | iteration | TBD | STORY-0092 |
| SCENARIO-0108 | Medical university graduate can staff a hospital profesor slot; technical graduate cannot | contract | integration | iteration | TBD | STORY-0093 |
| SCENARIO-0109 | Citizen near an idle hospital with satisfied needs does not sicken | surface | integration | iteration | TBD | STORY-0094 |
| SCENARIO-0110 | Sick citizen is cured only proportional to hospital staffing | failure-recovery | integration | iteration | TBD | STORY-0095 |
| SCENARIO-0111 | Cure rate is reduced, not zero, when only one hospital staff tier is present | failure-recovery | integration | iteration | TBD | STORY-0095 |
| SCENARIO-0112 | Hospital treatment throughput is capped below its bed count | contract | integration | iteration | TBD | STORY-0095 |
| SCENARIO-0113 | Unfuelled hospital dispatches no ambulance; citizen falls back to self-travel | failure-recovery | integration | iteration | TBD | STORY-0095 |
| SCENARIO-0114 | Cure rate degrades smoothly as medicine stock depletes, never halting outright | surface | integration | iteration | TBD | STORY-0096 |
| SCENARIO-0115 | Citizen death from prolonged untreated sickness never surfaces as a game-over state | failure-recovery | e2e | sentinel | TBD | STORY-0096 |
| SCENARIO-0116 | Crime buffer accrues from turn one with zero police buildings placed | surface | integration | iteration | TBD | STORY-0097 |
| SCENARIO-0117 | Crime buffer never exceeds the occupant-count cap | contract | unit | iteration | TBD | STORY-0097 |
| SCENARIO-0118 | Police officer arrests a specific named citizen, not a radius debit | surface | e2e | sentinel | TBD | STORY-0098 |
| SCENARIO-0119 | Unfuelled police station dispatches no officer | failure-recovery | integration | iteration | TBD | STORY-0098 |
| SCENARIO-0120 | Arrests back up in a queue when court capacity is exceeded | failure-recovery | integration | iteration | TBD | STORY-0099 |
| SCENARIO-0121 | Arrest is deferred, not dropped, when the prison is full | failure-recovery | integration | iteration | TBD | STORY-0098 |
| SCENARIO-0122 | Unsupplied prison degrades inmate quality-of-living without killing or releasing inmates | failure-recovery | integration | iteration | TBD | STORY-0098 |
| SCENARIO-0123 | Black-market leak rate rises with shortage severity and falls with enforcement | surface | integration | iteration | TBD | STORY-0100 |
| SCENARIO-0124 | Black-market goods are drawn from real inventory, never conjured | contract | integration | iteration | TBD | STORY-0100 |
| SCENARIO-0125 | Building on a powered road but without a laid wire stays dark | surface | integration | iteration | TBD | STORY-0101 |
| SCENARIO-0126 | Deficit subnetwork brownouts industry before housing goes dark | failure-recovery | integration | iteration | TBD | STORY-0104 |
| SCENARIO-0127 | Food factory rejects sub-threshold quality water | surface | integration | iteration | TBD | STORY-0109 |
| SCENARIO-0128 | Sewage treatment recovers second-grade water, never food-grade | contract | unit | iteration | TBD | STORY-0114 |
| SCENARIO-0129 | Sewage backup with no discharge point gates the producer's tap | failure-recovery | integration | iteration | TBD | STORY-0115 |
| SCENARIO-0130 | Cold snap without weather system cannot compute heat demand | contract | integration | iteration | TBD | STORY-0118 |
| SCENARIO-0131 | Heat shortfall draws extra electricity before a home goes cold | surface | integration | iteration | TBD | STORY-0119 |
| SCENARIO-0132 | Waste incinerator feeds either the electricity or heat network | contract | integration | iteration | TBD | STORY-0122, STORY-0116 |
| SCENARIO-0133 | Full waste container becomes a dispatcher source-job | surface | integration | iteration | TBD | STORY-0121 |
| SCENARIO-0134 | Import transformer feeds the local grid across the border | surface | integration | iteration | TBD | STORY-0106 |
| SCENARIO-0135 | Idle building still draws baseline lighting power | contract | unit | iteration | TBD | STORY-0107 |
| SCENARIO-0136 | Large grid solve stays within its per-tick time budget | contract | integration | iteration | TBD | STORY-0108 |
| SCENARIO-0137 | Industrial consumer cannot draw from a residential-reserved substation | failure-recovery | integration | iteration | TBD | STORY-0112 |
| SCENARIO-0138 | Sorted waste bypasses the separation plant; mixed waste does not | contract | integration | iteration | TBD | STORY-0122 |
| SCENARIO-0139 | Offline pumping station stops district heat delivery past it | failure-recovery | integration | iteration | TBD | STORY-0117 |
| SCENARIO-0140 | Consumer classes gate at different water-quality thresholds | contract | integration | iteration | TBD | STORY-0109 |
| SCENARIO-0141 | Delivery waits when no compatible vehicle is idle | surface | integration | iteration | TBD | STORY-0125 |
| SCENARIO-0142 | External buy order does not resolve instantly once queue requirement is in place | contract | integration | iteration | TBD | STORY-0028 |
| SCENARIO-0143 | External customs partner has a throughput cap | failure-recovery | integration | iteration | TBD | STORY-0028 |
| SCENARIO-0144 | External buy fails cleanly when no freight station exists | failure-recovery | unit | iteration | TBD | STORY-0029 |
| SCENARIO-0145 | External sell fails cleanly when no freight station exists | failure-recovery | unit | iteration | TBD | STORY-0029 |
| SCENARIO-0146 | Incompatible cargo class is never matched | contract | unit | iteration | TBD | STORY-0130 |
| SCENARIO-0147 | Deficit priority outranks pure distance | surface | unit | iteration | TBD | STORY-0136 |
| SCENARIO-0148 | Empty fuel tank halts vehicle dispatch | surface | integration | iteration | TBD | STORY-0124 |
| SCENARIO-0149 | Depot cannot exceed its physical parking slot count | failure-recovery | integration | iteration | TBD | STORY-0126 |
| SCENARIO-0150 | Dispatch progresses through all four states in order | surface | integration | iteration | TBD | STORY-0139 |
| SCENARIO-0151 | Vehicle at zero condition produces scrap materials, not deletion | surface | integration | iteration | TBD | STORY-0129 |
