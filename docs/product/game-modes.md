# Game modes and scenarios

**Kind:** concept
**Authority:** advisory — Post-1.0 direction; the charter's three authored Plans are the only 1.0 commitment in this area
**Status:** draft
**Owner:** project lead
**Last verified:** 2026-08-28
**Provenance:** Lane H (`docs/research/conversation-mining-2026-08-28/H-game-modes-and-progression.md`) validated the design thread's fifteen modes against history and code; the "who decides" questions come from the design bible §8.14.

## The finding that organises everything

**A mode is a rule preset; a scenario is a starting state.** The design thread listed fifteen
"modes" and conflated the two. Architecturally they are different objects and combine freely:
*Sovnarkhoz on a Frontier Corridor map* is a territorial-planning mode on a linear scenario.

- A **mode** changes four to eight institutional parameters over the same physical simulation:
  who proposes quotas, who sets reserves, who selects management, who allocates enterprise
  surplus, who controls housing and welfare, who approves overtime and norms, who coordinates
  inter-enterprise inputs. Never government-type bonuses.
- A **scenario** fixes the map, the starting buildings and stocks, and a brief.

None of the institutional parameters exist in code: `Government` holds only `money`; there is no
quota, period, priority class, territorial subdivision or role separation
([current substrate](../architecture/current-substrate.md)). W&R has one rule set and scenarios
over it; organisational modes would be genuinely novel in the genre.

## Historical grounding — CONFIRMED (Lane H §2)

| Mode | Historical basis | What changes |
|---|---|---|
| Sovnarkhoz | 1957–65: ~30 branch ministries replaced by 105, then 47, regional councils; reversed 1965 (Kibita 2013) | Allocation topology flips from vertical (by commodity) to horizontal (by territory); local information improves, cross-regional coordination degrades; *mestnichestvo* replaces *vedomstvennost'* |
| The Reform (Kosygin) | 1965: indicators cut to 4–8, profit retention, bonuses tied to sales; "changing the names on doors"; quietly abandoned early 1970s | Fewer Planner levers; enterprise autonomy; teaches why the reform failed when prices carry no information — which, by pillar, they cannot here |
| Self-Management | Yugoslavia 1950–91: workers' councils set targets, surplus, managers; a different system, not a Soviet variant | Removes quota authority over enterprises; the Planner sets infrastructure and social policy |
| Danwei | China 1950s–90s: the work unit as a "small society" providing housing, rations, care, permission to marry | Welfare bundled into enterprises; closing a factory displaces a society |
| Rationing / Mobilization | 1941: tiered rations by labour category; 1,360 factories relocated in three months; tank output 6,274 → 24,639 | Factory conversion, tiered allocation, industrial evacuation as physical logistics |
| Late-System Maintenance | 1980s: capital repair rising from ~10 % to ~20 % of investment; ageing eastern plant | Everything decays at once; triage is the game |
| Science / Closed City | Akademgorodok (1958), Baikonur, the nuclear cities | Priority provision creates a bubble of abundance at the cost of scarcity elsewhere |
| Frontier Corridor | BAM 1974–91: construction along a linear axis; settlements failed for lack of housing and services (CIA 1987) | The supply chain for building the supply chain |
| Housing Campaign | 1957 pledge to end the shortage in twelve years; Khrushchyovka standardisation | Construction materials compete between housing and industry; quality vs quantity |

## The mode cards

Lane H §3 holds a full card for each of sixteen modes — premise, starting state, rule changes,
pressure source, win-less success condition, ten-minute loop, ten-hour arc, what it teaches, and
base-system dependencies. This page keeps the shape and the dependency verdicts.

| Mode | Kind | Needs beyond the base game | Cost |
|---|---|---|---|
| Shortages Amid Plenty | scenario | nothing — it *is* the base game in steady state: adequate aggregate production, unreliable dispatch, hoarding, poor retail | lowest; **this is the core loop** |
| The Taut Plan | mode | quota targets and enforcement over the existing `request_multiplier` | cheapest mode |
| Frontier Corridor | scenario | a linear map; existing rail, freight station, construction | cheap |
| Everyday Socialism | scenario | the existing `BuyFood`/`Home`/`Work` loop, retail delivery; a district-scale focus on queues, childcare, commute, time | cheap once households exist |
| Monotown | both | enterprise welfare, housing queue, recruitment, turnover | medium |
| Housing Campaign | both | construction material competition, housing queue, mikrorayon completeness | medium |
| Closed City | mode | priority allocation, assigned (not recruited) workers | medium |
| Sovnarkhoz | mode | territorial subdivision of the map and of allocation | new system |
| The Reform | mode | an indicator system | new system |
| Self-Management | mode | enterprise autonomy and council AI | new system |
| Late-System Maintenance | mode | building age and condition, maintenance requirements | new system |
| National Project — Space | both | multi-tier priority freight, extended resource tree, remote settlement | expensive |
| National Project — Mobilization | both | factory conversion, tiered rationing, evacuation logistics | expensive |
| Science City | both | education tiers, priority provision, **a research system that is not in the charter** | most expensive |
| International Plan | mode | AI neighbouring economies | very expensive |
| Enterprise Director | inverted game | an AI Planner; the player *is* the dishonest enterprise | a different game — standalone expansion, not a base mode |

## What the design thread missed (Lane H §4)

- **Mid-save transitions.** History switched modes on one save (ministries → sovnarkhozy →
  ministries). Recommendation: modes are switchable mid-save with an institutional-disruption
  period; mode selection becomes a strategic act, not a menu.
- **Multiplayer as a mode.** `networking/` is a complete lockstep client/server crate. One player
  as Gosplan, others as ministries or directors is feasible — but `WorldCommand` has no role
  filter. A `Role` on commands, filtered at the server, would make a mode no competitor offers.
- **Chronicle.** Dwarf Fortress's Legends viewer applied to the republic's causal history: which
  enterprises hoarded, which shortage cascaded from what, who moved where and why. Not a mode — a
  view available from every mode, and it needs the [causal facts](../architecture/causality.md).
- **The tutorial is a mode-design problem.** The charter's First Plan must teach the loop through
  play: start simple, let request inflation emerge from the simulation, point the HUD strip at the
  discrepancy, escalate into dispatch and needs, hand over to the Second Plan. W&R uses eighteen
  tutorial maps; a single continuous scenario with emergent teaching is cheaper and less brittle.
- **Unlocking.** The three authored Plans are the progression; after them, all modes and endless
  mode open.
- **Never game over changes mode design.** Pressure must be qualitative — visible suffering the
  player cannot ignore — because termination is off the table.

## Open questions

1. Modes switchable mid-save, or fixed at start?
2. Pursue Enterprise Director at all?
3. Do the three authored Plans teach three modes (base → Taut Plan → Housing Campaign)?
4. How much enterprise AI is acceptable (Self-Management and Enterprise Director need a lot)?
5. Is multiplayer worth its testing and balancing burden?

## Related

- [National projects](../simulation/national-projects/index.md)
- [Alternate socialist institutions](../simulation/society/institutions/alternate-socialist-institutions.md) — the "who decides" questions in full
- [Post-1.0](post-1.0.md)
- [The Planner](player-role.md)
- [Lane H report](../research/conversation-mining-2026-08-28/H-game-modes-and-progression.md)
