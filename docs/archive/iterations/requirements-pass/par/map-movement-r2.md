## F1 — RoadNode ≤8 attached-segment cap
- classification: omission
- source: spec/roads.md:18
- verbatim: "`RoadNode` (intersections/endpoints, ≤8 attached segments), `RoadSegment` (edge between two nodes, flags, average length, modification index), per-segment `Lane` buffer (type, direction, speed, curve)."
- what should have covered it: no story / no scenario
- why it matters: a concrete structural constant (max 8 segments per node) that a junction-generation or road-drawing implementation could violate silently with no AC to catch it.

## F2 — Road-type authored prefab / lane-template data model
- classification: omission
- source: spec/roads.md:20
- verbatim: "Road *types* are authored prefabs (CS1 `NetInfo` pattern): a lane template with per-lane `m_speedLimit`, type (`Vehicle | Pedestrian | Parking | PublicTransport | Cargo…`) and direction; the live network instantiates from it (`NetInfo.cs:75-108`)."
- what should have covered it: "Classify roads by surface class with a physical speed payoff" touches speed modifiers but no AC/scenario addresses the prefab/authoring data model itself, nor the explicit lane-type taxonomy (Vehicle | Pedestrian | Parking | PublicTransport | Cargo).
- why it matters: without a defined lane-type enum, the "heavy vehicle excluded from pedestrian lane" scenario has no named taxonomy of what other lane types (Parking, PublicTransport, Cargo) must also gate access.

## F3 — Modification index invalidates cached routes
- classification: missing-scenario
- source: spec/roads.md:21
- verbatim: "A segment's `modificationIndex` invalidates cached routes when the road is redrawn (`PathUnit.cs:122-124`) — no central subscription needed."
- what should have covered it: no story / no scenario. AC-3 of "Compute turn-aware routes over the unified lane graph" asserts there is no general OD cache, but that is a different claim than "redrawing a road invalidates any in-flight/queued route referencing it."
- why it matters: redrawing a road mid-simulation is an observable event (construction, upgrade) whose effect on already-issued paths is unspecified in the extraction — a real correctness/determinism gap if routes silently reference deleted segments.

## F4 — Intra-facility internal connection graph for large compounds
- classification: omission
- source: spec/roads.md:25
- verbatim: "Intra-facility routing: large compounds (rail yards, ports) may author an internal connection graph, as W&R does for exactly two large prefabs via `$ROAD_CON_PID`/`$ROAD_CON_DETOUR` (research §D4, §G8)."
- what should have covered it: no story / no scenario
- why it matters: this is a named, distinct sub-mechanism (compound-internal graph) with no AC coverage at all — a whole feature area silently absent from the extraction.

## F5 — Road segments are physical, construction-gated objects
- classification: thin-AC
- source: spec/roads.md:12
- verbatim: "Roads are **built physical objects** — they exist only after a construction project delivers them ([spec/construction.md](construction.md)), and their capacity is a physical fact, not a budget slider."
- what should have covered it: no story addresses that a road segment cannot exist without a completed construction project (as opposed to e.g. a player instantly placing a road).
- why it matters: this is a binding constraint explicitly called out in project context ("nothing teleports, every effect has a physical cause") — the extraction has zero AC enforcing construction-gated road existence.

## F6 — Road upgrade consumes materials/labour, never money
- classification: omission
- source: spec/roads.md:27
- verbatim: "Upgrading dirt → paved is a construction project consuming asphalt/gravel and labour — never money."
- what should have covered it: "Classify roads by surface class with a physical speed payoff" story covers the speed payoff but no AC states the upgrade must be construction-driven (materials+labour) and explicitly NOT purchasable with money.
- why it matters: this is a core Kornai-economy binding constraint (no market-clearing by price) directly stated in the source and entirely unrepresented.

## F7 — Corridor saturation cannot be bypassed with money/priority (partially covered, duplicate check)
- classification: intentional-exclusion
- source: spec/roads.md:30
- verbatim: "You cannot buy past a saturated road; you build another one."
- what should have covered it: covered by "Expose corridor utilisation as an economic bottleneck readout" AC-2.
- why it matters: n/a — already represented, listed only to confirm it was checked, not a gap.

## F8 — Road wear/condition as maintenance sink (explicitly open question)
- classification: intentional-exclusion
- source: spec/roads.md:32
- verbatim: "Road wear/condition as a physical maintenance sink — no precedent in either lab (W&R road params are entirely native); fully OURS. In scope?"
- what should have covered it: n/a
- why it matters: explicitly an open, undecided question in the spec ("In scope?"), not a settled requirement — correctly excluded from extraction.

