# EPIC-035 — Vehicle asset lifecycle

**Summary:** Vehicle asset lifecycle
**Stories:** STORY-0139, STORY-0140, STORY-0141, STORY-0142, STORY-0143, STORY-0144
**Primary sources:** `spec/vehicles.md`
**Status:** 0/6 done

## STORY-0139

**Epic:** EPIC-035 — Vehicle asset lifecycle
**Title:** Model the vehicle as an owned physical asset

**As a** planner
**I want** every vehicle to be a persistent object with fuel, wear, capacity, cargo class, owner/depot and driver
**So that** the fleet is a finite, physical thing I must build and maintain rather than magic capacity

**Acceptance criteria:**
- AC-1: (POST-1.0 AC — excluded from 1.0 per charter:106 "vehicle lifecycle including fuel-as-commodity" — the fuel field and its empty-tank halt; vehicle-as-owned-asset remains in 1.0 via the other ACs) A Vehicle entity carries fuel level and fuel type fields, and an empty tank halts its movement/dispatch eligibility. [SUBSTRATE: ABSENT — transportation/vehicle.rs:34-44] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0149`
- AC-2: A Vehicle entity carries a wear/condition value that increases with distance travelled and load, exposed for later repair/scrap logic. [SUBSTRATE: ABSENT — transportation/vehicle.rs:34-44] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0149`
- AC-3: A Vehicle entity carries a cargoClass and capacity field so that only compatible resource classes up to that capacity can be loaded onto it. [SUBSTRATE: ABSENT — transportation/vehicle.rs:34-44] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0149`
- AC-4: Existing kinematic behaviour (position, speed, steering, `VehicleState::Panicking` gridlock handling) is unchanged by the new asset fields — a regression test on vehicle movement passes before and after the schema change. [SUBSTRATE: PROVIDED — transportation/vehicle.rs:34-44, vehicle.rs:19-20] · impact:`local` · seam:`integration` · scenario:`SCENARIO-0149`
- AC-5: A Vehicle entity carries speed, power and empty-weight fields (movement physics), and its capacity field is denominated in tonnes for freight cargo classes or persons for the passenger cargo class. [SUBSTRATE: ABSENT — transportation/vehicle.rs:34-44 has no power/weight fields or a capacity-unit distinction] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0149`

**Sources:**
- `spec/vehicles.md:1-33`

**Status:** pending

## STORY-0140

**Epic:** EPIC-035 — Vehicle asset lifecycle
**Title:** Reject unbounded transient vehicle spawning

**As a** planner
**I want** the fleet to be a fixed, finite set of vehicles that park (persist) rather than despawn between trips
**So that** logistics throughput is bounded by real machines I own, not spawned and deleted on demand

**Acceptance criteria:**
- AC-1: When no idle compatible vehicle exists for a dispatch, the delivery request waits (stays queued) instead of a new vehicle being spawned to fulfill it. [SUBSTRATE: ABSENT — greenfield; current dispatch has no vehicle-availability gate to test against] · impact:`journey` · seam:`integration` · scenario:`SCENARIO-0142`
- AC-2: A vehicle that completes a trip returns to its depot/parking slot and persists as an entity (its ID and asset fields remain queryable), rather than being deleted. [SUBSTRATE: PROVIDED (partially reusable) — map_dynamic/parking.rs:42,58 gives physical reserved parking slots; vehicle deletion-on-arrival is not present in current code so nothing to break here, but no depot-return behaviour exists yet] · impact:`local` · seam:`integration` · scenario:`SCENARIO-0142`
- AC-3: A vehicle stuck in gridlock is never force-deleted to clear congestion; it remains parked/panicking and eventually resumes, matching the existing 200s Panicking-then-resume behaviour. [SUBSTRATE: PROVIDED — transportation/vehicle.rs VehicleState::Panicking, vehicle.rs:19-20] · impact:`local` · seam:`integration` · scenario:`SCENARIO-0142`

**Sources:**
- `spec/vehicles.md:48-64,95-97`

**Status:** pending

## STORY-0141

**Epic:** EPIC-035 — Vehicle asset lifecycle
**Title:** Bound depot capacity by physical parking slots

**As a** planner
**I want** a depot's vehicle capacity to equal the number of physical parking slots it was built with
**So that** fleet size is a construction decision, not an abstract slider

