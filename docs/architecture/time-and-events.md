# Time and events

**Kind:** architecture
**Authority:** advisory
**Status:** draft
**Owner:** architecture
**Last verified:** 2026-08-28

## The law

> **Stable state sleeps. Changed state propagates.**

250,000 citizens with per-frame decision code is an architectural failure, not a threading
problem. Slow social and economic state updates on scheduled events or change-driven invalidation;
any full-population per-tick scan must justify itself ([scheduling standard](../engineering/performance.md)).

## Current substrate

Every registered system runs every tick at 50 Hz (`SeqSchedule`, `simulation/src/utils/scheduler.rs`).
The only frequency variation is `HumanDecision.wait`, a per-citizen random 30–80 tick cooldown
before the next decision (`souls/human.rs`). There is no event calendar, no timing wheel, no
cadence per system.

## Target design

**Lifecycle of a slow actor:**

```text
Sleep → Wake (scheduled time or event) → Decide → Emit intent → Commit → Schedule next wake → Sleep
```

Explicit state-machine enums, never suspended async futures: enums serialise, hash, replay,
inspect and version; futures do none of those.

**Deterministic event calendar.** A custom timing wheel or monotone queue keyed by simulation
time with a stable tie-break `(time, domain, entity)`; serialisable; bulk scheduling; no
dependence on thread wake order. `hierarchical_hash_wheel_timer` exists on crates.io (C1-09); a
hand-rolled wheel over a `BTreeMap<(SimTime, Domain, EntityKey), Event>` is the cheapest first
version.

**Cadence bands.** Different domains at natural rates:

```text
visible movement          every tick
traffic aggregation       medium/high
logistics dispatch        seconds to minutes
production                minutes
utilities                 seconds to minutes, by domain
household reviews         event-driven / daily
education, demography     daily / monthly / yearly
plan reporting            period boundaries
```

Cheapest mechanism: a `cadence: u32` on system registration; `SeqSchedule` skips a system when
`tick % cadence != 0`; default 1 for everything (no behaviour change); then move
`electricity_flow_system` to 5 and validate. Replay hashes change at every cadence boundary —
replays become cadence-version-dependent.

**Wake and stale memory.** A sleeping citizen's beliefs (which shop had bread) go stale. On wake
the citizen catches up from the [change journal](change-journal.md), not by scanning the world.
This is the resolution of Lane G's "stable things sleep vs citizens act on remembered
information" tension, and it makes the journal a prerequisite for sleeping citizens.

**Household and citizen schedules** are the domain form: a day is a few timestamps (depart,
arrive, shift end, shop, return, sleep); household reviews fire on pantry thresholds, births,
displacement, policy changes ([time](../simulation/society/time.md)).

## Migration

1. Cadence field with default 1.
2. Event calendar resource with one event kind (a household pantry review, once households exist).
3. Move one system to a lower cadence; measure.
4. Citizens as scheduled actors — after the record/body split ([entity identity](entity-identity.md)).

## Open decisions

- The steady-state active fraction at 250k — the one number that decides feasibility (Lane G
  open question 5). Nothing in any source states it.
- Whether replay compatibility across cadence changes matters before 1.0.

## Related

- [Simulation phases](simulation-phases.md)
- [Change journal](change-journal.md)
- [Performance](performance.md)
- [Adaptation (concept)](../simulation/concepts/adaptation.md)
- [Citizen architecture proposal](../plan/proposals/citizen-architecture.md)
