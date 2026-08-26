# Construction

**Status:** draft model (grounded in research)
**Phase:** 1
**Primary inspiration:** W&R physical construction, deepened to explicit phases (OURS); CS1 for road-cost + placement UX
**Evidence:** see [research/construction.md](../research/construction.md); consumes `spec/resources.md`, produced-by `spec/production.md`, delivered-by `spec/logistics.md`.

> The signature mechanic. A building or road **does not appear because it was paid for** — it is the *output of a physical process*: materials produced (`production`) → delivered (`logistics`) → assembled on site by workers + machinery over time. This is the emotional core of the project: **"we built a highway," not "I bought a highway."**

## Purpose

Define how a placed blueprint becomes a finished structure by physically consuming resources and labour on site — and, critically, how it can **stall** at any step (no steel → foundations wait; no crane → structure waits; road jammed → nothing arrives). Construction is where `resources` + `production` + labour + `logistics` converge, so its failure modes are the whole game's failure modes made visible.

## The lifecycle

```
Blueprint (player places)
      ↓  state authorises land + generates a material/labour bill of quantities
ConstructionProject  ── phases ──▶  (each phase = a mini production recipe on site)
      ↓  materials delivered by construction vehicles; workers + machinery apply workdays
Phase N complete → Phase N+1 begins
      ↓
ProjectComplete → BuildingActivated  (now it can produce/house/serve)
```

### What the research confirmed (`research/construction.md`)

