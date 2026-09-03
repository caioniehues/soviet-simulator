# Determinism

**Kind:** architecture
**Authority:** advisory
**Status:** draft
**Owner:** architecture
**Last verified:** 2026-08-28

## Current substrate

- `Simulation::hashes()` (`lib.rs`) bincode-encodes the world and each registered resource and
  hashes each with `common::hash_u64` — an `FxHasher`, which is fast but not stable across
  platforms or `rustc-hash` versions.
- `TestCtx::check_determinism` (`tests/mod.rs`) encodes → decodes → compares hashes. It proves
  **serialisation round-trip stability**, not repeat-run determinism. It runs every tick in
  `TestCtx::tick` and every 25 ticks in `advance_ticks`.
- `test_world_survives_serde` (`tests/test_iso.rs`) replays `world_replay.json` twice and
  compares; it bisects a divergence to a tick (`check_size`, `check_start`) but cannot say which
  system caused it. Any reorder or RNG change regenerates the baseline, so the test then proves
  only "stable for this version".
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
7. **Repeat-run test** — two fresh simulations, same seed, same commands, identical digests every N ticks. Absent today; the cheapest high-value test addition in the project.
8. **Divergence bisection** — with tick and phase digests: binary-search the first divergent checkpoint, identify the phase, inspect the transition journal around it.

**Floats.** Integer or fixed-point for conserved and accounting quantities (`Money` already is).
Floats are acceptable for bounded physical and presentation values when order is controlled,
drift cannot violate conservation, and outcomes are repeat-tested. If cross-platform determinism
is a goal, `libm` replaces the intrinsics in authoritative math (~50–100 substitutions in `geom/`,
Lane C1 §3.4; software `sqrt` is ~5× slower — use it for authoritative state only).

## Migration

1. Add the repeat-run test (no production change).
2. Add a portable digest alongside `hashes()`; compare both for one release.
3. Per-phase digests once phases are labelled.
4. `libm` in `geom/` authoritative paths, gated by the cross-platform decision.

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
