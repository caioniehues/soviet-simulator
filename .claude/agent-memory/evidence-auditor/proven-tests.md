---
name: proven-tests
description: Tests in this repo I have personally mutation-proven — watched fail, then revert — with date and the mutation used
metadata:
  type: project
---

A test listed here does not need re-proving unless the test or the code it guards changes.

## 2026-08-27 — sov-bo3, branch `fix/sov-bo3-lav-unbounded`, commit `4d1d18b`

`geom/src/skeleton.rs` module `cycle_tests`:

- **`iter_keys_terminates_on_cycle_not_through_head`** — PROVEN, twice.
  - Mutation A: remove `.take(limit)` and the `keys.len() == limit` branch → test never
    returns; SIGKILL by a 1G cgroup ceiling (`journalctl --user`: "The kernel OOM killer
    killed some processes in this unit", "1G memory peak").
  - Mutation B (the ticket's named trap): keep `.take(limit)`, drop the corrupt branch so
    the truncated prefix is returned → `iter_keys walked 9 vertices over an 8-vertex arena`.
    This is what proves the `keys.len() <= vs.len()` assertion is *not* vacuous.
- **`iter_keys_walks_a_full_arena_ring`** — PROVEN. Mutation: `limit = vs.len()` (drop the
  `+ 1`) → this test fails *and* all four pre-existing `skeleton::tests` fail on `.unwrap()`.
- **`simulation ... placement_stress::gen_exterior_house_8m_100k_placements`** (`#[ignore]`) —
  PROVEN. Mutation A above → SIGKILL between seed 30000 and 40000 under a 2G ceiling.
  Unmutated: ok, RSS 7488 → 8572 kB, 1.38 s.

**Proven NOT guarded** (same commit): neutering `any_corrupt()` to `false` — which deletes the
entire "refuse rather than truncate" half — leaves `cargo test -p geom` at 23 passed and the
100k sweep green. See [[weak-evidence-shapes]].

Useful side-fact: the six `.unwrap()`s added to `skeleton::tests` are live guards, not
None-hiders. Mutation C makes every one of them fire.
