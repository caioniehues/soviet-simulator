# Pathfinding

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** transport
**Last verified:** 2026-08-28

| Scope | 1.0 binding |

## What this is

A route authorizes movement only. It translates an already-authorized trip into a lane
sequence over the authoritative road graph. Creating, accepting, or traversing a route does
not transfer custody, satisfy a dwelling need, or settle roubles. Route is a derived state,
not an authority over goods or citizens.

## 1.0 requirement

`SPEC-PATHFINDING-001` — a route is derived from a stated origin, destination, compatible
lane access, and a recorded topology revision.

`SPEC-PATHFINDING-002` — vehicle route cost starts from compatible typed lanes, recorded
topology, and declared lane length and speed, then multiplies free-flow time by Traffic's
damped BPR cost for each lane. Pedestrian routes omit congestion cost. Road price is never
a cost.

`SPEC-PATHFINDING-005` — a route authorizes movement only; it neither transfers custody
nor satisfies a dwelling need.

`SPEC-PATHFINDING-006` — a blocked lane is excluded from new routes. Ambient congestion
changes alone MUST NOT continuously replan an en-route vehicle. Equal-cost choices use a
deterministic tie-breaker.

## Target design

The target is hierarchical routing (PLAUSIBLE, bible §13.21, C2-10): contraction hierarchies
or similar to reduce per-query cost. The current flat A* over the full lane graph runs for
every pathfinding request with no caching.

The link to the routing architecture page: `../../architecture/routing.md` (lead writes).

## Current substrate

`CarPath` in `simulation/src/map/pathfinding.rs:189-268` implements A* over the lane graph
using the `pathfinding` crate. The cost is:

```rust
cost = l.points.length() / l.speed_limit;
cost += common::rand::randu(l.dist_from_bottom.to_bits() ^ base_random);
```

This is free-flow time plus deterministic noise. There is no BPR volume-delay, no Gawron
damping, no congestion signal, no load/capacity field on lanes. The noise provides
deterministic tie-breaking but does not model congestion.

The heuristic is:
```rust
pos.distance(end_pos) * 1.2 / HEURISTIC_SPEED
```

Rail and pedestrian paths use the same A* with different lane access filters.

`MAP-SUB-003`: pathfinding is static and retry-only. Missing topology retries periodically
without terminal failure or a player-visible stalled-route queue.

## Open questions

- Which routing restrictions are necessary for 1.0 before the congestion model is ratified?
- What retry cadence and terminal stalled-state threshold preserve liveness?
- What repeat-run test will establish determinism claims?

## Related

- [Roads](roads.md)
- [Traffic](traffic.md)
- [Vehicles](vehicles.md)
- [Pathfinding spec](../../reference/specifications/pathfinding.md)
- [Routing architecture](../../architecture/routing.md) (lead writes)
