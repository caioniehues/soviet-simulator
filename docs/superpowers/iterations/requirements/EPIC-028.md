# EPIC-028 — Electricity — wired network replaces coverage

**Summary:** Electricity — wired network replaces coverage
**Stories:** STORY-0101, STORY-0102, STORY-0103, STORY-0104, STORY-0105, STORY-0106, STORY-0107, STORY-0108
**Primary sources:** `spec/electricity.md`
**Status:** 0/8 done

## STORY-0101

**Epic:** EPIC-028 — Electricity — wired network replaces coverage
**Title:** Replace road-adjacency power with laid-wire connectivity

**As a** planner
**I want** a building to be powered only when a wire I laid connects it to a producer with spare capacity
**So that** power exists only where a wire runs, matching the project's one physical-causality rule

**Acceptance criteria:**
- AC-1: A building adjacent to a powered road but with no wire hop laid to it is unpowered, even though today's ElectricityCache union-find would mark it powered. [SUBSTRATE: CONFLICTS — map/electricity_cache.rs:39-63,244-280] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0125`
- AC-2: Power reachability is computed over an explicit wire-hop graph the planner builds, not over road-graph adjacency; a producer connected only by road (no wire) supplies no one. [SUBSTRATE: CONFLICTS — map/electricity_cache.rs:244-280 (map_electricity_edges)] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0125`
- AC-3: A building wired to a producer whose network has no blackout still loses power if the specific wire path to it is capacity-saturated, i.e. blackout is not merely binary per-network as today. [SUBSTRATE: CONFLICTS — map_dynamic/electricity.rs:89] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0125`

**Sources:**
- `spec/electricity.md:10-24`

**Status:** pending

## STORY-0102

**Epic:** EPIC-028 — Electricity — wired network replaces coverage
**Title:** Two-tier voltage transmission with transformers and substations

**As a** planner
**I want** a typed chain of plant to HIGH transmission lines to transformer to LOW distribution lines to substation to buildings
**So that** the grid has real topology instead of a flat conductivity brush

**Acceptance criteria:**
- AC-1: HIGH-tier lines connect only to plants, transformers, and other HIGH lines; LOW-tier lines connect only to transformers, substations, and buildings — a building cannot draw directly from a HIGH line. [SUBSTRATE: ABSENT — greenfield, no voltage-tier concept exists in map/electricity_cache.rs] · impact:`cross-surface` · seam:`integration`
- AC-2: A transformer converts HIGH input to LOW output up to its own capacity; drawing more than the transformer's capacity through it leaves downstream substations under-supplied even if upstream plant capacity is ample. [SUBSTRATE: ABSENT — greenfield] · impact:`cross-surface` · seam:`integration`

**Sources:**
- `spec/electricity.md:20-24`

**Status:** pending

## STORY-0103

**Epic:** EPIC-028 — Electricity — wired network replaces coverage
**Title:** Capacitated lines with distance-based transmission loss

**As a** planner
**I want** each line class to have a throughput rating and per-km loss, with HIGH lines losing less than LOW
**So that** plant siting and substation placement are physically meaningful decisions

**Acceptance criteria:**
- AC-1: A wire segment carrying load above its rated capacity delivers no more than its capacity downstream, regardless of upstream supply. [SUBSTRATE: OURS/ABSENT — greenfield, no $CAPACITY equivalent exists on any Egregoria edge type] · impact:`local` · seam:`unit`
- AC-2: Power delivered at the far end of a LOW line is less than power injected at the near end by an amount proportional to line length, and the same length of HIGH line loses proportionally less. [SUBSTRATE: OURS/ABSENT — greenfield] · impact:`local` · seam:`unit`

**Sources:**
- `spec/electricity.md:26-29`

**Status:** pending

## STORY-0104

**Epic:** EPIC-028 — Electricity — wired network replaces coverage
**Title:** Priority-class brownout before blackout

**As a** planner
**I want** a starved subnetwork to shed load by planner-set priority class (hospitals before housing before industry) with graded brownout before any consumer goes fully dark
**So that** scarcity is felt as leaner tranches, not an unqualified binary outage

