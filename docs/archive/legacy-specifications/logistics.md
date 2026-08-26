# Logistics

**Status:** draft model (grounded in research)
**Phase:** 1
**Primary inspiration:** W&R distribution offices generalised into a policy scheduler; CS1 `TransferManager` offer-matching for comparison
**Evidence:** see [research/logistics.md](../research/logistics.md); moves `spec/resources.md` between `spec/production.md`, `spec/construction.md` nodes via `spec/vehicles.md`.

> The layer that makes "physical" mean something. Every resource that leaves one building and enters another does so as a **real cargo trip on a compatible vehicle over a real network** — no teleporting stock, no magic delivery. This is the third leg of the economy: `resources → production → **logistics** → construction/consumption`.

## Purpose

Define how supply meets demand **physically**: a building with surplus, a building with need, a compatible vehicle, a route, loading/unloading time, and a dispatch decision. And — per the vision — make the player set **policy** (target stock levels), not micromanage individual trucks. The scheduler turns policy into dispatches.

## The policy-scheduler model (the vision's core ask)

Instead of "Truck #174 go there," the player declares intent per storage:

```
Warehouse A:
   gravel  target 40–80%
   steel   target 30–70%
   food    target 60–90%
```

The scheduler then solves:

```
demand (below-min buffers)  +  supply (above-max buffers)
        ↓  match by resource + transport-class compatibility
        ↓  rank by cost (distance, mode, loading availability)
        ↓  assign a compatible idle vehicle from a depot/office
        ↓  dispatch: travel → load → travel → unload
```

### What the research confirmed (`research/logistics.md`)

**The W&R distribution office `.ini` carries the fleet, not the policy** — the key finding. `distribution_office_small.ini` contains only: its construction bill, a fuel tank for its own trucks (`$STORAGE_FUEL RESOURCE_TRANSPORT_OIL 20`), fleet size (`$WORKING_VEHICLES_NEEDED 6`), dock/parking geometry (`$VEHICLE_STATION`/`$VEHICLE_PARKING`), and a road hookup. **There is no target-level, source/destination, threshold or route token in any of the 488 `.ini` files** (CONFIRMED absence). The per-warehouse target stock levels the vision praises are **runtime player state stored in the savegame, executed by a native solver** — the data files expose only the solver's *hardware*, never its *plan*. So our scheduler's policy layer is genuinely OURS to design; W&R gives us the physical vocabulary:

- **Storage buildings** (`$TYPE_STORAGE`, 25 files) = one transport-class bucket + capacity + a dock loading rate: `$STORAGE RESOURCE_TRANSPORT_OPEN 330` + `$VEHICLE_LOADING_FACTOR 5.7` / `$VEHICLE_UNLOADING_FACTOR` (loading also draws power: `$ELETRIC_CONSUMPTION_LOADING_FIXED`). **Loading throughput is a real, declared bottleneck.**
- **Supply/demand is implicit in buckets** (no offer objects): a non-full `$STORAGE_IMPORT` bucket = demand; a non-empty `$STORAGE_EXPORT` bucket = supply; `$STORAGE_DEMAND_BASIC/…` = retail sinks. The solver reads buckets, not bids.
- **Cargo stations** (`$TYPE_CARGO_STATION`, 43 files) are the medium-transfer nodes (rail↔road↔ship↔air).
- The office is a **generic hauler** — it stores nothing it moves (fuel only); trucks shuttle export-bucket → import-bucket per the player's wiring.

## Transport edges & mode compatibility

A logistics edge exists only where the network + vehicle + resource class agree (all confirmed vocabulary from `spec/resources.md`):

- **Resource transport class** (`RESOURCE_TRANSPORT_GRAVEL/OIL/CEMENT/COOLER/COVERED/OPEN/…`) determines which vehicle can carry it.
- **Connection medium** (`$CONNECTION_ROAD/RAIL/CONVEYOR/PIPE/WATER/…`) determines how it enters/leaves a building.
- **Network-borne resources** (electricity, heat, water, sewage) flow on their own grids — no vehicle, handled by the utility specs, excluded from the vehicle scheduler.

So: fuel needs a tanker on a road/rail edge or a pipe; ore needs a bulk hauler or conveyor; meat needs a refrigerated vehicle. A mismatch is simply not an edge.

### The confirmed compatibility gate (end-to-end, W&R)

