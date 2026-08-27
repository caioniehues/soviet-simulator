---
name: idle-truck-blocks-lane
description: dispatch truck stuck in ToDestination/ToSource for tens of thousands of ticks is usually a PARKED-NOWHERE truck from an EARLIER completed dispatch blocking the lane, not an itinerary/routing bug
metadata:
  type: reference
---

Found on sov-lpj (2026-08-26): a flour-factory hoard test showed a dispatch permanently
stuck in `DispatchState::ToDestination` for 38,000+ ticks. The itinerary/dispatch state
machine (`simulation/src/economy/market.rs` `DispatchState`, `simulation/src/map_dynamic/itinerary.rs`
`Itinerary::has_ended`/`WaitForReroute`) was the natural first suspect and was NOT the cause —
every truck had a live, valid `Route` itinerary, never fell into `WaitForReroute`.

Root cause: `market.rs`'s `DispatchState::Unloading` completion branch (~line 987) leaves the
truck `state: Driving`, `it: Itinerary::NONE`, wherever it physically stopped — the code already
carries a `ponytail:` comment admitting this ("upgrade to RoutingStep::Park... if idle trucks
start blocking traffic"). If that stop point lands in a live lane (e.g. right at a building's
door), the truck sits there forever as a permanent physical obstruction. Every subsequent
dispatch's truck queues nose-to-tail behind it, and the vehicle-vs-vehicle gridlock breaker in
`simulation/src/transportation/road.rs::calc_decision` (the `Panicking` state, triggered when
`front_dist < 1.5` and the blocking vehicle's `flag` round-trips back to `me_u64`) never fires
across 60k+ ticks — no `"gridlock!"` log line at all — because new trucks keep queuing behind
the permanently-idle one and the flag-propagation (one-tick-lagged through
`transport_grid_synchronize`) never converges while the queue keeps growing.

**Diagnostic method that worked:** static reading of the dispatch/itinerary state machine gave
a plausible-looking but wrong theory (WaitForReroute wedge). Instrumenting `calc_decision`/
`calc_front_dist` directly with scratch `eprintln!` probes (position, front_dist, flag,
is_vehicle, group) revealed every "stuck" vehicle was blocked by ANOTHER STATIONARY VEHICLE,
confirming a lane-obstruction chain, not a routing/state-machine defect. Confirmed by mutation:
changing the Unloading-complete branch to `cbuf_vehicle.kill(v)` instead of
`ve.it = Itinerary::NONE` immediately unwedged the dispatch cycle (stock reached the upper
bound, dispatches started re-cycling) in the same test run.

**Why this matters for future debugging here:** a dispatch/truck "stuck" symptom in this repo
has (at least) three known distinct causes now: sov-jcl (Outbound Loading retry, unbounded, no
route — different mechanism), the freight-station/find_external seam (see
[[seeded-freight-station]] if written), and this one (idle-truck-blocks-lane). Don't assume the
first plausible mechanism from reading the state machine; instrument the physics/movement layer
directly (`transportation/road.rs`) when a truck's *position* itself is frozen, since that's a
layer below the dispatch/itinerary abstraction and static reading of the higher layer looks
consistent even when the real blocker is one layer down.