**Acceptance criteria:**
- AC-1: When a subnetwork's supply falls short of total demand, a lower-priority consumer's draw is reduced (brownout) before a higher-priority consumer loses any power, replacing today's single per-network blackout boolean that treats every consumer identically. [SUBSTRATE: CONFLICTS — map_dynamic/electricity.rs:89 (binary per-network blackout, no priority classes)] · impact:`journey` · seam:`integration` · scenario:`SCENARIO-0126`
- AC-2: A consumer only goes to full blackout after brownout has already reduced its draw to a floor and supply is still insufficient — blackout is never the first response to a deficit. [SUBSTRATE: CONFLICTS — map_dynamic/electricity.rs:89] · impact:`journey` · seam:`integration` · scenario:`SCENARIO-0126`

**Sources:**
- `spec/electricity.md:30-33`

**Status:** pending

## STORY-0105

**Epic:** EPIC-028 — Electricity — wired network replaces coverage
**Title:** Power plants are ordinary recipe buildings

**As a** planner
**I want** a power plant to require fuel and workers like any production building, producing electricity only while both are supplied
**So that** generation obeys the same production model as every other recipe building

**Acceptance criteria:**
- AC-1: A power plant with no fuel input or no assigned workers produces zero electricity, using the same recipe/backpressure machinery as goods buildings (power_production/power_consumption fields). [SUBSTRATE: PROVIDED pattern — prototypes/src/prototypes/building.rs:34-35, souls/goods_company.rs:36-39] · impact:`local` · seam:`integration`

**Sources:**
- `spec/electricity.md:16-18`

**Status:** pending

## STORY-0106

**Epic:** EPIC-028 — Electricity — wired network replaces coverage
**Title:** Cross-border electricity trade via import/export transformers

**As a** planner
**I want** dedicated import/export transformer buildings at the border to move electricity in and out as a wired utility link
**So that** cross-border power trade obeys the same physical-causality rule as every other wired connection instead of a magic border toggle

**Acceptance criteria:**
- AC-1: An import transformer built at the border injects electricity into the local grid only while it is wire-connected to the local network, drawing from the neighbouring jurisdiction with pricing/currency owned by spec/trade.md; a border tile with no local wire hop imports nothing. [SUBSTRATE: ABSENT — greenfield, spec/electricity.md:38-40] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0134`
- AC-2: An export transformer sends power across the border only out of the local network's surplus remaining after domestic brownout-priority demand is met — it never creates power that was not first generated locally. [SUBSTRATE: ABSENT — greenfield, spec/electricity.md:38-40] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0134`

**Sources:**
- `spec/electricity.md:38-40`

**Status:** pending

## STORY-0107

**Epic:** EPIC-028 — Electricity — wired network replaces coverage
**Title:** Idle consumers still draw baseline lighting power

**As a** planner
**I want** a building with no active production to still draw a nonzero idle/lighting electricity load
**So that** pausing production doesn't zero out a building's grid demand

**Acceptance criteria:**
- AC-1: A building with no active recipe running still draws a nonzero idleDraw amount of electricity, distinct from and smaller than its full operating consumption. [SUBSTRATE: ABSENT — greenfield, spec/electricity.md:32, research/production.md §A4 idleDraw] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0135`

**Sources:**
- `spec/electricity.md:32`

**Status:** pending

## STORY-0108

**Epic:** EPIC-028 — Electricity — wired network replaces coverage
**Title:** Grid solver work is budgeted per simulation tick

**As a** planner
**I want** the wire-graph solver to amortise a full network resolve across multiple ticks instead of solving the whole grid within one tick
**So that** a large network never causes a single-tick time spike

**Acceptance criteria:**
- AC-1: Resolving a large wire network's power flow is spread across multiple simulation ticks (budgeted, amortised) rather than completed within a single tick, mirroring CS1's 256-frame amortisation cycle; the per-tick time spent in the solver stays bounded regardless of network size. [SUBSTRATE: ABSENT — greenfield, spec/electricity.md:34-36] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0136`

**Sources:**
- `spec/electricity.md:34-36`

**Status:** pending