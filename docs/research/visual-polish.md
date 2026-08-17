# Research: zero-spend visual polish techniques (issue #85)

**Question.** What makes zero-spend procedural games look *finished* instead of
like programmer art? Ranked, concrete, applicable to our stack: Bevy 0.19.1
native, procedural meshes + CC0 textures, zero art spend.

**Context.** Playtest verdict on the current build: it "REALLY REALLY REALLY
LACKS POLISHMENT AND LOOKS LIKE SOMETHING DONE BY A CHILD." This document is
the direct answer, feeding "Art completion: the visual bar" (#81).

---

## 1. Where we actually stand (read from the code, not the docs)

The foundations are better than the verdict suggests — P1 "First Light"
(`docs/art-direction.md`) already bought the cheap half of the look:

- **Camera** (`src/game/camera.rs`): `Tonemapping::TonyMcMapface`,
  `Bloom::NATURAL`, linear `DistanceFog` (500→1600 m) matching the clear
  color. That is a real color script, not defaults.
- **World** (`src/game/world.rs`): warm low sun (11k lux, ~35°) with shadow
  maps, cool ambient fill, tiled CC0 field texture + normal map, olive tint,
  faint 32 m grid gizmo.
- **Buildings** (`src/game/buildings.rs`): 14 kinds as multi-part
  cuboid/cylinder silhouettes with per-kind muted colors, yard pads, smoke
  puffs on powered plants.
- **HUD** (`src/game/hud.rs`, `src/game/toolbar.rs`): dark translucent panels,
  rust-accent borders (`#a15933`-ish), FiraSans, ASCII progress bars; the Plan
  ledger is a real signature screen.

What is **missing** is exactly the layer that separates "correct" from
"finished": nothing is grounded (no AO of any kind — parts float on the
plane), edges are raw (no AA beyond MSAA default, no edge wear), the ground is
one texture to the horizon, the UI draws data with text glyphs instead of
drawn elements, and nothing reacts (no placement feedback, no hover
highlight, no motion except trucks and smoke).

**Unused ammunition already in `Cargo.toml`:** `bevy_hanabi` (GPU particles),
`bevy_mod_outline` (mesh outlines), `bevy_procedural_tree`,
`bevy_vector_shapes` (immediate-mode 2D shapes), `noiz` (noise). None is
referenced anywhere in `src/`. Several top-ranked items below are "wire in a
dependency we already ship."

## 2. What the comparable zero/low-asset builders actually do

- **Townscaper** (Oskar Stålberg): almost no textures. The look is carried by
  (a) a tight pastel palette, (b) procedural geometry that rounds and varies
  every block, and (c) **shading baked into the mesh** — vertex-colored
  ambient occlusion and gradients; Stålberg has said exported models "lack the
  appearance they have in Townscaper without the game's shaders"
  ([Steam discussion](https://steamcommunity.com/app/1291340/discussions/0/3039354735039851640),
  [80.lv on Townscaper-likes](https://80.lv/articles/island-architect-townscaper-inspired-city-builder-on-custom-engine)).
  The single transferable core: **darkness where geometry meets geometry**.
- **Islanders** (Grizzly Games): flat-shaded low poly that reads finished via
  one disciplined gradient palette per island, heavy fog/haze that dissolves
  the world edge, soft shadows, and photo-mode grading; the sequel added
  day/night with building lights and weather as its visual upgrades
  ([Boss Rush review](https://bossrush.net/2025/08/31/game-review-islanders-new-shores-a-compelling-sequel-to-the-hit-casual-city-builder/)).
- **Mini Motorways** (Dinosaur Polo Club): polish through subtraction — "no
  clutter and no unnecessary UI," one distinct palette per map, color-coded
  meaning, and constant smooth motion; the art director's job was palettes and
  UI, not assets
  ([Game Developer interview](https://www.gamedeveloper.com/audio/-i-mini-motorways-i-and-the-delicate-art-of-marrying-complexity-and-minimalism),
  [Blake Wood portfolio](https://blakemwood.com/projects/4bxqw2)).
- **Workers & Resources** (our reference): mass through repetition (identical
  windows, panel seams) and grime gradients — dark at ground, streaked below
  roofs; pale low-contrast skies so buildings own the value range (already
  noted in `docs/art-direction.md`).

Common thread: **none of these buys detail. They buy occlusion, palette
discipline, motion, and UI restraint** — all reproducible procedurally.

## 3. What Bevy 0.19 ships built-in (verified against docs.rs / release notes)

- **SSAO**: `ScreenSpaceAmbientOcclusion` camera component +
  `ScreenSpaceAmbientOcclusionPlugin`; darkens creases and gives "contact
  shadows where objects meet, giving entities a more 'grounded' feel."
  Requires `Msaa::Off`; strongly recommends pairing with
  `TemporalAntiAliasing`; not supported on WebGL2 (we are native — fine)
  ([docs.rs](https://docs.rs/bevy/latest/bevy/pbr/struct.ScreenSpaceAmbientOcclusion.html)).
- **Contact shadows** (new in 0.19): screen-space raycast shadows filling the
  gap shadow maps miss, toggled per light via `contact_shadows_enabled`
  ([Bevy 0.19 notes](https://bevy.org/news/bevy-0-19/)).
- **Vignette** (new in 0.19): `Vignette` component — intensity, radius,
  smoothness ([Bevy 0.19 notes](https://bevy.org/news/bevy-0-19/)).
- **Lens distortion** (new in 0.19): `LensDistortion` component (skip — wrong
  genre).
- Already in use: tonemapping (TonyMcMapface is the right neutral-filmic
  pick), `Bloom`, `DistanceFog`. Also available: `Fxaa`, SMAA, TAA,
  `EnvironmentMapLight`, `Skybox`, screen-space reflections, depth of field,
  motion blur, auto-exposure.

---

## 4. The ranked list

Ranked by **visual lift per unit effort**, weighted toward what the playtest
verdict actually saw. Effort: S = an hour or two, M = a day-ish, L = multiple
days.

### 1. Grounding pass: SSAO + contact shadows + TAA — lift: very high, effort: S
The single biggest programmer-art tell is objects that look *placed on* the
world instead of *in* it. One camera-component change: add
`ScreenSpaceAmbientOcclusion` (+ plugin), `TemporalAntiAliasing`, `Msaa::Off`
to `spawn_camera` in `camera.rs`, and `contact_shadows_enabled` on the sun.
Every cuboid seam, yard edge, and building foot darkens where it meets its
neighbor — the Townscaper trick, free from the renderer. TAA also kills the
jagged edges on our thin parts (headframe braces, wires, poles), a second
strong "unfinished" tell. This is the highest lift-per-line change available
to us.

### 2. UI redraw: from text panels to a drawn state-document system — lift: very high, effort: M
The HUD is the surface the player stares at 100% of the time, and ours says
"debug overlay": ASCII `[####----]` bars, uniform 15 px text walls, panels
that are just rounded rects. Keep the identity (dark sheet, rust accent,
state-document tone) but *draw* it: real progress-bar `Node`s with fill +
border (quota bars, staffing, cargo), a thin double-rule border on the ledger
like a printed form, letter-spaced ALL-CAPS FiraSans-Bold section headers at a
smaller size, consistent 8 px spacing grid, small drawn glyphs (via
`bevy_vector_shapes` or plain colored `Node`s) for resources instead of words
where a row repeats. Mini Motorways' lesson is subtraction: fewer words,
color-coded meaning, nothing that isn't information. This is where "done by a
child" is most cheaply reversed because UI polish is pure layout discipline,
zero assets.

### 3. Placement & interaction juice — lift: high, effort: M
Nothing currently acknowledges the player. Three feedback beats, all
zero-asset: **hover/selection outline** on buildings via `bevy_mod_outline`
(already a dependency); **placement thump** — spawn the building at ~85%
scale and ease to 100% over ~150 ms while a `bevy_hanabi` dust puff kicks at
the foundation (hanabi already a dependency); **valid/invalid ghost preview**
— tint the footprint gizmo green/red and show the actual building mesh as a
translucent ghost under the cursor instead of a wireframe rectangle. Motion
on interaction is the "alive vs dead" line every comparable title crosses.

### 4. Ground dressing: break the infinite lawn — lift: high, effort: M–L
One tiled texture to the horizon reads as a test scene no matter how good the
texture is. Cheapest wins first: (a) **scatter** — instance a few thousand
grass tufts/rocks/bushes as tiny procedural meshes with `noiz`-driven
placement density (thin out near roads/yards), (b) **trees** —
`bevy_procedural_tree` is already a dependency; clustered copses sell scale
and give the fog something to silhouette, (c) **macro variation** — a second
detail layer or vertex-tint on the ground plane using `noiz` so distant
ground shifts between olive/earth patches instead of visible tiling, (d)
**road shoulders** — a gravel fringe ribbon along each road segment so
asphalt doesn't knife-edge into grass (W&R's "edge raggedness" observation,
already in art-direction.md).

### 5. Geometry weathering: bake the grime the art doc promised — lift: medium-high, effort: M
`docs/art-direction.md` prescribes "darker base band on every wall,
rust-toned roof edges" — not yet in `buildings.rs`. Do it as geometry (the
repo's own stated rule): a 0.4 m darker plinth box around every building
base, thin dark fascia strips at roof lines, and **window strips** — flat
near-black quads in rows on dwelling/factory walls. Repetition of identical
windows is exactly how W&R sells industrial mass with no texture detail.
Optional stretch: darken vertex colors on lower part faces for fake baked AO
where SSAO can't reach under the camera angle.

### 6. Sky and horizon: kill the flat clear color — lift: medium, effort: S–M
A solid `ClearColor` is a void. Either a procedural gradient skybox (one
generated 6-face image, S) or Bevy's atmosphere/`Skybox` +
`EnvironmentMapLight` so the pale W&R sky has depth and buildings pick up
subtle sky fill. Keep it low-contrast per the art doc — the buildings own the
value range. Pair with a gentle `Vignette` (new 0.19 built-in, minutes to
add) to focus the frame the way Islanders' photo grading does.

### 7. Palette enforcement: one material factory — lift: medium, effort: S
The rules exist (albedo clamps, roughness ≥ 0.75, saturation only on
signals) but each file mixes its own colors. Centralize: a
`fn material(palette_role) -> StandardMaterial` in one module; every
presentation file requests roles, not RGB. Cheap now, and it is what keeps
every future building/vehicle on-palette — palette *discipline*, not palette
choice, is what Mini Motorways and Islanders are actually demonstrating.

### 8. Ambient life: motion in the idle frame — lift: medium, effort: M
A finished frame moves even when the player doesn't: smoke already exists —
add wind-drifted lean; flags on the customs office and ledger-relevant
buildings (3-quad cloth wave in a vertex shader or simple transform wobble);
birds as 3-triangle flocks on a spline; citizens already walk. Islanders'
sequel spent its visual budget exactly here (lights, weather) because idle
motion is what screenshots can't show but players feel.

### 9. Night lighting / powered-state glow — lift: medium, effort: M–L
Emissive window quads and yard lamps when `Powered`, dark when blacked out —
makes the power system *visible* and gives bloom something to do. Defer until
windows (item 5) exist. Full day/night cycle is L and optional; a fixed
"morning" with emissive accents is most of the lift.

### 10. Shadow quality tuning — lift: low-medium, effort: S
Tune `CascadeShadowConfig` for RTS distances (tighter far bound ≈ 600 m,
2–3 cascades) and soft-shadow settings so long morning shadows stay crisp
near and don't shimmer far. Do together with item 1; listed separately so it
isn't skipped — peter-panning/acne at RTS zoom is a subtle but constant
cheapness signal.

---

## 5. Suggested batching

- **Weekend batch (S items, transforms the game):** 1 + 6(vignette) + 7 + 10 —
  one camera/lighting PR, one material-factory PR.
- **The verdict batch:** 2 + 3 — UI redraw and interaction juice; this is the
  pair that answers "done by a child" head-on.
- **The world batch:** 4 + 5, then 8/9 as garnish.

## Sources

- Bevy SSAO docs: https://docs.rs/bevy/latest/bevy/pbr/struct.ScreenSpaceAmbientOcclusion.html
- Bevy 0.19 release notes (contact shadows, Vignette, LensDistortion): https://bevy.org/news/bevy-0-19/
- Townscaper shading-in-shaders (Stålberg on OBJ export): https://steamcommunity.com/app/1291340/discussions/0/3039354735039851640
- Townscaper-like builder breakdown: https://80.lv/articles/island-architect-townscaper-inspired-city-builder-on-custom-engine
- Mini Motorways minimalism interview: https://www.gamedeveloper.com/audio/-i-mini-motorways-i-and-the-delicate-art-of-marrying-complexity-and-minimalism
- Mini Motorways art (Blake Wood): https://blakemwood.com/projects/4bxqw2
- Islanders: New Shores visual additions: https://bossrush.net/2025/08/31/game-review-islanders-new-shores-a-compelling-sequel-to-the-hit-casual-city-builder/
- In-repo grounding: `docs/art-direction.md`, `src/game/camera.rs`, `src/game/world.rs`, `src/game/buildings.rs`, `src/game/hud.rs`, `src/game/toolbar.rs`, `Cargo.toml`
