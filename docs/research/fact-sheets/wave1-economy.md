# Wave 1 economy substrate fact-sheet

**Kind:** reference
**Authority:** reference
**Status:** active
**Owner:** economy
**Last verified:** 2026-09-03
**Commit:** `266f7b21230dcfaf597773e7818527381da85495`

This fact-sheet constrains the rewrite of needs, resources, production, logistics, and trade. Code
reality is descriptive; the charter rulings are normative.

## Domain rulings

| Surface | Current reality | Binding rewrite constraint | Ruling |
|---|---|---|---|
| Needs | Humans buy only bread. A domestic match creates a `RetailClaim` (no truck); the human walks to the seller and `settle_retail` debits seller stock at eat time before `last_ate` advances (`simulation/src/economy/market.rs:468-490,661-693`, `simulation/src/souls/desire/buyfood.rs:88-117,156-164`). Domestic matches carry no money by construction (`money_delta: Money::ZERO`, `market.rs:539-545`). An expired or released claim means going without: `last_ate` does not advance and the desire re-queues (`buyfood.rs:106-115,166-171`). | Food and Meat are separate dwelling needs. Satisfaction consumes authoritative physical stock; failure persists as waiting or going without. | **PARTIAL** — eat-time settlement and going-without re-queue exist for bread only. |
| Resources | Lua declares 21 items. Metadata is identity plus `optout_exttrade`; no unit, mass, volume, storage class, transport class, or capacity exists (`base_mod/items.lua:1-108`, `prototypes/src/prototypes/item.rs:6-25`). | The 1.0 catalogue is the charter's 15 physical resources plus import-only Medicine. Water is a network utility, never cargo. | **CURRENT SUBSTRATE ONLY** |
| Production | Recipes atomically consume/produce integer inventory and gate on inputs, output threshold, staffing, and some blackout state (`simulation/src/souls/goods_company.rs:32-67`). Request inflation is production-reachable: `recipe_init` calls `set_requested` with `amount × request_multiplier` and `recipe_act` re-posts from `requested` (`goods_company.rs:21-26,52-57`, `economy/market.rs:453-461`). | Request, receipt, true consumption, surplus, and binding constraint are distinct state; inputs, capacity, labour, power, and water are physical gates. | **PARTIAL** |
| Logistics | Domestic matching reserves seller stock without moving capital; a finite truck gates seller debit (at `Loading` arrival) and buyer credit (at `ToDestination` arrival) at physical endpoints (`simulation/src/economy/market.rs:605-610,911-915,1059-1063`). Retail (human) matches create no truck or `Dispatch` (`market.rs:661-693`). | Preserve the useful transfer seam, but define one custody ledger and one delivery authority with cancellation and recovery. | **CONSISTENT CORE, UNSAFE INTERFACES** |
| Trade | Imports are physical: an external buy creates a `Trade` plus a `Dispatch` out of the freight station, and stock moves only at truck arrival (`market.rs:615-653,694-704,911-915,1059-1063`). Roubles still settle at match time for BOTH halves: `money_delta` is attached during `make_trades` (import `-ext_value`, export `+ext_value`) and applied by `market_update` before `advance_dispatches` (`market.rs:646-652,735-741`, `economy/mod.rs:93-136`). Exports create no `Dispatch`: the export block runs after the dispatch loop and debits seller capital immediately at match (`market.rs:660-743`). Domestic trades carry no money by construction (`money_delta: Money::ZERO`, `market.rs:539-545`). Freight stations hold unitless counters (`souls/freight_station.rs:30-37`). | One border-only rouble, fixed per-kind prices, physical customs clearance, and no domestic money. | **VIOLATION** |

## Conflict register

### ECO-SUB-001 — Unmatched demand is not a durable queue for enterprises; humans re-queue by going without

Human buyers repost. `WaitingForTrade` drains matched `Bought` into `BoughtAt`, and when the buy
order is gone (a match consumes it, see `Market::make_trades`) with no live retail claim, the
desire returns to `Empty` and re-queues instead of waiting on a dead reservation
(`simulation/src/souls/desire/buyfood.rs:88-115`). The `BoughtAt` arm likewise resets to `Empty`
on store demolition and after every arrival, advancing `last_ate` only when a live claim settles
(`buyfood.rs:141-144,146-171`). Going without never advances `last_ate` (never game over).

Non-human buyers do not repost. Each `make_trades` pass extracts non-human buy orders
(`buy_orders.extract_if(.., |s, _| !matches!(s, SoulID::Human(_)))`); any order left in that
batch when no freight station answers `find_external` is dropped with `continue` — it is neither
restored nor aged (`simulation/src/economy/market.rs:630-639`). Human orders are excluded from
that extraction and survive untouched (`market.rs:624-632`).

Classification: **VIOLATION, scoped**. Human demand persists as waiting or going without;
enterprise demand can be erased by a missing station instead of persisting with age.

### ECO-SUB-002 — Imports are physical stock with early money; exports teleport stock and money

Imports no longer credit buyers directly. The external-buy branch pushes a `Trade` whose seller is
the freight station and that trade falls inside the `all_trades[dispatch_start..]` window, so it
gets a physical `Dispatch` like any domestic delivery
(`simulation/src/economy/market.rs:615-654,694-704`). No capital moves at match: the station's
capital is debited when the truck arrives and enters `Loading`, and the buyer is credited when it
arrives and enters `Unloading` (`market.rs:911-915,1059-1063`). Between those transitions the
quantity sits in the dispatch, in neither soul's capital (`market.rs:167-171`).

