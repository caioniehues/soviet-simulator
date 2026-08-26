# Vehicles

**Status:** draft model (grounded in research)
**Phase:** 1
**Primary inspiration:** W&R physical owned vehicles; CS1 transient-spawn as the rejected contrast
**Evidence:** see [research/vehicles.md](../research/vehicles.md); vehicles carry `spec/resources.md` for `spec/logistics.md`, work `spec/construction.md` sites, and are themselves manufactured by `spec/production.md`.

> Vehicles are **owned physical assets, not spawned on demand.** Every truck, bus, tram, train, ship, crane and excavator is a real object with fuel, wear, a capacity, a depot and a driver — and eventually must itself be **manufactured** from steel/electronics/etc. No infinite vehicle spawning; the fleet is a finite, physical, aging thing the player must produce and maintain.

## Purpose

Define a vehicle as a first-class physical asset so that: logistics throughput is bounded by the *actual fleet* (not magic); a vehicle needs fuel (a resource) and wears out (demand for repair + new vehicles); and vehicle production is a real industry that closes the loop (`needs → car aspiration → vehicle industry → steel/electronics demand`, the transcript's signature cascade).

## Vehicle as asset (the data)

```
Vehicle {
  id; type            // ROAD/RAIL/WATER/AIR; cargo vs passenger vs construction
  owner; depot        // belongs to a depot/office — the fleet is finite
  cargoClass          // which RESOURCE_TRANSPORT_* it can carry (spec/resources.md)
  capacity            // how much, per trip
  fuel; fuelType      // consumes a fuel resource (or eletric for trams/trolleybuses)
  wear; condition     // ages with use → repair (spec/construction.md repair office) or scrap
  driver              // a citizen (spec/citizens.md) — labour bound to the trip
  route; position; cargo; state
}
```

### What the research confirmed (`research/vehicles.md`)

**542 vehicle `script.ini`** across five media (356 road, 136 rail, 21 ship, 21 airplane, 8 helicopter). One vocabulary spans a 10 t truck to a 19,250 t tanker:

```
$TYPE VEHICLETYPE_ROAD|RAIL_LOCOMOTIVE|RAIL_VAGON|SHIP|AIRPLANE|HELICOPTER
$RESOURCE_CAPACITY <n>              // tonnes, or persons for PASSANGER (431 files)
$RESOURCE_TRANSPORT_TYPE <class>    // exactly ONE cargo class per vehicle (419) — the hard gate
$MOVEMENT_SPEED / _POWER_KW / _EMPTY_WEIGHT
$COST_RUB | $COST_USD               // ruble vs dollar = Eastern vs Western bloc (ties to trade.md)
$AVAILABLE <from> <to>              // historical production window, e.g. 1976 2001
$SKILL_*                            // working roles: construction skills + PERSONAL/FIRETRUCK/AMBULANCE/…
$LIFESPAN 35                        // ONLY on the 12 trolleybuses — the fleet's lone longevity token
```

**Key confirmed absences (findings, not omissions):**
- `$MOVEMENT_CONSPUMPTION` is **`0` in all 161 occurrences** — no vehicle has an authored fuel-consumption figure; consumption is native. But fuel *stock* is physical: fleet buildings carry `$STORAGE_FUEL` diesel tanks refilled by tanker logistics, and citizens' cars refuel at `$TYPE_GAS_STATION` holding the `fuel` resource.
- **Zero wear/maintenance/condition tokens** across all 542 files — wear exists as a runtime mechanic but is not data-parametrised. Repair is still physical: garages (`$TYPE_REPAIR_OFFICE`) consume the **same `CARPLANT` spare-parts buckets the vehicle factories use** — repairing vehicles eats the parts stream that builds them.
- `$COST_RUB` is the placeholder `4300` in 409/454 files (a truck and an ocean tanker share it) — real prices are native. The meaningful data is the **currency split** (bloc) and `$AVAILABLE` window.
- Passengers are literally a cargo class (`RESOURCE_TRANSPORT_PASSANGER`, bus capacity 138 persons) — people ride the same capacity/class model as freight.
- Trains split tractive vs cargo assets: locomotive = power, no cargo; wagon = capacity, no propulsion (`$PURCHASE_EXCLUDE` — bought as consists).
- **No abstract fleet cap exists** — a depot's capacity is literally its `$VEHICLE_PARKING` slot count (tram depot: 40 painted slots). A parked vehicle needs a physical slot.

## Bought vs built (the ownership loop)

A key confirmed contrast (from `spec/construction.md §A`): **vehicles are the only thing in W&R with a money cost** (`$COST_RUB`) — because a vehicle is either **imported** (bought with money from abroad) or **manufactured domestically** by the vehicle industry (`production_vehicle.ini`: steel + plastics + components + fabric + electronics + labour → `vehicles`, `spec/production.md §D4`).

That's the whole point: money buys vehicles *from abroad* (foreign trade, `spec/trade.md`), but a self-sufficient socialist republic **builds its own** from a real production chain. The player chooses import (fast, costs hard currency) vs domestic production (slow, needs the industrial base). Either way the vehicle is then a physical owned asset.

### The full W&R lifecycle, confirmed end-to-end

```
steel+plastics+mcomponents+ecomponents+fabric+eletronics
   → vehicle factory ($TYPE_PRODUCTION_LINE, one per medium: road/rail×2/drydock/airplane)
   → `vehicles` cargo (RESOURCE_TRANSPORT_VEHICLES, hauled by car-transporter trucks)
   → vehicle lot / rail marshalling / car dealer
   → fleet slot (depot/office/household)
   → service (fuel from physical tanks; repair at garages eating CARPLANT parts)
   → scrapyard: vehicle in → waste_steel + waste_aluminium out
```
Cradle-to-grave, all CONFIRMED in data. The two acquisition doors — **import for bloc currency** or **domestic manufacture** — both produce the same `vehicles`-class cargo that must be physically delivered before it becomes a usable asset.

### CS1 — the transient anti-model (CONFIRMED from code)

- `VehicleManager.CreateVehicle` (`:1585`) spawns a truck **per transfer**, born already assigned to it; `CargoTruckAI.ArriveAtSource` ends the round trip with `ReleaseVehicle` — deleted. A **congested truck is deleted outright** (`Flags.Congestion` → release, cargo refunded). No vehicle exists between transfers.
- Global hard cap: one flat `Array16<Vehicle>` of **16,384**; when full, `CreateVehicle` silently fails and deliveries just don't happen.
- **No fuel, wear, cost or lifespan field anywhere** in `Vehicle`/`VehicleInfo` (confirmed by field inventory). Transit pays `m_maintenanceCostPerVehicle = 50` per line — renting a service level, never owning machines.
- Depot fleets are **budget-scaled spawn quotas**: allowed fleet = `budget% × m_maxVehicleCount`; surplus buses are recalled and despawned. Slide the slider down, buses cease to exist.

**What we keep from CS1:** only the flat fixed-size vehicle array as a cache-friendly ECS layout, and the depot *recall* mechanic inverted — our decommissioned vehicles drive home and **park** (persist), not despawn.

## Fuel & wear (why the fleet costs to run)

W&R confirms the *physical substrate* (fuel tanks, gas stations, parts-eating garages, scrapyard) but leaves the *rates* native (consumption always `0` in data, zero wear tokens). **We author what W&R left native** — the three biggest gaps become first-class fields:
- **`fuel_l_per_km`** — default derivable from `$MOVEMENT_POWER_KW`/`_EMPTY_WEIGHT`; metered against distance *and load*, so route length and vehicle choice have a real operating cost. Empty fleet tank ⇒ fleet idles.
- **`wear_per_km` + condition curve** — wear → garage repair (consuming components, W&R's `CARPLANT` bucket made explicit) → eventual scrapping into recoverable steel/aluminium. Fleet age becomes a real planning problem, as it was for actual socialist economies.
- **Price from the manufacturing bill** — not a hand-typed `4300`; the import price relates to the domestic bill of materials via `spec/trade.md`.
- **Propulsion typing** (electric tram/trolleybus vs diesel vs 1917 horse/steam) — W&R leaves this native; we make it data, tied to the era arc.

## Construction & working vehicles (cross-ref)
Construction vehicles (cranes, excavators, bulldozers, pavers, rollers) are vehicles with a `$SKILL_CONSTRUCTION_*` throughput — fully covered in `spec/construction.md §B3`. On-site production/working vehicles use `$WORKING_VEHICLES_NEEDED` (`spec/production.md §B1`). This spec owns their *asset* nature (fuel, wear, ownership); those specs own their *work* nature.

## Why no transient spawning (the CS1 contrast — now fully confirmed)
CS1's model is confirmed in code: spawn per transfer, delete on return *or on congestion*, 16,384-slot global cap, budget-percentage fleets. We reject it entirely: a delivery that has **no available vehicle waits**, which is exactly the bounded-throughput constraint that makes logistics a real planning problem. Fleet size = parking slots built (W&R's physical rule), not a slider.

## Open questions
- **Wear model depth.** Simple condition scalar → repair/scrap, or per-component wear (engine/tyres) for Dwarf-Fortress depth? Lean: scalar now, components later.
- **Driver as bound labour.** Does each vehicle consume a citizen-driver for its trip (labour cost on every delivery), or is driver abstracted? Binding drivers makes transport compete for labour — richer but heavier. Lean: bind for private cars + key services, abstract for bulk freight pools initially.
- **Import vs build balance.** How expensive (hard currency) is importing a vehicle vs the industrial cost of building one? A `spec/trade.md` + `spec/production.md` tuning question.
- **Fuel granularity.** One `fuel` resource, or petrol/diesel split by era (1917 start → coal/steam → diesel → electric)? The historical arc suggests era-varying fuel/propulsion.
- **Depot capacity & parking.** Physical depot slots limit fleet size per depot (W&R `$VEHICLE_PARKING`). How hard a constraint?

## Data (draft)
See the `Vehicle {}` block above. Movement at **high** frequency (position), fuel/wear at **low/medium** frequency, manufacture at **very-low** (`architecture/simulation-clock.md`). Vehicles are dense array entities (`architecture/ecs.md`) — 80k vehicles as data, not GameObjects.

## Evidence log
| Claim | Evidence level | Source | Notes |
|---|---|---|---|
| W&R vehicle = persistent asset: one cargo class + capacity + speed/power/weight + price + era window | CONFIRMED | 542 `script.ini` | research/vehicles.md §A |
| Vehicles are the only W&R entity with a money price — but 409/454 are placeholder `4300` (real prices native) | CONFIRMED | W&R vehicle `script.ini` | currency *split* (RUB/USD bloc) is the real data — §B1 |
| Full lifecycle: factory (steel+components) → `vehicles` cargo → lot/dealer → fleet slot → garage → scrapyard (waste_steel/aluminium out) | CONFIRMED | `production_vehicle.ini`, `cardealer.ini`, `scrapyard.ini` | §B2–B3 |
| Fuel stock is physical (`$STORAGE_FUEL`, gas stations); per-vehicle consumption native (`$MOVEMENT_CONSPUMPTION` always 0) | CONFIRMED | W&R `.ini` | §A2 |
| Zero wear/maintenance tokens in data; garages consume the factories' `CARPLANT` parts buckets | CONFIRMED (absence + token) | W&R `.ini` | §A3 |
| Depot capacity = physical `$VEHICLE_PARKING` slots, no abstract cap | CONFIRMED | `tram_depo_small.ini` (40 slots) | §A4 |
| CS1: spawn-per-transfer, delete on return/congestion, 16,384 global array, no fuel/wear/cost fields | CONFIRMED | `VehicleManager.cs:1585,730`, `CargoTruckAI.cs:539` | §E — the rejected anti-model |
| CS1 depot fleets are budget-scaled spawn quotas (surplus despawned) | CONFIRMED | `DepotAI.cs:485,548` | §E4 |
| Author what W&R left native: fuel_l_per_km, wear curve, price from bill of materials, propulsion typing | OURS | — | §G |
| Fleet is finite; no available vehicle ⇒ delivery waits; decommission = park, not despawn | OURS | — | bounded-throughput constraint |

## Related
- ../research/vehicles.md · ../spec/logistics.md · ../spec/resources.md · ../spec/production.md · ../spec/construction.md · ../spec/trade.md · ../spec/citizens.md · ../architecture/ecs.md · ../architecture/simulation-clock.md
