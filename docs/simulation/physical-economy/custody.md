# Custody

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** economy
**Last verified:** 2026-08-28

| Scope | 1.0 binding |

## What this is

Custody is the accountable holder of a physical quantity between pickup and delivery. When a
truck loads goods at a source, custody transfers from the source to the haul. When the truck
unloads at the destination, custody transfers from the haul to the destination. At every
moment, every unit of every good has exactly one custodian.

The custody conservation equation at pickup is: `H_source -= x; R_source -= x; C_haul += x`.
At delivery: `C_haul -= x; H_dest += x`. The sum of all on-hand, reserved, in-custody,
embedded, and consumed quantities for an item equals the total that has entered the system.
This is the [physical-causality](../concepts/physical-causality.md) rule applied to goods
movement.

Post-pickup cancellation keeps cargo in custody. The goods do not teleport back to the
source. The vehicle must physically return them, or the haul must be reassigned to a new
destination.

## 1.0 requirement

`SPEC-LOGISTICS-006` describes the pickup and cancellation transitions.

`SPEC-LOGISTICS-001` — a haul has one authoritative fulfillment authority. It records
allocation, reservation, pickup, in-transit custody, delivery, physical return, or release.

`SPEC-LOGISTICS-003` — a vehicle identity is finite and may hold only cargo compatible with
its declared capacity. Logistics SHALL own cargo identity, quantity, and custody.

## Target design

The design proposes that each vehicle carries an authoritative cargo record (PLAUSIBLE,
bible §6.4). The current substrate does not embody cargo on the vehicle; the market dispatch
tracks item and quantity as fields of the `Dispatch` struct, not of `Vehicle` or `RailWagon`.

LOG-SUB-005: `Vehicle` has no cargo, capacity, or owner field
(`simulation/src/transportation/vehicle.rs:34-45`). Cargo identity, quantity, and custody
remain fields of `Market::dispatches`.

## Current substrate

The ledger scenario tests in `simulation/src/tests/scenarios/ledger.rs` verify custody
conservation across seven scenarios:

- `scenario_ledger_exttrade_double_spend` — no double debit on export
- `scenario_ledger_remove_leak` — demolition does not leak stock
- `scenario_ledger_job_opening_reserve_leak` — job-opening reservation cleanup
- `scenario_dead_buyer_tosource_releases_reservation` — pre-pickup cancel releases only reservation
- `scenario_dead_seller_frees_its_truck` — seller demolition frees the truck without leaking
- `scenario_demolish_buyer_building_end_to_end_conserves` — buyer demolition conserves stock
- `scenario_dead_buyer_loading_goods_returned` — loading-phase cancel returns goods to seller

These tests prove that the current dispatch seam conserves quantity across the transitions
it covers. The gap is that custody is not embodied on the vehicle itself.

## Open questions

- When does authoritative cargo move from the `Dispatch` struct to `Vehicle`/`RailWagon`?
- What is the recovery path for a vehicle that is destroyed while carrying cargo?

## Related

- [Reservation](reservation.md)
- [Logistics](logistics.md)
- [Storage](storage.md)
- [Physical causality](../concepts/physical-causality.md)
- [Logistics spec](../../reference/specifications/logistics.md#spec-logistics-006)
