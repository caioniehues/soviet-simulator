# Education

**Status:** draft model (grounded in research)
**Phase:** 1
**Primary inspiration:** CS1 enrolment-as-job + W&R attendance chain, field abstraction stripped (OURS)
**Evidence:** see [research/services.md](../research/services.md) §B/§E/§J and [research/citizens.md](../research/citizens.md) §B3/§D2-D3.

> Education progression feeding worker qualification; an aspiration target for citizens (spec/needs.md). Schools are staffed buildings citizens physically attend — never radius auras.

## Purpose

Citizens gain qualification by physically attending a staffed school with a free seat, over real time; qualification feeds the labour market (spec/citizens.md worker bindings and overqualification cascade). The plan must build, staff and supply schools where the children actually are — there is no ambient "education field."

## Draft model

### Enrolment as a workplace assignment (from CS1 — CONFIRMED substrate)

CS1 stores a student's school **in the workplace slot** (`SetStudentplace` → `m_workBuilding`) and reuses the exact same commute machinery as a job (research/services.md §B2). We adopt this: *studying is a citizen's job*. One mechanism handles capacity, commute, absence and the study-vs-work exclusivity.

- A school offers `studentCapacity` seats (CS1: `StudentCount * 5/4` slots, serviceable count throttled by production rate — §B2).
- A seat is a labour binding: while enrolled, the citizen is unavailable to industry (matches W&R's school-as-a-life-phase, research/citizens.md §B3).
- **Dropped: CS1's coverage-field education grant** (§B3 — citizens graduating via field strength with no trip). This is the exact abstraction our rule rejects.

### The chain and its deliberate narrowing (from W&R — CONFIRMED)

W&R's chain: kindergarten → school → university (+ orphanage as child welfare), with throughput narrowing up the ladder — `$CITIZEN_ABLE_SERVE` 10 → 12 → 3 (research/services.md §E). Higher education is a slow serial pipeline; we keep that shape:

```
Tier          throughput   staff shape (W&R confirmed)
kindergarten  wide         workers only (no profesors — kindergarten.ini:30)
school        wide         workers 10 + profesors 15 (school.ini:25-26)
university    narrow (3)   workers 100 + profesors 100 (university_soviet.ini:58-59)
```

The two-tier staff split (workers/profesors) means universities require university-educated staff to run — a bootstrap chicken-and-egg (import specialists early, or send students abroad) that is good planned-economy texture (§J.2).

### Qualification ladder (from CS1 — CONFIRMED shape)

CS1's three ordered education levels map cleanly onto our worker qualification tiers (research/citizens.md §A2). Progression requires **seat-time**: N months enrolled at a staffed, operating facility → tier flag. No enrolment, no progression.

### Specialisation (OURS, seeded by W&R)

W&R universities carry `$SUBTYPE_SOVIET/MEDICAL/TECHNICAL` — inferred to gate which profesor type a graduate becomes (§E, SPECULATIVE in research). We adopt it deliberately: university choice yields a *specialisation* (medical → hospital staff, technical → engineers), making education planning a real allocation decision.

### Data (draft)

```
School { tier; studentCapacity; enrolled[]; staff: {workers, profesors}; throughputPerCycle }
Citizen += { educationTier; specialisation?; enrolledAt?; seatTimeMonths }
```

## Open questions
- ~~Qualification: single scalar ladder or per-profession specialisation?~~ → ladder for tiers + specialisation at university level (above).
- Adult re-education / night schools as a planner lever? (No substrate in either game — would be OURS.)
- Who chooses to study — citizen aspiration (spec/needs.md) or plan quota? Lean: plan sets seats, citizen aspiration fills them; unfilled seats are a legibility signal.
- Orphanage (W&R has one, physically fed via `$STORAGE_DEMAND_PRISON`) — model with households or with services?

## Evidence log
| Claim | Evidence level | Source | Notes |
|---|---|---|---|
| CS1 students enrol via workplace slot, commute like workers | CONFIRMED | `ResidentAI.cs:2376-2385`, `:1726` | research/services.md §B2 |
| CS1 coverage field grants education flags with no trip | CONFIRMED | `ResidentAI.cs:1259-1330` | §B3 — the abstraction we strip |
| CS1 school seat capacity throttled by production rate | CONFIRMED | `SchoolAI.cs:68`, `:135` | §B2 |
| W&R chain narrows: serve-rate 10→12→3 up the tiers | CONFIRMED | `kindergarten.ini:31`, `school.ini:27`, `university_*.ini` | §E |
| W&R kindergarten needs no profesors; university needs 100 | CONFIRMED | `kindergarten.ini:30`, `university_soviet.ini:58-59` | §E — bootstrap texture |
| W&R attendance/graduation timing is native (absent from data) | CONFIRMED (absence) | 488-file census | §E/§I |
| University subtype → graduate specialisation | SPECULATIVE | `$SUBTYPE_MEDICAL` etc. | §E; we adopt as OURS |
| Seat-time-only progression, no field channel | OURS | — | §J.1 |

Evidence levels: CONFIRMED · OBSERVED · INFERRED · SPECULATIVE · OURS (see [spec/README](README.md)).

## Related
- ../research/services.md · ../research/citizens.md · ../spec/citizens.md · ../spec/needs.md · ../spec/healthcare.md
