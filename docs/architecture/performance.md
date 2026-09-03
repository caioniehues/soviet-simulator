# Performance

**Kind:** architecture
**Authority:** advisory
**Status:** draft
**Owner:** architecture
**Verified-at:** `266f7b2`
**Last verified:** 2026-09-03

## The contract and its gate

The charter targets **250,000 citizen identities at 60 fps on the development machine** and
delegates the benchmark gates to the implementation plan. **No simulation benchmark exists.** The 250k
benchmark contract (`sov-1ae`) was cancelled 2026-08-27; its WIP is preserved unmerged on
`wip/sov-m0q-wave1`. `sov-bo3` (an OOM in `LAV::iter_keys`) is fixed, not a live blocker:
iteration is bounded to `vs.len() + 1` and a corrupt cycle is flagged instead of collected forever
(`geom/src/skeleton.rs:738-754`); `skeleton` then refuses the corrupt polygon
(`geom/src/skeleton.rs:910-912`). The remaining scale gate is unbuilt, not blocked by that defect.
A green test suite or a save round-trip proves nothing about this target.

## Current substrate

Test cities have ~50–100 humans. Every system runs every tick; every human is drawn; routing is a
fresh A* per request; stock is a `BTreeMap` per item; `PersonalInfo` allocates a `String` per
citizen. No simulation CPU or whole-world profiling baseline is recorded in the repository; the
only committed baseline is renderer GPU timing
(`engine_demo/gpu_timing_baselines/radv-navi3x/baseline.json`, adapter-scoped, render passes only).

## The hierarchy — optimise in this order

1. **Representation and algorithm** — the wrong data structure cannot be threaded into speed.
2. **Update cadence** — stable things sleep ([time and events](time-and-events.md)).
3. **Locality and SoA** — dense cores, sparse side stores ([state storage](state-storage.md)).
4. **Incremental propagation** — change journal instead of rescans.
5. **Hierarchy and cache** — routing, components, keyed by revision.
6. **Parallelism** — only after 1–5, and only deterministically ([parallelism](parallelism.md)).
7. **SIMD** — only after profiling names a hot kernel; contiguous layouts first.

Do not jump to SIMD or threads before fixing per-citizen scanning and cache-hostile structures.

## The arithmetic nobody has done

Lane G: ~320 bytes per citizen of proposed state → 80 MB at 250k; a full sequential pass ~2.7 ms
at 30 GB/s, 3–8× worse scattered; several full passes per 20 ms tick exhaust the budget on memory
traffic alone. The design thread's answer — "stable things sleep" — is right and unquantified.
**The active fraction per tick is the number that decides feasibility,** and no source states it.
If 10 % wake per tick, 25,000 active entities is plausible but tight.

## Memory budgets

Explicit byte budgets, asserted at compile time, for `CitizenCore`, `HouseholdCore`, vehicle hot
state, `Haul`, scheduled event, causal fact, route-cache entry. Exact numbers need profiling and
become an accepted decision ([memory budget standard](../engineering/performance.md)).

## Benchmarks to build (none exist)

250k whole-world finite simulation benchmark — a fixed-seed, fixed-tick CPU runner, not the
`headless` lockstep server loop (`headless/src/main.rs:64-71`), which ticks merged server inputs
unboundedly and contains no replay loader. Replay advancement lives in the native app
(`native_app/src/network.rs:82-95`) and `SimulationReplayLoader` (`simulation/src/lib.rs:155-171`).
Further gates: routing; citizen daily-event; material
allocation; logistics transfer; snapshot publication; utility solvers; render-instance
preparation. Instruction-level benchmarks (`iai-callgrind`, needs valgrind) for hot kernels only
after the whole-world bottlenecks are known; Criterion for wall-clock. The `.planning/` tooling
handoff and `sov-m0q` hold the benchmark epic's state.

## Related

- [Time and events](time-and-events.md)
- [State storage](state-storage.md)
- [Performance standard](../engineering/performance.md), [benchmarking standard](../engineering/benchmarking.md)
- [Benchmarking (guide)](../developer/benchmarking.md), [profiling (guide)](../developer/profiling.md)
- [Tooling wave handoff](../plan/iterations/HANDOFF-2026-08-27-tooling-wave.md)
