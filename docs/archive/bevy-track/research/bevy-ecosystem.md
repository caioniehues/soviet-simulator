# Bevy Ecosystem Research — Large-Scale Socialist Planned-Economy City Builder

**Compiled:** 2026-08-16
**Target game:** 250k–1M persistent citizen agents · spline roads → lane graphs · hierarchical pathfinding · vehicle traffic · physical resource logistics · utility flow networks · phased construction · deep inspect-any-citizen UI · 3D RTS-camera presentation · deterministic-ish sim · save/load of huge worlds.

## Current Bevy version (Aug 2026)

**Bevy 0.19 (0.19.1 patch)** is the current stable, released **19 June 2026** (261 contributors, 1,185 PRs). It is unusually well-aligned with this project because of what shipped:

- **GPU-driven rendering**: automatic multi-draw-indirect batching, GPU batch-set unpacking, batched depth-only prepasses, GPU light clustering. A mobile RTX 4090 rendered **1.6 million cube entities** at 53 FPS with culling on (up from 21 FPS in 0.18).
- **GPU-side culling & LOD**: `VisibilityRange` LOD selection and frustum/occlusion culling moved to the GPU; `NoCpuCulling` opt-out per entity.
- **SIMD-friendly ECS**: contiguous query access (~3× faster bulk ops with AVX2); resources are now components on singleton entities.
- **Render graph = ECS schedules**: render passes are ordinary systems — sim and render share one data model, zero marshalling.
- **Next-gen scenes / BSN** (Bevy Scene Notation) + asset-handle round-tripping in serialization.

Bevy minor versions break plugins, so per-crate version tracking below is load-bearing. **Bottom line up front: this project should be built in pure Bevy 0.19, leaning heavily on native engine features and a small set of engine-agnostic Rust crates, with a large amount of bespoke simulation code.**

---

## TL;DR — Adopt / Evaluate / Avoid

