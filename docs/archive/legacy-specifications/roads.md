# Roads

**Status:** draft model (grounded in research)
**Phase:** 1
**Primary inspiration:** CS1 network engine (adopted) + W&R road-class model (adopted); CS2 two-layer concept dropped
**Evidence:** see [research/roads-traffic.md](../research/roads-traffic.md) for CS1-code and W&R-data sources.

> One graph: the lane network is both the geometry and the routing graph.

## Purpose

The physical substrate everything moves on. Roads are **built physical objects** — they exist only after a construction project delivers them ([spec/construction.md](construction.md)), and their capacity is a physical fact, not a budget slider. Finite throughput is the binding constraint on the plan: a single dirt road to a remote mine caps ore flow no matter how many trucks the plan assigns (research §G7).

## Draft model

### Network representation — CS1's three levels, in ECS (**CONFIRMED** source, OURS adoption)

- `RoadNode` (intersections/endpoints, ≤8 attached segments), `RoadSegment` (edge between two nodes, flags, average length, modification index), per-segment `Lane` buffer (type, direction, speed, curve). Mirrors CS1's `NetNode`/`NetSegment`/`NetLane` struct pools addressed by index (`NetSegment.cs:103-105`, `NetLane.cs:45`).
- **No separate abstract routing graph.** CS1 confirms the lane bezier network *is* the routing graph — pathfinding walks lanes/nodes directly, branching at junctions via per-lane lane-node lists (research §A3). The original two-layer (geometry vs simulation graph) framing in this stub is dropped.
- Road *types* are authored prefabs (CS1 `NetInfo` pattern): a lane template with per-lane `m_speedLimit`, type (`Vehicle | Pedestrian | Parking | PublicTransport | Cargo…`) and direction; the live network instantiates from it (`NetInfo.cs:75-108`).
- A segment's `modificationIndex` invalidates cached routes when the road is redrawn (`PathUnit.cs:122-124`) — no central subscription needed.

### Road class — W&R's discrete enum (**CONFIRMED** source, OURS adoption)

W&R data declares only connection points and a road *class* (road / mudroad / tramroad / pedestrian); no speed, width, lane or capacity tokens exist anywhere in the corpus (**CONFIRMED-absence**, research §D1-D2). Speed lives on the vehicle (`$MOVEMENT_SPEED`, 1053× vehicle-side only, research §D3).

Our synthesis: a small set of buildable road classes (dirt → paved → highway, plus tram/pedestrian), each a prefab carrying CS1-style lane/speed params. **Effective speed = vehicle top speed × road-class modifier × terrain** (research §G5-G6). Upgrading dirt → paved is a construction project consuming asphalt/gravel and labour — never money.

### Roads as physical assets (OURS)

- A road segment comes to exist via [spec/construction.md](construction.md): groundworks + materials + machines.
- Per-corridor throughput/utilisation is a first-class planning readout (the traffic-density signal exposed as an economic bottleneck metric — research §G7). You cannot buy past a saturated road; you build another one.
- Intra-facility routing: large compounds (rail yards, ports) may author an internal connection graph, as W&R does for exactly two large prefabs via `$ROAD_CON_PID`/`$ROAD_CON_DETOUR` (research §D4, §G8).

## Open questions

- ~~Do we need the two-layer geometry/simulation-graph split from day one?~~ Settled by research §A3: no — one unified lane graph, CS1-proven.
- Road wear/condition as a physical maintenance sink — no precedent in either lab (W&R road params are entirely native); fully OURS. In scope?
- Store topology as blob assets vs entity graph for Burst traversal — Phase 2 decision.
- Which road classes ship at v1 (dirt/paved minimum; tram-carrying road needed early for the Soviet feel?).

## Evidence log

| Claim | Evidence level | Source | Notes |
|---|---|---|---|
| CS1 network = `NetNode`/`NetSegment`/`NetLane` struct pools from `NetInfo` prefabs | CONFIRMED | `NetSegment.cs:103-105`, `NetInfo.cs:75-108`, `NetManager.cs:3623-3626` | research §A1-A2 |
| CS1 has one graph — lanes are the routing graph | INFERRED (from code reading) | `PathFind.cs:427-455`, `NetLane.cs:50` | research §A3 |
| Modification index invalidates cached paths | CONFIRMED | `PathUnit.cs:122-124`, `NetSegment.cs:96-97` | research §A4 |
| W&R data has no road speed/width/lane/capacity tokens; class only | CONFIRMED (absence) | corpus token sweep | research §D2, §E |
| W&R speed is vehicle-side (`$MOVEMENT_SPEED`) | CONFIRMED | 1053× vehicle files, 0× road files | research §D3 |
| Road classes + physical upgrade path; throughput as plan constraint | OURS | research §G5-G7 | this spec's model |

Evidence levels: CONFIRMED · OBSERVED · INFERRED · SPECULATIVE · OURS (see [spec/README](README.md)).

## Related

- [spec/traffic.md](traffic.md) — behaviour on the network
- [spec/pathfinding.md](pathfinding.md) — routing over the network
- [spec/construction.md](construction.md) — how roads come to exist
- Research: [research/roads-traffic.md](../research/roads-traffic.md)
