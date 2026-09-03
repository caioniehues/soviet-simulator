# ADR-0002: The fixture world is a materialised replay, never an authored save

**Kind:** decision
**Authority:** binding
**Status:** accepted
**Owner:** project lead
**Last verified:** 2026-09-03
**Date:** 2026-09-03
**Decision makers:** project lead (caioniehues), interviewed by the lead agent

## Context and problem

The delivery rule is that progress is judged from the running game, and planner-facing UI tickets
carry "one inspected live state" acceptance criteria. The only save on disk (`world/world.zip`)
loads an empty world: forest, water, one freight station, an Economy panel at 0. Every panel that
renders per-company or per-economy state draws nothing, so the whole class of tickets is unprovable
by construction (`sov-rvu`, blocking `sov-dda.4` and `sov-uy2`).

Facts established 2026-09-03 (read-only, cited):

- `native_app/src/game_loop.rs:53-54` starts with `load_from_disk("world").unwrap_or_else(|| new(true))`.
  There is no flag, menu, or environment override; a missing save silently becomes an empty world.
- `world/` is git-ignored (`.gitignore:21-22`). Saves are `CompressedBincode` versioned by major
- `simulation/src/tests/world_replay.json` (committed) records 67 `WorldCommand`s over 200,000
  ticks: 22 road/rail links (including a driving-road spur to the freight station, without
  which the border stays closed and no external trade exists), 30 lot-free houses, 12 companies,
  one freight station, one train. (At decision time it was 91 commands over 10,000 ticks; it was
  re-recorded twice by the scenario builder per §Decision outcome 3 — once for lot-free
  placement and a 35k tail, once more for the freight-station spur and a 200k tail, because
  import legs need ~10-20k ticks each way and domestic cycles need ~50k ticks to reach steady
  trade.) `test_world_survives_serde` replays it through `Simulation::schedule()` and the World
  populates to 30 humans, 37 vehicles, 12 companies, all employed, no blackout.
- The old replay's 42 houses were `MapBuildHouse(LotID)` (`world_command.rs:40`): they depended
  on auto-generated lots (`Lot::generate_along_road`, `map/map.rs:719`), which
  `sov-harness-lots-k54` / STORY-0013 is removing. The re-recorded replay uses only
  `MapBuildSpecialBuilding` houses, so this dependence is gone; the census guard (§Decision
  outcome 4) fails loudly if a future replay ever goes hollow again.
- The old replay's roads were all rail, so trucks, parking and the router were unexercised;
  the re-recorded replay has 18 non-rail roads.
- The Economy panel's level-0 history needs `128 × LEVEL_FREQS[0] = 32,000` ticks to fill
  (`simulation/src/economy/ecostats.rs:11`); the re-recorded replay runs 200,000, landing the
  level-0 window inside steady trade (imports ≈ −1281$ and exports ≈ +312$ in the window;
  domestic trades carry no money by construction — `market.rs:539-545` — so internal-trade
  money is always zero and the Economy panel's Expenses/Income prove border trade, not
  domestic flow).
- The in-game load window (`native_app/src/gui/hud/windows/load.rs:56`) plays the replay with
  `SeqSchedule::default()`, the same empty-schedule bug `sov-n8v` fixed in the test.

## Decision drivers

- No competing truth: the substrate already has one canonical populated city, the replay the
  determinism gate guards. A hand-authored save beside it would drift.
- Reproducibility across save-format breaks: `WorldCommand`s are the stable seam; binary saves are
  not.
- The failure mode to eliminate is *silent emptiness*, at startup and in fixtures.
- The fixture must not depend on auto-lot generation.

## Considered options

1. Commit `world/world.zip` (un-ignore it) and regenerate by hand after gameplay.
2. Commit only the replay; derive the save by materialising it through the real schedule, with a
   local cache.
3. Both: a committed save plus the replay.

## Decision outcome

Option 2, with these bindings:

1. **Fixture world** is the glossary term: a populated world derived from a committed replay,
   never authored directly. `simulation/src/tests/world_replay.json` is the only replay; it
   serves the determinism gate and the UI fixture alike.
2. **Startup fallback chain** in `native_app`: `world/world.zip` if present and loadable →
   otherwise materialise the committed replay through `Simulation::schedule()`, write it to
   `world/world.zip` as a cache, and use it → otherwise `Simulation::new(true)`. The documented
   way to load the fixture is: delete `world/world.zip` and start the game.
3. **The replay is re-recorded programmatically** (not by hand), by a scenario builder in the
   crate that issues `WorldCommand`s (never mouse input): houses via
   `MapBuildSpecialBuilding { kind: House }` with explicit footprints (lot-free), vehicle roads
   as well as rail so trucks run, the existing companies, freight station and train, a
   driving-road spur to the freight station so the border opens, and a tail long enough that
   trade reaches steady state (200,000 ticks). The tail must stay inside the fluid phase:
   past ~300k ticks corridor convoys freeze (`sov-aam`), so do not lengthen it past ~250k
   until that is fixed. The builder is the
   canonical definition of the minimum city and is the only sanctioned way to regenerate the
   replay; the determinism baseline moves deliberately, and the commit says so.
4. **Census guard**: the determinism gate asserts, at the end of the replay, minimum counts of
   humans, vehicles and companies, so a hollow replay fails loudly.
5. **The in-game load window** plays replays through `Simulation::schedule()`.

Why: the bead was written before the gate was real. Now that the replay demonstrably builds a
working city under the real schedule, authoring a second world by hand is the competing-truth
mistake the model rules forbid. Materialising costs minutes once per machine and is cached.

## Consequences

- `world/world.zip` stays git-ignored; new contributors get a populated world on first run.
- The determinism baseline moves when the replay is re-recorded. `docs/developer/debugging-determinism.md`
  already permits this when intended and stated.
- Materialising ≈200k ticks takes a few minutes on first launch without a cache (the
  determinism gate covers the same ground in ~165 s debug); the cache removes it thereafter.
  If this proves too slow, the tail length is the knob, not the mechanism — but keep it inside
  the fluid phase (`sov-aam`).
- `sov-harness-lots-k54` gains a second consumer that is already lot-free.
- Cost: one scenario builder to write and keep aligned with the command schema.

## Confirmation

- Starting `native_app` with no `world/` directory produces a populated world: Economy panel shows
  non-zero Expenses and Income; the building inspector shows per-company recipe and workers.
- `world_replay.json` contains no `MapBuildHouse` command.
- `cargo test -p simulation test_world_survives_serde` passes with the census guard active.
- `docs/developer/getting-started.md` (or its successor) documents the one-line load procedure.

## More information

- `bd show sov-rvu` and its comments record the interview.
- [ADR-0001](0001-households-and-utilities-are-1.0-scope.md) fixed the scope vocabulary this
  record relies on.
- `sov-n8v` / commit `7e771ce`: the gate this fixture shares.
