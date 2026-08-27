---
name: unpark-orphans-collider-if-not-parked
description: unpark() (transportation/vehicle.rs) has no precondition guard — calling it on a vehicle not actually VehicleState::Parked (e.g. mid-RoadToPark) orphans the old TransportGrid collider as a permanent phantom blocker
metadata:
  type: reference
---

Found on sov-7pg (2026-08-26/27), the second wedge in the same flour-factory hoard scenario
after [[idle-truck-blocks-lane]] (sov-2c4) was fixed. Once trucks were routed to a real parking
spot via `map_dynamic::router::park` (a `VehicleState::RoadToPark` spline, only becoming
`VehicleState::Parked` once the spline completes), a NEW wedge appeared: a truck freezes at a
fixed coordinate, `front_dist` computed via `calc_front_dist`'s ray-intersection branch (only
reachable when a neighbor's reported speed is nonzero — a tell that the "blocker" isn't a
genuinely stationary vehicle), and the gridlock-recovery `flag` alternates between two different
real vehicle IDs each retry instead of ever self-matching, so `VehicleState::Panicking` (the
built-in deadlock breaker) never fires.

Root mechanism (read from source, `simulation/src/transportation/vehicle.rs:107-124`): `unpark`
unconditionally does `mem::replace(&mut v.vehicle.state, VehicleState::Driving)` and unconditionally
calls `put_vehicle_in_transport_grid` (a fresh `TransportGrid::insert`), regardless of whether the
previous state was actually `Parked`. If it wasn't — e.g. a dispatcher grabbed the truck for a
new job while it was still mid-`RoadToPark`, animating toward a spot — `unpark` just logs
`"Trying to unpark {:?} that wasn't parked"` and proceeds anyway, silently orphaning whatever
collider handle the vehicle already had in the grid. That stale entry is never removed and lingers
as a permanent phantom collision object.

Fix (sim-implementer, market.rs): guard every `unpark` call site with
`matches!(ve.vehicle.state, VehicleState::Parked(_))` before grabbing a truck from the dispatcher,
and defer `dispatcher.free(...)` until AFTER a `park()` call succeeds (not before), so a truck
can't be re-grabbed mid-parking.

**Known sibling, unconfirmed:** `simulation/src/map_dynamic/router.rs:217`,
`RoutingStep::Unpark(vehicle)` calls `unpark` with NO parked-state guard at all — same
precondition gap, different call path (human-personal-vehicle router, not the dispatch-truck
system). Not verified reachable/reproducible; flag if you're touching that path.

**Methodology note — mutation testing against a two-fix change:** I could NOT reproduce this
specific wedge by mutating either of the sim-implementer's two guards individually or combined
(bypassing the `parked` check, reverting the free-before-park ordering, both together — ~55 runs,
zero reproductions). A sanity-check mutation (fully reverting the Unloading branch to the
pre-[[idle-truck-blocks-lane]] abandon-in-lane behavior) DID reproduce a wedge on the first try,
proving the harness itself was capable of catching one. Conclusion: the specific race window
these two guards close may simply not be hit often (or at all) within a bounded number of test
runs of THIS scenario — don't conclude "the fix doesn't matter" from a mutation that fails to
reproduce; check whether your harness can even detect the failure class at all before trusting a
negative mutation result. Report such a result as PLAUSIBLE (code-corroborated) rather than
CONFIRMED, and say exactly how many mutation attempts you made.

**Also confirmed this session:** genuine flakiness (~2-3 failures in ~65 runs) traced to a
concurrent-edit build race — another agent was actively landing changes to the same file mid
test-run, and `git diff --stat` was visibly growing between my checks. On a settled tree (50+
consecutive runs after edits stopped), zero failures. If a test flakes intermittently while
another agent's file-change notices are firing in your session, check whether the tree was
actually settled during your reproduction runs before treating it as a logic defect.
