# EPIC-031 — Heating — district network with temperature-driven demand

**Summary:** Heating — district network with temperature-driven demand
**Stories:** STORY-0116, STORY-0117, STORY-0118, STORY-0119
**Primary sources:** `spec/heating.md`
**Status:** 0/4 done

## STORY-0116

**Epic:** EPIC-031 — Heating — district network with temperature-driven demand
**Title:** Heat plants burn fuel into district heat

**As a** planner
**I want** a heat plant to consume coal and workers and produce heat as an ordinary recipe building, with waste incineration as an alternate fuel source
**So that** heat generation obeys the same production model as electricity and follows the waste-to-heat coupling

**Acceptance criteria:**
- AC-1: A heat plant with no coal input or no assigned workers produces zero heat, using the standard recipe/backpressure machinery. [SUBSTRATE: PROVIDED pattern, ABSENT instance — recipe mechanics exist (souls/goods_company.rs:36-39) but no heat plant prototype or `heat` resource exists] · impact:`local` · seam:`integration` · scenario:`SCENARIO-0132`
- AC-2: A waste incinerator building can alternatively consume typed waste and produce heat (per spec/waste.md's incinerator_heat coupling), feeding the same district heat network as a coal plant. [SUBSTRATE: ABSENT — greenfield, depends on the waste story 'Waste incineration produces power or district heat'] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0132`
- AC-3: A heat plant supplied with its full recipe input (0.28 coal per tick, 30 workers assigned) produces heat at the recipe's rated output of 350 per tick — the coal-to-heat ratio is a fixed proportional constant, not merely a positive value once inputs are present. [SUBSTRATE: ABSENT — greenfield, research/utilities.md §E2 heating_plant_big.ini:3-25] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0132`

**Sources:**
- `spec/heating.md:16-18`

**Status:** pending

## STORY-0117

**Epic:** EPIC-031 — Heating — district network with temperature-driven demand
**Title:** Third pipe network: trunk-and-branch district heating

**As a** planner
**I want** a heat pipe network distinct from water and sewage, with a large-gauge trunk tier and small-gauge branch tier, routed plant to pumping station to endstation to buildings
**So that** heat distribution has real topology and per-km loss like electricity and water

**Acceptance criteria:**
- AC-1: Heat pipes form a graph independent of the water and sewage pipe graphs — a building wired for heat is not automatically wired for water. [SUBSTRATE: ABSENT — greenfield; ElectricityCache's union-find is a template to clone per-utility, not shared state (docs/egregoria-substrate-audit.md §6)] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0139`
- AC-2: Heat delivered at the end of a branch pipe is reduced from the amount injected at the trunk in proportion to pipe length, mirroring the capacity+loss model used for electricity and water. [SUBSTRATE: OURS/ABSENT — greenfield] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0139`
- AC-3: A pumping station that is unpowered or unstaffed (offline) delivers no heat past it even when downstream pipe capacity is otherwise available — heat must be actively pumped, so an inoperative pumping station is a distinct failure point, not a passive conduit that merely adds length-loss. [SUBSTRATE: ABSENT — greenfield, spec/heating.md:22 $ENGINE_SPEED] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0139`

**Sources:**
- `spec/heating.md:20-24`

**Status:** pending

## STORY-0118

**Epic:** EPIC-031 — Heating — district network with temperature-driven demand
**Title:** Heat demand scales continuously with outdoor temperature

**As a** planner
**I want** a building's heat demand to scale as (20 minus outdoor temperature) times a constant, clamped 0-400%, with no separate winter mode
**So that** cold snaps produce a real, continuous load spike instead of a seasonal flag flip

**Acceptance criteria:**
- AC-1: Building heat demand at outdoor temperature T equals base demand times clamp((20-T) x k, 0, 4) — this requires a weather/climate system providing T(t), which does not exist anywhere in simulation/src today; this story is blocked on that prerequisite landing first. [SUBSTRATE: ABSENT — greenfield, no weather or climate system exists in simulation/src (docs/egregoria-substrate-audit.md §6, 'hidden prerequisite')] · impact:`journey` · seam:`integration` · scenario:`SCENARIO-0130`
- AC-2: All inhabited and workplace buildings have temperature-driven heat load by default; only explicitly unheated structures (sheds, storage) opt out — inverting W&R's opt-in default to match the cold-climate setting. [SUBSTRATE: ABSENT — greenfield] · impact:`local` · seam:`integration` · scenario:`SCENARIO-0130`

**Sources:**
- `spec/heating.md:26-36`

**Status:** pending

## STORY-0119

**Epic:** EPIC-031 — Heating — district network with temperature-driven demand
**Title:** Unmet district heat falls back to electricity

**As a** planner
**I want** a building short on piped heat to draw extra electricity as an expensive electric-heater fallback, and only go cold if neither pipe heat nor spare electricity is available
**So that** heating failure degrades gracefully instead of jumping straight to an unmet need

**Acceptance criteria:**
- AC-1: A building whose heat pipe delivery falls short of its temperature-driven demand draws the shortfall as additional electricity consumption from the building's power connection, provided the electricity network has spare capacity. [SUBSTRATE: ABSENT — greenfield, depends on both the heat network and the rebuilt electricity network stories] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0131`
- AC-2: A building with neither piped heat nor spare electricity registers its warmth need as unmet, with consequences flowing into the citizen needs/health model rather than a silent no-op. [SUBSTRATE: ABSENT — greenfield, depends on spec/needs.md warmth need which is itself unmodelled today] · impact:`journey` · seam:`integration` · scenario:`SCENARIO-0131`

**Sources:**
- `spec/heating.md:30-32`

**Status:** pending