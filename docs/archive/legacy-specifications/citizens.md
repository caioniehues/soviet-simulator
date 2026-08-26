# Citizens

**Status:** draft model (grounded in research)
**Phase:** 1
**Primary inspiration:** CS1 citizen struct + lifecycle (kept), CS1 job market (replaced by planning), W&R fluid labour (rejected, with one borrowed escape valve)
**Evidence:** see [research/citizens.md](../research/citizens.md) for CS1-code and W&R-data sources.

> Individual persistent citizen identities: identity, life stage, education, employment (a **fixed workplace**), health, needs, current trip. The labour side of the physical economy — every worker slot filled is a citizen who lives somewhere, commutes, and consumes.

## Purpose

Make labour a physical, planned resource. A factory does not "hire from the ether": each worker slot is bound to a specific citizen with a home, a commute, an education level and a health state. Citizen state feeds `spec/needs.md` (satisfaction) and `spec/production.md` (staffing/efficiency); citizen trips feed `spec/logistics.md` and `spec/vehicles.md`.

## The signature design choice

The two research labs sit at opposite poles (**CONFIRMED** both, see research §F):

- **CS1:** persistent 30-byte citizen struct with a fixed `m_workBuilding` binding, found via a `TransferManager` labour market (`Worker0..3` channels, one per education tier) — but **no commute feasibility check**, and tenure is auto-severed at every age-band boundary.
- **W&R:** zero per-citizen data-side state (CONFIRMED-absence over 488 `.ini`); workers are sourced per shift like a fluid, and construction offices literally declare `$RESOURCE_SOURCE_WORKERS` — labour as dispatchable cargo.

**OURS:** take CS1's data shape and binding, replace its market with a labour-allocation plan, and reject W&R's fluid labour — except for its construction-office pattern, which we keep as a *labour-pool office* for surge work (construction, harvest) that never breaks fixed assignments.

## Draft model

### Identity & state (CS1-shaped, CONFIRMED substrate)

```
Citizen {                       // flat ECS array, value struct
  flags                         // location(2b), student, sick, dead, needs-flags, streaks
  homeBuilding, workBuilding    // single fixed refs; school reuses workBuilding + Student flag
  visitBuilding, instanceRef    // transient trip state
  age                           // coarse ticks; AgeGroup thresholds à la CS1 15/45/90/180/240
  health, wellbeing             // 0..100, recomputed from physical inputs
  education                     // ordered tiers (start with 2, W&R worker/profesor split; schema allows 4)
  householdRef                  // see spec/households.md
}
```

The rendered pawn is a separate transient entity, allocated only while travelling (both games agree — CS1 `CitizenInstance`, **CONFIRMED**).

### Labour allocation (OURS, replacing CS1's market)

