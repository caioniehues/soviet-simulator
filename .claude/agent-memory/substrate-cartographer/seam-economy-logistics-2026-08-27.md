---
name: seam-economy-logistics-2026-08-27
description: Economy/logistics depth map (sov-00c) — Market is a ledger with a 340-line haul state machine inside it; sold.0 leaks unboundedly; EcoStats counts matches not deliveries; four substrate.md rows are stale
metadata:
  type: project
---

SEAM: sov-00c architecture review, economy + transportation + souls lens.
Verified **2026-08-27 against 8531d3c + dirty working tree** (sov-lpj in flight:
`request_multiplier` uncommitted in companies.lua/recipe.rs/goods_company.rs).

## The single most brief-wrecking fact

**`Market` is two modules wearing one name.** It is (a) an item ledger and (b) a
physical haul state machine. `advance_dispatches` (market.rs:735-1072) is ~340
lines taking **7 parameters** (`world, map, binfos, dispatcher, cbuf_vehicle,
parking, tick`); `remove` (market.rs:252-404) takes **6**. A brief that says
"add X to the market" must say which of the two it means, and any interface
work must decide whether the haul machine moves out.

The ratified spec already names the module that should exist:
`SPEC-LOGISTICS-001` (docs/reference/specifications/logistics.md:29) — "A haul
has one authoritative fulfillment authority… records allocation, reservation,
pickup, in-transit custody, delivery, physical return, or release; **it does not
own consumed state or quantity**." Today the thing that owns custody IS the thing
that owns quantity. `EVID-LOGISTICS-007` (logistics.md:113) explicitly lists
"Add a second fulfillment owner" as the falsifier.

## CONTRADICTS — docs/reference/architecture/substrate.md is part-stale

Four rows in the "Logistics and economy seams" table no longer describe the tree:

- **:60** "Delivery completion has no return-to-depot behavior, and failed
  dispatch has no recovery policy" — **FALSE at 7e4b82f**. `DispatchState::Returning`
  (market.rs:178), `MAX_RETURN_ROUTE_RETRIES = 20` (market.rs:140), and
  park-on-completion via `parking.reserve_near` + `park` (market.rs:1029-1035).
- **:63** "Imports credit stock immediately" — **fixed at fdfabca**; the credit
  `*capital.entry(buyer) += qty_buy` is now inside the `Some(ext)` arm (market.rs:669-673).
- **:64** "Unmatched demand can be removed instead of persisting as a shortage
  queue" — **no longer true for humans**: `extract_if(.., |s,_| !matches!(s, SoulID::Human(_)))`
  (market.rs:661-663) keeps human buy orders alive across the ext-trade pass.
- **:59** "Companies retain truck IDs, but global dispatch ignores that ownership"
  — **STILL TRUE**, verified: `comp.trucks` has exactly ONE reader in the whole
  repo (goods_company.rs:283) and it only picks `.first()` to assign a driver
  role. `Dispatcher::update` (dispatch.rs:100-104) registers every
  `VehicleKind::Truck` globally with no owner check.

Rows :57, :61, :62 also still hold.

## PRESENT-BUT-DEAD / leaking

**`CompanyEnt::sold` grows without bound for any seller that has no driver.**
- Pushed at economy/mod.rs:89-93 for every trade where the seller is a
  GoodsCompany and kind ≠ job-opening. **Includes human retail buys.**
- Popped ONLY inside the driver block (goods_company.rs:254), which is gated on
  `c.comp.driver` being `Some` and that human's `WorkKind::Driver{deliver_order: None}`.
- `company_soul` only spawns trucks `if ckind == CompanyKind::Factory`
  (goods_company.rs:132), and the driver role is only assigned
  `if c.comp.trucks.first()` is Some (goods_company.rs:283-284).
- **Lua: 6 of 26 companies are `kind = "store"`** — including `bakery`
  (companies.lua:11), the one company humans actually buy from. Stores never get
  trucks → never get a driver → **never pop `sold`**. `sold` is `Serialize`, so
  it grows in the save file forever.

**`DeliverAtBuilding` is a no-op for company buyers.** human.rs:106-121 mutates
nothing unless `matches!(b.kind, BuildingKind::RailFreightStation(_))`. The
driver's truck carries no ledger quantity. So one matched trade can put TWO
vehicles on the road — a Market `Dispatch` (which moves stock) and a driver
(which moves nothing) — and only the first is custody.

**`Market::dispatches()`** (market.rs:450) — still zero callers outside
`tests/scenarios/ledger.rs`. No Planner-visible haul readout exists, which is
what `logistics.md:94-98` ("Observability") requires.

## Nothing-teleports adjacency: two quantity-blind counters

1. **`EcoStats` records at MATCH time, not delivery time.** `EcoStats::advance`
   is called at economy/mod.rs:76 with `trades` — i.e. before
   `advance_dispatches` runs at :111. A match that never gets a truck, or whose
   truck vanishes (`log::warn!("dispatch lost …")`, market.rs:856), is still
   counted as internal trade in the only economic graph the game has.
