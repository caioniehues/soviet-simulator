# Art direction — "First Light" (P1, #16)

**Target:** W&R-adjacent industrial realism, achieved with our own procedural
geometry + CC0 materials. Gritty, weathered, materially honest. Studied from
reference; no extracted assets — every non-procedural asset has a row in the
provenance table below.

## The look in one paragraph

An overcast-but-bright continental morning. Sun low and warm, long soft
shadows, cool ambient fill. Everything man-made is concrete, rusted steel,
soot-stained brick and oiled timber; nothing is saturated except signal
colors (power lamps, HUD warnings). The ground is worn field — patchy grass,
mud and gravel around every yard. Silhouettes do the talking at RTS zoom:
a mine is a headframe, a plant is its chimneys, a factory is a sawtooth roof.

## Palette

| Role | Color | Notes |
|---|---|---|
| Field ground | `#6b7050` desaturated olive | never lawn-green |
| Worn earth/yards | `#7a6a52` | blends around buildings/roads |
| Concrete | `#9a968c` warm gray | walls, plants |
| Soot brick | `#6e4a3c` | mine, factory bases |
| Rusted steel | `#8a5a3a` | headframes, roofs, trucks |
| Timber | `#5f4a34` | poles, props |
| Asphalt | `#3a3a3c` | paved roads |
| Dirt road | `#8a7355` | dry mud, lighter than earth |
| Signal ok | `#ffd34d` | power lamp lit |
| Signal attention | `#d98f2b` | the middle severity (R0.4): needs a look, not a stop |
| Signal fail | `#a12a1c` | blackout, HUD warnings, refusals |
| Sky/haze | `#c7d0d8` (horizon) → `#8fa3b5` (zenith) | sky dome gradient; fog colour is the horizon stop |

Extended rows, added when the factory landed (R0.2) because the table above
never named materials the game already draws:

| Role | Color | Notes |
|---|---|---|
| Coal | `#1f1f22` | piles, tyres, tracks — the floor of the albedo clamp |
| Gravel | `#8f8a80` | quarry heaps, aggregate loads |
| Glass | `#7a8894` | cab windows; the one role allowed past the roughness floor |
| Cloth | `#4a4640` | citizen coats — drab, never a costume colour |
| Machine ochre | `#8a6a2e` | construction plant, so equipment reads apart from buildings |
| Cab green | `#3f4f44` | vehicle cabs; drab industrial, not the old saturated teal |
| Smoke | `#b8b8bc` | chimney puffs (unlit, alpha-graded) |

## Enforcement (R0.2, [#112](https://github.com/caioniehues/soviet-simulator/issues/112))

This table is **normative, not aspirational** — it had already drifted once, to
the lawn-green ground it explicitly forbids. Two mechanisms hold it now:

- `src/game/palette.rs` is the **only** sanctioned way to build a
  `StandardMaterial`. Every colour in `src/game/` is a `Role` from this table,
  and `Mat::build` applies the material rules below on the way out. Adding a
  colour means adding a row here first.
- Ground textures are **baked on-palette offline** by `tools/bake_ground.py`,
  which rescales each CC0 photo's linear mean onto its role colour and halves
  its chroma. This is not a stylistic preference: correcting ambientCG's grass
  to `#6b7050` needs linear multipliers of (2.5, 1.5, 3.6), and a material
  tint clamps at 1.0. Sources are kept beside the bakes as `*_src.png`.

## Material rules

- Perceptual roughness ≥ 0.75 everywhere except glass/lamps. Metal only on
  explicitly metallic parts (`metallic ≤ 0.6`, still rough).
- No pure white, no pure black: clamp albedo to `[0.05, 0.85]`.
- Weathering by geometry, not decals (M1): darker base band on every wall,
  rust-toned roof edges.
- One directional sun (warm, ~35° elevation), cool ambient, filmic
  tonemapping. Distance fog closes the world at ~1.2 km.

## Silhouette language (per building kind)

- **Mine** — low soot-brick shed + steel headframe tower with angled braces
  + coal pile in the yard.
- **Quarry** — open pit walls, gravel heaps, low timber office.
- **Power plant** — big concrete hall + two tall chimneys (smoke while
  `PowerOutput > 0`).
- **Factory** — long hall with sawtooth roof + one chimney; lamp on a mast.
- **Truck** — cab + flatbed with sideboards; visible cargo mound when loaded;
  wheels that spin.

## Asset provenance table

Every non-procedural asset ships only with a row here.

| Asset | Source | License | Receipt |
|---|---|---|---|
| Ground/road textures (as downloaded into `assets/textures/`) | ambientCG | CC0 1.0 | ambientcg.com license page |
| UI font (if bundled) | Google Fonts | OFL 1.1 | font's OFL.txt in repo |
| W&R mod assets | — | pending per-mod permission receipts | none yet — nothing imported |

## Reference study notes (no assets, observations only)

- W&R reads industrial mass through *repetition* (identical windows, panel
  seams) and *grime gradients* (dark at ground, streaked below roof lines).
- Its roads sell realism via edge raggedness against terrain, not texture
  detail.
- Its skies are pale and low-contrast; buildings carry the value range.
