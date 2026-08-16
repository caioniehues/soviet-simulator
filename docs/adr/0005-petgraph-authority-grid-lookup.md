# petgraph is the editable topology authority; spatial lookup is a hand-rolled uniform grid

Typed network topology (road/rail/transit/utility graphs) lives in `petgraph`: flexible and right
for the *editable authority* side, wrong for hot search — which is exactly the authority/mirror
split the carried pathfinding doc prescribes, so the crate slots into an existing seam. The
packed search mirror is built later as a derived, version-stamped structure. Spatial lookup uses
hand-rolled uniform grids (static and moving membership separated, rebuildable from authoritative
state); `kiddo` is in Cargo.toml but held in reserve until a benchmark shows the grid losing —
bounded local queries over mostly-static members favour a grid over a k-d tree.
