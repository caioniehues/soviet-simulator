# Determinism standard

**Kind:** standard
**Authority:** operational
**Status:** active
**Owner:** project lead
**Last verified:** 2026-08-28

The simulation must produce the same authoritative state from the same initial state and the same
commands, on one machine, every time. Cross-platform determinism is an open decision; the rules
below keep it reachable.

## Rules

1. **Must — stable iteration.** No authoritative decision iterates a hash map or hash set. Use
   `BTreeMap`, sorted vectors or dense indexes.
2. **Must — stable tie-break.** Equal cost, equal priority, equal time: resolve on immutable
   identity or declared order. Never on address, insertion order of a `Mutex<Vec>`, or thread arrival.
3. **Must — idempotent transitions.** Anything replayable carries an immutable ID; applying it
   twice is a no-op ([simulation transitions](simulation-transitions.md)).
4. **Must — no wall clock, no OS randomness** in the simulation crate.
5. **Should** *(target)* — **keyed randomness.** New random outcomes derive from
   `(master_seed, domain, entity, ordinal)` rather than the sequential `RandProvider` stream;
   never let unrelated iteration order change an outcome ([randomness](../architecture/randomness.md)).
6. **Must** *(if parallel work is introduced)* — **deterministic merge.** Parallel workers emit
   intents with a source key; intents are sorted on that key before commit. Correctness never
   depends on Rayon scheduling, lock order or `DashMap`.
7. **Should — canonical digest.** Authoritative state is digestible per tick with a portable hash;
   `FxHasher` is for hash maps, not digests ([determinism](../architecture/determinism.md)).
8. **Must — replayable.** Every input enters through `WorldCommand` so a replay reproduces the run.
9. **Should — repeat-run test.** A change to scheduling, randomness or entity ordering is
   accompanied by a two-run digest comparison, not only the round-trip test.

## Randomness

`common::rand` (stateless hash) is the primitive for keyed randomness; `RandProvider` (sequential
Xorshift128) is legacy. New code should key, not draw.

## Floats

Conserved and accounting quantities are integer or fixed-point. Floats are acceptable for bounded
physical and presentation values when order is controlled, drift cannot violate conservation, and
outcomes are repeat-tested. If cross-platform replay is adopted, transcendental functions in
authoritative paths use `libm`.

## Tooling expected

Tick and per-phase digests; a bisection that finds the first divergent checkpoint and phase; the
transition journal around it ([debugging determinism](../developer/debugging-determinism.md)).

## What exists today

Round-trip hash comparison every tick in tests; replay-based `test_world_survives_serde`; no
repeat-run test; `FxHasher` digests; sequential RNG. See
[current substrate](../architecture/current-substrate.md#persistence-and-determinism).

## Related

- [Determinism (architecture)](../architecture/determinism.md)
- [Parallelism (architecture)](../architecture/parallelism.md)
- [Testing standard](testing.md)
