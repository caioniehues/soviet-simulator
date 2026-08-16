# Single-world presentation; the invariant is direction, not a mirror layer

The carried architecture mirrors active pawns into separate presentation entities. In a
single-world Bevy app that bookkeeping buys nothing yet: pawns carry `Transform` directly and a
`PostSimEasing` set in `Update` eases rendered transforms toward authoritative positions. The
load-bearing rule is directional — presentation reads sim state and never writes it — enforced by
set ordering. The mirror layer stays documented as the upgrade path and becomes mandatory only if
the sim moves to its own world/thread (ADR 0001's open door).
