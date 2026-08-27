---
name: skeleton-lav-traps
description: Traps in geom/src/skeleton.rs LAV walking, plus the seed-32362 corrupting polygon and how to re-find such inputs
metadata:
  type: project
---

`geom/src/skeleton.rs` LAV traps, and how to reproduce a corrupt LAV.

- Bound a LAV walk by `vs.len() + 1`, **never** by `LAV::len`. `len` is maintained by the
  same bookkeeping (`unify`, `from_chain`) that is under suspicion, so bounding by it is
  circular. `vs.len() + 1` is a proof of a repeated vertex, not a heuristic. The `+ 1`
  matters: the initial LAV of a polygon legitimately spans the whole arena.
- `skeleton()` returns `Option<Vec<Subtree>>` since sov-bo3. `None` means "the algorithm's
  own vertex lists went inconsistent" — REFUSE, never a truncated skeleton. `any_corrupt()`
  is what makes that decision; it is the thing test coverage keeps forgetting.
- Corruption is built by `handle_split_event`'s edge matching, which uses `approx_equal`
  at 0.1% relative tolerance and an absolute `EPSILON = 1e-5` on normalized cross products.
  An absolute epsilon makes small footprints fail more (`gen_exterior_house` scales by
  `size / 40.0`).

**Why:** sov-bo3 — an unbounded `iter_keys` walk reached 17.6 GB RSS and OOM-killed the
game from an ordinary building placement, breaking the "never game over" pillar.

**How to apply:** the known corrupting input is
`simulation::map::procgen::gen_exterior_house(8.0, 32362)` — the ONLY one in seeds
0..100000. Its 12-vertex polygon is hardcoded in
`cycle_tests::skeleton_refuses_a_polygon_that_corrupts_a_lav`. To find more, put a
temporary `eprintln!("{:?}", polygon)` at both `any_corrupt` return sites in `skeleton()`
and sweep `gen_exterior_house` over seeds from a temporary `#[ignore]`d test in
`simulation`; `f32` `Debug` output round-trips exactly, so the printed literal reproduces
the corruption bit-for-bit. Never round those literals. See
[[procgen-house-init-trap]] for the `crate::init::init()` requirement in such a probe.
