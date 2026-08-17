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

## Status — B4 "Streets Alive" complete (#38)

Traffic gets real at scale: async pathfinding, emergent congestion, jams as
car-following, stalls as planning signal, curved roads.

- **Pathfinding engine** (#39) — BFS replaced by cost-aware A* over a packed
  CSR snapshot of the lane graph, solved on the `AsyncComputeTaskPool` (never
  the sim thread): trucks submit a request, hold in place, and install the
  route when the ticket resolves. Cost is time-like (`length / speed_mod ×
  congestion`) with a CS1-style scatter band so identical trips spread over
  near-equal corridors; walkers keep a deterministic plain-distance profile.
  Snapshot refresh keys on graph id counters + modification indices +
  congestion version — a route solved against a stale snapshot is still
  valid, just possibly suboptimal.
- **Congestion** (#40) — vehicles stamp load onto their current segment; a
  low-pass pass (±5 per 15-tick interval, CS1's confirmed shape) turns it
  into a 0–100 density scalar that multiplies routing cost up to 2×.
  A saturated corridor genuinely prices traffic onto the detour.
- **Lane reservation + car-following** (#41) — per-lane occupancy rebuilt
  each movement pass; a follower's advance is capped by the gap to its
  leader (8 m footprint) and junction crossing waits for a clear mouth, so
  queues and spillback emerge from local rules alone — no queue objects.
- **Wait / re-route / stall** (#42) — a truck held past 90 ticks asks the
  solver for a better corridor (identical answers are discarded so the
  counter keeps climbing); held past 240 ticks it registers a corridor stall
  on the **StallBoard**: HUD STALL line + pulsing red glow along the jammed
  segment's curve. Nothing ever despawns — a jam is information.
- **Curved roads** (#43) — segments compile a quantized cubic-bezier
  centreline with node-fan tangents (two-segment through-nodes smooth
  Catmull-style; endpoints and junctions stay chord-straight). Motion, arc
  length, and the render ribbon all read the same polyline; saves are
  unchanged because geometry re-derives on load.

Benchmark gate (`cargo run --release --bin bench_traffic`, #44): **10 000
simultaneous vehicles** on a 24×24 grid (1104 segments), full pipeline —
async routing churn, occupancy, congestion — at **mean 1.12 ms/tick** (p95
1.47 ms) against the 16 ms (60 fps) budget. Prior gates still green
(bench_chain 0.075 ms, bench_citizens 0.37 ms, bench_dispatch 1.34×/0.34 ms).

Acceptance video (#44): `cargo run --release --bin capture_m4 -- frames
screenshots/result/4 480` then ffmpeg; latest render at
`screenshots/result/4/video.mp4` (local, untracked). Arc: freight flows over
an S-curved corridor; a breakdown convoy jams the middle bend and the fleet
queues behind it at footprint spacing; the corridor registers a STALL (HUD
line + red glow along the curve); a bypass built moments later is priced
against the congestion and the fleet re-routes onto it; flow resumes, orders
drain to zero, and the dead convoy is still there — never despawned.

## Status — B5 "The Lines" complete (#45)

Public transit is playable: player-drawn bus lines, depot-owned buses on the
traffic network, citizens planning multi-leg trips, and mode choice that makes
the line the labour supply when the network deserves it.

- **Stops + lines** (#46) — `BusStop` shelter (docks a road node like any
  yard), `TransitLine` as an ordered stop loop created at the command
  barrier; the key-6 tool clicks stops in order and right-click closes the
  loop; each line renders a coloured overlay.
- **Buses** (#47) — `VehicleKind::Bus` with `TransportClass::Passenger` (no
  resource maps to it, so the freight dispatcher can never seize one);
  bought into depot slots, assigned to a line, and driven stop-to-stop
  through the whole B4 stack — async routes, car-following, congestion,
  stalls. Passenger transit runs on the commute travel-second timescale
  (24 m/s cruise — riding genuinely beats the 8 m/s walk).
- **Riders** (#48) — commuter trips become walk → queue → ride → walk:
  `StopQueues` FIFO at shelters, boarding bounded by the 30-seat capacity
  (the overflow stays visibly queued — overcrowding is a signal, not a
  despawn), riders follow the bus and resume on foot after alighting; a
  vanished bus or endless wait degrades to walking past a give-up threshold.
- **Mode choice + degradation** (#49) — per-trip cost comparison (walk legs
  + flat wait penalty + ride time, simutrans-flavoured weights); labour
  feasibility accepts a transit itinerary inside the same 120 s commute
  budget **only when its walk legs are road-connected** (component check —
  a straight-line estimate never binds a worker to a phantom itinerary);
  workday departures refuse over-budget trips, so deleting the only viable
  line leaves the far factory unstaffed while tenure holds.
- **Legibility** (#50) — TRANSIT line on the dispatch panel (lines, buses
  out, aboard, waiting), per-line coloured loop overlay, a distinct teal bus
  body, riders hidden while aboard, queues clustering at shelters.

Benchmark gate (`cargo run --release --bin bench_transit`, #51): **5 000
concurrent multi-leg transit trips** across 125 districts whose corridors
exceed walking tolerance, deliberately undersized single-bus fleets so
shelter queues are part of the load — **mean 0.26 ms/tick** (p95 0.69 ms)
against the 2 ms band budget. All prior gates re-run green.

Acceptance video (#51): `cargo run --release --bin capture_m5 -- frames
screenshots/result/5 480` then ffmpeg; latest render at
`screenshots/result/5/video.mp4` (local, untracked). Arc: commuters stream
to the shelter and the single bus boards a full 30 — TRANSIT panel reads
"30 aboard, 10 waiting" while the overflow stands at the stop; riders arrive
and the inspected far factory staffs up; the line is deleted mid-clip, the
bus dead-heads home to its slot, and next morning the factory reads
"0 present / 10 assigned" — the plan problem is the missing line.

## Status — B6 "Building for Real" complete (#52)

The construction stub is dead: a blueprint becomes a building only through
the phase ladder, consuming delivered materials and machine-work — "we built
it", never "we bought it".

- **ConstructionSite** (#53) — every placed building starts as a 3-phase
  site (earthworks → structure → finishing) with a footprint-area bill of
  quantities; phased construction is plugin-opt-in (fixtures keep fiat
  placement, the running game builds for real). The building is inert —
  production, power, hiring all skip it — until the site completes;
  component removal *is* activation. Bill v1 rides the existing resource
  set (gravel pad, goods as structure stand-in) until B9/B10 industry.
- **Materials via the dispatcher** (#54) — a site is just another storage:
  its band tracks the current phase's outstanding tonnage, the ordinary
  matcher/fleet deliver, finished phases fold leftovers into the works, and
  an unsupplied site sits on the DeficitBoard like any starving yard.
- **Office + machine fleet** (#55) — ConstructionOffice with slot-bound
  excavators (`GROUNDWORKS`) and cranes (`CRANE`); idle machines
  self-dispatch to the nearest site whose current phase wants their skill,
  drive out over the full traffic stack, park, and add their skill to the
  site's throughput — W&R's duration law: `phase_time = work / Σskill`,
  never a timer. No matching machine anywhere ⇒ the phase stalls.
- **Lifecycle + legibility** (#56) — inspect shows phase, work %, bill and
  the *named* stall (`NO MATERIAL` amber ring / `NO MACHINE` grey ring,
  pulsing); buildings visibly rise with real progress; dispatch panel SITES
  line; office `T`/`Y` machine purchases, depot `U` bus purchase.
- **Demolition first cut** (#57) — `Delete` demolishes the selected
  building; everything self-heals: workers via the sever pass, households
  evicted into the visible housing queue, transit lines prune dead shelters,
  riders alight and walk, freight bound for the rubble writes off, machines
  abandon cancelled sites. Explosives/rubble arrive with the demolition
  office (later stage per spec).

Benchmark gate (`cargo run --release --bin bench_sites`, #58): **100
concurrent construction sites** — dispatcher hauling every bill while 200
machines drive and work — at **mean 0.11 ms/tick** (p95 0.16 ms) against
the 2 ms budget. All five prior gates re-run green.

Acceptance video (#58): `cargo run --release --bin capture_m6 -- frames
screenshots/result/6 480` then ffmpeg; latest render at
`screenshots/result/6/video.mp4` (local, untracked). Arc: the gravel pad is
hauled and graded and the squat frame rises; the structure phase starves —
amber ring, "STALLED: NO MATERIAL" in the inspect panel; a goods shipment
lands and the freshly bought crane drives out; the block climbs phase by
phase and snaps to full height, activated, with both machines back on the
office apron.

## Run

```
cargo run            # the game
cargo test           # 95 sim tests
```

Keys: `1` dirt road · `2` paved road · `3` building (cycles kind) · `4` wire ·
`5` haul policy (click source, then sink) · `6` bus line (click stops, right-click closes) · `Esc` inspect · `X` cut · `R` rebuild last cut · `Space` pause ·
`[` `]` speed · `F5` quicksave · `F9` quickload · WASD pan · Q/E rotate ·
wheel zoom.

With a **depot** selected: `T` buy a bulk truck · `Y` covered truck · `U` bus.
With a **construction office** selected: `T` buy an excavator · `Y` a crane.
`Delete` demolishes the selected building.
With a **storage** selected: `B` cycles the focused resource · `,` `.`
lower/raise its min band 5% · `Shift+,` `Shift+.` the max band. Buildings the
dispatcher cannot feed pulse a red ring.

## What's next

P1 "First Light" done (#16, zero-spend). B2–B6 complete (above): staffing,
dispatcher, traffic at scale, public transit, phased construction. Next: B7
housing-and-the-plan per the ladder (see `ROADMAP.md`, including the
parallel P-ladder).

## Assets

| Asset | Source | Status |
|---|---|---|
| Buildings, trucks, poles, smoke | Multi-part procedural meshes (P1) | in use |
| Ground/road textures | ambientCG (CC0): Grass001, Ground048, Asphalt010, Gravel022 | in use |
| HUD font | Fira Sans (OFL 1.1, license bundled) | in use |
| Generated art pass | `/asset-gen` (paid) | deferred, post-demo decision |
