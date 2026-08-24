# Current simulation regression inventory

**Kind:** generated current-regression inventory
**Authority:** current substrate only
**Status:** informational — not target-proof evidence
**Generator:** `python3 docs/plan/iterations/evidence/build_evidence.py --extract docs/plan/iterations/extract/requirements.json --specifications docs/reference/specifications --bindings docs/plan/iterations/evidence/evid-spec-bindings.json --output-dir docs/plan/iterations/evidence`

These are every test currently listed by the serial `simulation` test binary. They are deliberately separate from planned `TARGET-EVID-*` scenarios; identical numeric fragments from the legacy corpus are never used as a target binding.

| Regression ID | Current test | Exact command | Scope |
| --- | --- | --- | --- |
| `REGRESSION-001` | `economy::ecostats::tests::history_is_not_zero` | `cargo test -p simulation economy::ecostats::tests::history_is_not_zero -- --test-threads=1` | current substrate regression |
| `REGRESSION-002` | `economy::market::tests::calculate_prices` | `cargo test -p simulation economy::market::tests::calculate_prices -- --test-threads=1` | current substrate regression |
| `REGRESSION-003` | `economy::market::tests::test_match_orders` | `cargo test -p simulation economy::market::tests::test_match_orders -- --test-threads=1` | current substrate regression |
| `REGRESSION-004` | `map::electricity_cache::tests::test_connectivity` | `cargo test -p simulation map::electricity_cache::tests::test_connectivity -- --test-threads=1` | current substrate regression |
| `REGRESSION-005` | `map::electricity_cache::tests::test_loop_removal` | `cargo test -p simulation map::electricity_cache::tests::test_loop_removal -- --test-threads=1` | current substrate regression |
| `REGRESSION-006` | `map::procgen::presets::tests::parismap_valid` | `cargo test -p simulation map::procgen::presets::tests::parismap_valid -- --test-threads=1` | current substrate regression |
| `REGRESSION-007` | `map::procgen::presets::tests::testfield_valid` | `cargo test -p simulation map::procgen::presets::tests::testfield_valid -- --test-threads=1` | current substrate regression |
| `REGRESSION-008` | `map_dynamic::dispatch::tests::bench_query` | `cargo test -p simulation map_dynamic::dispatch::tests::bench_query -- --test-threads=1` | current substrate regression |
| `REGRESSION-009` | `map_dynamic::dispatch::tests::dispatch_one_register_one_works` | `cargo test -p simulation map_dynamic::dispatch::tests::dispatch_one_register_one_works -- --test-threads=1` | current substrate regression |
| `REGRESSION-010` | `map_dynamic::dispatch::tests::query_same_lane_works` | `cargo test -p simulation map_dynamic::dispatch::tests::query_same_lane_works -- --test-threads=1` | current substrate regression |
| `REGRESSION-011` | `map_dynamic::dispatch::tests::query_two_lanes_bfs` | `cargo test -p simulation map_dynamic::dispatch::tests::query_two_lanes_bfs -- --test-threads=1` | current substrate regression |
| `REGRESSION-012` | `souls::freight_station::tests::test_deliver_to_freight_station_incrs_station` | `cargo test -p simulation souls::freight_station::tests::test_deliver_to_freight_station_incrs_station -- --test-threads=1` | current substrate regression |
| `REGRESSION-013` | `tests::scenarios::hoarding::scenario_0082_dispatch_gates_stock_not_match` | `cargo test -p simulation tests::scenarios::hoarding::scenario_0082_dispatch_gates_stock_not_match -- --test-threads=1` | current substrate regression |
| `REGRESSION-014` | `tests::scenarios::hoarding::scenario_0083_zero_trucks_blocks_delivery` | `cargo test -p simulation tests::scenarios::hoarding::scenario_0083_zero_trucks_blocks_delivery -- --test-threads=1` | current substrate regression |
| `REGRESSION-015` | `tests::scenarios::hoarding::scenario_0151_inflated_request_hoards_honest_does_not` | `cargo test -p simulation tests::scenarios::hoarding::scenario_0151_inflated_request_hoards_honest_does_not -- --test-threads=1` | current substrate regression |
| `REGRESSION-016` | `tests::scenarios::ledger::scenario_ledger_exttrade_double_spend` | `cargo test -p simulation tests::scenarios::ledger::scenario_ledger_exttrade_double_spend -- --test-threads=1` | current substrate regression |
| `REGRESSION-017` | `tests::scenarios::ledger::scenario_ledger_job_opening_reserve_leak` | `cargo test -p simulation tests::scenarios::ledger::scenario_ledger_job_opening_reserve_leak -- --test-threads=1` | current substrate regression |
| `REGRESSION-018` | `tests::scenarios::ledger::scenario_ledger_remove_leak` | `cargo test -p simulation tests::scenarios::ledger::scenario_ledger_remove_leak -- --test-threads=1` | current substrate regression |
| `REGRESSION-019` | `tests::scenarios::recipe_provided::scenario_0093_recipe_multi_input_multi_output` | `cargo test -p simulation tests::scenarios::recipe_provided::scenario_0093_recipe_multi_input_multi_output -- --test-threads=1` | current substrate regression |
| `REGRESSION-020` | `tests::scenarios::recipe_provided::scenario_0094_extraction_no_consumed_inputs` | `cargo test -p simulation tests::scenarios::recipe_provided::scenario_0094_extraction_no_consumed_inputs -- --test-threads=1` | current substrate regression |
| `REGRESSION-021` | `tests::scenarios::recipe_provided::scenario_0095_full_output_storage_halts_production` | `cargo test -p simulation tests::scenarios::recipe_provided::scenario_0095_full_output_storage_halts_production -- --test-threads=1` | current substrate regression |
| `REGRESSION-022` | `tests::scenarios::recipe_provided::scenario_0096_workforce_sourced_live_from_present_population` | `cargo test -p simulation tests::scenarios::recipe_provided::scenario_0096_workforce_sourced_live_from_present_population -- --test-threads=1` | current substrate regression |
| `REGRESSION-023` | `tests::scenarios::recipe_provided::scenario_0097_production_never_checks_treasury` | `cargo test -p simulation tests::scenarios::recipe_provided::scenario_0097_production_never_checks_treasury -- --test-threads=1` | current substrate regression |
| `REGRESSION-024` | `tests::scenarios::scenario_harness_smoke` | `cargo test -p simulation tests::scenarios::scenario_harness_smoke -- --test-threads=1` | current substrate regression |
| `REGRESSION-025` | `tests::test_iso::quickcheck_map_ser` | `cargo test -p simulation tests::test_iso::quickcheck_map_ser -- --test-threads=1` | current substrate regression |
| `REGRESSION-026` | `tests::test_iso::test_world_survives_serde` | `cargo test -p simulation tests::test_iso::test_world_survives_serde -- --test-threads=1` | current substrate regression |
