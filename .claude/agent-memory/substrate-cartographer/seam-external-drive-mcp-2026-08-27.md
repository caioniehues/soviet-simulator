---
name: seam-external-drive-mcp-2026-08-27
description: Can an out-of-tree crate own a Simulation and drive it? Verified at 0aa5c35 — YES, but WorldCommand::apply returns () and reports no failure; that is the load-bearing gap for any MCP test server
metadata:
  type: project
---

Verified **2026-08-27** against commit **`0aa5c35`** (clean tree). Read via `ct view` / `/usr/bin/grep`.
Question: feasibility of a NEW workspace crate owning a `Simulation` directly (not networking, not
inside `native_app`) so an agent can drive and observe the game.

## Verdict: QUALIFIED YES

`headless/src/main.rs:35-44` already proves the sequence with zero UI:
`simulation::init::init()` → `Simulation::load_from_disk("world")` or `Simulation::new(true)` →
`Simulation::schedule()` → `w.tick(&mut sched, cmds)`. The `Server<Simulation, WorldCommands>` at
`:46` is the only networking part and is fully separable.

## The single biggest gap — commands cannot report failure

`simulation/src/world_command.rs:223` — `pub fn apply(&self, sim: &mut Simulation)`. **Returns `()`.**
Every failure is silent or log-only:
- `:236` `MapRemoveRoad(id) => drop(sim.map_mut().remove_road(id))` — `Option<Road>` **dropped**
- `:250` `make_connection(...)` return value discarded
- `:291` `build_special_building(...)` — `if let Some(id)`, else **nothing**, not even a log
- `:224-225` cost is subtracted **unconditionally**; `Government.money` can go negative. No command
  is ever refused for cost (consistent with the charter's "money is not a gate").

Consequence for an MCP: the ONLY way to know a build succeeded is to diff `Map` before/after.

## PRESENT-BUT-DEAD command variant

`world_command.rs:302-306` — `AddTrain { dist, n_wagons, lane } => {}`. Empty match arm, all three
fields bound to `_`. **It costs money** (`government.rs:25`: `1000 + 100*n_wagons`) and does nothing.
`SpawnTrain` (`:307-313`) is the live one. Do not let a brief say "AddTrain spawns a train".

## Mutation paths that BYPASS WorldCommand (Q: MCP blind spots)

`Simulation::map_mut()` is `pub(crate)` (`lib.rs:339`) — but `pub fn write<T>()` (`lib.rs:327`) is a
public generic over any `Any + Send + Sync`, so `sim.write::<Map>()` gives the same `RefMut<Map>`
from outside the crate. Together with `world_mut_unchecked()` (`lib.rs:210`) and `world_res()`
(`lib.rs:203`), an external crate has **total unrestricted write access** to world and resources.
`Map`'s mutators are all `pub` (`map/map.rs:93,128,150,217,245,300,331,395`).

In the shipping game all player mutation does go through `WorldCommand`
(`native_app/src/network.rs:53,70`), so `WorldCommand` is a complete *player* vocabulary — but not a
complete *mutation* vocabulary.

## Determinism — better than expected

- `SeqSchedule::execute` (`utils/scheduler.rs:39-57`) is a plain sequential `for`. No rayon.
- Only rayon in the whole sim crate: `map/terrain.rs:66` `into_par_iter().map().collect()` —
  order-preserving, terrain gen only.
- `Instant::now` appears only for timing/logging (`scheduler.rs:42`, `lib.rs:241,354,395`,
  `world_command.rs:387`). Never feeds sim state.
- `lib.rs:3` `#![warn(clippy::iter_over_hash_type)]` — the codebase actively lints against
  HashMap-iteration nondeterminism.
- Seed: `RNG_SEED = 123` (`lib.rs:112`), `SimulationOptions.seed` pub (`lib.rs:120`), applied via
  `Init` → `RandProvider::new(opts.seed)` (`world_command.rs:331-332`).
- `Simulation::hashes() -> BTreeMap<String,u64>` is **pub** (`lib.rs:268`) — a ready-made state
  fingerprint for an MCP.

Two different "determinism" checks, do not conflate:
- `tests/mod.rs:106-120` `check_determinism` — encode/decode round-trip, compares `hashes()`,
  **does `assert_eq!`**. Proves serde fidelity only.
- `tests/test_iso.rs:241-306` `test_world_survives_serde` — two sims from one replay, but every
  mismatch branch (`:276,:284,:292`) does `continue 'main` with `check_size /= 2`; `:253` then
  breaks and the test returns green. **Re-verified unchanged at 0aa5c35.** See
  [[seam-persistence-determinism-2026-08-27]].

## Init contract

`simulation/src/init.rs:33-145`. Required — `init_funcs()` (`:171`) does
`.expect("init() not called")`. Idempotent by `OnceLock`: `init.rs:144` `let _ = REGISTRY.set(..)`
and `load.rs:33` `let _ = PROTOTYPES.set(p)`. A second call re-parses Lua and `Box::leak`s a
discarded `Prototypes` (`load.rs:69`) — wasteful, not racy. `TestCtx` wraps it in `Once`
(`tests/mod.rs:26,31`).

**CWD trap:** `init.rs:39-42` — `#[cfg(not(test))] base = "./"`, `#[cfg(test)] base = "../"`.
`load.rs:26,30` then reads `<base>base_mod/?.lua` and `<base>base_mod/data.lua`. `base_mod/` lives at
the repo root. A non-test binary therefore **must run with CWD = repo root** or `init()` panics
(`init.rs:47`).

`native_app/src/init.rs:85-86` `pub static mut INIT_FUNCS/SAVELOAD_FUNCS` — **CONFIRMED still
present** at 0aa5c35. The sim side is fixed (OnceLock); the UI side is not.

## Save/load

`lib.rs:295` `save_to_disk` → `CompressedBincode` = bincode `DefaultOptions` + zlib level 1
(`common/src/saveload.rs:128-136`), extension `.zip`. Path is hardcoded
`world/{name}.{ext}` (`saveload.rs:49-51`) relative to **CWD**; `save_silent` does
`std::fs::create_dir("world")` (`:65`). **No save-directory seam exists** — an MCP must chdir or
add one. Replay saved alongside as `{name}_replay.json` when enabled (`lib.rs:297-300`).
Version gate warns only, and ignores the patch field (`lib.rs:404-415`).

## Missing accessors (smallest useful set)

1. `WorldCommand::apply` → return a result. Everything else is a workaround for this.
2. `Dispatch.truck` / `Dispatch.ticks_left` are **private** (`economy/market.rs:188,191`) —
   `market.dispatches()` (`:450`) returns `&[Dispatch]` but cargo-to-truck custody is unreadable.
3. `Replay.commands` private (`utils/replay.rs:10`), only `push`. No way to read back the command
   log except via serde.
4. `Dispatcher.dispatches` private (`map_dynamic/dispatch.rs:24`), no reader for `reserved_by`.
5. A save-path parameter (see `saveload.rs:49`).

## What is ALREADY public and sufficient

`World` (`world.rs:208-214`) has six `pub` HopSlotMaps and `sim.world()` is pub (`lib.rs:207`).
`Market` query API is fully pub: `capital` `:425`, `requested` `:446`, `reserved` `:432`,
`dispatches` `:450`, `retail_claim` `:456`, `inner` `:724`, `iter` `:220`. The dishonest-enterprise
loop (requested vs consumed) is observable with **zero new API**: `requested` from the market,
`consumption[i].amount` and `request_multiplier` from `prototypes/src/types/recipe.rs:52`.

Related: [[seam-persistence-determinism-2026-08-27]], [[seam-economy-logistics-2026-08-27]],
[[seam-simwide-structure-2026-08-27]].
