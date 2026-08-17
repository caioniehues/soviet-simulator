# No generic edit queue; node snapping is shared

Six domains carry the identical shape — `XEditQueue(Vec<XEdit>)`, `XIds { next: u64 }`, and
an `apply_x_edits` that drains, matches and increments. It reads as textbook duplication and
every architecture review will propose collapsing it into one generic queue. It is rejected,
recorded here so the proposal stops recurring.

The deletion test only half-passes. A generic queue and counter would concentrate the drain
loop and the id allocation — perhaps thirty lines across six domains — while each `apply`
body stays genuinely different work: placing a building runs a siting verdict, placing a
road pays gravel from nearby yards and recompiles ribbon geometry, creating a transit line
validates docked stops. Generalising the frame while the bodies remain divergent buys a type
parameter and almost no locality, and it puts a layer between player input and the code that
actually decides what an edit means.

The narrower duplication *is* worth taking. `roads.rs` and `wires.rs` each hand-roll
snap-to-existing-node-or-create and remove-nearest — a road snapping a segment endpoint to a
node and a wire snapping a span endpoint to a pole are the same operation on two different
graphs. Two adapters of one operation is a real seam, unlike the queue frame's six adapters
of six different operations. R1 makes node snapping visible in the preview, and doing that
against two independent implementations is how they drift apart.
