# Physical economy

**Kind:** index
**Authority:** advisory
**Status:** draft
**Owner:** economy
**Last verified:** 2026-08-28

## What belongs here

This section describes the physical sequence that every unit of goods follows from
request to consumption. Each page covers one state or transition in that sequence.
Planned-economy pages (quotas, plan periods, credibility) live in
[`../planned-economy/`](../planned-economy/index.md). Transport pages (roads, vehicles,
pathfinding, traffic) live in [`../transport/`](../transport/index.md). Infrastructure
pages (electricity, water, heating) live in [`../infrastructure/`](../infrastructure/index.md).

## The canonical physical sequence

Every good in the simulation passes through these states, each governed by a different
authority. No step may be skipped; no step implies that the next has occurred.

```text
 1. Request         — a stated demand (Production, Needs, Construction)
 2. Allocation      — the Logistics authority selects a source (Logistics)
 3. Reservation     — an encumbrance on source stock (Market / Resources)
 4. Vehicle reservation — a finite truck or wagon is assigned (Dispatcher)
 5. Travel to source    — the vehicle drives a route (Pathfinding, Traffic)
 6. Loading         — physical transfer at the dock (Logistics, dock rate)
 7. Pickup          — custody moves: H_source -= x, R_source -= x, C_haul += x
 8. Custody         — goods in transit on a named vehicle (Logistics)
 9. Haul            — the vehicle drives to the destination (Pathfinding, Traffic)
10. Unloading       — physical transfer at the destination dock (Logistics, dock rate)
11. Delivery        — custody moves: C_haul -= x, H_dest += x
12. On-hand         — stock sits in the destination's accountable storage
13. Consumption     — the owning subsystem (Production, Needs) debits on-hand stock
```

Steps 6 and 10 (loading and unloading) are absent in the current substrate; freight
station cargo is a counter (`freight_station.rs`, `FreightStation.waiting_cargo`).
Steps 7 and 11 exist as debit/credit in `Market::advance_dispatches`. The target design
adds finite loading/unloading rates (`SPEC-LOGISTICS-011`) and dock power coupling.

## Reading path

1. [Resources](resources.md) — the catalogue and its granularity rule
2. [Requests](requests.md) — stated demand as a distinct state
3. [Allocation](allocation.md) — how Logistics selects a source
4. [Reservation](reservation.md) — the non-additive encumbrance
5. [Custody](custody.md) — conservation between pickup and delivery
6. [Storage](storage.md) — on-hand stock, capacity, and hoarding
7. [Production](production.md) — the run as a physical transformation
8. [Logistics](logistics.md) — the haul authority and its lifecycle
9. [Construction](construction.md) — building as production over time

## Authoritative documents

- [Charter 1.0](../../plan/charter-1.0.md) — binding scope (Resources and production; Transport and border)
- [Resources spec](../../reference/specifications/resources.md) — `SPEC-RESOURCES-001`…`006`
- [Production spec](../../reference/specifications/production.md) — `SPEC-PRODUCTION-001`…`009`
- [Logistics spec](../../reference/specifications/logistics.md) — `SPEC-LOGISTICS-001`…`011`
- [Construction spec](../../reference/specifications/construction.md) — `SPEC-CONSTRUCTION-001`…`008`
- [Trade spec](../../reference/specifications/trade.md) — `SPEC-TRADE-001`…`008`

## Related

- [Planned economy](../planned-economy/index.md)
- [Transport](../transport/index.md)
- [Infrastructure](../infrastructure/index.md)
- [Glossary](../../reference/glossary.md)
