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
| Signal fail | `#a12a1c` | blackout, HUD warnings |
| Sky/haze | `#c7d0d8` → `#8fa3b5` | fog + clear color gradient |

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
