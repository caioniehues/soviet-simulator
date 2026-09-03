# Art direction — First Light

**Kind:** reference
**Authority:** reference
**Status:** active
**Owner:** project lead
**Last verified:** 2026-08-24

## Intent

First Light targets W&R-adjacent industrial realism through original procedural geometry and CC0
materials: weathered concrete, rusted steel, soot-stained brick, oiled timber, worn ground, and
signal colours that read at RTS zoom. This is presentation direction, not evidence that the current
renderer already produces every listed effect. No extracted W&R asset is permitted without a
separate recorded permission receipt.

## Palette reference

| Role | sRGB | Intended use |
|---|---|---|
| Field ground | `#6b7050` | desaturated olive, never lawn green |
| Worn earth/yards | `#7a6a52` | building yards and disturbed ground |
| Dirt road | `#8a7355` | dry mud, lighter than yards |
| Concrete | `#9a968c` | walls and plants |
| Soot brick | `#6e4a3c` | mine and factory bases |
| Rusted steel | `#8a5a3a` | headframes, roofs, and trucks |
| Asphalt | `#3a3a3c` | paved roads |
| Signal attention/fail | `#d98f2b` / `#a12a1c` | warning and refusal states |

Additional intended roles are coal `#1f1f22`, gravel `#8f8a80`, glass `#7a8894`, cloth
`#4a4640`, machinery ochre `#8a6a2e`, cab green `#3f4f44`, and smoke `#b8b8bc`.

## Current renderer and asset evidence

The old Bevy-only claim that `src/game/palette.rs` centrally governs material colour is false:
that path does not exist in this Rust/Egregoria fork. Current observed seams are deliberately
narrower:

- [`native_app/src/rendering/map_rendering/terrain.rs`](../../native_app/src/rendering/map_rendering/terrain.rs)
  draws the simulation heightmap through `HeightmapRender`; it is the terrain presentation entry
  point, not a global palette authority.
- [`engine/src/passes/fog.rs`](../../engine/src/passes/fog.rs) renders a fog pass using
  [`assets/shaders/fog.wgsl`](../../assets/shaders/fog.wgsl), which derives atmospheric colour from
  [`assets/shaders/atmosphere.wgsl`](../../assets/shaders/atmosphere.wgsl).
- [`assets/shaders/tonemap.wgsl`](../../assets/shaders/tonemap.wgsl) contains the active
  tonemap function. The present code does not prove the previous document's filmic curve,
  35-degree sun, or 1.2 km fog target.
- [`tools/bake_ground.py`](../../tools/bake_ground.py) is an observed offline asset pipeline: it
  bakes `field`, `dirt`, and `road_dirt` textures toward the three ground roles above and keeps
  originals as `*_src.png`. It does not enforce colours used elsewhere in runtime rendering.

No single current module enforces this complete palette. Treat any additional central material
authority as a future implementation decision, not an existing contract.

## Presentation constraints for future work

- At RTS zoom, silhouette and value separation take priority over small texture detail.
- Avoid unweathered pure-white/pure-black building surfaces except where an inspected renderer
  path explicitly requires them.
- Prefer visible physical state over decorative abstraction: loaded vehicles, operating machinery,
  queues, shortages, and outages must read from authoritative simulation state.
- Any renderer or asset-pipeline change must cite its actual source seam and provide inspected
  visual proof; this reference alone does not prove a frame matches the target.

## Asset provenance

| Asset class | Source | Licence | Receipt |
|---|---|---|---|
| Ground and road textures under `assets/textures/` | ambientCG | CC0 1.0 | ambientCG licence page; source copies retained beside bakes |
| Bundled UI font, if any | Google Fonts | OFL 1.1 | Font `OFL.txt` in repository |
| W&R mod assets | none imported | permission required | none |

## Related documents

- [Documentation authority](../meta/document-authority.md) is the documentation authority map.
- [`../plan/charter-1.0.md`](../plan/charter-1.0.md) sets product scope; it does not make a
  renderer mechanism current.
- [`../archive/bevy-track/`](../archive/bevy-track/) retains the discarded track's visual claims
  as history only.
