# EPIC-007 — Physical foreign trade

**Summary:** Physical foreign trade
**Stories:** STORY-0023, STORY-0024, STORY-0025, STORY-0026, STORY-0027, STORY-0028, STORY-0029
**Primary sources:** `docs/egregoria-substrate-audit.md`, `spec/logistics.md`, `spec/trade.md`
**Status:** 0/7 done

## STORY-0023

**Epic:** EPIC-007 — Physical foreign trade
**Title:** Seed a stocked starter warehouse at game start

**As a** planner
**I want** a pre-stocked warehouse of essential goods present at turn one
**So that** the very first production chains have something to draw on before any internal chain or import route exists

**Acceptance criteria:**
- AC-1: On a new game start, a designated starter warehouse building/store holds a non-zero, config-defined quantity of at least the goods required to bootstrap the opening production chains, before any player action. [SUBSTRATE: ABSENT — greenfield, decided 2026-08-22: 'a stocked starter warehouse seeds turn one', docs/egregoria-substrate-audit.md:159-161] · impact:`cross-surface` · seam:`e2e` · scenario:`SCENARIO-0015`
- AC-2: Goods drawn from the starter warehouse are consumed via the same physical recipe-input path as any other sourced input (Market::buy_until / recipe_init), never a special-cased free-goods branch. [SUBSTRATE: PARTIAL — buy_until always requests exactly item.amount from the static recipe with no distinction of source today, economy/market.rs:161-167, souls/goods_company.rs:23,47, docs/egregoria-substrate-audit.md:142-144] · impact:`local` · seam:`integration` · scenario:`SCENARIO-0015`

**Sources:**
- `docs/egregoria-substrate-audit.md:159-161`

**Status:** pending

## STORY-0024

**Epic:** EPIC-007 — Physical foreign trade
**Title:** Offer customs imports at a markup as a permanent paid deadlock escape

**As a** planner
**I want** to pay a price premium at customs to import a good whenever a domestic supply chain is deadlocked
**So that** a stalled chain always has a costly-but-available way out, distinct from the one-time starter warehouse

**Acceptance criteria:**
- AC-1: At any point in the game (not only turn one) the player can place a customs import order for a deficit good at a price strictly higher than its domestic administered/shadow price, and this option remains available for the life of the save. [SUBSTRATE: ABSENT — greenfield, decided 2026-08-22: 'customs imports at a markup remain the permanent paid escape hatch whenever a chain deadlocks', docs/egregoria-substrate-audit.md:159-161] · impact:`journey` · seam:`integration` · scenario:`SCENARIO-0016`
- AC-2: A customs import order debits the treasury's hard-currency (foreign rouble) account, never the enterprise beznal or household nal circuits, distinguishing this from internal trade. [SUBSTRATE: ABSENT — greenfield; today the only money pool at all is Government.money and it is undifferentiated, economy/government.rs:10, docs/egregoria-substrate-audit.md:127-128] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0016`
- AC-3: The markup import does not resolve instantly: it still requires the same physical border-clearance and inland haul as any other foreign trade order (see border-crossing story), it is not a same-tick money-for-goods shortcut. [SUBSTRATE: CONFLICTS — today every unmet buy order is satisfied instantly and unconditionally by an infinite external partner with no vehicle trip, economy/market.rs:285-304, docs/egregoria-substrate-audit.md:130-135] · impact:`journey` · seam:`e2e` · scenario:`SCENARIO-0016`

**Sources:**
- `docs/egregoria-substrate-audit.md:130-135,159-161`
- `spec/trade.md:9-12`

**Status:** pending

## STORY-0025

**Epic:** EPIC-007 — Physical foreign trade
**Title:** Require a physical vehicle trip to move goods across the border

**As a** planner
**I want** an ordered import/export to remain unfulfilled until a vehicle physically hauls the goods across a customs building
**So that** money at the border is never sufficient by itself — the one rule that nothing teleports holds at the border too

**Acceptance criteria:**
- AC-1: Placing a foreign trade order does not immediately mutate any goods quantity or capital field; goods only appear/disappear from domestic stock after a vehicle entity completes a trip through a border-crossing building. [SUBSTRATE: CONFLICTS — today external buy/sell loops mutate capital in the same tick with no vehicle trip at all, economy/market.rs:285-304 (buy side), :307-331 (sell side), docs/egregoria-substrate-audit.md:130-135] · impact:`journey` · seam:`e2e` · scenario:`SCENARIO-0017`
- AC-2: If no freight/vehicle capacity exists to service a foreign trade order, the order remains queued and no capital or money changes hands — replacing today's behaviour where capital is credited before existence of a freight station is even checked. [SUBSTRATE: CONFLICTS — confirmed bug: both external loops mutate capital before checking a freight station exists (market.rs:291 credits, :293 checks; symmetric at :317/:320); if find_external returns None goods appear with no trade record and no money debit, docs/egregoria-substrate-audit.md:136-141] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0017`
- AC-3: Foreign trade throughput is capped by physical means (customs bay count, fleet size, border-edge capacity) rather than by any numeric trade-volume token, so a saturated customs house measurably slows import/export rate. [SUBSTRATE: ABSENT — greenfield; no vehicle-bay or throughput model exists at the border today, docs/egregoria-substrate-audit.md:130-135] · impact:`journey` · seam:`integration` · scenario:`SCENARIO-0017`

