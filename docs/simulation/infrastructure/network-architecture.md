# Network architecture

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** infrastructure
**Last verified:** 2026-09-03

| Scope | Post-1.0 hook |

## What this is

The infrastructure networks share structural concepts: typed nodes, typed edges, endpoint
attachments, connected components, topology revision tracking, and compact adjacency
representation (CSR). Each network has different physical state per node and edge — voltage
vs pressure vs temperature vs density — but the topology management code is identical.

"Share topology, scheduling, IDs, journals; do not force water, power, traffic, sewage,
heat and gas through one solver" (design law 20).

## Target design

The design proposes a shared topology kernel (PLAUSIBLE, bible §10.1, §13.22):

```text
Network<N, E>
  nodes:       Vec<NodeData<N>>
  edges:       Vec<EdgeData<E>>
  attachments: Vec<AttachmentData>
  components:  ComponentTracker
  revision:    TopologyRevision
  adjacency:   CSR<NodeIdx, EdgeIdx>
```

Each domain instantiates this with its own node and edge state:
- Electricity: `Network<WireNode, WireEdge>` with generation/load/storage
- Water: `Network<WaterNode, PipeEdge>` with pressure/quality/buffer
- Sewage: `Network<SewageNode, PipeEdge>` with buffer/pump
- Heating: `Network<HeatNode, HeatPipeEdge>` with temperature/flow

Separate solver state per domain. The kernel handles add/remove node, add/remove edge,
component merge/split, revision tracking, and serialization. Each domain owns its own
`allocate()` function that runs per tick on top of the shared topology.

Gas linepack as Post-1.0 research (PLAUSIBLE, D §3.7): gas pipelines store gas under
pressure. The pipeline itself is a buffer. A supply disruption causes a gradual pressure
drop as stored gas is consumed. The linepack integrator is one ODE per segment:
`mass += (flow_in - flow_out) * dt; pressure = f(mass)`. This is compelling for gameplay
but has no charter commitment.

Weather as a shared stressor (HYPOTHESIS, D §4.9): one weather state drives correlated
stress across every network simultaneously. Snow reduces road capacity; cold increases
heating demand; ice risks water-main freeze; peak electricity demand rises in winter.
No weather spec exists (SYNTHESIS §7).

## Current substrate

The only network topology is `ElectricityCache`
(`simulation/src/map/electricity_cache.rs:52-62`): a `BTreeMap` adjacency graph over
`NetworkObjectID` (roads, intersections, buildings) with BFS connectivity, not explicit
wires. No shared topology abstraction exists for other utilities. The only infrastructure
code is that road-derived electricity graph and the binary blackout flow system.

The target kernel link: `../../architecture/network-kernel.md` (lead writes).

## Open questions

- When is gas linepack compelling enough to enter the design scope?
- Which weather observations drive which networks?
- Is a single `Network<N, E>` generic, or do domains need distinct graph representations?

## Related

- [Electricity](electricity.md)
- [Water](water.md)
- [Sewage](sewage.md)
- [Heating](heating.md)
- [Hydrology](hydrology.md)
- [Network kernel architecture](../../architecture/network-kernel.md) (lead writes)
