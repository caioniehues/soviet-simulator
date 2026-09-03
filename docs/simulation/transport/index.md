# Transport

**Kind:** index
**Authority:** advisory
**Status:** draft
**Owner:** transport
**Last verified:** 2026-09-03

## What belongs here

This section describes movement: roads, pathfinding, traffic, vehicles, and rail. It does
not cover what is moved (see [physical economy](../physical-economy/index.md)) or the
infrastructure networks that do not carry vehicles (see
[infrastructure](../infrastructure/index.md)).

## Authority distinction

Six authorities govern transport. Each page names its authority; no page crosses into
another's ledger.

```text
Route       — Pathfinding translates an authorized trip into a lane sequence
Movement    — the vehicle physically traverses the route
Traffic     — concurrent movement creates observable capacity pressure
Vehicle     — a finite persistent identity used by Logistics
Haul        — the Logistics fulfillment lifecycle (pickup → custody → delivery)
Cargo custody — Logistics owns what the vehicle carries; the vehicle carries it
```

Route authorizes movement only; it does not transfer custody or satisfy a need
(`SPEC-PATHFINDING-005`). Traffic observes congestion but does not clear requests or settle
roubles (`SPEC-TRAFFIC-005`). The Haul and Cargo custody authorities live in
[logistics](../physical-economy/logistics.md).

## Reading path

1. [Roads](roads.md) — the Planner-authored physical network
2. [Pathfinding](pathfinding.md) — route as a derived state
3. [Traffic](traffic.md) — congestion as a scarcity signal
4. [Vehicles](vehicles.md) — finite persistent identities
5. [Freight rail](freight-rail.md) — the charter's minimal rail model
6. [Public transport (future)](public-transport-future.md) — Post-1.0 direction

## Authoritative documents

- [Charter 1.0](../../plan/charter-1.0.md) — Transport and border commitment
- [Roads spec](../../reference/specifications/roads.md) — `SPEC-ROADS-001`…`006`
- [Pathfinding spec](../../reference/specifications/pathfinding.md) — `SPEC-PATHFINDING-001`…`006`
- [Traffic spec](../../reference/specifications/traffic.md) — `SPEC-TRAFFIC-001`…`008`
- [Vehicles spec](../../reference/specifications/vehicles.md) — `SPEC-VEHICLES-001`…`006`

## Related

- [Physical economy](../physical-economy/index.md)
- [Infrastructure](../infrastructure/index.md)
- [Routing architecture](../../architecture/routing.md) (lead writes)
- [Glossary](../../reference/glossary.md)