| Verdict | Crate / Feature | Category | Bevy ver | Why |
|---|---|---|---|---|
| **ADOPT** | Native Bevy 0.19 GPU-driven batching + `VisibilityRange` + GPU culling | Rendering many instances / LOD | 0.19 | Draws ~1.6M instances natively; no third-party instancing crate needed |
| **ADOPT** | `bevy_egui` (0.19) | UI / debug HUD | 0.19 | Mature immediate-mode UI; backbone for dev tooling and complex panels |
| **ADOPT** | `bevy-inspector-egui` 0.37 | Inspector | 0.19 | World/entity inspector; near-free "inspect any entity" during dev |
| **ADOPT** | Native `bevy_picking` + native transform/text gizmos | Picking / gizmos | 0.19 | Shipped in-engine; no `bevy_mod_picking` (archived) |
| **ADOPT** | `bevy_panorbit_camera` (0.19) | RTS camera | 0.19 | On 0.19, ortho+perspective, smoothed pan/orbit/zoom |
| **ADOPT** | `petgraph` + `pathfinding` crate | Lane graph + A* | agnostic | Engine-agnostic backbone for the road/lane graph and per-level A* |
| **ADOPT** | `kiddo` (kNN) + hand-rolled hash grid (broad-phase) | Spatial index | agnostic | Fastest kd-tree + trivial uniform grid for 1M moving agents |
| **ADOPT** | `bevy_rand` (0.19) | Determinism / RNG | 0.19 | ECS-scoped seedable RNG for reproducible sim |
| **ADOPT** | Native `FixedUpdate` + `AsyncComputeTaskPool` | Determinism / async | 0.19 | Fixed-timestep sim + background heavy compute, both in-engine |
| **ADOPT** | Custom serde (`bincode 2.0` / `postcard`) + `zstd` | Big-world save/load | agnostic | Only viable path to save 1M entities fast & compact |
| **ADOPT** | `bevy_kira_audio` **or** native `bevy_seedling` (0.19) | Audio | 0.19 | Both on 0.19; Kira mature, Seedling modern |
| **ADOPT** | `bevy_asset_loader` + `bevy_embedded_assets` (0.19) | Asset pipeline | 0.19 | Loading states + shipping embedded assets |
| **EVALUATE** | `bevy_lookup_curve` (0.19) | Curves / tuning | 0.19 | Editable response curves for sim tuning (not road geometry) |
| **EVALUATE** | `bevy_northstar` 0.7 (0.19) | Hierarchical pathfinding | 0.19 | Grid-HPA*; study as reference — doesn't fit lane graphs directly |
| **EVALUATE** | `bevy_rerecast` 0.5 (0.19) | Navmesh | 0.19 | For free-roam pedestrian nav only, not traffic lanes |
| **EVALUATE** | `moonshine_save` (0.19) | Save (sparse entities) | 0.19 | OK for a handful of unique/heterogeneous entities, not the 1M bulk |
| **EVALUATE** | `bevy_editor_cam` (0.18) | CAD/editor camera | 0.18 | If you build an in-house road/scene editor; lags one version |
| **EVALUATE** | `bevy-persistent` / `bevy_simple_prefs` (0.19) | Settings persistence | 0.19 | Config/keybinds only — never the world save |
| **AVOID** | Godot-Bevy split (`godot-bevy`) | Architecture | 0.18 | FFI boundary is the bottleneck at 1M agents; see §10 |
| **AVOID** | `big_space` for a city | Floating origin | 0.18 | f32 near origin covers a city; only needed at planet scale |
| **AVOID** | `bevy_terrain` | Terrain | old/stale | Overkill + stale (Apr 2025) for flat buildable city terrain |
| **AVOID** | `bevy_save` | Save framework | 0.16 | Stuck on Bevy 0.16; reflection-heavy; non-starter for 0.19 + 1M |
| **AVOID** | `bevy_mod_picking` | Picking | archived | Archived; superseded by native `bevy_picking` |
| **AVOID** | `bevy-spatial` | Spatial index | 0.16 | Three versions behind; wrap `kiddo` yourself instead |
| **AVOID** | `oxidized_navigation` | Navmesh | 0.15 | Archived & deprecated → use rerecast |

---

## 1. Camera (RTS/pan-orbit), Picking, Gizmos

| Crate / feature | Repo | What it does | Bevy | Last commit | Stars | Fit |
|---|---|---|---|---|---|---|
| **bevy_panorbit_camera** | Plonq/bevy_panorbit_camera | Smoothed pan/orbit/zoom, ortho **and** perspective, touch | **0.19** ✅ (v0.27) | 2026-06-23 | 295 | Primary RTS camera controller |
| **bevy_editor_cam** | aevyrie/bevy_editor_cam | CAD/editor-style camera, orbit-around-cursor | **0.18** (v0.9.1) | 2026-08-05 | 109 | Only if building an in-house editor; one version behind |
| **bevy_pancam** | johanhelsing/bevy_pancam | 2D orthographic pan/zoom | maintained | 2026-06-19 | 231 | Only for 2D overlays/minimap |
| Native **`bevy_picking`** | (in-engine) | Mesh/sprite/UI picking + pointer events | **0.19** ✅ | — | — | "Click any citizen/building" — use this, not a crate |
| Native **transform gizmo** | (in-engine 0.19) | Translate/rotate/scale handles (`TransformGizmoPlugin`) | **0.19** ✅ | — | — | In-editor object manipulation |
| Native **text gizmos** | (in-engine 0.19) | Zero-setup world-space debug text | **0.19** ✅ | — | — | Debug overlays over agents/nodes |
| transform-gizmo | urholaukkarinen/transform-gizmo | Standalone 3D transform gizmo | **0.18** | 2026-08-08 | 332 | Fallback if native gizmo insufficient; lags one version |
| ~~bevy_mod_picking~~ | aevyrie/bevy_mod_picking | Picking (pre-native) | **ARCHIVED** | 2025-03-04 | 842 | **Avoid** — folded into engine as `bevy_picking` |

