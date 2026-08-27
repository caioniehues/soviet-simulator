---
name: seam-simwide-structure-2026-08-27
description: Sim-wide structure fact-sheet (souls, map_dynamic, world_command, init, utils, tests) — no Refusal/Verdict type exists at all, Dispatcher seam leaks 15 free() sites into market.rs, freight_station panic path
metadata:
  type: project
---

SEAM: sim-wide structure outside the economy core — `souls/`, `map_dynamic/`,
`world.rs`, `world_command.rs`, `init.rs`, `utils/`, `multiplayer/`, `tests/`.
Mapped for the architecture-review explorer pass (bd `sov-00c`).

**Verified 2026-08-27 against commit `8531d3c` + DIRTY WORKING TREE.**
Test suite state as observed: `cargo test -p simulation` → **44 passed, 0 failed, 12.40s**.

**The tree is NOT clean and this matters.** `git status --short` shows modified:
`prototypes/src/types/recipe.rs`, `simulation/src/economy/{market.rs,mod.rs}`,
`simulation/src/map_dynamic/{mod.rs,router.rs}`,
`simulation/src/souls/goods_company.rs`,
`simulation/src/tests/scenarios/{mod.rs,recipe_provided.rs}`,
`base_mod/companies.lua`; untracked: `simulation/src/tests/scenarios/inflation.rs`.
This is sov-lpj in flight. **The entire `request_multiplier` feature is
uncommitted** — `git show HEAD:prototypes/src/types/recipe.rs | grep -c
request_multiplier` → **0**, and at HEAD `goods_company.rs:24,57` still read
`.requested(soul, item.id).unwrap_or(item.amount as u32)`. If sov-lpj is
reverted, the dishonest enterprise becomes unreachable again.
Findings below about `Refusal`/`Verdict`, the Dispatcher seam leak, the
freight-station panic, `SimDrop`, `init.rs`, and the test harness are all in
**committed** code and do not depend on sov-lpj.

## Tooling note — matters for every future agent in this repo

