---
name: parcommandbuffer-exec-after-kill
description: ParCommandBuffer::apply runs exec closures for entities killed in the SAME drain — no liveness check; the "kills drain before execs" safety argument is FALSE
metadata:
  type: project
---

`ParCommandBuffer::apply` (simulation/src/utils/par_command_buffer.rs:50-83) drains
kills first (:62-68) and execs second (:80-82) — but the exec loop **discards the entity
id**: `for (_, exec) in exec_ent`. There is NO liveness check before `exec(sim)`.

So "kills are drained before execs, therefore a killed entity's queued closure can't run"
is **false**. Both queues are drained in the same `apply`; the kill removes the entity and
runs `sim_drop`, then the closure for that same dead id still executes.

Proved empirically 2026-08-27 with a throwaway test (inserted a CompanyEnt, queued
`exec_on` + `kill` for the same id, called `apply`):

```
TMPPROBE exec RAN for an entity killed in the same drain
TMPPROBE entity alive after apply: false
```

**Why it matters:** `CompanyEnt::sim_drop` (world.rs:193-204) calls `Market::remove`, which
erases the soul's `requested` row (market.rs:281). `recipe_act` (goods_company.rs:55) then
does `market.requested(soul, item.id).unwrap()`. Dead entity + queued exec = panic.

**How to apply:** never accept "the buffer drains kills first" as a safety argument for an
unwrap inside an `exec_on`/`exec_ent` closure. The guard that actually holds today is that
`goods_company.rs:193-196` returns early after `cbuf.kill(me)`, so the *same system* never
queues both for one entity in one tick. That is single-site mutual exclusion, not a
structural guarantee — a second kill path anywhere makes the unwrap live.

Related: [[market-remove-dispatch-drop]], [[dispatch-truck-park-seam]].
