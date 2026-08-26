---
name: ruling-retail-dispatch
description: Ruling on sov-dispatch-wedge-ab4 — human retail purchases must not create truck Dispatches; the shopper's own trip IS the physical movement (Option C)
metadata:
  type: project
---

**Ruling 2026-08-26 (sov-dispatch-wedge-ab4): Option C.** Retail is two legs.
Factory→store stays dispatch-based (truck, real cargo). Store→consumer is the
shopper's own walk; it creates NO Dispatch. Settlement moment = the tick the human
arrives at the store building (`buyfood.rs:86-94`, the `BoughtAt(b)` arm where
`last_ate` is set). Seller capital -1 and reservation -1 happen there, together.

**Why:** the "nothing teleports" pillar requires a physical movement, not
specifically a *truck*. The human already walks. A dispatch for a loaf is a second
physical movement layered on the first — double-counted transport, and it demands
a SmallTruck from a `kind="store"` soul which spawns zero trucks
(`goods_company.rs:129` gates truck spawn on `CompanyKind::Factory`). Rejected
Option B (home delivery) on Kornai grounds: delivery-to-door removes the queue,
and the queue at the shop counter is the canonical shortage-clearing device.
W&R agrees — `$TYPE_SHOP` buildings have `$STORAGE_DEMAND_BASIC` inbound
(truck-fed) but no outbound delivery vehicles; citizens walk.

**Second, larger defect found while ruling (NOT in the ticket):** humans never
consume the bread they buy. `buyfood.rs:73` calls `market.buy(...,1)` each meal
cycle, `advance_dispatches` credits `capital[human] += 1`, and nothing anywhere
calls `produce(SoulID::Human(..), bread, -1)`. Verified by grep: the only
`market.produce` callers are `goods_company.rs:49,56,166` and tests. Human bread
capital is monotonically increasing — every citizen becomes an infinite bread
hoarder. Any fix to the settlement moment must also debit the human at eat-time
(or better: never credit the human at all — see the ruling body).

**Constraints recorded for the implementation:**
- Reservation at match time is kept (prevents double-sale). Release paths must
  be: (1) human arrives and eats, (2) human despawns — `Market::remove` already
  clears `reserved` wholesale for the removed soul, but if the human is the
  *buyer* the reservation lives on the *seller's* row, so `remove` must decrement
  the seller's reserved for every outstanding retail claim, (3) TTL expiry =
  going without.
- `recipe_should_produce` (`goods_company.rs:33-45`) reads RAW capital, not
  capital-minus-reserved. Once reservations are long-lived this is the wedge:
  the bakery counts reserved loaves as stock and stops baking. It must read
  `capital - reserved` on the production side.
