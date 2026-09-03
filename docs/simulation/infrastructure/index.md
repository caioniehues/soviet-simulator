# Infrastructure

**Kind:** index
**Authority:** advisory
**Status:** draft
**Owner:** infrastructure
**Last verified:** 2026-08-28

## What belongs here

This section describes the utility networks: electricity, water, sewage, heating, waste,
and hydrology. Each network has its own physical solver, its own inertia signature, and its
own failure mode for the Planner.

**Shared topology concepts are not a shared physical solver.** The target design proposes a
shared topology kernel — typed nodes, edges, attachments, components, revision tracking, and
CSR adjacency — but each domain keeps its own solver state. Electricity is not water; a
binary blackout is not a pressure head calculation. The shared kernel avoids duplicating
graph code. The distinct solvers respect that each network has a different physical inertia.

| Network | Inertia signature | Failure delay |
|---|---|---|
| Electricity | Electrical — near-instant | Seconds |
| Water | Pressure/head — hydraulic | Minutes |
| Sewage | Gravity/buffer — hydraulic | Minutes to hours |
| Heating | Thermal mass + transport delay | Hours |
| Gas (Post-1.0) | Linepack — pressurized pipeline | Hours to days |

## Reading path

1. [Electricity](electricity.md) — the only network with any substrate
2. [Water](water.md) — pressure, quality, and the border meter
3. [Sewage](sewage.md) — gravity, backpressure, and treatment
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
