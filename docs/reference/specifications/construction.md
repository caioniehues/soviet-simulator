# Construction specification

**Kind:** specification
**Authority:** binding
**Status:** draft
**Owner:** construction
**Last verified:** 2026-08-24

The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT, RECOMMENDED, NOT
RECOMMENDED, MAY, and OPTIONAL in this document are to be interpreted as described in RFC 2119 and
RFC 8174.

## Purpose

Construction turns a Planner-approved placement into a physical, non-operating Site and then into
one completed building. It implements the charter's Ghost, Verdict, material-bill, rescind, and
physical-causality commitments without making domestic roubles, elapsed time, or a UI preview
evidence of construction.

## Scope and exclusions

This specification covers the shared Ghost/Verdict path for buildings, the Site lifecycle,
physical material delivery, work gates, rescind before ground is broken, and the completion result
consumed by Buildings. It does not define road construction, demolition, renovation, construction
office fleets, vehicle manufacture, fuel lifecycle, detailed labour policy, or a domestic price.
Those are not silently supplied by this draft.

## Invariants

- `SPEC-CONSTRUCTION-001` — Construction SHALL own exactly one proposal record containing a
  Ghost, requested building declaration, location, material bill, and one Verdict. The Ghost MUST
  show the footprint, full material bill, and refusal reason before commit. The same proposal and
  Verdict path MUST be re-evaluated at commit; a UI preview alone MUST NOT authorize placement.
- `SPEC-CONSTRUCTION-002` — An approved proposal creates one non-operating Site, not a completed
  building. Construction owns Site state, bill requirements, received-material records, work-gate
  progress, ground-broken state, and completion result. Map owns completed-building topology and
  MUST NOT become a parallel Site owner.
- `SPEC-CONSTRUCTION-003` — A material bill is a quantity of Resources catalogue identities and
  units, never a domestic rouble price. Resources is the sole on-hand-balance mutator; Logistics
  is the sole authority for reservation, pickup, custody, and delivery. A `DeliveryID` is immutable
  and may be accepted at most once, only by its named Site. On acceptance, Resources atomically
  performs `C_haul(i) -= q` and `H_site(i) += q` for each item `i`; Construction records the
  Resources-owned receipt result and MUST NOT create, debit, retarget, replay, or teleport
  materials.
- `SPEC-CONSTRUCTION-004` — A Site becomes ground broken only when its first positive material
  delivery is accepted or its first positive required work is recorded. Before that threshold,
  rescind MUST cancel the proposal/Site, release outstanding goods reservations and vehicle
  assignments through Logistics, and leave undelivered stock and vehicles physically where they
  are. After ground is broken, rescind is refused with the recorded physical reason; it is not a
  general undo or deletion path.
- `SPEC-CONSTRUCTION-005` — A work gate SHALL record its required received material quantities,
  required work, completed work, and binding constraint. Work progresses only while its required
  physical inputs and eligible assigned work are present. Missing material, labour, access, or
  utility prerequisite produces an observable stalled Site; elapsed ticks alone MUST NOT complete
  a gate.
- `SPEC-CONSTRUCTION-006` — Completion occurs only after every required bill quantity and work
  gate is satisfied. Construction emits one completion result keyed to Site ID; Buildings consumes
  it once to request Map topology materialization and operating activation. No dwelling, company,
  service, storage, or soul MAY become active before that result is accepted.
- `SPEC-CONSTRUCTION-007` — A refused proposal, stalled Site, partial delivery, and post-ground-
  broken rescind refusal preserve their state and reason for Planner inspection. Scarcity becomes
  a queue or incomplete Site, never a rouble shortcut or game-over condition.
- `SPEC-CONSTRUCTION-008` — For every item `i`, construction conservation SHALL hold:
  `ΣH_onhand(i) + ΣC_haul(i) + ΣC_embedded(i) = initial(i) + declared_sources(i) -
  declared_other_sinks(i)`. Reservations are non-additive encumbrances and are excluded from this
  equation. Each immutable, Site-keyed `EmbedID` is accepted once only; Resources atomically
  performs `H_site(i) -= q` and `C_embedded(i) += q` while Construction records the embed result.
  Completion requires `Σreceipts(i) - Σembeds(i) = remaining(i) >= 0` for each item and the full
  required bill/work gates in `SPEC-CONSTRUCTION-006`; received material MUST NOT disappear at
  completion.

## Model and authority

`Construction` is the sole authority for proposal, Verdict, Ghost, Site, material acceptance,
work progress, ground-broken state, and completion result. `Resources` owns catalogue and on-hand
balances. `Logistics` owns all physical material movement and cancellation/recovery. `Buildings`
owns declared capabilities and operating state after it accepts the Construction result. `Map`
owns completed building existence, footprint/topology, and its attached geometry; it receives no
authority to activate a Site early. References are IDs and immutable results, never copied state.
Construction's per-Site receipt and embed ledgers retain immutable `DeliveryID`/`EmbedID` and the
corresponding Resources transaction IDs, not duplicate quantity balances or custody state.

The Site lifecycle is `proposed -> approved -> awaiting-material/work -> ground-broken ->
complete-result -> activated`, with a pre-ground-broken `rescinded` terminal branch. A refusal is
not a Site. A completed Site cannot be re-completed under the same Site ID.

## Failure and observability

