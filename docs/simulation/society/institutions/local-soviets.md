# Local Soviets — information aggregation, not popularity

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** society
**Last verified:** 2026-08-28

| Scope | Label |
|---|---|
| Local Soviets as information channel | Post-1.0 |
| Nakazy (electors' mandates) | Post-1.0, CONFIRMED form |
| Standing commissions | Post-1.0, CONFIRMED form |

All content on this page is Post-1.0.

## What this is

The local Soviet (gorodskoi sovet, raionnyi sovet) was the lowest tier of elected government.
Its deputies were elected in single-candidate elections with party-controlled nomination.
Its useful mechanic for the game is not democracy — it is information aggregation. The local
Soviet collects information the enterprise misses: a factory reports on-time production, but
the local-Soviet deputy reports that workers complain about cold apartments and long queues.

## Target design

### Single-candidate elections — CONFIRMED form

**Formal institution:** Elections were single-candidate, party-nominated. Voters approved or
rejected the sole candidate; rejection rates were low. The game does not model multiparty
competition — the charter's fixed 1950s–60s era had none, and the design bible explicitly
rejects a popularity meter.

**Observed practice:** The election was a ritual, not a contest. The value was not in the
outcome but in the process: deputies were expected to be accessible to constituents and to
pursue their mandates.

### Nakazy — CONFIRMED (electors' mandates)

**Formal institution:** Nakazy were specific requests from electors to their deputy. A deputy
was formally bound to pursue these mandates. They were collected during election meetings and
recorded as obligations.

**Observed practice:** Nakazy covered concrete local problems: road repair, heating failures,
shop supply, school capacity, housing allocation. They were a bottom-up information channel —
citizens told the state what was broken.

**Candidate mechanic:** Nakazy are information events. Citizens report physical problems
(cold apartments, empty shops, broken roads) to their local deputy. The deputy aggregates and
forwards. The Planner receives a filtered list of district problems, prioritised by frequency
and severity. This is a sensor network: the local Soviet reports what the enterprise report
and the union report both miss.

The bias: nakazy skew toward visible, immediate problems (the leaking roof, the empty shelf)
and away from systemic causes (the freight dispatch failure that caused the empty shelf). The
Planner must trace from the symptom to the cause.

### Standing commissions — CONFIRMED form

**Formal institution:** Standing commissions were advisory bodies attached to each soviet.
Typical commissions: planning and budget; working and living conditions for women;
mother-and-child welfare; housing and communal services; trade and public catering.

**Observed practice:** Commissions investigated specific issues, heard testimony, and made
recommendations. They had no executive power. Their reports reached the soviet plenary and
could be forwarded to higher authorities.

**Candidate mechanic:** Standing commissions are another information channel with a different
bias — they investigate systematically where nakazy are reactive. A "working and living
conditions" commission might detect that female workers at a specific factory have unusually
high absenteeism (a housing problem, a childcare gap, a transport gap). This information
would not appear in the enterprise report or the nakazy.

## Current substrate

No local-Soviet institution exists in code. No election, no deputy, no nakaz, no commission.
The Planner has direct, unfiltered access to simulation state.

## Research basis

- Soviet local government structure documented in standard references.
- Bible §8.11: "the useful mechanic is nakazy and standing commissions."
- Synthesis §3.5: the lead confirmed the institutional form and noted the period accuracy.
- The model of local Soviets as a sensor network rather than a governance body is the bible's
  own contribution.

## Open questions

- How does the local-Soviet information channel interact with the enterprise and union
  channels in the [representation-error model](workplace-representation.md)?
- Should the local Soviet have any executive authority (e.g., ordering a road repair), or is
  it purely an information source for the Planner?
- What determines the quality of a local-Soviet channel — deputy quality, commission activity,
  or citizen participation?

## Related

- [Workplace representation](workplace-representation.md)
- [Trade unions](trade-unions.md)
- [Housing](../housing.md)
- [Provisioning](../provisioning.md)
- [Information concept](../../concepts/information.md)
