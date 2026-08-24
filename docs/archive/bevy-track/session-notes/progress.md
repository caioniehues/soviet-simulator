# Progress — M2.6 (issue #28)

## Session 2026-08-17 (early morning)
- 01:39 Resumed track; read ticket #28, surveyed all sim modules + specs + ecosystem survey + render sync. Design settled (findings.md).
- 01:45 Cargo.toml: +serde, +postcard. Wrote `src/sim/needs.rs` (needs, meals, wellbeing, sticky attendance roll) + 8 tests.
- 01:47 commute.rs: depart gate reads `CitizenNeeds.attends` (Option — plugin-less tests unaffected). Fixed double-borrow in a needs test.
- 01:48 Visibility pass for the loader: id counters pub across 6 modules; attach_staffing/attach_flat_tables loader guards; RoadSegment::recompile extracted and compile_dirty_segments refactored onto it.
- Next: write src/sim/save.rs, then game/saveload.rs + lib.rs wiring, then cargo test.

## Test results
- Not run yet this session (module incomplete — save.rs referenced by mod.rs but not yet written).
- 01:55 save.rs written; fixed observer-order clobber hazard (optionals before Building); 52/52 tests after correcting a wrong test assumption (planning pass filled the Factory first, not the Mine — loader was faithful).
- 02:00 capture.rs plugin set repaired (broken since M2.2), headless smoke render verified visually. README + .gitignore updated.
- 02:02 Committed bd8c5c3, pushed, closed #28, memory updated. M2.6 done.

## Test results (final)
- cargo test: 52 passed, 0 failed. Clippy: no new warnings (5 preexisting).

# Progress — M3 (issues #33–#37)

## Session 2026-08-17 (02:40–)
- Survey done (findings.md). M3.3+M3.4 shipped as sim/dispatch.rs (matching pass + freight trip machine), 12 tests.
- M3.5: shuttles retired to policy sugar; save v3 (bands, orders, trip state); bins migrated; bench_chain 0.13ms, bench_citizens 0.41ms.
- M3.6: HUD dispatch panel, band bars w/ roles, depot fleet inspect.
- M3.7: bench_dispatch green (dispatcher-only 1.42x at 2x storages; 1498 live orders at 0.74ms vs 2ms gate). Perf: dock cache, component-bucketed suppliers, lazy goal resolution, per-tick order index.
- capture_m3 acceptance clip rendering (self-balance → drain refill → queue growth).
- M3 closed: #30/#33–#37 all shipped and pushed (51e059b). Polish: in-game truck purchase, stall requeue, order chaining, band tuning UI, starvation rings. 68 tests, 3 bench gates, 16s video verified.

# Progress — B4 (issues #38–#44)

## Session 2026-08-17 (04:16–)
- Issues filed: #38 parent, #39–#44 subtickets (B4.1–B4.6).
- B4.1 implemented: sim/pathfinding.rs (GraphSnapshot CSR mirror, A* w/ time-like
  cost + congestion jitter band, PathService request/poll on AsyncComputeTaskPool,
  snapshot key = id counters + node count + mod-index sum + congestion_version).
- vehicles.rs: BFS find_route deleted; ActiveVehicle.pending_path (transient).
- dispatch.rs drive_toward: async request/hold/resolve; commute walkers use
  route_now (Pedestrian profile, no jitter). Build clean; cargo test running (bg).
- B4.2 shipped: sim/traffic.rs SegmentTraffic (0–100 low-pass density, blocked
  flag, cost_multiplier into A*); congested-pair avoidance test across seeds.
- B4.3 shipped (771b594, #41 closed): LaneOccupancy per directed lane (LanePrep
  set), gap-capped advance (8 m footprint), junction entry gating (spillback),
  ActiveVehicle.blocked_ticks. 77 tests, 3 bench gates green.
