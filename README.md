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

## Status — M2 "Hands" complete (#22)

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

Benchmark gate (`cargo run --release --bin bench_citizens`, #29): 50,000
citizen identities housed, labour-planned, commuting on top of the 100-chain
M1 load — **mean 0.36 ms/tick** (p95 0.49 ms) against the ≤2 ms gate.

Acceptance video (#29): `cargo run --release --bin capture_m2 -- frames
screenshots/result/2 450` then ffmpeg; latest render at
`screenshots/result/2/video.mp4` (local, untracked). Three-day arc: day 1 the
dwelling cluster staffs the chain and the lamp lights; cutting the commute
spur (haul road untouched) idles the whole chain on day 2; rebuilding brings
the workers — and the lamp — back on day 3.

## Status — M3 "The Dispatcher" complete (#30)

Hauling is policy, not hand-wired shuttles: target stock bands per storage, a
shared dispatcher, and a finite depot-owned fleet (tickets #31–#37).

- **Storage policy substrate** (#31) — transport classes on resources
  (coal/gravel Bulk, goods Covered — the hard vehicle-compatibility gate),
  per-resource min/max bands against yard capacity (below min = demand, above
  max = supply), warehouse building kind, recipe-derived default policies.
- **Depots & the finite fleet** (#32) — depot building with six physical
  parking slots (fleet size *is* the slot count); trucks are depot-owned
  assets acquired only by fiat purchase (manufacture is B10); shuttles seize
  an idle class-compatible truck instead of minting one — no free trucks;
  parked trucks render countable on the apron; fleet save columns (save v2).
- **Dispatcher matching** (#33) — medium-frequency bucket scan (no offer
  objects): below-min deficits matched to above-max surpluses by resource +
  transport class + route existence (O(1) per pair via per-pass road
  components); published score rule `priority / (1 + d²·w)` with a distance
  weight that actually bites (CS1 ships 1e-7 there — the confirmed cause of
  its cross-map hauling); resources round-robined across matching frames;
  unmatched deficits persist on a starvation board.
- **Freight execution** (#34) — each order gets an idle class-compatible truck
  from the nearest depot (none free ⇒ the order waits — bounded throughput is
  the point); trip machine ToPickup → Loading → ToDropoff → Unloading →
  ReturnToDepot with the dock transfer rate as the loading bottleneck;
  matched supply is earmarked so two orders never load the same tonnes;
  severed pre-load trips requeue the order, loaded trucks hold and resume.
- **Shuttles retired** (#35) — `CreateShuttle` is now paired-policy sugar
  (source exports everything, sink imports to 90%); the M1 chain runs on the
  dispatcher alone; save v3 adds policy bands, orders in flight, and mid-trip
  truck state (hash-identical round trip mid-haul).
- **Dispatch readout** (#36) — DISPATCH panel: fleet busy/idle, per-depot
  parked/out, waiting orders with age, oldest starving deficit; storage
  inspect shows per-resource band bars with role (DEMANDING/SUPPLYING/in
  band); depot inspect lists slot occupancy and each truck's trip phase.

Benchmark gate (`cargo run --release --bin bench_dispatch`, #37): dispatcher
cost **1.42× at 2× storages** (sub-linear gate < 2×); full load **1498 live
freight orders at mean 0.74 ms/tick** (p95 1.35 ms) against the ≤2 ms budget.
M1/M2 gates still green (bench_chain 0.13 ms, bench_citizens 0.41 ms).

Acceptance video (#37): `cargo run --release --bin capture_m3 -- frames
screenshots/result/3 480` then ffmpeg; latest render at
`screenshots/result/3/video.mp4` (local, untracked). Arc: the parked two-truck
fleet rolls out and the warehouses fill to their bands; draining one warehouse
makes the dispatcher refill it; draining everything at once outruns the fleet
and the order queue grows on the DISPATCH panel.

## Run

```
cargo run            # the game
cargo test           # 67 sim tests
```

Keys: `1` dirt road · `2` paved road · `3` building (cycles kind) · `4` wire ·
`5` haul policy (click source, then sink) · `Esc` inspect · `X` cut · `R` rebuild last cut · `Space` pause ·
`[` `]` speed · `F5` quicksave · `F9` quickload · WASD pan · Q/E rotate ·
wheel zoom.

With a **depot** selected: `T` buy a bulk truck · `Y` buy a covered truck.
With a **storage** selected: `B` cycles the focused resource · `,` `.`
lower/raise its min band 5% · `Shift+,` `Shift+.` the max band. Buildings the
dispatcher cannot feed pulse a red ring.

## What's next

P1 "First Light" done (#16, zero-spend). B2 staffing and B3 dispatcher
complete (above). Next: B4 per the ladder (see `ROADMAP.md`, including the
parallel P-ladder).

## Assets

| Asset | Source | Status |
|---|---|---|
| Buildings, trucks, poles, smoke | Multi-part procedural meshes (P1) | in use |
| Ground/road textures | ambientCG (CC0): Grass001, Ground048, Asphalt010, Gravel022 | in use |
| HUD font | Fira Sans (OFL 1.1, license bundled) | in use |
| Generated art pass | `/asset-gen` (paid) | deferred, post-demo decision |
