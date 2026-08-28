---
name: defect-skeleton-oom
description: Small building footprints drive geom::skeleton into an unbounded list walk that OOM-kills the process; found 2026-08-27 while building the 250k bench
metadata:
  type: project
---

`geom::skeleton::LAV::iter_keys` (geom/src/skeleton.rs:721) collects a circular
linked list with `std::iter::successors`, terminating only when the walk returns
to `head`. If the list is corrupted so the cycle misses `head`, the iterator never
ends and `collect()` grows a `Vec<VertexID>` until allocation fails. Observed
request: exactly 2^33 bytes (2^30 elements x 8 bytes).

Reached from production code, not a test path:
`WorldCommand::MapBuildSpecialBuilding` -> `Map::build_special_building` ->
`Building::make` -> `map::procgen::building::gen_exterior_house` -> `skeleton`.

**Trigger is footprint size.** `Building::make` derives `size` from the OBB, and
`gen_exterior_house` builds a polygon at width 15-20 / height 20-28 then scales it
by `size / 40.0`. A house OBB of 8 m scales by 0.2 and produces near-degenerate
edges; 20 m (scale 0.5) did not reproduce it in 250k placements. The polygon seed
is `rand2(center.x, center.y)`, so a bad case is deterministic in map position.

**Why:** found while placing 250000 houses for the sov-1ae benchmark. At 8 m
footprints the runner was OOM-killed at ~40000 houses after reaching 17.6 GB RSS.
The `'retry` loop in `gen_exterior_house` cannot help because the process dies
inside `skeleton` before the retry.

**How to apply:** never conclude "the sim cannot reach 250k" from an OOM without
checking footprint size first. When running anything that places many buildings,
cap memory with `ulimit -v` — an uncapped run takes the whole machine down and
other agents share it. If a future story lets the player place small buildings,
this is a crash-on-place bug and needs a bound in `iter_keys`.

Related: [[bench-contract-250k]], [[baselines-250k]]
