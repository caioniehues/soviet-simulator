# Trade unions — Soviet unions by period

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** society
**Last verified:** 2026-09-03

| Scope | Label |
|---|---|
| Union welfare allocation | Post-1.0 |
| UnionCommittee sketch | Post-1.0, HYPOTHESIS |

All content on this page is Post-1.0.

## What this is

Soviet trade unions were not Western bargaining bodies. They did not negotiate wages or
organise strikes. They administered social insurance, inspected labour safety, allocated
housing and sanatoria vouchers, and represented workers within the enterprise's institutional
framework. The union was a welfare distributor, not a pressure organisation.

## Target design

### Period separation

Soviet union functions changed over time. The game's fixed 1950s–60s era determines which
role is primary.

**1930s–50s:** Social insurance administration was the primary role from 1933 onward, when
unions took over sickness pay, maternity benefits, and pension supplements from the state
insurance apparatus. Unions were subordinate to management on production questions.

**1950s–60s (the game era):** Expanded welfare role. Key functions:

| Function | Description |
|---|---|
| Social insurance | Sickness pay, maternity, pension supplements — from 1933 |
| Labour protection | Factory safety committees; technical inspection authority |
| Housing lists | Joint role with management in allocating enterprise housing |
| Sanatoria vouchers (putyovki) | 20 % free, remainder at 20–30 % of cost |

In 1960, trade unions controlled almost all sanatoriums and rest resorts. The putyovka system
was a significant welfare benefit: access to holiday rest was mediated by the union, not
purchased on a market.

**Late-Soviet:** More institutional independence on paper; less practical influence as the
overall system ossified.

### What the union is NOT

- Not a Western bargaining body. Wages were set centrally; the union did not negotiate them.
- Not a strength meter. No `union_power = 0.7`. The union's effectiveness depends on whether
  management cooperates on welfare issues and whether the technical inspector enforces safety
  findings.
- Not a production actor. The union never owns stock, sets quotas, or manages output.

### UnionCommittee sketch — HYPOTHESIS

The design proposes a `UnionCommittee` institutional entity per enterprise:

```text
UnionCommittee:
  membership: Set<CitizenID>
  safety_cases: Vec<SafetyCase>         -- from physical inspection findings
  welfare_cases: Vec<WelfareCase>       -- housing requests, sanatoria applications
  proposals: Vec<Proposal>             -- from production conferences
  meeting_cadence: u32                 -- ticks between meetings
  memory: InstitutionalMemory          -- past actions and outcomes
```

The committee meets periodically, reviews accumulated cases, and acts (or does not). Its
effectiveness is measurable: cases submitted vs cases acted on. This is the
[institutional-confidence](workplace-representation.md) model applied to the union.

### Sanatoria vouchers — Post-1.0

Putyovki were allocated by the union committee: 20 % free, the rest at 20–30 % of cost.
Access to sanatoria was a valued welfare benefit and a union-mediated allocation channel. In
the game, this would be a rest/recovery mechanism that reduces worker fatigue and illness — a
welfare investment with measurable returns in reduced absenteeism.

## Current substrate

No union institution exists in code. Companies have workers and a recipe
(`simulation/src/world.rs:185-191`); there is no committee, no voucher, no welfare
allocation. This page is entirely target design.

## Research basis

- Deutscher (1950), *Soviet Trade Unions*: the foundational English-language study. Documents
  the shift from independent unions (1920s) to state-managed welfare administrators (1930s+).
- ILO Research Repository: Semenov (1983) on workers' participation in OSH in the USSR.
- B1-MISSED-03: trade unions as housing/vacation allocators, with putyovka data.
- Open Book Publishers, "Holiday Convergences": putyovka system detail.
- JSTOR, "Trade Union in Soviet Social Insurance": union role in insurance administration.
- Bible §8.8: the "not a bargaining body, not a strength meter" framing.

## Open questions

- How does the union's housing-list role interact with the enterprise housing channel in the
  [housing queue](../housing.md)?
- Should the putyovka system exist as a Post-1.0 hook (a data field on workers) or only as a
  later Post-1.0 mechanic?
- What is the institutional memory's retention class — does the union "remember" past
  management refusals?

## Related

- [Soviet workplaces](soviet-workplaces.md)
- [Workplace representation](workplace-representation.md)
- [Workplaces](../workplaces.md)
- [Housing](../housing.md)
- [Healthcare](../healthcare.md)
- [Labour](../labor.md)
