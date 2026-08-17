# ECS layout — Bevy mapping

**Status:** ratified (ticket #5, 2026-08-16). Grounds: the Unity-track draft
(`~/Projects/soviet/architecture/ecs.md`, CS1/W&R evidence cited there) and the quarter-million
spike (issue #4, branch `spike/250k`). The engine-agnostic constraints carry over verbatim; this
doc records how they land on Bevy 0.19.

## Carried design constraints (unchanged)

1. **Physical identity survives inactivity** — a parked truck, off-screen citizen or idle building
   remains an entity.
2. **Money is not an execution component** — systems act on inventories, assignments, routes,
   capacity and labour; finance records outcomes.
3. **One authoritative owner per fact** — grids, route caches, dashboards and render state are
   derived and disposable.
4. **Clock band determines hotness** — component boundaries follow which systems read data
   together.
5. **Structural change follows lifecycle, not mood** — no archetype churn for transient flags
   (hungry, waiting, low-fuel are values, not components).

## Stable identity

Per-family `u64` monotonic counters (`CitizenId`, `BuildingId`, `VehicleId`, …). *As built these
are one small counter resource per family (`BuildingIds`, `RoadIds`, `VehicleIds`, …), each
serialized so a load restores the allocator rather than inferring it from max-seen — not the single
`IdAllocator` resource this section originally named.* **IDs are never reused; runtime `Entity` refs are never
serialized.** Loading resolves stable IDs to fresh entities in a dedicated remap pass. This closes
CS1's slot-reuse hazard and gives the band buckets their churn-stable key (ADR 0004). Everything
else about save/load — format, column serde, versioning — is deferred to its own ticket.

## Persistent state versus active pawn

The carried seam, as linked entities:

```text
Citizen ──0..1──> CitizenPawn { position, velocity, trip, route cursor }
VehicleAsset ──0..1──> ActiveVehicle { position, velocity, lane, route cursor, cargo }
```

Mapped onto **Bevy 0.19 first-class relationships**, thinly: the pawn link is a proper
relationship pair so despawn cleanup and referential integrity come from the engine — but pawn
creation/destruction still goes through the named command barriers below, never ad-hoc spawns.
Fallback to plain `Entity` fields costs no design change if relationships prove awkward for the
0..1 case. Arrival commits to the persistent entity first; only then is the pawn released.

## One band, one narrow query

| Band | Hot component sets | Excluded cold data |
|---|---|---|
| **High (1)** | active position, velocity, lane/segment, collision envelope | biography, demographics, history |
| **Medium (16)** | trip state, route cursor, loading/unloading, fuel/wear deltas | household needs, annual totals |
| **Low (256)** | production state, inventories, condition, network demand/supply, requests | transforms, presentation |
| **Very low (4096)** | citizen needs, education/labour binding, household/demographic state | pawn motion, route data |
| **Calendar** | plan/accounting accumulators, era/season state | per-frame motion |

Band membership is `BandRegistry` metadata consumed via `BandSweep<P>`
(see [simulation-clock.md](simulation-clock.md)), not duplicated per entity.

## System ownership and ordering

Systems own transformations, not data domains; one canonical writer per component per phase; no
cross-domain singleton-manager mutation. The per-tick pipeline is nine `SystemSet`s in the
`SimTick` schedule:

```text
ApplyCommands
  → CalendarEdges
  → NeedsAndServiceDemand
  → AllocationAndDispatch
  → RouteRequests / RouteResults
  → MovementAndTransfers
  → ProductionAndUtilities
  → CommitInventoriesAndCondition
  → AccountingAndCausalHistory
  → BuildDerivedIndicesAndPresentation
```

Explicit `apply_deferred` barriers sit at the `ApplyCommands` and
`CommitInventoriesAndCondition` seams (and nowhere else unless a benchmark demands it); Bevy's
automatic sync-point insertion is not relied on. Structural changes and cross-system writes are
buffered commands that land only at those barriers.

## Presentation

**Single world, direct rendering, direction-only invariant.** Pawn entities carry `Transform`; a
`PostSimEasing` system set in `Update` (after the SimTick driver) eases rendered transforms toward
authoritative sim positions. Presentation reads simulation state and never writes it — that
directional invariant is the load-bearing rule, enforced by set ordering, not by a mirror layer.
The carried mirror-entity seam stays documented as the upgrade path; it becomes mandatory only if
the sim moves to its own world (the clock doc's dedicated-thread door).

## Spatial indices and simulation graphs

- **Intended: a typed topology authority separate from a packed search mirror** — road/lane, rail,
  pedestrian, transit, electricity, water, sewage, heating. Right for the *editable authority*
  side; wrong for hot search. **As built, none of this exists**: topology is plain ECS
  (`RoadNode.segments: Vec<Entity>`) and nearest-node lookups are linear scans. `petgraph` was
  named here as the store and was never used; the crate has been dropped. See ADR 0005, which
  carried the same claim and was corrected — reading either document as fact once produced a wrong
  claim in a later ADR.
- **Intended: hand-rolled uniform grids for spatial lookup** — rebuildable, topology/version-stamped,
  one for mostly-static members (buildings, workplaces, services) and one for moving members
  (pawns). **As built, also absent**: lookups are linear scans with a distance filter. `kiddo` was
  held in reserve and has likewise been dropped; the trigger for building either structure is a
  benchmark showing the scans losing, not a date.
- Every cache carries a version stamp and can be rebuilt from authoritative data. World scans are
  forbidden in gameplay systems unless a benchmark records the bounded population and cadence that
  make one safe.

## Lifecycle and integrity

1. Reserve a stable ID, then create the persistent entity.
2. Resolve references only after all entities in a load/batch exist.
3. Create/destroy pawns and request entities only through the named barriers.
4. Before destroying a persistent entity: detach ownership, assignments, inventories and graph
   links; emit a causal event.
5. Destroy derived state freely; never destroy physical stock, cargo or a citizen as cleanup for a
   failed match, full queue or blocked route. Capacity exhaustion is a visible planning signal,
   not a silent delete.

## Open questions (inherited, still open)

- Persistent-citizen split point: benchmark the ~30-byte hot payload against colder split facets
  as populations grow past the spike's synthetic shape.
- Dynamic buffers versus pooled stores for inventory lines, route hops, causal history.
- Save/load column-serde strategy (own ticket; only the ID scheme is pinned here).
