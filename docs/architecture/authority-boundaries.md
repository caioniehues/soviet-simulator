# Authority boundaries

**Kind:** architecture
**Authority:** advisory
**Status:** draft
**Owner:** architecture
**Last verified:** 2026-08-28

## The rule

Every mutable authoritative field has exactly one owning module. Cross-domain code passes typed
IDs, immutable results, service views and intents. No module duplicates another's ledger for
convenience. The [specification register](../reference/specifications/README.md) already assigns
1.0 authorities: Resources own catalogue and on-hand stock; Logistics own allocation, reservation,
pickup, custody, delivery and return; Vehicles own identity, capacity, location, depot and
recovery; Roads own topology and parking reservations; Pathfinding owns route request and result;
Traffic owns load, queue, pressure and stall; Production owns industrial consumption and output;
Needs own dwelling consumption; Trade owns customs and rouble settlement. The
[authority index](../reference/authority-index.md) is the table.

## Current substrate

- Systems receive `(&mut World, &mut Resources)` or `&mut Simulation`. `Resources` is a
  type-erased map with runtime borrow checking (`utils/resources.rs`); `resources.read::<T>()`
  panics on a conflicting mutable borrow.
- `ParCommandBuffer::exec_ent` closures take `FnOnce(&mut Simulation)` and run after every
  system. They are how a vehicle arrival releases a market reservation and frees a dispatcher slot.
  **This is the primary cross-system mutation channel**, and it has full access to everything.
- `Market` (`economy/market.rs`, ~1,500 lines) holds six responsibilities: order book, trade
  matching, dispatch lifecycle, retail claims, external trade, price calculation. Lane C2 §2.3
  traces each to its methods.
- The UI holds `Arc<RwLock<Simulation>>` and reads any resource directly.

## Target design

**Typed system contexts** replace `&mut Simulation`:

```rust
struct ProductionContext<'a> {
    resources: &'a ResourcesRead,
    labor: &'a LaborRead,
    power: &'a PowerServiceRead,
    out: &'a mut ProductionIntentBuffer,
}
```

The compiler then enforces what the spec register asserts. Two obstacles the design thread did not
address (Lane C2 §4.3–4.5):

1. **Deferred callbacks.** `exec_ent` closures capture arbitrary state. Either they keep
   `&mut Simulation` (the narrowing leaks) or each pre-declares its resource set (verbose). A
   middle path: replace closures with typed *intent enums* per entity type, applied by the owning
   module in the commit step. This is also what deterministic parallelism needs.
2. **Context explosion.** One context per system or system group; trait bounds get verbose. Start
   with one system (`electricity_flow_system`) and measure the ergonomics before generalising.

**Market decomposition.** Extract the dispatch lifecycle (`Dispatch`, `DispatchState`,
`advance_dispatches`, `release_tosource_truck`) into a `DispatchManager` the market delegates to;
extract retail claims next. Mechanical, behaviour-preserving, serialisation-identical — a two-
to-three-day step with no downstream dependency.

## Migration

1. `DispatchManager` extraction (independent).
2. One typed context for one system, wrapper-registered in the scheduler.
3. Intent enums for the deferred-callback path, one entity type at a time.
4. Every system on typed contexts; `&mut Simulation` reserved for `WorldCommand::apply`.

Invariant at every step: the round-trip determinism test passes; replay hashes unchanged.

## Open decisions

- Closure narrowing versus intent enums for the deferred path.
- Which resources the Planner view may *not* read (this is the same boundary seen from the UI side — [snapshots](snapshots.md)).

## Related

- [Authority (concept)](../simulation/concepts/authority.md)
- [Authority index](../reference/authority-index.md)
- [Parallelism](parallelism.md)
- [Authority standard](../engineering/authority.md)
- [Lane C2](../research/conversation-mining-2026-08-28/C2-architecture-vs-code.md)
