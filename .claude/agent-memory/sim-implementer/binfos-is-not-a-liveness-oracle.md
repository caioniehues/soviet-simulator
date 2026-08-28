---
name: binfos-is-not-a-liveness-oracle
description: BuildingInfos entries survive demolition — never use binfos.get(b)/owner(b) to ask whether a BuildingID is still alive; only map.buildings() answers that
metadata:
  type: project
---

`BuildingInfos` (`simulation/src/map_dynamic/binfos.rs`) has **no `remove` method**, and
nothing anywhere in the crate deletes an entry when a building is demolished. Its backing
store is a `slotmapd::SecondaryMap`, whose `get(key)` (`slotmapd/src/secondary.rs:479`)
only compares the key's version against **its own** slot — it never consults the primary
`Buildings` map. So after `Map::remove_building(b)`:

- `binfos.get(b)` → still `Some`
- `binfos.owner(b)` → still `Some(soul)`, even though the company soul is killed by
  `goods_company.rs:195` (`cbuf.kill(me)` when `map.buildings.get(...)` is None)
- `binfos.building_owned_by(soul)` → still `Some(b)` (the `owners` BTreeMap is never pruned)

The only liveness check for a `BuildingID` is `map.buildings().contains_key(b)`.

**Why:** verified 2026-08-28 fixing sov-xyx. The ticket asserted "apply already receives
binfos, no signature change" — false for exactly this reason. `BuyFood::apply` had to take
`&Map` (which `update_decision`, `human.rs:170`, already held).

**How to apply:** any time a brief proposes an existence check on a `BuildingID` through
`binfos`, reject it and thread `&Map` instead. `scenario_demolished_store_releases_bought_at_customer`
in `simulation/src/tests/scenarios/retail.rs` asserts `binfos.get(b).is_some()` after a real
demolition, specifically to keep this documented in the test suite.

Related: [[sim-test-setup-traps]], [[feedback-stale-brief-check]].
