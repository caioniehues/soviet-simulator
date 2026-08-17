# Placement snaps to roads and angles, never to a global world grid

R1 "The Planner's Hands" makes placement snap, and the obvious move — a global N-metre
lattice that buildings and road nodes both quantise to — is rejected deliberately, so
nobody re-proposes it. R8 brings a heightfield terrain with road draping and grade gates;
a lattice that assumes a flat world is exactly the assumption R8 invalidates, and we would
be tearing out the snapping model two rungs after shipping it. The three layers we do
build all survive terrain unchanged: **frontage snap** (a building near a road orients to
the road tangent at a fixed setback, which is what produces legible street frontages),
**angle snap** (rotation and new road segments quantise to fixed increments, with a
modifier for free angles), and **node snap** (a segment endpoint within radius grabs an
existing road node). Node snap is load-bearing beyond feel: it keeps near-miss junctions
out of the road topology, so it is data integrity as much as UX. It already exists on the
commit path (`snapped_node`, `SNAP_RADIUS = 6.0`) and is invisible to the player, which is
the shape of most of this rung's work — moving decisions the sim already makes into the
frame before the click.

Frontage snap forces a schema change: `Building` carries only `id`, `kind` and `pos`, so
there is nowhere to store an orientation, and the save format bumps to v6 at R1 rather than
at R8 as originally sketched. Position-only alignment was considered and rejected — the
whole payoff is buildings facing the street, and without rotation a snapped row on a curve
reads worse than no snapping at all. The bump is cheapest now: one field while the save
holds five rungs of content, rather than the same field alongside R8's heightfield.
