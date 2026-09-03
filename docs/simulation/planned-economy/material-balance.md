# Material balance

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** economy
**Last verified:** 2026-08-28

Scope: **1.0 candidate** — the identity is a physical invariant the ledger must preserve;
the UI layout and drill-down are design proposals.

## What this is

The material balance is the accounting identity that connects every stock, flow, and holder
in the economy:

```text
opening + production + arrivals − consumption − departures = closing
```

This is equivalent to the standard Gosplan material balance equation:
`Q_t + Q_{t-1} + M_t = ID_t + FD_t + X_t`
(production + inventories + imports = inter-industry demand + final demand + exports)
(CONFIRMED — Lane A, A-10).

Per resource, the balance decomposes into:

- **Physical stock** — producer, operating, safety/reserve, state/project, in transit.
- **Period flow** — produced, imported, consumed, allocated-not-delivered.
- **Information** — technical forecast, reported demand, outstanding request age, suspected
  discrepancy.

Every line drills to holders, hauls, consumers, or reports. A non-zero residual means something
teleported — a bug, and a visible one. The material balance is the `ledger-invariant-checker`'s
standing question.

## 1.0 requirement

No specification commits to the material-balance UI. The underlying invariant is implied by
the conservation rules across multiple specifications:

- [`SPEC-RESOURCES-005`](../../reference/specifications/resources.md#spec-resources-005) — no
  silent deletion or creation.
- [`SPEC-LOGISTICS-006`](../../reference/specifications/logistics.md#spec-logistics-006) —
  pickup conservation: `H_source -= x; R_source -= x; C_haul += x`.
- [`SPEC-PRODUCTION-007`](../../reference/specifications/production.md#spec-production-007) —
  atomic recipe run: inputs debited and outputs credited together, or none commit.
- [`SPEC-TRADE-006`](../../reference/specifications/trade.md#spec-trade-006) — clearance
  conservation: signed quantity and rouble legs apply once.

Together, these specifications define a system where the material-balance identity should hold.
A violation is a bug.

## Target design

The design proposes (Lane A §3f; design bible §5.14) a per-item, per-period structure:

```text
per item, per period:
  opening_stock: i64       // total across all holders at period start
  domestic_production: i64 // recipe_act outputs this period
  arrivals: i64            // imports cleared this period
  consumption: i64         // recipe_act inputs + retail consumption
  departures: i64          // exports cleared this period
  closing_stock: i64       // total across all holders at period end
```

At period boundaries, the identity is verified: `opening + production + arrivals - consumption -
departures == closing`. A discrepancy is a ledger bug.

### The UI layout

The material-balance inspector shows one row per resource. Each term (opening, production,
arrivals, consumption, departures, closing) is a clickable link that drills to the holders,
hauls, or consumers that compose it. A non-zero residual is highlighted.

Each line carries a provenance column: the value is either measured (physical stock sweep),
reported (enterprise declarations), or computed (difference). The Planner sees which terms
are trustworthy and which depend on institutional reports
(see [reports and information](reports-and-information.md)).

## Current substrate

`EcoStats` (`simulation/src/economy/ecostats.rs:48-52`) tracks ring-buffered histories of
trade volumes (exports, imports, internal trade) per item at four frequency levels (10 min,
1 hr, 10 hr, 50 hr). It records `past_ring_items` (quantity) and `past_ring_money` per level.

It does **not** track production, consumption, or stock levels — only trade matches. The
`handle_trade` method (`ecostats.rs:79-93`) accumulates trade quantities and money deltas into
the ring buffer. No production or consumption event is recorded here.

The material-balance identity cannot be computed from current state (Lane A §3f). There is no
opening/closing stock snapshot, no per-period production counter, and no per-period consumption
counter.

## Research basis

The material balance is the fundamental accounting tool of Gosplan planning. Gosplan maintained
material balances for approximately 1,943 product categories (CONFIRMED — Lane A, A-01,
citing Material Balance Planning, Wikipedia). The identity is a physical conservation law
and does not depend on any economic theory.

## Related

- [Physical causality](../concepts/physical-causality.md) — the conservation rules the balance
  expresses.
- [Plan cycle](plan-cycle.md) — the balance is verified at period boundaries.
- [Reports and information](reports-and-information.md) — provenance of balance terms.
- [Reserves](reserves.md) — stock terms decompose into reserve classes.
- [Enterprise behavior](enterprise-behavior.md) — discrepancies reveal hoarding.
- [Design bible §5.14](../../vision/design-bible.md) — the material-balance identity.