- Workplaces declare **tiered vacancies** (CS1 `workPlaces0..3` schema; two tiers at first).
- A **planning pass** assigns citizens → vacancies subject to: minimum education per slot, **commute feasibility** (the check CS1 lacks — research §C1), and plan priorities.
- **Overqualification cascade kept as a visible policy lever** (CS1 CONFIRMED mechanic): a graduate may fill a lower slot, never the reverse; doing so costs wellbeing (CS1 already raises happiness thresholds with education).
- Tenure persists until reassignment (plan decision or citizen request), death, or building loss — **not** auto-severed by age bands (we decouple tenure from CS1's graduation severance).
- **Labour-pool office** (from W&R `$RESOURCE_SOURCE_WORKERS`, CONFIRMED): dispatches unassigned/temporary workers to short-lived demands without touching fixed assignments.

### Education (attendance-based pipeline)

- **School as workplace assignment** (CS1's `SetStudentplace` trick, CONFIRMED): a student seat is a planned allocation like a job — unifies commutes, capacity, and labour-vs-study trade-offs.
- **Reject CS1's ambient education field** (coverage-granted diplomas, CONFIRMED mechanic) — violates physicality. Education requires attended capacity; W&R's `$CITIZEN_ABLE_SERVE` throughput (kindergarten 10, school 12, university 3 — CONFIRMED) is the data shape: higher education is deliberately a slow pipeline.

### Health & sickness

- Health recomputed from **physical inputs only** (CS1 `UpdateHealth` shape, CONFIRMED: water/sewage, pollution, garbage, care coverage vs. age-phase requirement — money never appears). Swap immaterial coverage fields for real service capacity where feasible.
- **Sickness is a service event, not a timer** (CS1, CONFIRMED): low-health streak → sick roll → citizen posts a hospital-transport need; resolution consumes hospital capacity. Sick citizens' economic life freezes.

### Efficiency coupling (the socialist feedback loop)

CS1 CONFIRMED tables kept, made sharper by fixed workplaces: `workEfficiency(health)` 10–100% and `workProbability(wellbeing)` feed the *specific slot* in the *specific plant* — neglecting people physically reduces output (`spec/production.md` per-worker model consumes this).

### Trips

- **Commute:** time-of-day probability curve (CS1 shape, CONFIRMED) is acceptable for v1; shift schedules are an open question.
- **Shopping:** keep the household goods-buffer loop (drain → need flag → visit → **debit shop stock on arrival**). Fix CS1's teleport bug: it credits the pantry on *match*, before the trip (CONFIRMED) — we debit/credit only on physical arrival. Replace CS1's 8 random `Shopping..H` shards with real need categories mapped to our resource ontology (`spec/needs.md`, `spec/resources.md`).

### Lifecycle

Coarse age ticks with threshold graduation (CS1 shape): child → student → worker → pensioner; death by age window widened by poor health + small accident chance (CONFIRMED formulas as tuning priors). Family/birth mechanics live in `spec/households.md`.

## Open questions

- ~~Self-directed job search vs. planner-directed?~~ Settled: planner-directed allocation with citizen-initiated *requests*; labour-pool office for surge.
- ~~Education in-sim or statistical?~~ Settled: attendance-based, throughput-limited pipeline.
- Shift schedules vs. CS1's probabilistic attendance curve — needed for "night shift" planning texture, or v2?
- How many education tiers at launch — 2 (W&R) or 4 (CS1)? Schema supports 4; start 2.
- Commute-feasibility definition for the allocator: max travel time? mode-dependent? recomputed when the network changes?
- ECS budget: is the ~30-byte struct + separate transient pawn enough at 100k+ citizens? (Prototype candidate.)
- Do pensioners/children generate non-school trips in v1?

## Evidence log

| Claim | Evidence level | Source | Notes |
|---|---|---|---|
| CS1 citizen = persistent struct w/ fixed `m_workBuilding` | CONFIRMED | `Citizen.cs:253-279` | single ushort ref; school reuses it |
| CS1 job market: `Worker0..3` channels by education tier | CONFIRMED | `ResidentAI.cs:801-886`, `CommonBuildingAI.cs:3615-3757` | priority×distance, no commute check |
| Overqualification cascade (min-education per slot) | CONFIRMED | `CommonBuildingAI.cs:3643-3646` | educated overflow shrinks lower-tier demand |
| Tenure severed at age bands 15/45/90/180 | CONFIRMED | `ResidentAI.cs:620-710` | we reject auto-severance |
| health→efficiency, wellbeing→attendance | CONFIRMED | `Citizen.cs:1162-1227` | 10–100% per individual |
| Health from physical inputs, sickness = service event | CONFIRMED | `ResidentAI.cs:888-1085` | money never appears |
| Ambient education field (coverage grants diplomas) | CONFIRMED | `ResidentAI.cs:1259-1330` | rejected: violates physicality |
| Shopping credits pantry on match (teleport) | CONFIRMED | `ResidentAI.cs:2394-2417` | rejected: debit on arrival |
| W&R: no per-citizen state in data (488 ini census) | CONFIRMED-absence | research §E | algorithms native |
| W&R labour as dispatchable resource | CONFIRMED | `construction_office.ini:44` `$RESOURCE_SOURCE_WORKERS` | borrowed as labour-pool office only |
| W&R two-tier staffing worker/profesor | CONFIRMED | `school.ini:2-5` etc. | our launch tier count |
| W&R per-shift sourcing radius behaviour | INFERRED | gameplay, no data surface | experiments could confirm |
| Fixed-workplace planned allocation | OURS | research §G1 | the spec's core proposal |

Evidence levels: CONFIRMED · INFERRED · SPECULATIVE · OURS (see [spec/README](README.md)).

## Related

- [spec/households.md](households.md) — household struct, housing allocation, family formation
- [spec/needs.md](needs.md) — satisfaction model consuming citizen state
- [spec/production.md](production.md) — worker slots, per-worker output
- [spec/logistics.md](../spec/logistics.md) — the freight chain shopping terminates
- [research/citizens.md](../research/citizens.md) — primary-source research
