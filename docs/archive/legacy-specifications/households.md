# Households

> Superseded by ../../reference/specifications/households.md — provenance only.

**Status:** draft model (grounded in research)
**Phase:** 1
**Primary inspiration:** CS1 `CitizenUnit` mechanics (kept, generalized), W&R data grammar (kept), allocation policy OURS
**Evidence:** see [research/households.md](../research/households.md) for CS1-code and W&R-data sources.

> First-class household agents: members, residence (an allocated flat), shared goods pantry, aggregate needs, demographic events. The unit at which housing is allocated and at which big aspirations (car, bigger flat) live.

## Purpose

Group citizens into the unit that occupies dwellings and shares goods. Housing in a planned economy is **allocated, not bought**: the household is what sits on a waiting list, receives a flat, and vacates one. Household aggregate satisfaction feeds `spec/needs.md`; occupancy and queue length feed construction demand (`spec/construction.md`).

## The signature design choice

Both labs agree on the deep structure (**CONFIRMED/INFERRED**, research §F): *a dwelling is a fixed-capacity container people are assigned into* — neither game prices housing. CS1's "market" (homeless→vacancy offers matched by priority×distance) is a waiting list in disguise. **OURS:** make the queue explicit and player-visible, run by a housing office with policy levers. No RCI bar — queue length *is* the demand signal; per the one rule, nothing moves in because money exists.

## Draft model

### Structure (CS1-shaped, CONFIRMED substrate)

```
Household {                      // the simulation actor for demographic events (CS1-proven)
  members[≤cap]                  // small fixed-capacity buffer (CS1 hard-codes 5; keep a cap, not the number)
  dwellingRef                    // link into a building's flat table; 0 = in housing queue
  goods                          // shared pantry: buffer drains per step, refilled by shopping trips
  familyId; queueEntry?
}
Building flat table: flats × maxHouseholdSize   // authored per prefab
```

- CS1 chains 5-slot `CitizenUnit`s off buildings, **one unit per apartment** — the dwelling, not the person, is the capacity unit (CONFIRMED, `CitizenManager.cs:1145-1218`). We keep that two-level shape as ECS entities.
- **Capacity authored in flats, not people** (OURS): W&R authors an explicit per-prefab integer (3–220 people, same `$STORAGE` grammar as coal — CONFIRMED); we author `flats × max_household_size` so **overcrowding** (two households in one flat) is a representable state, not a bug.
- **Pantry** (CS1 CONFIRMED: start 200, −20/step, shop below 200, +100/trip): the terminal demand bucket of the distribution chain; per-household, not per-citizen. Trip mechanics in `spec/citizens.md`.

### Allocation: the housing queue (OURS)

