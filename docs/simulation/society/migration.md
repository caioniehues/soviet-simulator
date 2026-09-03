# Migration — propiska as THE eligibility gate

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** society
**Last verified:** 2026-08-28

| Scope | Label |
|---|---|
| Propiska | Post-1.0 |
| Limitchiki | Post-1.0 |
| Komandirovki | Post-1.0 |
| Regional supply tiers | Post-1.0 |
| Graduate assignment | 1.0 candidate (via education) |

All subsections here are Post-1.0 or candidate. The charter says nothing about migration;
propiska is absent from all draft specifications.

## What this is

Migration in the Soviet Union was not free movement. It was administratively controlled
through the propiska (residence registration) system. A citizen without propiska for a city
could not enter the housing queue, hold permanent employment, or access city services.
Migration was driven by housing opportunities, jobs, climate, services, and administrative
eligibility.

## Target design

### Propiska — Post-1.0 (B1-MISSED-01; CONFIRMED)

The propiska was THE eligibility gate for housing and employment in Soviet cities. Without
propiska, a citizen was invisible to the municipal housing queue.

**Formal institution:** Propiska was tied to a specific address and city. A citizen's propiska
determined their access to services, employment, and housing allocation.

**Candidate mechanic:** A boolean per citizen — `has_propiska` with a city reference. Without
propiska: cannot enter municipal housing queue, cannot access some services, cannot hold
permanent employment (only temporary/limit contracts). The Planner can restrict or expand
propiska to control migration, housing-queue growth, and labour supply.

**Research uncertainty:** The degree of enforcement varied by city and period. Moscow's
propiska was strictly enforced; smaller cities were more permeable. The game must decide
whether propiska is a hard gate or a probabilistic filter.

### Limitchiki — Post-1.0 (B1-MISSED-02)

Workers recruited to large cities on temporary contracts with limited registration.

**Formal institution:** Limitchiki filled labour gaps in cities that could not attract
permanent workers. They lived in enterprise dormitories under temporary propiska.

**Observed practice:** Limitchiki had restricted access to services, permanent housing, and
city amenities. High turnover — they came for work, endured poor conditions, and left.

**Candidate mechanic:** A Planner facing labour shortage in a city can recruit limitchiki
as a fast but socially costly solution. They work but do not enter the permanent housing queue.
They live in dormitories with poor conditions. They have high turnover. This creates a visible
underclass the Planner must manage — or invest in housing to convert to permanent residents.

### Komandirovki — Post-1.0 (B1-MISSED-04)

Business trips (komandirovki) served as an informal economy mechanism.

**Observed practice:** Travellers to better-supplied cities brought back deficit goods. A
Moscow komandirovka was an opportunity to buy things unavailable in the provinces.

**Candidate mechanic:** A citizen on a business trip can bring back deficit goods, updating
their household's shop knowledge and buffering local shortages. This connects to the citizen
[information model](provisioning.md) and [informal networks](social-networks.md). A physical
information and goods channel.

### Regional supply tiers — Post-1.0 (B2 §4.2; CONFIRMED)

CIA documented a strict supply hierarchy:

```text
Moscow >> provincial city >> small town >> rural
```

"Practically anything could be purchased in the city of Moscow" while "shortages of consumer
goods existed in small towns and rural centres." Soviet visitors to the US "expressed
amazement that there was no significant discrepancy in the supply of material goods between
cities and small towns."

If the game models multiple settlements or supply regions, this hierarchy is load-bearing.
A city's tier determines baseline supply quality. A closed city (science city, nuclear city)
has paradoxically superior supply because it serves strategic programmes — but at the cost of
restricted population and secrecy.

### Graduate assignment — 1.0 candidate (B1-28; B2-13; CONFIRMED)

Raspredelenie: mandatory job placement for graduates, established 1933. Graduates were
assigned by commission to positions anywhere in the USSR for 3–4 years. This created forced
migration and household disruption. Cross-referenced with [education](education.md).

### Migration drivers — CONFIRMED (B1-18)

Soviet migration was driven by: housing opportunities (the strongest pull), jobs, climate,
services, and administrative eligibility. These drivers connect migration to every other
society mechanic: housing, labour, provisioning, education.

## Current substrate

Citizens do not move between areas. `spawn_human` places one person per empty house; there is
no movement between settlements, no propiska, no limitchiki, no migration of any kind.

## Research basis

- Propiska: standard feature of the Soviet internal passport system; documented in all major
  Soviet-studies references.
- Limitchiki: Russiapedia, "Limitchik"; Encyclopedia.com, "Administration for Organized
  Recruitment."
- Komandirovki: attested in emigre accounts and B1-MISSED-04.
- Regional supply tiers: CIA, "Consumer Frustrations and the Soviet Regime" (1979).
- Graduate assignment: Wikipedia, "Job by distribution"; Equal Times (2022) on the Belarusian
  continuation.

## Open questions

- Should propiska be a Planner policy (grant/deny) or automatic based on housing availability?
  (B1 §6.2.)
- Is migration modelled in 1.0 at all, or is the single-city assumption sufficient?
- How does the game represent regional supply differentiation if it simulates a single city?

## Related

- [Housing](housing.md)
- [Labour](labor.md)
- [Citizens](citizens.md)
- [Education](education.md)
- [Social networks](social-networks.md)
- [Provisioning](provisioning.md)
- [Glossary: propiska](../../reference/glossary.md)
