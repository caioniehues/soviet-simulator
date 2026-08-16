# Band cadence is data (BandRegistry + phase buckets); ordering is schedule structure

Two carried invariants collided: "the scheduler owns the modulo" (no CS1-style scattered cadence
masks) and the nine-stage per-tick pipeline, which cuts across bands — so bands cannot be
schedules. Resolution: one `SimTick` schedule with nine `SystemSet`s owns ordering (explicit
`apply_deferred` barriers at the named seams, never auto sync points); a `BandRegistry` data
plugin owns phase counters and per-phase entity buckets, consumed via a `BandSweep<P>`
system-param.

Buckets are mandatory, not an optimisation: the spike showed naive modulo-scan sweeps pay full
query iteration over 250k entities to process 61 (0.25 ms), while pre-bucketed `Vec<Entity>` +
random-access `get_mut` cuts sparse bands 15–250×. Bucket keys are a stable hash of serialized
identity + system salt — never `Entity` index — so compaction/archetype moves cannot
double-process or skip a sweep.
