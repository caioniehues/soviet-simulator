# Testing standard

**Kind:** standard
**Authority:** operational (Phase 3 of the development cycle enforces it)
**Status:** active
**Owner:** project lead
**Last verified:** 2026-08-28

> **A guard nobody has seen fail proves nothing.**

## Rules

1. **Must:** every new guard is seen failing — mutate what it protects, watch it go red, paste the
   real output, revert (Phase 3, `evidence-auditor`). Mechanical form: cargo-mutants on eligible
   changes ([mutation policy](../process/mutation-policy.md)).
2. **Must:** a test filter executes at least one test. `cargo test -p simulation sentinel` once ran
   zero tests and exited green; that is the failure this rule prevents.
3. **Must:** an acceptance test for a specification claim is named for it (`evid_<subsystem>_<claim>`)
   and the spec's evidence table names the deliberately wrong implementation the test must reject.
   **None of the 107 `evid_*` tests exists yet**; each is created with its mechanism, never before.
4. **Must:** a test asserts the behaviour the story claims, not arithmetic that happens to hold.
5. **Must:** ledger tests assert the conservation identity, not only the happy path
   ([simulation transitions](simulation-transitions.md)).
6. **Must:** every replayable transition has an idempotency test — apply twice, second is a no-op.
7. **Should:** changes to scheduling, randomness or ordering add a repeat-run determinism check
   (two fresh simulations, same commands, equal digests), not only `check_determinism`'s round-trip.
8. **Should:** property tests for conservation and state machines. `quickcheck` is already a
   dev-dependency; adding `proptest` alongside it is an open decision — do not add both without one.
9. **Should:** simplified engineering solvers are validated against a reference oracle (EPANET,
   SWMM, HEC, IDM, CTM cases) in a test, without making the oracle a runtime dependency.
10. **Must:** UI and renderer changes are proven by an inspected frame or short video, because the
    sim test harness cannot drive the UI (`CLAUDE.md` §Delivery; [MCP harness proposal](../plan/proposals/mcp-test-harness.md)).

## The harness

`cargo test -p simulation` — parallel-safe. `TestCtx` (`simulation/src/tests/mod.rs`): `new`,
`build_roads`, `build_house_near`/`build_house_at`, `apply(&[WorldCommand])`, `tick`,
`advance_ticks(n)`; the round-trip determinism check runs inside `tick` and every 25 ticks in
`advance_ticks`. Scenario helpers live in `tests/scenarios/mod.rs` (`build_company_at`,
`setup_seller_buyer`, `drain_dispatches`, `remove_default_freight_station`). A new scenario file
must be declared in `tests/scenarios/mod.rs` — the shared file that ownership tables forget.

## Related

- [Writing evidence tests (guide)](../developer/writing-evidence-tests.md)
- [Determinism standard](determinism.md)
- [Mutation policy](../process/mutation-policy.md)
- [Development cycle — Phase 3](../process/development-cycle.md)
