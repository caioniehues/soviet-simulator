# Waste

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** infrastructure
**Last verified:** 2026-09-03

| Scope | 1.0 — charter row *Utilities* |

## What this is

Waste is typed physical material held in finite containers and moved only by
Logistics-compatible vehicles. It is not a piped utility. A full container overflows.
An absent truck leaves waste sitting. The Planner provides collection infrastructure:
landfills and incinerators.

Waste uses Logistics for transport. It does not create a second dispatcher. It does not
generate its own trucks. It requests a compatible haul, and Logistics assigns a vehicle
and routes it.

## 1.0 requirement

`SPEC-WASTE-001` — Waste SHALL be sole authority for waste type, generation event,
container identity/capacity/fill, collection request, overflow/backlog, processing
disposition, and landfill retention. Container fill is physical quantity, not a
cleanliness score.

`SPEC-WASTE-002` — Waste SHALL request one compatible Logistics haul. It MUST NOT
assign a vehicle, move quantity, copy custody, or teleport a container.

`SPEC-WASTE-003` — full containers, absent vehicle/route, blocked processor, or full
landfill retain typed physical waste and aged backlog.

`SPEC-WASTE-004` — each collected quantity has exactly one disposition: landfill retention,
Resources-accepted recovered-material handoff, or Production-accepted incineration input.

`SPEC-WASTE-006` — one immutable `CollectionReceiptID` for each pickup.

`SPEC-WASTE-007` — one immutable `WasteDispositionReceiptID` per delivered quantity with
exactly one fate.

## Target design

The charter commits to landfill and incinerator in 1.0. Waste is vehicle-hauled, with the
same custody rules as freight. The target lifecycle:

1. Building generates waste into a typed container
2. Container fills over time
3. When above threshold, Waste requests a Logistics haul
4. A compatible truck collects waste (pickup, custody transfer)
5. Truck delivers to landfill or incinerator
6. Landfill retains quantity. Incinerator feeds a Production run that may yield energy
   or residue.

Full containers degrade local service (healthcare, quality of life) but do not end the
plan.

## Current substrate

No waste building kind or registered system exists
(`simulation/src/map/objects/building.rs:17-37`, `simulation/src/init.rs:52-70`).
Existing freight and vehicles cannot prove waste collection. This is entirely greenfield.

## Open questions

- Which waste types and source categories are 1.0?
- Which recovery handoffs and incineration recipes are in scope?

## Related

- [Logistics](../physical-economy/logistics.md)
- [Production](../physical-economy/production.md)
- [Vehicles](../transport/vehicles.md)
- [Waste spec](../../reference/specifications/waste.md)
