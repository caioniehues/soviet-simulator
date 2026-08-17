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
out of the petgraph topology of ADR 0005, so it is data integrity as much as UX.
