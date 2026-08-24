# Zoning specification

**Kind:** specification
**Authority:** binding
**Status:** draft
**Owner:** zoning
**Last verified:** 2026-08-24

The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT, RECOMMENDED, NOT
RECOMMENDED, MAY, and OPTIONAL in this document are to be interpreted as described in RFC 2119 and
RFC 8174.

## Purpose

Zoning records Planner-authored land-use intent and makes planned mismatch visible. It informs the
shared Construction Ghost/Verdict path but never causes a building, Site, demolition, allocation,
or domestic payment to exist.

## Scope and exclusions

This specification covers land-use intent, its relation to proposal Verdicts, mismatch feedback,
and Planner observability. It excludes automatic development, demand-driven spawning, rezoning
demolition, property prices, household allocation, physical construction, and grid-cell growth
engines. It does not turn current per-building Zone geometry into land-use policy.

## Invariants

- `SPEC-ZONING-001` — Zoning SHALL own Planner-authored land-use intent records, their boundary,
  permitted building classes, revision, and declared mismatch policy. An intent record is neither
  a Site nor a building and has no material, stock, work, soul, or rouble balance.
- `SPEC-ZONING-002` — Construction consults a referenced intent record when producing its Ghost
  and Verdict, and records any mismatch as an explicit approval/refusal reason. Construction
  remains the sole authority for proposal/Verdict and Site creation; Zoning MUST NOT bypass that
  shared path.
- `SPEC-ZONING-003` — Creating, changing, or removing intent MUST NOT spawn, activate, demolish,
  relocate, or alter a building/Site; it MUST NOT reserve materials, assign a vehicle, or debit
  domestic roubles. A later explicit Construction proposal is required for every physical change.
- `SPEC-ZONING-004` — Zoning may expose shortage or coverage indicators as decision support, but
  they are diagnostic inputs, not a queue-clearing price, automatic builder, or completion rule.
  Shortage remains observable until the Planner authorizes a physically feasible proposal.
- `SPEC-ZONING-005` — A changed intent does not erase an existing Site or completed asset. Any
  mismatch is observable and its physical disposition requires the owning Construction or
  Buildings mechanism; no auto-despawn or instant deletion is permitted.
- `SPEC-ZONING-006` — The Planner can inspect intent type/boundary, permitted classes, revision,
  covered proposals/assets, mismatch reason, and unmet indicator. Presentation consumes this
  authoritative intent and cannot mutate building topology directly.

## Model and authority

Zoning owns `LandUseIntent` and its diagnostics. Construction owns the only Ghost/Verdict and
Site transition, Buildings owns completed-asset operating state, and Map owns physical topology.
The only allowed construction-facing exchange is `LandUseIntent ID + revision -> Construction
proposal -> Verdict reason`; the Verdict does not copy zoning state. This keeps a zoned-but-empty
area as a visible plan backlog rather than latent automatic development.

## Failure and observability

An incompatible proposal is refused or approved-with-mismatch according to its recorded policy;
either outcome is visible. An empty zoned area, unmet housing/service indicator, or changed intent
does not end the plan and cannot create a domestic-money shortcut. The Planner sees the condition
needed to make a new proposal feasible.

## Acceptance evidence

All guards are **UNIMPLEMENTED** and block ratification. A command that executes zero tests is a
failure, never green.

| Evidence | Command | Observable assertion | Required red mutation | Player-facing proof |
|---|---|---|---|---|
| `EVID-ZONING-001` | `cargo test -p simulation evid_zoning_intent_never_spawns_or_demolishes -- --test-threads=1` | Create/change/remove intent leaves Site/building count, topology, materials, vehicles, and souls unchanged. | Spawn or delete a building as an intent side effect. | Inspected intent and world-state capture. |
| `EVID-ZONING-002` | `cargo test -p simulation evid_zoning_verdict_records_intent_mismatch -- --test-threads=1` | Construction Ghost/Verdict references the current intent revision and preserves its mismatch reason. | Create a Site without a Verdict or ignore a mismatched revision. | Inspected Ghost/Verdict capture. |
| `EVID-ZONING-003` | `cargo test -p simulation evid_zoning_no_domestic_price_or_auto_clearance -- --test-threads=1` | Intent mutation neither changes domestic roubles nor clears a shortage; only an explicit physical proposal may progress. | Debit money or clear the indicator when zoning changes. | Inspected plan-backlog session. |
| `EVID-ZONING-004` | `cargo test -p simulation evid_zoning_intent_authority_and_inspector -- --test-threads=1` | One authoritative intent retains boundary, permitted classes, revision, and mismatch policy; the Planner inspector exposes those fields plus covered proposals/assets, mismatch, and unmet indicator. | Duplicate intent state in Map, drop a required intent field, or omit it from the inspector. | Inspected intent-authority and inspector capture. |

## Current substrate and conflict

The current `Zone` is geometry attached directly to `Building` (`simulation/src/map/objects/building.rs:46-80`), and `UpdateZone` writes it directly through `Map`
(`simulation/src/world_command.rs:337-340`). Only goods-company prototypes expose an optional zone
declaration (`prototypes/src/prototypes/goods_company.rs:18-44`). Therefore it is not a current
Planner land-use intent or a foundation for automatic growth.

The inherited command path charges `Government.money` for zone edits and other domestic actions
(`simulation/src/world_command.rs:223-225`; `simulation/src/economy/government.rs:21-75`), in
conflict with this target and the charter. Placement already materializes buildings immediately
(`simulation/src/world_command.rs:284-299`), while road/lot UI can queue house build commands
(`native_app/src/gui/tools/lotbrush.rs:56-56`). These are substrate conflicts, not target zoning
behavior. The [Wave 2 fact-sheet](../../research/fact-sheets/wave2-substrate.md#2a--built-world-construction-buildings-and-zoning)
records the same classification and warns that current Zone geometry has no settlement-land-use
owner.

## Deferred behavior

District geometry editing UX, exact permitted-class taxonomy, proposed-project workflows, and
service/housing diagnostic formulas are deferred. They cannot authorize automatic construction or
demolition in 1.0.

## Open questions

- Are intent boundaries polygons, parcels, or another Map-owned geometry reference?
- Which mismatches refuse placement and which merely require Planner acknowledgement?
- Which future household and service indicators belong in the initial diagnostic surface?
