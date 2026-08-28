---
name: default-city-border-is-closed
description: A fresh map has NO external trade and no cargo train until the player lays road to the station - ratified design, not a bug; do not "fix" it with a road spur in START_COMMANDS
metadata:
  type: project
---

`START_COMMANDS` (`simulation/src/lib.rs:443+`) seeds **10** `MapMakeConnection` commands
carrying **13** lane patterns, every one `Rail` and **zero** `Driving`, plus one
`RailFreightStation`. Its door sits at `(4297.39, 6315.35)` and
`nearest_lane(door, Driving, Some(50.0))` is `None`. Verified by probe 2026-08-28.

Since sov-abs an import is a physical truck movement, so `market_update`
(`simulation/src/economy/mod.rs:63-91`) refuses a station outside `DISPATCH_LANE_CUTOFF`
of a driving lane. Consequence chain: no import trade -> `souls/goods_company.rs:226-231`
never raises `wanted_cargo` -> `souls/freight_station.rs:139` never reaches the
`waiting_cargo + wanted_cargo >= 10` threshold -> **no cargo train ever runs in a default
game**.

**Why:** ratified by the user 2026-08-28 — the train arriving is the reward for connecting
the station. Do NOT add a road spur to `START_COMMANDS` to "fix" it.

**How to apply:** it is asserted in both directions by
`tests::scenarios::ledger::sov_ie6_default_city_border_is_closed_until_road_reaches_the_station`
and recorded in `docs/reference/architecture/substrate.md` (Logistics and economy seams).

Two harness traps found building that test:
- `build_company_at` gives no soul unless a road exists near the building; `company_soul`
  needs one. Build a road first or `BuildingInfos::owner` returns `None`.
- A company's buy order is drained by `make_trades`' `extract_if` on the very tick it is
  posted, so reading it after `tick()` always shows `None`. Never assert on buy-order
  presence to prove demand; and an aged company may stop re-posting, so run A/B as two
  identical fresh `TestCtx`s differing in one variable rather than one long run.
- A road laid straight through the station footprint does NOT demolish it; endpoints just
  must not project onto a building (that hits `unreachable!()` in `Map::make_connection`).

Related: [[testctx-always-has-freight-station]], [[dispatch-reachability-50-units]].
