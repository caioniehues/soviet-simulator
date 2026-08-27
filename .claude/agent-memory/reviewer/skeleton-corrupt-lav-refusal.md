---
name: skeleton-corrupt-lav-refusal
description: geom::skeleton returns Option since sov-bo3; LAV::iter_keys bound is vs.len()+1 and the +1 is load-bearing; one production caller only
metadata:
  type: project
---

`geom::skeleton::skeleton()` returns `Option<Vec<Subtree>>` since sov-bo3 (branch
`fix/sov-bo3-lav-unbounded`, commits 4d1d18b + ca3ef2e). `None` means the algorithm
proved its own LAV linked lists inconsistent; it is NOT "empty skeleton".

**Why:** `LAV::iter_keys` walked a circular list with `std::iter::successors`,
stopping only at `next == head`. A cycle not through `head` grew the `Vec` to 17.6 GB
and OOM-killed the game — a "never game over" pillar violation reachable from an
ordinary player building placement.

**How to apply:**
- The bound is `vs.len() + 1`, deliberately NOT `self.len` (self.len is maintained by
  the same bookkeeping under suspicion). A LAV is a ring of distinct arena vertices,
  so >vs.len() keys is a *proof* of a repeat, not a heuristic cap.
- **The `+1` is load-bearing and proven so by mutation**: setting `limit = vs.len()`
  turns 5 geom tests red (`iter_keys_walks_a_full_arena_ring` plus all four
  pre-existing `skeleton::tests`), because the initial LAV of a polygon legitimately
  spans the whole arena.
- Detection is a `Cell<bool>` on `LAV`; `any_corrupt(&lavs)` is checked in `skeleton()`
  after `SLAV::new` seeding (skeleton.rs:910) and at the end of every event loop
  iteration (skeleton.rs:965). The `lavs: Vec<LAV>` arena is never removed from —
  `SLAV::lavs: Vec<LavID>` is the active list — so the flag is monotone and cannot be
  dropped. `LAV::from_chain` flags a *local* `l` that is then moved into `lavs.push`.
- **There is exactly ONE production caller repo-wide**: `simulation/src/map/procgen/
  building.rs:94`, `skeleton(...)?` inside `catch_unwind` -> `.ok().flatten()` ->
  `continue 'retry`. None and panic collapse to the same retry. Every `.unwrap()` on
  `skeleton()` is in `#[cfg(test)]`.
- The corrupting oracle input is `gen_exterior_house(8.0, 32362)` — I re-derived the
  12-vertex polygon independently and it matches the hardcoded literal in
  `geom/src/skeleton.rs::cycle_tests` bit-for-bit. Retry 1 for that seed succeeds.
- Follow-ups filed and NOT fixed: `sov-cnq` (the memory guard runs under nothing
  automated), `sov-crl` (`building.rs:32 'retry: loop` is unbounded), and root-cause
  option (b) — preventing the corrupt cycle in `handle_split_event`'s x/y selection.

Related: [[review-method-patterns]], [[determinism-test-cannot-fail]]