**Notes:** Picking and basic gizmos are now first-party — a big win. `bevy_panorbit_camera` is the one camera crate you need and it already tracks 0.19. A city-builder RTS camera (edge-pan, height-based zoom, rotate) is a thin custom layer regardless; panorbit gives you a well-behaved starting point.

## 2. UI: egui, HUD/inspector, UI frameworks

| Crate | Repo | What it does | Bevy | Last commit | Stars | Fit |
|---|---|---|---|---|---|---|
| **bevy_egui** | vladbat00/bevy_egui | Immediate-mode UI integration | **0.19** ✅ | 2026-08-11 | 1,399 | Backbone for complex dev/inspection panels |
| **bevy-inspector-egui** | jakobhellermann/bevy-inspector-egui | World/entity/resource inspector (v0.37) | **0.19** ✅ | 2026-06-20 | 1,628 | "Inspect any citizen" for free during dev |
| **bevy_lunex** | bytestring-net/bevy_lunex | Retained path-based layout engine | **0.18** (v0.5) | 2026-02-24 | 942 | Candidate for polished shipping HUD; lags one version |
| **haalka** | databasedav/haalka | Reactive FRP-signals UI | **0.18** (v0.7) | 2026-02-11 | 194 | Reactive shipping UI; lags one version |
| **bevy_hui** | Lommix/bevy_hui | XML/HTML component UI with hot reload | 0.18-era | 2026-06-20 | 214 | HTML-authored HUD with hot reload |
| **bevy_flair** | eckz/bevy_flair | CSS styling for Bevy UI | **0.19** ✅ | 2026-06-22 | 151 | CSS-style theming of native `bevy_ui` |
| Native **`bevy_ui`** | (in-engine) | Flexbox UI, text input (`EditableText` in 0.19) | **0.19** ✅ | — | — | Shipping HUD base; BSN improves ergonomics |

**Recommendation:** Use **`bevy_egui` + `bevy-inspector-egui`** immediately for all dev tooling and the deep inspector (this alone delivers much of "inspect any citizen" cheaply). For the polished shipping HUD, decide later between native `bevy_ui` (+ `bevy_flair` for CSS theming) and a retained framework (`bevy_lunex`/`haalka`) once those catch up to 0.19. Don't block the sim on shipping-UI polish.

## 3. Pathfinding / Navigation / Graphs

| Crate | Repo | What it does | Bevy | Last commit | Stars | Fit |
|---|---|---|---|---|---|---|
| **petgraph** | petgraph/petgraph | Standard Rust graph lib + Dijkstra/A*/etc (v0.8.3) | agnostic | 2026-08-16 | 3,993 | **Lane-graph storage** + single-query solver |
| **pathfinding** | samueltardieu/pathfinding | A*, Dijkstra, IDA*, BFS, flow (v4.15) via successor closures | agnostic | 2026-08-08 | 1,070 | **A* per hierarchy level** over your lane graph |
| **bevy_northstar** | JtotheThree/bevy_northstar | Grid-based **HPA*** hierarchical pathfinding (v0.7) | **0.19** ✅ | 2026-07-08 | 122 | Study as chunked-HPA* reference; grid ≠ lane graph |
| **bevy_rerecast** | janhohenheim/rerecast | Rust port of Recast navmesh generator (v0.5) | **0.19** ✅ | 2026-08-05 | 161 | Free-roam **pedestrian** nav only, not traffic |
| **vleue_navigator** | vleue/vleue_navigator | Polyanya navmesh pathfinding, dynamic obstacles | **0.18** (v0.15) | 2026-03-30 | 483 | Pedestrian nav; lags one version; wrong shape for lanes |
| bevy_flowfield_tiles_plugin | BlondeBurrito/... | Grid flow-field pathfinding (RTS mass routing) | **0.17** | 2025-11-08 | 68 | Reference for flow-field idea; dense-grid only |
| ~~oxidized_navigation~~ | (archived) | Old Recast port | **0.15** | archived | 195 | **Avoid** — deprecated → rerecast |

