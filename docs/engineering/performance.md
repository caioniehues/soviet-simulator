# Performance standard

**Kind:** standard
**Authority:** operational
**Status:** active
**Owner:** project lead
**Last verified:** 2026-08-28

## Rules

1. **Must — measure before optimising.** A performance change cites the benchmark or profile that
   motivated it and the one that shows the gain ([benchmarking](benchmarking.md)).
2. **Must — justify any full-population per-tick scan.** A system that touches every citizen,
   household or enterprise every tick states why an event, a cadence or an index would not do.

   > **Stable state sleeps. Changed state propagates.**

3. **Should — event-driven slow state.** Sleep → wake on condition → decide → emit intent → commit
   → schedule next → sleep ([time and events](../architecture/time-and-events.md)).
4. **Should — filter cheaply, think expensively.** Narrow a population with dense or bitset
   indexes before running per-entity logic:

   ```text
   250,000 citizens → dense/bitset qualification filter → 2,100 candidates → detailed household decision
   ```

   The same shape applies to labour, education, healthcare, shopping, migration, transport and
   social networks.
5. **Must — cache by explicit revision or epoch** (topology revision, traffic epoch, policy
   version, data generation). Never "probably still valid".
6. **Must — optimise in order:** representation and algorithm → cadence → locality → incremental
   propagation → hierarchy and cache → parallelism → SIMD. Skipping ahead is a review send-back.
7. **Should — memory budgets.** Hot records have a stated byte budget, asserted at compile time,
   once profiling sets it ([state storage](../architecture/state-storage.md)).
8. **Must not:** per-citizen heap strings, vectors, behaviour trees, individually allocated timers,
   route searches or rendered bodies at 250k scale.
9. **Must not:** build in `/tmp` (it is a 16 GB tmpfs; a build there once killed a session —
   `task_plan.md` incident). Worktrees live under `~/`.

## What exists today

No benchmark, no budget, no profile in the repository; the 250k target has no gate
([performance (architecture)](../architecture/performance.md)).

## Related

- [Performance (architecture)](../architecture/performance.md)
- [Benchmarking standard](benchmarking.md)
- [Profiling (guide)](../developer/profiling.md)
