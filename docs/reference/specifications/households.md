# Households specification

**Kind:** specification
**Authority:** binding
**Status:** draft
**Owner:** settlement
**Last verified:** 2026-08-24

The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT, RECOMMENDED, NOT
RECOMMENDED, MAY, and OPTIONAL in this document are to be interpreted as described in RFC 2119
and RFC 8174.

## Purpose

Households make residence, shared dwelling provision, and housing shortage legible without
turning either homes or people into market accounts. This is in scope under the charter's
[agriculture and services commitment](../../plan/charter-1.0.md#10-scope) and its persistent
identity pillar. It is a target contract, not a claim that current code provides households.

## Scope and exclusions

This specification covers household membership, residence assignment, a shared Food/Meat pantry
point of use, and an observable housing queue. Needs owns dwelling consumption and satisfaction;
Resources alone mutates on-hand balances; Logistics owns physical fulfillment. Construction,
building capacity, births, emigration, vehicles, rent, domestic prices, and criminal outcomes do
not receive mechanisms here. Domestic housing is allocated by non-price policy, never purchased.

## Invariants

- `SPEC-HOUSEHOLDS-001` — A Household has a persistent ID and an explicit member set of persistent
  Citizen IDs. A citizen belongs to at most one household at a time; a building, Market account,
  or rendered occupant list MUST NOT substitute for household identity.
- `SPEC-HOUSEHOLDS-002` — Household owns residence assignment and housing-queue entry. A queued,
  displaced, or overcrowded household remains observable with its members intact; shortage MUST
  NOT delete people, dissolve demand, or end the plan.
- `SPEC-HOUSEHOLDS-003` — A residence assignment consumes declared dwelling capacity only under
  the future Buildings authority. Household records its assignment and queue reason but MUST NOT
  mutate building topology or duplicate capacity state.
- `SPEC-HOUSEHOLDS-004` — Each household has distinct shared Food and Meat pantry records at its
  permitted dwelling point of use. The only physical chain is completed Logistics fulfillment →
  Resources-owned compatible on-hand receipt at that point of use → named Needs consumption
  event; these are separate, authoritative transactions.
- `SPEC-HOUSEHOLDS-005` — A household pantry MUST NOT be credited by request, allocation,
  reservation, payment, route start, or a citizen's arrival alone. Food and Meat remain distinct
  under `SPEC-NEEDS-001`; a household record cannot satisfy either need before Needs records
  authoritative consumption.
- `SPEC-HOUSEHOLDS-006` — Housing priority is a Planner policy over observable queue attributes
  such as age, displacement, size, and declared fit. It MUST NOT rank or clear by domestic
  roubles or a price, and a partial allocation leaves the unassigned household queued.
- `SPEC-HOUSEHOLDS-007` — A completed fulfillment is immutable as `(FulfillmentID,
  destination-holder, item, quantity)` as a Logistics delivery result. Logistics alone performs
  the haul-custody-to-destination delivery transition and emits that result. Resources atomically
  accepts the immutable result once only when destination and item match, and mutates only the
  named destination's on-hand balance by `H_destination(item) += q`; it MUST NOT mutate haul
  custody. Replay, retargeting to another pantry, and duplicate receipt are no-ops. Households
  references the accepted receipt only. The resulting Food or Meat receipt may feed only its
  matching Needs event, preserving its partial remainder and independence from the other kind.
- `SPEC-HOUSEHOLDS-008` — Households consumes one immutable Citizens `DeathResultID` once to remove
  its named deceased Citizen ID from membership. Replayed results are no-ops; the transition MUST
  NOT remove another member, delete the household, or duplicate Citizen lifecycle state. When the
  final member dies, the household enters explicit `EmptyAfterDeath` state, retaining household ID
  and audit history while its remaining residence/pantry/need references follow their owners'
  recovery rules. Deathcare is neither required nor implied.

## Model and state

Households is the sole authority for household identity, membership, residence-assignment intent,
housing queue state, and pantry point-of-use composition. Each household records ID, members,
assigned dwelling or queue state, queue age/reason, and Food/Meat pantry references. Needs owns
the per-need record, consumption event, satisfaction, and going-without result. Resources owns
every stock balance and balance mutation. Logistics owns reservation, pickup, custody, delivery,
and recovery, including the completed immutable delivery result. Resources owns the one accepted
receipt ID and named destination on-hand mutation under that result; Households may reference the
receipt but never accept a delivery, mutate custody, or mutate stock itself. Buildings will own
dwelling capacity; Citizens owns individual identity, lifecycle, and individual itinerary. A
membership-change record references one `DeathResultID`; Households alone applies the membership
removal and records `EmptyAfterDeath` when applicable, while Citizens retains death lifecycle/audit
state. These references are interfaces, not duplicated state.

## Failure behavior

No capacity, incompatible pantry delivery, missing stock, route, vehicle, or worker leaves the
household queued or its need waiting with the owning module's reason. Displacement re-enters the
housing queue and retains household identity. Failed provision never creates pantry stock or
silently removes Food/Meat demand; Needs records waiting, approved substitution where policy
allows it, or going without.
After death, the named member is removed once through the Citizens result; surviving members and
the household remain intact. A last-member household becomes `EmptyAfterDeath` rather than
silently disappearing, and no deathcare absence can block that recorded lifecycle transition.

## Observability

The Planner can inspect household ID, members, dwelling or queue state, queue age/reason,
declared capacity fit, Food and Meat pantry references, and links to outstanding needs and
fulfillments. It can also inspect each consumed `DeathResultID`, one membership-removal outcome,
surviving members, and `EmptyAfterDeath` state. Aggregate housing shortages cannot replace these
per-household records.

## Acceptance evidence

All guards are **UNIMPLEMENTED** and block ratification. A command that runs zero tests is a
failure, never green. The current 26-test suite proves no target below.

| Evidence | Future guard command and observable assertion | Required red mutation | Player-facing proof |
|---|---|---|---|
| `EVID-HOUSEHOLDS-001` | `cargo test -p simulation evid_households_identity_queue -- --test-threads=1` — an unassigned/displaced household retains its ID and members while queue age increases. | Delete the household or a member when no dwelling is available. | Inspected household queue and member view. |
| `EVID-HOUSEHOLDS-002` | `cargo test -p simulation evid_households_pantry_needs_boundary -- --test-threads=1` — for Food and Meat independently, only a completed immutable `(FulfillmentID, destination-holder, item, quantity)` Logistics delivery result can precede one matching Resources-owned pantry receipt. Logistics alone performs the haul-custody-to-destination transition; Resources accepts that result once and adds `q` only to the named pantry's on-hand balance. Only named Needs consumption can satisfy. Request, allocation, reservation, payment, route start, or arrival alone changes neither pantry balance nor satisfaction; replay/retarget/duplicate receipt is a no-op and partial remainder stays with its named kind. | Let Resources mutate haul custody; credit pantry or satisfy a need on request, allocation, reservation, payment, route start, or arrival; replay one delivery result into two pantries, retarget it to another household, duplicate its Resources receipt, or bypass completed Logistics delivery/Resources receipt before Needs consumption. | Inspected Logistics delivery result, custody transition, named pantry, one Resources receipt, Food/Meat balances, partial remainder, and consumption capture. |
| `EVID-HOUSEHOLDS-003` | `cargo test -p simulation evid_households_nonprice_capacity -- --test-threads=1` — capacity-limited non-price assignment leaves an unassigned household queued. | Sort by roubles or allocate beyond declared capacity. | Inspected allocation rationale and waiting household. |
| `EVID-HOUSEHOLDS-004` | `cargo test -p simulation evid_households_death_membership_once -- --test-threads=1` — one Citizens DeathResultID removes only its named member once; replay is a no-op, surviving members remain, and the last-member household becomes retained `EmptyAfterDeath` with an audit link. | Apply the result twice, remove a different/surviving member, silently delete the household or its audit, or make deathcare a prerequisite. | Inspected death-result link, membership delta, surviving household, and EmptyAfterDeath capture. |

## Substrate and decisions

Current code has no household ID, membership, shared inventory, or housing queue: `HumanEnt`
stores one home, bread desire, and optional work only (`simulation/src/world.rs:86-104`), and an
empty house spawns one human (`simulation/src/souls/mod.rs:15-55`). Spawn assigns that human as
the building owner (`simulation/src/souls/human.rs:251-272`), while the inspector renders one
owner and current occupants rather than a household (`native_app/src/gui/inspect/inspect_building.rs:96-117`).
These are substrate limitations, not household behavior; see the [Wave 2 fact-sheet,
settlement](../../research/fact-sheets/wave2-substrate.md#2b--settlement-citizens-households-and-services).

## Deferred behavior

Vehicle ownership/manufacture and fuel lifecycle, loyalty, legitimacy, crime, kindergarten,
tourism, rent, and domestic money are outside this 1.0 mechanism. Tourism is Never; the named
charter cuts receive no acceptance criteria here.

## Open questions

- Which declared dwelling-capacity states permit temporary overcrowding, and what need outcome do
  they produce?
- Which Planner-authored non-price queue attributes and tie-breaks are required for 1.0?
- Which lifecycle events may create, merge, split, or retire a household while preserving citizen
  identity and audit history?
