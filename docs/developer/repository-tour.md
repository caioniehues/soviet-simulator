# Repository tour

**Kind:** guide
**Authority:** operational
**Status:** active
**Owner:** project lead
**Verified-at:** `4e9e930b2a73`
**Last verified:** 2026-08-28

## Crates

| Crate | What | Owner lane (dev cycle) |
|---|---|---|
| `simulation/` | The sim: map, pathfinding, transportation, souls (citizens, companies, freight stations), economy, tests | `sim-implementer` |
| `native_app/` | The game binary: game loop, UI (yakui + egui), inspectors, tools, rendering glue, network client | `ui-implementer` |
| `engine/` | wgpu renderer, terrain, PBR, LOD, GPU timing, capture, input, audio | `engine-implementer` |
| `engine_demo/` | Renderer demo | `engine-implementer` |
| `prototypes/` | Lua-driven data definitions: items, companies, recipes, vehicles, rolling stock; validation | `data-implementer` |
| `base_mod/` | The Lua data itself (`items.lua`, `companies.lua`, `rollingstock.lua`, `roadvehicles.lua`, `leisure.lua`, `colors.lua`, `data.lua`) | `data-implementer` |
| `geom/` | Vectors, matrices, splines, polygons, frustum, heightmap, noise — determinism-critical | `geom-implementer` |
| `common/` | Timestep, save/load, hashing, RNG, logging, history buffers | `common-implementer` |
| `headless/` | Headless runner and multiplayer server | `common-implementer` |
| `networking/` | Lockstep client/server, authentication, packets, world send, catch-up | `net-implementer` |
| `goryak/`, `egui-inspect/`, `egui-inspect-derive/`, `assets_gui/` | Reusable widgets, inspection derive, asset viewer | `widget-implementer` |

## Inside `simulation/src`

```text
lib.rs            Simulation, tick, save/load (SimulationSer), hashes, START_COMMANDS
init.rs           system registration order (the schedule), resource registration
world.rs          World: one HopSlotMap per entity type; HumanEnt, VehicleEnt, TrainEnt, …
world_command.rs  WorldCommand — the only way input enters the sim
economy/          market.rs (orders, matching, dispatch, retail, ext-trade, prices), government.rs, ecostats.rs
souls/            human.rs, goods_company.rs, freight_station.rs, desire/{home,buyfood,work}.rs
map/              map.rs, objects/ (lane, road, lot, parking, building, …), pathfinding.rs,
                  electricity_cache.rs, traffic_control.rs, terrain.rs, procgen/
map_dynamic/      dispatch.rs, electricity.rs, itinerary.rs, router.rs, parking.rs, binfos.rs
transportation/   vehicle.rs, road.rs, train.rs, pedestrian.rs
utils/            scheduler.rs (SeqSchedule), par_command_buffer.rs, rand_provider.rs, replay.rs, resources.rs
tests/            mod.rs (TestCtx), scenarios/, test_iso.rs, vehicles.rs, world_replay.json
multiplayer/      chat
rerun.rs          dead (commented out)
```

The [current substrate](../architecture/current-substrate.md) says what each of these provides
and does not.

## Data flow in one tick

`WorldCommand::apply` → time advances → eighteen systems in registration order, each followed by
`ParCommandBuffer::apply` → UI reads `Simulation` through `Arc<RwLock<_>>`
([simulation phases](../architecture/simulation-phases.md)).

## Documentation

`docs/index.md` is the front door; `docs/SUMMARY.md` the navigation; `docs/meta/document-authority.md`
the rules. Root files: `README.md` (status), `CLAUDE.md` and `AGENTS.md` (agent entry points),
`CONTEXT.md` (glossary redirect), `book.toml` (mdBook).

## Tooling

`.beads/` — `bd` task tracker (versioned export `issues.jsonl`). `deny.toml` — dependency policy.
`.github/workflows/` — `dependency-policy.yml`, `docs.yml`. `scripts/check_docs.py`.
`.claude/agents/` — the 22-agent roster. A knowledge-graph MCP (`code-review-graph`) indexes
the code; `docs/reference/code-intelligence.md` says when to use it versus LSP.

## Related

- [Getting started](getting-started.md)
- [Current substrate](../architecture/current-substrate.md)
- [Development cycle — Phase 2 lanes](../process/development-cycle.md)
