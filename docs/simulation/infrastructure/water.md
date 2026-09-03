# Water

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** infrastructure
**Last verified:** 2026-09-03

| Scope | 1.0 — charter row *Utilities* ([ADR-0001](../../decisions/0001-households-and-utilities-are-1.0-scope.md)) |

## What this is

Water is a connected, finite-rate, quality-bearing utility network. It is tradable at the
border but never cargo, never vehicle custody, and never freight-station stock. A connected
pipe does not mean adequate pressure: a building on the ninth floor may have no water because
the static head is too low.

Water is distinct from electricity. Connection alone does not guarantee service — pressure,
quality, and finite rate matter.

## 1.0 requirement

`SPEC-WATER-001` — Water SHALL be sole authority for topology, attachment, network
quantity/quality, buffers, flow budget, transfer progress, and directional border meter.
It MUST NOT use Logistics custody, a vehicle, or cargo stock.

`SPEC-WATER-002` — transfer progresses only over a connected compatible path at finite
tick capacity.

`SPEC-WATER-003` — source debit and destination credit are opposite-signed and equal
after named physical loss/treatment. Quality changes require a named Water process.

`SPEC-WATER-004` — border transfer increments one directional meter and completes only
after the whole order crosses. Trade MAY then clear once.

`SPEC-WATER-005` — missing quantity, unsuitable quality, disconnected infrastructure,
full storage, or insufficient rate leaves visible partial transfer/unmet request.

`SPEC-WATER-006` — idempotent transfer by `(WaterTransferID, monotonic leg/tick sequence)`.

## Target design

Static head and tank storage are inside 1.0, not deferred refinements: they are the mechanism
that distinguishes water from electricity
([ADR-0001](../../decisions/0001-households-and-utilities-are-1.0-scope.md)).

Tree-based static head solver (PLAUSIBLE, D §3.5): the cheapest model that gives the
"connected but no pressure" distinction. Full GGA (EPANET Newton iteration) is overkill
for a game.

Algorithm:
1. Topological sort from sources
2. For each node in order: `head = parent_head - pipe_head_loss * pipe_length - elevation_diff`
3. If `head < min_pressure_for_floor(building)`: curtailed
4. Finite capacity: flow through pipe capped at pipe capacity

The Planner sees: a building on floor 9 has no water because head is too low. A pump station
is needed. Pump power couples to electricity — if power is curtailed, the pump stops, and
water stops.

Tank delay: water towers buffer supply against demand variation, adding minutes of inertia to
the water network. The tank drains before service fails, and that delay is 1.0 gameplay
(mechanism PLAUSIBLE; the scope commitment is ADR-0001).

## Current substrate

No water network or system exists. No building kind, no registered system, no data structure
(`simulation/src/map/objects/building.rs:17-37`, `simulation/src/init.rs:52-70`). The
[Wave 2 fact-sheet](../../research/fact-sheets/wave2-substrate.md) records renderer water
only. This is entirely greenfield.

## Research basis

EPANET (Rossman 2000) uses the Global Gradient Algorithm for steady-state hydraulic
equations. For a game, a tree-network static head calculation suffices — `O(n)` by
topological sort, no Newton iteration (PLAUSIBLE, D §3.5).

W&R reference: water is modelled by binary connectivity with quality thresholds
(`$CONNECTION_WATERPIPE_INPUT/OUTPUT`, `$CONSUMPTION_WATER_REQUIRED_QUALITY`). W&R does NOT
model pressure. The proposed model exceeds W&R by adding a head calculation.

## Open questions

- Which quality classes and demand endpoints are 1.0?
- Which pump and tank endpoints does 1.0 need? (Whether head is required at all is settled:
  ADR-0001 puts static head and tank storage in 1.0.)

## Related

- [Electricity](electricity.md)
- [Sewage](sewage.md)
- [Network architecture](network-architecture.md)
- [Water spec](../../reference/specifications/water.md)
- [Phase lag](../concepts/phase-lag.md)
