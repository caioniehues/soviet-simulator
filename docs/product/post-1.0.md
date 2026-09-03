# Post-1.0 direction

**Kind:** plan
**Authority:** advisory — direction only; nothing here may acquire 1.0 acceptance criteria (charter §Scope discipline)
**Status:** draft
**Owner:** project lead
**Last verified:** 2026-09-03

What the design thread and the validation lanes identified as worth building after 1.0, grouped
by the system it extends. Every item links to the page that explains it. "Hook" means 1.0 should
avoid an architectural dead end; it never means implementing dormant complexity now.

## Planned economy

- Adaptive request inflation from experienced reliability; planning credibility record; the ratchet — [reliability and buffering](../simulation/planned-economy/reliability-and-buffering.md). Shortage propagation and going without are 1.0 (charter identity plus the Resources row); only these three proposals are deferred — [ADR-0001](../decisions/0001-households-and-utilities-are-1.0-scope.md)
- Plan periods, storming, freight-plan stability metrics — [storming](../simulation/planned-economy/storming.md)
- Reserve classes (the five-class custody-state model), confiscation — [reserves](../simulation/planned-economy/reserves.md). Physical stock with inspectable surplus is 1.0 (charter Resources and production row); only the class decomposition is deferred — [ADR-0001](../decisions/0001-households-and-utilities-are-1.0-scope.md)
- Priority classes and priority inflation — [priorities](../simulation/planned-economy/priorities.md)
- Tolkachi, ministries, assortment plans, investment hunger, OTK quality attestation, plan-fulfilment falsification — [enterprise behaviour](../simulation/planned-economy/enterprise-behavior.md)
- Rationing regimes, substitution chains — [allocation](../simulation/planned-economy/allocation.md)
- Quality lots and rework (charter: perishables cut; quality is unmentioned) — hook

## Society

- Household time budgets and the social-reproduction balance — [time](../simulation/society/time.md)
- Childcare / kindergarten (charter cut) as a household-scheduling hook — [workplaces](../simulation/society/workplaces.md)
- Propiska, limitchiki, komandirovki, regional supply tiers — [migration](../simulation/society/migration.md)
- Kommunalka → separate-flat housing tiers; enterprise housing as recruitment — [housing](../simulation/society/housing.md)
- Household plots, blat, non-monetary access channels — [social networks](../simulation/society/social-networks.md), [provisioning](../simulation/society/provisioning.md)
- Alcohol as a health, time and productivity system — [healthcare](../simulation/society/healthcare.md)
- Cohort expectations and the fertility echo — [demography](../simulation/society/demography.md)
- Labour differentiation, tenure ramp, labour hoarding — [labour](../simulation/society/labor.md)
- Work collectives, trade unions, safety inspection, local Soviets, representation error — [institutions](../simulation/society/institutions/index.md)

## Physical economy and transport

- Handling classes, dock rates, deadhead metrics — [logistics](../simulation/physical-economy/logistics.md)
- Construction phases, offices, crews — [construction](../simulation/physical-economy/construction.md)
- Mass/grade/traction vehicle physics, IDM following, MOBIL lane changes — [vehicles](../simulation/transport/vehicles.md)
- Meso traffic (CTM/LTM) with spillback; junction deadlock resolution — [traffic](../simulation/transport/traffic.md)
- Rich rail: signalling as capacity, yards, wagon compatibility, empty repositioning; passenger rail and electrification (charter cut) — [freight rail](../simulation/transport/freight-rail.md)
- Buses, trams, trolleybuses — [public transport](../simulation/transport/public-transport-future.md)
- Winter roads, snow clearing, road wear — [roads](../simulation/transport/roads.md)

## Infrastructure

- Sewage network (gravity DAG, buffers, treatment) — charter cut 2026-09-03, [ADR-0001](../decisions/0001-households-and-utilities-are-1.0-scope.md) — [sewage](../simulation/infrastructure/sewage.md)
- Generator ramp, startup, reserve; grid frequency (far future) — [electricity](../simulation/infrastructure/electricity.md)
- Gas pipelines with linepack (charter cut: pipelines) — [network architecture](../simulation/infrastructure/network-architecture.md)
- CHP (charter cut) — hook only
- Unified weather authority stressing every network — [network architecture](../simulation/infrastructure/network-architecture.md)

## Architecture

- Record/body citizen split, SoA stores, bitset cohorts, event calendar, cadence bands — [entity identity](../architecture/entity-identity.md), [state storage](../architecture/state-storage.md), [time and events](../architecture/time-and-events.md)
- Phase labels, keyed randomness, deterministic parallelism — [simulation phases](../architecture/simulation-phases.md), [parallelism](../architecture/parallelism.md)
- Change journal, observatory, causal facts, snapshots — [change journal](../architecture/change-journal.md), [observatory](../architecture/observatory.md), [snapshots](../architecture/snapshots.md)
- Shadow simulation (Gosplan computer), LP feasibility instrument — [observatory](../architecture/observatory.md)
- Multiplayer as a Gosplan-vs-ministry mode over the existing `networking/` crate — [game modes](game-modes.md)

## Game modes and national projects

Sixteen mode cards, the scenario-vs-mode distinction, mid-save transitions, chronicle, and the
tutorial problem — [game modes](game-modes.md), [national projects](../simulation/national-projects/index.md).

## What stays out

The charter's Never list: tourism, hotels, attractions, fires, disasters. Enterprise Director
(the player as the dishonest enterprise) contradicts the Planner information model and is
recorded as a possible standalone expansion, not a base-game mode.

## Related

- [1.0 scope](scope-1.0.md)
- [Charter](../plan/charter-1.0.md)
- [Migration sequence](../architecture/migration-sequence.md)
