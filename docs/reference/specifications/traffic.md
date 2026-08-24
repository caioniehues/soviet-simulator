# Traffic specification

**Kind:** specification
**Authority:** binding
**Status:** draft
**Owner:** transport and presentation
**Last verified:** 2026-08-24

The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT, RECOMMENDED, NOT
RECOMMENDED, MAY, and OPTIONAL in this document are to be interpreted as described in RFC 2119
and RFC 8174.

## Purpose

Traffic turns concurrent physical road movement into observable capacity pressure, queues, stalls,
and recovery work for the Planner. It makes congestion a scarcity signal without treating a local
collision rule as the complete congestion model.

## Scope and exclusions

This specification covers road movement pressure and Planner-facing traffic state. It does not add
road pricing, passenger-rail depth, rail signals/electrification, fuel lifecycle, vehicle
manufacture, or any game-over failure mode; each is excluded or deferred by the
[charter](../../plan/charter-1.0.md#explicit-cuts).

## Invariants

- `SPEC-TRAFFIC-001` Every moving vehicle remains a physical identity on a compatible lane until
  it reaches an explicit route transition, parking state, or recoverable stalled state.
- `SPEC-TRAFFIC-002` Capacity pressure, queue state, and stall age are durable authoritative
  records associated with lanes or corridors and affected vehicles/jobs; local collision or
  signal behavior alone is insufficient.
- `SPEC-TRAFFIC-003` A vehicle that cannot progress waits, may reroute through the pathfinding
  contract, and becomes an observable stall when the policy threshold is crossed. It MUST NOT be
  silently despawned, free its obligations, or teleport its cargo/delivery outcome.
- `SPEC-TRAFFIC-004` Traffic must expose pressure and its physical causes to the Planner so a
  bottleneck can be distinguished from unrelated shortage or route failure.
- `SPEC-TRAFFIC-005` Traffic does not satisfy needs, clear domestic requests, or settle roubles.
  Any freight effect is reported to the single logistics fulfillment authority.
- `SPEC-TRAFFIC-006` Traffic only observes blockage at a parking space. Roads alone owns its
  reservation; Vehicles reference it and Logistics requests recovery, but Traffic MUST NOT mutate
  the reservation.
- `SPEC-TRAFFIC-007` Each lane maintains one authoritative exponentially weighted moving-average
  load, updated in constant time from physical vehicle occupancy or passage and declared capacity.
  Traffic derives observed BPR volume-delay cost as `1 + 0.15 * (load / capacity)^4`; zero
  capacity produces blocked state rather than a finite cost.
- `SPEC-TRAFFIC-008` Before BPR cost is published to Pathfinding, Traffic applies Gawron damping:
  `remembered' = 0.3 * observed + 0.7 * remembered`. Pathfinding reads only this damped value;
  blocked state remains a hard route exclusion rather than a high scalar.

## Model and state

Traffic state records lane/corridor identity, observed load or queue, capacity-pressure state,
age, and the vehicle/job identities contributing to a stall. Movement state is separate from
route state and from logistics custody. These dynamic records are keyed to a road, lane, or
corridor supplied by the roads authority; roads owns only topology, static lane/access/speed/
capacity declarations, revision, and parking reservations. A policy may define thresholds for
wait, reroute, and stall, but it cannot make a stalled vehicle vanish or make a route
congestion-aware before that capability is separately implemented and evidenced.

## Failure behavior

Blocked movement becomes a visible queue and then a visible stall, with the preservation of the
vehicle and any linked job. Recovery can follow capacity relief, an authorized alternate route,
or a logistics cancellation/reassignment contract. A jam never destroys stock, clears a need, or
ends the plan; its consequences propagate as late or stalled work rather than disappearance.

## Observability

The Planner can inspect corridor/lane pressure, queue and stall age, affected vehicle/job counts,
the immediate blocking reason, and recovery status. The readout remains durable long enough to
support a planning response, rather than being only a momentary animation.

## Acceptance evidence

All guards below are **UNIMPLEMENTED** and block ratification. A command that executes zero tests
is failure, never green. The current 26-test suite proves no target below.

| Evidence | Future guard command and observable assertion | Negative mutation that must turn it red | Player-facing proof |
|---|---|---|---|
| `EVID-TRAFFIC-001` | `cargo test -p simulation evid_traffic_ema_bpr_gawron_pressure -- --test-threads=1` — occupancy updates durable EMA load, BPR matches known volume/capacity points, and published cost uses 0.3/0.7 Gawron damping. | Use instantaneous occupancy, alter the BPR coefficient/exponent, or publish raw observed cost. | Inspected traffic/bottleneck cost view. |
| `EVID-TRAFFIC-002` | `cargo test -p simulation spec_traffic_stall_preserves_obligation -- --test-threads=1` — thresholded stall retains vehicle and linked job obligation. | Despawn the blocked vehicle or free its job. | Inspected stalled vehicle/job view. |
| `EVID-TRAFFIC-003` | `cargo test -p simulation spec_traffic_recovery_without_despawn -- --test-threads=1` — relieved capacity or alternate route recovers the same stalled state. | Replace recovery with a new vehicle or job. | Inspected stall-to-recovery session. |
| `EVID-TRAFFIC-004` | `cargo test -p simulation evid_traffic_zero_capacity_blocks_lane -- --test-threads=1` — zero capacity or explicit blockage publishes hard blocked state for Pathfinding. | Return a finite route cost for a blocked or zero-capacity lane. | Inspected blocked-lane exclusion capture. |

## Substrate and decisions

`MAP-SUB-004` provides microscopic following, collisions, signals, and a `Panicking` gridlock
state (`simulation/src/transportation/road.rs:15-78,185-250`; `simulation/src/map/traffic_control.rs:38-74`),
but records no durable congestion ledger, queue age, road load/capacity state, or Planner readout.
`MAP-SUB-003` confirms routing is static and retry-only (`simulation/src/map/pathfinding.rs:189-268`;
`simulation/src/map_dynamic/itinerary.rs:171-198`), so this specification does not claim
congestion-aware routing exists. `LOG-SUB-008` and `LOG-SUB-009` further record absent
return/recovery and indefinite dispatch retry in the freight seam
(`simulation/src/economy/market.rs:494-527,562-599`); their recovery requirements belong to
logistics, not traffic alone. See the [substrate](../../research/fact-sheets/wave1-substrate.md#roads-routing-and-traffic)
and [logistics](../../research/fact-sheets/wave1-logistics.md#partial-or-conflicting-contracts)
fact-sheets. External CS1 and Workers & Resources material in archived legacy research is
comparison evidence only, never mechanism authority.

## Deferred behavior

Passenger-rail depth, rail signals/electrification, vehicle fuel lifecycle, and vehicle
manufacture are Post-1.0 and receive no 1.0 mechanism or acceptance evidence here. Road pricing
is prohibited by the binding non-price domestic-clearing model, rather than deferred.

## Open questions

- What authoritative load and capacity measures are cheap enough for the 1.0 performance target?
- Which thresholds distinguish normal wait, reroute, and a Planner-notified stall?
- How should multiple physical blockers be attributed without hiding a shared corridor bottleneck?