2. **Freight cargo is unitless and qty-blind.** `f.f.wanted_cargo += 1` per
   trade regardless of qty (goods_company.rs:231) and `waiting_cargo += 1`
   (human.rs:117); trains subtract a flat **100** on arrival
   (freight_station.rs:102-103). A 55-unit trade and a 1-unit trade are the same
   "1". substrate.md:76 already classifies freight stations as partial because
   "their cargo remains unitless counters" — that row still holds.

## The interface is the test surface

`TestCtx` is **121 lines** (simulation/src/tests/mod.rs); the scenarios it
serves are **2,291 lines** (hoarding 284, inflation 315, ledger 592,
recipe_provided 259, retail 805, mod 36). What the scenarios must do because the
interface will not:
- Fabricate SoulIDs from raw keys: `CompanyID::from(slotmapd::KeyData::from_ffi(id))`
  (hoarding.rs:17) — no harness affordance for "a soul that trades".
- Re-assemble `Market::remove`'s six arguments by hand: `remove_soul`
  (hoarding.rs:77-88) exists purely to gather `map`/`binfos`/`world`/`dispatcher`/`tick`
  off `ctx.g`.
- Call `make_trades` directly with a fake external closure `|_| Some(ext)`
  (ledger.rs:91) — bypassing the tick loop entirely, so what is tested is not
  what runs.
- Poll for completion by ticking blind: `drain_dispatches` (hoarding.rs:92-103)
  loops up to `max_ticks` in chunks of 50 asking `dispatches().is_empty()`.

## LUA — base_mod/, verified 2026-08-27

- `companies.lua`: **26** goods-companies (`grep -c 'type = "goods-company"'`).
  **21 `kind = "factory"`, 6 `kind = "store"`** — note 21+6=27 vs 26 total; the
  `name =` line count is also 27 because one nested `bgen` block carries a name.
  Trust the `type =` count.
- **Every one of the 26 declares `n_trucks`**; `grep -c "n_trucks = 0"` → **0**.
  Truck count is declared but only honoured for factories (goods_company.rs:132).
- `request_multiplier`: **2 of 26**, both uncommitted (companies.lua:40 = 4,
  :582 = 3). Default is `1` at recipe.rs:63.
- `items.lua`: **21 items**, exactly **1** with `optout_exttrade = true`
  (`job-opening`, items.lua:6). The perennial trap, re-confirmed.

## Pillar friction found (report, do not "fix")

**Money gates construction, and the gate is unchecked.**
`world_command.rs:223-225`: `let cost = Government::action_cost(self, sim);
sim.write::<Government>().money -= cost;` — debited unconditionally, with no
affordability check anywhere. Reachable from `native_app/src/debug_gui/hud.rs:47`.
So money is a *scorekeeper*, not a gate — "never game over" holds by accident
(balance simply goes negative), while charter's "money is not a gate" is
technically satisfied only because nobody checks the balance. Inherited
Egregoria; `Money::new_bucks(150_000)` starting balance (government.rs:17).

## TRAPS for anyone working this seam

1. **`recipe_act` `.unwrap()`s `requested`** (goods_company.rs:55). Any soul
   that never ran `recipe_init` panics. Old saves predating sov-lpj are the
   live case.
2. **`dbg!(price_consumption, price_workers, qty)` at market.rs:1124** runs
   inside `calculate_prices`, called from `Market::default()`. It fires on every
   Market construction, including every `TestCtx::new()`.
3. **`ItemID::new("job-opening")` is control flow via string compare**, in five
   places: market.rs:585, market.rs:605, economy/mod.rs:49/81/90,
   goods_company.rs:165. The labour system is smeared through market internals.
   Renaming that Lua item silently breaks hiring, reservations and dispatch
   suppression at once.
4. **Matching has no age, priority or partial fill.** `make_trades` sorts
   candidates by `sorder.pos.distance2(border.pos)` (market.rs:523) each tick and
   rejects any buyer whose `qty_buy > qty_sell` outright (market.rs:520-522). A
   distant or large buyer can starve indefinitely with no queue position and no
   recorded age — against logistics.md:91-92 ("Shortage remains in the request
   queue until a permitted substitution or going-without decision is recorded").
5. **`unsafe { get_unchecked_mut }` in ecostats.rs:88,90** guarded only by a
   comment asserting the cursor is modulo HISTORY_SIZE.

## Where the primary sources live
- Ledger + haul machine: `simulation/src/economy/market.rs` (1280 lines;
  `remove` 252, `make_trades` 497, `advance_dispatches` 735, `calculate_prices` 1081).
- Trade application + resource gather: `simulation/src/economy/mod.rs:44-119`.
- Recipe request/consume: `simulation/src/souls/goods_company.rs:21-67`.
- Driver shadow path: `souls/desire/work.rs:41-67` → `souls/human.rs:106-121`.
- Dispatcher: `simulation/src/map_dynamic/dispatch.rs` (query 222, reserve 203).
- Retail: `souls/desire/buyfood.rs` + `market.rs:466` (`settle_retail`).
- Specs: `docs/reference/specifications/logistics.md:29-45,70-98`, `trade.md:34-42`.

See [[MEMORY]], [[seam-hoard-panel-story0107]], [[false-claims-failure-inventory]].
