# Public transport (future)

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** transport
**Last verified:** 2026-08-28

| Scope | Post-1.0 |

This page is Post-1.0 direction. It has no "1.0 requirement" section.

## What this is

Public transit — buses, trams, trolleybuses — is the dominant daily transport mode in a
Soviet city. The charter cuts passenger rail, signals, and electrification to Post-1.0 but
does not cut road-based public transit explicitly. The binding scope names
"public transit dominant, private cars emergent" as a design principle.

## Target design

The design proposes (PLAUSIBLE, D-12; bible §9.12):

- Boarding throughput: passengers enter and exit at a finite rate per door per stop
- Dwell time: the bus waits while passengers board and alight
- Crowding: a full bus turns away additional passengers
- Headway: buses on a route maintain a spacing; the Planner controls frequency
- Bunching: buses that fall behind pick up more passengers, slow further, and
  bunch with the bus ahead — a real planning challenge
- Power coupling: trolleybuses depend on the electricity network

Passenger rail with capacity, dwell, and headway is a separate Post-1.0 item.

## Current substrate

`VehicleKind::Bus` exists (`simulation/src/transportation/vehicle.rs:30`) but has no
passengers, no boarding, no route, no schedule, no capacity, and no dwell time. A bus is
a wider, slower vehicle with no distinct behaviour. `VehicleKind::Bus` provides:
- `width()`: 9.0
- `acceleration()`: 2.0 m/s²
- `speed_factor()`: 0.8

No transit route, stop, schedule, or passenger system exists anywhere in the codebase.

## Open questions

- Is road-based public transit (buses without electrification) in 1.0 scope, or is it
  deferred alongside passenger rail?
- How does boarding throughput interact with traffic congestion at bus stops?
- Does trolleybus electrification couple to the 1.0 electricity model or the Post-1.0
  electrification?

## Related

- [Vehicles](vehicles.md)
- [Freight rail](freight-rail.md)
- [Traffic](traffic.md)
- [Electricity](../infrastructure/electricity.md)
