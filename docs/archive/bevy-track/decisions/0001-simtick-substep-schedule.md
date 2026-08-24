# Sim loop is a custom SimTick schedule, not FixedUpdate, not a sim thread

The simulation needs CS1's speed model — speed is a substep multiplier (1/2/4 SimTick runs per
render frame), pause is 0 substeps with housekeeping still ticking — which Bevy's stock
`FixedUpdate`/`Time<Fixed>` cannot express without being fought. A dedicated sim thread (CS1's
actual topology) was rejected for now: the quarter-million spike prices the full citizen tick at
~0.2 ms, so blocking the render frame is a non-issue for several milestones. The thread topology
remains the documented upgrade path and stays open because the sim never reads render state.
