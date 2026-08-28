---
name: finding-exttrade-teleport
description: Ext-trade IMPORT teleport is FIXED (commit 7721cdd, sov-abs, signed off 2026-08-28); the EXPORT half still teleports; the border is still an unbounded source
metadata:
  type: project
---

## FIXED — the free-goods leak (verified 2026-08-26)

`find_external` returning `None` runs BEFORE any credit. With no freight
station, unmatched enterprise buy orders are correctly denied. Humans are
carved out entirely (`extract_if(.., |s,_| !matches!(s, SoulID::Human(_)))`),
so retail clears by queue and going-without only.

## FIXED — the import teleport (commit 7721cdd, sov-abs, 2026-08-28)

The ext-trade BUY block now sits **ahead of** the dispatch-creation loop in
`Market::make_trades` and **touches no capital**. An import becomes an ordinary
`Dispatch` whose seller is a `SoulID::FreightStation`; the border is debited at
`Loading`, the buyer credited on arrival at `ToDestination`.

**Signed off with conditions** by kornai-economist 2026-08-28. Full ruling is the
`bd comments sov-abs` entry authored `kornai-economist`.

### Magnitudes that make it real shortage, not a one-tick delay
- `UP_DT = 20ms` (`common/src/timestep.rs:3`) → 50 ticks/second.
- Truck max speed `6.0` u/s (`transportation/vehicle.rs:64`) → **0.12 units/tick**.
- `DISPATCH_DWELL_TICKS = 3` (`market.rs:133`).
- A 200-unit station→factory leg ≈ **1700 ticks**, plus the ToSource leg.
- `companies.lua` sets `n_trucks = 1` on every company → the shared Dispatcher
  pool is ~one truck per enterprise; imports contend with domestic deliveries.
- No truck free → the dispatch waits in `ToSource` with **nothing debited**.
  That is a genuine queue.

### Guards that must not be removed (G1–G3)
1. The human carve-out predicate, guarded by
   `retail::scenario_human_order_never_fills_via_external_market`.
2. The **ordering** — the ext-trade buy block above the dispatch loop. Moving it
   back reinstates the teleport with no compile error;
   `ledger::sov_abs_ext_trade_import_is_physical` is the only catcher.
3. No capital credited in the ext-trade buy block.

## STILL LIVE — three successor items, none blocking

- **The EXPORT half still teleports.** The seller-surplus block still debits
  capital and credits money in the same tick with no `Dispatch`. Does not
  corrupt the hoarding signal (hoards are on the INPUT side; `sell_all` only
  posts production items), but output storage pressure never bites where a
  reachable station exists, so `storage_multiplier` halts are unreachable there.
- **The border is an unbounded SOURCE.** `advance_dispatches` debits the
  FreightStation seller with no stock and no reservation; its capital goes
  negative without limit. Accepted for 1.0 (transport is the binding
  constraint), but Kornai separates the soft domestic budget constraint from the
  **hard external** one. The right bound is freight-station **throughput**
  (units/tick), which is also the W&R precedent.
- **The border out-competes domestic supply by one tick.** Domestic matching
  runs first, but only for the tick the order is posted; `extract_if` then hands
  every survivor to the border. An enterprise never queues on a domestic
  producer that will have stock next tick. Not a violation — shortage is
  preserved — but it makes the border the DEFAULT supplier, not the residual one.

## Reachability filter (accepted scope addition)

`find_external` now filters to stations whose door is within
`DISPATCH_LANE_CUTOFF` (50.0, `map_dynamic/dispatch.rs:86`) of a driving lane.
**An unreachable border is a closed border** — correct economics. Two caveats:
it tests proximity to *any* lane, not route connectivity (a station on a
disconnected road island still passes and stalls in `ToSource`), and a city
whose only station is unreachable silently gets no external trade with no
Planner-facing signal. Both want a readout, not a code change.

## Standing hoarding caveat — DOWNGRADED, not obsolete

The old rule "any hoarding demonstration must run in a city with NO freight
station" is relaxed. A demonstration may now use one, but must constrain the
truck pool or the station distance, or assert on **delay and queue length**
rather than permanent denial — with plentiful trucks and a near station the
backfill still eventually completes, so shortage becomes delay, not denial.
`sov_lpj_flour_factory_hoards_bounded_no_freight_station` stays as the clean
bounded-hoard proof.

**Lesson for future audits:** re-verify the standing "known violations" list
against the tree before citing it. This entry has now gone stale twice.

Related: [[ruling-inflation-source]], [[numbers-base-mod]]