The Planner can inspect a Ghost's footprint, bill, Verdict and refusal reason; a Site's received
and remaining material by identity, work-gate progress, binding constraint, ground-broken flag,
delivery IDs, age, and rescind disposition. A disconnected source, unavailable vehicle, shortage,
or missing work capacity leaves the Site waiting with that reason. No failure creates a completed
asset, domestic roubles, or unaccounted stock.

## Acceptance evidence

All guards are **UNIMPLEMENTED** and block ratification. A zero-test command is failure, never
green. The serial command below is intentional because the current suite has a known init race.

| Evidence | Command | Observable assertion | Required red mutation | Player-facing proof |
|---|---|---|---|---|
| `EVID-CONSTRUCTION-001` | `cargo test -p simulation evid_construction_ghost_verdict_commit_revalidation -- --test-threads=1` | The identical proposal produces an inspectable Ghost/Verdict and commit refuses when a physical condition changes after preview. | Skip commit revalidation or materialize from a stale preview. | Inspected Ghost and refusal capture. |
| `EVID-CONSTRUCTION-002` | `cargo test -p simulation evid_construction_delivery_receipt_embed_conservation -- --test-threads=1` | An immutable `DeliveryID` is accepted once only by its named Site through the Resources-owned `C_haul -> H_site` receipt; a unique Site `EmbedID` performs `H_site -> C_embedded` once; every item satisfies `ΣH + ΣC_haul + ΣC_embedded` conservation with reservations excluded, and completion retains any remaining receipt. | Replay one `DeliveryID`, accept it at a second Site, omit the haul debit, duplicate an `EmbedID`, or complete by deleting received material. | Inspected per-item source, haul, Site receipt, embedded-material, remaining-material, and reservation capture. |
| `EVID-CONSTRUCTION-003` | `cargo test -p simulation evid_construction_ground_broken_rescind -- --test-threads=1` | Pre-threshold rescind releases reservation/vehicle through Logistics; first material or work blocks rescind without deleting the Site. | Allow rescind after first positive receipt/work or reset the vehicle location. | Inspected rescind and recovery capture. |
| `EVID-CONSTRUCTION-004` | `cargo test -p simulation evid_construction_work_gates_do_not_complete_on_timer -- --test-threads=1` | Missing input or work stalls a gate across elapsed ticks; valid physical inputs and work advance it. | Increment completion from tick count despite a missing gate. | Inspected stalled-gate timeline. |
| `EVID-CONSTRUCTION-005` | `cargo test -p simulation evid_construction_completion_precedes_activation -- --test-threads=1` | No capability/soul exists before all bill and work gates complete; one completion result activates once. | Spawn a soul at Site creation or activate before final gate. | Inspected Site-to-building timeline. |
| `EVID-CONSTRUCTION-006` | `cargo test -p simulation evid_construction_failure_preserves_site_reason_and_plan -- --test-threads=1` | Refusal, stalled Site, partial delivery, and post-ground-broken rescind refusal each retain their record and physical reason; none creates domestic roubles or a game-over state. | Discard a refusal/stall reason, erase a partial receipt, allow post-ground-broken deletion, or credit roubles on failure. | Inspected refusal, stalled Site, partial-receipt, and rescind-refusal capture. |
| `EVID-CONSTRUCTION-007` | `cargo test -p simulation evid_construction_site_authority_nonoperating_transition -- --test-threads=1` | One approved proposal creates exactly one non-operating Construction-owned Site; only Construction advances its Site state, bill requirements, received-material records, work-gate progress, ground-broken state, and one completion result, while Map materializes completed-building topology only after Buildings accepts that result. | Let Map create, advance, complete, or retain a parallel Site; let Buildings activate a proposal/Site before its Construction completion result; duplicate the Site or completion result for one approved proposal; or let a Site become operating on creation. | Inspected proposal-to-Site identity, non-operating state, Site transition audit, sole completion result, and post-acceptance Map topology capture. |

## Current substrate and conflict

The current command application immediately debits `Government.money` before dispatching every
command (`simulation/src/world_command.rs:223-225`), and computes domestic house, zone, and
special-building prices in bucks (`simulation/src/economy/government.rs:21-75`). This conflicts
with the charter's domestic non-price rule; it is a displacement target, not a construction
mechanism. `MapBuildSpecialBuilding` immediately calls `Map::build_special_building` and inserts
`BuildingInfos` (`simulation/src/world_command.rs:284-299`), while `Map` materializes the building
after only overlap rejection (`simulation/src/map/map.rs:245-297`). Neither path has Site, bill,
delivery, work, or ground-broken state; `Building` contains only geometry/kind/zone/road fields
(`simulation/src/map/objects/building.rs:70-159`).

The special-building tool offers road, sidewalk, endpoint, and overlap checks in its UI
(`native_app/src/gui/tools/specialbuilding.rs:111-185`), but that is not the commit-time Verdict.
The scheduler creates souls for ownerless houses, companies, and freight stations
(`simulation/src/souls/mod.rs:16-54`), proving the present immediate-activation conflict. These
facts match the current [Wave 2 substrate fact-sheet](../../research/fact-sheets/wave2-substrate.md#2a--built-world-construction-buildings-and-zoning);
they do not prove the target contract.

## Deferred behavior

Road construction, demolition, refurbishment, construction-office dispatch, worker skill
taxonomy, and asset-progress rendering are deferred. They cannot be used to bypass the Site
lifecycle or material conservation above.

## Open questions

- Which building declarations determine bill quantities and work gates?
- Which named work authority supplies eligible labour and equipment without duplicating Logistics?
- Which physical prerequisites belong in a Verdict for each building class?
