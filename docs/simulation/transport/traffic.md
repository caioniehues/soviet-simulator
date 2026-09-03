# Traffic

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** transport
**Last verified:** 2026-08-28

| Scope | 1.0 binding |

## What this is

Traffic turns concurrent physical road movement into observable capacity pressure, queues,
and stalls. A congestion signal visible to the Planner lets the Planner distinguish a
transport bottleneck from an unrelated input shortage.

Traffic does not satisfy needs, clear domestic requests, or settle roubles. It observes
road state. It reports freight delays to the Logistics authority. That separation is absolute.

## 1.0 requirement

`SPEC-TRAFFIC-007` — each lane maintains one authoritative EWMA load, updated in constant
time. Traffic derives observed BPR volume-delay cost as
`1 + 0.15 * (load / capacity)^4`. Zero capacity produces blocked state.

`SPEC-TRAFFIC-008` — before BPR cost is published to Pathfinding, Traffic applies Gawron
damping: `remembered' = 0.3 * observed + 0.7 * remembered`. Pathfinding reads only this
damped value.

`SPEC-TRAFFIC-001` — every moving vehicle remains a physical identity on a compatible lane.

`SPEC-TRAFFIC-003` — a vehicle that cannot progress waits, may reroute, and becomes an
observable stall. It MUST NOT be silently despawned.

## Target design

Queue storage and spillback (PLAUSIBLE, D §3.8): the CTM (Cell Transmission Model, Daganzo
1994) or LTM (Link Transmission Model, Yperman 2005-2007) provides meso-scale queue
propagation without microsimulating every link. When a downstream link jams, upstream links
back up. This creates visible congestion waves and strategic planning: the Planner must build
bypass roads before a bottleneck cascades.

CTM/LTM is a major new system. Nothing in the current code prepares for it.

Industrial gates (PLAUSIBLE, D-10): factory traffic spills onto public roads. Shift waves
(PLAUSIBLE, D-11): factory shift changes create passenger traffic peaks. Shift staggering is
a planning mechanic the Planner can use.

## Current substrate

`MAP-SUB-004`: traffic is purely microscopic. The `calc_decision` function in
`simulation/src/transportation/road.rs:186-407` computes a geometric cone-based avoidance
check — braking distance, spatial-grid neighbour query, ray intersection. This is not IDM
(Intelligent Driver Model). There is no continuous acceleration response, only a binary
stop/go from distance thresholds.

The gridlock detector (`road.rs:217-225`): when `speed < 0.2` and `front_dist < 1.5`,
the vehicle enters `Panicking` state and waits up to 200 seconds with a randomized wait time.
The `flag` field propagates a gridlock token through following vehicles. Recovery is a random
perturbation, not a resolution.

No durable congestion ledger, queue age, road load/capacity state, or Planner-facing traffic
readout exists. BPR and Gawron are prescribed by the draft specs but are entirely
unimplemented.

## Research basis

**BPR** (Bureau of Public Roads, 1964): `t = t_free * (1 + 0.15 * (v/c)^4)`. Standard
volume-delay function. CONFIRMED as an appropriate game model.

**Gawron** (Christian Gawron, 1998): iterative route choice that blends observed travel times
with remembered travel times. The 0.3/0.7 blend is the SUMO DUA standard. CONFIRMED.

**CTM** (Daganzo, 1994): partitions roads into cells, tracks density per cell, uses
supply/demand functions for flow between cells. Captures shockwaves and spillback. CONFIRMED
as a standard model for meso traffic.

**LTM** (Yperman, 2005-2007): efficiency improvement over CTM using cumulative vehicle counts
at link boundaries. More accurate for long links. CONFIRMED.

## Open questions

- What authoritative load and capacity measures are cheap enough for the 1.0 performance
  target?
- Is BPR/Gawron on the existing micro model the first target, or does the meso CTM layer
  come with it?
- Which thresholds distinguish normal wait, reroute, and a Planner-notified stall?

## Related

- [Roads](roads.md)
- [Pathfinding](pathfinding.md)
- [Vehicles](vehicles.md)
- [Traffic spec](../../reference/specifications/traffic.md)
- [Queues](../concepts/queues.md)
