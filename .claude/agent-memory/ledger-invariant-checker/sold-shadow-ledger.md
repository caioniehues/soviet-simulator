---
name: sold-shadow-ledger
description: CompanyEnt::sold is a bookkeeping vec, not a quantity bucket — it conserves, but grows without bound on the 6 store companies; the audited shape and why it is a save leak not a mint
metadata:
  type: project
---

Audited 2026-08-27 at HEAD `8531d3c` (+ uncommitted sov-lpj, which does not
touch this path). Verdict: **quantity CONSERVED, unbounded growth confirmed.**

## Why it conserves
`sold` holds `Trade` *records*, never a balance. The only two sites:
- **push** `economy/mod.rs:91` — `world.companies.get_mut(id).unwrap().sold.0.push(trade)`,
  guarded by `SoulID::GoodsCompany(id)` seller (`:89`) and `trade.kind != job_opening` (`:90`).
- **pop** `souls/goods_company.rs:254` — `let Some(trade) = c.sold.0.pop() else { return }`.

The pop's only effect is setting `WorkKind::Driver { deliver_order }` to a
`BuildingID` (`:271`). That driver trip terminates in
`HumanDecisionKind::DeliverAtBuilding` (`souls/human.rs:106-121`), which
**mutates nothing unless the target building is `BuildingKind::RailFreightStation`**
(`:110`), and even then only bumps `f.f.waiting_cargo`. It never touches
`Market` capital/reserved/dispatches. So the driver trip is *cosmetic motion*
running in parallel with the authoritative `Dispatch` that actually settles
capital. Two physical representations of one trade, one ledger.

## The unbounded-growth shape (real, ordinary gameplay)
Pop is gated on `c.comp.driver` being `Some` (`goods_company.rs:239`). A driver
is only ever assigned inside `if let Some(truck) = c.comp.trucks.first()` +
`proto.kind == CompanyKind::Factory` (`:283-284`), and trucks are only spawned
`if ckind == CompanyKind::Factory` (`:132`). So **a store never gets a truck,
never gets a driver, never pops** — its `sold` vec only ever grows.

`base_mod/companies.lua` counts (verified by grep, 2026-08-27):
`kind = "store"` = **6**, `kind = "factory"` = **21**, total goods-companies = **26**.
(26 = 21 + 6 minus one duplicate-counted line; the 6/26 store figure is what matters.)

`Sold` is `#[derive(Inspect, Default, Serialize, Deserialize)]`
(`economy/mod.rs:36`) and `CompanyEnt::sold` is a plain field (`world.rs:189`)
with no `#[serde(skip)]` — so every uncollected record is written to the save
file. Bread retail sales hit this: a store selling to a `SoulID::Human` buyer
still satisfies both push guards, so the vec grows one entry per bread sale
forever.

**Not a mint.** No arithmetic reads the vec's length or sums its `qty`. It is a
save-size/memory leak plus a dead-code smell, not a conservation break.

## The rule this teaches
A record vec that is pushed unconditionally but popped only under a capability
gate (truck / driver / vehicle present) leaks for every soul that lacks the
capability. **When you find a `push` and a `pop`, check whether every pusher's
soul can reach the popper's gate.** Same shape as [[break-families]] Family C
(unconditional acquire, conditional release) — but here the leaked thing is a
record, not a reservation, so it costs bytes instead of goods.

See [[market-balance-index]] for the buckets that *do* carry quantity.
