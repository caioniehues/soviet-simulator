# Allocation

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** economy
**Last verified:** 2026-08-28

| Scope | 1.0 binding |

## What this is

Allocation is the Logistics authority's first act after a request exists. It selects which
source will fill a given demand. In a planned economy, allocation is not a price auction. It
follows a target-stock policy: enterprises that are further below their target stock receive
goods before enterprises that are closer to it. Distance breaks ties. Money never participates.

## 1.0 requirement

`SPEC-LOGISTICS-005` — domestic dispatch and fulfillment use no money or price priority.

`SPEC-LOGISTICS-010` — Planner target-stock minima and maxima per storage bucket create
demand below minimum and supply above maximum. Compatible demands are ordered by greatest
normalized target deficit first, then meaningful route distance and a stable identity
tie-break; money and price never participate.

## Target design

The design proposes deficit-first allocation (PLAUSIBLE, bible §6.6): the allocation
authority computes `deficit = target_stock - on_hand` for each requestor, normalizes it, and
serves the largest deficit first. At equal deficit, the nearest source wins. A stable ID
tie-break makes the ordering deterministic. This is verbatim `SPEC-LOGISTICS-005`/line 62 of
the logistics spec.

## Current substrate

`Market::make_trades` (`simulation/src/economy/market.rs`) sorts potential trades by
distance (`OrderedFloat` over Euclidean distance between buyer and seller positions). There
is no deficit calculation, no target-stock concept, no request age, and no plan priority.
The sort is:

```
self.potential.sort_unstable_by_key(|(_, x)| OrderedFloat(*x));
```

This is pure distance sort. It serves the nearest buyer first, regardless of need severity.

## Open questions

- What normalized deficit scale preserves the required deficit-first ordering across
  resource kinds with different magnitudes?
- How do target-stock minima and maxima interact with the hoarding floor already
  present in `recipe_should_produce` (`goods_company.rs:43-46`)?

## Related

- [Requests](requests.md)
- [Reservation](reservation.md)
- [Logistics](logistics.md)
- [Logistics spec](../../reference/specifications/logistics.md#spec-logistics-010)
- [Glossary](../../reference/glossary.md)
