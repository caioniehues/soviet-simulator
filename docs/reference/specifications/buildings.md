# Buildings specification

**Kind:** specification
**Authority:** binding
**Status:** draft
**Owner:** buildings
**Last verified:** 2026-08-24

The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT, RECOMMENDED, NOT
RECOMMENDED, MAY, and OPTIONAL in this document are to be interpreted as described in RFC 2119 and
RFC 8174.

## Purpose

Buildings defines a completed asset's declared capabilities and its operating eligibility. A
building exists for simulation purposes only after Construction completes its physical Site; a
placed order, domestic payment, prototype declaration, or rendered mesh never makes it operate.

## Scope and exclusions

This specification covers building declarations, completed-asset identity, capability exposure,
activation, and inactive/stalled observability. It does not define placement Verdicts, material
delivery, work progression, zoning policy, households, service mechanics, maintenance,
demolition, or a domestic price. Prototype/Lua declarations are inputs, not proof that an
operating building exists.

## Invariants

- `SPEC-BUILDINGS-001` — Buildings SHALL own the immutable declaration and operating state of a
  completed asset: kind, declared capacity/connection requirements, capability identifiers, and
  activation disposition. Map solely owns its completed footprint, topology, and attached geometry;
  Buildings MUST NOT maintain a second topology or placement authority.
- `SPEC-BUILDINGS-002` — Buildings accepts exactly one Construction completion result for a Site
  ID. Only then may it request Map materialization and mark the resulting asset operating. It MUST
  reject duplicate, absent, incomplete, or mismatched completion results.
- `SPEC-BUILDINGS-003` — Before activation, declared dwelling, production, storage, service, and
  utility-facing capabilities are unavailable. No soul, company, resident, inventory, production,
  service queue, or utility consumer may be created merely because a Ghost, Site, mesh, or
  prototype exists.
- `SPEC-BUILDINGS-004` — Activation publishes only declared capability references. The subsystem
  that owns each capability's runtime state—such as Production, Needs, or a future utility—owns
  its own demand, consumption, queue, or flow and MUST NOT be replaced by Building state.
- `SPEC-BUILDINGS-005` — A completed but prerequisite-blocked asset remains an observable
  non-operating or limited-capability building with a physical reason. It does not create stock,
  clear domestic demand by price, or terminate the plan.
- `SPEC-BUILDINGS-006` — The Planner can inspect declaration identity, associated Site and
  completion result, activation state, active/inactive capabilities, and each unavailable
  capability's reason. UI representation is a consumer of this state, not an alternate authority.

## Model and authority

Buildings owns `BuildingDeclaration`, `CompletedAsset`, and `OperatingState`. Construction owns
the preceding Site lifecycle and emits an immutable completion result. Map creates and owns only
the completed spatial building after Buildings accepts that result. Zoning owns Planner intent;
it may be consulted by Construction's Verdict but cannot activate an asset. The activation path is
`Construction completion result -> Buildings acceptance -> Map materialization -> Buildings
operating state -> named capability consumers`. Each arrow is one-way and keyed by Site/asset ID.

## Failure and observability

If the completion result is absent, duplicated, or inconsistent with its declaration, activation
is refused and the completed Site remains visible through Construction. If an active capability
lacks its own physical prerequisite, the asset exposes that shortage rather than inventing an
output or domestic purchase. The Planner sees the asset's status and exact inactive reason.

## Acceptance evidence

All guards are **UNIMPLEMENTED** and block ratification. A command that executes zero tests is a
failure, never green.

| Evidence | Command | Observable assertion | Required red mutation | Player-facing proof |
|---|---|---|---|---|
| `EVID-BUILDINGS-001` | `cargo test -p simulation evid_buildings_completion_result_activates_once -- --test-threads=1` | One valid Construction result materializes one asset and duplicate results do not duplicate topology or activation. | Accept the same Site completion twice. | Inspected Site/asset activation capture. |
| `EVID-BUILDINGS-002` | `cargo test -p simulation evid_buildings_no_capability_before_completion -- --test-threads=1` | A Ghost/Site/prototype has no active capability or soul until completion is accepted. | Spawn a company or resident at Site creation. | Inspected inactive-capability timeline. |
| `EVID-BUILDINGS-003` | `cargo test -p simulation evid_buildings_blocked_capability_is_observable -- --test-threads=1` | A missing physical prerequisite keeps the named capability inactive with its reason and preserves demand/stock. | Produce an output or clear demand while the prerequisite is absent. | Inspected building capability inspector. |
| `EVID-BUILDINGS-004` | `cargo test -p simulation evid_buildings_singular_capability_authority_and_inspector -- --test-threads=1` | The asset keeps one declaration and one operating-state reference; capability consumers retain their own demand/stock/flow state, and the inspector exposes declaration, completion result, status, capabilities, and reasons. | Copy topology into Buildings, mutate demand/stock from Buildings, or omit a required inspector field. | Inspected declaration-to-capability authority capture. |

## Current substrate and conflict

Current `BuildingKind` has only House, goods company, freight/train station, and external-trading
variants (`simulation/src/map/objects/building.rs:17-37`); `Building` itself stores geometry, kind,
zone, and connected road but no declaration, Site, activation, or capability state
(`simulation/src/map/objects/building.rs:70-159`). `Map::build_special_building` immediately calls
`Building::make`, attaches the building to topology, and returns its ID
(`simulation/src/map/map.rs:245-297`; `simulation/src/map/objects/building.rs:117-159`).

`BuildingInfos` is inserted immediately by the command path
(`simulation/src/world_command.rs:284-299`), then ownerless buildings are used to create human,
company, or freight-station souls (`simulation/src/souls/mod.rs:16-54`). This is current substrate
and conflicts with `SPEC-BUILDINGS-002` and `SPEC-BUILDINGS-003`; it is not partial evidence of
the target. Prototype fields parse size, generator, asset, price, and optional power
(`prototypes/src/prototypes/building.rs:25-54`), which is declaration data only. The current
classification and save/UI limits are recorded in the [Wave 2 fact-sheet](../../research/fact-sheets/wave2-substrate.md#2a--built-world-construction-buildings-and-zoning).

## Deferred behavior

Building condition, maintenance, refurbishment, demolition, detailed capacity semantics, and
specific household/service/utility contracts are deferred to their ratified specifications.

## Open questions

- Which declaration fields are mandatory for each 1.0 building class?
- Which capability prerequisites are construction gates versus post-completion operating gates?
- How will completed-asset identity survive replacement or renovation without duplicate topology?
