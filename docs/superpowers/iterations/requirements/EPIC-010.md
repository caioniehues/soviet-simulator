# EPIC-010 — Road network & routing substrate

**Summary:** Road network & routing substrate
**Stories:** STORY-0035, STORY-0036, STORY-0037, STORY-0038, STORY-0039, STORY-0040, STORY-0041
**Primary sources:** `spec/pathfinding.md`, `spec/roads.md`
**Status:** 0/7 done

## STORY-0035

**Epic:** EPIC-010 — Road network & routing substrate
**Title:** Compute turn-aware routes over the unified lane graph

**As a** vehicle or pedestrian agent needing to travel
**I want** a route computed as a chain of lane-hop segments over the single lane graph, honoring intersection turn restrictions
**So that** my trip follows the physically real network with no illegal turns and no separate abstract routing graph to keep in sync

**Acceptance criteria:**
- AC-1: A* pathfinding over the lane graph produces a route as a sequence of lane-hop positions, and never routes through a turn an intersection does not permit. [SUBSTRATE: PROVIDED — map/pathfinding.rs:134,247; map/objects/intersection.rs:289,304] · impact:`local` · seam:`integration` · scenario:`SCENARIO-0030`
- AC-2: Baseline route cost for a lane-hop is length / speed_limit, with no toll or price term for citizen trips. [SUBSTRATE: PROVIDED — map/pathfinding.rs:224-225] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0030`
- AC-3: Each trip requests a fresh path at the moment a destination is assigned; there is no general origin-destination path cache to invalidate or go stale. [SUBSTRATE: PROVIDED — map/pathfinding.rs:134,247] · impact:`local` · seam:`integration` · scenario:`SCENARIO-0030`
- AC-4: An en-route vehicle does not re-plan its route as ambient congestion changes mid-trip; only a blockage/invalidation event or the stall escalation (see 'Escalate a stalled vehicle to re-route, then to a planner-visible bottleneck event') triggers a fresh path request — traffic adapts at trip granularity, not continuously. [SUBSTRATE: UNAUDITED — spec/pathfinding.md:31 (INFERRED)] · impact:`local` · seam:`integration` · scenario:`SCENARIO-0030`
- AC-5: When a road segment referenced by an in-flight or queued route is redrawn by a completed construction/upgrade project, any route referencing that segment is invalidated and the affected agent requests a fresh path rather than continuing to reference the stale segment. [SUBSTRATE: UNAUDITED — spec/roads.md:21] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0030`

**Sources:**
- `spec/roads.md:16-21`
- `spec/pathfinding.md:16-24`

**Status:** pending

## STORY-0036

**Epic:** EPIC-010 — Road network & routing substrate
**Title:** Classify roads by surface class with a physical speed payoff

**As a** planner deciding where to invest in infrastructure
**I want** roads to carry a discrete surface class (dirt / paved / highway, plus tram / pedestrian) distinct from lane purpose
**So that** upgrading a road produces a measurable, physical speed benefit rather than a cosmetic relabel

**Acceptance criteria:**
- AC-1: Roads have no surface-class field today; LaneKind encodes lane purpose (vehicle/pedestrian/parking/etc.), not road surface quality. [SUBSTRATE: ABSENT — map/objects/lane.rs:13] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0033`
- AC-2: Effective vehicle speed on a lane equals vehicle top speed × road-class modifier × terrain modifier; a paved road yields strictly higher effective speed than an otherwise-identical dirt road for the same vehicle. [SUBSTRATE: ABSENT — greenfield] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0033`
- AC-3: Road types are authored as prefabs — a lane template with per-lane speed limit, direction, and a lane type drawn from a fixed taxonomy (Vehicle | Pedestrian | Parking | PublicTransport | Cargo) — and the live network instantiates lanes from that template rather than each lane carrying ad-hoc per-instance data. [SUBSTRATE: PARTIAL — map/objects/lane.rs:13 has LaneKind already; taxonomy alignment UNAUDITED; spec/roads.md:20] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0033`
- AC-4: The dirt-road speed penalty is vehicle-class-specific: a heavy truck's effective speed on dirt is reduced by a strictly larger factor than a light vehicle's on the same dirt segment, replacing CS1's toll-cost term. [SUBSTRATE: ABSENT — greenfield; spec/pathfinding.md:28] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0033`

**Sources:**
- `spec/roads.md:23-27`

**Status:** pending

## STORY-0037

**Epic:** EPIC-010 — Road network & routing substrate
**Title:** Carry routing policy on the trip request, not the graph

**As a** trip requester (delivery truck, pedestrian, transit vehicle)
**I want** each path request to encode its own policy — permitted lane/vehicle-type mask, heavy-vehicle flag, stable-path (no congestion jitter) — independent of any global graph state
**So that** different agent types can be routed correctly over the same shared lane graph without per-agent-type graph variants