**Acceptance criteria:**
- AC-1: A depot cannot own more vehicles than it has reserved parking slots for; assigning a vehicle to a full depot fails or queues rather than silently over-allocating. [SUBSTRATE: PARTIAL — map_dynamic/parking.rs:42,58 provides physical reserved-slot parking, but no depot-to-fleet-size linkage exists] · impact:`local` · seam:`integration` · scenario:`SCENARIO-0150`
- AC-2: A depot/office is a generic hauler that stores no cargo of its own (only its own fuel tank); it owns and dispatches vehicles that shuttle between a separately-designated export bucket and import bucket, and never itself accumulates the resource being moved. [SUBSTRATE: ABSENT — greenfield; map_dynamic/parking.rs provides slots but no depot-as-hauler-not-warehouse role exists, spec/logistics.md:53] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0150`

**Sources:**
- `spec/vehicles.md:22-24,64`

**Status:** pending

## STORY-0142

**Epic:** EPIC-035 — Vehicle asset lifecycle
**Title:** Manufacture vehicles as a real production chain

**Deferred:** true
**Deferred reason:** charter:108 "vehicle manufacture"

**As a** planner
**I want** new vehicles to be either imported for hard currency or manufactured domestically from steel/electronics/etc via a vehicle factory recipe
**So that** the fleet is finite and its growth costs real industrial input, closing the needs -> vehicle industry -> steel/electronics cascade

**Acceptance criteria:**
- AC-1: A vehicle factory recipe consumes steel/plastics/components/electronics inputs and produces a `vehicles` cargo item that must be physically delivered to a depot before it becomes a usable, dispatchable Vehicle entity. [SUBSTRATE: ABSENT — greenfield; recipe machinery exists generically (prototypes/src/types/recipe.rs:35-47) but no vehicle-producing recipe or vehicles-cargo-to-Vehicle-entity conversion exists] · impact:`journey` · seam:`integration`
- AC-2: Importing a vehicle debits Government.money at a price derived from the domestic bill of materials (not a hand-typed constant), and yields the same usable Vehicle entity as domestic manufacture. [SUBSTRATE: ABSENT — greenfield; Government.money exists (governance.rs:10) but no vehicle price-from-BOM or import path exists] · impact:`cross-surface` · seam:`integration`
- AC-3: A vehicle type's import price is denominated per its origin bloc (ruble-priced Eastern-bloc vehicles vs dollar-priced Western-bloc vehicles); for 1.0 all vehicle purchases resolve through the single rouble currency and the dollar/Western-bloc pricing mechanism is captured as data on the vehicle type but its settlement is deferred to a later milestone. [SUBSTRATE: OURS/DEFERRED — greenfield; single-rouble-for-1.0 decision (2026-08-22) defers the USD-bloc settlement half, spec/vehicles.md:38,47] · impact:`cross-surface` · seam:`integration`

**Sources:**
- `spec/vehicles.md:40-58`

**Status:** pending

## STORY-0143

**Epic:** EPIC-035 — Vehicle asset lifecycle
**Title:** Split rail vehicles into locomotive and wagon consists

**As a** planner
**I want** rail vehicles to be represented as two coupled asset types — a locomotive providing propulsion with zero cargo capacity, and a wagon providing cargo capacity with no propulsion — purchased and dispatched together as a consist
**So that** rail freight matches the confirmed W&R rule instead of forcing trains into the flat one-vehicle model built for trucks

**Acceptance criteria:**
- AC-1: A rail Locomotive has power and speed fields and a capacity of zero; cargo cannot be assigned to a Locomotive directly. [SUBSTRATE: ABSENT — greenfield; transportation/vehicle.rs has one flat Vehicle schema with no locomotive/wagon distinction, spec/vehicles.md:37] · impact:`local` · seam:`unit`
- AC-2: A rail Wagon has a capacity/cargoClass field and no power field; a Wagon cannot move or be dispatched unless coupled to a Locomotive in the same consist. [SUBSTRATE: ABSENT — greenfield, spec/vehicles.md:37] · impact:`local` · seam:`unit`
- AC-3: A rail consist (one Locomotive plus one or more Wagons) is purchased and assigned as a single bundle; individual Wagons are not independently tradeable or dispatchable outside a consist. [SUBSTRATE: ABSENT — greenfield, spec/vehicles.md:37] · impact:`local` · seam:`integration`

**Sources:**
- `spec/vehicles.md:37`

**Status:** pending

## STORY-0144

**Epic:** EPIC-035 — Vehicle asset lifecycle
**Title:** Scrap a vehicle at end of life into recoverable materials

**Deferred:** true
**Deferred reason:** charter:106 "vehicle lifecycle including fuel-as-commodity"

**As a** planner
**I want** a vehicle that reaches zero condition or exceeds its lifespan to be scrapped into waste_steel and waste_aluminium cargo, closing the vehicle cradle-to-grave loop
**So that** vehicle disposal re-enters the resource economy instead of the vehicle simply vanishing

**Acceptance criteria:**
- AC-1: When a Vehicle's condition reaches zero (or its age exceeds its lifespan), it is removed from the fleet and a scrapyard produces waste_steel and waste_aluminium cargo in its place, rather than the vehicle entity being deleted with no material output. [SUBSTRATE: ABSENT — greenfield; no scrap/end-of-life path exists for transportation/vehicle.rs entities, spec/vehicles.md:67] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0152`

**Sources:**
- `spec/vehicles.md:67`

**Status:** pending