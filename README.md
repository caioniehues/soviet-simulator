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

## Status — M2 "Hands" in progress (#22, M2.1–M2.6 done)

Nothing runs unstaffed anymore:

- **Citizens & households** (#23) — flat Citizen/Household tables, dwelling
  kind (khrushchyovka), spawn queue.
- **Housing** (#24) — plan-recruited immigration (+/- lever), FIFO housing
  office, population HUD panel.
- **Labour planning** (#25) — tiered vacancies, Dijkstra commute-feasibility
  over the lane graph, fixed tenure surviving road cuts.
- **Commutes** (#26) — transient commuter pawns walk the lane graph on a
  jittered day cycle; arrival tallies per-building presence.
- **Staffing gates production** (#27) — CS1 curve
  `f = 2e − 200e/(staff%+100)` with health-based efficiency `e` multiplies
  mine/quarry extraction, plant burn/output, and factory rate (Liebig with
  the power gate); zero presence stops the building; inspect panel shows
  present/assigned/needed.
- **Needs & attendance** (#28) — per-citizen food/rest satisfactions with
  low-frequency decay; meals drain the household pantry (fiat refill until B3
  shops); wellbeing recomputed from needs feeds a sticky per-day attendance
  roll (CS1's `GetWorkProbability` coupling) — exhausted citizens skip
  workdays and staffing falls.
- **Save/load first cut** (#28) — custom serde-column format (postcard):
  tables sorted by stable u64 id, entity refs as indices, derived road
  geometry recompiled on load; `F5` quicksave / `F9` quickload
  (`saves/quicksave.sav`); round-trip preserves the sim state hash.

Left in M2: acceptance & benchmark gate (#29).

## Run

```
cargo run            # the game
cargo test           # 52 sim tests
```

Keys: `1` dirt road · `2` paved road · `3` building (cycles kind) · `4` wire ·
`5` shuttle · `Esc` inspect · `X` cut · `R` rebuild last cut · `Space` pause ·
`[` `]` speed · `F5` quicksave · `F9` quickload · WASD pan · Q/E rotate ·
wheel zoom.

## What's next

P1 "First Light" done (#16, zero-spend). B2 staffing underway (above).
Next: M2.7 acceptance gate → B3 dispatcher (see `ROADMAP.md`, including the
parallel P-ladder).

## Assets

| Asset | Source | Status |
|---|---|---|
| Buildings, trucks, poles, smoke | Multi-part procedural meshes (P1) | in use |
| Ground/road textures | ambientCG (CC0): Grass001, Ground048, Asphalt010, Gravel022 | in use |
| HUD font | Fira Sans (OFL 1.1, license bundled) | in use |
| Generated art pass | `/asset-gen` (paid) | deferred, post-demo decision |
