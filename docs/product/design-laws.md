# Design laws

**Kind:** concept
**Authority:** advisory — laws 1, 3, 5, 11 and the never-game-over rule are charter pillars (binding); the rest are design principles from the design thread, recorded, not ratified
**Status:** draft
**Owner:** project lead
**Last verified:** 2026-08-28

Twenty laws in four groups. Each law names its status: **charter** (binding pillar), **spec**
(already in a draft specification), **new** (from the design thread; advisory). Where the code
violates a law today, the [current substrate](../architecture/current-substrate.md) says so.

## Physical causality

1. **Goods move physically or do not move.** Allocation, matching, payment, route creation and reservation never teleport stock. *(charter)* → [physical causality](../simulation/concepts/physical-causality.md)
2. **Request, allocation, reservation, pickup, custody, delivery, on-hand and consumption are separate states.** *(spec: SPEC-PRODUCTION-003; glossary "Request")*
3. **Failure persists and never ends the game.** Missing stock, vehicle, route, dock, labour, power, water, housing, school or clinic capacity becomes a visible waiting, partial, stalled, substitution or going-without state. *(charter)* → [scarcity](../simulation/concepts/scarcity.md)
4. **No silent deletion.** Goods, demand, citizens, vehicles, queues and sites do not vanish because a transaction failed. *(new; violated today — ECO-SUB-001)*
5. **No domestic price clearing.** Scarcity resolves by policy, queue, priority, substitution, rationing, reserve, adaptation or going without. The rouble is border foreign currency only. *(charter; violated today by the inherited treasury — ECO-SUB-004)*
6. **Physical opportunity cost must be visible.** Prioritising one use removes actual capacity, materials, labour, transport, housing or service access from another. *(new)* → [priorities](../simulation/planned-economy/priorities.md)

## Information causality

7. **Reports are not truth.** Reported demand, plan fulfilment, institutional reports and citizen knowledge are distinct from physical state. *(new)* → [information](../simulation/concepts/information.md)
8. **Information is a resource.** Better reporting, monitoring, representation and reliable institutions improve planning quality without magically improving supply. *(new)*
9. **No omniscient player UI.** Player-facing data comes through Planner-visible snapshots and institutional observation, never unrestricted access to the simulation. *(new; violated today — the UI reads `Simulation` directly)* → [snapshots](../architecture/snapshots.md)
10. **No hidden honesty flag.** Strategic behaviour is inferred from discrepancies: request, receipt, consumption, on-hand, surplus, queue age, declared capacity, physical output. *(spec: SPEC-PRODUCTION-009)* — "no hidden *verdict*" is not "no hidden *state*": the reasons an enterprise inflates live in the simulation, unseen by the Planner. → [enterprise behaviour](../simulation/planned-economy/enterprise-behavior.md)

## Social causality

11. **Citizens persist as identities.** Embodiment may be bounded; the record is not disposable. *(charter; spec: SPEC-CITIZENS-001)*
12. **Households are first-class actors.** Residence, pantry, care obligations, housing queues, adaptation, family history. *(spec: SPEC-HOUSEHOLDS-004; absent in code)*
13. **Social reproduction is physical.** Workers must be housed, fed, heated, educated, transported, kept healthy and given time. *(new; CONFIRMED historically)* → [social reproduction](../simulation/concepts/social-reproduction.md)
14. **Citizens adapt.** They search, queue, substitute, buffer, reschedule, use plots, use contacts, relocate, change jobs or go without. *(new; CONFIRMED historically)* → [adaptation](../simulation/concepts/adaptation.md)
15. **No single happiness scalar.** Preserve causes: queue burden, crowding, warmth, health, time pressure, access, commute, career, household reliability. *(new)*

## Technical causality

16. **Stable things sleep; pressure wakes them.** No per-frame AI for slow social and economic state. *(new; the path to 250k)* → [time and events](../architecture/time-and-events.md)
17. **Compute → deterministic merge → commit.** Parallel workers calculate intents; only ordered commits mutate authoritative state. *(new)* → [parallelism](../architecture/parallelism.md)
18. **One authority per state transition.** Cross-domain code references IDs and results; it never mutates another domain's ledger. *(spec register's authority table)* → [authority](../simulation/concepts/authority.md)
19. **Every replayable transaction is idempotent** under an immutable ID. *(spec: SPEC-WATER-006, SPEC-ELECTRICITY-002)* → [simulation transitions](../engineering/simulation-transitions.md)
20. **No generalised abstraction before shared invariants are proven.** Share topology, scheduling, IDs, journals; never force water, power, traffic, sewage, heat and gas through one solver. *(new)* → [network architecture](../simulation/infrastructure/network-architecture.md)

## Two rules above the laws

> **Automate execution, not decisions.** The player chooses strategy; the simulation performs recurring execution.

> **Simulate a distinction only when it can change a decision or a consequence.** → [simulation philosophy](simulation-philosophy.md)

## Related

- [Vision](vision.md)
- [Invariants index](../reference/invariants.md) — the laws as testable cross-system invariants, with the specs that instantiate them
- [Charter](../plan/charter-1.0.md)
- [Current substrate](../architecture/current-substrate.md) — where the code violates a law today
