# Randomness

**Kind:** architecture
**Authority:** advisory
**Status:** draft
**Owner:** architecture
**Last verified:** 2026-08-28

## Current substrate

Two sources:

1. `RandProvider` (`simulation/src/utils/rand_provider.rs`) — a single Xorshift128 registered as a
   serialised resource, seeded from `RNG_SEED`, drawn sequentially: `spawn_human` draws three
   times (name, shirt colour, pedestrian), `spawn_parked_vehicle` draws, and so on. Deterministic
   for a fixed call order; **any change in entity creation order reshuffles every later draw.**
2. `common::rand` — a stateless Bob-Jenkins hash: `rand(x)`, `rand2(x, y)`, `randu(u)`. Used for
   positional randomness (`update_decision` uses `rand2(pos.x, pos.y)`; pathfinding noise uses
   `randu(dist_bits ^ base_random)`). This *is* keyed randomness, keyed ad hoc by position.

## Target design

Derive every authoritative random value from a stable key:

```text
master_seed · domain · entity_id · event_ordinal
```

A pure function — no mutable stream — so each entity's randomness is independent of insertion
order and of thread scheduling. This is the **first prerequisite** of every parallelism and
phase-reorder proposal (Lane C2: MUST-DO-FIRST). It needs no new dependency; the existing hash
functions in `common::rand` are the primitive.

## Migration (Lane C2 §3.2)

1. Add `fn keyed_rand(seed: u64, domain: u32, entity: u64, ordinal: u32) -> u64` in
   `simulation/src/utils/`.
2. Convert one call site (`spawn_human`); the round-trip test must still pass; the replay baseline
   changes once and is regenerated deliberately.
3. Convert the remaining ~50 call sites; retire the sequential draws; keep `RandProvider` only if
   something genuinely needs a stream (test traffic).

## Open decisions

None architectural; the domain enumeration is a naming exercise.

## Related

- [Determinism](determinism.md)
- [Parallelism](parallelism.md)
- [Randomness standard](../engineering/determinism.md#randomness)