Roubles still settle at match time on both halves, before any physical clearance. Imports attach
`money_delta: -(ext_value × qty)` and exports attach `money_delta: +(ext_value × qty)`
(`market.rs:646-652,735-741`); `market_update` applies every trade's `money_delta` to
`Government.money` and records `EcoStats`/sold/bought from the same match slice before calling
`advance_dispatches` (`simulation/src/economy/mod.rs:93-136`). An import stalled in `ToSource`
(no truck, no capital movement) is already paid for. Domestic trades carry no money by
construction (`money_delta: Money::ZERO`, `market.rs:539-545`).

Exports still teleport. The export block runs after the dispatch-creation loop, debits seller
capital (`*cap -= qty_sell`) and shrinks the sell order at match time, and creates no `Dispatch`
(`market.rs:707-743`).

Classification: **VIOLATION** of border-only settlement (early roubles both halves) and of
physical causality on exports (stock debited with no dispatch or clearance event).

### ECO-SUB-003 — Domestic matching is price-free but not queue-clearing

Domestic `money_delta` is `Money::ZERO` by construction — no internal money moves on a domestic
match (`market.rs:539-545`). Matching sorts candidate pairs by distance and requires one seller to
cover the buyer's full quantity; matched quantity is reserved on the seller's row, not debited
(`market.rs:517-551,564-613`). There is no partial multi-seller fill, request age, or plan
priority.

Classification: **PARTIAL**. Absence of price does not itself implement shortage allocation.

### ECO-SUB-004 — The inherited treasury still prices domestic actions

Workers, roads, zones, buildings, houses, and trains debit `Government.money`
(`simulation/src/economy/mod.rs:53-55`, `economy/government.rs:22-75`,
`world_command.rs:223-225`). Commands can drive it negative, so it is not a hard gate.

Classification: **CONFLICTING** with the rouble's border-only meaning.

### ECO-SUB-005 — Dishonest-enterprise request inflation is reachable but unobserved

Production inflation is reachable in production, not test-only. `recipe_init` calls
`market.set_requested(soul, item.id, amount × request_multiplier)` for every consumed input and
posts the inflated buy order in the same function; `recipe_act` re-posts from `requested` each
cycle (`simulation/src/souls/goods_company.rs:21-26,52-57`). The setter and accessor are
`Market::set_requested` / `Market::requested` (`simulation/src/economy/market.rs:453-461`).
`flour-factory` (4) and `slaughterhouse`-family recipes with a multiplier above 1 therefore
over-request in live cities.

Nothing in `native_app/` reads `Market::requested()`, and no UI exposes requested, received,
consumed, reserved, in-transit, or surplus state
(`native_app/src/gui/inspect/inspect_building.rs:244-299`).

Classification: **REACHABLE, UNOBSERVABLE** in gameplay.

### ECO-SUB-006 — Fulfillment has competing timestamps and authorities

`EcoStats`, `Sold`, and `Bought` record match-time promises. Company drivers react to `Sold`, while
the new market dispatch separately drives a global truck and transfers stock at endpoints
(`economy/mod.rs:93-125`, `souls/goods_company.rs:235-270`, `economy/market.rs:615-704`).

Classification: **CONFLICTING**. Allocation, delivery, consumption, and reporting are not one
coherent contract.

## Rewrite constraints

- Persist unsatisfied requests with age, partial fulfillment, substitution, and going-without evidence.
- Separate reported request, received, consumed, on-hand, reserved, and in-transit quantities.
- Keep production treasury-independent; do not call absence of enterprise finance a soft budget constraint.
- Establish one dispatch/custody authority and make every stalled or cancelled transfer recoverable.
- Settle border stock and roubles at an explicit physical clearance event, never at match time.

## Historical snapshot (2026-08-24, commit `186e08179b5ad9415dc4cd2d42d77a49303e35d6`)

Superseded claims from the pre-retail/import-physics substrate, retained for audit only:

- ECO-SUB-002 (then): imports credited buyer capital immediately at match
  (`market.rs:399-416` at that commit); exports debited seller capital before `find_external`
  (`market.rs:425-450` at that commit).
- ECO-SUB-005 (then): `set_requested` had no non-test caller; request inflation was configured
  manually in `tests/scenarios/hoarding.rs:224-246` only.
- Needs row (then): reaching the seller updated `last_ate` without consuming inventory
  (`economy/mod.rs:95-104`, `buyfood.rs:78-90` at that commit).
- ECO-SUB-001 (then): waiting citizens did not repost (`buyfood.rs:40-49` at that commit).

All four were superseded by `sov-abs` (physical import dispatch), the `sov-lpj` `recipe_init`
`set_requested` call, and the `settle_retail`/`RetailClaim` re-queue state machine. The body above
is current; this block is history.

## Verification boundary

All cited Rust, Lua, prototype, UI, charter, and scenario locations were reopened at the header
commit. Production reachability is confirmed: `recipe_init` in
`simulation/src/souls/goods_company.rs:21-26` calls `set_requested` in production (ECO-SUB-005 is
reachable). The simulation test command was not executed for this sweep; no gameplay, save/load,
mutation, profiler, or reference-game run was performed.
