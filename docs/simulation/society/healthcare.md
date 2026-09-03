# Healthcare — health as production of future capacity

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** society
**Last verified:** 2026-09-03

| Scope | Label |
|---|---|
| Healthcare service chain | 1.0 — charter row *Agriculture and services* |
| Medicine as import-only resource | 1.0 — charter row *Resources and production* |
| Alcohol as health/time/productivity sink | Post-1.0, data-justified |

## What this is

Health is not a generic welfare bonus. It is the production of future labour capacity. A
healthy worker contributes more hours over a longer career. A sick worker misses shifts and
reduces productivity; in the target design, severe illness may contribute to a future
Citizens-owned death outcome (see [Demography](demography.md)). Healthcare investment is a
labour-force investment — every rouble of imported Medicine that cures a worker returns
future production hours.

## 1.0 requirement

The charter commits to "healthcare." The draft healthcare specification
(`SPEC-HEALTHCARE-001` through `SPEC-HEALTHCARE-006`) defines the contract:

- A care request persists through missing capacity, Medicine, staff, or route
  (`SPEC-HEALTHCARE-001`).
- Treatment requiring Medicine consumes only compatible on-hand Medicine after physical import
  clearance and delivery (`SPEC-HEALTHCARE-003`). Medicine is import-only in 1.0.
- Care requires finite declared staffed capacity and physical arrival
  (`SPEC-HEALTHCARE-004`). No timer, coverage radius, or payment substitutes for capacity.
- Non-price triage orders scarce care (`SPEC-HEALTHCARE-005`).
- Each Medicine treatment is keyed to one `CareRequestID` and accepts one `ConsumptionID`
  at most once (`SPEC-HEALTHCARE-006`).

## Target design

### Health as future capacity — CONFIRMED (B1-32; B2-17)

The design proposes that health outcomes affect citizen participation: illness reduces work
hours, extends recovery time, and in severe cases contributes to a Citizens-owned death
outcome (target design; Healthcare never performs lifecycle mutation). The healthcare
service chain restores capacity.

CIA tracked Soviet health decline as an economic problem: male life expectancy fell from 67
(1964) to 62 (1980); death rate rose 49 % from 6.9/1,000 (1964) to 10.3/1,000 (1981). The
Soviet government withheld age-specific mortality data after the mid-1970s, which CIA noted
"probably betrayed their embarrassment."

### Life-expectancy calibration — from B2 §3 only

| Parameter | Value | Source |
|---|---|---|
| Male life expectancy | 67 (1964) → 62 (1980) | CIA health data |
| Female life expectancy | ~74 (static) | CIA health data |
| Infant mortality | 24.7/1,000 (1970) → 25.4/1,000 (1986); +35 % since 1971 | Soviet data cited by CIA |
| Death rate | 6.9/1,000 (1964) → 10.3/1,000 (1981); +49 % | CIA analysis |

### Alcohol — Post-1.0, data-justified, sensitive

Alcohol is the single largest non-structural drag on Soviet labour productivity. CIA data
(1986, CIA-RDP87T00787R000200200003-0, "Gorbachev's Campaign Against Alcohol") documents:

| Indicator | Value | Source |
|---|---|---|
| Cost to the economy | ~10 % of national income | CIA 1986 |
| Managers reporting chronic alcoholism problems | 60 % (of those supervising 25+ people) | Emigre survey of 3,000 |
| Males drinking on the job | 20 % | Soviet survey cited by CIA |
| Male workforce chronically drunk | 37 % (1982) vs 11 % (1925) | CIA 1982 society report |
| Hospitalisation from alcohol | 50 % of all hospitalisations | CIA 1986 |
| Absenteeism decrease after 1985 campaign | -33 % industry; -40 % construction | CIA 1986 |
| Vodka/liquor production cut (1985 campaign) | -30 to -40 % | CIA 1986 |

Alcohol is a triple-threat resource:
- **Time sink** — drinking and recovery consume hours.
- **Health sink** — reduces future labour capacity; the largest single cause of Soviet male
  excess mortality.
- **Informal currency** — used for under-the-table transactions, repairs, farm work. A barter
  medium outside the formal economy.

It is also a Planner policy lever: an anti-alcohol campaign reduces absenteeism but causes
sugar shortages (diverted to moonshine) and removes a comfort that diverts public frustrations.

CIA noted the regime's dilemma: alcohol provided comfort to the population, diverted
frustrations into apolitical channels, and brought substantial state revenue.

The data justify a first-class system. The topic requires careful design — it is sensitive but
historically central. The design labels it Post-1.0 to defer the implementation question, not
to dismiss the evidence.

## Current substrate

No healthcare type, capacity queue, or health decision exists. `BuildingKind`
(`simulation/src/map/objects/building.rs:17-24`) has no healthcare variant. Human decisions
(`simulation/src/souls/human.rs:127-230`) enumerate only Home, Work, and Food. Medicine is a
target import-only resource; no healthcare service consumes it.

The existing food flow settles the live retail claim at eat-time — seller debited,
reservation released, `last_ate` advanced only on success
(`simulation/src/souls/desire/buyfood.rs:157-168`;
`simulation/src/economy/market.rs:480-491`). The healthcare physical chain is greenfield:
no facility, queue, or Medicine consumption exists to repeat or violate anything yet.

## Open questions

- Which healthcare facility types and treatment capacities are in 1.0?
  (`healthcare.md:105`.)
- Which health outcomes affect citizen participation without giving Healthcare ownership of
  Citizen lifecycle? (`healthcare.md:106-107`.)
- Which care cases require Medicine, and what compatible quantity is consumed per treatment?
  (`healthcare.md:108`.)
- How explicit should the alcohol mechanic be? CIA data supports it as a first-class system.
  (B1 §6.3; B2 §7.2.)

## Related

- [Citizens](citizens.md)
- [Time](time.md)
- [Labour](labor.md)
- [Education](education.md)
- [Healthcare specification](../../reference/specifications/healthcare.md)
- [Demography](demography.md)