Queue entry sources — exactly CS1's four confirmed event sources, redirected from its market:
1. New couples (partner matching among singles — keep the queue-of-singles, drop CS1's gender-as-offer-direction hack; its own birth gate ignores gender anyway).
2. Adult children leaving home (CS1: slots 2–4 continuously bid to move out).
3. Immigrants — **recruited by the plan** (replacing CS1's demand-gated fabrication at the map edge; W&R also makes immigration player-driven).
4. Households displaced by building failure/condemnation.

Housing office assigns flats by policy: priority classes, proximity to assigned workplace (couples with `spec/citizens.md` fixed jobs), family size vs. flat size, QoL fairness. **Eviction feeds the queue, never deletion** — CS1 silently deletes citizens who find no home (CONFIRMED), a simulation-integrity cop-out; hostels (W&R `$SUBTYPE_HOSTEL`, CONFIRMED) are the overflow tier, so housing shortage is a visible planning failure.

### Quality & comfort

- `qualityOfLiving` authored per prefab (W&R CONFIRMED: 0.60 khrushchyovka → 1.02 village house) + default-on heat/electricity requirements (W&R CONFIRMED-by-absence: dwellings never declare them; only non-residential opts out via `$HEATING_DISABLE`).
- Shortfalls drive **queue-jumping requests and emigration pressure**, not CS1-style abandonment-with-kill-percentage.
- Dwelling quality is an input to member satisfaction (`spec/needs.md` housingQuality); we reject CS1's dwelling-stamps-wealth-onto-residents direction as-is, but keep the insight that the flat shapes the household's status.

### Demographics (CS1 mechanics kept)

- **Birth:** couple present + free member slot + both adults → per-step chance, boosted by childcare access (CS1 CONFIRMED: 1/12 → 1/8) — a services-to-demographics hook worth keeping.
- **Fission:** adult children enter the housing queue (new household on assignment, not before).
- **Relocation:** whole-household, via the queue.
- Death frees a slot without dissolving the household (CS1 CONFIRMED).

## Open questions

- ~~Household or citizen as entity of record?~~ Settled: both — household is the demographic/housing actor, citizens the labour/needs actors.
- ~~Consumer inventory at home?~~ Settled: yes, household pantry (CS1 shape, W&R dwelling-stores-resources pattern).
- ~~Waiting-list priority: pure queue or weighted?~~ Settled: weighted by policy levers (priority classes, workplace proximity, size fit) — exact weights are a tuning question.
- Overcrowding mechanics: voluntary (multigenerational) vs. office-assigned doubling-up — same state, different wellbeing penalty?
- Max household size cap — CS1 proves a cap simplifies everything; 5? 6 (three-generation flat)?
- When a household outgrows its flat: automatic queue re-entry (aspiration → application) or player policy?
- Queue starvation: what stops the office from never housing low-priority households? (Fairness aging term?)

## Evidence log

| Claim | Evidence level | Source | Notes |
|---|---|---|---|
| CS1 household = 5-slot `CitizenUnit`, one per apartment | CONFIRMED | `CitizenUnit.cs:4-38`, `CitizenManager.cs:1145-1218` | homes 1 unit each; work/visit pooled ceil(n/5) |
| Household pantry drives shopping | CONFIRMED | `CitizenManager.cs:1180`, `ResidentAI.cs:509-522` | 200 start, −20/step, +100/trip |
| Partner matching = TransferManager market | CONFIRMED | `ResidentAI.cs:2225-2258` | gender = offer direction (dropped) |
| Birth gate: slots 0–1 couple, childcare boosts rate | CONFIRMED | `ResidentAI.cs:459-488` | 1/12 → 1/8 with childcare |
| Children continuously bid to move out | CONFIRMED | `ResidentAI.cs:2090-2151` | unmatched → citizen deleted |
| CS1 deletes citizens who find no home | CONFIRMED | `ResidentAI.cs:2434-2445` | rejected: evict into queue |
| RCI residential demand formula | CONFIRMED | `ZoneManager.cs:771-786` | replaced by visible queue |
| Vacancy advertised as typed transfer goods | CONFIRMED | `ResidentialBuildingAI.cs:610-622,832-868` | `Family0-3`/`Single0-3` commodities |
| Immigration = demand-gated household fabrication | CONFIRMED | `OutsideConnectionAI.cs:496-531,1100` | replaced by plan-recruited immigration |
| W&R dwelling = people-tank + QoL scalar | CONFIRMED | `civ_dedina_drevenica1.ini:21-24`, `hrusevka.ini:27-30` | `$STORAGE RESOURCE_TRANSPORT_PASSANGER n` |
| W&R heat/electricity default-on for dwellings | INFERRED | `$HEATING_DISABLE` ×101 on non-residential only | opt-out grammar |
| W&R: no household/flat/queue concept in data (488 ini) | CONFIRMED-absence | research §E | allocation policy native → OURS |
| Explicit policy-weighted housing queue | OURS | research §G3 | the spec's core proposal |

Evidence levels: CONFIRMED · INFERRED · SPECULATIVE · OURS (see [spec/README](README.md)).

## Related

- [spec/citizens.md](citizens.md) — members: labour, trips, lifecycle
- [spec/needs.md](needs.md) — satisfaction the household aggregates; housing need
- [spec/construction.md](construction.md) — where new flats come from; queue as demand signal
- [spec/logistics.md](logistics.md) — distribution chain the pantry terminates
- [research/households.md](../research/households.md) — primary-source research
