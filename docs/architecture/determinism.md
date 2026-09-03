# Determinism

**Kind:** architecture
**Authority:** advisory
**Status:** draft
**Owner:** architecture
**Last verified:** 2026-09-03

## Current substrate

- `Simulation::hashes()` (`simulation/src/lib.rs:311`) bincode-encodes the ECS `World` and each
  registered resource and hashes each with `common::hash_u64` — an `FxHasher`, which is fast but
  not stable across platforms or `rustc-hash` versions.
- `TestCtx::check_determinism` (`simulation/src/tests/mod.rs`) encodes → decodes → compares hashes.
  It proves **serialisation round-trip stability**, not repeat-run determinism. It runs every tick
  in `TestCtx::tick` and every 25 ticks in `advance_ticks`.
- `test_world_survives_serde` (`simulation/src/tests/test_iso.rs:239`) **is** a repeat-run gate
  (fixed by closed `sov-n8v` / `sov-y66`). It builds two `Simulation`s from the same 67-command
  replay (`world_replay.json`, `last_tick_recorded` 200000), ticks both with the real
  `Simulation::schedule()` — all eighteen systems, never an empty schedule — and at each compared
  tick checks three branches: `sim` vs `sim2` (replay-path divergence), bincode-decoded `deser`
  vs `sim`, and `deser` vs `sim2` (serialisation/equality mismatches). `is_equal`
  (`simulation/src/lib.rs:239`) compares every registered resource **and** the bincode-encoded
  ECS `World`; `transport_grid` cells live in a hash map, so that one resource is compared with
  the order-insensitive `transport_grid_equal` (`simulation/src/transportation/mod.rs:84`)
  instead of byte equality. A mismatch narrows the checkpoint window (`check_size / 2` from
  `(tick - check_size).max(3)` — the `.max(3)` floor is load-bearing, ticks below it are never
  compared) and panics after narrowing, leaving `world`/`world2` dumps behind. Only ticks with
  `tick % check_size == 0` (from `check_size` 1024 downwards) are compared, so a divergence that
  appears and resolves between checkpoints, or in the unobserved tail, is not observed; the gate
  bisects to a tick but cannot say which system caused it. Any reorder or RNG change regenerates
  the baseline, so the test then proves only "stable for this version". Debug runtime is ~165 s.
- Guards around the gate: the fixture-world census (`sov_rvu_fixture_world_census` in
  `simulation/src/tests/determinism_gate.rs:207` — at least 20 humans, 10 vehicles, 10 companies
  and one non-rail road, so the gate cannot go green over a hollow city) and the
  emptied-tree-cell environment round-trip guard
  (`sov_rvu_environment_roundtrip_drops_emptied_tree_cells` in
  `simulation/src/tests/fixture_builder.rs:320`).
- Still-true qualifications: the fixture is built with `SimulationOptions::default()`, so the
  seed is the default 123 (`simulation/src/lib.rs:113`); `RandProvider` is a live sequential
  Xorshift resource (`simulation/src/utils/rand_provider.rs`), so every reorder also reshuffles
  the global draw stream; `common::rand::RandGen` is a stateful LCG (`common/src/rand.rs:82`),
  not a stateless hash (the stateless primitives are `rand4` / `randu` / `randhash`); "no wall
  clock" means no wall clock in simulation state — `Instant::now` is used for profiling scopes
  in `Simulation::tick` and serialisation logging (`simulation/src/lib.rs:284,397,438`) and never
  feeds state; resources outside `saveload_funcs()` are invisible to `is_equal` (an
  unregistered-resource blind spot, not a suspect list).
- Positions, speeds, costs are `f32`. `geom/` calls `sin`, `cos`, `sqrt`, `atan2` as platform
  intrinsics; there is no `libm`. IEEE 754 fixes results only for `+ − × ÷ sqrt`; transcendentals
  differ in the last bit across x86, ARM and wasm. `OrderedFloat` (a dependency) handles NaN
  ordering, not rounding. Cross-platform replay and multiplayer are therefore not deterministic
  today, despite the lockstep design.
- The RNG is one global sequential stream ([randomness](randomness.md)).

## Target design

Canonical, in this order:

1. **Stable iteration** — never iterate a hash map to make an authoritative choice.
2. **Stable tie-breaks** — every equal-priority choice resolves on immutable identity or declared order.
3. **Keyed RNG** — `(master_seed, domain, entity, ordinal)`.
4. **Idempotent transitions** — immutable IDs; the second application is a no-op ([simulation transitions](../engineering/simulation-transitions.md)).
5. **Deterministic parallel merge** — [parallelism](parallelism.md).
6. **Canonical digest** — a portable hash of authoritative state per tick and per phase. Candidates: `xxhash-rust` XXH3 (fast, portable) or `blake3` (portable, heavier); this is an open conflict. `FxHasher` stays for hash maps only.
7. **Repeat-run test** — two fresh simulations, same seed, same commands, identical digests every N ticks. Partly present: `test_world_survives_serde` replays the same command log through the real schedule in two simulations and compares resources plus the ECS World (see current substrate); still absent: a same-seed-from-scratch digest comparison and per-phase digests.
8. **Divergence bisection** — with tick and phase digests: binary-search the first divergent checkpoint, identify the phase, inspect the transition journal around it.

**Floats.** Integer or fixed-point for conserved and accounting quantities (`Money` already is).
Floats are acceptable for bounded physical and presentation values when order is controlled,
drift cannot violate conservation, and outcomes are repeat-tested. If cross-platform determinism
is a goal, `libm` replaces the intrinsics in authoritative math (~50–100 substitutions in `geom/`,
Lane C1 §3.4; software `sqrt` is ~5× slower — use it for authoritative state only).

## Migration

1. Compare a portable digest alongside `hashes()` for one release (the replay-based repeat-run gate itself already exists — see current substrate).
2. Per-phase digests once phases are labelled.
3. `libm` in `geom/` authoritative paths, gated by the cross-platform decision.

## Open decisions

- Is cross-platform deterministic replay a 1.0 requirement? (Decides `libm` and fixed-point scope.)
- XXH3 versus BLAKE3 for the digest.
- Replay compatibility across versions.

## Related

- [Randomness](randomness.md)
- [Parallelism](parallelism.md)
- [Determinism standard](../engineering/determinism.md)
- [Debugging determinism (guide)](../developer/debugging-determinism.md)
- [Lane C1 §2 C1-10, C1-14](../research/conversation-mining-2026-08-28/C1-rust-crates.md)
