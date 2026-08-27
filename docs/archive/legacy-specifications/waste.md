# Waste

> Superseded by ../../reference/specifications/waste.md — provenance only.

**Status:** draft model (grounded in research)
**Phase:** 1
**Primary inspiration:** W&R typed circular-economy waste loop; CS1 confirms the hauled-not-piped shape
**Evidence:** see [research/utilities.md](../research/utilities.md) §F/§G/§H.

> Physical waste collection and processing — the one "utility" that moves by vehicle, not by network. Both source games agree on that (CONFIRMED both sides).

## Purpose

Buildings fill real containers with **typed** waste; a garbage-office fleet hauls it to sorting, recycling, incineration, treatment or landfill. Recycling returns named materials to the economy; incineration yields power or heat; only residue landfills. Waste management is a production decision, not a chore.

## Draft model

### Typed waste resources (from W&R — CONFIRMED, adopted)

Waste is a transport class (`RESOURCE_TRANSPORT_WASTE`) carrying named sub-resources: `waste_mixed, waste_bio, waste_steel, waste_aluminium, waste_plastic, waste_toxic, waste_gravel, waste_burnable, waste_other` (research/utilities.md §F2). Producers declare which types they emit (`$RESOURCE_SOURCE_WASTE_*`). CS1's single abstract `Garbage` material (§F1) is the impoverished contrast.

### Generation and containers (from W&R — CONFIRMED)

Residents/buildings deposit into container stands — small buffers with optional per-type sorting bins (`containerstand_big.ini` §F2); industry uses large trash storage. Sorting at source (separate bins) vs mixed collection is a planner choice with downstream consequences (mixed needs a separation plant first).

### Collection: one dispatch system (from W&R — CONFIRMED, adopted)

The garbage office (`$TYPE_GARBAGE_OFFICE`) is structurally identical to the distribution office: truck pool + fuel + no policy tokens (§F2). Adopted per §H: **reuse the spec/logistics.md deficit-driven dispatcher** — a full container is a source-job like any other, under the same class gate and fleet limits. No second dispatch system. CS1 corroborates the shape: garbage is TransferManager offers, tuned distance-first (priority rises as bins fill, `5E-07` distance multiplier — §F1).

### Processing: three fates (from W&R — CONFIRMED chain)

1. **Sort + recycle** — separation plant: `waste_mixed` → extraction yields (gravel 0.65, steel 0.87, aluminium 0.85, plastic 0.6); then recycling plants: `waste_steel 1.20 → steel 0.65` etc. Recovered scrap re-enters the material economy (spec/resources.md, spec/production.md).
2. **Incinerate** — for electricity (`incinerator_powerplant`: waste 3.0 → eletric 33, per-type burn ratios, ash + high pollution) or district heat (`incinerator_heat`: waste 2.5 → heat 450) — couplings to spec/electricity.md / spec/heating.md.
3. **Treat toxic / landfill** — toxic neutralised with chemicals (high pollution); landfills are plain `$TYPE_STORAGE` holding waste forever. CS1 agrees: a landfill never empties and can't be bulldozed while full (§F1).

### Consequences of neglect (CS1 substrate + OURS)

CS1: bins overflow visibly, offer priority escalates (§F1). OURS coupling: uncollected waste lowers local attractiveness and raises sickness rate (spec/healthcare.md) — physical, local, no city-wide "garbage meter."

### Data (draft)

```
WasteType { mixed | bio | steel | aluminium | plastic | toxic | gravel | burnable | other }
Container { capacity; perTypeBins?; fillLevel[type] }
GarbageOffice → logistics dispatch office (fleet, fuel, class gate WASTE)
SeparationPlant { in: mixed; extractionYield[type] }   RecyclingPlant { in: waste_X; out: material }
Incinerator { in: waste (perType burnRatio); out: electricity|heat + ash; pollution }
Landfill { store; permanent }
```

## Open questions
- ~~Own scheduling or reuse logistics dispatch?~~ → reuse the one dispatcher (both games + §H agree).
- ~~Incineration→energy coupling?~~ → yes, both variants (power and heat), from W&R.
- Collection routes: pure deficit-dispatch may thrash for many small containers — batch a truck's round over multiple containers (route-per-trip)? Flagged for the logistics prototype.
- Ash/residue: model as final `waste_gravel`-like stream to landfill, or delete?

## Evidence log
| Claim | Evidence level | Source | Notes |
|---|---|---|---|
| Both games haul waste by vehicle, not grid/pipe | CONFIRMED | `LandfillSiteAI.cs` + `$TYPE_GARBAGE_OFFICE` | §F |
| W&R: typed waste sub-resources + per-source emission tokens | CONFIRMED | 488-file census (§F2 counts) | §F2 |
| W&R: containers with optional per-type sorting bins | CONFIRMED | `containerstand_big.ini:3-11` | §F2 |
| W&R: garbage office = distribution office (fleet+fuel, no policy) | CONFIRMED (+absence) | `technical_services.ini:5-42` | §F2 |
| W&R: separation yields, recycling back to named materials | CONFIRMED | `waste_generalseparation.ini:16-34`, `waste_steelrecycling.ini:14-16` | §F2 |
| W&R: incineration → electricity or district heat, per-type burn ratios | CONFIRMED | `incinerator_powerplant.ini`, `incinerator_heat.ini` | §F2 |
| CS1: garbage via TransferManager offers, distance-first, priority ~ fill | CONFIRMED | `CommonBuildingAI.cs:2244-2261`, `TransferManager.cs:1036` | §F1 |
| CS1: landfill stores forever, incinerator consumes (+power/material) | CONFIRMED | `LandfillSiteAI.cs:26-38, 399-430` | §F1 |
| Uncollected waste → local attractiveness/sickness coupling | OURS | — | §H |

Evidence levels: CONFIRMED · OBSERVED · INFERRED · SPECULATIVE · OURS (see [spec/README](README.md)).

## Related
- ../research/utilities.md · ../spec/logistics.md · ../spec/resources.md · ../spec/electricity.md · ../spec/heating.md · ../spec/healthcare.md
