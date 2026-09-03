# Infrastructure

**Kind:** index
**Authority:** advisory
**Status:** draft
**Owner:** infrastructure
**Last verified:** 2026-09-03

## What belongs here

This section describes the utility networks: electricity, water, heating, waste, sewage
(Post-1.0 charter cut), hydrology, and the shared topology kernel. The table below is the
**target** architecture: each network gets its own physical solver, its own inertia
signature, and its own failure mode for the Planner. Only electricity has any code today
(see Current substrate below); the rest is design guidance, not implementation.

**Shared topology concepts are not a shared physical solver.** The target design proposes a
shared topology kernel — typed nodes, edges, attachments, components, revision tracking, and
CSR adjacency — but each domain keeps its own solver state. Electricity is not water; a
binary blackout is not a pressure head calculation. The shared kernel avoids duplicating
graph code. The distinct solvers respect that each network has a different physical inertia.

| Network | Inertia signature | Failure delay | 1.0 status |
|---|---|---|---|
| Electricity | Electrical — near-instant | Seconds | 1.0 — charter row *Utilities* (partial substrate) |
| Water | Pressure/head — hydraulic | Minutes | 1.0 — charter row *Utilities* (greenfield) |
| Heating | Thermal mass + transport delay | Hours | 1.0 — charter row *Utilities* (greenfield) |
| Waste | Vehicle-hauled, not piped | Hours to days | 1.0 — charter row *Utilities* (greenfield) |
| Sewage | Gravity/buffer — hydraulic | Minutes to hours | Post-1.0 charter cut (ADR-0001) |
| Hydrology | Reservoir mass balance | Seasonal | 1.0 — charter row *Terrain and environment* (greenfield) |
| Gas | Linepack — pressurized pipeline | Hours to days | Post-1.0 |

## Current substrate

Electricity is the only network with any code: a road-derived `BTreeMap` adjacency graph
with BFS connectivity (`simulation/src/map/electricity_cache.rs:52-62,181-187`) and a binary
per-network blackout (`simulation/src/map_dynamic/electricity.rs:43-93`). `Map` holds only
`electricity` and `environment` among utility resources
(`simulation/src/map/map.rs:31-48`); no water, sewage, heating, waste, hydrology, or weather
system is registered (`simulation/src/init.rs:54-143`).

## Reading path

1. [Electricity](electricity.md) — the only network with any substrate (partial)
2. [Water](water.md) — 1.0 target: pressure, quality, and the border meter (greenfield)
3. [Sewage](sewage.md) — Post-1.0 charter cut: gravity, backpressure, and treatment
4. [Heating](heating.md) — pipe delay, thermal mass, coal grades
5. [Hydrology](hydrology.md) — the charter's breadth exception
6. [Waste](waste.md) — vehicle-hauled, not piped
7. [Network architecture](network-architecture.md) — the shared topology kernel

## Authoritative documents

- [Charter 1.0](../../plan/charter-1.0.md) — Terrain and environment commitment (hydro);
  explicit cuts (voltage tiers, CHP, electric-heating fallback)
- [Electricity spec](../../reference/specifications/electricity.md)
- [Water spec](../../reference/specifications/water.md)
- [Sewage spec](../../reference/specifications/sewage.md)
- [Heating spec](../../reference/specifications/heating.md)
- [Waste spec](../../reference/specifications/waste.md)

## Related

- [Physical economy](../physical-economy/index.md)
- [Transport](../transport/index.md)
- [Network kernel architecture](../../architecture/network-kernel.md) (lead writes)
- [Glossary](../../reference/glossary.md)
