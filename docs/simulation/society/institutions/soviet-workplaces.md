# Soviet workplace institutions

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** society
**Last verified:** 2026-09-03

| Scope | Label |
|---|---|
| Enterprise trade-union committee | Post-1.0 |
| Production conferences | Post-1.0 |
| Work-collective issue aggregation | Post-1.0, HYPOTHESIS |
| Safety inspection | Post-1.0, PLAUSIBLE |

All content on this page is Post-1.0.

## What this is

Soviet enterprises had internal institutions through which workers could raise issues, report
unsafe conditions, and contribute to planning. These were not independent trade unions in the
Western sense — they operated within the enterprise under party oversight. Their function was
information aggregation: they collected workplace problems and filtered them upward.

## Target design

### Enterprise trade-union committee and production conferences — Post-1.0

**Era caveat:** The 1983 Law on Labour Collectives gave work collectives legal standing as
actors. This is anachronistic for the game's fixed 1950s–60s era. In the 1950s–60s, the
enterprise trade-union committee and production conferences played the collective's role.

**Formal institution:** The enterprise trade-union committee (profkom) was elected by workers
within the enterprise. It met regularly and held a joint role with management in housing
allocation, welfare distribution, and labour-protection inspection. Production conferences
(proizvodstvennye soveshchaniya) were periodic meetings where workers discussed production
problems, norms, and workplace conditions.

**Observed practice:** The profkom's practical influence depended on the enterprise director's
willingness to cooperate. On housing and welfare allocation, the profkom had real leverage. On
production decisions, management prevailed. Production conferences produced proposals that
management could accept or ignore.

### Work-collective issue aggregation — HYPOTHESIS (bible §8.6–8.7)

The design proposes that workplace issues derive from physical facts, not abstract grievance
scores. No `grievance = 82`. Instead:

- Repeated month-end overtime → an issue the trade-union committee records
- Unsafe machine with deferred maintenance → a safety-inspection finding
- Housing delay for a promised allocation → a complaint with a causal reference to the
  housing queue
- Canteen shortage → a complaint with a causal reference to the food-supply chain
- Congestion on the commute route → a complaint with a causal reference to traffic state

Each issue keeps causal references — it IS the fact it reports. The institution aggregates
these facts, filters them (some are suppressed, some amplified), and presents them to
management and upward to the Planner. The filtering is the institution's bias — it is what
the [representation-error model](workplace-representation.md) measures.

### Safety inspection — PLAUSIBLE (bible §8.9; ILO/Semenov 1983; CIA ID 2114)

**Formal institution:** The technical inspector (tekhnicheskii inspektor) was a trade-union
official with the right to inspect workplace safety and order stoppages.

**Research basis:** The ILO Research Repository includes Semenov (1983) on workers'
participation in occupational safety and health in the USSR. CIA's "Labor Safety in Soviet
Industry" (March 1965, ID 2114) documents the institutional framework.

**Candidate mechanic:** Safety inspection takes physical inputs:
- Maintenance backlog (deferred maintenance raises risk)
- Fatigue (overtime hours in recent periods)
- Machine condition (age, maintenance state)
- Minor incidents (near-misses as leading indicators)

Choices the Planner or enterprise faces:
- Stop the line (production loss, safety gain)
- Defer (production continues, risk accumulates)
- Reallocate maintenance resources (trade-off with other maintenance)
- Ease quota pressure (reduces storming, reduces fatigue)

Consequences stay physical: storming and alcohol both raise accident rates. An accident
removes a worker (injury or death), damages equipment, and may trigger an inspection.

**Research uncertainty:** The degree to which technical inspectors actually enforced stoppages
varied by enterprise and period. The formal authority existed; its exercise was inconsistent.

## Current substrate

No workplace institution exists in code. Companies have workers and a recipe; no profkom, no
production conference, no safety inspection, no issue tracking.

## Open questions

- Should workplace issues be aggregated at the enterprise level (a list of physical facts) or
  only visible through individual citizen inspection?
- How does the 1950s–60s institutional form differ mechanically from the 1983 collective?
  The design says "the same information, different legal standing" — is that sufficient?

## Related

- [Trade unions](trade-unions.md)
- [Workplace representation](workplace-representation.md)
- [Workplaces](../workplaces.md)
- [Labour](../labor.md)
- [Healthcare](../healthcare.md)
- [Time](../time.md)
