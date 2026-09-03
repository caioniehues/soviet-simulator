# Render boundary

**Kind:** architecture
**Authority:** advisory
**Status:** draft
**Owner:** architecture
**Last verified:** 2026-08-28

## Current substrate

`engine/` is a forward wgpu renderer (wgpu 0.20.1 / winit 0.29 — several breaking eras behind
upstream; PBR IBL, cascaded shadows, depth prepass, SSAO, fog, MSAA, GPU instancing, GPU timing,
frame capture). `native_app/src/rendering/` reads simulation state directly to build draw lists;
it draws every human and vehicle. Both `yakui` and `egui` sit on the same renderer. The technical-
stack research (2026-08-24) records the migration risk and the reproducibility problem (unpinned
git deps).

## Target design

- **CPU decides, GPU draws.** Authoritative economy and social decisions stay on the CPU. The GPU
  is for culling and LOD, citizen and vehicle interpolation, crowds, heatmaps, smoke, stockpiles,
  lights, snow — visual state only.
- **`RenderSnapshot`** is the only thing the renderer reads: immutable, published per tick via
  `ArcSwap` ([snapshots](snapshots.md)); positions and instance data as validated POD
  (`bytemuck::Pod`/`Zeroable`, already a dependency). Never make an authoritative struct `Pod` for
  convenience.
- **Scale.** Spatial render cells or chunks; frustum and distance culling; citizen and vehicle
  LOD; dirty or partial instance-buffer updates. The charter's "bounded visible citizens" is the
  product rule that makes 250k identities compatible with a renderer that draws a few thousand
  bodies. L4 of the citizen materialisation levels ([entity identity](entity-identity.md)).
- **Proof stays visual.** Per `CLAUDE.md`, renderer and UI work is proven by an inspected frame or
  a short video, never by compilation; the [MCP test-harness proposal](../plan/proposals/mcp-test-harness.md)
  records what can and cannot be automated.

## Migration

1. `RenderSnapshot` for humans and vehicles only; renderer reads it behind a flag.
2. Culling and LOD on the snapshot.
3. Remove direct `Simulation` reads from `rendering/`.
4. The wgpu/winit/UI upgrade is a separate, staged programme (technical-stack research §risks).

## Related

- [Snapshots](snapshots.md)
- [Performance](performance.md)
- [Art direction](../reference/art-direction.md)
- [Technical stack research](../explanation/research/technical-stack-upstream-2026-08-24.md)