**Honest gap (large):** No crate solves *road-network* pathfinding. Navmesh crates model walkable *surfaces*, and northstar/flow-field crates are *grid*-based; a lane network is a sparse **directed graph** with turn restrictions, one-way rules, lane counts, and speed limits. You will custom-build: (1) the spline→lane-graph compiler, (2) HPA*-style hierarchy over the lane graph (districts → inter-district transit edges → refine), using the `pathfinding` crate's A* as the per-level solver and `petgraph` for storage, and (3) flow-field/route-caching adapted to the sparse lane graph for rush-hour mass routing. `bevy_rerecast` or `vleue_navigator` are worth keeping *only* if you also want free-roaming pedestrians over open ground.

## 4. Splines/Curves, Road Meshes, Procedural Meshes, Terrain

| Crate | Repo | What it does | Bevy | Last commit | Stars | Fit |
|---|---|---|---|---|---|---|
| **bevy_lookup_curve** | villor/bevy_lookup_curve | Editable response/lookup curves (v with editor) | **0.19** ✅ | 2026-06-27 | 69 | Sim **tuning** curves (demand, decay) — not road geometry |
| Native **cubic splines** | (in-engine `bevy_math`) | `CubicBezier`/`CubicCardinal`/`CubicBSpline`, arc-length | **0.19** ✅ | — | — | Road centerline authoring math |
| Native mesh API | (in-engine) | Runtime `Mesh` construction, custom vertex attrs | **0.19** ✅ | — | — | Extrude lane ribbons from splines |
| **noisy_bevy** | johanhelsing/noisy_bevy | Simplex/perlin noise (WGSL + Rust) | **0.19** ✅ | 2026-07-01 | 117 | Terrain/texture noise |
| **noiz** | ElliottjPierce/noiz | Fast configurable noise built for Bevy | **0.19** ✅ | 2026-06-20 | 96 | Alt noise lib, Bevy-native |
| **bevy_terrain** | kurtkuehnert/bevy_terrain | GPU chunked-LOD planetary terrain | old/stale | 2025-04-29 | 311 | **Avoid** — overkill + stale for flat city terrain |

**Honest gap (large):** There is **no road/spline authoring crate**. `bevy_math` gives you the spline primitives; everything else — interactive spline road authoring, intersection geometry, lane-ribbon mesh extrusion, and baking the result into the lane graph — is bespoke. This is one of the project's biggest build items alongside traffic. For terrain, skip `bevy_terrain`: a hand-built chunked heightmap mesh (noise via `noisy_bevy`/`noiz`) is simpler, deformable for construction, and won't break on engine upgrades.

## 5. Spatial Indexing / Broad-Phase

| Crate | Repo | What it does | Bevy | Last commit | Stars | Fit |
|---|---|---|---|---|---|---|
| **kiddo** | sdd/kiddo | Fast SIMD kd-tree (v6.0.2), immutable + dynamic | agnostic | 2026-08-14 | 172 | kNN / radius queries over agent positions |
| **rstar** | georust/rstar | R*-tree (v0.13) range/rectangle queries | agnostic | 2026-08-13 | 551 | Static geometry (buildings, zones, road segments) |
| Hand-rolled **hash grid** | (custom) | Uniform spatial hash, O(1) insert/query | — | — | — | **Best broad-phase for 1M moving agents** |
| **bevy-spatial** | laundmo/bevy-spatial | ECS kd-tree nearest-neighbor plugin | **0.16** | 2025-05-03 | 188 | **Avoid** — 3 versions behind; wrap kiddo yourself |
| avian / bevy_rapier | Jondolf/avian, dimforge | Physics with built-in SAP/BVH broad-phase | **0.19** ✅ (avian) | 2026-08-13 | 3,127 | Broad-phase *if* already using physics; else overkill |