## F9 — Which road classes ship at v1 (open question, tram-carrying road)
- classification: intentional-exclusion
- source: spec/roads.md:34
- verbatim: "Which road classes ship at v1 (dirt/paved minimum; tram-carrying road needed early for the Soviet feel?)."
- what should have covered it: n/a
- why it matters: phrased as an open question with a "?", not a normative requirement — correct to exclude, though note F2 shows the tram/pedestrian/vehicle mode taxonomy elsewhere in roads.md line 27 ("plus tram/pedestrian") is stated more firmly and IS a gap (see F2).

## F10 — Storage topology decision deferred to Phase 2 (blob assets vs entity graph)
- classification: intentional-exclusion
- source: spec/roads.md:33
- verbatim: "Store topology as blob assets vs entity graph for Burst traversal — Phase 2 decision."
- what should have covered it: n/a
- why it matters: explicitly deferred, non-normative for this phase.

## F11 — Bucketed-priority-queue / thread-count solver architecture (CS1 reference, not adopted verbatim)
- classification: intentional-exclusion
- source: spec/pathfinding.md:23
- verbatim: "CS1 runs 1-4 solver threads with a bucketed priority queue (`BufferItem[65536]` in 1024 priority rows — an O(1)-ish bucketed Dijkstra, `PathFind.cs:130, 367-377`). Our version: Burst-compiled A* jobs over the lane graph (research §G3)."
- what should have covered it: n/a — this describes CS1's implementation detail, not a requirement of the target system beyond "Burst-compiled A* jobs," which is an implementation/architecture note, not a testable behavior.
- why it matters: correctly non-normative (describes prior-art mechanics used only to justify the adopted approach).

## F12 — En-route vehicles do not re-plan as congestion builds (no-continuous-replan constraint)
- classification: missing-scenario
- source: spec/pathfinding.md:31
- verbatim: "En-route vehicles do **not** re-plan as congestion builds (INFERRED, research §B5) — re-plan only on blockage/invalidation or the stall escalation in [spec/traffic.md](traffic.md). Traffic adapts at trip granularity."
- what should have covered it: no story states this as a constraint on ordinary (non-stalled) vehicles; only the stall-triggered re-route path (AC in "Escalate a stalled vehicle...") is covered. No scenario proves a vehicle that is NOT stalled continues on its original route despite congestion rising around it mid-trip.
- why it matters: this is a determinism/negative-behavior requirement — the absence of continuous re-planning — that is easy to accidentally violate (e.g. by re-costing routes every tick) and has zero test coverage in the extraction.

## F13 — Path-request throughput / solver budget at scale (open question, but numeric anchor given)
- classification: omission
- source: spec/pathfinding.md:37 (Open questions)
- verbatim: "Path-request throughput at 100k citizens: CS1 caps at 262144 pooled PathUnits and 4 threads — what's our Burst-job budget per tick? Feeds the citizen-scale prototype ([spec/citizens.md](citizens.md) scale question)."
- what should have covered it: no story / no scenario addresses any performance/tick-budget requirement for pathfinding at scale.
- why it matters: this is flagged as an explicit open question rather than a settled requirement, so arguably borderline, but the extraction gives zero proof obligation for pathfinding performance/scale at all (not even a placeholder), and performance/determinism requirements were called out by the brief as a category to check — flag as omission since no AC anywhere addresses tick budgets or agent-count scaling for the whole domain.

## F14 — Admissible heuristic choice left open
- classification: intentional-exclusion
- source: spec/pathfinding.md:38
- verbatim: "Admissible heuristic details: CS1's exact heuristic wasn't fully traced (research Gaps) — pick our own (straight-line/maxspeed)."
- what should have covered it: n/a
- why it matters: explicitly an open/undecided implementation detail, not a settled behavioral requirement.

## F15 — Hierarchical shortcuts (highway abstraction) explicitly speculative
- classification: intentional-exclusion
- source: spec/pathfinding.md:39
- verbatim: "Hierarchical shortcuts (highway-level abstraction) if flat lane-graph A* proves too slow on large maps — SPECULATIVE, only if the prototype says so."
- what should have covered it: n/a
- why it matters: explicitly speculative/conditional, correctly excluded.

## F16 — Car-ban ×7.5 and transit-lane ×0.95 hard modifiers
- classification: omission
- source: spec/pathfinding.md:28
- verbatim: "**Hard modifiers:** car-ban ×7.5, closed segments rejected, transit lanes ×0.95 for transit vehicles. CS1's toll costs are dropped for citizens (no road pricing in a planned economy); class-based penalties (dirt road slows heavy trucks) replace them ([spec/roads.md](roads.md))."
- what should have covered it: no story / no scenario. "Carry routing policy on the trip request" AC-1 covers lane/vehicle-type masking generally but names no numeric modifier; the ×7.5 car-ban penalty, closed-segment rejection, and ×0.95 transit-lane discount are unrepresented specific numeric constants.
- why it matters: three concrete, numerically-specified cost modifiers with no AC or scenario to pin them down — easy to implement inconsistently or omit.

