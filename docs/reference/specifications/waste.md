# Waste specification

**Kind:** specification
**Authority:** binding
**Status:** draft
**Owner:** utilities
**Last verified:** 2026-08-24

The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT, RECOMMENDED, NOT RECOMMENDED, MAY, and OPTIONAL in this document are to be interpreted as described in RFC 2119 and RFC 8174.

## Purpose

Waste is typed physical material held in finite containers and moved only by Logistics-compatible vehicles. Waste owns generation records, containers, collection requests, processing disposition, and backlog; it does not create a second dispatcher or become a piped utility.

## Invariants

- `SPEC-WASTE-001` — Waste SHALL be sole authority for waste type, generation event, container identity/capacity/fill, collection request, overflow/backlog, processing disposition, and landfill retention. Container fill is physical quantity, not a cleanliness score.
- `SPEC-WASTE-002` — Waste SHALL request one compatible Logistics haul; it MUST NOT assign a vehicle, move quantity, copy custody, or teleport a container. Pickup, delivery, cancellation, and recovery obey Logistics.
- `SPEC-WASTE-003` — Full containers, absent compatible vehicle/route, blocked processor, or full landfill retain typed physical waste and aged backlog. Failure degrades declared local service but MUST NOT delete waste or end the plan.
- `SPEC-WASTE-004` — Each collected quantity has exactly one disposition: Waste-held landfill retention, Resources-accepted recovered-material handoff, or Production-accepted incineration input. Recycling/incineration MUST NOT directly credit material, electricity, or heat.
- `SPEC-WASTE-005` — Collection/processing priority is explicit and non-price-based. Healthcare, Buildings, and consumers reference Waste results and MUST NOT mutate containers, custody, or disposition.
- `SPEC-WASTE-006` — A pickup crossing Waste container state and Logistics custody SHALL use one immutable `CollectionReceiptID`. On its single atomic acceptance, Waste decrements that named container fill by `q` while Logistics credits the same `q` to the referenced haul custody; duplicate receipt application is a no-op. This receipt does not mutate a Resources on-hand balance.
- `SPEC-WASTE-007` — Each delivered waste quantity SHALL have one immutable `WasteDispositionReceiptID` and one disposition only. A landfill receipt retains exactly its accepted quantity. A recycling or incineration receipt atomically binds to one immutable Production-owned `ProductionRunID`/result which may accept processor input `p <= q`; the unaccepted remainder `q - p` remains in named delivered/processor-queue custody. Production exclusively owns transformation/yield and proves `p = outputs + named_residue + permitted_named_loss`; Resources accepts a resulting material on-hand balance once from that named Production result. Duplicate receipt or Production-result application is a no-op; Waste never directly credits a receiving output.

## Model and state

Waste owns `WasteType`, `WasteGenerationID`, `WasteContainerID`, capacity/fill, source endpoint, `WasteCollectionRequestID`, overflow age/reason, processor queue, final disposition, `CollectionReceiptID`, and `WasteDispositionReceiptID`. It submits compatible haul requirements to Logistics and accepts delivery outcomes by reference. Recycling and incineration bind to one Production-owned `ProductionRunID`/result; Resources then accepts a named recovered-material output once. A later Electricity/Heating result remains owned by its module. Landfill retains quantity physically.

## Failure behavior and observability

Finite containers fill, overflow, and wait visibly for a compatible haul/processor. Interrupted haul follows Logistics cancellation/recovery, leaving waste in container or accountable vehicle custody. Full landfill refuses intake without deleting contents. The Planner can inspect type, source, fill/capacity, collection age/reason, haul/custody reference, processor queue, disposition, and receiving Resources/Production result.

## Acceptance evidence

All guards are **UNIMPLEMENTED** and block ratification. A zero-test command is failure; the current serial suite proves no target below.

| Evidence | Future guard command and observable assertion | Required red mutation | Player-facing proof |
|---|---|---|---|
| `EVID-WASTE-001` | `cargo test -p simulation evid_waste_exactly_one_compatible_logistics_haul -- --test-threads=1` — each collection request creates exactly one compatible Logistics haul; one `CollectionReceiptID` atomically decrements its container and credits the same haul custody once, while duplicate receipt application is a no-op. | Create duplicate hauls, replay/duplicate pickup receipt, credit processor at request/assignment, or use incompatible vehicle. | Inspected request, haul ID, receipt ID, container, and custody timeline. |
| `EVID-WASTE-002` | `cargo test -p simulation evid_waste_retention_overflow_and_cancel_recovery -- --test-threads=1` — no compatible route, blocked processor, and full landfill retain typed waste/backlog; cancellation follows Logistics recovery. | Delete overflow, accept into full landfill/blocked processor/no route, strand reservation, or reset custody. | Inspected backlog, refusal reason, landfill fill, and recovery capture. |
| `EVID-WASTE-003` | `cargo test -p simulation evid_waste_disposition_conservation -- --test-threads=1` — one `WasteDispositionReceiptID` applies exactly one fate: landfill retains its accepted quantity, or it atomically binds one Production-owned `ProductionRunID`/result accepting `p <= q`, retaining `q - p` in named delivered/processor-queue custody, and proving `p = outputs + named_residue + permitted_named_loss`. Resources accepts each named resulting material on-hand balance once from that Production result. Receipt/result replay is a no-op and Waste never credits output directly. | For `q=10,p=5`, delete the `5` remainder; let Resources transform/yield; omit a residue holder; replay/duplicate receipt or Production result; accept output without its bound receipt/result; send one delivery to two fates; accept `p > q`; emit outputs/residue/loss greater than `p`; or direct-credit energy/material. | Inspected delivery, receipt, ProductionRunID/result, accepted input, named remainder custody, Resources once-only acceptance, output/residue/loss, yield, and landfill ledger capture. |
| `EVID-WASTE-004` | `cargo test -p simulation evid_waste_nonprice_authority_references_not_copies -- --test-threads=1` — non-price priority and no outside waste mutation. | Rank by roubles or add Healthcare/Buildings waste balance. | Inspected queue/authority capture. |

## Substrate and decisions

No waste building kind or registered system exists (`simulation/src/map/objects/building.rs:17-37`; `simulation/src/init.rs:52-70`). Existing freight/vehicles cannot prove waste collection. The [Wave 2 fact-sheet](../../research/fact-sheets/wave2-substrate.md#2c--utilities-electricity-water-sewage-heating-and-waste) records no waste save/UI/test surface. Domestic fulfillment/custody belongs to Logistics (`docs/reference/specifications/logistics.md:29-62`) and on-hand balance mutation belongs to Resources (`docs/reference/specifications/resources.md:45-55`). Legacy `spec/waste.md` is provenance only.

## Deferred behavior

Player-placeable container systems, vehicle manufacture/fuel lifecycle, waste treatment tiers, and unratified electricity/heat output variants have no 1.0 acceptance criteria.

## Open questions

- Which waste types/source categories are 1.0?
- Which recovery handoffs and incineration recipes are in scope?