The transport class is pinned by the **same string in three places** — resource handling class (`spec/resources.md §B1`), storage bucket class, and the vehicle's `$RESOURCE_TRANSPORT_TYPE` (each cargo vehicle declares exactly one; 293 vehicles: 104 PASSANGER, 43 OPEN, 35 COVERED, 31 GRAVEL, 24 OIL, 11 WASTE, 10 COOLER…). A gravel tipper cannot carry bagged goods; an oil tanker cannot carry ore. `$TYPE VEHICLETYPE_ROAD/RAIL/…` further gates the *medium* the vehicle can dock at. **CONFIRMED** — this is the hard constraint CS1 lacks.

**Two kinds of edge (CONFIRMED):** vehicle-served docks (road/rail/water/air + loading rate) **vs fixed conveyance** — conveyor/bulk-chute/pipe/cable/heat links that move goods **with no vehicle at all**. Adjacent mine→processor belts bypass the truck fleet entirely. CS1 has no fixed-edge layer (every transfer spawns a vehicle); we keep W&R's.

### CS1's TransferManager — the contrasting model (CONFIRMED from code)

Every building each step posts `TransferOffer`s (Amount, Priority 0–7, Position, Active/Exclude/Unlimited) into per-material priority buckets; `MatchOffers` greedily clears the market:

```
score = (partnerPriority + 0.1) / (1 + squaredDistance × distanceMultiplier)   // TransferManager.cs:1338
```

- **The multiplier is per-material** — and for all cargo it's `1E-07` (distance barely matters → priority dominates → goods routinely hauled across the map: *the confirmed root cause of CS1's absurd cross-city freight*). Services/emergencies use `1E-05` (nearest-first). Dummy traffic is negative (prefers farther!).
- The **Active** side of a match spawns the vehicle (`StartTransfer`); a warehouse's exports are Active, imports passive.
- Warehouse "policy" is only **Fill / Balanced / Empty** modes biasing offer priority by fill fraction — no numeric targets (`WarehouseAI.cs:584,645`).
- The map border (`OutsideConnectionAI`) posts **unlimited priority-0 offers** for every cargo — an infinite supplier/sink that local partners always outbid. No customs, no physical border.
- Rate limiting: materials are round-robined across frames (`GetFrameReason`) — a proven amortisation trick we adopt.

### W&R customs — trade transport is physical (CONFIRMED)

`$TYPE_CUSTOMHOUSE` (5 files) is a border **pass-through**: one `$STORAGE <class> 1` token-bucket per transport class (it handles any cargo but stockpiles nothing), `$BORDER_BUILDING`, an inward `$CONNECTION_ROAD` and an outward `$CONNECTION_ROAD_BORDER`/`$CONNECTION_RAIL_BORDER`, plus vehicle bays where trucks clear customs. Utilities cross at dedicated border buildings (`$TYPE_ELETRIC_EXPORT/IMPORT`, `$TYPE_FOREIGN_PIPELINE_EXPORT`) — the good's transport medium decides which border building it crosses at, exactly as domestically. Imports are **real vehicles on real crossings**, not CS1's infinite offer. (Prices live in `spec/trade.md`; movement is just another edge here.)

## Two contrasting scheduling philosophies (CONFIRMED both sides)
- **W&R — centralised planning:** player sets the dial (target stock per warehouse) + the wiring diagram (which source feeds which sink); a native solver moves goods with a pooled fleet under hard transport-class constraints, with free fixed conveyors where they exist.
- **CS1 — decentralised bidding:** every building posts offers; a greedy priority-×-distance auction clears the market each frame, one spawned truck per trade — with cargo distance weighted so lightly (`1E-07`) that goods teleport-haul across the map.

