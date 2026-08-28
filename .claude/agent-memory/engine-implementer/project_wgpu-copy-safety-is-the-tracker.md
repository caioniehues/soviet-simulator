---
name: wgpu-copy-safety-is-the-tracker
description: In wgpu, what makes interleaved encoder commands safe is wgpu-core's resource tracker emitting barriers — not in-order execution; a common wrong reason for a right conclusion
metadata:
  type: project
---

**In-order command submission does not by itself prevent a read-after-write hazard in Vulkan.
A barrier does.** If you write a comment saying "commands in one encoder execute in order, so
X cannot clobber Y", it is wrong even when the code is correct.

What actually makes it safe in wgpu 0.21 is `wgpu-core`'s resource tracker, which emits a
barrier on each side of the operation. Verified verbatim at the pinned sources:

- `wgpu-core-0.21.1/src/command/query.rs:504` —
  `raw_encoder.transition_buffers(dst_barrier.into_iter());` before `copy_query_results`
- `wgpu-core-0.21.1/src/command/transfer.rs:735` —
  `cmd_buf_raw.transition_buffers(src_barrier.into_iter().chain(dst_barrier));` before
  `copy_buffer_to_buffer`

**Why this matters:** the barriers are derived from the *interleaving* the tracker observes.
Batching all the resolves ahead of all the copies, or lowering to raw `hal`, changes what the
tracker sees and drops the guarantee **with nothing failing** — no validation error, no panic,
just occasional wrong bytes.

**How to apply:** whenever justifying that two encoder commands are safe against each other,
cite the `transition_buffers` call in wgpu-core, not the ordering. And when reusing a scratch
buffer across loop iterations (see [[wgpu-query-resolve-alignment]]), keep the drain of
iteration N adjacent to the write of iteration N — and say in the comment *why* that adjacency
is load-bearing, or someone will "optimise" it away.
