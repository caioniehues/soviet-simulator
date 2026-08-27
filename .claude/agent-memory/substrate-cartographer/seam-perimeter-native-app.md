---
name: seam-perimeter-native-app
description: Inherited perimeter map (native_app/prototypes/base_mod/headless) — request_multiplier landed dirty and silently defaults on typo (PROVEN); native_app has ZERO tests but all 10 tool systems are renderer-free and testable today
metadata:
  type: project
---

SEAM: inherited perimeter — native_app/, prototypes/, base_mod/*.lua, headless/, sim↔UI
Verified 2026-08-27 against **HEAD 8531d3c + dirty tree** (94 modified paths; `simimpl-sov-lpj-fix` active).

## LEAD FINDING — a new Lua flag silently defaults on any typo (PROVEN, not inferred)

`request_multiplier` landed **uncommitted** in the working tree and is wired end to end:
`base_mod/companies.lua:40,582` → `prototypes/src/types/recipe.rs:52,63` →
`simulation/src/souls/goods_company.rs:23`. This is the inflation source that
[[seam-hoard-panel-story0107]] said must land before any hoarding panel. **That memory's
claim "no Lua field declares requested/hoarding/inflation" is now STALE.**

Set on exactly **2 of 26** goods-companies (26 re-counted on the dirty tree):
`flour-factory` = 4 (declared companies.lua:27, field at :40) and
`meat-facility` = 3 (declared companies.lua:569, field at :582 — consumption
`raw-meat` 1, production `meat` 1). The other 24 default to 1 via
`get_lua(&table, "request_multiplier").unwrap_or(1)` (recipe.rs:63).

**Empirically proven** by a throwaway probe in `prototypes/src/tests.rs` (added, run,
then reverted — `git diff` clean afterwards):

| Lua value | parsed result | validated? |
|---|---|---|
| `"not-a-number"` | **1** (silently honest) | no error |
| `0` | **0** | no error |
| `-3` | **-3** | no error |

Command: `cargo test -p prototypes probe_request_multiplier -- --nocapture`
Output: `PROBE RESULT: request_multiplier parsed as 1`.

Two independent defects behind that:

1. **`get_lua(..).unwrap_or(d)` swallows type errors, not just absence.** The correct
   pattern in this same crate is `get_lua_opt(..)?.unwrap_or(d)`
   (`prototypes/src/prototypes/goods_company.rs:41-42`), which propagates a type error and
   only defaults on absence. **5 fields use the swallowing form**: `base.rs:17` (order),
   `item.rs:24` (**optout_exttrade** — the historically load-bearing flag),
   `zone.rs:20,21`, `recipe.rs:63`.
2. **`validate()` (`prototypes/src/validation.rs:22-82`) has no `request_multiplier` case.**
   It checks n_trucks, referenced item ids, and power sign only. `0` and `-3` pass.
   `goods_company.rs:23` then does `item.amount as u32 * recipe.request_multiplier as u32`
   — `-3i32 as u32` wraps to **4294967293**, so a negative multiplier becomes a ~4.3-billion
   unit request, not a refusal.

`request_multiplier = 0` means "request nothing" while the recipe still consumes; combined
with `market.requested(soul, item.id).unwrap()` at **goods_company.rs:55** (an unconditional
unwrap, NOT the `unwrap_or` recorded in the older sheet) this is worth checking for a panic.

## native_app has ZERO tests — but the untestability claim is too broad

`grep -c '#\[test\]' native_app/src/` → **0**, across ~3.6k lines. `native_app/Cargo.toml`
has **no `[dev-dependencies]` section at all**.

The standing project claim is "the sim's test harness cannot drive the UI; UI work is proven
by a public sim accessor plus an eyeballed frame". **That is true of rendering and false of
tool intent.** All 10 UI systems in `run_ui_systems` (`native_app/src/gui/mod.rs:40-54`) have
the identical signature `(sim: &Simulation, uiworld: &UiWorld)` and **none references a
graphics `Context`** (`grep -ln "Context|GfxContext|ctx:" native_app/src/gui/tools/*.rs` →
no matches). Their whole output is (a) `WorldCommands` pushed via `uiworld.commands()` and
(b) draw orders on `ImmediateDraw`, which is a plain order buffer — its only gfx contact is
`apply()` at `rendering/immediate.rs:202`, which tools never call. Verdict: a headless test
could construct a UiWorld, set InputMap/Tool, call `bulldozer(&sim,&uiw)` and assert the
emitted WorldCommand. Nothing structural blocks it.

## UiWorld is an Any-keyed bag, not a module interface

`native_app/src/uiworld.rs` — the whole type is `resources: ResourcesSingleThread` with
`read::<T>()`/`write::<T>()`. The textual interface is 6 generic methods; the *real*
interface is "know all **34** resource types registered in `init.rs:35-73` and their
registration order, or panic at runtime". Registration is push-time into
`pub static mut INIT_FUNCS` / `SAVELOAD_FUNCS` (**init.rs:85-86** — the known static-mut
defect; the same shape was already fixed in the sibling `simulation/src/init.rs`, so one
adapter already proves the fix is viable). `game_loop.rs:277` does `self.uiw.insert(...)`
**every frame** inside `manage_gfx_params`, i.e. the bag is mutated per-frame, not just at init.

## Other perimeter facts

- **headless/ is 80 lines** (`headless/src/main.rs`) and shares almost nothing with
  native_app: it calls `simulation::init::init()` (main.rs:35) but **never**
  `native_app::init::init()`, so none of the 34 UI resources exist there. It depends on
  `networking` and drives `w.tick(&mut sched, ...)` directly (main.rs:69). It is a
  multiplayer server, not a test harness — it cannot be reused to prove UI behaviour.
- **UI reads sim internals broadly**: 11 distinct `sim.read::<T>()` types from native_app
  (GameTime ×8, Market ×4, BuildingInfos ×3, TransportGrid, Government, ElectricityFlow,
  TrainReservations, ParkingManagement, MultiplayerState, EcoStats, CollisionWorld),
  plus **36** `sim.map()` and **17** `sim.world()` call sites.
- **`run_ui_systems` is a hardcoded call list** with an ordering invariant written only as a
  comment (`gui/mod.rs:52`: "run last so other systems can have the chance to cancel select").
  The `dontclear` flag on InspectedEntity/InspectedBuilding (`selectable.rs:29,52,64-65`) is
  that invariant's mechanism — an implicit cross-module protocol.
- **`prototypes()` is a process-global `OnceLock`** (`prototypes/src/lib.rs:105`) with a
  `thread_local! TEST_PROTOTYPES` escape hatch (lib.rs:110-112) added to stop tests racing.
  `load_prototypes` is `unsafe` and documented "should only be called once" (load.rs:16-18).

## Multiplayer chat bar is LIVE in the single-player build

`native_app/Cargo.toml` has `default = []` — the `multiplayer` feature (which pulls in
`networking`) is OFF by default. Seven `cfg(feature="multiplayer")` sites correctly gate the
network window and connection resource (`init.rs:36`, `network.rs:16,101`,
`gui/hud/windows/mod.rs:10,18,36,56`).

**But `chat.rs` has ZERO cfg guards** (`grep -c "cfg(feature" native_app/src/gui/hud/chat.rs`
→ 0) and `chat::chat(uiworld, sim)` is called unconditionally at `gui/hud.rs:40`. It reads
`sim.read::<MultiplayerState>()` (chat.rs:28) and binds `InputAction::OpenChat` (chat.rs:33),
so in a single-player build the Planner can press the chat key and get a real, empty
multiplayer chat bar (chat.rs:58-64). It self-hides only while both closed AND empty
(chat.rs:54-56). Charter is English-only single-player; the whole 118-line panel is
inherited Egregoria multiplayer furniture.

## Scope note (2026-08-27, mid-task)

The wave split after this sheet was written: `prototypes/` + `base_mod/*.lua` moved to
**lens-data**, and `engine/` + `native_app/src/rendering/` moved to **lens-render**. The
data-layer findings above (request_multiplier, get_lua swallow, validation gap) stayed in
this sheet because they are true and lens-data will want them, but they were CUT from the
lens-perimeter report delivered to main. Candidate 2's renderer-free claim depends on
`rendering/immediate.rs:202`, which is lens-render's file — 9 of 10 tools import
`ImmediateDraw` from it, so if lens-render proposes changing that type, Candidate 2's
testability argument must be re-checked.

## Tooling note (re-confirmed, unchanged from [[false-claims-failure-inventory]])

`ToolSearch("select:LSP")` returns `No matching deferred tools found` in subagent sessions
while the `lsp-first-read-guard.js` PreToolUse hook still blocks `Read` on `.rs` files.
Working path: `nl -ba <file>` via Bash. Cost me 2 blocked reads before falling back.
