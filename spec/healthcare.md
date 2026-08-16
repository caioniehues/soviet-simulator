# Healthcare

**Status:** draft model (grounded in research)
**Phase:** 1
**Primary inspiration:** CS1 ambulance/bed loop + W&R fuelled-vehicle infrastructure, coverage field stripped (OURS)
**Evidence:** see [research/services.md](../research/services.md) §C/§F/§J and [research/citizens.md](../research/citizens.md) §B4.

> Health need satisfaction, medical services, physical supply of medicine. A sick citizen is a real productivity loss (CS1 health→efficiency coupling, confirmed in research/citizens.md).

## Purpose

Citizens get sick; treatment requires a reachable, staffed, fuelled, supplied hospital — an ambulance that physically arrives or a self-made trip, then a real bed until cured. Health couples back into work efficiency and lifespan. There is no passive "hospital nearby = healthier" radius.

## Draft model

### Sickness (from CS1 — CONFIRMED substrate)

Sickness is an acute per-citizen event (CS1 `Sick` flag when health collapses; causes include pollution and age — research/citizens.md §B4). Baseline health evolves from need satisfaction (food, warmth, water — spec/needs.md), **not** from proximity to a hospital.

- **Dropped: CS1's `HealthCare` coverage field** (research/services.md §C1 — passive health bonus by radius). Health improves only through actual treatment.

### Treatment: dispatch + bed + cure (from CS1 — CONFIRMED loop)

CS1's two-mode acute loop survives our rules intact (§C2/§C3):

1. Sick citizen posts a request; hospital dispatches an **ambulance** (`Sick`) or the citizen **self-travels** (`SickMove`).
2. Patient occupies one of `patientCapacity` **beds** (CS1: 100).
3. A per-tick cure rate — scaled by staffing/operation — clears the bed (CS1: probabilistic roll gated by production rate, `HospitalAI.cs:293-315`).

W&R supplies the physical infrastructure grammar (§F): hospitals are **vehicle bases** — `$WORKING_VEHICLES_NEEDED 7`, fuel storage (`$STORAGE_FUEL OIL 40`), parking bays, heliport. An unfuelled hospital dispatches nothing.

### Throughput, not aura (from W&R — CONFIRMED)

W&R hospital serve-rate is tiny (`$CITIZEN_ABLE_SERVE 3` — same as universities): treatment is a slow serial pipeline (§F). Combined: beds + staff + fuel + cure-rate make healthcare a resource-constrained throughput facility (§J.4).

### Medicine as a commodity (OURS, seeded by W&R grammar)

W&R data shows no medicine-supply token on hospitals (§F) — but its prisons/orphanages *are* physically fed via storage-demand buckets (§G2). We extend that grammar: hospitals carry a medicine storage demand fed by the freight chain (spec/logistics.md); an unsupplied hospital's cure rate degrades. Medicine becomes a real production/import chain (spec/resources.md, spec/trade.md).

### Staff loop

Hospitals need two-tier staff (W&R: workers 50 + profesors 50, `hospital.ini:48-50`); the profesor tier comes from medical universities (spec/education.md specialisation) — education → healthcare-labour is a closed loop in building types (§F).

### Consequences (from CS1 — CONFIRMED)

- Low health → lower work efficiency (`GetWorkEfficiency`, research/citizens.md).
- Untreated sickness → death; deathcare-as-logistics noted in research/citizens.md §B2 (own spec later or fold here — open).

### Data (draft)

```
Hospital { beds; occupied[]; staff: {workers, profesors}; vehicles: {ambulances, fuelStore};
           medicineStore; cureRatePerTick(staffing, medicine) }
Citizen += { sick: bool; sickSince; atHospital? }
```

## Open questions
- ~~Medicine as consumable commodity — confirmed in data?~~ → absent from W&R hospital data (CONFIRMED-absence); adopted as OURS via the storage-demand grammar.
- Preventive layer: does *anything* passive reduce sickness rate (clinics, sanitation via spec/sewage.md), or is prevention purely need-satisfaction? Lean: prevention = needs + environment (pollution), no service aura.
- Epidemics / public-health events in scope for v1?
- Deathcare: own spec or a section here?

## Evidence log
| Claim | Evidence level | Source | Notes |
|---|---|---|---|
| CS1 healthcare field raises health passively by radius | CONFIRMED | `HospitalAI.cs:255-261`, `ImmaterialResourceManager.cs:974-989` | §C1 — stripped |
| CS1 sickness → ambulance dispatch (`Sick`) or self-drive (`SickMove`) | CONFIRMED | `HospitalAI.cs:196-217`, `AmbulanceAI.cs:170-352` | §C2 |
| CS1 patients occupy beds; per-tick probabilistic cure gated by operation | CONFIRMED | `HospitalAI.cs:293-315` | §C3 |
| W&R hospitals are fuelled vehicle bases (7 vehicles, oil store, heliport) | CONFIRMED | `hospital.ini:33-54` | §F |
| W&R treatment throughput is 3 per cycle (serial pipeline) | CONFIRMED | `hospital.ini:48-50` | §F |
| W&R sickness parameters absent from data (mechanics native) | CONFIRMED (absence) | 488-file census | §F/§I |
| Medical university feeds hospital profesor staff | INFERRED | `$SUBTYPE_MEDICAL` | §F |
| Medicine as supplied commodity gating cure rate | OURS | W&R storage-demand grammar (§G2) | §J.4 extension |

Evidence levels: CONFIRMED · OBSERVED · INFERRED · SPECULATIVE · OURS (see [spec/README](README.md)).

## Related
- ../research/services.md · ../research/citizens.md · ../spec/needs.md · ../spec/logistics.md · ../spec/education.md · ../spec/trade.md