- B4.4 shipped (c7e36d9, #42 closed): re-route pre-step in drive_toward
  (REROUTE_AFTER=90, same-corridor answers discarded), StallBoard (STALL_AFTER=240)
  + report_stalls, HUD STALL line + red corridor glow. 78 tests, gates green.
- B4.5 shipped (7b878d9, #43 closed): RoadSegment.curve bezier polyline w/
  node-fan tangents (Catmull at 2-seg nodes, chord at junctions), point_at()
  drives vehicles+commuters, curved ribbon mesh, save re-derives. 79 tests.
- B4.6 shipped (d8c3753, #44+#38 closed): bench_traffic 10k vehicles at
  1.12 ms mean (16 ms gate), capture_m4 demo (S-curve, jam, stall alarm,
  bypass reroute) — 16 s video verified frame-by-frame. B4 COMPLETE.

# Progress — B5 (issues #45–#51)

## Session 2026-08-17 (cont.)
- Issues filed #45–#51. B5.1 shipped (343b649, #46 closed): BusStop kind,
  TransitLine loop model + edit queue, key-6 line tool. 82 tests.
- B5.2 shipped (fc65670, #47 closed): Bus kind + Passenger class, AssignBus,
  run_buses loop over shared drive_toward, dwell, park-on-delete. 84 tests.
- B5.3 shipped (2a55de9, #48 closed): CommutePhase machine, StopQueues,
  board/alight w/ BUS_CAPACITY, BUS_SPEED 24, give-up fallback. 86 tests.
- B5.4 shipped (bd74ff1, #49 closed): transit-extended labour catchment +
  behavioural commute-budget refusal; enable/degrade test. 87 tests.
- B5.5 shipped (d262537, #50 closed): TRANSIT HUD line, line overlay, bus
  body, rider visibility. 87 tests.
- B5.6 shipped (e2ba33b + 13f2ec1, #51+#45 closed): bench_transit 5000
  concurrent riders at 0.26 ms; phantom-itinerary fix (component check);
  capture_m5 16 s video verified. B5 COMPLETE. 87 tests, 5 gates.

# Progress — B6 (issues #52–#58)
- Issues filed #52–#58; plan reset to B6 phases.
- B6.1 shipped (ba455a4, #53 closed): ConstructionSite 3-phase model, named
  bottlenecks, inert production gating, opt-in plugin. 90 tests.
- B6.2 shipped (b4f8a56, #54 closed): outstanding-tracking site bands,
  leftover purge, activation restores kind yard/policies. 91 tests.
- B6.3 shipped (7ca180d, #55 closed): office + excavator/crane fleet,
  assign/run machine loops, standing-radius fix. 93 tests.
- B6.4 shipped (933aa78, #56 closed): labour gate, site inspect + stall
  rings, rising render, live-game wiring, T/Y/U purchases. 93 tests.
- B6.5 shipped (ce15f8c, #57 closed): Demolish edit + Delete key, eviction
  requeue, line pruning, freight write-off, site cancel. 95 tests.
- B6.6 shipped (3322c5c, #58+#52 closed): bench_sites 100 sites at 0.11 ms,
  capture_m6 16 s verified (rise -> NO MATERIAL -> delivery+crane ->
  activation). B6 COMPLETE. 95 tests, 6 bench gates.

# Progress — B7 (issues #59–#63)
- Issues filed #59–#63; plan reset to B7 phases.
- B7.1 shipped (a197ff5, #60 closed): scored allocation + doubling +
  overcrowding rest cap. 98 tests.
- B7.2 shipped (6aa5a55, #61 closed): daily fission + couple pairing.
  100 tests.
- B7.3 shipped (1f7547a, #62 closed): zoning districts + siting gate + PLAN
  dashboard + key-7 tool. 102 tests.
- B7.4 shipped (347caf0, #63+#59 closed): gates held (citizens 0.57 ms,
  transit 0.28 ms after doubling-proximity policy fix the bench caught),
  SimDriver ordering fix, capture_m7 verified. B7 COMPLETE. 102 tests.

# Progress — B8 (issues #64–#68)
- Issues filed #64–#68; plan reset to B8 phases.
- B8.1 shipped (f8e656a, #65 closed): sim/network.rs pool solver, priority
  brownout, dwelling draw + dark-home cap. 105 tests.
- B8.2 shipped (1e0d6d1, #66 closed): NetKind hardware + save v4, water
  cycle solve, factory Liebig water gate. 107 tests.
- B8.3 in flight: heat.rs drafted (climate sinusoid, HeatPlant coal burn,
  Heated solve over Heat spans, 2 tests). COLD_HOME_REST_CAP=0.65 wired in
  needs.rs (min-stacks under dark-home 0.75); HUD SEASON +temp / COLD HOMES
  line. HeatPlant kind fan-out (buildings/save/labour/storage/game/tools/
  vehicles + plugin registration) delegated to haiku agent.
- B8.3 shipped (6ac3586, #67 closed): Climate sinusoid + HeatPlant coal
  burn + Heated solve + cold-home cap 0.65 + HUD SEASON line. 109 tests.
- B8.4 shipped (9e17acc, #68+#64 closed): bench_networks 0.044 ms (gate
  1 ms), seven gates green, capture_m8 16 s verified frame-by-frame,
  README B8 + ROADMAP [x]. B8 COMPLETE. 109 tests, 7 bench gates.

# Progress — G1 (issues #75–#79; B9 #69–#74 filed but PAUSED)
- Grilling session settled direction: planner fantasy, quotas + winter
  pressure, allocation points, toolbar UI, played-session acceptance.
- ROADMAP G-ladder added; issues #75–#79 filed; plan reset to G1 phases.
- G1.1 shipped (#76 closed): sim/plan.rs quota periods + rouble tranche +
  Treasury gating vehicles/recruitment + HUD plan block. 112 tests.
- G1.2 shipped (#80 closed): CustomsOffice kind, dock-rate export sales,
  border drive-in gate (InTransitFromBorder). 114 tests. NOTE: plan/customs
  state not yet in save.rs — land before G1.5 playtest.
- G1.3 shipped (#77 closed): game/toolbar.rs category bar + flyouts,
  ToolMode-driven highlight, UiHover click suppression, HUD Option<Res>
  for capture bins. Verified via capture frame. 114 tests.
- G1.4 shipped (#78 closed): fullscreen Plan ledger on P (quota bars,
  treasury, tranche forecast), right HUD column flex-stacked. Verified via
  capture frames. 114 tests.
- G1.5 code shipped (59b5f86): authored 5-period ladder, save v5
  (plan/treasury/border), PlacePrebuilt customs (observer-order bug caught
  on camera), legacy bins funded w/ infinite treasury, 7 gates green,
  capture_g1 verified. 115 tests. AWAITING user 30-min playtest (#79).
- Wayfinder brief written + pushed (docs/wayfinder-brief.md): full gap
  analysis vs W&R install (80 types/45 resources), 22 decision tickets,
  suggested /wayfinder prompt. Next: user playtest, then fresh-session
  /wayfinder pointed at the brief.
