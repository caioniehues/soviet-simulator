# Simulation transitions standard

**Kind:** standard
**Authority:** operational (mirrors the pattern the draft specifications already require: SPEC-WATER-006, SPEC-ELECTRICITY-002, SPEC-LOGISTICS conservation rows)
**Status:** active
**Owner:** project lead
**Last verified:** 2026-08-28

An **authoritative transition** is a once-only change to owned state: a pickup, a delivery, a
production run, a utility allocation, an embedding into a Site, a death. The shared concept is the
transition, not a generic transaction framework.

## Rules

1. **Must:** a transition that can be retried or replayed carries an immutable ID (`DeliveryId`,
   `ProductionRunId`, `WaterTransferID`, `ElectricityAllocationID`, `EmbedId`, `DeathResultId`).
2. **Must:** it names source, destination and subject, and the explicit quantity or state delta.
3. **Must:** where conservation applies, validation and commit are atomic — all debits and credits
   or none. A failed run applies nothing.
4. **Must:** applying the same ID twice is a no-op, and a test proves it ([testing](testing.md)).
5. **Must:** one module applies it — its owner ([authority](authority.md)); other modules receive
   the result by ID.
6. **Must:** reservations are encumbrances, never additive quantity. Pickup of `x` is
   `H_source −= x; R_source −= x; C_haul += x`. Post-pickup cancellation keeps cargo in custody until
   return, reassignment or delivery — it never "releases" goods into nothing.
7. **Must:** a transition that cannot complete leaves a recoverable state with a reason
   ([failure model](failure-model.md)), never a silent drop.
8. **Should:** a transition emits a change-journal event and, where it matters to the player, a
   causal fact ([observability](observability.md)).

## The conservation identity every economy change is checked against

```text
source on-hand + destination on-hand + in-custody + embedded + declared consumed/sinks
  = initial + declared sources
```

The `ledger-invariant-checker` gate asks exactly this of any diff touching economy, market,
dispatch, storage or trade ([development cycle](../process/development-cycle.md)).

## Related

- [Determinism standard](determinism.md)
- [Custody (design)](../simulation/physical-economy/custody.md)
- [Invariants index](../reference/invariants.md)
- [Logistics specification](../reference/specifications/logistics.md)
