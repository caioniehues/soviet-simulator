# The edit appliers run in one declared total order

**Status:** decided 2026-08-17, **built** 2026-08-19 — groundwork, [#118](https://github.com/caioniehues/soviet-simulator/issues/118).

Six appliers shared `SimStage::ApplyCommands` with exactly one ordering edge between them
(zoning before buildings), so whether a wire placed this tick could snap to a building
placed this tick was an emergent property of registration order rather than a decision.
`CustomsSimPlugin` registered two systems with no ordering at all.

The order is now declared once where the stage set is configured (`ApplierOrder`, `stages.rs`),
following the direction the dependencies actually run: **zones → roads → buildings → policy →
wires → households → vehicles → transit**. A building may frontage-snap to a road placed the
same tick, a storage-policy edit may adjust bands on a building placed the same tick (policy
sits after buildings for that reason), a wire snaps to a building, a vehicle is bought at a
depot, a household spawns into a dwelling, a transit line needs its stops.

`CustomsSimPlugin`'s `sell_exports` and `buy_imports` are chained too: sell runs first so this
tick's export proceeds are already in the treasury when imports spend it, a dependency that was
previously accidental (both hold conflicting `Inventory`/`Treasury` access, so were already
serialized by registration order without being declared). `release_border_arrivals` is
unrelated fleet bookkeeping and runs last in the chain so it can't be mistaken for a step in the
trade pair. Within `HouseholdSimPlugin`, `attach_flat_tables` now runs before
`apply_household_spawns` rather than as an unchained tuple.

The usual objection to a total order is that it forecloses parallelism, which is worth
nothing here — these six systems drain queues holding at most one frame of player input.
What it buys is that same-tick behaviour is one fact readable in one place. The alternative
of declaring edges only where a dependency exists today was rejected because it leaves the
next contributor unable to distinguish "no edge because independent" from "no edge because
nobody considered it", which is the state this decision replaced.
