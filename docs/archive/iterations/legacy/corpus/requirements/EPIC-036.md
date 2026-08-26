# EPIC-036 — Policy scheduler

**Summary:** Policy scheduler
**Stories:** STORY-0145, STORY-0146, STORY-0147, STORY-0148, STORY-0149
**Primary sources:** `spec/logistics.md`
**Status:** 0/5 done

## STORY-0145

**Epic:** EPIC-036 — Policy scheduler
**Title:** Author policy as target stock levels per storage bucket

**As a** planner
**I want** to set a min/max target percentage per resource per storage building
**So that** I set intent (policy) rather than micromanaging individual truck routes

**Acceptance criteria:**
- AC-1: A StoragePolicy {resourceId, minPct, maxPct} can be set on a storage bucket and persists across save/load. [SUBSTRATE: ABSENT — greenfield; no policy token exists anywhere in current code, confirmed absent in W&R data too per audit §Purpose] · impact:`local` · seam:`unit`
- AC-2: A storage bucket below its policy minPct is treated as demand (a buy order is posted); a bucket above its maxPct is treated as supply (a sell order is posted). [SUBSTRATE: PARTIAL — economy/market.rs BuyOrder/SellOrder machinery exists (market.rs:150-175) but is driven by direct qty calls today, not by a policy-threshold translation layer] · impact:`cross-surface` · seam:`integration`
- AC-3: By default the scheduler auto-matches a demand bucket to the nearest/best compatible supply bucket within a district without requiring the player to manually wire source to destination; the player may set an explicit source-destination override that the scheduler then honours instead of auto-matching for that pair. [SUBSTRATE: ABSENT — greenfield; economy/market.rs has no wiring/override concept, spec/logistics.md:101] · impact:`cross-surface` · seam:`integration`

**Sources:**
- `spec/logistics.md:18-30`

**Status:** pending

## STORY-0146

**Epic:** EPIC-036 — Policy scheduler
**Title:** Rank dispatch candidates by deficit priority and meaningful distance, not distance alone

**As a** planner
**I want** the scheduler to prioritise the sink furthest below its target level, weighted by distance, when multiple demands compete for the same supply
**So that** the matching rule is legible and avoids CS1's bug where near-zero distance weighting caused cross-map hauling

**Acceptance criteria:**
- AC-1: Given two buy orders for the same item at equal distance from a seller but different target-level deficits, the trade matcher selects the order with the larger deficit first. [SUBSTRATE: CONFLICTS — economy/market.rs:219 scores candidate pairs purely by `sorder.pos.distance2(border.pos)`; no priority or deficit term exists in the score at all, and BuyOrder (market.rs) has no deficit/priority field] · impact:`cross-surface` · seam:`unit` · scenario:`SCENARIO-0148`
- AC-2: Given two buy orders with equal deficit but different distances, the nearer buyer is served first (distance remains a live tiebreaker term, unlike CS1's near-zero cargo multiplier). [SUBSTRATE: PARTIAL — economy/market.rs:219 already ranks by squared distance; only the deficit term is missing] · impact:`cross-surface` · seam:`unit` · scenario:`SCENARIO-0148`

**Sources:**
- `spec/logistics.md:60-72,95-97`

**Status:** pending

## STORY-0147

**Epic:** EPIC-036 — Policy scheduler
**Title:** Model loading/unloading dock throughput as a real bottleneck

**As a** planner
**I want** each storage building's dock to have a loading/unloading rate that caps how fast cargo can transfer, independent of vehicle availability
**So that** a single dock cannot be instantly drained or filled regardless of fleet size

**Acceptance criteria:**
- AC-1: A storage bucket has a loading-rate field; a vehicle at that dock can only transfer up to `rate * elapsed_time` per tick, even if it has spare capacity and the bucket has spare stock. [SUBSTRATE: ABSENT — greenfield; no storage/dock entity or loading-rate field exists in simulation/src] · impact:`local` · seam:`integration`
- AC-2: A storage bucket's dock also draws electrical power while loading; a dock with no power available cannot transfer cargo even if its loading-rate budget and vehicle capacity both have headroom. [SUBSTRATE: ABSENT — greenfield; no coupling exists between simulation/src storage/dock logic and map/electricity_cache.rs, spec/logistics.md:39] · impact:`local` · seam:`integration`

**Sources:**
- `spec/logistics.md:20-27`

**Status:** pending

## STORY-0148

**Epic:** EPIC-036 — Policy scheduler
**Title:** Rate-limit and spatially partition scheduler matching

**As a** developer
**I want** the trade matcher to round-robin materials across frames and match locally within spatial partitions rather than performing a full global solve over every store every tick
**So that** the scheduler stays performant at thousands of stores, matching the adopted CS1 amortisation trick and the project's never-globally-solve-what's-local rule

**Acceptance criteria:**
- AC-1: Not every resource/store pair is re-evaluated by the matcher on every tick; matching work for distinct materials is round-robined across frames (`GetFrameReason`-style amortisation), so a single tick's matcher pass touches a bounded subset of materials. [SUBSTRATE: ABSENT — economy/market.rs make_trades has no frame-amortisation, it scans all orders every call it's invoked on, spec/logistics.md:72] · impact:`none` · seam:`unit`
- AC-2: Trade matching is spatially partitioned so a demand in one region is not compared against every supply on the map every tick; a local match is preferred within a bounded radius/district before falling back to a wider search. [SUBSTRATE: CONFLICTS — economy/market.rs:219 ranks ALL candidate pairs for a resource by squared distance with no spatial partitioning, spec/logistics.md:99] · impact:`none` · seam:`integration`

