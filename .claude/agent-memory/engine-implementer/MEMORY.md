# Memory Index

- [GPU timing: two rival implementations](project_gpu-timing-two-implementations.md) — neither is on main; check your branch before assuming gpu_timing.rs exists
- [wgpu query-resolve alignment](project_wgpu-query-resolve-alignment.md) — destination_offset must be a multiple of 256; for a 144-byte buffer only 0 is ever legal
- [wgpu copy safety is the tracker](project_wgpu-copy-safety-is-the-tracker.md) — in-order execution proves nothing; the barrier comes from wgpu-core's tracker
- [Renderer proof obligations](project_renderer-proof-obligations.md) — one hard-coded capture scene hides conditional-pass bugs; how to exercise an alternate scene
- [Bind operator-supplied proof](feedback_bind-operator-supplied-proof.md) — a new proof argument must be tied to the run in the same commit, or it is a new place to be wrong
