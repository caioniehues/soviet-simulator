# Change journal

**Kind:** architecture
**Authority:** advisory
**Status:** draft
**Owner:** architecture
**Last verified:** 2026-08-28

## Current substrate

No change events exist. `EcoStats` (`economy/ecostats.rs`) keeps ring-buffered trade-volume
histories at four frequency bands — history, not events. `simulation/src/rerun.rs` is 48 lines of
commented-out Rerun integration from upstream; the dependency is commented out in `Cargo.toml`
(Rerun is active at v0.36 and could serve as a journal visualiser if revived — Lane C1 §2 C1-26).
Every consumer that wants to know what changed rescans.

## Target design

Authoritative commits emit compact, typed changes:

```text
StockChanged · HaulStateChanged · HouseholdMoved · EmploymentChanged · QualificationChanged
NetworkTopologyChanged · PlanChanged · CollectiveIssueRaised
```

Consumers: indexes (bitsets, spatial), the [observatory](observatory.md), notifications,
[causal history](causality.md), [snapshots](snapshots.md), debugging, and **sleeping citizens
catching up on wake** ([time and events](time-and-events.md)). The world propagates what changed
instead of rescanning the civilisation.

The journal is transient (drained per tick), not serialised; what must persist becomes a causal
fact or an index.

## Migration (Lane C2 §3.2)

1. A `ChangeJournal` resource with `push(event)` and `drain()`.
2. One event kind — `TradeMatched { buyer, seller, kind, qty }` — emitted from `market_update`.
3. A first consumer: the material-balance aggregate in the observatory.
4. Grow event kinds as systems are touched; never retrofit all at once.

Read-only addition; no simulation logic changes; one to two days for the skeleton.

## Open decisions

- Whether `tracing` spans (already a transitive dependency) can carry debug-only journal output;
  the causal DAG itself needs purpose-built structures either way.
- Revive Rerun as the visualiser, or rely on Tracy and logs.

## Related

- [Observatory](observatory.md)
- [Causality](causality.md)
- [Snapshots](snapshots.md)
- [Time and events](time-and-events.md)
- [Observability standard](../engineering/observability.md)
