# Benchmarking

**Kind:** guide
**Authority:** operational
**Status:** active
**Owner:** project lead
**Last verified:** 2026-08-28

There is no benchmark runner in the repository. This guide says how to add one so its results can
be trusted, and what the first ones should be.

## Adding a wall-clock benchmark

1. Add `criterion` as a dev-dependency of the crate (check the [dependency policy](../process/dependency-policy.md);
   re-record it after the lockfile changes).
2. `benches/<name>.rs` with a deterministic fixture: fixed seed, fixed `WorldCommand` list, fixed
   tick count. The headless `Simulation` (`TestCtx`-style construction) is the fixture.
3. `cargo bench -p simulation --bench <name>`; record command, commit, machine class and output in
   the `bd` issue or the research page that motivated it.
4. Compare against a recorded baseline on the same machine; a single run is not a result.

## Adding an instruction-count benchmark

`iai-callgrind` (needs valgrind installed) for a hot kernel that a whole-world profile has named.
Not before.

## The first benchmarks the project needs

1. **Whole world at 250k, headless** — the charter's gate. Blocked on `sov-bo3`; the cancelled
   `sov-1ae` WIP on `wip/sov-m0q-wave1` is the starting point.
2. **Routing** — A* per request at city scale.
3. **Citizen daily events** — once citizens are scheduled actors.
4. **Material allocation** and **logistics transfer** — `make_trades`, `advance_dispatches`.
5. **Snapshot publication** — once snapshots exist.
6. **Utility solvers** and **render-instance preparation** — as they arrive.

## Reporting

Numbers without their run are not evidence ([benchmarking standard](../engineering/benchmarking.md)).
Never estimate.

## Related

- [Performance (architecture)](../architecture/performance.md)
- [Profiling](profiling.md)
- [Benchmarking standard](../engineering/benchmarking.md)
