# Simulation knowledge tree

**Kind:** index
**Authority:** advisory — design explanation; specifications bind, this tree explains
**Status:** active
**Owner:** simulation
**Last verified:** 2026-08-28

Why each mechanic exists, how it behaves in the design, what 1.0 requires of it, what the code does
today, and what the evidence says. Every page keeps those five states apart under labelled
headings ([document authority](../meta/document-authority.md)).

## Belongs here

Concept pages for game mechanics and the cross-cutting ideas they share. Each page answers one
question and links to its specification, its current-substrate entry and its research.

## Does not belong here

Normative rules (specifications), software architecture (architecture handbook), code standards
(engineering), evidence reports (research), task state (`bd`).

## The tree

| Section | Question | Pages |
|---|---|---|
| [Concepts](concepts/index.md) | What ideas recur across every system? | authority, physical causality, scarcity, queues, reserves, phase lag, reliability, information, adaptation, social reproduction |
| [Planned economy](planned-economy/index.md) | How does the Plan generate behaviour? | plan cycle, material balance, enterprise behaviour, reports and information, reserves, priorities, reliability and buffering, storming, allocation |
| [Physical economy](physical-economy/index.md) | How do goods actually move and change hands? | resources, requests, allocation, reservation, custody, storage, production, logistics, construction |
| [Society](society/index.md) | How are workers physically reproduced? | citizens, households, housing, labour, workplaces, education, healthcare, time, provisioning, demography, migration, social networks, [institutions](society/institutions/index.md) |
| [Transport](transport/index.md) | How do vehicles, routes, traffic and rail behave? | roads, pathfinding, traffic, vehicles, freight rail, public transport (future) |
| [Infrastructure](infrastructure/index.md) | How does each network's inertia differ? | electricity, water, sewage, heating, hydrology, waste, network architecture |
| [National projects](national-projects/index.md) | How does a nationwide distortion stress the ordinary economy? | space, housing campaign, mobilization; the modes |
| [Causal loops](causal-loops.md) | Which cross-system feedback loops define play? | the catalogue |

## Reading path

1. [Concepts index](concepts/index.md) — read [physical causality](concepts/physical-causality.md), [scarcity](concepts/scarcity.md), [reliability](concepts/reliability.md), [information](concepts/information.md) first.
2. [Plan cycle](planned-economy/plan-cycle.md) and [enterprise behaviour](planned-economy/enterprise-behavior.md) — the core loop.
3. [Physical economy index](physical-economy/index.md) — the canonical sequence from request to consumption.
4. [Causal loops](causal-loops.md) — how the domains couple.
5. Then the domain of your task.

## Authoritative documents this tree depends on

- [Charter](../plan/charter-1.0.md) — scope
- [Glossary](../reference/glossary.md) — terms
- [Specifications](../reference/specifications/README.md) — mechanism (all `draft`)
- [Current substrate](../architecture/current-substrate.md) — code reality
- [Mechanics index](../reference/mechanics-index.md) — the navigation table across all of the above

## Related

- [Product](../product/index.md) — the why above the tree
- [Architecture](../architecture/index.md) — how the tree is computed
- [Research](../research/index.md) — the evidence beneath it
