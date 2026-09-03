# Workplaces — the enterprise as miniature welfare state

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** society
**Last verified:** 2026-09-03

| Scope | Label |
|---|---|
| Enterprise welfare provision | Post-1.0 |
| Settlement multiplier | Post-1.0 |
| Shift waves | Post-1.0 hook |
| Storming and fatigue | Post-1.0 hook |
| Canteen vs kitchen | Post-1.0 |

## What this is

A Soviet enterprise was not merely a production unit. It was a miniature welfare state:
housing, dormitory, childcare, canteen, clinic, cultural facilities, and sometimes transport.
Closing a factory removed not just jobs but the social infrastructure of an entire district.

The enterprise is where the production side of the economy meets the social-reproduction side.
An enterprise that builds a canteen recovers household time. An enterprise that provides
housing retains workers. An enterprise that neglects welfare loses workers to competitors.

## Target design

### Enterprise as welfare state — CONFIRMED (B1-02; Filtzer 1994; CIA §5.7)

The design proposes that enterprises own and operate social infrastructure alongside
production. The welfare facilities an enterprise may provide:

| Facility | Effect |
|---|---|
| Housing | Fastest queue channel; retains workers |
| Dormitory | Temporary housing for limitchiki and new recruits |
| Childcare (nursery) | Releases specific adult care-hours |
| Canteen (stolovaya) | Recovers household food-preparation time |
| Clinic (medsanchast') | Reduces sick-day absence |
| Cultural facility | Discretionary-time quality (minor; Post-1.0) |
| Transport | Commute-time reduction for remote enterprises |

Each facility requires physical inputs (food for the canteen, medicine for the clinic, staff
for the nursery) and produces a social outcome measurable in the household time budget.

CIA enterprise analyses ("The Soviet Enterprise and How it Operates", 1955; "Management of the
Soviet Industrial Enterprise", 1956; "Production Associations in the USSR", 1971) document
enterprises as welfare providers, not merely production units.

### Settlement multiplier — PLAUSIBLE (B1-03)

A plant employing 8,000 workers requires a settlement of ~20,000 people — a 2.5× multiplier.
Soviet labour-force participation rates of 50–55 % give a multiplier of 1.8–2.0×; with
dependants, retirees, and service workers, 2.5× is in the right range but on the high side.
The exact ratio depends on household size, age structure, and the extent of enterprise welfare.

This ratio makes every industrial project a settlement project. The Planner who places a
factory is committing to housing, services, and infrastructure for 2–3× the workforce.

### Shift waves — Post-1.0 hook

Large workplaces pulse commute demand at shift changes. The design proposes that staggering
shifts (07–15 / 08–16 / 09–17) lowers peak load with the same infrastructure.

Work intervals in the current code already carry random offsets
(`simulation/src/souls/desire/work.rs:32-37`) — a cheap move toward explicit shift scheduling.
This is the most code-ready workplace mechanic.

### Overtime and fatigue under storming — CONFIRMED (B1-31)

End-of-period storming (shturmovshchina) enters worker life as overtime, sleep loss, fatigue,
absenteeism, quality problems, family-time loss, and future turnover. The design proposes that
storming pressure from the production side propagates into the household time budget: overtime
consumes discretionary hours, fatigue reduces next-period effectiveness, and family-time loss
increases turnover probability.

This is one domain instance of [cross-system causal loops](../concepts/phase-lag.md) — the
economy presses on society through the enterprise.

### Canteen vs kitchen — Post-1.0 (B1-MISSED-07)

The enterprise canteen (stolovaya) provided cheap meals and reduced the household's
food-preparation time. A functioning canteen converts enterprise food supply into recovered
household time. When canteen supply fails, the burden shifts to the household kitchen — 10–12
hours per week of food preparation (women; 1965 data).

Building a canteen at an enterprise is a Planner investment in time recovery. The canteen is a
service that connects [enterprise welfare](workplaces.md) to the
[household time budget](time.md).

## Current substrate

Companies have workers and a recipe. `CompanyEnt`
(`simulation/src/world.rs:185-191`) holds `comp: GoodsCompanyState`, `workers: Workers`,
`sold`, and `bought` alongside `trans`. `GoodsCompanyState`
(`simulation/src/souls/goods_company.rs:70-78`) stores `proto`, `building`, `max_workers`,
`progress`, `driver`, and `trucks` — no workers field, no welfare provision, no childcare,
no canteen, no clinic, no dormitory, no cultural facility, no enterprise transport. The
worker type is `Workers(Vec<HumanID>)` (`simulation/src/economy/mod.rs:42`).

The work desire (`simulation/src/souls/desire/work.rs`) assigns workers with a time interval
(approximately 08:00–18:00 with random offsets). No shift pattern, no overtime, no fatigue.

## Research basis

- Filtzer (1994), *Soviet Workers and De-Stalinization*: the enterprise as social institution.
- CIA, "The Soviet Enterprise and How it Operates" (December 1955, ID 15044).
- CIA, "Management of the Soviet Industrial Enterprise" (November 1956, ID 1876).
- CIA, "Production Associations in the USSR" (March 1971, ID 1760).
- B1-02 documents the welfare list; B1-03 the settlement multiplier.
- B1-MISSED-07 documents the canteen/kitchen split.

## Open questions

- Which enterprise welfare facilities should exist as Post-1.0 hooks (data fields, no mechanic)
  versus later Post-1.0 features?
- Should the Planner directly assign welfare facilities to enterprises, or should enterprises
  build them autonomously?
- How does enterprise closure propagate through the settlement's social infrastructure?

## Related

- [Labour](labor.md)
- [Housing](housing.md)
- [Time](time.md)
- [Households](households.md)
- [Provisioning](provisioning.md)
- [Institutions: soviet workplaces](institutions/soviet-workplaces.md)
- [Phase lag concept](../concepts/phase-lag.md)