**Sources:**
- `spec/logistics.md:72,99`

**Status:** pending

## STORY-0149

**Epic:** EPIC-036 — Policy scheduler
**Title:** Sequence every dispatch through travel, load, travel, unload

**As a** planner
**I want** a dispatched delivery to move through an explicit travel-to-source, load, travel-to-destination, unload state sequence, and return to service (empty or with a new assignment) once unload completes
**So that** the concrete falsifiable form of nothing-teleports holds for every trip, not just for the vehicle-availability gate

**Acceptance criteria:**
- AC-1: A Dispatch carries a state field constrained to travel-to-source, loading, travel-to-destination, unloading (in that order); a dispatch cannot enter loading before reaching the source, and cannot enter unloading before reaching the destination. [SUBSTRATE: ABSENT — greenfield; no Dispatch entity or state field exists in economy/market.rs or transportation/, spec/logistics.md:111,32] · impact:`journey` · seam:`integration` · scenario:`SCENARIO-0151`
- AC-2: When a dispatch's unloading state completes, the vehicle becomes idle and either returns empty toward its depot or immediately accepts a new assignment; it is never left mid-cycle with stale cargo or a stale destination. [SUBSTRATE: ABSENT — greenfield, spec/logistics.md:15,111] · impact:`local` · seam:`integration` · scenario:`SCENARIO-0151`
- AC-3: Seller stock decrements only on the dispatch's transition into `loading`, and buyer stock increments only on the transition into `unloading`; between those transitions the quantity is held by the dispatch itself and counted in neither bucket. [SUBSTRATE: CONFLICTS — today economy/market.rs:277-279 moves stock at trade-MATCH time (`*cap_buyer += trade.qty;` then `*capital.get_mut(&trade.seller.0).unwrap() -= trade.qty;`), before any physical dispatch runs, so without this AC the dispatch state machine could pass while the truck is decorative] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0151`
- AC-4: The dispatch state machine is implemented as an extension of the existing `map_dynamic::Dispatcher` (already imported and used by souls/freight_station.rs:5-9), not a bespoke parallel trip mechanism that would later need to be torn out. [SUBSTRATE: PARTIAL — map_dynamic::Dispatcher exists and is in use, souls/freight_station.rs:5-9] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0151`

**Sources:**
- `spec/logistics.md:32,111`

**Status:** pending