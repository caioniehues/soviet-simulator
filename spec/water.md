# Water

**Status:** draft model (grounded in research)
**Phase:** 1
**Primary inspiration:** W&R quality-graded pipe network; CS1 grid rejected
**Evidence:** see [research/utilities.md](../research/utilities.md) §B/§D/§G/§H.

> Physical water supply network: sources, treatment, pipes, consumers. A citizen need (spec/needs.md) and a quality-gated industrial input (spec/production.md).

## Purpose

Water flows from real sources through real pipes to homes and factories, carrying a **quality grade** end-to-end. Sensitive consumers demand minimum quality; shortage and contamination propagate as unmet need and gated production, not as a red overlay.

## Draft model

### Sources, treatment, storage (from W&R — CONFIRMED)

W&R's typed chain (research/utilities.md §D1–D2): wells/surface intakes (`$TYPE_MINE_WATER`, whole-building output — the per-worker exception, production.md §A2), treatment plants (dirty `usagewater` + chemicals + power → clean water capped at `$OUTWATER_MAX_QUALITY 0.99`), reservoirs/switches as in-line storage and junctions, substations (`$TYPE_WATER_ENDSTATION`) as leaf buffers. Adopted: a real hydraulic topology with explicit storage/junction/leaf hardware.

### Quality grading (from W&R — CONFIRMED, adopted as core)

Water carries a 0–1 quality; consumers gate on `$CONSUMPTION_WATER_REQUIRED_QUALITY` (animal farm 0.93, food factory 0.97, nuclear cooling 0.60 — §D2). Sewage treated back to water caps at 0.85 vs fresh treatment's 0.99 — recovered water is second-grade by construction (§D3). Adopted verbatim: quality degrades through use, is recovered at a cost, and gates sensitive consumers (food, hospitals). CS1's single pollution byte (§B2) only gestures at this.

### Pipe network (W&R grammar; CS1 grid rejected)

Explicit `$CONNECTION_WATERPIPE_INPUT/OUTPUT` pipe network (§D1). **Dropped:** CS1's shared 256-cell grid where a pipe is just conductivity (§B1–B2) — though note CS1 water is its *most* physical grid (requires a real pipe segment within 95 m, pollution travels with flow). Capacity/loss: not in W&R data (Gaps) — OURS, as with electricity: pipes have throughput; pumps push over distance/elevation.

### Pipe OR truck (from W&R — CONFIRMED, adopted)

Water is also a transport-class cargo (`RESOURCE_TRANSPORT_WATER`, tanker-haulable via water stations — §D3): buildings off the pipe grid can be served by tanker at real logistics cost (spec/logistics.md). CS1 has the same idea only as disaster water trucks (§B5).

### Routing policy

W&R exposes routing-policy flags (`$WATER_NOT_USE_FOR_INDUSTRY_SUBSTATIONS` — INFERRED: reserve residential substations from industrial draw, §D4). Adopted as planner policy: per-substation consumer-class reservation.

### Data (draft)

```
WaterSource { outputRate; sourceQuality }   Treatment { in: usagewater+chemicals+power; out: water(q≤0.99) }
Pipe { capacity }   Pump { pushRate }   Reservoir { store }   Substation { leafBuffer; classPolicy }
Consumer += { drawPerSecond; requiredQuality }
Water flow state: { rate; quality }
```

## Open questions
- ~~Shared layer with sewage/heating?~~ → separate networks (W&R model); CS1's shared grid was an implementation economy, not a design.
- ~~Water quality in scope?~~ → yes, core (confirmed W&R substrate).
- Elevation/pressure: model real head (pumps required uphill) or abstract pump-hop capacity? Lean abstract in v1.
- Source contamination: couple industrial pollution to source quality (nuclear plant's `$PRODUCTION_SEWAGE_POLLUTION 0.67` shows the W&R direction)?

## Evidence log
| Claim | Evidence level | Source | Notes |
|---|---|---|---|
| W&R: typed source/treatment/reservoir/substation/switch chain, explicit pipe points | CONFIRMED | `water_well_big.ini:5-17`, `water_treatment_big.ini:4-20`, §D1 census | §D |
| W&R: water quality graded end-to-end; consumers gate on required quality | CONFIRMED | `$OUTWATER_MAX_QUALITY`, `$CONSUMPTION_WATER_REQUIRED_QUALITY` (13 files) | §D2 |
| W&R: sewage-recovered water caps at 0.85 vs 0.99 fresh | CONFIRMED | `sewage_treatment_big.ini:3-20` | §D3 |
| W&R: water also tanker-haulable (pipe OR truck) | CONFIRMED | `RESOURCE_TRANSPORT_WATER` vehicles, water stations | §D3, logistics.md §B2 |
| W&R: substation routing-policy flags | CONFIRMED tokens, INFERRED meaning | `$WATER_NOT_USE_FOR_INDUSTRY_SUBSTATIONS` (6) | §D4 |
| CS1: water/sewage/heating share one 256-cell grid, separate pulse layers | CONFIRMED | `WaterManager.cs:18-44, 970-976` | §B1 — rejected |
| CS1: water fetch needs real pipe within 95 m; pollution byte travels | CONFIRMED | `WaterManager.cs:1394-1487` | §B2 |
| Pipe capacity + pump-hop model; per-substation class policy | OURS | — | §H |

Evidence levels: CONFIRMED · OBSERVED · INFERRED · SPECULATIVE · OURS (see [spec/README](README.md)).

## Related
- ../research/utilities.md · ../spec/sewage.md · ../spec/needs.md · ../spec/production.md · ../spec/logistics.md
