# Authority standard

**Kind:** standard
**Authority:** operational (the module table is the draft specification register's; binding on ratification)
**Status:** active
**Owner:** project lead
**Last verified:** 2026-08-28

## Rules

1. **Must:** every mutable authoritative field has exactly one owning module. The owner is the
   only code that writes it.
2. **Must:** cross-domain code passes typed IDs, immutable results, service views or intents. It
   never copies another module's ledger and never mutates it.
3. **Must:** a state transition (pickup, delivery, production run, allocation, death) is applied by
   its owner and identified by an immutable ID ([simulation transitions](simulation-transitions.md)).
4. **Must:** a new system or a new field names its owner in the [authority index](../reference/authority-index.md)
   before review. A field with two writers is a review send-back.
5. **Should** *(target)*: deferred callbacks (`ParCommandBuffer::exec_ent`) declare the resources
   they touch, or are replaced by typed intents applied by the owner. Today they take
   `&mut Simulation`; new code should not widen that channel.

## The 1.0 owners (specification register)

| State or transition | Owner |
|---|---|
| Durable demand and its unmet outcome | Needs, Production or Trade (the requester) |
| Catalogue identity and on-hand stock | Resources |
| Allocation, reservation, pickup, custody, delivery, physical return | Logistics |
| Vehicle identity, capacity, location, owner/depot, recovery | Vehicles |
| Road topology and parking-slot reservations | Roads |
| Route request and result | Pathfinding |
| Load, queue, pressure, stall | Traffic |
| Industrial consumption and production | Production |
| Dwelling consumption and satisfaction | Needs |
| Customs clearance and rouble settlement | Trade |
| Network topology, transfer and service result per utility | that utility (Electricity, Water, Sewage, Heating, Waste) |

## Check

Review asks, for every new write: which module owns this field? Is the writer that module? If a
second module needs the value, does it hold an ID or a result rather than a copy?

## Related

- [Authority (concept)](../simulation/concepts/authority.md)
- [Authority boundaries (architecture)](../architecture/authority-boundaries.md)
- [Authority index](../reference/authority-index.md)
- [Specification register](../reference/specifications/README.md)
