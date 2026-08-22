# EPIC-002 — Construction Agents

**Summary:** Construction Agents
**Stories:** STORY-0007, STORY-0008, STORY-0009
**Primary sources:** `spec/buildings.md`, `spec/construction.md`
**Status:** 0/3 done

## STORY-0007

**Epic:** EPIC-002 — Construction Agents
**Title:** Stand up a construction office that stocks materials and dispatches its own vehicle fleet

**As a** the planner
**I want** a construction-office building type that receives materials through ordinary logistics, stocks them by class, and dispatches its owned construction-vehicle fleet to active sites
**So that** construction sites are served the same physically-grounded way freight stations serve cargo

**Acceptance criteria:**
- AC-1: A construction-office prototype declares resource-source flags per material class — WORKERS, GRAVEL, ASPHALT, CONCRETE, OPEN[steel/boards], OPEN_BRICKS, OPEN_PANELS, OPEN_BOARDS, plus the covered-storage classes COVERED and COVERED_ELECTRO (e.g. for weather-sensitive electrical components) — mirroring the $RESOURCE_SOURCE_* grammar, with covered classes tracked distinctly from open ones. [SUBSTRATE: ABSENT — greenfield; FreightStation (souls/freight_station.rs) is the nearest existing 'logistics office' shape to imitate per audit §3, explicitly 'a pattern to copy, not code to reuse'] · impact:`local` · seam:`integration` · scenario:`SCENARIO-0011`
- AC-2: The office owns a fleet sized by a declared vehicle count, tiered 8-24 vehicles by office tier ($WORKING_VEHICLES_NEEDED), and dispatches an idle vehicle to the nearest stalled site that needs its skill; a higher-tier office declares a larger fleet than a lower-tier office of the same kind. [SUBSTRATE: ABSENT — greenfield, no dispatch logic exists for construction vehicles] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0011`
- AC-3: A construction site with no reachable office in logistics range never receives materials, and its phases stall indefinitely rather than erroring or silently completing. [SUBSTRATE: UNAUDITED — no construction-office or site-reachability logic exists to check against] · impact:`local` · seam:`integration` · scenario:`SCENARIO-0011`
- AC-4: Workers assigned to a phase are sourced from the dispatching office's $RESOURCE_SOURCE_WORKERS pool like any other material class; a very large job additionally bills a fixed $COST_RESOURCE workers <n> lump that must be satisfied from the same pool before the phase can proceed. [SUBSTRATE: ABSENT — greenfield; no worker-sourcing-from-office mechanism exists, spec/construction.md:72 names this pool and the lump-sum variant] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0011`

**Sources:**
- `spec/construction.md:68-74`

**Status:** pending

## STORY-0008

**Epic:** EPIC-002 — Construction Agents
**Title:** Model construction vehicles with a skill-typed work throughput

**As a** the simulation
**I want** construction vehicles (crane, groundworks/excavator, bulldozer, asphalt-laying, rolling) to carry a numeric skill throughput matched against a phase's required skill
**So that** assigning the correct machine to a phase is meaningful and a mismatched machine contributes nothing

**Acceptance criteria:**
- AC-1: A crane vehicle assigned to an earthworks phase (which requires GROUNDWORKS skill) contributes zero throughput to that phase's progress. [SUBSTRATE: ABSENT — greenfield; Vehicle (transportation/vehicle.rs:34-44) is a bare kinematic shell with no skill or economic fields at all, audit §2 'Vehicle as economic asset: ABSENT'] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0004`
- AC-2: A vehicle's throughput counts toward a phase only after it has physically travelled to and parked in that phase's declared station slot. [SUBSTRATE: ABSENT — greenfield, no station-slot or vehicle-assignment concept exists] · impact:`local` · seam:`integration` · scenario:`SCENARIO-0004`

**Sources:**
- `spec/construction.md:68-74`
- `spec/construction.md:100-119`

**Status:** pending

## STORY-0009

**Epic:** EPIC-002 — Construction Agents
**Title:** Demolish and repair buildings as separate physical office-dispatched processes

**As a** the planner
**I want** demolition and repair dispatched from dedicated offices that consume explosives/materials over time and emit sorted rubble (demolition) or restore condition (repair), never instant deletion or a money refund
**So that** destroying or fixing a building is as physically grounded as building one

**Acceptance criteria:**
- AC-1: Demolishing a building consumes explosives plus machine-work over multiple ticks and only removes the building entity once the demolition process completes — no single-tick deletion path exists for a player-triggered demolition. [SUBSTRATE: ABSENT — greenfield; no demolition mechanic exists in audited code, and the audit does not confirm today's removal path either way, so this AC is a design requirement, not a proven regression] · impact:`journey` · seam:`integration` · scenario:`SCENARIO-0006`
- AC-2: A completed demolition emits typed rubble items (e.g. waste_gravel, waste_steel, waste_toxic) into the logistics/market system; no Money is credited to the player as a refund. [SUBSTRATE: ABSENT — greenfield; Money(i64) (prototypes/src/types/money.rs:14) has no refund-on-demolish path today because demolition itself does not exist] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0006`
- AC-3: A repair process restores a degraded building's condition by consuming a materials/work bill over time, dispatched from a distinct repair-office building entity ($TYPE_REPAIR_OFFICE/$REPAIR_AREA) rather than merely following a separate code path from the demolition office. [SUBSTRATE: ABSENT — greenfield; spec/construction.md:73 names repair and demolition as separate physical offices, not just separate flows] · impact:`local` · seam:`integration` · scenario:`SCENARIO-0006`
- AC-4: Once a demolition process completes and the building entity is removed, the underlying lot/land becomes available for a new siting checklist pass and a new ConstructionProject — it is not left permanently blocked. [SUBSTRATE: ABSENT — greenfield; spec/buildings.md:41 states demolition 'frees land', no lot-availability path exists to verify] · impact:`local` · seam:`integration` · scenario:`SCENARIO-0006`
- AC-5: A demolition process makes zero progress while its required explosives have not been delivered to the site, resuming automatically once explosives arrive — mirroring the material-stall behaviour of ordinary construction phases. [SUBSTRATE: ABSENT — greenfield; spec/construction.md:73 names explosives as a required input, not a formality] · impact:`local` · seam:`integration` · scenario:`SCENARIO-0006`

**Sources:**
- `spec/construction.md:68-74`
- `spec/buildings.md:41`

**Status:** pending