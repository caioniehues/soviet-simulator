# The edit appliers run in one declared total order

Six appliers shared `SimStage::ApplyCommands` with exactly one ordering edge between them
(zoning before buildings), so whether a wire placed this tick could snap to a building
placed this tick was an emergent property of registration order rather than a decision.
`CustomsSimPlugin` registered two systems with no ordering at all.

The order is now declared once where the stage set is configured, following the direction
the dependencies actually run: **zones → roads → buildings → wires → households → vehicles
→ transit**. A building may frontage-snap to a road placed the same tick, a wire snaps to a
building, a vehicle is bought at a depot, a household spawns into a dwelling, a transit
line needs its stops.

The usual objection to a total order is that it forecloses parallelism, which is worth
nothing here — these six systems drain queues holding at most one frame of player input.
What it buys is that same-tick behaviour is one fact readable in one place. The alternative
of declaring edges only where a dependency exists today was rejected because it leaves the
next contributor unable to distinguish "no edge because independent" from "no edge because
nobody considered it", which is the state this decision replaced.
