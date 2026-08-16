# Pathfinding

**Status:** draft model (grounded in research)
**Phase:** 1
**Primary inspiration:** CS1 PathFind (adopted as Burst A*); W&R confirms the whole layer is OURS to build
**Evidence:** see [research/roads-traffic.md](../research/roads-traffic.md) for CS1-code and W&R-data sources.

> Background-job A* over the lane graph; routes are short, ref-counted lane-hop chains; congestion is one edge scalar. General origin/destination caching is not assumed.

## Purpose

Every trip — commute, shopping visit, freight delivery, service call — is a real path over the real network. Pathfinding is the hinge between intent and physical motion. The division of labour is clean and proven in CS1: the economic layer ([spec/logistics.md](logistics.md) dispatch) decides *whether and to where* an agent travels; the pathfinder decides *how* (research §B2).

## Draft model

### Route object — the `PathUnit` pattern (**CONFIRMED** source, OURS adoption)

A computed route is a chain of ≤12 lane-hop positions `{segment, lane, offset}` (`PathUnit.cs:22-63`), chained for longer routes, **ref-counted for safe ownership**, and invalidated for free by the segment modification index (`PathUnit.cs:122-124`). Focused tracing found no general origin/destination cache: ordinary requests allocate fresh paths (`research/pathfinding.md` §D2). DOTS mapping: immutable route chunks with explicit sharing only for convoys, transit lines or plan-mandated routes; automatic caching is deferred to benchmarks.

### Request lifecycle (**CONFIRMED** source)

- Paths are requested **per trip**, at the moment a destination is assigned — never continuously. 30+ CS1 AI classes (all vehicles + `CitizenAI` pedestrians) call `PathManager.CreatePath`; the agent waits on a ready-flag while a background worker solves (research §B2).
- CS1 runs 1-4 solver threads with a bucketed priority queue (`BufferItem[65536]` in 1024 priority rows — an O(1)-ish bucketed Dijkstra, `PathFind.cs:130, 367-377`). Our version: Burst-compiled A* jobs over the lane graph (research §G3).
- Request inputs encode the policy: permitted lane/vehicle type masks, heavy-vehicle flag, ignore-blocked, stable-path (disables congestion jitter) — cost policy travels with the request, not the graph (`PathManager.cs:425+`).

### Cost model (**CONFIRMED** source, OURS adaptation)

Core relaxation (`PathFind.cs:962-984`): **cost ≈ `length / (laneSpeed × budget)`** — time-like, so faster lanes are cheaper — plus:

- **Congestion term:** segment length × random factor in `[0.9, (1000 + density×10)/1000]` — a saturated segment costs up to ~2×, and the widening randomness scatters drivers onto alternates (`PathFind.cs:969`). This single per-edge scalar is the *entire* congestion feedback; no global solver ([spec/traffic.md](traffic.md)).
- **Hard modifiers:** car-ban ×7.5, closed segments rejected, transit lanes ×0.95 for transit vehicles. CS1's toll costs are dropped for citizens (no road pricing in a planned economy); class-based penalties (dirt road slows heavy trucks) replace them ([spec/roads.md](roads.md)).
- Pedestrian lanes: same time-cost without congestion jitter, small random tie-breaker so walkers spread (`PathFind.cs:670, 822`).
- En-route vehicles do **not** re-plan as congestion builds (INFERRED, research §B5) — re-plan only on blockage/invalidation or the stall escalation in [spec/traffic.md](traffic.md). Traffic adapts at trip granularity.

### W&R contrast (**CONFIRMED-absence**)

No cost tokens, no graph, no routing data anywhere in W&R's `.ini` corpus — the entire pathfinding layer is native (research §E). The one authored graph is the internal road layout of two large compound prefabs (`$ROAD_CON_PID`/`$ROAD_CON_DETOUR`, research §D4) — adopted for our large compounds (research §G8). Everything else here is OURS to build, with CS1 as the proven mechanism.

### Levels, revised

The CS2-style three-level framing collapses to two in practice:
- **A. Mode/strategic choice** (walk vs transit vs assigned vehicle) — an input mask on the request, as CS1 does, plus OURS mode-choice policy from [spec/needs.md](needs.md) mobility.
- **B+C merged. Lane-hop path** — the lane graph *is* the trajectory substrate; lane-level execution reads the same `PathPosition` chain (CS1 proves no third layer is needed — research §A3).

## Open questions

- ~~Does CS1 ref-counting prove a general route cache?~~ No: it proves ownership sharing only; ordinary requests allocate fresh paths (`research/pathfinding.md` §D2). V1 has no automatic OD cache; convoys may deliberately share.
- ~~Recompute policy?~~ Settled: per-trip + invalidation-driven (modification index) + stall-triggered re-route; never continuous.
- Path-request throughput at 100k citizens: CS1 caps at 262144 pooled PathUnits and 4 threads — what's our Burst-job budget per tick? Feeds the citizen-scale prototype ([spec/citizens.md](citizens.md)).
- Admissible heuristic details: CS1's exact heuristic wasn't fully traced (research Gaps) — pick our own (straight-line/maxspeed).
- Hierarchical shortcuts (highway-level abstraction) if flat lane-graph A* proves too slow on large maps — SPECULATIVE, only if the prototype says so.

## Evidence log

| Claim | Evidence level | Source | Notes |
|---|---|---|---|
| Route = ≤12 lane-hop `PathUnit`, chained and ref-counted | CONFIRMED | `PathUnit.cs:22-67, 235-254`, `PathManager.cs:189-208` | research §B1; general cache not confirmed |
| Paths requested per-trip by 30+ AI classes; async workers | CONFIRMED | `PathManager.cs:211-216`, AI grep | research §B2, §B4 |
| Cost = `length/(speed×budget)` + congestion multiplier (≤~2×) + bans | CONFIRMED | `PathFind.cs:962-990` | research §B5 |
| No continuous re-planning en route | INFERRED | re-route only on explicit events | research §B5 |
| W&R pathfinding entirely native; zero data representation | CONFIRMED (absence) | corpus token sweep | research §D-E |
| Burst A*, explicit convoy sharing, no toll term, benchmark-gated strategic hierarchy | OURS | `research/pathfinding.md` §G | architecture refines this model |

Evidence levels: CONFIRMED · OBSERVED · INFERRED · SPECULATIVE · OURS (see [spec/README](README.md)).

## Related

- [spec/roads.md](roads.md) — the lane graph being searched
- [spec/traffic.md](traffic.md) — the density scalar and stall-driven re-route
- [spec/logistics.md](logistics.md) — dispatch decides destinations; pathfinding decides routes
- Research: [research/roads-traffic.md](../research/roads-traffic.md) · [research/pathfinding.md](../research/pathfinding.md)
