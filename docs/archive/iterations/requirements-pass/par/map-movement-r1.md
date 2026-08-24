## F1 — Numeric hard modifiers for route cost (car-ban ×7.5, transit ×0.95, closed segments rejected)
- classification: thin-AC
- source: spec/pathfinding.md:31
- verbatim: "**Hard modifiers:** car-ban ×7.5, closed segments rejected, transit lanes ×0.95 for transit vehicles."
- what should have covered it: story "Carry routing policy on the trip request, not the graph" / scenario "Heavy vehicle request is excluded from a pedestrian-only lane"
- why it matters: the existing story/scenario only tests binary lane-type exclusion (heavy vehicle vs pedestrian lane); the three concrete numeric multipliers/rejections that make up the "hard modifiers" cost term are stated in the spec but no AC or scenario asserts any of the three specific values or the "closed segments rejected outright" behavior.

## F2 — Paths are solved asynchronously via background worker, not inline
- classification: omission
- source: spec/pathfinding.md:22
- verbatim: "the agent waits on a ready-flag while a background worker solves (research §B2)."
- what should have covered it: no story, no scenario
- why it matters: this is an observable execution-model requirement (non-blocking path solve) carried over from CS1 and intended as OURS's Burst-job model (spec/pathfinding.md:23, "Our version: Burst-compiled A* jobs over the lane graph"); no AC or scenario asserts that path requests do not block the requesting agent/frame.

## F3 — RoadNode capped at ≤8 attached segments
- classification: omission
- source: spec/roads.md:18
- verbatim: "`RoadNode` (intersections/endpoints, ≤8 attached segments)"
- what should have covered it: no story, no scenario
- why it matters: a concrete structural capacity constant on the network's intersection representation is stated in the spec but not represented by any AC or falsifiable scenario (e.g. a 9th segment being rejected).

## F4 — Road upgrade is a construction project consuming materials/labour, never money
- classification: omission
- source: spec/roads.md:27
- verbatim: "Upgrading dirt → paved is a construction project consuming asphalt/gravel and labour — never money."
- what should have covered it: story "Classify roads by surface class with a physical speed payoff" (covers only the resulting speed delta) / no scenario
- why it matters: this is the highest-value CS-style divergence in the roads domain — a stock city-builder would let you buy a road upgrade with cash; the spec explicitly rejects money as the resource and requires physical construction inputs instead. No AC or scenario tests that the upgrade path is money-free and resource/labour-gated.

## F5 — Roads only exist after a construction project delivers them (no pre-existing/free road placement)
- classification: omission
- source: spec/roads.md:12
- verbatim: "Roads are **built physical objects** — they exist only after a construction project delivers them ([spec/construction.md](construction.md)), and their capacity is a physical fact, not a budget slider."
- what should have covered it: no story, no scenario
- why it matters: this is a direct rejection of stock Cities-Skylines-style instant/toolbar road placement (also directly relevant given the project's other DISABLED-automatic-generation constraint); no AC/scenario falsifies instant road creation or asserts capacity is fixed rather than adjustable via a slider/budget.

## F6 — Intra-facility routing: large compounds may author an internal connection graph
- classification: omission
- source: spec/roads.md:33
- verbatim: "Intra-facility routing: large compounds (rail yards, ports) may author an internal connection graph, as W&R does for exactly two large prefabs via `$ROAD_CON_PID`/`$ROAD_CON_DETOUR` (research §D4, §G8)."
- what should have covered it: no story, no scenario
- why it matters: this is an adopted (OURS, not merely an open question) mechanism for how large compounds connect internally to the lane graph; it is absent from the extraction entirely, with no story or scenario covering internal-compound routing.

## F7 — Saturated segment hard-flags Blocked (buffer overflow)
- classification: intentional-exclusion
- source: spec/traffic.md:21
- verbatim: "buffer overflow hard-flags the segment `Blocked` (`RoadBaseAI.cs:1400-1440`)"
- what should have covered it: n/a
- why it matters: this describes CS1's raw buffer/density mechanism, which the extraction's EMA-counter story (map-movement.json "Track per-lane traffic load...") explicitly supersedes ("no per-segment traffic density signal exists... to build on"); the CS1 Blocked-byte flag is superseded design detail, not a requirement carried forward, so exclusion is correct.

## F8 — Three-stage stall escalation order (Wait → Re-route → Register stall) as a strict sequence
- classification: thin-AC
- source: spec/traffic.md:30-32
- verbatim: "1. **Waits** (jams persist physically, as in W&R), 2. **Re-routes** if an alternative exists, 3. **Registers a logistics stall**"
- what should have covered it: stories "Never delete a vehicle for being gridlocked" and "Escalate a stalled vehicle to re-route..." / scenarios "Stalled vehicle with an alternative route re-routes..." and "...registers a planner-visible bottleneck event"
- why it matters: the extracted scenarios test re-route (with alternative) and stall-event (without alternative) as two independent branches, but no AC/scenario asserts the vehicle waits first (a distinct pre-re-route phase) before attempting a re-route — the ordered three-stage escalation itself is untested, only its two terminal outcomes.

## F9 — Congestion must feed back into economy: delivery stalls production, commutes erode wellbeing
- classification: intentional-exclusion
- source: spec/traffic.md:12
- verbatim: "Congestion must feed back into the economy: late deliveries stall production ([spec/logistics.md](logistics.md)), long commutes erode wellbeing ([spec/needs.md](needs.md))."
- what should have covered it: n/a for this domain
- why it matters: this cross-domain consequence belongs to the logistics/needs domains (explicitly cross-referenced), not map-movement; the map-movement extraction correctly stops at emitting the bottleneck/stall event and leaves the downstream economic effect to those other domains.

TOTAL FINDINGS: 6