**Sources:**
- `spec/trade.md:9-17`
- `docs/egregoria-substrate-audit.md:130-141`

**Status:** pending

## STORY-0026

**Epic:** EPIC-007 — Physical foreign trade
**Title:** Build customs houses as typed border-crossing buildings

**As a** planner
**I want** to construct a customs house with a transport mode (road/rail/air), a per-cargo-class buffer, vehicle bays, and both a domestic-facing and a border-facing edge
**So that** foreign trade has a physical building the player sites and can bottleneck, not an implicit map-edge shop

**Acceptance criteria:**
- AC-1: A customs house is a placeable building entity distinct from ordinary production buildings, with fields for mode, per-transport-class buffer (1-unit buffer per cargo class), bay list, domestic edge, and border edge. [SUBSTRATE: ABSENT — greenfield; no BorderCrossing/customs building type exists in the codebase, docs/egregoria-substrate-audit.md:130-135 (external partner has no building at all)] · impact:`cross-surface` · seam:`app-level` · scenario:`SCENARIO-0025`
- AC-2: There is no map-edge 'infinite external partner' reachable without passing through a built customs house — every export/import order resolves only via a player-built or pre-placed customs house entity. [SUBSTRATE: CONFLICTS — today the external partner exists with no capacity limit and is reachable with no building constructed, economy/market.rs:285-331, docs/egregoria-substrate-audit.md:130-135] · impact:`journey` · seam:`e2e` · scenario:`SCENARIO-0025`
- AC-3: Electricity import/export transformers and border pipelines exist as separate typed border-crossing endpoints from road/rail/air customs houses, with the transport medium of a good deciding which border building it crosses at (the same routing rule used domestically). [SUBSTRATE: ABSENT — greenfield; no utility border building type exists, only a road/rail/air customs house concept is modelled at all, spec/trade.md:14-15] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0025`
- AC-4: A customs house may be either a pre-placed/map-provided instance or a player-constructed instance (the `$SUBTYPE_OWN_CUSTOM` distinction), and both variants participate in border trade identically once built. [SUBSTRATE: ABSENT — greenfield; no customs house entity exists yet to have this ownership distinction, spec/trade.md:13] · impact:`local` · seam:`integration` · scenario:`SCENARIO-0025`

**Sources:**
- `spec/trade.md:9-17`
- `docs/egregoria-substrate-audit.md:130-135`

**Status:** pending

## STORY-0027

**Epic:** EPIC-007 — Physical foreign trade
**Title:** Settle foreign trade only on physical border clearance

**As a** planner
**I want** treasury debit/credit for a trade to occur at the moment the vehicle clears the border, not at order-match time
**So that** treasury and simulation state never diverge

**Acceptance criteria:**
- AC-1: A trade order's status progresses ordered -> atCustoms -> cleared, and the treasury balance changes only on the transition into cleared, never earlier. [SUBSTRATE: CONFLICTS — settlement happens at match time today (the same tick the pair is drained in make_trades), not at any border-clearance event, economy/market.rs:193-336, docs/egregoria-substrate-audit.md:130-135] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0016`
- AC-2: If a save is loaded mid-transit (a trade order in the atCustoms state), the treasury balance on load reflects only cleared trades, and the in-flight order resumes without double-crediting or losing the goods. [SUBSTRATE: ABSENT — greenfield; no in-flight trade state or save-compatible representation of it exists today, docs/egregoria-substrate-audit.md:130-135] · impact:`cross-surface` · seam:`process-level` · scenario:`SCENARIO-0016`
- AC-3: (OPEN/DEFERRED — whether a trade order locks its price at order time or floats to the clearance-time price is an undecided design question per spec/trade.md:57,63, not scheduled for 1.0 until resolved) A trade order's price-snapshot policy is not yet defined; the draft data model reserves an optional `priceAtOrder?` field for whichever policy is later chosen. [SUBSTRATE: ABSENT — greenfield; no policy decision or field exists in code, spec/trade.md:57,63] · impact:`none` · seam:`unit`
- AC-4: A cleared trade's paired ledger entry is tagged as import or export at settlement, enabling per-direction trade-balance reporting distinct from a currency-only or amount-only record. [SUBSTRATE: ABSENT — greenfield; no ledger entry type distinguishes import/export direction today, spec/trade.md:26] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0016`

