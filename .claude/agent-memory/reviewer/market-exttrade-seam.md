---
name: market-exttrade-seam
description: The Egregoria market's external-trade block — buy side now moves physically (sov-abs), sell side still teleports, and find_external's Driving-lane filter closes the default city's border
metadata:
  type: project
---

`simulation/src/economy/market.rs::make_trades` has TWO capital paths per item,
and reviewers keep only looking at the first:

1. the domestic potential-trade match loop (the one everybody edits), and
2. the `if !*optout_exttrade` blocks — now SPLIT in two places.

**UPDATED 2026-08-28 (branch `fix/sov-wave-market`, commit 7721cdd, sov-abs):**

- **Buy side FIXED.** The import block moved *before* the dispatch-creation
  loop (market.rs:606) and no longer credits `capital`. An import is now an
  ordinary `Dispatch` whose seller is `SoulID::FreightStation(..)`; the border
  row is debited at `Loading` and the buyer credited at `ToDestination`.
- **Sell side STILL TELEPORTS.** market.rs:~735 still does `*cap -= qty_sell`
  at match time with no dispatch. The pillar is half closed. Border accounting
  is now asymmetric: imports are double-entry (border row goes negative),
  exports are single-entry (nothing credits the border). `total_qty` in
  `tests/scenarios/ledger.rs:41` sums all capital rows, so a future scenario
  spanning both reads as a leak.

**The reachability filter is the thing to check first (economy/mod.rs:72-90).**
`find_external` now filters freight stations to those whose `door_pos` is within
`DISPATCH_LANE_CUTOFF` (= 50.0, `map_dynamic/dispatch.rs:86`) of a `LaneKind::Driving`
lane. Measured consequences:

- `START_COMMANDS` (`simulation/src/lib.rs:443`) builds **only Rail lanes** —
  all 10 `MapMakeConnection` entries are `Rail`. So the one default
  `RailFreightStation` can NEVER pass the filter.
- `find_external` is shared by the import block AND the export block, so the
  default city now does **zero** external trade in either direction.
- `souls/goods_company.rs:226` increments `wanted_cargo` only on a trade whose
  seller is a FreightStation, and `souls/freight_station.rs:139` needs
  `waiting_cargo + wanted_cargo >= 10` to dispatch a train — so the train
  cargo system is inert in a default game too.
- Removing the filter turns **8 tests red** (unreachable-station imports pile
  up immortal in `ToSource`), which is the proof the default city really did
  pick that station before.

**Why:** `base_mod/items.lua` sets `optout_exttrade = true` on **job-opening
only**; all 20 physical goods take path 2.

**How to apply:** when a diff touches `Market`, stock movement, reservations,
or "nothing teleports", read BOTH ext-trade blocks and `find_external`'s filter.
Any claim that "the export half is untouched" is false while `find_external` is
shared. Related: [[sim-test-harness-quirks]], [[market-remove-dispatch-drop]],
[[dispatch-truck-park-seam]].
