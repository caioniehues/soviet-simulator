# Simulation clock — Bevy mapping

**Status:** ratified (ticket #5, 2026-08-16). Grounds: the Unity-track draft
(`~/Projects/soviet/architecture/simulation-clock.md`, CS1/W&R evidence cited there) and the
quarter-million spike (issue #4, branch `spike/250k`).

## Sim loop topology

The simulation ticks in a **custom `SimTick` schedule in the main world**, run by a driver system
from `Update`: the driver executes `SimTick` *N* times per render frame, where *N* is the speed
multiplier (1/2/4) and 0 when paused. Not Bevy's stock `FixedUpdate` — it cannot express
"speed is a substep multiplier, pause still ticks housekeeping" without fighting `Time<Fixed>`.
Not a dedicated sim thread — the spike prices the whole citizen tick at ~0.2 ms, so blocking the
frame is a non-issue for several milestones. The dedicated-thread topology (CS1's) remains the
documented upgrade path; it stays open because the sim never reads render state (see
[ecs.md](ecs.md) § Presentation).

## Counters and constants

Two wrap-safe `u32` counters, both serialized:

- `FrameIndex` — advances once per `SimTick` run; the authority every frame-band phases on.
- `TickIndex` — advances once per real driver pass regardless of speed; drives the
  speed-independent Housekeeping band (fires even while paused).

Pinned starting constants, revisable, all in one module (`sim::clock`):

| constant | value |
|---|---|
| sim rate at speed 1 | 60 ticks/s |
| `FRAMES_PER_GAME_DAY` | 600 |
| speed multipliers | 1 / 2 / 4 substeps |
| per-frame sim budget | 8 ms (band budgets are denominated in it) |

Pause = 0 substeps: no `FrameIndex` advance, no calendar advance; `TickIndex` keeps advancing.
Speed never rescales time — `SecondsPerFrame` is constant, so every spec rate is speed-invariant.

## The six bands

| Band | Period (frames) | Subsystems |
|---|---|---|
| **High** | 1 | vehicle/train movement, lane interactions, collisions, weather integration |
| **Medium** | 16 | pedestrian movement, lead-vehicle AI, loading/unloading, transit ops |
| **Low** | 256 | production step, utility-solver slice, dispatch scan, building maintenance |
| **Very low** | 4096 | citizen needs, work/school binding, demographics, migration |
| **Calendar** | day/season/year edges | plan accounting, era factors, heating season, plan boundaries |
| **Housekeeping** | 1024 ticks (speed-independent) | autosave, notification/achievement scans |

Calendar (day/season/year) is derived from `FrameIndex`, never stored per-entity. The visual
day-night cycle, if any, is presentation-only and decoupled from the calendar.

## Band registry: cadence is data, ordering is schedule structure

The carried draft said "one schedule per band"; ratification reshaped that, because the nine-stage
ordering in [ecs.md](ecs.md) is a per-tick pipeline that cuts across bands. The reconciliation:

- **Ordering** lives in `SimTick` as nine `SystemSet`s (the stage seams), with explicit
  `apply_deferred` barriers at the named seams — never auto sync points.
- **Cadence** lives in a `BandRegistry` *data* plugin. It owns the phase counters, the per-phase
  entity buckets, and churn-stable bucket assignment. Systems consume it through a
  `BandSweep<P>` system-param: "give me this frame's slice of my band."

So the registry still owns the modulo (no CS1-style scattered masks), and happens-before edges
stay visible schedule structure.

### Phase-bucketed sweeps (spike-forced)

The spike killed modulo-scan sweeps: in Bevy, "process 61 entities" costs 0.25 ms naive because
query iteration over the full 250k population dominates; pre-bucketed `Vec<Entity>` lists with
random-access `get_mut` cut sparse bands 15–250× (0.21 ms/frame total for all six bands).

- Buckets are keyed by a **stable hash of serialized identity plus system salt** — never by
  `Entity` index or archetype position, so compaction and archetype moves cannot double-process
  or skip.
- Bucket membership is maintained by component lifecycle hooks on the band-membership components;
  the registry enforces the no-double-processing rule under spawn/despawn churn.
- Swept entities store a `last_processed_frame` stamp and integrate elapsed frames on their turn,
  so a sweep visit is self-contained.

## Warm-up (deferred)

The 16,384-frame headless settle pass is carried as a documented pattern but not built until a
system needs settling — earliest: production chains. Systems will opt out via a sentinel substep,
per the CS1 pattern.

## Determinism scope

Single-run stability only: stable bucket assignment + explicit system ordering. Cross-platform
bit-determinism (replay/multiplayer) is **deliberately deferred** — adopting it would immediately
forbid the `par_iter` accumulation and float math the spike relies on. See ADR 0006.
