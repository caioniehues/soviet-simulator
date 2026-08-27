# Citizen Needs / Wants / Aspirations

> Superseded by ../../reference/specifications/needs.md — provenance only.

**Status:** draft model (grounded in research)
**Phase:** 1
**Primary inspiration:** CS1 citizen skeleton + W&R physical causality, pushed deeper (OURS)
**Evidence:** see [research/needs.md](../research/needs.md) for CS1-code and W&R-data sources.

> The root layer of the whole economy. The state does not create demand — citizens generate needs, wants and aspirations, and the planned economy tries to understand, prioritize and satisfy them. Production, housing, transport, education, retail, culture and political stability all hang from this.

## Purpose

Give every citizen a small set of drives whose (dis)satisfaction (a) produces real consequences, and (b) aggregates upward into all economic demand. No magic demand numbers — a figure like "car demand: 12,430" is only the sum of individual citizen desires.

## Three distinct systems

Grounded in the transcript's clarification and the two games' models.

### 1. Needs — absence has direct consequences
Satisfied by **physically delivered goods** or a **real trip to a stocked, staffed building**. Each is a per-citizen 0–1 satisfaction, decaying over time, refilled by consumption/visits.

| Need | Satisfied by | Source evidence |
|---|---|---|
| Food | food/meat goods delivered to shops → bought | W&R `$TYPE_SHOP`, `food`/`meat` resources — **CONFIRMED** |
| Warmth / heating | district heating reaching the home | W&R `$TYPE_HEATING_PLANT`, `heat` — **CONFIRMED** |
| Water | water network | W&R `$TYPE_WATER_PUMP`, `water` — **CONFIRMED** |
| Health | reachable, staffed hospital/clinic | W&R `$TYPE_HOSPITAL`; CS1 `m_health`→productivity — **CONFIRMED** |
| Housing (space + quality) | a `$TYPE_LIVING` unit with adequate `$QUALITY_OF_LIVING` | W&R — **CONFIRMED** |
| Rest / sleep | schedule allows it (short commute helps) | OURS (CS1 has no explicit sleep need) |

### 2. Wants — improve wellbeing, absence is tolerable
Map to W&R's leisure/culture building types; satisfaction is spatial and quality-weighted (pollution lowers it, nature/water raise it — W&R `$ATTRACTIVE_FACTOR_*`, **CONFIRMED**).

Consumer goods (clothes, appliances, electronics), alcohol/social (`$TYPE_PUB`), cinema (`$TYPE_KINO`), sport (`$TYPE_SPORT`), culture/museums (`$ATTRACTIVE_TYPE_MUSEUM`), spiritual (`$TYPE_CHURCH`), tourism/holiday (`$TYPE_HOTEL`).

### 3. Aspirations — future-oriented, persist for years (**OURS**)
Not in CS1 or W&R; built on their confirmed substrate. An aspiration is a slow-building pressure from *chronically* unmet needs/wants + status/preference, that becomes **economic demand** when it crosses a threshold.

```
Desire_car = MobilityGap + LeisureAccess + WorkAccess + FamilyNeeds
           + StatusAspiration + ComfortPreference
           - TransitQuality - CarCost - Scarcity
```
Examples: own a car (→ `$TYPE_CAR_DEALER`, **CONFIRMED W&R building**), a bigger flat, a university education, move to the capital, a specific imported good. Aspirations give citizens **memory** and make good urban planning *demonstrably* reduce car dependence (fix the tram line → mobility gap falls → car desire drops) rather than imposing "Soviets use transit" by fiat.

## The core loop (why this is the root)

```
citizen needs/wants/aspirations (per-citizen)
        ↓ aggregate
demand for goods & services & housing & mobility
        ↓
state planning sees shortages → sets production/construction targets
        ↓
factories need inputs + power + workers + machinery  (spec/production.md)
        ↓
more freight, more electricity, more industrial employment  (spec/logistics.md)
        ↓
which changes citizens' access, incomes, and satisfaction
        ↺ back to the top
```

## Consequence coupling (from CS1 — **CONFIRMED**)

Reuse CS1's confirmed derivations so needs have teeth:
- low **wellbeing** → higher crime, lower work probability (`GetCrimeRate`, `GetWorkProbability`)
- low **health** → lower work efficiency (`GetWorkEfficiency`)
- add **loyalty / political legitimacy** from W&R (`$MONUMENT_GOVERNMENT_LOYALTY`, broadcast/propaganda) as a satisfaction-driven meta-need.

## Data (draft)

```
CitizenNeeds {
  // 0..1 satisfaction, per band update frequency (see architecture/simulation-clock.md)
  food, warmth, water, health, housingSpace, housingQuality, rest   // needs
  goods, alcohol, culture, sport, spiritual, recreation             // wants
}
Aspiration { kind; pressure0to1; ageMonths; threshold }   // list per household/citizen
```
Needs update at **low** frequency; aspirations at **very-low** (months). See `architecture/simulation-clock.md`.

## Open questions
- Household vs individual: which needs resolve at household level (housing, appliances, car) vs individual (food, health, education)? Lean household for durables (matches CS1's household abstraction), individual for consumables.
- Do aspirations satisfy via a **waiting-list / allocation** system (period-authentic) or open purchase? Transcript implies allocation/queues.
- How do expectations rise over generations (1917 austerity → later mass-consumption)? A drifting baseline per era.

## Evidence log
| Claim | Evidence level | Source | Notes |
|---|---|---|---|
| Citizen has health/wellbeing/education/wealth + behaviour couplings | CONFIRMED | CS1 `Citizen.cs` | see research/needs.md §A |
| Needs map to dedicated building types; goods are typed & physically stored | CONFIRMED | W&R `buildings_types/*.ini` | research/needs.md §B |
| Construction consumes materials + labour-work, not money | CONFIRMED | W&R `$COST_RESOURCE_AUTO`,`$COST_WORK` | research/needs.md §B2 |
| Aspirations as endogenous, persistent demand generators | OURS | — | built on confirmed substrate |

## Related
- ../research/needs.md · ../spec/citizens.md · ../spec/households.md · ../spec/vehicles.md · ../spec/production.md · ../architecture/simulation-clock.md