**Recommendation:** For 1M agents that move every frame, a **custom uniform hash grid** (cheap per-frame rebuild) is usually the best broad-phase and often beats trees. Use **`kiddo`** where you genuinely need k-nearest-neighbor, **`rstar`** for semi-static geometry queries. `avian` (on 0.19, 3.1k★) is a fine physics engine if you need real collision, but don't pull it in solely for broad-phase.

## 6. Save/Load & Serialization (huge worlds)

| Approach | Repo | Bevy | Last commit | Stars | Fit for 1M entities |
|---|---|---|---|---|---|
| **Custom serde: `bincode 2.0` / `postcard` + `zstd`** | (agnostic) | agnostic | active | 3,062 / 1,490 | **Primary path** — near-memcpy speed, compact |
| **moonshine_save** | Zeenobit/moonshine_save | **0.19** ✅ | 2026-06-21 | 146 | OK for *sparse* unique entities; reflection-heavy → poor at bulk |
| Native scenes (`DynamicWorld`, BSN) | (in-engine) | **0.19** ✅ | — | — | Prefabs/templates only; reflection → not a bulk saver |
| **bevy_save** | hankjordan/bevy_save | **0.16** ❌ | 2026-01-21 | 216 | **Avoid** — stuck 3 versions behind; reflection-heavy |
| bevy-persistent | umut-sahin/bevy-persistent | **0.19** ✅ | 2026-06-22 | 123 | **Settings only** |
| bevy_simple_prefs | rparrett/bevy_simple_prefs | **0.19** ✅ | 2026-06-25 | 13 | **Settings only** |

**Honest gap (medium):** Reflection-based frameworks (`moonshine_save`, `bevy_save`, native `DynamicScene`) serialize per-component-per-entity through boxed `dyn Reflect` — tens of millions of dynamic dispatches at 1M entities; saves go to seconds/minutes and files bloat. **Roll a custom serde serializer:** pull component columns into `Vec<T>` of POD structs (by archetype, in parallel), serialize with **bincode 2.0** (speed/maturity) or **postcard** (smallest, still fast), then **zstd** the blob. Serialize your own stable integer IDs for graph edges (not `Entity` bits) and remap on load; add a `u32` schema-version header with migration shims. Recommended hybrid: custom serde for the 1M-entity bulk; `moonshine_save` acceptable for a handful of unique heterogeneous entities; native BSN/scenes for prefabs; `bevy-persistent` for settings.

## 7. Big-World / Floating-Origin, LOD, Rendering Many Instances

| Crate / feature | Repo | Bevy | Last commit | Stars | Fit |
|---|---|---|---|---|---|
| Native **GPU-driven batching** | (in-engine 0.19) | **0.19** ✅ | — | — | **1.6M instances natively** — no instancing crate needed |
| Native **`VisibilityRange`** LOD | (in-engine 0.19) | **0.19** ✅ | — | — | HLOD tiers (mesh→low-poly→billboard→cull), dithered crossfade, GPU-side |
| Native custom-instancing example | bevy.org example | **0.19** ✅ | — | — | Per-instance buffer path if auto-batching isn't enough |
| Native meshlets (virtual geometry) | (in-engine, experimental) | **0.19** ✅ | — | — | For hero buildings later; not for agent swarms |
| **big_space** | aevyrie/big_space | **0.18** (v0.12) | 2026-07-16 | 394 | **Avoid for a city** — f32 near origin suffices; planet-scale only |
| **bevy_terrain** | kurtkuehnert/bevy_terrain | old/stale | 2025-04-29 | 311 | **Avoid** — see §4 |
| bevy_mod_billboard | kulkalkul/... | old (2024) | 2024-07-10 | 107 | **Stale** — roll far-LOD billboard quad yourself |

