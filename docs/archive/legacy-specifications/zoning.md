# Zoning

**Status:** draft model (grounded in research)
**Phase:** 1
**Primary inspiration:** CS1 zoning grid (studied and largely rejected) + W&R planned placement (kept)
**Evidence:** see [research/zoning-buildings.md](../research/zoning-buildings.md) for CS1-code and W&R-data sources.

> State authorises land for development; buildings do NOT spawn magically — a construction process begins.

## Purpose

The core question of this spec: **what replaces market-driven growables in a planned economy?** The two labs sit at opposite poles — CS1 is a full demand-driven spawning machine (paint zone → RCI demand → engine spawns a prefab), W&R has no zoning at all (**CONFIRMED-absence** — every building is player-placed and physically constructed). Our model: zoning as **planner intent** — a land-use plan layer that constrains and guides siting, and never causes a building to exist.

## The two poles (grounded)

### CS1 — zoning as a spawn machine (the anti-model, **CONFIRMED**)

- The zoning surface is a swarm of 8×4-cell `ZoneBlock` grids stapled to each side of a road segment (`ZoneBlock.cs:6-44`, `RoadAI.CreateZoneBlocks`) — **the entire zoning fabric is a by-product of the road network**; no road, no zoning, and depth is hard-capped at 4 cells (~32m).
- Buildability is geometry, not policy: flat (+/-8m), road-adjacent, in-bounds, unoccupied, inside electricity coverage (`CalculateBlock1`, `IsGoodPlace`). Painting only sets a 4-bit zone field per cell — it creates nothing and costs nothing (`ZoneTool.cs:863`).
- The growth trigger is one line: **a building spawns iff `rand(100) < demand`** (`ZoneBlock.cs:1156-1177`). `ZoneBlock` picks where/class/size; `BuildingManager.GetRandomBuildingInfo` picks the prefab; success decrements demand by 5. No money anywhere in the loop.
- RCI demand is a demographic servo: `homeless − vacancies`, `empty jobs − unemployed`, clamped 0-100, eased per tick (`ZoneManager.cs:771-810`). A closed control loop with **no monetary variable** — a market costume over a demographic thermostat.
- Dezoning is the demolition tool: a growable whose paint no longer matches self-demolishes (`PrivateBuildingAI.SimulationStep`).

### W&R — no zoning exists (**CONFIRMED-absence**)

Whole-corpus sweep of 488 `buildings_types/*.ini`: zero zoning/RCI/growable/spawn tokens (all `DEMAND` hits are `$STORAGE_DEMAND_*` supply tokens). Every building is an authored prefab the player sites by hand and construction crews physically build from hauled materials. Placement legality is player judgment plus connection reach — no grid, no paint, no demand scalar.

## Draft model (OURS, grounded in research §G)

1. **Buildings are placed by the plan, then built by crews.** No spawn path exists. The planner sites a specific building on authorised land; it is a construction site until [spec/construction.md](construction.md) completes it. (research §G1)
2. **Zoning is a planning overlay** — the general plan (генплан) marks districts residential / industrial / agricultural / mixed. It constrains what the planner may site there and surfaces mismatches. A zoned-but-empty district is *a plan not yet fulfilled* — a visible backlog, not latent demand. (research §G2)
3. **Placement validity is a physical checklist, not a grid.** Adopt CS1's validator — flat, network-adjacent, in-bounds, unoccupied, within utility reach — evaluated at the planner's cursor at siting time. Reject the road-birthed `ZoneBlock` swarm and the 32m depth cap (artifacts of the growth model). (research §G3)
4. **Shortage signals replace RCI demand.** Compute CS1's diagnostic numbers (`homeless − vacancies` per district and education tier, `empty jobs − unemployed`, service coverage gaps) and surface them as a plan-fulfillment dashboard that tells the planner what to build next — never wired to a spawner. The housing queue ([spec/households.md](households.md)) *is* residential demand. (research §G5)

## Open questions

- ~~Grid-cell zoning layer vs district-level land-use polygons?~~ Settled by research §G2-G3: district-level intent polygons + a siting-time validity checklist; no cell grid.
- Who initiates development on zoned land — player only, or a planning-office agent that *proposes* projects from the shortage dashboard for player approval?
- Is informal construction (dachas, self-built housing) in scope as a pressure valve when the plan under-delivers housing?
- Does zone mismatch ever force anything (e.g. rezoning an occupied district), or is it advisory only? (CS1's auto-despawn on mismatch is rejected — research §G7.)

## Evidence log

| Claim | Evidence level | Source | Notes |
|---|---|---|---|
| CS1 zoning grid = 8×4 `ZoneBlock` per road side, 4-cell depth | CONFIRMED | `ZoneBlock.cs:6-56`, `RoadAI.cs:235-375` | research §A1, §A4 |
| CS1 buildability = flat + road-adjacent + powered + unoccupied | CONFIRMED | `ZoneBlock.cs:229-327`, `ZoneBlock.cs:976-1020` | research §A2, §B2 |
| CS1 spawn gate is `rand(100) < demand`; success −5 demand | CONFIRMED | `ZoneBlock.cs:1156-1177`, `1586-1609` | research §B2, §B4 |
| CS1 RCI demand = demographic formulas, no money term | CONFIRMED | `ZoneManager.cs:771-810` | research §D2-D3 |
| CS1 dezoning/style-mismatch self-demolishes growables | CONFIRMED | `PrivateBuildingAI.cs:157-344` | research §C3 |
| W&R has no zoning/demand/spawn grammar | CONFIRMED (absence) | 488-file token sweep | research §E3; native runtime not decompiled |
| Zoning as planner-intent overlay + shortage dashboard | OURS | research §G1-G3, §G5 | this spec's model |

Evidence levels: CONFIRMED · OBSERVED · INFERRED · SPECULATIVE · OURS (see [spec/README](README.md)).

## Related

- [spec/buildings.md](buildings.md) — what gets built on zoned land
- [spec/construction.md](construction.md) — the process that builds it
- [spec/households.md](households.md) — housing queue as the demand signal
- Research: [research/zoning-buildings.md](../research/zoning-buildings.md)
