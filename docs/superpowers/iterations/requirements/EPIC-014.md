# EPIC-014 — Emergent congestion (greenfield)

**Summary:** Emergent congestion (greenfield)
**Stories:** STORY-0065, STORY-0066, STORY-0067, STORY-0068, STORY-0069
**Primary sources:** `spec/pathfinding.md`, `spec/roads.md`, `spec/traffic.md`
**Status:** 0/5 done

## STORY-0065

**Epic:** EPIC-014 — Emergent congestion (greenfield)
**Title:** Track per-lane traffic load as a smoothed EMA counter

**As a** traffic system with no global solver
**I want** each lane to accumulate an exponential-moving-average load counter, updated O(1) per lane per tick with a time constant of a few in-game minutes
**So that** congestion state persists smoothly frame-to-frame and is cheap enough to run for every lane every tick

**Acceptance criteria:**
- AC-1: Every lane maintains an EMA load value updated in O(1) time per tick from vehicles currently occupying or passing through it; no per-segment traffic density signal exists in the current codebase to build on. [SUBSTRATE: ABSENT — map/pathfinding.rs:224-225 (only a per-trip random jitter exists, carrying no load information); audit §7] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0034`
- AC-2: The EMA counter is the single source of truth for lane load: both the routing cost term and any player-facing congestion readout read the same counter, with no second, independently-computed density tracker. [SUBSTRATE: ABSENT — greenfield; audit §7 explicitly warns against a duplicate-tracker bug class] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0034`

**Sources:**
- `spec/traffic.md:16-24`

**Status:** pending

## STORY-0066

**Epic:** EPIC-014 — Emergent congestion (greenfield)
**Title:** Price congestion into route cost with a BPR volume-delay function

**As a** pathfinder choosing between alternative lanes
**I want** route cost multiplied by a BPR volume-delay function of the lane's volume/capacity ratio, replacing the current random jitter
**So that** loaded lanes become measurably more expensive to route through, without a global equilibrium solver

**Acceptance criteria:**
- AC-1: Route cost over a lane is t0 * (1 + 0.15 * (v/c)^4) where t0 is the freeflow length/speed_limit cost and v/c is the lane's current volume-to-capacity ratio from the EMA counter. [SUBSTRATE: ABSENT — greenfield; audit §7 recommended design] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0035`
- AC-2: This BPR term replaces the existing tick-and-lane-seeded random jitter as the congestion feedback in route search; the CS1-derived '[0.9, (1000+density*10)/1000]' jitter formula and its implied '~2x congestion multiplier' are not carried forward, since that multiplier is unconfirmed and the jitter carries no real load information. [SUBSTRATE: PARTIAL — map/pathfinding.rs:224-225 (jitter exists but must be replaced, not extended)] · impact:`local` · seam:`integration` · scenario:`SCENARIO-0035`

**Sources:**
- `spec/pathfinding.md:26-33`
- `spec/traffic.md:36-38`

**Status:** pending

## STORY-0067

**Epic:** EPIC-014 — Emergent congestion (greenfield)
**Title:** Damp congestion cost with Gawron blending before it re-enters routing

**As a** planner relying on stable traffic patterns
**I want** the congestion cost that re-enters A* to be a blend of freshly observed cost and previously remembered cost, not the raw observed value
**So that** agents do not flap back and forth between two competing corridors every time either one's congestion reading changes

**Acceptance criteria:**
- AC-1: Before a lane's BPR cost re-enters the router, it is blended as remembered' = 0.3 * observed + 0.7 * remembered, and every re-route cycle reads only the damped value, never the raw instantaneous observed cost. [SUBSTRATE: ABSENT — greenfield; audit §7, SUMO default] · impact:`cross-surface` · seam:`unit` · scenario:`SCENARIO-0036`
- AC-2: Given two parallel corridors of equal base cost and a load perturbation that makes one briefly cheaper, the fraction of agents that switch corridors on successive re-route cycles decays toward zero (converges) rather than oscillating at a sustained non-zero rate; this is falsifiable by simulating N re-route cycles and asserting the switch-fraction time series is non-increasing after damping is applied, as a regression guard against the exact ping-pong failure A/B Street shipped without damping and had to remove. [SUBSTRATE: ABSENT — greenfield; audit §7 cites A/B Street's removed congestion-rerouting as the confirmed failure mode] · impact:`journey` · seam:`process-level` · scenario:`SCENARIO-0036`

**Sources:**
- `spec/traffic.md:36-38`

**Status:** pending

## STORY-0068

**Epic:** EPIC-014 — Emergent congestion (greenfield)
**Title:** Expose corridor utilisation as an economic bottleneck readout

**As a** planner deciding where to build additional road capacity
**I want** per-corridor utilisation/congestion shown as a first-class planning readout, derived from the same load signal used by routing
**So that** I can see that a corridor is saturated and choose to build another route, rather than inferring it indirectly from missed deliveries

**Acceptance criteria:**
- AC-1: The corridor utilisation readout shown to the planner is computed from the same per-lane EMA counter that feeds routing cost, not a second independently-sampled density measurement. [SUBSTRATE: ABSENT — greenfield; depends on the EMA counter story; audit §7 flags dual-tracker as a known bug class (TM:PE #66)] · impact:`cross-surface` · seam:`app-level` · scenario:`SCENARIO-0041`
- AC-2: A saturated road corridor cannot be routed around by money or priority alone; the only remedy visible to the planner is building additional physical capacity. [SUBSTRATE: ABSENT — greenfield] · impact:`journey` · seam:`app-level` · scenario:`SCENARIO-0041`

**Sources:**
- `spec/roads.md:29-33`

**Status:** pending

## STORY-0069

**Epic:** EPIC-014 — Emergent congestion (greenfield)
**Title:** Maintain safe following distance without a global solver

**As a** driver agent approaching a slower vehicle ahead
**I want** to brake based on a lookahead of the vehicle in front
**So that** jams emerge from many independent local braking decisions, with no queue object or scheduler needed

**Acceptance criteria:**
- AC-1: A vehicle computes a forward lookahead distance to the vehicle ahead and brakes to avoid collision, using an IDM-like raycast rather than a shared queue/scheduler object. [SUBSTRATE: PARTIAL — transportation/road.rs calc_front_dist] · impact:`local` · seam:`integration` · scenario:`SCENARIO-0042`
- AC-2: The spec's target model (reserve braking-distance space ahead as ½v²/a + half-length, CS1-style) is not the mechanism currently implemented; the existing raycast lookahead is a functional stand-in whose fidelity against the target formula is unverified. [SUBSTRATE: PARTIAL — transportation/road.rs calc_front_dist; no NetLane.ReserveSpace-equivalent object exists] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0042`

**Sources:**
- `spec/traffic.md:16-24`

**Status:** pending