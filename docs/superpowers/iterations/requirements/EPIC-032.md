# EPIC-032 — Waste — vehicle-hauled typed circular economy

**Summary:** Waste — vehicle-hauled typed circular economy
**Stories:** STORY-0120, STORY-0121, STORY-0122, STORY-0123
**Primary sources:** `spec/waste.md`
**Status:** 0/4 done

## STORY-0120

**Epic:** EPIC-032 — Waste — vehicle-hauled typed circular economy
**Title:** Buildings deposit typed waste into containers

**As a** planner
**I want** buildings to fill typed waste containers (mixed, bio, steel, aluminium, plastic, toxic, gravel, burnable, other) rather than an abstract garbage meter
**So that** downstream processing choice (sort vs mixed) has real consequences

**Acceptance criteria:**
- AC-1: A residential building deposits waste into a container-stand buffer; the container has per-type sorting bins as an optional planner-built upgrade, and unsorted deposit always goes to a waste_mixed bin. [SUBSTRATE: ABSENT — greenfield, zero waste-system footprint in simulation/src] · impact:`local` · seam:`integration`
- AC-2: An industrial recipe building can declare which waste sub-types it emits as production byproducts, distinct from a generic mixed-waste stream. [SUBSTRATE: ABSENT — greenfield] · impact:`local` · seam:`integration`

**Sources:**
- `spec/waste.md:16-23`

**Status:** pending

## STORY-0121

**Epic:** EPIC-032 — Waste — vehicle-hauled typed circular economy
**Title:** Waste collection reuses the logistics dispatcher

**As a** planner
**I want** a full waste container to become a source-job for the same deficit-driven dispatcher that handles ordinary goods logistics, under a WASTE class gate
**So that** there is exactly one dispatch system in the game, not a duplicate garbage-specific one

**Acceptance criteria:**
- AC-1: A garbage office building is a fleet-and-fuel dispatch office structurally identical to an ordinary distribution office, differing only in its class gate (WASTE); it does not run its own separate scheduling loop. [SUBSTRATE: ABSENT — greenfield; audit could not confirm a generic reusable dispatch-office abstraction exists (FreightStation, souls/freight_station.rs, is a pattern to study, not reuse as-is) — flag any AC assuming a ready-made dispatch office as blocked on the logistics work] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0133`
- AC-2: A full waste container is queued as a source-job under the same fleet-limit and class-gate rules as any other logistics source, with no garbage-specific priority formula bypassing the shared dispatcher. [SUBSTRATE: ABSENT — greenfield] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0133`

**Sources:**
- `spec/waste.md:24-27`

**Status:** pending

## STORY-0122

**Epic:** EPIC-032 — Waste — vehicle-hauled typed circular economy
**Title:** Waste processes to recycled material, energy, or landfill

**As a** planner
**I want** hauled waste to reach a separation-and-recycling plant (recovers named materials), an incinerator (produces electricity or district heat plus ash), or a landfill (permanent storage), by planner choice
**So that** waste management is a production decision with a real material/energy payoff, not a chore

**Acceptance criteria:**
- AC-1: A separation plant consuming waste_mixed produces named material outputs (e.g. gravel, steel, aluminium, plastic) at fixed extraction yields, which re-enter the ordinary item economy as recipe inputs. [SUBSTRATE: ABSENT — greenfield] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0132`
- AC-2: An incinerator consuming typed waste produces either electricity — incinerator_powerplant mode converts 3.0 units waste to 33 units electricity — or district heat — incinerator_heat mode converts 2.5 units waste to 450 units heat — plus ash, at fixed per-waste-type burn ratios distinct per mode, with high pollution as a byproduct; the magnitude, not just the presence, of output is fixed by mode. [SUBSTRATE: ABSENT — greenfield, cross-couples to the rebuilt electricity network and the heat plant story] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0132`
- AC-3: A landfill stores waste permanently — it never empties on its own and has no output — as the residual sink for toxic/unrecyclable material; a demolition/removal action targeting a landfill that still holds waste is rejected. [SUBSTRATE: ABSENT — greenfield] · impact:`local` · seam:`integration` · scenario:`SCENARIO-0132`
- AC-4: Waste already sorted into per-type bins at the source routes directly to its type-specific recycling/processing plant, bypassing the separation plant; waste collected as mixed must pass through a separation plant first before any type-specific recycling can consume it. [SUBSTRATE: ABSENT — greenfield, spec/waste.md:22] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0132`
- AC-5: Toxic waste can alternatively be routed to chemical neutralisation — consuming chemicals and producing high pollution — as a fate distinct from plain landfill storage and from incineration or recycling. [SUBSTRATE: ABSENT — greenfield, spec/waste.md:32] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0132`

**Sources:**
- `spec/waste.md:28-33`

**Status:** pending

## STORY-0123

**Epic:** EPIC-032 — Waste — vehicle-hauled typed circular economy
**Title:** Uncollected waste lowers attractiveness and raises sickness

**As a** planner
**I want** an overflowing, uncollected container to reduce local attractiveness and raise nearby sickness rate
**So that** neglecting waste collection has a local physical consequence instead of a city-wide meter

**Acceptance criteria:**
- AC-1: A container that stays full past its collection threshold for a sustained period reduces the attractiveness score of nearby buildings and raises the local sickness rate, with the effect scoped to the container's vicinity, not applied city-wide. [SUBSTRATE: ABSENT — greenfield, depends on an attractiveness/sickness model (spec/healthcare.md) that is itself unmodelled] · impact:`cross-surface` · seam:`integration`

**Sources:**
- `spec/waste.md:34-36`

**Status:** pending