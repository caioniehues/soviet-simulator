# 1.0 scope

**Kind:** plan
**Authority:** advisory portal — the [charter](../plan/charter-1.0.md) is binding; this page summarises it and must never replace it
**Status:** active
**Owner:** project lead
**Last verified:** 2026-09-03

The one page that keeps long-term ambition and the current release from being confused. Where
this page and the charter differ, the charter is right.

## The commitments (charter §1.0 scope)

| Area | 1.0 commitment | Design page | Spec |
|---|---|---|---|
| Resources and production | Fifteen domestic resources and twelve new recipe buildings; Food and Meat are separate dwelling needs; Water is a utility, never cargo; Medicine is a sixteenth, import-only resource | [resources](../simulation/physical-economy/resources.md), [production](../simulation/physical-economy/production.md) | resources, production |
| Agriculture and services | Field-cycle farming, livestock conversion, demographics including death, two education tiers, healthcare, landfill, incinerator | [demography](../simulation/society/demography.md), [education](../simulation/society/education.md), [healthcare](../simulation/society/healthcare.md), [waste](../simulation/infrastructure/waste.md) | education, healthcare, waste; **agriculture has no spec** |
| Planner interaction | Snapping and rotation; a ghost with footprint, material bill and refusal; one placement verdict; rescind before ground is broken; selection and inspect-depth feedback; tooltips, toolbar icons, camera polish; no general undo | [construction](../simulation/physical-economy/construction.md) | construction, buildings, zoning |
| Terrain and environment | Procedural seed maps, heightfield terrain, reservoir-graph water, hydro dams, ore siting, minimal bridges, pollution coupled to sickness, crop yield and basin water | [hydrology](../simulation/infrastructure/hydrology.md) | **terrain, hydrology, pollution have no spec** |
| Transport and border | Minimal freight rail (three buildings, one locomotive, one wagon); fixed-consist border purchase; multiple customs offices; all sixteen resources tradable at fixed per-kind prices in one rouble; one fixed 1950s–60s era | [freight rail](../simulation/transport/freight-rail.md), [logistics](../simulation/physical-economy/logistics.md) | vehicles, trade, logistics, roads, pathfinding, traffic |
| Plans and onboarding | Three authored Plans on one continuous save, then procedural endless mode; the First Plan alone teaches a new player for two hours | [the Planner](player-role.md), [plan cycle](../simulation/planned-economy/plan-cycle.md) | **Plan/Quota/Tranche and onboarding have no spec** |
| Shell and comfort | Main menu; named saves plus three rotating period-end autosaves; pause, date, speed; minimal settings; action-needed notifications and event log; camera polish; HUD onboarding strip; panic log and autosave-on-crash | — | **no spec** |
| Presentation and audio | Zero-spend art and audio; grounded palette; bounded visible citizens; day/night and visible seasons; legible state and refusal feedback; ambience, optional menu music | [art direction](../reference/art-direction.md) | **no spec** |
| Distribution | English-only, fixed keybindings, no telemetry, CI-built Linux and Windows binaries as an unlisted itch build; friends-grade shell, stranger-grade visuals and feel | [dependency policy](../process/dependency-policy.md) | — |
| Households and citizens | Persistent citizen identities grouped into households; residence assignment with a housing queue and an observable housing shortage; household consumption with explicit going without. Housing tiers (kommunalka to separate flat), propiska and household time budgets are Post-1.0. | [households](../simulation/society/households.md), [citizens](../simulation/society/citizens.md) | households, citizens, needs |
| Utilities | Electricity, water, heating and waste as connected finite-rate networks with distinct inertia; Water is a utility, never cargo, and includes static head and tank storage. Sewage is Post-1.0. | [infrastructure](../simulation/infrastructure/index.md) | electricity, water, heating, waste |

The last two rows were added to the charter on 2026-09-03 by
[ADR-0001](../decisions/0001-households-and-utilities-are-1.0-scope.md), which also cut sewage.

**Performance target:** 250,000 citizen identities at 60 fps on the development machine. The
charter names no benchmark gates; the implementation plan must define them. No benchmark exists
today and the 250k benchmark lane (`sov-1ae`) was cancelled 2026-08-27
([performance](../architecture/performance.md)).

**Saves:** explicit version-gated hard breaks are allowed during development; from the 1.0 release
candidate onward, released saves stay compatible ([persistence](../architecture/persistence.md)).

## The cuts (charter §Explicit cuts) — Post-1.0, cannot receive 1.0 acceptance criteria

Loyalty, legitimacy, broadcast, monuments, crime, vehicle manufacture and fuel lifecycle; voltage
tiers and grid depth (transformers, treatment tiers, CHP); electric-heating fallback; passenger
rail, signals, electrification; ships, docks, pipelines, sewage, cableways, containers, aircraft,
petrochemicals; era calendar, dual currency, free terraform, cell-level water, kindergarten,
deathcare, epidemics, perishables, refrigerated transport, Steam, marketing.

**Never:** tourism, hotels, attractions; fires and disasters — scarcity, not random destruction,
is the pressure source.

## Scope discipline

An in-scope system cannot be marked implemented from a legacy document, a generated roadmap, a
handoff or an old ADR. Completion needs current implementation evidence plus the acceptance
evidence its specification requires. A feature outside the charter belongs in
[Post-1.0](post-1.0.md) or a charter revision, never in a 1.0 requirement by implication.

## Where 1.0 stands today

Every specification is `draft`; every planned `EVID-*` target (107) is unimplemented
([generated roadmap](../generated/roadmap.md)); the code's working core is the dishonest
enterprise's request inflation and a physical truck leg with ledger tests
([current substrate](../architecture/current-substrate.md)). Task state is in `bd`.

## Related

- [Charter](../plan/charter-1.0.md) — binding
- [Post-1.0](post-1.0.md)
- [ADR-0001](../decisions/0001-households-and-utilities-are-1.0-scope.md) — Households and Utilities are charter rows
- [Missing specifications](../research/conversation-mining-2026-08-28/SYNTHESIS.md#7-open-questions-for-the-planner--consolidated)
- [Migration sequence](../architecture/migration-sequence.md)
