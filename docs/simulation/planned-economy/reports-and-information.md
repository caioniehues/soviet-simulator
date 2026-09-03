# Reports and information

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** economy
**Last verified:** 2026-08-28

Scope: **1.0 candidate** — the charter commits to inspectable discrepancies
([`SPEC-PRODUCTION-009`](../../reference/specifications/production.md#spec-production-009));
the discrepancy inspector and provenance columns are design proposals.

## What this is

Every quantity the Planner sees has a provenance. Some values are measured directly (the
Planner inspects a warehouse). Some are reported by enterprises (the enterprise declares its
output). Some are computed from other values. The provenance determines how much the Planner
should trust the number.

The key distinctions:

| Quantity | What it means | Provenance |
|---|---|---|
| **Technical forecast** | What the recipe consumes per cycle | Known (recipe definition) |
| **Reported requirement** | What the enterprise says it needs | Reported (may be inflated) |
| **Allocated** | What the allocation system promised | Computed (allocation records) |
| **Received** | What was physically delivered | Measured (delivery records) |
| **Consumed** | What the recipe actually used | Measured (production records) |
| **On-hand** | What is physically in stock | Measured (physical inspection) |
| **Request age** | How long the request has been outstanding | Computed (queue records) |

A discrepancy between reported requirement and consumed is the Planner's primary signal
for hoarding. A discrepancy between reported production and physical stock change is the
signal for falsification.

## 1.0 requirement

The production specification requires:

> The Planner SHALL infer suspected deception from inspectable request, receipt, consumption,
> on-hand, surplus, and outstanding-request-age discrepancies.
> — [`SPEC-PRODUCTION-009`](../../reference/specifications/production.md#spec-production-009)

## Target design

The design proposes a discrepancy inspector (design bible section 16;
`docs/plan/proposals/causal-inspector.md`) that shows, per enterprise, per input:

```text
Input    | Forecast | Reported | Allocated | Received | Consumed | On-hand | Surplus | Age
Copper   | 100 t    | 135 t    | 120 t     | 108 t    | 96 t     | 47 t    | 12 t    | 3 d
```

Each line carries a provenance column: measured, reported, aggregated, observed, estimated, or
unknown (design bible section 4, section 13.20). The Planner sees that the enterprise reported
needing 135 tonnes, received 108, consumed 96, and has 47 on hand. The surplus is the hoarding
signal. The inflation (reported minus forecast) is the strategic-request signal. No label says
"dishonest" — the numbers tell the story.

### Plan-fulfilment falsification

An enterprise can report higher output than it physically produced (Lane A, M-03;
Harrison 2011, "Forging Success", documenting grain overstatement up to 53 percent). The
inspector detects this by comparing reported production against physical stock changes. This
is SPEC-PRODUCTION-009 applied to the output side.

## Current substrate

The building inspector (`native_app/src/gui/inspect/inspect_building.rs:150-267`) shows
workers, productivity, power, progress, and storage capital per item. It does not show
requested, consumed, reserved, in-transit, surplus, request age, or provenance.

`Market::requested()` (`simulation/src/economy/market.rs:77-78, 500-501`) is a public accessor
returning the requested quantity per soul per item. No code in `native_app/` calls it. Wiring
it into `inspect_building.rs` is approximately 30 lines — the single cheapest high-value change
in the project (Lane E, section 3).

No `PlannerSnapshot` exists. The UI reads `Simulation` state directly.

## Research basis

Plan-fulfilment falsification is CONFIRMED (Harrison 2011). The Planner's ability to detect
falsification depends on comparing multiple information sources — the four-realities model
applied to enterprise reporting (see [information](../concepts/information.md)).

## Related

- [Information](../concepts/information.md) — the four realities and provenance.
- [Enterprise behavior](enterprise-behavior.md) — request inflation as a strategic act.
- [Material balance](material-balance.md) — the identity that reveals discrepancies.
- [Plan cycle](plan-cycle.md) — reporting as a stage in the control loop.
- [Reliability and buffering](reliability-and-buffering.md) — credibility shapes reporting.
- [Production specification](../../reference/specifications/production.md).
- [Causal inspector proposal](../../plan/proposals/causal-inspector.md).
