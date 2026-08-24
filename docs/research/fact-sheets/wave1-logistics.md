# Wave 1 logistics substrate fact-sheet

**Kind:** reference
**Authority:** reference
**Status:** draft
**Owner:** logistics
**Last verified:** 2026-08-24
**Commit:** `186e08179b5ad9415dc4cd2d42d77a49303e35d6`

This fact-sheet constrains the controlled rewrite of vehicles, logistics, roads, pathfinding, and
traffic. It describes current reachability, not desired 1.0 behavior.

## Provided and reachable

### LOG-SUB-001 — Two dispatch classes use real networks

`Dispatcher` has `FreightTrain` and `SmallTruck` identities and maps them to rail and driving lanes
respectively (`simulation/src/map_dynamic/dispatch.rs:27-39,58-72`). Its update registers all trains
and every `VehicleKind::Truck` by position (`dispatch.rs:82-105`). Query reserves the nearest
eligible identity until `free` removes the reservation (`dispatch.rs:107-140`).

Classification: **PROVIDED** for nearest available vehicle reservation. It does not encode owner,
cargo, capacity, driver, depot, priority, or service class.

### LOG-SUB-002 — Market dispatch drives a truck over routed itineraries

`Market::advance_dispatches` requests a small truck near the seller, routes it to the seller, then
routes the same truck to the buyer (`simulation/src/economy/market.rs:470-523,546-565`). Seller
capital is debited on arrival at source and buyer capital credited on arrival at destination
(`market.rs:528-543,570-580`). With no available truck or source route, the dispatch remains in
`ToSource` and no seller stock moves (`market.rs:494-527`).

Classification: **PROVIDED** for a physical movement/timing witness and delayed inventory transfer.

### LOG-SUB-003 — Freight trains use the same reservation surface

Freight stations reserve a `FreightTrain`, route it to the station, model loading as an itinerary
wait, route it to the first external train station, then free it
(`simulation/src/souls/freight_station.rs:73-160`).

Classification: **PROVIDED** for train dispatch and movement. Cargo is an aggregate station counter,
not an embodied wagon inventory.

### LOG-SUB-004 — Companies spawn a finite number of parked trucks

Factory creation spawns exactly `proto.n_trucks` parked trucks and aborts company creation if it
cannot obtain all requested parking spots (`simulation/src/souls/goods_company.rs:113-148`). The
company retains their `VehicleID`s in `GoodsCompanyState.trucks`.

Classification: **PROVIDED** for finite spawned identities and company-side ID ownership.

## Partial or conflicting contracts

### LOG-SUB-005 — Cargo is not embodied by the vehicle

`Vehicle` contains movement state, kind, tint, speed multiplier, wait time, and a gridlock flag; it
has no owner, cargo, capacity, cargo class, depot, fuel, wear, or driver field
(`simulation/src/transportation/vehicle.rs:15-45`). In-flight item and quantity remain fields of the
market dispatch while the vehicle merely carries the itinerary.

Classification: **PARTIAL**. The rewrite may say that a real truck gates delivery, but it must not
claim goods reside in the truck or that truck capacity constrains quantity.

### LOG-SUB-006 — Company ownership does not constrain global dispatch

Companies retain truck IDs (`goods_company.rs:127-148`), but dispatcher registration pools every
truck globally without consulting that ownership (`map_dynamic/dispatch.rs:95-104`). Market queries
only by kind and seller position (`economy/market.rs:496-503`).

Classification: **CONFLICTING** with a finite enterprise/depot fleet contract. A company-owned truck
can serve any market dispatch.

### LOG-SUB-007 — Old company delivery and new market freight both remain live

Every sold trade can still be assigned to the company's human driver through
`WorkKind::Driver.deliver_order` (`simulation/src/souls/goods_company.rs:235-270`). The market also
creates and advances its independent truck dispatch for non-labour trades. These paths do not share
one reservation or fulfillment authority.

Classification: **CONFLICTING**. The controlled rewrite must describe this as substrate debt, not as
the target logistics model.

### LOG-SUB-008 — Completion releases a truck without parking it

At unload completion the dispatcher reservation is freed and the itinerary cleared, but the truck
is left in `VehicleState::Driving` where it stopped (`simulation/src/economy/market.rs:583-599`).

Classification: **PARTIAL** for reusable dispatch; **ABSENT** for return-to-depot/parking recovery.

### LOG-SUB-009 — Dispatch failure has no terminal recovery policy

No route or no truck causes indefinite retry. A missing truck entity during travel produces
`arrived = false`; only successful unloading removes the dispatch
(`simulation/src/economy/market.rs:479-609`). There is no timeout, reassignment, cancellation,
cargo recovery, or player-visible stalled-job state in this seam.

Classification: **ABSENT**. Failure currently wedges rather than becoming an explicit queue with a
recoverable reason.

## Rewrite constraints

- Distinguish allocation, reservation, pickup, custody, delivery, and consumption as separate events.
- Say inventory transfers at source/destination in the current market seam; do not say the vehicle owns cargo.
- Treat finite company fleets, capacity, cargo class, depot allocation, and recovery as desired contracts, not provided behavior.
- Require one fulfillment authority before claiming company drivers and global dispatch form a coherent logistics system.
- Make stalled dispatches observable and recoverable without deleting demand or goods.

## Verification boundary

All cited source locations were reopened on 2026-08-24. No runtime test, mutation, save/load test, or
UI capture was executed for this fact-sheet. Existing tests can support individual transfer guards
only after the evidence auditor proves their assertions fail for the intended defect.
