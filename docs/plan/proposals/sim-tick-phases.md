# Proposal — labelled simulation phases, keyed randomness, and the road to deterministic parallelism

**Kind:** decision (draft)
**Authority:** advisory — binds nothing until accepted as a numbered decision
**Status:** proposed
**Owner:** project lead
**Date:** 2026-08-28
**Feeds:** the 250k target; the lockstep multiplayer crate; every structural proposal in the architecture handbook

## Context

The design thread presents a ten-phase deterministic order as an architecture conclusion. The
code runs eighteen systems in flat registration order with electricity first and map update second-last;
randomness is one global sequential stream; the determinism test proves serialisation round-trip
only; `ParCommandBuffer` is applied in insertion order; `networking/` assumes bit-identical frames.
Lane C2 established the dependency order: keyed randomness and typed contexts → phase labels →
parallelism, with a repeat-run digest test before any reorder.

## Decision proposed

1. **Keyed randomness first.** `keyed_rand(seed, domain, entity, ordinal)` in `simulation/src/utils/`;
   convert call sites; regenerate the replay baseline once, deliberately.
2. **Repeat-run determinism test** (two fresh simulations, same commands, equal digests) and a
   portable digest beside `hashes()`.
3. **Label phases without reordering.** Phase markers in `SeqSchedule`; the eighteen systems grouped
   under COMMAND · TOPOLOGY · ALLOCATION · DECISION · ROUTING · MOVEMENT · ARRIVAL · PRODUCTION ·
   UTILITIES · ACCOUNTING · REPORTING in their *current* order; per-phase digests.
4. **Reorder only by decision**, one move at a time, with the replay version bumped.
5. **Parallelism only after 1–4 and typed contexts**, as intents sorted on a source key before a
   serial commit — never `DashMap`, lock order or Rayon scheduling.

## Alternatives

- Adopt the thread's order directly. Rejected: it moves electricity from first to ninth and map
update from second-last to second; behavioural change with no test able to localise a regression.
- Skip keyed randomness and parallelise. Rejected: non-deterministic by construction.

## Consequences

Replays become version-dependent; a replay-compatibility policy is needed. Multiplayer either
constrains every step to bit-identity or is dropped — that decision is separate and open.

## Validation

Round-trip test unchanged after 1 and 3; the new repeat-run test green; per-phase digests equal
across two runs; multiplayer frame assertions hold in a two-client headless run.

## Open for the Planner

Keep lockstep multiplayer? Cross-platform determinism (`libm`, fixed-point) a 1.0 goal? Replay
compatibility across versions?

## Related

- [Simulation phases](../../architecture/simulation-phases.md) · [Randomness](../../architecture/randomness.md) · [Determinism](../../architecture/determinism.md) · [Parallelism](../../architecture/parallelism.md) · [Lane C2 §3](../../research/conversation-mining-2026-08-28/C2-architecture-vs-code.md)
