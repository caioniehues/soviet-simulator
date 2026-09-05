# Memory Index

- [Dispatcher truck-pool semantics](dispatcher-truck-pool.md) — how a truck leaks out of the pool, and the only way to observe it in a test
- [Test setup traps](sim-test-setup-traps.md) — every factory spawns its own truck; company footprints come from Lua; build_special_building unwraps
- [Stale briefs on long-lived issues](feedback-stale-brief-check.md) — check bd comments before implementing; a multi-round issue's brief may describe finished work
- [`--exact` forces an integration test](test-exact-name-module-path.md) — a bare test name in the acceptance command cannot be satisfied by a unit test in `src/`
- [Re-derive recorded spike numbers](rederive-recorded-spike-numbers.md) — a close reason's benchmark ratio may be an asymmetric timed region; sov-dda.3's "2.3x" inverted to 0.26x
- [TestCtx always has a freight station](testctx-always-has-freight-station.md) — START_COMMANDS seeds one unconditionally; how to actually remove it (two steps, not one). NOTE: its "find_external has no distance cutoff" line is STALE since sov-abs — see below
- [BuildingInfos is not a liveness oracle](binfos-is-not-a-liveness-oracle.md) — binfos entries survive demolition; only `map.buildings()` answers "is this building alive"
- [A graph zero is not an absence](graph-zero-callers-is-a-lie.md) — code-review-graph found 0 callers of `unpark`; grep found 3. Always cross-check
- [Refusal signals need caller rollback](refusal-signals-need-caller-rollback.md) — a caller that already reserved something must undo it on `false`, not just log
- [Dispatcher::free does not re-queue](dispatcher-free-does-not-requeue.md) — `reserve` drops the position cache entry; a freed truck is queryable only after the next `update`
- [Dispatch reachability: the 50-unit cutoff](dispatch-reachability-50-units.md) — a door >50 units from a lane never gets a truck; plus door_pos rotation and road-surgery traps
- [Tests passing on the teleport](tests-passing-on-the-teleport.md) — pre-sov-abs ext-trade credited instantly; two scenario tests were green for the wrong reason
- [Export gate blocks on the teleport](export-gate-blocks-on-teleport.md) — ungating exports (sov-nun) drains sellers to the border; measured A/B; must wait for sov-20g
- [Dead-entity leaks need reserved_by](dead-entity-leaks-need-reserved-by.md) — "every live truck is still reservable" can never detect a leak held by a despawned entity; use `Dispatcher::is_reserved`
- [Determinism guard's real reach](determinism-guard-reach.md) — test_world_survives_serde runs an EMPTY schedule; is_equal ignores the ECS World (sov-n8v)
- [Default city border is closed](default-city-border-is-closed.md) — no external trade and no cargo train until the player lays road; ratified, not a bug; plus three test-harness traps
- [A match rollback has four halves](market-match-rollback-both-halves.md) — the seller's SellOrder is the one every rollback forgets; clamp the restore to capital
- [The ext-trade block eats unplaceable orders](ext-trade-block-eats-unplaceable-orders.md) — fixed 2026-09-02; a domestic seller masks it from every existing test
