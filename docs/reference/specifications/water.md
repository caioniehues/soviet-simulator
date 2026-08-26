# Water specification

**Kind:** specification
**Authority:** binding
**Status:** draft
**Owner:** utilities
**Last verified:** 2026-08-24

The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT, RECOMMENDED, NOT RECOMMENDED, MAY, and OPTIONAL in this document are to be interpreted as described in RFC 2119 and RFC 8174.

## Purpose

Water is a connected, finite-rate, quality-bearing utility network. It is tradable at the border but never cargo, vehicle custody, or freight-station stock. Water owns transfer progress and the border meter; Trade consumes its completed result for clearance and settlement.

## Invariants

- `SPEC-WATER-001` — Water SHALL be sole authority for topology, attachment, network quantity/quality, buffers, flow budget, transfer progress, and directional border meter. It MUST NOT use Logistics custody, a vehicle, or cargo stock.
- `SPEC-WATER-002` — A transfer progresses only over a connected compatible path and by no more than finite tick capacity. Disconnected or zero-capacity paths make no progress; partial flow is not delivery or clearance.
- `SPEC-WATER-003` — Source debit and destination credit are opposite-signed and equal after any named physical loss/treatment. Quality changes require a Water-owned process result and MUST NOT silently create quantity or potable Water.
- `SPEC-WATER-004` — Border transfer increments one directional meter and completes only after the whole order crosses a connected rate-limited path. Trade MAY then clear once; it MUST NOT advance flow or clear partial transfer.
- `SPEC-WATER-005` — Missing quantity, unsuitable quality, disconnected infrastructure, full storage, or insufficient rate leaves visible partial transfer/unmet request. It MUST NOT issue a tanker, settle roubles early, create Water, or end the plan.
- `SPEC-WATER-006` — Water SHALL apply a physical transfer at most once for immutable key `(WaterTransferID, monotonic leg/tick sequence)`. Accepted cumulative progress MUST NOT exceed ordered quantity `q`; for a border leg, the directional meter delta equals that accepted transfer delta exactly once. Replay is a no-op.

## Model and state

Water owns `WaterNodeID`, topology, endpoint compatibility, quantity, quality, buffer capacity, tick budget, `WaterTransferID`, monotonic leg/tick application sequence, progress, and meter reading. Buildings, Production, Needs, Sewage, and Trade reference results; none holds a duplicate water balance or meter. A border order references the transfer required by `SPEC-TRADE-007`/`008`; Trade alone owns clearance and rouble settlement.

## Failure behavior and observability

Finite capacity produces visible partial flow; blocked source, quality, buffer, or line leaves the request queued. Recovery needs physical connectivity, quantity, quality, and rate. The Planner can inspect path, endpoint quantities/quality, capacity, rate, progress, blockage age/reason, meter direction/cumulative reading, and linked Trade clearance.

## Acceptance evidence

All guards are **UNIMPLEMENTED** and block ratification. A zero-test command is failure; the current serial suite proves no target below.

| Evidence | Future guard command and observable assertion | Required red mutation | Player-facing proof |
|---|---|---|---|
| `EVID-WATER-001` | `cargo test -p simulation evid_water_connected_finite_rate_conservation -- --test-threads=1` — connected finite transfer conserves opposite-signed buffers, applies each `(WaterTransferID, leg/tick)` once, and keeps progress `<= q`. | Replay one application key, credit destination without source debit, exceed rate, or progress past `q`. | Inspected transfer key, progress, and partial-flow capture. |
| `EVID-WATER-002` | `cargo test -p simulation evid_water_disconnected_zero_capacity_no_progress -- --test-threads=1` — disconnected/zero-capacity transfer remains pending. | Advance absent a connected positive-capacity path. | Inspected blockage capture. |
| `EVID-WATER-003` | `cargo test -p simulation evid_water_border_completion_before_trade_clearance -- --test-threads=1` — full metered transfer precedes one Trade clearance, has no freight haul, and each directional meter delta equals its once-applied accepted border-transfer delta. | Replay/double a meter application, clear partial/order age, omit meter, or attach a haul. | Inspected transfer key, meter delta, and clearance capture. |
| `EVID-WATER-004` | `cargo test -p simulation evid_water_quality_process_conservation -- --test-threads=1` — a named Water-owned quality process accounts for every source debit, destination credit, loss, and quality result; it cannot create potable quantity/quality. | Upgrade quality/create quantity without a named process or erase a loss. | Inspected quality-process and conservation capture. |
| `EVID-WATER-005` | `cargo test -p simulation evid_water_unsuitable_quality_full_buffer_blocks -- --test-threads=1` — unsuitable quality and a full destination buffer retain an aged blocked transfer with unchanged incompatible/full state. | Deliver into an unsuitable/full endpoint or delete the blocked request. | Inspected quality, buffer, and blockage capture. |
| `EVID-WATER-006` | `cargo test -p simulation evid_water_quality_storage_authority -- --test-threads=1` — consumers reference Water results only. | Let a consumer alter Water quantity/quality. | Inspected authority links. |

## Substrate and decisions

No Water network/system exists in current building-kind and scheduler enumerations (`simulation/src/map/objects/building.rs:17-37`; `simulation/src/init.rs:52-70`). The physical-economy contracts prohibit Water cargo and require a Water-owned connected finite-rate metered transfer before Trade clearance (`docs/reference/specifications/resources.md:21-25,41-43`; `docs/reference/specifications/trade.md:52-64,77-82`). The [Wave 2 fact-sheet](../../research/fact-sheets/wave2-substrate.md#2c--utilities-electricity-water-sewage-heating-and-waste) records renderer water only. The [archived legacy Water specification](../../archive/legacy-specifications/water.md) is rewrite provenance; its tanker claim is rejected.

## Deferred behavior

Cell-level water, treatment tiers, tanker Water, and any vehicle/freight Water transport have no 1.0 acceptance criteria.

## Open questions

- Which quality classes and demand endpoints are 1.0?
- Which named physical loss/treatment processes are required?
