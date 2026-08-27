# Crime

> Superseded by ../../reference/specifications/crime.md — provenance only.

**Status:** draft model (grounded in research)
**Phase:** 1
**Primary inspiration:** CS1 generation formula + physical-arrest path; W&R fed-prisoner grammar; black market is OURS
**Evidence:** see [research/services.md](../research/services.md) §D/§G/§J and [research/citizens.md](../research/citizens.md) (`GetCrimeRate` coupling).

> Crime, policing, and (later) black markets emerging from shortages. Crime is driven by neglect — unmet needs, unemployment, low wellbeing — never by a dice-roll aura.

## Purpose

Crime emerges from the confirmed CS1 social substrate (wellbeing↓ + unemployment↑ → crime↑), is cleared only by real officers travelling to arrest specific citizens who are then physically held and fed, and — uniquely here — chronic shortage breeds a black market that redistributes goods outside the plan.

## Draft model

### Generation (from CS1 — CONFIRMED, adopted nearly as-is)

CS1's crime generation is already physical/social, not monetary (research/services.md §D1/§D2, §J.5):

```
perCitizenCrime = min(rate(unemploymentLength), maxRate(wellbeing))     // Citizen.cs:1229-1265
  unemployment: 0→10, 1→15, 2→20, 3→25, 4→35, 5+→50
  wellbeing cap: VeryUnhappy→100 … VeryHappy→40
  known criminal: ~4× harder
buildingCrimeBuffer += randomized sum over occupants, +25% at night
buildingCrimeBuffer ≤ citizenCount × 100
```

We keep: unemployment + wellbeing drivers, the criminal-recidivism multiplier, the night multiplier, the per-building cap. We drop: CS1's "no crime until police unlocked" progression gate (§D2) — crime should exist from day one; policing is the response, not the enabler.

### Clearance: physical arrest only (CS1 prison-van path — CONFIRMED; patrol-debit dropped)

CS1 has two clearance channels (§D4): patrol cars *drain the crime buffer* on arrival (a coverage-style debit), and prison vans run `ArrestCriminals` — flag specific citizens `Arrested`, transport via `CriminalMove`, hold against `JailCapacity`. **We keep only the arrest path** (§J.6): an officer travels, arrests a specific citizen, and physically transports them to a real cell. The `PoliceDepartment` coverage field is dropped; deterrence, if any, comes from visible enforcement outcomes, not radius.

### Prisoners are fed residents (from W&R — CONFIRMED, adopted verbatim)

W&R prisons take food deliveries (`$STORAGE_DEMAND_PRISON`, dry + refrigerated) and carry the lowest `$QUALITY_OF_LIVING` in the game (0.50) — inmates are low-quality residents in the freight chain, not counters (§G2). Adopted: jails consume food via spec/logistics.md; an unsupplied prison starves its inmates, with political consequences (§J.7).

### Institutions (from W&R data — CONFIRMED shells)

W&R's justice chain: police (workers+profesors 1:1 — half the force university-educated), courts (profesor-judges), prison (worker-guards only), secret police (profesors + fuel only) (§G1). We adopt the staffed-shell grammar: each is a real staffed, fuelled building; court capacity throttles sentencing throughput between arrest and prison.

### Black market (OURS — confirmed no substrate in either game)

W&R has zero crime tokens in 488 files (§G3); CS1 has no shortage economy. The black market is entirely ours, built on confirmed pieces: chronic unmet needs (spec/needs.md aspirations) + goods sitting in state inventories (spec/logistics.md) → a parallel allocation channel that leaks inventory, satisfies needs unofficially, and generates crime + corruption pressure. Sketch: leak rate per warehouse scales with local shortage severity and inversely with enforcement presence.

### Data (draft)

```
Citizen += { crimePropensity; criminal: bool; arrested: bool }
Building += { crimeBuffer ≤ occupants×100 }
PoliceStation { staff; vehicles+fuel; cells }
Court { staff; caseThroughput }   Prison { guards; cells; foodDemand; qualityOfLiving }
BlackMarket { perDistrict leakRate(shortage, enforcement) }   // v2?
```

## Open questions
- Black market v1 scope: flavour statistic vs real parallel allocation draining state inventories? Lean: real but coarse (district-level leak rates), individual fences later.
- Political dimension: is "crime" also dissent (W&R's secret police exists in data but its mechanic is native/undocumented — §G3 Gaps)? Keep separate from property crime; couples to loyalty (spec/needs.md).
- Prison labour — period-authentic but sensitive; decision deferred, note W&R models prisons as residential-with-serve-rate only.
- Sentencing: does the court stage add anything mechanically in v1, or arrest→prison direct?

## Evidence log
| Claim | Evidence level | Source | Notes |
|---|---|---|---|
| Crime rises with unemployment length, capped by wellbeing; criminals ~4× | CONFIRMED | `Citizen.cs:1229-1265` | §D1 |
| Building crime buffer: randomized, +25% night, cap citizens×100 | CONFIRMED | `CommonBuildingAI.cs:2723-2770` | §D2 |
| CS1 crime doesn't exist until police unlocked | CONFIRMED | `CommonBuildingAI.cs:2744-2746` | §D2 — dropped by us |
| Patrol cars drain crime buffer (coverage-debit); prison vans physically arrest → jail | CONFIRMED | `PoliceCarAI.cs:382-436`, `PoliceStationAI.cs:180-199` | §D4 — arrest path kept, debit dropped |
| W&R: zero crime/arrest/sentence tokens in 488 .ini files | CONFIRMED (absence) | token census | §G3 — mechanics native |
| W&R prisons: food deliveries + lowest quality-of-living | CONFIRMED | `prison.ini:13-16` | §G2 — adopted |
| W&R justice staffing shapes (police 1:1 profesors, courts judges, prison guards) | CONFIRMED | `police.ini`, `court.ini`, `prison.ini` | §G1 |
| Black market as shortage-driven parallel allocation | OURS | — | no substrate in either game |

Evidence levels: CONFIRMED · OBSERVED · INFERRED · SPECULATIVE · OURS (see [spec/README](README.md)).

## Related
- ../research/services.md · ../research/citizens.md · ../spec/needs.md · ../spec/logistics.md · ../spec/households.md
