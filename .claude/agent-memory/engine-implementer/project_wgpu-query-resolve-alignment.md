---
name: wgpu-query-resolve-alignment
description: wgpu 0.21 requires resolve_query_set destination_offset % 256 == 0, which makes positional offsets impossible for this project's 144-byte GPU-timing resolve buffer
metadata:
  type: project
---

`resolve_query_set`'s `destination_offset` must be a multiple of
`QUERY_RESOLVE_BUFFER_ALIGNMENT` = **256** (`wgpu-types-0.20.0/src/lib.rs:80`, enforced at
`wgpu-core-0.21.1/src/command/query.rs:423`). Violating it raises
`ResolveError::BufferOffsetAlignment`, which wgpu's default uncaptured-error handler turns into
a **panic** — a "never game over" violation.

`engine/src/gpu_timing.rs` has 9 passes x 2 slots x 8 bytes = **144 bytes** of resolve buffer,
smaller than one alignment unit. So **0 is the only destination offset that can ever be legal
there**. Any scheme that resolves partial runs to positional offsets is dead on arrival.

**Why:** a sov-abc fix shipped exactly that scheme in 45a87b1, passed a build and three green
unit tests, and only panicked on a scene where some pass did not run (e.g. `ssao: false`). The
demo's single hard-coded scene runs all 9 passes, so nothing caught it.

**How to apply:** to resolve partial query runs, resolve each run to offset 0 of a scratch
buffer and `copy_buffer_to_buffer` it to its real slot — copies only need
`COPY_BUFFER_ALIGNMENT` = 4, and commands in one encoder execute in order. Still never resolve
an unwritten slot: wgpu-core resets a query pool only for runs a command buffer actually used,
so reading an unreset query with `WAIT` is Vulkan UB.

**The test lesson, which is the real value here:** the three original unit tests *asserted* the
illegal offsets 16, 48 and 128. Green, pure-CPU, and they locked the defect in. A CPU test that
encodes an API precondition without ever checking it against that API is worse than no test.
The guard that works is a sweep over all 2^9 masks asserting `offset % 256 == 0`; it fails
against the bad code in milliseconds. See [[renderer-proof-obligations]].
