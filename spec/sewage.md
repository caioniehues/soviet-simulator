# Sewage

**Status:** draft model (grounded in research)
**Phase:** 1
**Primary inspiration:** W&R separate sewage network with treat-or-discharge choice
**Evidence:** see [research/utilities.md](../research/utilities.md) §B3/§D3/§G/§H.

> Physical sewage network — the return half of the water cycle, with treatment recovering second-grade water and discharge polluting the environment.

## Purpose

Sewage is produced wherever water is consumed and as a production byproduct; it must physically travel — by pipe or tanker — to treatment or discharge. Treatment costs chemicals and power but recovers usable water; discharge is free and poisons the map. The plan chooses.

## Draft model

### Generation (from W&R — CONFIRMED)

Sewage is a byproduct resource: households produce it from water use; industry declares it directly (`$PRODUCTION_SEWAGE_POLLUTION`, 9 files — e.g. nuclear cooling 0.67; research/utilities.md §D3, production.md §C1).

### Network (from W&R — CONFIRMED; CS1 rejected)

A **separate** pipe network (`$CONNECTION_SEWAGE_INPUT/OUTPUT` — distinct from water pipes, §D1), with typed pumps, treatment plants, discharge points and endstations. CS1 instead runs sewage as the water grid in reverse on shared conductivity (`TryDumpSewage` — §B3); rejected with the rest of the grid model.

### The treat-or-discharge choice (from W&R — CONFIRMED, adopted as the spec's core tension)

- **Treatment** (`sewage_treatment_big.ini`): sewage + chemicals + power + 20 workers → water capped at quality 0.85 (second-grade — feeds industry, not food factories; spec/water.md).
- **Discharge** (`sewage_discharge.ini`): a cheap dump endpoint — no inputs, but pollutes the environment.

Cheap-now-vs-clean-later is a genuine planning decision with physical consequences (pollution → health, spec/healthcare.md; attractiveness).

### Pipe OR tanker (from W&R — CONFIRMED)

Sewage is a transport class (`RESOURCE_TRANSPORT_SEWAGE`, 8 tanker vehicles, sewage cargo stations — §D3): off-grid buildings can be served by sewage truck at real logistics cost (spec/logistics.md).

### Overflow behaviour (OURS — native in W&R, Gaps)

What happens when treatment capacity is exceeded is not in either game's data. OURS: network backs up to the nearest discharge point if one exists (auto-pollute), otherwise producers' local buffers fill and water consumption gates shut (a blocked drain physically stops the tap).

### Data (draft)

```
Producer += { sewagePerWaterUnit | declaredSewageRate }
SewagePipe { capacity }   SewagePump { pushRate }
TreatmentPlant { in: sewage+chemicals+power+workers; out: water(q≤0.85) }
Discharge { rate; pollutionEmitted }
```

## Open questions
- ~~Pollution model local or shared?~~ → shared: discharge emits into the common pollution model (ground/water), spec TBD at architecture phase.
- Sewage:water ratio — fixed per consumer class, or track actual water drawn?
- Storm/rain load in scope? (Neither game models it; probably out for v1.)

## Evidence log
| Claim | Evidence level | Source | Notes |
|---|---|---|---|
| W&R: separate sewage pipe network with typed pump/treatment/discharge/endstation | CONFIRMED | `$CONNECTION_SEWAGE_*` census, §D1 table | §D |
| W&R: treatment = sewage+chemicals+power → water q≤0.85 | CONFIRMED | `sewage_treatment_big.ini:3-20` | §D3 |
| W&R: discharge endpoint = free dump | CONFIRMED | `sewage_discharge.ini:3-5` | §D3 |
| W&R: sewage as production byproduct | CONFIRMED | `$PRODUCTION_SEWAGE_POLLUTION` (9 files) | §D3, production.md §C1 |
| W&R: sewage tanker-haulable (pipe OR truck) | CONFIRMED | `RESOURCE_TRANSPORT_SEWAGE` vehicles/stations | §D3 |
| CS1: sewage = water grid in reverse, shared conductivity | CONFIRMED | `WaterManager.cs:1278` | §B3 — rejected |
| Overflow: back-up to discharge, else gate water consumption | OURS | — | Gaps |

Evidence levels: CONFIRMED · OBSERVED · INFERRED · SPECULATIVE · OURS (see [spec/README](README.md)).

## Related
- ../research/utilities.md · ../spec/water.md · ../spec/production.md · ../spec/logistics.md · ../spec/healthcare.md
