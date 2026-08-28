---
name: dispatcher-pool-and-reachability
description: Verified facts about the truck pool, DISPATCH_LANE_CUTOFF, the backward-BFS connectivity already in DispatchOne::query, the tick rate, and the still-teleporting export half
metadata:
  type: project
---

Verified at source 2026-08-28 in `/home/caio/sov-wave-market-wt` at `7721cdd`
(branch `fix/sov-wave-market`). Expensive to re-derive; treat as current until
those files change.

## Tick rate — the number every retry bound must be judged against

`common/src/timestep.rs:3` — `UP_DT = Duration::from_millis(20)`, so **50 ticks per
second**. Therefore `MAX_RETURN_ROUTE_RETRIES = 20` is **0.4 seconds of game time at
1x**, not "a while". Any bound expressed in ticks must be divided by 50 before you
judge whether it survives a player action. A road drag takes seconds.

## The truck pool is one truck per factory, and nothing else

- `souls/goods_company.rs:129-137` — trucks spawn only for `CompanyKind::Factory`,
  `proto.n_trucks` each.
- `base_mod/companies.lua` — every factory entry has `n_trucks = 1`. `kind = "store"`
  entries have no `n_trucks` at all.
- Freight stations spawn **no trucks** (they use trains, `souls/freight_station.rs`).
  So an ext-trade import borrows a factory's truck to fetch from the border.
- `Dispatcher::query` is FCFS by dispatch index with **no ranking**. The roadmap's
  "deficit priority and meaningful distance" ranking does not exist yet. Long border
  round trips therefore crowd out short domestic hops.

## Proximity is not reachability — but the substrate already has reachability

- `map_dynamic/dispatch.rs:86` — `pub const DISPATCH_LANE_CUTOFF: f32 = 50.0`, used at
  `:245` in `DispatchOne::query`. `dispatch.rs:163` in `register()` still uses a bare
  `50.0` literal; three sites, not two.
- `economy/mod.rs` `find_external` filters freight stations by
  `map.nearest_lane(door_pos, Driving, Some(DISPATCH_LANE_CUTOFF))`. That is
  **proximity only**. A door 40 m from a lane on a disconnected road component passes.
- The real connectivity test is already in `dispatch.rs:264-293`: `DispatchOne::query`
  does a **backward BFS over the lane graph** from the target lane and returns only a
  vehicle graph-connected to it. So the substrate refuses the delivery; the proximity
  filter only stops the *promise*, and only for the no-lane-at-all case (the hardcoded
  START_COMMANDS station at ~(4300,6300)).
- Consequence: a lane-proximate but disconnected station leaves the dispatch in
  `ToSource`/`None` forever, and the buy order was already `extract_if`-removed at
  match. `capital` is never credited, `recipe_should_produce` stays false,
  `recipe_act` never runs, `buy_until` never re-posts — the enterprise is permanently
  dead and does not recover when the player connects the road. See
  [[dispatch-tosource-wedge-surface]].

## The export half still teleports

`economy/market.rs:706-733` — the seller-surplus export does `*cap -= qty_sell` with
**no `Dispatch`**, after the dispatch-creation loop. Goods leave the building and reach
the border in the same tick. `sov-abs` fixed only the import half. The zero-vehicle
falsification test still fails on this path. File it; do not assume "ext trade is
physical" means both directions.

## Freeing a truck is not parking it

`market.rs:1050-1079` (sov-2c4) is the correct exit: `parking.reserve_near` +
`map_dynamic::router::park`, **then** `dispatcher.free`. Three other exit sites
(`:910` sov-jcl outbound exhaustion, `:951` return exhaustion, `:968` both-gone) do a
bare `free`, leaving the truck `Driving` with an ended itinerary and a live collider —
stopped in a lane, re-offered by the pool, and refused every time by the `Parked` guard
at `:800-804`. In the pool and permanently unusable. See [[vehicle-substrate-unpark]].

## A dead truck is purged from the dispatcher — but a reserved dead truck leaks

`world.rs:80-83` — `VehicleEnt::sim_drop` calls `Dispatcher::unregister` for every
removed `VehicleKind::Truck`. So "the pool can hand out a stale dead id" is **refuted**.
Residual: `DispatchOne::unregister` (`dispatch.rs:218-224`) early-returns when
`positions` has no entry, and `reserve` (`:211`) *removes* the entry from `positions`.
A truck destroyed while reserved therefore keeps its id in `reserved_by` forever. Inert
for `query` (it is no longer in `lanes`) and market's `world.vehicles.get(v).is_none()`
arms call `free` on the next tick, so it is a leak, not a wedge.
