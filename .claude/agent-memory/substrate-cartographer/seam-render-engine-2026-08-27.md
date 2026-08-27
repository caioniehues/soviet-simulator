---
name: seam-render-engine-2026-08-27
description: Render/engine seam fact-sheet — art-direction.md's "no palette authority" claim is FALSE (colors.lua is one); seasons ABSENT (0 hits); entity_render scans all entities per frame; headless is NOT a render adapter
metadata:
  type: project
---

Verified 2026-08-27 at commit `8531d3c` (working tree dirty, agent-memory + .claude/agents only).
LSP tool was DISABLED this session; used Read + bash grep.

## FALSE CLAIM FOUND (highest value)

`docs/reference/art-direction.md:52-53` asserts:

> "No single current module enforces this complete palette. Treat any additional central
> material authority as a future implementation decision, not an existing contract."

**This is false for map geometry.** `simulation::colors()` (`simulation/src/lib.rs:59`) returns a
live `ColorsPrototype` with 16 colour fields (`prototypes/src/prototypes/colors.rs:10-33`), parsed
from `base_mod/colors.lua`. `native_app/src/rendering/map_rendering/map_mesh.rs` consumes 8 of them
at `:601-604` (road_low/mid/hig/line), `:765-767` (lot_unassigned/residential), `:783`
(road_pylon_col), `:981` (road_mid_col). It IS a central palette authority for roads,
intersections, lots and pylons.

The doc's *other* half is correct: `src/game/palette.rs` genuinely does not exist (that was the
discarded Bevy track).

## NAMING TRAP — two different "palettes"

- `simulation::colors()` → Lua-declared `ColorsPrototype`, 16 colour fields. Data palette.
- `gfx.palette()` / `gfx.palette_ref()` (`engine/src/gfx.rs:578,585`) → an `Arc<Texture>` for
  `assets/sprites/palette.png`, used as the houses material (`map_mesh.rs:154`). Texture atlas.

A brief that says "add a palette seam" must say which one. They are unrelated.

## SEASONS — ABSENT, zero substrate

`grep -rin season simulation/src native_app/src engine/src assets/shaders base_mod prototypes/src`
= **1 hit**, and it is the surname "Season" in `simulation/src/souls/names.txt:18190`. No season
type, field, uniform or shader input exists. A charter stake with nothing under it.

## DAY/NIGHT — exists, but no seam; formula duplicated x3 verbatim

Sun direction + colour computed in `native_app/src/game_loop.rs:270-309` (`manage_gfx_params`),
hardcoded 8-hour offset at `:272`. The identical expression

```rust
params.sun_col = 4.0 * sun.z.max(0.0).sqrt().sqrt()
    * LinearColor::new(1.0, 0.95 + sun.z * 0.05, 0.95 + sun.z * 0.05, 1.0);
```

appears character-for-character in `assets_gui/src/main.rs:72-75` and
`engine_demo/src/main.rs:106-109`. Three `engine::framework::State` adapters exist
(`engine/src/framework.rs:16`) — a real seam — but each re-derives atmosphere by hand.

`RenderParams` (`engine/src/gfx.rs:183-202`) is a flat 19-field `#[repr(C)]` uniform hand-mirrored
in `assets/shaders/render_params.wgsl` with **no cross-check** beyond a `size_of < 1024` assert at
`gfx.rs:205`. Adding a field means editing both sides; a mismatch is silent.

## 250k CITIZEN TARGET — per-frame full scan

`native_app/src/rendering/entity_render.rs:63-107` iterates **all** `sim.world().vehicles`,
**all** `.wagons`, **all** `.humans` every frame. The `Location::Outside` check at `:100` filters
what is *drawn*, not what is *scanned*. `query_trans_itin` (`simulation/src/world.rs:273`) chains
humans+vehicles+trains for the path-not-found sprite at `:110`.

Contrast: `trees.rs:103` frustum-culls and `map_mesh.rs:606` spatial-queries. Entity rendering does
neither. Storage is `HopSlotMap` (`world.rs:209-212`).

## TESTABILITY — zero on the sim→frame path

`headless/` is **NOT** a second render adapter: `headless/Cargo.toml` depends only on
simulation/networking/common/structopt/log — **no `engine` dependency**. It is a network server.

All 26 `#[test]` in `engine/` are in `geometry/earcut.rs` plus one size assert in `gfx.rs:205`.
`native_app/` has zero tests. (Consistent with [[seam-perimeter-native-app]].)

## DEEP SEAMS — preserve, do not refactor away

- `MapSubscriber` chunk invalidation: 4 independent adapters (map_mesh road + building subs,
  terrain, trees, lamps). Genuinely deep.
- `ImmediateDraw` (`native_app/src/rendering/immediate.rs`): 8 order kinds, retained + persistent
  lists, builder-with-Drop. Reached by 13 files.

## Lower priority

wgpu leak is narrow in practice: `engine/src/lib.rs` has 22 glob re-exports plus `pub use wgpu`,
but only **7** `wgpu::` sites exist in `native_app/src`, across 2 files.

`assets/shaders/tonemap.wgsl` is a Reinhard-style rational approximation of `1-exp(-x)`, not a
filmic curve — art-direction.md was right to doubt the old filmic claim.

Related: [[seam-simwide-structure-2026-08-27]], [[false-claims-failure-inventory]].