**Sources:**
- `spec/trade.md:26-29,57,63`

**Status:** pending

## STORY-0028

**Epic:** EPIC-007 — Physical foreign trade
**Title:** Make the external partner a finite, physical customs crossing

**As a** planner
**I want** unmet buy/sell orders to only clear through a real, capacity-limited customs vehicle trip, never an unconditional instant partner
**So that** excess demand becomes a visible queue instead of teleporting goods across a fictional border, matching Kornai's shortage-economy target

**Acceptance criteria:**
- AC-1: An unmet buy order for which `find_external` returns a customs partner is NOT satisfied unless a real vehicle trip (freight station dispatch, travel time) is scheduled to deliver it; the trade must not resolve on the same tick it is posted. [SUBSTRATE: CONFLICTS — economy/market.rs:285-304 (buy side) resolves every unmet buy order instantly and unconditionally in the same `make_trades` call, with no vehicle, no capacity limit, no delay] · impact:`journey` · seam:`integration` · scenario:`SCENARIO-0142`
- AC-2: An unmet sell-side surplus for which `find_external` returns a partner is likewise gated on a real freight-station vehicle trip before capital/money changes hands. [SUBSTRATE: CONFLICTS — economy/market.rs:307-331 (sell side) is symmetric to the buy-side bug: instant, unconditional, no vehicle trip, no capacity] · impact:`journey` · seam:`integration` · scenario:`SCENARIO-0142`
- AC-3: The external partner has a finite per-tick or per-customs-house throughput limit; when the limit is exceeded, further unmet orders remain queued (unresolved) rather than being cleared. [SUBSTRATE: CONFLICTS — economy/market.rs:285-331 has no capacity limit of any kind on the external loops, exactly the CS1 'unlimited priority-0 offer' pattern spec/logistics.md names and rejects] · impact:`journey` · seam:`integration` · scenario:`SCENARIO-0142`
- AC-4: The customs house is a per-transport-class pass-through with a token-bucket capacity of 1 per class; it stockpiles no cargo between vehicle trips, so a customs implementation that accumulates goods as a warehouse violates this AC. [SUBSTRATE: ABSENT — greenfield; economy/market.rs external loops have no customs-house entity at all, spec/logistics.md:76] · impact:`local` · seam:`integration` · scenario:`SCENARIO-0142`
- AC-5: The border crossing point a good uses is selected by its transport medium (a road-medium good crosses at a road border building, a rail-medium good at a rail border building, a utility crosses at its dedicated utility border building), matching the domestic medium gate rather than a single generic crossing. [SUBSTRATE: ABSENT — greenfield; no medium-specific border-building selection exists, spec/logistics.md:76] · impact:`cross-surface` · seam:`integration` · scenario:`SCENARIO-0142`

**Sources:**
- `spec/logistics.md:48-58,95-100`

**Status:** pending

## STORY-0029

**Epic:** EPIC-007 — Physical foreign trade
**Title:** Fix capital mutation ordering on the external-trade seam

**As a** developer
**I want** the external buy/sell loops to check that a freight station exists before mutating capital, not after
**So that** goods never appear or vanish with no trade record and no money debit once the queue/vehicle requirement lands on this seam

**Acceptance criteria:**
- AC-1: On the buy side, when `find_external(order.pos)` returns `None`, `capital` for the buyer is left unchanged (currently it is credited at market.rs:291 before the `None` check at market.rs:293, so goods appear with no trade and no debit). A test that forces `find_external` to return `None` and then asserts `capital[buyer]` is unchanged must pass. [SUBSTRATE: CONFLICTS — economy/market.rs:291 credits capital before the existence check at market.rs:293] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0144`
- AC-2: On the sell side, when `find_external(order.pos)` returns `None`, `capital` and `order.qty` for the seller are left unchanged (currently `cap -= qty_sell` at market.rs:317 and `order.qty -= qty_sell` at market.rs:318 both run before the `None` check at market.rs:320, so goods vanish with no trade record and no credit). A test that forces `find_external` to return `None` and asserts both fields are unchanged must pass. [SUBSTRATE: CONFLICTS — economy/market.rs:317-318 mutate before the existence check at market.rs:320] · impact:`local` · seam:`unit` · scenario:`SCENARIO-0144`

**Sources:**
- `spec/logistics.md:95-100`

**Status:** pending