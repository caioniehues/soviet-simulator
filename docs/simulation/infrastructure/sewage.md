# Sewage

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** infrastructure
**Last verified:** 2026-09-03

| Scope | Post-1.0 — charter explicit cut 2026-09-03 ([ADR-0001](../../decisions/0001-households-and-utilities-are-1.0-scope.md)) |

## What this is

Sewage is a separate finite physical network for collecting, buffering, treating, and
discharging effluent. It flows by gravity. When the treatment plant is full or the pipe is
blocked, sewage backs up. That backpressure is visible to the Planner as a physical queue,
not a hidden flag.

Sewage is not electricity and not water. It has its own topology, its own buffers, and its
own failure mode. Pump stations couple it to electricity: no power, no pump, sewage backs up.

## Target requirement (Post-1.0)

The `SPEC-SEWAGE` ids below are Post-1.0 hooks that keep the network buildable later; they are
not 1.0 acceptance criteria.

`SPEC-SEWAGE-001` — Sewage SHALL be sole authority for sewage topology, endpoint attachment,
buffers, pipe/pump capacity, transfer progress, treatment result, and discharge record. It
MUST NOT reuse Electricity connectivity or Water quantity as sewage state.

`SPEC-SEWAGE-002` — per-tick movement is bounded by connected path and pipe/pump rate.
Treatment/discharge removes quantity only via named result.

`SPEC-SEWAGE-003` — full buffers, disconnected path, or blocked treatment retain observable
backlog. Backlog MUST NOT disappear or end the plan.

`SPEC-SEWAGE-004` — treatment receives named sewage and emits only a declared result.
A Water handoff uses one immutable `SewageWaterHandoffReceiptID`.

`SPEC-SEWAGE-006` — `Q = T + R + L` for each treatment/discharge. One
`SewageTreatmentDispositionID` applies once.

## Target design

Gravity DAG with buffers and backpressure (PLAUSIBLE, D §3.5):

- Directed acyclic graph from sources to treatment plant, following gravity
- Per-pipe capacity limit (flow rate)
- Buffer at each junction/treatment plant
- Backpressure when downstream buffer is full: upstream pipes back up
- Pump-power coupling: pumps require electricity

This is a finite-capacity DAG flow with buffers — much simpler than SWMM's PDE solver.
The inertia signature is minutes to hours: a backed-up sewer fills slowly, and the Planner
has time to respond (add capacity or fix the treatment plant).

## Current substrate

No sewage kind or registered system exists (`simulation/src/map/objects/building.rs:17-37`,
`simulation/src/init.rs:52-70`). Electricity's road cache cannot establish sewage. This is
entirely greenfield.

## Research basis

SWMM (EPA Storm Water Management Model) uses kinematic wave or full dynamic wave routing
for gravity-driven sewer networks. It models surcharging, reverse flow, and surface ponding.
For the game, a gravity DAG with finite-capacity buffers suffices (PLAUSIBLE, D §3.5).

W&R reference: sewage treatment plants, sewage pumps, sewage substations, and sewage
switches exist. Buildings connect via `$CONNECTION_SEWAGE_OUTPUT`. Sewage has a pollution
metric. Treatment produces water with a quality cap.

## Open questions

- Which endpoints generate sewage and by what Water-service relation?
- Which treatment/discharge endpoints does the Post-1.0 network need?

## Related

- [Water](water.md)
- [Electricity](electricity.md)
- [Network architecture](network-architecture.md)
- [Sewage spec](../../reference/specifications/sewage.md)
- [Phase lag](../concepts/phase-lag.md)
