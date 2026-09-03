# Priorities

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** economy
**Last verified:** 2026-08-28

Scope: **1.0 candidate** — the logistics specification commits to deficit-first dispatch
([`SPEC-LOGISTICS-005`](../../reference/specifications/logistics.md#spec-logistics-005));
priority classes and inflation are design proposals.

## What this is

Priority decides where scarcity appears, not whether it exists (CONFIRMED — Kornai; Nove).
Copper to the Space Programme is copper not in radios, machine tools, or construction. The
Planner must see the displaced use. Priority is a reallocation instrument, never a supply
instrument.

## 1.0 requirement

The logistics specification commits to non-price dispatch ordering:

> Compatible demands are ordered by greatest normalized target deficit first, then meaningful
> route distance and a stable identity tie-break; money and price never participate.
> — [`SPEC-LOGISTICS-005`](../../reference/specifications/logistics.md#spec-logistics-005)

This is the 1.0 form of priority. It contains no plan-level priority class — only physical
deficit ordering. No specification commits to Planner-set priority classes.

## Target design

The design proposes (design bible section 5.8, HYPOTHESIS) that the Planner can assign
priority classes to enterprises or projects. Higher priority means earlier allocation.

**Priority inflation** is the risk: when everyone labels a request critical, priority means
nothing. Gosplan maintained lists of "especially important" consignments (Nove). The design
proposes constraining who assigns priority classes and exposing the share of activity running
under emergency status. If 40 percent of dispatches carry "urgent" status, the Planner knows
the system is in trouble.

### National-project priority loops

A national project receives housing, specialists, and freight priority. It performs — other
districts lose exactly those resources — queues, strain, and labour shortage appear elsewhere.
No "national project penalty" modifier is needed; the physical displacement is the cost
(design bible section 11). This is one instance of the general
[scarcity](../concepts/scarcity.md) concept.

## Current substrate

`make_trades` in `simulation/src/economy/market.rs:551-591` sorts potential trades by distance
(`sorder.pos.distance2(border.pos)`) using `OrderedFloat`. No plan priority, no request age,
no deficit-first ordering exists. The dispatch priority rule in SPEC-LOGISTICS-005
(deficit then distance then stable ID) is the target; the current implementation uses distance
only.

## Research basis

Priority as relocation of scarcity is a first-principles consequence of finite resources
(CONFIRMED — Kornai; Nove). Soviet priority allocation is well documented: defence and space
received preferential access to materials, equipment, and skilled labour at the direct expense
of consumer goods and agriculture.

## Related

- [Scarcity](../concepts/scarcity.md) — priority operates within scarcity, not against it.
- [Allocation](allocation.md) — how allocation clears without price.
- [Enterprise behavior](enterprise-behavior.md) — enterprises adapt to priority signals.
- [Reserves](reserves.md) — state and project reserves are priority instruments.
- [Logistics specification](../../reference/specifications/logistics.md).
- [Design bible section 5.8](../../vision/design-bible.md).
