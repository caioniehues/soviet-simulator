# Housing — persistent non-price queue

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** society
**Last verified:** 2026-09-03

| Scope | Label |
|---|---|
| Housing queue | 1.0 — charter row *Households and citizens* |
| Kommunalka / separate-flat tiers | Post-1.0 hook |
| Housing as labour infrastructure | Post-1.0 hook |

Residence assignment with a housing queue and an observable housing shortage is 1.0; the tiers
and the labour-infrastructure feedback are deferred hooks
([ADR-0001](../../decisions/0001-households-and-utilities-are-1.0-scope.md)).

## What this is

Housing is a persistent non-price queue, not a purchase. A household enters the queue when its
per-capita living space falls below the sanitary norm. It waits — years, sometimes a decade.
Housing determines where workers live, how long they commute, and whether they stay or leave.
It is labour infrastructure: an enterprise that cannot house its workers cannot keep them.

## 1.0 requirement

The charter requires persistent individual identities and observable state, which implies that
housing shortage is a visible, inspectable condition. The draft households specification
(`SPEC-HOUSEHOLDS-002`) says a queued, displaced, or overcrowded household remains observable;
shortage never deletes people or ends the plan. `SPEC-HOUSEHOLDS-006` says housing priority is
a non-price Planner policy.

No separate housing specification exists yet. The housing queue is part of the household
contract.

## Target design

### The queue — CONFIRMED (Andrusz 1984; Morton 1980; UPI 1988)

The design proposes a persistent ordered structure (B1 §3b):

```text
HousingQueue:
  entries: BTreeMap<HousingQueueKey, HouseholdID>
  positions: HashMap<HouseholdID, HousingQueueKey>   -- O(1) position lookup

HousingQueueKey:
  channel: HousingChannel
  priority: HousingPriority     -- from policy: family size, displacement, queue age, veteran
  registration_tick: u64        -- tie-break: first registered wins
```

Eligibility: a household enters when per-capita living space falls below the sanitary norm
(policy-set; historically ~9 m² per person, varying by city).

### Three historical channels — CONFIRMED

| Channel | Wait time | Mechanism |
|---|---|---|
| Enterprise | ~2 years | Tied to employment; fastest; lost on job change |
| Municipal | 10+ years (Moscow/Leningrad) | Administered by city soviet; slowest |
| Cooperative | Variable | Partial self-financing; Khrushchev-era onward |

12–36 % of families were registered on waiting lists by city (1988 UPI data). 18 % of all
Soviet families had waited 10+ years.

### Two tiers: kommunalka and separate flat — CONFIRMED (B1-MISSED-06)

Up to 80 % of Moscow residents lived in communal apartments (kommunalki) until the mid-1960s.
The transition from a shared room in a kommunalka to a separate flat in a khrushchyovka was
the most important quality-of-life improvement for Soviet households.

The design proposes at least two housing tiers: shared housing (kommunalka / dormitory) and
separate flat. Moving up is the queue transition. A household in a kommunalka has reduced
privacy, shared kitchen, and conflict — modelled as reduced discretionary time and lower
satisfaction (B1-MISSED-06). The tiers are Post-1.0 (ADR-0001); 1.0 needs only residence
assignment and the queue.

### Displacement

When a household is displaced (building demolished, enterprise closure), it re-enters the
queue with a displacement priority bonus but keeps its original registration tick. This
prevents displacement from resetting the wait.

### Housing as labour infrastructure — CONFIRMED (Bater 1980; Feshbach)

Housing was the primary cause of voluntary labour turnover (tekuchest'). Enterprises without
housing could not retain workers. Enterprises competed for workers through housing allocation —
the enterprise housing channel was fast precisely because it was a recruitment instrument.

The monotown feedback loop (B1-04, CONFIRMED): housing shortage → overcrowding → worker
departure → production shortfall → inability to build housing → worse shortage. A plant of
8,000 workers requires a settlement of ~20,000 people (B1-03, PLAUSIBLE; 2.5× is the high
side of range given 50–55 % labour-force participation rates).

### Mikrorayon completeness — CONFIRMED (B1-06)

A housing plan can succeed numerically while daily life fails. A mikrorayon — the standard
Soviet residential district — requires schools, shops, clinics, childcare, heating, and
transit to function. A numerically complete district where services lag is a failure the
Planner must detect. Housing queues are one domain instance of
[queues under scarcity](../concepts/queues.md).

## Current substrate

`Home` (`simulation/src/souls/desire/home.rs:8-11`) is `{ house: BuildingID, last_score: f32 }`
with a constant score of 0.2. `spawn_human` (`simulation/src/souls/human.rs:237-278`) creates
one human per empty house and immediately assigns ownership. There is no queue, no waiting, no
eligibility check, no sanitary norm, no channel, no displacement, no kommunalka, no household.

## Research basis

Soviet housing allocation is one of the best-documented features of the system. Andrusz (1984)
and Morton (1980) provide the queue mechanics. CIA reports document housing as a labour
recruitment instrument (six reports listed in B2 §8: CIA-RDP79R01141A000800070002-3, "Housing
in the USSR", July 1957, and five others). The UPI Archives (1988) document the queue scale.
Feshbach links housing to turnover in "Labor constraints in the five-year plan" (JEC, 1973).

## Open questions

- Should the Planner have direct control over propiska (residence registration), or should it
  be automatic based on housing availability? (B1 §6.2.)
- Should the housing model distinguish kommunalka and separate flat with a queue transition?
  This adds gameplay depth but also data-structure complexity. (B1 §6.4.)
- Which non-price queue attributes and tie-breaks are required for 1.0?
  (`households.md:131`.)

## Related

- [Households](households.md)
- [Labour](labor.md)
- [Workplaces](workplaces.md)
- [Time](time.md)
- [Migration](migration.md)
- [Households specification](../../reference/specifications/households.md)
- [Queues concept](../concepts/queues.md)
