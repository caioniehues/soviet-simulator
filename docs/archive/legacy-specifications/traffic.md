# Traffic Behaviour

**Status:** draft model (grounded in research)
**Phase:** 1
**Primary inspiration:** CS1 emergent congestion (adopted, minus despawn) — verified: no global solver
**Evidence:** see [research/roads-traffic.md](../research/roads-traffic.md) for CS1-code and W&R-data sources.

> Congestion is emergent from local vehicle rules; gridlock is a planning signal, never a garbage-collection event.

## Purpose

Traffic is the emergent sum of individual agents moving for real reasons. Congestion must feed back into the economy: late deliveries stall production ([spec/logistics.md](logistics.md)), long commutes erode wellbeing ([spec/needs.md](needs.md)). In a planned economy, a jam is information — a bottleneck the plan must answer with infrastructure, not a nuisance to despawn away.

## Draft model

### Emergence, verified (**CONFIRMED** — no global traffic solver in CS1)

The claim under test was confirmed (research §C5). CS1's whole congestion machine is three local mechanisms, and we adopt all three:

1. **Vehicles stamp load onto their segment.** Each frame, every vehicle calls `AddTraffic` on the segment it occupies — a saturating add into a per-segment buffer (`CarAI.cs:367`, `NetSegment.cs:1905-1909`).
2. **Density = buffer ÷ lane capacity, low-pass filtered.** `RoadBaseAI.SimulationStep` converts the buffer into a 0-100 `m_trafficDensity` byte, moved ±5 per step toward target; buffer overflow hard-flags the segment `Blocked` (`RoadBaseAI.cs:1400-1440`). The same pattern is reused across rails, runways, docks — one uniform load mechanism per network kind.
3. **Jams are lane space reservation.** A car reserves braking-distance space ahead (`½v²/a + half-length`, `CarAI.cs:373-395` → `NetLane.ReserveSpace`); a follower reading a full lane brakes toward 0. No queue object, no scheduler — a jam is many cars independently refusing to enter reserved space (research §C3).

Routing reads only the per-segment density byte as a cost multiplier ([spec/pathfinding.md](pathfinding.md)). Nothing computes network-wide flow or equilibrium. W&R declares nothing about traffic in data but natively detects jams (`WARNING_MESSAGE_TRAFFIC_JAM` in `config_default.ini` — research §D5), so both labs agree: emergent, no solver.

### The one rejection: despawn → wait / re-route / stall (OURS)

CS1 deletes vehicles blocked 100-150 consecutive frames (`CarAI.cs:87-96`) — its escape valve and most-criticised behaviour. Rejected: a coal truck that "gives up" is a plan failure silently erased. Instead, a vehicle blocked past a threshold (research §G4):

1. **Waits** (jams persist physically, as in W&R),
2. **Re-routes** if an alternative exists,
3. **Registers a logistics stall** — surfaced to the planner as a bottleneck event, feeding the corridor-utilisation readout ([spec/roads.md](roads.md)) and delivery-delay consequences ([spec/logistics.md](logistics.md)).

This is the road-layer analogue of the housing queue: shortage made visible, not deleted.

### Behavioural route cost

Time dominates: cost is `length/(speed × budget)` with the congestion multiplier (detail in [spec/pathfinding.md](pathfinding.md)). CS1's toll/ticket terms have no place for citizens in a planned economy; comfort/reliability terms (CS2-inspired) are OURS extensions, unproven in either lab.

## Open questions

- ~~Lane-level simulation vs segment flow aggregate?~~ Settled direction: CS1's lane-reservation model is local, cheap, and Burst-friendly; adopt it. Whether the reservation byte per lane suffices at 100k citizens still needs the prototype ([spec/citizens.md](citizens.md) scale question).
- Parking: neither lab models it as pressure (W&R declares only parking *spots*). CS2-style parking search is OURS if wanted — in scope?
- Stall threshold tuning: how long does a truck wait before re-route vs stall-report? Per-cargo urgency?
- Do stalled vehicles block the plan's vehicle pool (they should — they're physical assets, [spec/vehicles.md](vehicles.md))?

## Evidence log

| Claim | Evidence level | Source | Notes |
|---|---|---|---|
| No global traffic solver in CS1 | CONFIRMED | §C1-C5 synthesis; only per-segment density byte | research §C5 |
| Per-vehicle `AddTraffic` → smoothed 0-100 density | CONFIRMED | `CarAI.cs:367`, `NetSegment.cs:1905-1909`, `RoadBaseAI.cs:1400-1440` | research §C1-C2 |
| Car-following = braking-distance lane reservation | CONFIRMED | `CarAI.cs:373-395`, `NetLane.cs:604-701`, `VehicleAI.cs:489-490` | research §C3 |
| CS1 despawns stuck vehicles at 100/150 blocked frames | CONFIRMED | `CarAI.cs:87-96` | research §C4; rejected |
| W&R traffic model entirely native; jams detected natively | CONFIRMED (absence) | corpus sweep + `config_default.ini` | research §D5, §E |
| Wait/re-route/stall replaces despawn; jams as planning signal | OURS | research §G4, §G7 | this spec's model |

Evidence levels: CONFIRMED · OBSERVED · INFERRED · SPECULATIVE · OURS (see [spec/README](README.md)).

## Related

- [spec/roads.md](roads.md) — the network and its throughput readout
- [spec/pathfinding.md](pathfinding.md) — how density feeds route cost
- [spec/logistics.md](logistics.md) / [spec/vehicles.md](vehicles.md) — who is on the road and what a stall means
- Research: [research/roads-traffic.md](../research/roads-traffic.md)
