# Egregoria substrate architecture

**Kind:** reference
**Authority:** reference
**Status:** active
**Owner:** architecture
**Last verified:** 2026-08-28 (drift notes added; the per-subsystem narrative now lives in
[`docs/architecture/current-substrate.md`](../../architecture/current-substrate.md), verified at `4e9e930b2a73`)

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
| Initialization uses unsynchronised `static mut` registries and is unsafe for parallel test initialization. | **Stale — fixed 2026-08-26** (`sov-test-race-initfuncs-qt6`): `simulation/src/init.rs` now uses `static REGISTRY: OnceLock<Registry>`; `cargo test -p simulation` is parallel-safe. The same shape survives in `native_app/src/init.rs` (UI crate) | Foundation contract / Initialization (historical); `simulation/src/init.rs` `REGISTRY` |
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
| Imports credit stock immediately and exports can debit stock before a border endpoint exists. | Conflicting; economy violation — **import half fixed** (`sov-abs`: imports are a physical truck from the freight station); **export half still live** (the ext-trade block of `make_trades` debits seller capital at match time, no dispatch) | `ECO-SUB-002` and its 2026-08-28 drift note |
| Dishonest-enterprise request inflation is reachable in production but unobservable by the Planner. | Partial — `recipe_init` calls `set_requested` (since `0caee71`); nothing in `native_app/` reads `Market::requested()` | `ECO-SUB-005` and its 2026-08-28 drift note |
| Unmatched demand can be removed instead of persisting as a shortage queue. | Conflicting; economy violation | `ECO-SUB-001` |

**A default city has no external trade, by design.** `START_COMMANDS`
(`simulation/src/lib.rs:443+`) seeds ten `MapMakeConnection` commands carrying 13 lane patterns —
every one of them `Rail`, not one `Driving` — plus a single `RailFreightStation` whose door lands at
`(4297.4, 6315.3)`. An import is a physical truck movement since sov-abs, so `market_update`
(`simulation/src/economy/mod.rs:63-91`) only offers a freight station whose door sits within
`DISPATCH_LANE_CUTOFF` (50 units, `simulation/src/map_dynamic/dispatch.rs:86`) of a
`LaneKind::Driving` lane. On a fresh map no such lane exists, so no import trade is ever matched.
The effect reaches past the border: `simulation/src/souls/goods_company.rs:226-231` raises
`wanted_cargo` only for a trade whose seller is a `SoulID::FreightStation`, and
`simulation/src/souls/freight_station.rs:139` only summons a train at
`waiting_cargo + wanted_cargo >= 10` — so **no cargo train runs in a default game until the player
lays road to the station**. This is a ratified design decision (user, 2026-08-28): the train
arriving is the reward for connecting. It is asserted in both directions by
`tests::scenarios::ledger::sov_ie6_default_city_border_is_closed_until_road_reaches_the_station`.

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
must remain unclaimed until a later evidence artifact records their command and result. The
planning corpus and discovery paths derived from these fact-sheets are inventoried separately in
[wave3-corpus](../../research/fact-sheets/wave3-corpus.md) (superseded; kept as provenance).
