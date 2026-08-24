# Vehicles specification

**Kind:** specification
**Authority:** binding
**Status:** draft
**Owner:** logistics
**Last verified:** 2026-08-24

The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT, RECOMMENDED, NOT
RECOMMENDED, MAY, and OPTIONAL in this document are to be interpreted as described in RFC 2119 and
RFC 8174.

## Purpose

Vehicles are finite persistent identities used by logistics; their availability constrains service
rather than creating or deleting transport capacity on demand.

## Scope and exclusions

1.0 includes minimal freight rail: three buildings, one locomotive type, and one wagon type. It
does not include passenger rail, rail signals, rail electrification, ships, docks, pipelines, cableways,
containers, aircraft, vehicle manufacture, or vehicle fuel lifecycle. Water remains a utility,
never cargo. Archived reference-game vehicle systems are comparison evidence only.

## Invariants

- `SPEC-VEHICLES-001` — Each operational vehicle has a stable identity and an observable state:
  available, reserved, travelling, loading, unloading, recovering, or unavailable.
- `SPEC-VEHICLES-002` — Vehicle reservation is separate from stock allocation and custody. A
  vehicle cannot make a haul physically complete merely by being reserved or routed.
- `SPEC-VEHICLES-003` — A vehicle's disappearance or route failure preserves the haul and its
  accountable stock, records a recovery reason, and frees or repairs the reservation according to
  the logistics policy.
- `SPEC-VEHICLES-004` — The 1.0 freight-rail consist is limited to the charter's locomotive and
  wagon types. It uses the same no-teleport custody rules as road freight.
- `SPEC-VEHICLES-005` — Each freight vehicle and wagon SHALL have finite compatible cargo
  capacity, an accountable owner/depot and physical parking/recovery location, and an observable
  load. Reservation or routing cannot exceed capacity or substitute for custody.
- `SPEC-VEHICLES-006` — Roads solely owns parking-slot reservation. A Vehicle stores only a slot
  reference and recovery acknowledgement; it MUST NOT create, release, or instantaneously occupy a
  parking slot.

## Model and state

Vehicle identity, class, location, state, reservation, compatible capacity, owner/depot, recovery
state, active-haul/load reference, and Roads-owned parking-slot reference are authoritative fleet
state. Roads alone owns parking reservation; Logistics requests recovery and waits for the Roads
acknowledgement. Logistics owns the referenced cargo identity, quantity, and custody; Vehicles does
not copy that ledger. The latter fleet fields are required target mechanics, not current-substrate assertions.

## Failure behavior

Finite availability turns excess work into a visible queue. A failed or missing vehicle leaves its
haul queued or explicitly cancelled with custody returned to an accountable holder; it never
silently destroys goods, demand, or the fleet identity.

## Observability

The Planner can inspect vehicle identity, class, location, reservation, active haul, state, load,
capacity, compatibility, owner/depot, parking, and recovery reason. Current presentation lacks
several of these target fields; that gap is identified below rather than hidden.

## Acceptance evidence

All listed guards are **UNIMPLEMENTED** and block ratification. A command that executes zero tests
is failure, never green. The current 26-test suite proves no target below.

| Evidence | Command | Observable assertion | Required red mutation | Player-facing proof |
|---|---|---|---|---|
| `EVID-VEHICLES-001` | `cargo test -p simulation evid_vehicles_exclusive_capacity_parking_recovery -- --test-threads=1` | Reservation is exclusive, load cannot exceed compatible finite capacity, and recovery waits for a Roads-owned physical parking acknowledgement. | Assign one vehicle twice, accept over-capacity load, or mark parked without Roads acknowledgement. | Inspected fleet queue capture. |
| `EVID-VEHICLES-002` | `cargo test -p simulation evid_vehicles_rail_pickup_delivery_custody -- --test-threads=1` | Locomotive/wagon freight changes custody only at pickup and delivery. | Credit destination before wagon delivery. | Inspected rail-haul session. |

## Substrate and decisions

Current dispatch reserves nearest available SmallTruck or FreightTrain identities
([`LOG-SUB-001`](../../research/fact-sheets/wave1-logistics.md#log-sub-001--two-dispatch-classes-use-real-networks));
companies spawn a finite number of parked truck identities and retain their IDs
([`LOG-SUB-004`](../../research/fact-sheets/wave1-logistics.md#log-sub-004--companies-spawn-a-finite-number-of-parked-trucks)).
The global dispatcher ignores that ownership ([`LOG-SUB-006`](../../research/fact-sheets/wave1-logistics.md#log-sub-006--company-ownership-does-not-constrain-global-dispatch)). `Vehicle` has no authoritative
cargo, capacity, owner, depot, fuel, wear, driver, or recovery field
([`LOG-SUB-005`](../../research/fact-sheets/wave1-logistics.md#log-sub-005--cargo-is-not-embodied-by-the-vehicle)), so this
document makes none of those a provided claim.

## Deferred behavior

Vehicle manufacture, fuel lifecycle, wear, repair, condition, passenger operations, and advanced
rail control are Post-1.0 or otherwise excluded by the charter.

## Open questions

- Which owner/depot sharing policy permits a freight vehicle to serve a haul while retaining its
  required accountable owner/depot assignment?
- What wagon capacity and compatibility data is required by the 1.0 freight model?
