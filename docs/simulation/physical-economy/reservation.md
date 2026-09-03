# Reservation

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** economy
**Last verified:** 2026-09-03

| Scope | 1.0 — charter row Resources and production |

## What this is

A reservation is a non-additive encumbrance on source stock. It prevents the same goods from
being allocated to two different buyers. A reservation does not transfer stock — the goods
remain physically at the source until pickup. It is an accounting hold, not a physical
movement.

Pre-pickup cancellation releases the reservation without teleporting anything. Post-pickup
cancellation keeps the cargo in transit custody until physical return, reassignment, or
delivery.

## 1.0 requirement

`SPEC-LOGISTICS-006` — reservation is a non-additive encumbrance. For pickup quantity `x`,
the atomic transition is `H_source -= x`, `R_source -= x`, `C_haul += x`. Pre-pickup
cancellation changes only `R_source`; post-pickup cancellation preserves `C_haul` until
physical return, reassignment, or delivery.

`SPEC-RESOURCES-006` — pre-pickup cancellation releases only the non-additive reservation;
post-pickup cancellation retains accountable in-transit custody until physical return,
reassignment, or delivery.

## Current substrate

`SingleMarket.reserved` (`simulation/src/economy/market.rs:46-55`) is a
`BTreeMap<SoulID, u32>` that tracks reserved quantities per seller. When `make_trades`
matches a buyer to a seller, it increments the seller's reserved count
(`market.rs:604-609`; `job-opening` matches debit immediately instead, `market.rs:599-603`):

The reservation is released (decremented) when:
- The dispatch reaches `Loading` state and debits the seller's capital (`market.rs:912-917`)
- A retail claim expires (TTL) or the buyer despawns
- A pre-pickup cancellation occurs (scenario tests confirm this:
  `scenario_dead_buyer_tosource_releases_reservation`,
  `scenario_tosource_unpark_refusal_releases_the_truck`)

No teleport occurs on cancellation. The reservation is purely accounting. This is
consistent with the spec target.

## Open questions

- Does a partial reservation (only part of the order can be held) create a partial
  dispatch or a waiting state?

## Related

- [Allocation](allocation.md)
- [Custody](custody.md)
- [Logistics](logistics.md)
- [Logistics spec](../../reference/specifications/logistics.md#spec-logistics-006)
- [Resources spec](../../reference/specifications/resources.md#spec-resources-006)