**Acceptance criteria:**
- AC-1: A path request can restrict which lane/vehicle types it may use (e.g. a heavy truck is excluded from pedestrian-only lanes) and this restriction is expressed as an input to the request, not as a property baked into the graph. [SUBSTRATE: UNAUDITED] · impact:`local` · seam:`integration` · scenario:`SCENARIO-0043`
- AC-2: Pedestrian route requests are costed by the same time-like formula as vehicles but without the congestion cost term, using only a small random tie-breaker so pedestrians spread across equivalent paths. [SUBSTRATE: UNAUDITED] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0043`
- AC-3: A path request applies fixed hard modifiers by trip type: a car-banned segment costs the base lane-hop cost ×7.5 for a car request rather than being merely discouraged, a transit vehicle's cost on a transit lane is base cost ×0.95, and a closed segment is rejected outright (never selected) rather than merely penalized. [SUBSTRATE: ABSENT — greenfield; spec/pathfinding.md:31] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0043`

**Sources:**
- `spec/pathfinding.md:20-33`

**Status:** pending

## STORY-0038

**Epic:** EPIC-010 — Road network & routing substrate
**Title:** Solve paths off the critical path within a bounded per-tick budget

**As a** simulation running thousands of concurrent agents
**I want** path requests solved by background Burst-compiled A* jobs that agents wait on via a ready-flag, processed within a bounded per-tick job budget
**So that** no single frame stalls waiting for a path solve, and the solver's throughput is a tunable, observable budget rather than unbounded work dumped on one tick

**Acceptance criteria:**
- AC-1: A path request does not block the requesting agent's tick; the agent continues simulating other state while a background A* job resolves and the agent later observes a ready-flag. [SUBSTRATE: UNAUDITED — spec/pathfinding.md:22-23] · impact:`local` · seam:`integration` · scenario:`SCENARIO-0047`
- AC-2: The pathfinding solver processes at most a bounded number of path-solve jobs per tick (a configured per-tick budget), queuing any excess requests to later ticks rather than solving an unbounded number in one frame. [SUBSTRATE: ABSENT — greenfield; spec/pathfinding.md:37 (open question naming CS1's 262144-PathUnit/4-thread anchor, no fixed number adopted)] · impact:`local` · seam:`integration` · scenario:`SCENARIO-0047`

**Sources:**
- `spec/pathfinding.md:22-23`
- `spec/pathfinding.md:37`

**Status:** pending

## STORY-0039

**Epic:** EPIC-010 — Road network & routing substrate
**Title:** Cap intersection nodes at eight attached segments

**As a** road network builder
**I want** a RoadNode to refuse a ninth attached road segment
**So that** junction geometry and lane-merge logic never has to handle an unbounded fan-in

**Acceptance criteria:**
- AC-1: A RoadNode accepts at most 8 attached RoadSegments; construction that would attach a 9th segment to an existing node is rejected. [SUBSTRATE: UNAUDITED — spec/roads.md:18] · impact:`local` · seam:`integration` · scenario:`SCENARIO-0048`

**Sources:**
- `spec/roads.md:18`

**Status:** pending

## STORY-0040

**Epic:** EPIC-010 — Road network & routing substrate
**Title:** Deliver and upgrade roads only through physical construction, never a purchase

**As a** planner expanding the road network
**I want** a road segment to come into existence only when a construction project delivers it, and a dirt→paved upgrade to consume asphalt/gravel and labour rather than money
**So that** the network stays a physical fact — never a toolbar paint tool, never a price the treasury can buy past

**Acceptance criteria:**
- AC-1: No road segment exists in the lane graph until a construction project completes delivering it; there is no instant/toolbar-style road placement, and a segment's capacity is fixed by what was built, not adjustable via a budget or slider. [SUBSTRATE: UNAUDITED — spec/roads.md:12] · impact:`journey` · seam:`integration` · scenario:`SCENARIO-0049`
- AC-2: Upgrading a road segment from dirt to paved is itself a construction project that consumes asphalt/gravel materials and labour; no code path allows a money payment alone to change a road segment's surface class. [SUBSTRATE: ABSENT — greenfield; spec/roads.md:27] · impact:`journey` · seam:`integration` · scenario:`SCENARIO-0049`

**Sources:**
- `spec/roads.md:12`
- `spec/roads.md:27`

**Status:** pending

## STORY-0041

**Epic:** EPIC-010 — Road network & routing substrate
**Title:** Route within a large compound via an authored internal connection graph

**As a** vehicle navigating inside a rail yard or port
**I want** large compounds to author their own internal connection graph that the router can traverse, distinct from the public lane graph
**So that** intra-facility movement (dock to warehouse, yard track to yard track) doesn't require the whole facility to be paved with ordinary public roads

**Acceptance criteria:**
- AC-1: A large compound (rail yard, port) may author an internal connection graph that the pathfinder can route through when a trip both starts and ends inside the compound, without every internal connection being a public RoadSegment. [SUBSTRATE: ABSENT — greenfield; spec/roads.md:25,33] · impact:`local` · seam:`integration` · scenario:`SCENARIO-0051`

**Sources:**
- `spec/roads.md:25`
- `spec/roads.md:33`

**Status:** pending