The **`LSP` tool was disabled session-wide** ("LSP is disabled for this session,
in subagents as well as here"), and `ToolSearch("select:LSP")` returned no match.
Meanwhile `~/.claude/hooks/lsp-first-read-guard.js` **blocks the `Read` tool on
every `.rs` file** until an LSP warmup that cannot happen. The `Grep` tool is
also absent. Net effect: **code can only be read via `grep -n "" <file>` in
Bash**, which the Grep tool's own error message explicitly sanctions. `Read`
still works fine on non-code (`.md`). Do not burn turns re-probing this.

## CONTRADICTS — claims this tree disproves

1. **`Refusal` and `Verdict` do not exist in code. At all.**
   `grep -rn "Refusal\|Verdict" --include=*.rs .` → **0 hits**.
   `docs/reference/glossary.md:44-50` ratifies both as binding vocabulary
   ("Verdict: one stated judgment of a proposed placement: approved or refused
   with a physical reason"). The substrate has neither.
   - `WorldCommand::apply` returns `()` (`world_command.rs:223`). No result type.
   - Failure is a log line: `map.rs:606` "merge refused because patterns don't
     match", `map.rs:614` "merge refused because connecting to itself". These two
     `log::info!` calls are the **only** occurrences of the word "refuse" in the
     whole workspace.
   - `world_command.rs:302-306`: the `AddTrain` arm is an **empty body** `=> {}`.
     The command is constructible (`WorldCommands::add_train`, `:141`) and does
     nothing. A dead command that reads as live.
   Any brief that says "extend the refusal path" / "return a Verdict" assumes a
   substrate that was never built. This is the `map_dynamic::Dispatcher` failure
   shape repeating.

2. **My own earlier memory was stale — corrected.** See
   [[seam-hoard-panel-story0107]]. `set_requested` is no longer test-only;
   `goods_company.rs:24` calls it in production via `recipe.request_multiplier`.

## The Dispatcher seam leak — the highest-value structural finding

`Dispatcher` (`map_dynamic/dispatch.rs:22-141`) is a genuinely **deep** module:
4-method interface (`update`/`query`/`free`/`unregister`) over ~280 lines of
BFS-over-lanes spatial caching with a `PRECISION_RADIUS` hysteresis
(`dispatch.rs:14`, 175-185).

But truck-lifetime management **leaked out of it into the economy**:
- `dispatcher.free(DispatchID::SmallTruck(v))` appears **15 times in
  `simulation/src/economy/market.rs`** (lines 313, 351, 365, 384, 400, 787, 806,
  821, 860, 908, 925, 946, 976, 999, 1036).
- `dispatcher.query(...)` appears **once** (`market.rs:766`).
- 15 releases : 1 acquire is the classic leaked-resource shape. Every one is a
  hand-audited early-exit path.

**`ParkingManagement` solved the identical problem and Dispatcher did not.**
`parking.rs:10` defines `SpotReservation(ParkingSpotID)`; `parking.rs:27-32`
`free()` consumes it by value and `std::mem::forget`s it, so the type system
tracks the reservation. `Dispatcher::free` takes `impl Into<DispatchID>` — a
plain `Copy` id, freeable twice, never, or by anyone. Same problem, two
different answers, in sibling modules.

## Panic paths — both violate the "never game over" pillar

1. **`freight_station.rs:109`**:
   `let ext = *map.external_train_stations.first().unwrap();`
   That vec is drained on demolish at `map.rs:135`
   (`self.external_train_stations.retain(|id| *id != b.id)`).
   Demolishing the last external station while a train is in
   `FreightTrainState::Loading` panics the simulation. Reachable by a Planner
   with a bulldozer.
2. **`goods_company.rs:55`** — **LATENT, NOT LIVE. Corrected after checking the
   guard shape; do not repeat the overstated version.**
   `market.requested(soul, item.id).unwrap()` where `Market::remove` erases
   `requested` (`market.rs:281`), and `recipe_act` runs **deferred** via
   `cbuf.exec_on` (`goods_company.rs:213-216`). I first called this a live panic.
   It is not, today: the only `CompanyEnt` kill is `goods_company.rs:193-196`
   (`unwrap_or!(map.buildings.get(...), { cbuf.kill(me); return; })`) — the
   `return` makes kill and the recipe block at `:201` mutually exclusive within a
   pass, and `ParCommandBuffer::apply` (`par_command_buffer.rs:62-81`) drains all
   kills **before** all execs, so they never interleave. The unwrap is
   unreachable **by construction of the current single call site**, not by any
   invariant the type system holds. It becomes a live panic the day a second
   company-kill path appears. (This is uncommitted sov-lpj code; at HEAD it is
   still `.unwrap_or(item.amount as u32)`.)

`unwrap()`/`expect()` counts in the lens: dispatch.rs 24 (mostly its own
`#[cfg(test)]` block), freight_station.rs 7, goods_company.rs 3,
itinerary.rs 2, electricity.rs 1.

## The test harness cannot observe through any interface

`TestCtx` (`tests/mod.rs:19-121`) provides only **drivers**: `build_roads`,
`build_house_near`, `build_house_at`, `apply`, `tick`, `advance_ticks`,
`check_determinism`. **Zero observation methods.** So every scenario reaches
past it into `ctx.g`:

| reach | count across `tests/scenarios/*.rs` |
|---|---|
| `ctx.g.read::<Market>` | 54 |
| `ctx.g.write::<Market>` | 27 |
| `ctx.g.map` | 22 |
| `ctx.g.read::<BuildingInfos>` | 21 |
| `ctx.g.map_mut` | 6 |
| `ctx.g.write::<BuildingInfos>` | 5 |
| `ctx.g.world_res` | 5 |
| `ctx.g.world` | 5 |
| `ctx.g.world_mut_unchecked` | **3** |

`world_mut_unchecked` (`lib.rs:210`) appearing in tests is the tell.

**`TestCtx::new()` forces an inherited world.** `Simulation::new_with_options`
replays the hardcoded `START_COMMANDS` JSON blob (`lib.rs:443`, ~370 lines
embedded in the source) which seeds a rail connection, an `ExternalTrading`
building and a freight station. A scenario that must not have one has to
**demolish it**: `tests/scenarios/inflation.rs:39-54`
`remove_default_freight_station()` finds the station, applies
`MapRemoveBuilding`, ticks once, and asserts the entity is gone. That helper is
pure friction created by the harness.

`build_company_at` is **duplicated verbatim** in `recipe_provided.rs:18` and
`inflation.rs:56` (the latter with one extra `connected_road` arg). `mk_human`
is duplicated in `ledger.rs:24` and `retail.rs:21`.

## PROVIDED — deep modules that earn their keep (do not "simplify" these)

- **`SimDrop`** (`utils/par_command_buffer.rs:6-8`, impls at `world.rs:68,107,139,154,164,193`).
  One despawn-cleanup site per entity type. `HumanEnt::sim_drop` (`world.rs:107-126`)
  removes the transport-grid collider, calls `Market::remove` with 6 args, and
  frees parking reservations. Deletion test: **passes hard** — the alternative is
  that cleanup logic spread across every `kill()` call site.
- **`ParkingManagement`** (`parking.rs`) — token-typed reservations, 7-deep BFS
  spot search (`:58-111`). Small interface, real behaviour.
- **`Resources`** (`utils/resources.rs`) — TypeId→RwLock registry. Caveat:
  `read`/`write` (`:80,:92`) `.unwrap()` on a missing resource, so reading an
  unregistered resource panics; `try_read`/`try_write` exist but are barely used.
- **`init.rs` registry** — the `static mut` race is genuinely **fixed**: it is now
  a `std::sync::OnceLock<Registry>` (`init.rs:168`), populated once in `init()`
  and read through `init_funcs()`/`saveload_funcs()`/`gsystems()` (`:170-180`).
  Registration order at `:54-114` **is** the tick schedule.

## PRESENT-BUT-DEAD / near-dead

- `WorldCommand::AddTrain` — empty match arm (`world_command.rs:302-306`).
- `multiplayer/` is **NOT dead** (I expected it to be): 53 lines total, but
  `native_app/src/gui/hud/chat.rs:28,50,100` reads `MultiplayerState`, calls
  `messages_since`, and pushes `WorldCommand::SendMessage`. Reachable, in-game.
- `Replay`/`SimulationReplayLoader` reachable via
  `native_app/src/gui/hud/windows/load.rs:52`.

## Shallow modules

- `souls/desire/home.rs` — 28 lines. `Home::score()` returns the **constant 0.2**
  (`:26`). `Home::apply()` is one line. The whole "Home desire" is a
  `BuildingID` plus a magic number.
- `souls/desire/mod.rs` — 7 lines, pure re-export.
- The desire tournament in `update_decision` (`human.rs:193-234`) is a
  **hand-unrolled** max-by-score over exactly three `Option`s, written three
  times with copy-pasted `if score > max_score` blocks, then re-matched in a
  second `match` block. Adding a fourth desire means editing both blocks.
  `update_decision` takes **15 arguments** (`human.rs:166-182`).

## The determinism check does not check determinism

`TestCtx::check_determinism` (`tests/mod.rs:106-119`) encodes the sim, decodes
it, and compares `hashes()` of the two. That proves **serde round-trip
stability** — it never runs the sim twice. Two seeded runs that diverged would
pass it every time. This independently confirms `substrate.md`'s determinism row
("No cited check proves repeat-run determinism; the current helper proves
serialization round-trip stability only" — classified **Absent**). The method
name is the trap: a brief that says "the determinism harness already covers
this" is wrong about what it covers.

## LUA

`base_mod/companies.lua` — **26** companies (`grep -c 'type = "goods-company"'`
→ 26; a `grep -c '^        name = '` gives 27 because it also catches a nested
`bgen` name — use the `type =` count). `request_multiplier` declared by
exactly **2**: `flour-factory = 4` (`:40`), `meat-facility = 3` (`:582`).
**Both lines are uncommitted** (`git diff HEAD base_mod/companies.lua` shows
both as `+`).
Default is `1` via `.unwrap_or(1)` at `prototypes/src/types/recipe.rs:63`, so
the other 25 are honest. Doc comment on the field (`recipe.rs:48-52`) states the
design intent: "1 = honest. >1 = inflates its requirement and hoards the surplus.
There is no honesty flag."

## Where the primary sources live
- Schedule/registration: `simulation/src/init.rs:33-145`.
- Tick loop: `simulation/src/lib.rs:235-262`; command-first, then GameTime++, then
  `SeqSchedule::execute`.
- Command-buffer drain: `utils/scheduler.rs:41-56` — six `ParCommandBuffer::<E>::apply`
  calls **hardcoded by entity type**, after *every* system.
- Player action surface: `world_command.rs:35-100` (21 variants), `apply` at `:223-382`.
- Desire pipeline: `souls/human.rs:135-235`, `souls/desire/{home,work,buyfood}.rs`.
- Harness: `simulation/src/tests/mod.rs:19-121`; scenario corpus in
  `simulation/src/tests/scenarios/` (5 files, ~2260 lines).
