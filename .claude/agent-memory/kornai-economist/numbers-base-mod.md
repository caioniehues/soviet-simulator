---
name: numbers-base-mod
description: Magnitudes from base_mod/companies.lua — store vs factory kinds, recipe amounts, storage_multiplier, and the bakery/consumer bread loop
metadata:
  type: project
---

Verified 2026-08-26 against `base_mod/companies.lua`.

**Every store recipe is `storage_multiplier = 5`, `duration = "100s"`, amounts of 1.**

| prototype | kind | consumption | production |
|---|---|---|---|
| bakery | store | flour 1 | bread 1 |
| supermarket | store | meat 1, vegetable 1, cereal 1 | (none) |
| clothes-store | store | cloth 1 | (none) |
| florist | store | flower 1 | (none) |
| high-tech-store | store | high-tech-product 1 | (none) |

Note the bakery is the ONLY store with a `production` block — it is both the
factory and the shop for bread. Every other store consumes and produces nothing,
i.e. it is a pure sink; goods it "sells" to humans never existed as sell orders.
Only bread is actually bought by humans (`buyfood.rs:73`, the sole
`ItemID::new("bread")` demand site in the sim).

**Truck spawning:** `goods_company.rs:129` — `if ckind == CompanyKind::Factory`.
Stores get zero trucks regardless of `n_trucks`. Bakery has `n_workers = 3`,
`size = 10.0`, `power_consumption = "200W"`.

**Magnitude of the wedge:** bakery caps at `capital(bread) < 1 * (5+1) = 6`
(`recipe_should_produce`, `goods_company.rs:41`). With reservations never
released, six matched-but-undelivered loaves freeze the bakery permanently —
a six-unit window, reachable within six meal cycles of a single citizen.

Related: [[ruling-retail-dispatch]]
