# Benchmarking standard

**Kind:** standard
**Authority:** operational
**Status:** active
**Owner:** project lead
**Last verified:** 2026-08-28

## Rules

1. **Must:** a benchmark result is reported with the exact command, the commit, the machine class
   and the raw output. A number without its run is not evidence.
2. **Must:** whole-world benchmarks come before instruction-level ones. A hot kernel is
   micro-benchmarked only after a whole-world run names it.
3. **Must:** the 250k whole-game headless benchmark is the final gate for the charter's
   performance target; a green test suite or a save round-trip does not stand in for it.
4. **Should:** the benchmark set, when it exists, covers: whole world at 250k; routing; citizen
   daily events; material allocation; logistics transfer; snapshot publication; utility solvers;
   render-instance preparation.
5. **Should:** wall-clock benchmarks use Criterion; instruction and cache counts use
   `iai-callgrind` (requires valgrind) for hot kernels; both are recorded, never estimated.
6. **Must not:** claim a regression or an improvement from intuition, a single run, or a debug
   build (`--release` is not optional; a debug build is unplayably slow).
7. **Must:** a benchmark's fixture is deterministic (fixed seed, fixed commands) so two runs are
   comparable.

## What exists today

No bench runner. `sov-1ae` (the first 250k benchmark contract) is cancelled with WIP preserved on
`wip/sov-m0q-wave1`; `sov-bo3` (OOM constructing a large city) is the known blocker. The
`perf-engineer` role owns this gate when it exists.

## Related

- [Performance standard](performance.md)
- [Performance (architecture)](../architecture/performance.md)
- [Benchmarking (guide)](../developer/benchmarking.md)
- [Tooling wave handoff](../plan/iterations/HANDOFF-2026-08-27-tooling-wave.md)
