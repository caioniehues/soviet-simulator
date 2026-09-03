# Authority

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** simulation
**Last verified:** 2026-08-28

Scope: **architecture hook**. The pattern is already established in the draft specifications
and the specification register's authority table.

## What this is

Every mutable field in the simulation has exactly one owning module. That module is the sole
writer; every other module references identifiers and results through the owner's interface. No
module copies, shadows, or independently mutates another module's ledger.

This prevents the class of defect where two systems both believe they own the same quantity,
custody record, or route state. When they disagree, the simulation has two truths for one fact.
The player sees a number that contradicts another number and cannot tell which is real.

## 1.0 requirement

The specification register (`docs/reference/specifications/README.md`) defines the authority
table for 1.0. Its key assignments:

| State or transition | Authoritative module |
|---|---|
| Durable demand and its unmet outcome | Requesting module (Needs, Production, or Trade) |
| Catalogue identity and on-hand stock | Resources |
| Fulfillment: allocation through delivery and return | Logistics |
| Vehicle identity, capacity, location, recovery | Vehicles |
| Road topology and parking-slot reservations | Roads |
| Industrial consumption and production | Production |
| Dwelling consumption and satisfaction | Needs |
| Customs clearance and rouble settlement | Trade |

Each draft specification names its authority and states what it references from neighbours. A
specification MUST NOT define a parallel authoritative copy of custody, route, fleet, pressure,
consumption, or settlement state
([spec register](../../reference/specifications/README.md)).

## Target design

The design proposes that the one-authority rule extends to every new domain:

- **One owning module per state transition** (design law 18). Cross-domain code references IDs
  and results; it does not mutate another domain's ledger.
- **Authoritative transitions with immutable IDs**: `DeliveryId`, `ProductionRunId`,
  `ElectricityAllocationId`, `WaterTransferId`, and so on. Each transition is keyed by an
  immutable identifier and applies exactly once. Retry of the same ID is a no-op
  (design law 19 — HYPOTHESIS; already present in
  [`SPEC-WATER-006`](../../reference/specifications/water.md#spec-water-006) and
  [`SPEC-ELECTRICITY-002`](../../reference/specifications/electricity.md#spec-electricity-002)).
- **Typed system contexts** replace the broad `&mut Simulation` access that systems currently
  take. Each system declares the resources it reads and writes. This makes the authority contract
  machine-checked rather than convention-enforced (design bible §13).

## Current substrate

The authority pattern is partially present. Each draft specification names its authority, and the
specification register defines the table. In code, however, systems take `&mut World, &mut Resources`
(broad mutable access) and `ParCommandBuffer::exec_ent` accepts `FnOnce(&mut Simulation)`,
which is the main cross-system mutation channel
(`simulation/src/utils/par_command_buffer.rs`). Typed contexts must first give deferred callbacks
a declared resource set before the authority contract can be enforced at compile time
(SYNTHESIS §3.10; Lane C2 §4.3).

Two live authority conflicts exist. Company drivers and market dispatch both attempt to deliver
goods for the same enterprise ([`LOG-SUB-006`](../../research/fact-sheets/wave1-logistics.md#log-sub-006--company-ownership-does-not-constrain-global-dispatch),
[`LOG-SUB-007`](../../research/fact-sheets/wave1-logistics.md#log-sub-007--old-company-delivery-and-new-market-freight-both-remain-live)).

## Research basis

The one-authority-per-transition principle is a standard invariant in distributed-systems
design: exactly-once semantics require a single writer for each piece of state. The
idempotent-transition pattern (apply once by key; retry is a no-op) is standard in
message-processing systems and is already adopted by the utility specifications in this
project.

## Related

- [Specification register](../../reference/specifications/README.md) — the 1.0 authority table.
- [Physical causality](physical-causality.md) — distinct states require distinct owners.
- [Reserves](reserves.md) — reserve classes sum to physical stock under one owner.
- [Design bible §2, §13](../../vision/design-bible.md) — laws 18–19 and typed contexts.
- [Architecture proposals](../../plan/proposals/sim-tick-phases.md) — typed contexts and
  phase order.
