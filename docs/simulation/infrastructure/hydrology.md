# Hydrology

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** infrastructure
**Last verified:** 2026-09-03

| Scope | 1.0 — charter row *Terrain and environment* |

## What this is

The charter names "reservoir-graph water, hydro dams" as a deliberate breadth exception:
terrain, water, and hydro are in scope even though they require significant new systems.
Hydrology tracks water as a mass balance across reservoirs, rivers, and dams.

A hydro dam converts water flow into electricity: `P = ρ g Q H η`, where `ρ` is water
density, `g` is gravity, `Q` is flow rate, `H` is head, and `η` is turbine efficiency.
The reservoir stores water; the dam releases it. The Planner manages reservoir levels
against seasonal inflow and downstream demand.

## Target design

Reservoir-graph water balance (PLAUSIBLE, bible §10.11):
- Nodes are reservoirs and river reaches
- Edges are flows with capacity
- Mass balance at each node: `dV/dt = inflow - outflow - evaporation - seepage`
- Dam release is a Planner-controlled policy

Hydro generation: `P = ρ g Q H η`. The output is an electricity generation offer
accepted by the Electricity authority. The head `H` depends on reservoir level; a
depleted reservoir produces less power.

Pollution coupling (charter): pollution affects basin water and crop yield. This
requires a pollution transport model on the reservoir graph.

## Current substrate

No hydro, reservoir, or basin system exists. No spec and no code. The weather/hydrology
spec is identified as missing (SYNTHESIS §7).

## Open questions

- Is the hydrological model seasonal or continuous?
- How does the reservoir graph interact with terrain generation?
- Is a simplified two-reservoir model sufficient for 1.0?
- What is the weather interface this requires?

## Related

- [Electricity](electricity.md)
- [Water](water.md)
- [Network architecture](network-architecture.md)
