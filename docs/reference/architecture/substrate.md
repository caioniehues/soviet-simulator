# Egregoria substrate architecture

**Kind:** reference
**Authority:** reference
**Status:** active
**Owner:** architecture
**Last verified:** 2026-08-24

This is the current Rust/Egregoria substrate map, not a target design. A classification of
**provided** means the cited behavior has a reachable production path; **partial** means only a
subset is provided; **conflicting** means live paths disagree; **absent** means the cited fact-sheet
found no implementation for the stated contract. Binding target behavior belongs in a ratified
specification.

## Authoritative seam

The primary live seam is:

```text
WorldCommand → command-first serial schedule → authoritative Simulation → presentation consumers
```

`MAP-SUB-001` establishes typed lanes and authoritative road commands; the foundation fact-sheet
establishes that tools read immutable simulation state, queue commands, and presentation consumes
simulation state. The serial schedule is **provided**, but the claim that every mutation enters a
fixed tick is **conflicting**: four instant commands bypass ticking and other commands can force a
tick while paused. [Foundation fact-sheet](../../research/fact-sheets/wave1-substrate.md)

## Runtime and persistence

| Claim | Classification | Evidence |
|---|---|---|
| Nominal time is 50 ticks/second and command handling precedes the serial schedule. | Partial | Foundation contract / Tick; `prototypes/src/types/time.rs:10`; `simulation/src/lib.rs:244-270` |
| Systems run in registration order and commit command buffers between systems. | Provided | Foundation contract / Schedule; `simulation/src/init.rs:52-109` |
| Initialization uses unsynchronised `static mut` registries and is unsafe for parallel test initialization. | Conflicting | Foundation contract / Initialization; `simulation/src/init.rs:111-130` |
| Version mismatch only warns, and failed resource decoding can leave a loaded world with fresh default resources. | Partial/conflicting | Foundation contract / Save-load; `simulation/src/lib.rs:359-448` |
| No cited check proves repeat-run determinism; the current helper proves serialization round-trip stability only. | Absent | Foundation contract / Determinism |

The foundation-contract headings and the `MAP-SUB-*` identifiers are evidence anchors in
[wave1-substrate](../../research/fact-sheets/wave1-substrate.md); file:line citations identify the
observed source. No row promotes an observed behavior into a desired contract.

## Map, routing, and traffic

| Claim | Classification | Evidence claim |
|---|---|---|
| Typed driving, parking, walking, rail, and other lanes support physical movement. | Provided | `MAP-SUB-001` |
| Road construction leaves placement entirely Planner-authored. | Conflicting | `MAP-SUB-002`: roads create roadside lots automatically |
| Vehicle routing responds to congestion, capacity, closures, or freight restrictions. | Partial | `MAP-SUB-003`: routing is static and retry-only |
| Traffic supplies an observable, durable congestion model. | Partial | `MAP-SUB-004`: microscopic collision/signal behavior has no durable ledger or Planner readout |
| Parking spots are exclusive reservable capacity. | Provided | `MAP-SUB-005` |

## Logistics and economy seams

| Claim | Classification | Evidence claim |
|---|---|---|
| A truck can gate source and destination inventory transfers through routed movement. | Provided | `LOG-SUB-002` |
| Vehicles carry no authoritative cargo or capacity state. | Absent | `LOG-SUB-005` |
| Companies retain truck IDs, but global dispatch ignores that ownership. | Conflicting | `LOG-SUB-006` |
| Delivery completion has no return-to-depot behavior, and failed dispatch has no recovery policy. | Absent | `LOG-SUB-008`, `LOG-SUB-009` |
| One delivery authority controls all company and market fulfillment. | Conflicting | `LOG-SUB-007`; also `ECO-SUB-006` |
| Domestic matching is price-free but lacks partial multi-seller fill, request age, and plan priority. | Partial | `ECO-SUB-003` |
| Imports credit stock immediately and exports can debit stock before a border endpoint exists. | Conflicting; economy violation | `ECO-SUB-002` |
| Unmatched demand can be removed instead of persisting as a shortage queue. | Conflicting; economy violation | `ECO-SUB-001` |

The logistics and economy classifications come from [wave1-logistics](../../research/fact-sheets/wave1-logistics.md)
and [wave1-economy](../../research/fact-sheets/wave1-economy.md). They are rewrite constraints,
not an implementation backlog.

## Prototype authority

Lua is not automatically runtime authority. The foundation fact-sheet's prototype-authority rows
for Items, Goods companies, and Rolling stock are **provided** because the exact parsed declarations
have reachable consumers. Solar subtype fields are **partial**; road vehicles and leisure
declarations are **unreachable** or partial; freight stations are **partial** because their cargo
remains unitless counters. See the prototype-authority table in
[wave1-substrate](../../research/fact-sheets/wave1-substrate.md).

Every specification that relies on a prototype field must cite both its parsing location and its
reachable production consumer. A declaration without that chain is data vocabulary, not a live
mechanism.

## Evidence boundary

The three Wave 1 fact-sheets are the current cited source maps. They did not run gameplay,
performance, save-migration, corrupted-save, or mutation validation. Those unperformed checks
must remain unclaimed until a later evidence artifact records their command and result.
