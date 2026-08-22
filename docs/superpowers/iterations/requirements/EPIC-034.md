# EPIC-034 — Transport-class compatibility

**Summary:** Transport-class compatibility
**Stories:** STORY-0130, STORY-0131, STORY-0132, STORY-0133, STORY-0134
**Primary sources:** `spec/logistics.md`, `spec/vehicles.md`
**Status:** 0/5 done

## STORY-0130

**Epic:** EPIC-034 — Transport-class compatibility
**Title:** Gate cargo assignment by resource transport-class compatibility

**As a** planner
**I want** a resource to only be loadable onto a vehicle whose cargoClass matches the resource's transport class
**So that** gravel tippers never carry bagged goods and oil tankers never carry ore, as in the confirmed W&R gate

**Acceptance criteria:**
- AC-1: Attempting to load a resource whose transport class does not match a vehicle's cargoClass is rejected (no trade/dispatch is created for that pair). [SUBSTRATE: ABSENT — greenfield; BuyOrder/SellOrder in economy/market.rs carry no transport-class or cargoClass field to gate on] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0146`
- AC-2: The internal market matcher only proposes (seller, buyer) trade pairs for a resource when at least one compatible-class vehicle exists at the scheduler's disposal; incompatible pairs never enter the sorted-by-distance candidate list. [SUBSTRATE: CONFLICTS/PARTIAL — economy/market.rs:219 ranks candidate pairs purely by squared distance with no transport-class gating at all] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0146`

**Sources:**
- `spec/logistics.md:31-40`

**Status:** pending

## STORY-0131

**Epic:** EPIC-034 — Transport-class compatibility
**Title:** Exclude network-borne resources from the vehicle scheduler

**As a** developer
**I want** electricity, heat, water and sewage to never be assigned a vehicle dispatch
**So that** utility flows stay on their own grids and don't pollute the physical logistics matcher

**Acceptance criteria:**
- AC-1: Items flagged as network-borne (electricity/heat/water/sewage) are never posted as BuyOrder/SellOrder entries in the vehicle-matched market, or are routed to a separate non-vehicle solver entirely. [SUBSTRATE: UNAUDITED — no per-item network-borne flag exists on ItemPrototype (prototypes/src/prototypes/item.rs:8-12 is `{base, id, optout_exttrade}` only); electricity itself is modelled via a separate ElectricityCache union-find (map/electricity_cache.rs:39-63), not via economy/market.rs, so today's separation is incidental rather than enforced by a flag] · impact:`cross-surface` · seam:`integration`
- AC-2: Solid/liquid cargo items flagged for fixed conveyance (adjacent-building conveyor/bulk-chute/pipe transfer) move at a metered rate between the two adjacent buildings without ever being posted as a BuyOrder/SellOrder or assigned a vehicle dispatch, distinct from the network-borne utility exclusion above. [SUBSTRATE: ABSENT — greenfield; no fixed-conveyance edge type exists in map/ or economy/market.rs, spec/logistics.md:58] · impact:`cross-surface` · seam:`integration`

**Sources:**
- `spec/logistics.md:33-37`

**Status:** pending

## STORY-0132

**Epic:** EPIC-034 — Transport-class compatibility
**Title:** Move goods across fixed conveyance edges without a vehicle

**As a** planner
**I want** adjacent buildings (e.g. a mine feeding its processor) to be linkable by a fixed conveyor/bulk-chute/pipe/cable edge that moves goods at a metered rate with no vehicle at all
**So that** tightly-coupled adjacent production stays free of truck-fleet overhead while still respecting the nothing-teleports rule (goods flow at a rate, not instantaneously)

**Acceptance criteria:**
- AC-1: A LogisticsEdge between two adjacent buildings may be flagged as fixed-conveyance (conveyor/bulk-chute/pipe/cable/heat), in which case goods cross it at a bounded per-tick rate and never enter the vehicle-matched BuyOrder/SellOrder market. [SUBSTRATE: ABSENT — greenfield; no LogisticsEdge or fixed-conveyance concept exists in economy/market.rs or map/, spec/logistics.md:58] · impact:`cross-surface` · seam:`integration`
- AC-2: A fixed-conveyance edge still obeys transport-class compatibility (the same resource-class gate that governs vehicle cargo); an incompatible resource cannot cross a conveyance edge built for a different class. [SUBSTRATE: ABSENT — greenfield, spec/logistics.md:58] · impact:`local` · seam:`unit`

**Sources:**
- `spec/logistics.md:58,39-41`

**Status:** pending

## STORY-0133

**Epic:** EPIC-034 — Transport-class compatibility
**Title:** Transport passengers under the same cargo-class model as freight

**As a** planner
**I want** a bus/tram to carry citizens as a passenger cargo class using the exact same capacity/cargoClass gating vehicles use for freight
**So that** passenger transport is not a bolted-on separate system but a confirmed reuse of the freight vehicle model

**Acceptance criteria:**
- AC-1: A passenger-class vehicle (e.g. bus) has a cargoClass of Passenger and a capacity denominated in persons, and is gated by the same cargoClass-compatibility check used for freight (AC of 'Gate cargo assignment by resource transport-class compatibility') rather than a separate passenger-only code path. [SUBSTRATE: ABSENT — greenfield; no passenger cargo class or citizen-as-cargo concept exists in economy/market.rs or transportation/vehicle.rs, spec/vehicles.md:36] · impact:`cross-surface` · seam:`integration`

**Sources:**
- `spec/vehicles.md:36`

**Status:** pending

## STORY-0134

**Epic:** EPIC-034 — Transport-class compatibility
**Title:** Route cargo through medium-transfer cargo stations

**As a** planner
**I want** a cargo station building to transship goods between transport media (rail to road, road to ship, ship to air) as an explicit node in a multi-modal delivery
**So that** a delivery whose fastest/cheapest route crosses media isn't blocked by the single-medium vehicle model

**Acceptance criteria:**
- AC-1: A cargo station accepts a compatible-class cargo delivery from a vehicle of one medium (e.g. rail) and makes it available for pickup by a vehicle of a different medium (e.g. road) without the cargo ever existing outside a vehicle or the station's dock buffer. [SUBSTRATE: ABSENT — greenfield; no cargo-station/transshipment node exists in economy/market.rs or map/, spec/logistics.md:41] · impact:`journey` · seam:`integration`

**Sources:**
- `spec/logistics.md:41`

**Status:** pending