# Observatory

**Kind:** architecture
**Authority:** advisory
**Status:** draft
**Owner:** architecture
**Last verified:** 2026-08-28

The physical simulation owns truth. A derived layer — the observatory — maintains what the
Planner, the notifications and the inspector need, incrementally, from the
[change journal](change-journal.md).

## Current substrate

`EcoStats` ring buffers of trade volumes per item; the building inspector reads `Market` capital
directly. No material balance (production, consumption and stock are not tracked), no labour or
service balance, no discrepancy analysis, no forecast.

## Target design

**Derived state the observatory maintains:** material balances (the identity opening +
production + arrivals − consumption − departures = closing, per resource, drillable); labour
balances; service pressure and queue metrics; enterprise discrepancy (reported requirement vs
received vs consumed vs on-hand vs request age); exposure indexes; causal explanations; Planner
indicators; reserves in natural units.

**Provenance is part of the value.** Every observatory output carries how it is known — measured,
reported by an enterprise, aggregated statistically, observed via an institution, estimated, or
unknown — because the Planner snapshot must never expose hidden physical truth for convenience
([snapshots](snapshots.md), [information](../simulation/concepts/information.md)).

**Implementation choice.** Start hand-rolled: dirty flags and recompute-on-read over journal
events. Salsa (v0.28, used by rust-analyzer) is viable for a per-tick derived layer — ~200 µs for
1,000 inputs and 100 queries — but its database model does not compose easily with `Resources`,
and its value appears only past roughly twenty query types (Lane C1 §3.1–3.2). Differential
Dataflow has no shipped-game precedent and requires the `timely` runtime; research only.

**Forecast (Gosplan computer).** Determinism enables branching a headless simulation from a
snapshot to compare plans. Crucial rule: forecasts consume *Planner-visible, reported* state, so a
mathematically feasible plan can still fail physically. `Simulation` is `Serialize + Deserialize`
and headless ticking exists, so `fork()` is possible today at roughly 100 ms per clone (Lane C2
§2.10) — too slow for frequent use, fine for a deliberate "what if".

**Feasibility instrument (LP/MILP).** "Given reported capacities, recipes, stocks and declared
transport limits, is this plan materially feasible?" An instrument, never the player. `good_lp`
with `microlp` (pure Rust, MILP since 0.6) or HiGHS (C++, faster on large problems) — an open
conflict.

## Migration

1. Material balance from one journal event kind.
2. Discrepancy view for one enterprise, with provenance labels.
3. Reserves-in-natural-units for coal and one utility.
4. `Simulation::fork()` behind a debug tool.
5. LP feasibility as a developer tool before any player-facing use.

## Open decisions

- Salsa versus hand-rolled (decide by query-graph complexity when it exists).
- `microlp` versus HiGHS (Windows CI build cost versus solver speed).
- Which resources the Planner may not see at all.

## Related

- [Change journal](change-journal.md)
- [Snapshots](snapshots.md)
- [Causality](causality.md)
- [Material balance (design)](../simulation/planned-economy/material-balance.md)
- [Causal inspector proposal](../plan/proposals/causal-inspector.md)
- [Rust crates research](../research/engineering/rust-architecture-crates.md)
