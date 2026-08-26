# Specification register

**Kind:** reference
**Authority:** operational
**Status:** active
**Owner:** project lead
**Last verified:** 2026-08-24

This directory is the canonical mechanism layer for the Rust/Egregoria fork. A specification with
`Authority: binding` constrains implementation only when its status is `active`. A `draft` records
the proposed contract for review; it cannot establish completion or override the charter.

## Authority and evidence

The [1.0 charter](../../plan/charter-1.0.md) controls scope, the
[glossary](../glossary.md) controls terminology, current code plus the cited
[substrate map](../architecture/substrate.md) controls claims about what exists, and `br` controls
task state. A specification controls only the target mechanism inside charter scope.

Each specification separates:

1. **Binding target** — stable `SPEC-SUBSYSTEM-NNN` claims proposed or ratified here.
2. **Current substrate** — descriptive claims cited to a current fact-sheet anchor or heading.
3. **Deferred behavior** — charter-listed Post-1.0 direction with no 1.0 acceptance criteria.
4. **Acceptance evidence** — executable or player-visible proof required before completion.
5. **Open questions** — unresolved choices that create no implementation claim.

Each state transition has one authoritative module. A specification may reference another module's
identifier and result through its interface, but it must not define a parallel authoritative copy
of custody, route, fleet, pressure, consumption, or settlement state.

| State or transition | Authoritative module |
|---|---|
| Durable demand and its unmet outcome | Requesting module: Needs, Production, or Trade |
| Catalogue identity and on-hand stock | Resources |
| Fulfillment allocation, reservation, pickup, in-transit custody, delivery, and physical return | Logistics |
| Vehicle identity, capacity, location, owner/depot, and recovery state | Vehicles |
| Road topology and physical parking-slot reservations | Roads |
| Route request and result | Pathfinding |
| Dynamic load, queue, pressure, and stall | Traffic |
| Industrial consumption and production | Production |
| Dwelling consumption and satisfaction | Needs |
| Customs clearance and rouble settlement | Trade |

External observations use `CONFIRMED`, `OBSERVED`, `INFERRED`, or `SPECULATIVE` only as provenance
labels. `OURS` marks a deliberate project proposal. None of those labels makes comparison evidence
binding or proves current fork behavior.

## Claim anchors and lifecycle

- Allocate anchors monotonically within a file; never reuse a retired identifier for new meaning.
- Link requirements and scenarios to anchors, not mutable line numbers.
- Cite current runtime behavior through a Wave fact-sheet claim plus its source location.
- Move from `draft` to `active` only after ordered review findings are fixed, accepted, or filed.
- Mark a replaced specification `superseded` and link its active successor; archive historical
  inputs without granting them current authority.

## Controlled rewrite register

| Cluster | Draft specifications |
|---|---|
| Physical economy | [Resources](resources.md), [Production](production.md), [Logistics](logistics.md), [Vehicles](vehicles.md), [Trade](trade.md) |
| Needs and movement | [Needs](needs.md), [Roads](roads.md), [Pathfinding](pathfinding.md), [Traffic](traffic.md) |
| Built world | [Construction](construction.md), [Buildings](buildings.md), [Zoning](zoning.md) |
| Settlement | [Households](households.md), [Citizens](citizens.md), [Education](education.md), [Healthcare](healthcare.md), [Crime](crime.md) |
| Utilities | [Electricity](electricity.md), [Water](water.md), [Sewage](sewage.md), [Heating](heating.md), [Waste](waste.md) |

Every listed specification is currently `draft`, not ratified. Their archived legacy
specification counterparts are provenance only and are not current mechanism authority. See
[`docs/archive/legacy-specifications/`](../../archive/legacy-specifications/) when original
rewrite inputs must be inspected.
