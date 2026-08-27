# Memory Index

- [Dispatcher truck-pool semantics](dispatcher-truck-pool.md) — how a truck leaks out of the pool, and the only way to observe it in a test
- [Test setup traps](sim-test-setup-traps.md) — every factory spawns its own truck; company footprints come from Lua; build_special_building unwraps
- [Stale briefs on long-lived issues](feedback-stale-brief-check.md) — check bd comments before implementing; a multi-round issue's brief may describe finished work
- [`--exact` forces an integration test](test-exact-name-module-path.md) — a bare test name in the acceptance command cannot be satisfied by a unit test in `src/`
- [TestCtx always has a freight station](testctx-always-has-freight-station.md) — START_COMMANDS seeds one unconditionally; find_external has no distance cutoff; how to actually remove it (two steps, not one)