**Recommendation (central):** Rendering 1M agents is largely a **solved problem in native 0.19**. Use automatic mesh+material batching + GPU-driven rendering; add a custom per-instance buffer (official example pattern) only if profiling demands it; mark dense agent meshes `NoCpuCulling`. Build LOD as `VisibilityRange` tiers: near = full/skinned mesh, mid = instanced low-poly, far = instanced billboard quad (hand-rolled), very-far = cull. **`big_space` is not warranted** — f32 holds sub-mm precision to ~10 km and cm precision to ~100 km from origin; a city centered near origin fits comfortably. Every third-party crate here lags the engine and would break on upgrades — the native-first path is also the version-safe path.

## 8. Async Compute / Background Tasks

- **Native `AsyncComputeTaskPool`** (in-engine, 0.19): offload heavy computation (pathfinding batches, logistics solves, chunk meshing) to a background thread pool; poll results across ticks via the async-channel / `Task` component pattern (official examples exist). **This is the primary tool** — no crate needed.
- **`bevy_fixed_update_task`** (ThierryBerger, v0.1.2): runs a heavy `FixedUpdate` sim in a background task by extracting ECS data, computing, and syncing when simulated time catches up. Young (v0.1) but directly targets a heavy sim tick — **evaluate** if the sim step exceeds frame budget.
- Determinism caveat: spawning tasks *from within tasks* yields non-deterministic result ordering — spawn directly from the driving system to keep it deterministic.

## 9. Determinism / Fixed Timestep / RNG

- **Native `FixedUpdate`** (0.19): run all simulation on the fixed timestep; timing (`delta`) auto-reflects the fixed step. This is the spine of a deterministic-ish sim — keep sim in `FixedUpdate`, rendering/interpolation in `Update`.
- **`bevy_rand`** (Bluefinger, **0.19** ✅, 2026-08-15, 105★): ECS-optimized, seedable RNG integrating the `rand` ecosystem. Use per-entity or per-system seeded RNG components for reproducible outcomes. **Adopt.**
- **`bevy_transform_interpolation`** (Jondolf): smooths rendering between fixed-timestep sim states — pairs with the FixedUpdate sim to avoid stutter. Evaluate.
- Determinism realism: true cross-platform lockstep determinism is hard in Bevy (parallel system ordering, float nondeterminism). For a single-player sim, "deterministic-ish" (seeded RNG + fixed timestep + stable iteration order where it matters) is the pragmatic target; full determinism is only mandatory if you later add lockstep multiplayer.

## 10. Godot Interop — `godot-bevy` and `gdext`

| Crate | Repo | Bevy / Godot | Version | Last commit | Stars | State |
|---|---|---|---|---|---|---|
| **godot-bevy** | bytemeadow/godot-bevy | Bevy **0.18** / Godot 4.6 (HEAD api-4-6) | v0.11.0 | 2026-08-04 | 530 | Active, experimental-grade |
| **gdext (godot)** | godot-rust/gdext | Godot 4.x incl. 4.6/4.7 | 0.5.5 | 2026-08-10 | 5,094 | Very active, mature glue |

**How the split would work:** `godot-bevy` runs a full Bevy `App` inside a single Godot GDExtension autoload node. Each visual frame Godot drives the loop; `PreUpdate` reads Godot→ECS transforms, `Update`/`FixedUpdate` runs logic, `Last` writes ECS→Godot. ECS entities map to Godot nodes via marker components and a transform-sync plugin. `gdext` is the mature Rust↔Godot binding it builds on.