**Our synthesis:** W&R's policy model (persistent intent) with CS1's matching *shape* (`score = f(priority, distance)`) made **explicit, legible and fixed**: priority driven by **target-level deficit** (how far below target a sink is), distance weighted *meaningfully* for cargo, and the whole rule published to the player — no native black box, no cross-map hauling bug. One scheduler serves truck distribution, rail dispatch, ship logistics, warehouse replenishment, **and construction supply** (`spec/construction.md`'s office is just a consumer whose target level is its bill of materials).

## Why this couples to everything
- Production stalls when its **output export bucket** fills (no freight) or an **input import bucket** empties (no delivery) — `spec/production.md`'s cascade engine is a logistics failure.
- Construction stalls when materials don't reach the site — `spec/construction.md`.
- Citizen needs go unmet when shops aren't replenished — `spec/needs.md`.
Logistics is where all three cascades physically originate.

## Open questions

**Resolved by research:**
- ~~Push vs pull~~ → W&R has no offer objects; the solver reads buckets (import-not-full = demand, export-not-empty = supply) and the office hauls to targets. Bucket-driven pull, office-executed. Confirmed shape; exact native rule INFERRED.
- ~~Priority tiers~~ → CS1 confirms 0–7 priority buckets work; we drive priority from **target-level deficit** instead of fill-mode toggles.
- ~~Vehicle sourcing~~ → pooled fleets at distribution offices (`$WORKING_VEHICLES_NEEDED`), confirmed. Asset side in `spec/vehicles.md`.
- ~~Loading as bottleneck~~ → confirmed real: `$VEHICLE_LOADING_FACTOR`/`_UNLOADING_FACTOR` + loading power draw. We model dock throughput.

**Still open:**
- **Scheduler granularity vs performance.** A global solve every tick over thousands of stores is infeasible. CS1's confirmed trick — round-robin materials across frames — plus spatial partitioning (`architecture/` rules: never globally solve what's local). How local can matching be without starving distant needs?
- **The matching objective.** Exact formula for `score(deficit, distance, priority)` and its tuning — W&R's is native/unrecoverable, so this is genuinely OURS. Needs a prototype (`/prototype` candidate: does deficit-driven dispatch feel right at scale?).
- **Wiring: explicit or automatic?** W&R makes the player wire source→destination manually. Do we require wiring (planning gameplay) or auto-match within a district with optional overrides? Lean: auto-match by default, player overrides as the planning tool.
- **Fixed-conveyance placement rules.** Conveyors/pipes as player-built network edges — which pairs may link directly? Belongs partly to `spec/roads.md`-style network specs.
- **Rail capacity/signaling.** The scheduler must respect line capacity; detail deferred to a rail pass.

## Data (draft)
```
StoragePolicy { resourceId; minPct; maxPct }              // player-set intent per bucket
StorageBucket { resourceId; transportClass; capacity; current; policy }
LogisticsEdge { fromNode; toNode; medium; transportClass }  // exists only where class+medium+network agree
TransferRequest { resourceId; qty; fromNode; toNode; priority; cost }  // scheduler output
Dispatch { vehicleId; request; state }                    // travel→load→travel→unload
```
Scheduler runs at **medium** frequency (matching), vehicle movement at **high** frequency. Spatially partitioned (`architecture/simulation-clock.md`, `architecture/` performance rules).

## Evidence log
| Claim | Evidence level | Source | Notes |
|---|---|---|---|
| Transport class gates end-to-end: resource = bucket = vehicle `$RESOURCE_TRANSPORT_TYPE` | CONFIRMED | W&R `.ini` ×3 (resource/bucket/vehicle) | research/logistics.md §B2; 293 vehicles classed |
| Distribution office `.ini` = fleet + fuel + docks only; **no policy tokens exist in data** | CONFIRMED (absence) | W&R `distribution_office_*.ini` + whole-file grep | policy is runtime savegame state + native solver — §A1 |
| Storage building = one class bucket + capacity + dock loading rate | CONFIRMED | W&R `warehouse_open.ini` etc. | `$VEHICLE_LOADING_FACTOR` — loading is a real bottleneck |
| Supply/demand implicit in buckets (no offer objects) | CONFIRMED (shape) / INFERRED (rule) | W&R `$STORAGE_IMPORT/_EXPORT/_DEMAND_*` | solver logic native — §A4 |
| Fixed conveyance (conveyor/pipe/cable) moves goods with no vehicle | CONFIRMED | W&R `$CONNECTION_CONVEYOR/PIPE/…` | CS1 has no equivalent — §B1 |
| CS1 match: `score=(priority+0.1)/(1+d²·mult)`, per-material distance mult | CONFIRMED | CS1 `TransferManager.cs:1338,1032` | cargo mult `1E-07` → cross-map hauling bug — §D2-D3 |
| CS1 warehouse policy = only Fill/Balanced/Empty priority biases | CONFIRMED | CS1 `WarehouseAI.cs:584,645` | no numeric targets — §E3 |
| CS1 border = unlimited priority-0 offers (no physical customs) | CONFIRMED | CS1 `OutsideConnectionAI.cs:967` | §E5 |
| W&R customs = per-class pass-through + border connections; utilities cross at dedicated border buildings | CONFIRMED | W&R `zoll_*.ini` | trade transport is physical — §C |
| Per-frame material round-robin as scheduler rate-limit | CONFIRMED (CS1) → adopted | CS1 `GetFrameReason` | implementation note |
| Deficit-driven priority + meaningful cargo distance weight + published matching rule | OURS | — | fixes CS1's bug; replaces W&R's black box |
| Network-borne resources (power/heat/water) excluded from vehicle scheduler | OURS | — | see utility specs |

## Related
- ../research/logistics.md · ../spec/resources.md · ../spec/vehicles.md · ../spec/production.md · ../spec/construction.md · ../spec/trade.md · ../spec/needs.md · ../architecture/simulation-clock.md
