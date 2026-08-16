# Soviet Simulator (Bevy track)

A city/economy sim where every constraint is physical: coal is hauled by truck,
power exists only where a wire runs, and a saturated road is a plan problem you
build your way out of. Specs in `spec/`, architecture in `architecture/` and
`docs/adr/`, milestone ladder in `ROADMAP.md`.

## Status — M1 "The First Chain" complete (#8)

The one rule proven end to end: coal physically hauled by truck powers a plant
that powers a factory; cut the road and the blackout cascades; rebuild and it
recovers.

Built (tickets #9–#15):

- **Sim core** — `SimTick` schedule, nine stage sets, speed-substep clock
  (pause/1×/2×/4×), named command barriers (ADRs 0001–0002).
- **World & camera** — flat plane, 32 m grid, RTS rig (pan/rotate/zoom).
- **Roads** — unified lane graph (node/segment/per-lane buffer), click-chain
  tool, dirt free / **paved consumes delivered gravel** from yards within 80 m
  (rejected with a HUD shortfall readout otherwise), X cuts, R rebuilds the
  last cut, 1-ring incremental recompile.
- **Buildings & resources** — mine/quarry/plant/factory, typed shared-capacity
  yard inventories (coal/gravel/goods), stage-1 recipes.
- **Vehicles** — truck asset + `ActiveVehicle` pawn (first-class relationship),
  lane-following shuttle loop, BFS routing, dock radius (a building cut off
  from any road node really is severed), severed-route re-route on rebuild.
- **Electricity** — single wire class, pole/building snapping, connectivity +
  capacity solve, fuel-gated plant, factory power lamp, blackout cascade.
- **Legibility** — HUD (active tool, key legend, sim speed), inspect panel,
  selection ring, yard/truck fill bars, time controls.

Benchmark gate (`cargo run --release --bin bench_chain`): 100 concurrent
chains headless — **mean 0.034 ms/tick** against the ≤0.33 ms gate (Unity M1
reference 0.34 ms).

Acceptance video: `cargo run --release --bin capture -- frames screenshots/result/0 480`
then ffmpeg (see `bevy.md`); latest render at `screenshots/result/0/video.mp4`
(local, `screenshots/` is untracked).

## Run

```
cargo run            # the game
cargo test           # 24 sim tests
```

Keys: `1` dirt road · `2` paved road · `3` building (cycles kind) · `4` wire ·
`5` shuttle · `Esc` inspect · `X` cut · `R` rebuild last cut · `Space` pause ·
`[` `]` speed · WASD pan · Q/E rotate · wheel zoom.

## What's next

P1 "First Light" polish pass on the M1 scene (#16, zero-spend), then B2 staffing →
B3 dispatcher → … (see `ROADMAP.md`, including the new parallel P-ladder).

## Assets

| Asset | Source | Status |
|---|---|---|
| Buildings, trucks, roads, wires | Procedural primitives (M1 charter Q12) | in use |
| Generated art pass | `/asset-gen` (paid) | deferred, post-demo decision |