## F17 — Pedestrian tie-breaker / no-congestion-jitter detail
- classification: thin-AC
- source: spec/pathfinding.md:29
- verbatim: "Pedestrian lanes: same time-cost without congestion jitter, small random tie-breaker so walkers spread (`PathFind.cs:670, 822`)."
- what should have covered it: "Carry routing policy on the trip request" AC-2 states pedestrians are costed without the congestion term and have a tie-breaker — this one is actually covered. Listing to confirm checked.
- why it matters: n/a (covered) — included only as a check note, not a real gap.

## F18 — Class-based penalty replacing toll (dirt road slows heavy trucks)
- classification: omission
- source: spec/pathfinding.md:28
- verbatim: "class-based penalties (dirt road slows heavy trucks) replace them ([spec/roads.md](roads.md))"
- what should have covered it: "Classify roads by surface class with a physical speed payoff" only covers a generic road-class × terrain speed modifier applied uniformly; it does not cover a vehicle-class-specific penalty (heavy trucks specifically penalized more on dirt than other vehicle types).
- why it matters: this is a distinct, more specific claim than "road class affects effective speed" — it says the penalty varies BY VEHICLE CLASS (heavy trucks worse-hit), which no AC or scenario tests.

## F19 — Stall-threshold tuning left as open question (numeric contradiction risk)
- classification: intentional-exclusion
- source: spec/traffic.md:44
- verbatim: "Stall threshold tuning: how long does a truck wait before re-route vs stall-report? Per-cargo urgency?"
- what should have covered it: n/a for this specific open question, BUT note the extraction's own AC ("Never delete a vehicle for being gridlocked" AC-2) asserts a concrete "~200 seconds" Panicking threshold sourced from `transportation/vehicle.rs`, which is fork-code-derived, not from this open spec question — correctly excluded as a spec gap since the spec itself declines to commit to a number.
- why it matters: flagging so the reviewer knows the 200s number in the extraction is NOT drawn from this open question (it is PROVIDED/code-derived), avoiding a false "resolved" impression.

## F20 — Vehicle stall/plan-pool blocking (open question)
- classification: intentional-exclusion
- source: spec/traffic.md:45
- verbatim: "Do stalled vehicles block the plan's vehicle pool (they should — they're physical assets, [spec/vehicles.md](vehicles.md))?"
- what should have covered it: n/a
- why it matters: phrased as an open question ("?"), the spec itself does not commit — correctly outside the extraction's normative scope, though it borders a real gap since the parenthetical "(they should...)" leans normative. Flagging as borderline for the lead's attention rather than a firm omission.

## F21 — Parking as pressure/search (explicit open scope question)
- classification: intentional-exclusion
- source: spec/traffic.md:43
- verbatim: "Parking: neither lab models it as pressure (W&R declares only parking *spots*). CS2-style parking search is OURS if wanted — in scope?"
- what should have covered it: n/a
- why it matters: explicitly an open "in scope?" question, not a committed requirement.

## F22 — Comfort/reliability route-cost terms are unproven OURS extensions
- classification: intentional-exclusion
- source: spec/traffic.md:38
- verbatim: "CS1's toll/ticket terms have no place for citizens in a planned economy; comfort/reliability terms (CS2-inspired) are OURS extensions, unproven in either lab."
- what should have covered it: n/a
- why it matters: explicitly labeled unproven/speculative extension, not a settled requirement — correct to exclude.

## F23 — Reservation formula ½v²/a + half-length as the *target* mechanism, not yet the delta from current stand-in
- classification: thin-AC
- source: spec/traffic.md:22
- verbatim: "A car reserves braking-distance space ahead (`½v²/a + half-length`, `CarAI.cs:373-395` → `NetLane.ReserveSpace`); a follower reading a full lane brakes toward 0."
- what should have covered it: "Maintain safe following distance without a global solver" AC-2 does note the target formula isn't implemented and fidelity is unverified — this is already thin but present. No scenario tests the actual ½v²/a+half-length formula numerically (only a qualitative brake/no-collision scenario exists). Flagging the numeric formula itself as untested even qualitatively-covered.
- why it matters: the acceptance criterion explicitly says fidelity against the formula is "unverified" yet no scenario exists to eventually verify it — a proof obligation is named as missing but never turned into a scenario, i.e. a self-acknowledged gap in the extraction itself.
TOTAL FINDINGS: 11
