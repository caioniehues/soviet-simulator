# EPIC-015 — Stall handling as a planning signal

**Summary:** Stall handling as a planning signal
**Stories:** STORY-0070, STORY-0071
**Primary sources:** `spec/traffic.md`
**Status:** 0/2 done

## STORY-0070

**Epic:** EPIC-015 — Stall handling as a planning signal
**Title:** Never delete a vehicle for being gridlocked

**As a** planner watching the plan execute
**I want** a vehicle stuck in traffic to remain in the simulation indefinitely rather than being despawned
**So that** a blocked delivery is a visible plan failure to fix, never a silently erased trip

**Acceptance criteria:**
- AC-1: A vehicle blocked in traffic is never removed from the simulation purely for being blocked; there is no despawn-on-gridlock timer analogous to CS1's 100-150 frame deletion. [SUBSTRATE: PROVIDED — transportation/vehicle.rs:19-20] · impact:`journey` · seam:`integration` · scenario:`SCENARIO-0038`
- AC-2: After roughly 200 seconds blocked, the vehicle transitions to a Panicking state and continues seeking to resume rather than being removed. [SUBSTRATE: PROVIDED — transportation/vehicle.rs:19-20, VehicleState::Panicking] · impact:`local` · seam:`integration` · scenario:`SCENARIO-0038`

**Sources:**
- `spec/traffic.md:26-34`

**Status:** pending

## STORY-0071

**Epic:** EPIC-015 — Stall handling as a planning signal
**Title:** Escalate a stalled vehicle to re-route, then to a planner-visible bottleneck event

**As a** logistics system managing a delivery fleet
**I want** a vehicle stalled past a threshold to first attempt a re-route if an alternative path exists, and otherwise register a stall event the planner can see
**So that** gridlock becomes an actionable bottleneck signal feeding infrastructure decisions and delivery-delay consequences, instead of a truck that silently sits forever or silently disappears

**Acceptance criteria:**
- AC-1: No hook currently exists connecting the vehicle's blocked/Panicking state to a re-route request; the vehicle resumes on its existing route rather than evaluating an alternative. [SUBSTRATE: ABSENT — transportation/vehicle.rs:19-20 has the Panicking state but no re-route call; audit §7 names this the missing piece] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0039`
- AC-2: When a stalled vehicle has no viable alternative route, a bottleneck/stall event is emitted that a planner-facing system can observe, feeding the corridor-utilisation readout and downstream delivery-delay consequences. [SUBSTRATE: ABSENT — greenfield] · impact:`cross-surface` · seam:`app-level` · scenario:`SCENARIO-0039`
- AC-3: The stall response follows a strict order — the vehicle first waits in place (the jam persists physically), only after the stall threshold does it attempt a re-route, and only if no alternative exists does it register a stall event; a vehicle never skips straight to re-routing or stall-registration before waiting out the threshold. [SUBSTRATE: UNAUDITED — spec/traffic.md:30-32] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0039`

**Sources:**
- `spec/traffic.md:26-34`

**Status:** pending