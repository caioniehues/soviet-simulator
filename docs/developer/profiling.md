# Profiling

**Kind:** guide
**Authority:** operational
**Status:** active
**Owner:** project lead
**Last verified:** 2026-08-28

## Rules that apply

Measure before optimising; optimise in the order representation → cadence → locality → incremental
→ hierarchy → parallelism → SIMD ([performance standard](../engineering/performance.md)). Record
the command and the commit with any number you report.

## CPU

- Always `--release`. Build under `~/`, never `/tmp`.
- `perf record -g cargo run --release -p headless` then `perf report`, or `cargo flamegraph`
  (requires `perf`). The headless binary isolates the simulation from the renderer.
- `SeqSchedule` has per-system timing available in the scheduler (`ordered-float` is used for
  it); once phase labels exist, time per phase is the first number to look at.
- Tracy/`profiling` spans exist in the engine; use them for frame-level attribution in
  `native_app`.

## GPU

`engine/src/gpu_timing.rs` provides GPU timestamp queries; `engine/src/capture.rs` frame capture.
Renderer work is judged from frames, not from build output.

## Memory

- `/usr/bin/time -v` for peak RSS; `heaptrack` or `valgrind --tool=massif` for allocation sites.
  The known large-city blocker is `sov-bo3` (OOM at 17.6 GB RSS in `LAV::iter_keys`).
- Check per-citizen allocations first (`PersonalInfo.name: String`, boxed per human).

## What to profile first for the 250k target

1. A headless city large enough to matter (blocked on `sov-bo3`).
2. Per-system time per tick; which systems scan every entity.
3. Routing calls per tick.
4. Render instance count versus visible count.

## Related

- [Performance (architecture)](../architecture/performance.md)
- [Benchmarking (guide)](benchmarking.md)
- [Tooling wave handoff](../plan/iterations/HANDOFF-2026-08-27-tooling-wave.md)