**Honest assessment — AVOID the split for THIS project.** Godot's scene-tree API is **main-thread-only**, so presenting 250k–1M agents means marshalling ~1M transforms across the GDExtension boundary onto Godot's main thread **every frame** — the single most expensive thing you'd do, and it exists *only because of* the split. Godot's `MultiMesh` is the only scalable render path there, but it culls as one object (bad for a spread-out city) and gives you no per-agent picking/gizmos (rebuild by hand). `godot-bevy` is unproven at this scale (its own issue #40 openly questions parallel gains; no large-count benchmarks; still on Bevy 0.18), and adds a second toolchain and version-lag risk. Meanwhile **Bevy 0.19 already does the hard part** — GPU-driven rendering of ~1.6M instances with the sim and renderer sharing one ECS data model and zero FFI copy. The Godot split only pays off for *small* entity counts where you want Godot's mature editor/UI for hand-authored scenes — the opposite of this brief. **Recommendation: pure Bevy 0.19; build RTS UI with `bevy_egui`/`bevy_ui` rather than owning a two-engine boundary.**

## 11. Audio, Asset Pipeline, Hot Reload

| Crate | Repo | What it does | Bevy | Last commit | Stars | Fit |
|---|---|---|---|---|---|---|
| **bevy_kira_audio** | NiklasEi/bevy_kira_audio | Kira-backed game audio (mixing, tweening) | **0.19** ✅ | 2026-07-25 | 462 | Mature audio; ambience, SFX, music |
| **bevy_seedling** | CorvusPrudens/bevy_seedling | Firewheel audio-graph integration | **0.19** ✅ | 2026-08-09 | 147 | Modern node-graph audio; evaluate vs Kira |
| **bevy_fmod** | Salzian/bevy_fmod | FMOD integration | maintained | — | — | If you want FMOD authoring tooling |
| **bevy_asset_loader** | NiklasEi/bevy_asset_loader | Loading states, asset collections | **0.19** ✅ | 2026-07-19 | 693 | Structured load screens / asset organization |
| **bevy_embedded_assets** | vleue/bevy_embedded_assets | Embed assets in the binary | **0.19** ✅ | 2026-06-24 | 214 | Single-file shipping builds |
| Native **hot reload** | (in-engine) | Asset hot-reloading (`file_watcher` feature) | **0.19** ✅ | — | — | Iterate on meshes/textures live |
| **bevy_framepace** | aevyrie/bevy_framepace | Frame pacing / limiting | **0.19** ✅ | 2026-07-16 | 375 | Smooth frame delivery |
| **blenvy** | kaosat-dev/Blenvy | Blender→Bevy component workflow | 0.18-era | 2026-03-05 | 842 | Author buildings + components in Blender |
| Physics (**avian**) | Jondolf/avian | ECS physics (2D/3D) | **0.19** ✅ | 2026-08-13 | 3,127 | If vehicles/ragdolls need real collision |

**Recommendation:** **`bevy_kira_audio`** (mature) or **`bevy_seedling`** (modern graph) for audio — both on 0.19. **`bevy_asset_loader`** for load states and **`bevy_embedded_assets`** for shipping. Native asset hot-reload covers iteration. **`blenvy`** is worth evaluating for authoring buildings with attached Bevy components in Blender (lags one version). `avian` if the traffic/physics layer needs genuine collision rather than kinematic movement.

---

## Where you must build your own (the honest gaps)

1. **Spline road authoring → lane graph compiler** — no crate exists. `bevy_math` gives spline primitives; interactive authoring, intersection geometry, lane-ribbon mesh extrusion, and baking to a directed lane graph are all bespoke. *Biggest tooling item.*
2. **Microscopic traffic simulation** — car-following, lane-changing, intersection/signal logic, queueing, congestion feedback. No Rust/Bevy crate provides this.
3. **Hierarchical + flow-field pathfinding over the lane graph** — existing crates are grid/navmesh-based; you'll build HPA* over the sparse directed lane graph (petgraph + pathfinding crate as building blocks) plus route-caching/flow-fields for rush-hour mass routing.
4. **Physical resource logistics** — production chains, stockpiles, vehicle dispatch, throughput — entirely custom domain code.
5. **Utility flow networks** (power/water/heat as flow graphs) — build on `petgraph` with your own max-flow / network-flow solvers; no game-ready crate.
6. **Huge-world save/load** — custom serde (bincode/postcard + zstd) over query-extracted component columns; reflection frameworks don't scale to 1M.
7. **RTS camera & selection UX polish**, **LOD far-tier billboards**, **terrain mesh** — thin custom layers on native features.
