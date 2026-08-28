---
name: phantom-collider-congestion
description: Why an orphaned TransportGrid entry is an unrecoverable gridlock, not a slowdown — the calc_front_dist filters, and why it breaks BPR/EMA congestion pricing
metadata:
  type: project
---

Verified at source 2026-08-28, `simulation/src/transportation/road.rs` and
`transportation/mod.rs:87`. This is the model consequence of sov-6qx.

## A phantom never decays

`transport_grid_synchronize` (`transportation/mod.rs:87`) iterates
`world.query_trans_speed_coll_vehicle()` — entities that *hold* a `Transporter`. An
orphaned handle is held by no entity, so its position, `dir`, `speed` and `flag` are never
updated again. It sits at its insertion point with `speed = 0.0`, `flag = 0`,
`group = Vehicles`, forever. Only `Transporter::destroy` removes a grid entry, and nothing
holds the handle to call it.

## calc_front_dist does NOT filter it out

Two branches, and the speed filter is in the wrong one:

- **Ray-crossing branch**: has `if nei_physics_obj.speed <= 0.01 { continue; }` — phantoms
  are excluded here.
- **Front-cone branch**: no speed filter. A phantom passes when
  `cos_direction_angle = phantom.dir · my_dir > 0.0`, which holds for anyone driving the
  same way down that lane. It clamps `min_front_dist` and the approaching vehicle stops.

The "ignore myself" test is **positional**, not by identity
(`towards_vec.is_close(Vec2::ZERO, 1.0) && towards_vec.x > 0.0`). So the phantom is
self-ignored at the instant of creation and only becomes an obstacle once the real vehicle
drives away.

## The gridlock breaker cannot fire against a phantom

`calc_decision` escalates to `Panicking` only when `me_u64 == flag`, where `flag` is the
blocking object's flag. The phantom's flag is frozen at `0`. The blocked vehicle sets
`vehicle.flag = me_u64` (its own), reads `0` again next tick, and never matches. It holds
speed 0 with a `wait_time` jitter indefinitely. **A phantom therefore produces a permanent,
non-escalating stall — never a `Panicking` recovery, never a planner-visible "gridlock!"
log.** That is a hard *never game over* violation, and the reason the sov-6qx fix is a
model-level correction and not code hygiene.

## Why it also breaks the congestion models the roadmap commits to

`docs/plan/iterations/requirements/movement.md` commits to BPR volume-delay
(`t = t0 * (1 + alpha*(v/c)^beta)`, alpha 0.15, beta 4) with EMA-smoothed per-lane load. A
phantom is a permanent occupancy that no EMA can decay, so it becomes a static `v/c` offset
on that lane. With `beta = 4` the delay term explodes near capacity, so one phantom would
permanently and enormously inflate the cost of a road that is physically clear, pushing all
routing off it. Removing phantoms is a **precondition** for congestion pricing meaning
anything. Record this before any BPR work starts.

Secondary: each spurious unpark added one grid entry that is never removed, so the grid grew
without bound over a session and carried that growth through save/load.

See [[vehicle-substrate-unpark]] for the state machine itself.