**W&R construction is a physical bill with no money at all** — CONFIRMED. Every building `.ini` carries:
- **`$COST_WORK <PHASE> <work-multiplier>`** — opens a named construction phase (1133 uses). **Phases are explicit, named and ordered** (this is the big finding — the skeleton's provisional phasing is real):
  `SOVIET_CONSTRUCTION_GROUNDWORKS` (475, always mult 0.0 — pure earthmoving) → `SKELETON_CASTING` (concrete) / `STEEL_LAYING` / `BRICKS_LAYING` / `PANELS_LAYING` (prefab `panelák`) / `BOARDS_LAYING` → `WIRE_LAYING` (only 3 files!) → `ROOFTOP_BUILDING`, plus `TUNNELING`. A building lists only the phases its construction method uses, in craft order.
- **`$COST_RESOURCE_AUTO <material-class> <mult>`** — the per-phase material bill (1387 uses), by *class* not raw resource: `ground`/`ground_asphalt`, `wall_brick`, `wall_concrete`, `tech_steel`/`wall_steel`/`electro_steel`, `wall_panels`, `roof_*`, `wall_wood`. The **quantity is derived natively from mesh-node geometry × the multiplier** — a bigger building costs more because its nodes are bigger.
- **`$COST_WORK_VEHICLE_STATION <coords>`** — parking slots on the site where construction vehicles stand to work each phase (big buildings list several per phase → multiple cranes in parallel).
- **No `$COST_MONEY`/`$COST_RUB` token exists in any building `.ini`** — CONFIRMED absent. `$COST_RUB` is only a *vehicle* purchase price. **Money is not how a building comes into existence.**

**CS1 is the exact opposite** — CONFIRMED: a flat `m_constructionCost × 100` charged at placement + a cosmetic `m_constructState` 0→255 render ramp (`+1088/constructionTime`/frame; service buildings instant, growables `constructionTime=30`). **No materials, no workers, no construction vehicles.** Money (and for growables a timer) is sufficient. This is the sharpest embodiment of the project's one rule — we take W&R's model wholesale.

## Phased construction (grounded on W&R's confirmed phases, extended — OURS)

W&R **already has explicit named phases** (confirmed above); the transcript wants a slightly fuller, uniform pipeline for buildings **and** roads so the player can *watch* a structure rise and see exactly where it stalls. Our phases map onto W&R's real ones, filling its two gaps (it folds foundations into groundworks and barely uses `WIRE_LAYING`):

**Building phases:**

| Our phase | W&R phase(s) it maps to | Consumes (material class → resource) | Machinery skill |
|---|---|---|---|
| 1 Earthworks | `GROUNDWORKS` (mult 0.0) | `ground`/`ground_asphalt` → gravel/asphalt pad | `GROUNDWORKS`, `BULLDOZER` (excavators) |
| 2 Foundations | `SKELETON_CASTING` | `wall_concrete` → concrete | `CRANE` |
| 3 Structure | `STEEL_LAYING` / `BRICKS_LAYING` / `PANELS_LAYING` | `tech_steel`/`wall_steel`, `wall_brick`, `wall_panels` → steel/bricks/prefab | `CRANE` |
| 4 Utilities | `WIRE_LAYING` (expanded — OURS) | `electro_steel`, ecomponents/mcomponents → wiring/plumbing | technicians |
| 5 Finishing | `BOARDS_LAYING` / `ROOFTOP_BUILDING` | `roof_*`, `wall_wood` → boards/glass | `CRANE` / labour |

(`TUNNELING` is a special earthworks variant for underground work.)

**Road/infrastructure phases (OURS — W&R computes road cost natively with no phases):** earthworks → sub-base (gravel) → paving (concrete) → surfacing (asphalt) → markings → open. Worked by `GROUNDWORKS`→`ASPHALT_LAYING`→`ROLLING` vehicles, consuming gravel then asphalt. This is the mechanic the project is named for — a highway visibly progressing grading → laying → surfacing.

Each phase is effectively a `production`-style recipe (materials + machine-work → phase-complete), so it **reuses the production factor-gate model** (`spec/production.md`): missing any factor stalls that phase. That's the unification — construction *is* production, applied to a one-shot on-site project. A phase cannot start until (a) its predecessor phase is done, (b) its material is on site, and (c) a machine with the matching skill is assigned.

### Duration is emergent from supply, never a fixed timer (CONFIRMED mechanism)

The key W&R mechanic to adopt: construction vehicles carry a `$SKILL_CONSTRUCTION_*` tag with a numeric **work throughput** (cranes 21–95, groundworks 15–37, rolling 18–27). The game matches a phase to vehicles with the corresponding skill. So:

```
phase_time = phase_work / Σ(assigned_vehicle_skill)     // more/better machines → faster
           = ∞  whenever material is missing OR no matching machine is assigned
```

This is why W&R's phase number is a *work multiplier*, not a worker-day count — throughput comes from the machines. **We adopt this as the explicit duration law** and reject CS1's fixed `m_constructionTime` ramp. CS1's road cost, by contrast, is pure money × distance: `cost = m_constructionCost × round(length/8)`, instant, no materials (`PlayerNetAI.cs:28`) — the anti-pattern we replace with the phased road pipeline above.

## Construction agents (who does the building) — CONFIRMED

- **Construction office** (`$TYPE_CONSTRUCTION_OFFICE`, + `_RAIL` variant) — the actor. It **stocks physical materials** it will dispatch to sites via `$RESOURCE_SOURCE_*` flags (WORKERS, GRAVEL, ASPHALT, CONCRETE, OPEN[steel/boards], OPEN_BRICKS, OPEN_PANELS, OPEN_BOARDS, COVERED, COVERED_ELECTRO) and **owns a vehicle fleet** (`$WORKING_VEHICLES_NEEDED`, 8–24 by office tier). The office is itself filled by ordinary cargo logistics, then trucks material to each site. Generalises into `spec/logistics.md`'s scheduler.
- **Construction vehicles** — physical assets (`spec/vehicles.md`) with a `$SKILL_CONSTRUCTION_*` throughput: `CRANE` (lifting: steel/brick/panel/roof), `GROUNDWORKS` (excavators), `BULLDOZER` (earthmoving/demolition), `ASPHALT_LAYING` + `ROLLING` (roads). Must travel to the site and park in a phase's `$COST_WORK_VEHICLE_STATION` slot. No matching machine ⇒ phase stalls.
- **Workers** — from the office's `$RESOURCE_SOURCE_WORKERS` pool (`spec/citizens.md`); a few very large jobs also bill a fixed `$COST_RESOURCE workers <n>` lump.
- **Demolition** (`$TYPE_DEMOLITION_OFFICE`) and **repair** (`$TYPE_REPAIR_OFFICE`/`$REPAIR_AREA`) are **separate physical offices** — CONFIRMED. Demolition consumes **explosives** + machine-work and **emits sorted rubble** (`waste_gravel`/`waste_steel`/`waste_toxic`) back into logistics — no money refund, no instant deletion. We keep this.

## Why this is the payoff spec

Every arrow in `needs.md`'s core loop that "can fail" terminates here:
- housing shortage → new residential blocks → **construction** needs steel → steel mill (`production`) → coal trains (`logistics`) → …
- if any link breaks, construction visibly stalls at a specific phase with a named bottleneck (reusing `production`'s `bottleneck` field).

Money never substitutes for a missing material or crane. That is the project's one rule, enforced at the point the player feels it most.

## Zoning interaction (from vision)
The player places **all** buildings (no organic auto-spawn — confirmed constraint). A residential "zone" authorises land + generates development demand; the actual building is still a `ConstructionProject`. See `spec/zoning.md`.

## Open questions

**Resolved by research:**
- ~~W&R fidelity~~ → W&R construction **is** explicitly phased (named phases, per-phase material bill). Our phasing is grounded, not invented.
- ~~Machinery sourcing~~ → construction office owns the fleet + stocks materials (`$WORKING_VEHICLES_NEEDED` + `$RESOURCE_SOURCE_*`). Confirmed.
- ~~Demolition~~ → separate physical office; explosives in, sorted rubble out. Confirmed.

**Still open:**
- **Phase granularity.** W&R uses ~7 phases but barely wires utilities (`WIRE_LAYING` in only 3 files) and has no plumbing/finishing split. Do we expand to a full earthworks→foundations→structure→utilities→finishing pipeline (richer stalls) or stay close to W&R? Lean: expand modestly — "causal depth, not micromanagement" (vision).
- **Material delivery model.** Incremental (build proceeds as materials arrive) or full-bill-first per phase? Lean incremental — matches the transcript's "21 of 28 tonnes arrived, waits for 7 more." (W&R's exact gating is native — INFERRED.)
- **Material quantity source.** W&R derives quantity from **mesh-node geometry** natively — we have no meshes at spec stage. We need our own **bill-of-quantities formula** (per building size/type). This is an OURS design task, flagged for Phase 2.
- **Roads: per-segment or per-project?** Each segment independent, or one project per drawn route? Ties to `spec/roads.md`'s two-representation model. (W&R computes road cost natively per length — no guidance.)
- **Phase parallelism.** W&R runs multiple vehicles per phase (multiple station slots) but phases are ordered. Do any phases overlap (e.g. utilities during structure)? Lean: strict order for legibility, parallel machines within a phase.

## Data (draft)
```
BillOfQuantities { materials: [{ resourceId, qty }]; work; requiredSkill }   // per phase; work in skill-units
ConstructionPhase {
  id; phaseKind                          // earthworks | foundations | structure | utilities | finishing
  bill: BillOfQuantities
  requiredSkill                          // CRANE | GROUNDWORKS | ASPHALT_LAYING | ...  (matches vehicle skill)
  deliveredSoFar; workDone; state        // pending | active | complete | STALLED
  stationSlots[]                         // where machines park (our analogue of $COST_WORK_VEHICLE_STATION)
}
ConstructionProject {
  blueprintId; siteLocation
  phases: ConstructionPhase[]            // ordered; a phase starts only when predecessor complete
  currentPhase
  bottleneck                             // no-material | no-machine | no-worker  → player UI (shared w/ production.md)
  assignedOffice; assignedVehicles[]
}
// duration law: phase_time = phase.work / Σ(assignedVehicles matching requiredSkill).skill
```
Construction progress runs at **medium** frequency (material accounting) with **high**-frequency vehicle movement to/from site (`architecture/simulation-clock.md`). Visual progress can reuse a CS1-style 0→255 render ramp, but it is *driven by* real material/machine state, never a timer.

## Evidence log
| Claim | Evidence level | Source | Notes |
|---|---|---|---|
| W&R construction consumes materials + labour-work, **no money token in building `.ini`** | CONFIRMED | W&R `$COST_RESOURCE_AUTO` (1387), `$COST_WORK` (1133); no `$COST_MONEY` | research/construction.md §A |
| Construction is **explicitly phased**, named, ordered (GROUNDWORKS → structure → ROOFTOP) | CONFIRMED | W&R `SOVIET_CONSTRUCTION_*` keywords | research/construction.md §A1 |
| Per-phase material bill by class (`ground_asphalt`, `wall_steel`, `wall_panels`…) | CONFIRMED | W&R `$COST_RESOURCE_AUTO <class> <mult>` | class→resource mapping is INFERRED (native) — §A2 |
| Duration = phase_work / Σ(vehicle `$SKILL_CONSTRUCTION_*` throughput) | CONFIRMED (tokens) / INFERRED (formula) | W&R vehicle `script.ini` | research/construction.md §B3 |
| Construction office owns fleet + stocks materials (`$RESOURCE_SOURCE_*`, `$WORKING_VEHICLES_NEEDED`) | CONFIRMED | W&R `construction_office.ini` | research/construction.md §B |
| Demolition is physical: explosives in, sorted rubble out (no refund) | CONFIRMED | W&R `$TYPE_DEMOLITION_OFFICE` | research/construction.md §B5 |
| CS1 construction = money at placement + cosmetic `m_constructState` ramp, no materials | CONFIRMED | CS1 `PlayerBuildingAI.cs:1112`, `CommonBuildingAI.cs:1449` | the anti-pattern — §E |
| CS1 road cost = `m_constructionCost × round(length/8)`, instant | CONFIRMED | CS1 `PlayerNetAI.cs:28` | research/construction.md §E3 |
| Construction reuses the production factor-gate model (a phase = a recipe) | OURS | — | unifies with spec/production.md |
| Expanded/uniform phase pipeline for buildings **and** roads (utilities/finishing split) | OURS | — | transcript §15–16; extends W&R's real phases |
| Material quantity from mesh geometry (W&R) → we need our own bill-of-quantities formula | OURS | W&R native | Phase-2 design task |
| Player places all buildings; no organic auto-spawn | CONFIRMED (design) | vision doc | see spec/zoning.md |
| Incremental material delivery (build proceeds as materials arrive) | INFERRED | transcript image; W&R native gating | to confirm |

## Related
- ../research/construction.md · ../spec/resources.md · ../spec/production.md · ../spec/logistics.md · ../spec/vehicles.md · ../spec/roads.md · ../spec/zoning.md · ../spec/needs.md · ../architecture/simulation-clock.md
