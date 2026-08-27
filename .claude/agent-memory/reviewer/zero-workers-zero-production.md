---
name: zero-workers-zero-production
description: In simulation/, a scenario needs BOTH staffing and electricity before recipe_act ever runs — n_workers or a blackout silently zeroes productivity, so "driven by company_system" tests can prove only recipe_init
metadata:
  type: project
---

`CompanyEnt::productivity` (`simulation/src/souls/goods_company.rs:95-113`)
returns 0.0 through **two independent gates**, and either one silently reduces a
production scenario to `recipe_init` only:

1. **Workforce** — `raw_productivity` (`:82-92`) is
   `workers.len() / proto.n_workers`. Every base_mod factory ships
   `n_workers = 10`, so a scenario with no humans gets 0.
2. **Electricity** — any company with `power_consumption > 0` returns 0.0 on a
   blackout. `build_special_building`'s `connected_road: None` puts the building
   in an isolated single-building network with zero producers, so a
   `flour-factory` (`power_consumption = "10kW"`) is permanently blacked out even
   when fully staffed.

With productivity 0, `c.comp.progress` never reaches 1.0 and the
`cbuf.exec_on(...recipe_act)` branch at `goods_company.rs:208-217` is unreachable.

Confirmed on sov-lpj (2026-08-26/27). First pass: stock pinned at the request
ceiling for all 20k ticks (`max_stock=4`, `flour_stock=0`, `progress=[0.0, 0.0]`).
After staffing via `spawn_human` **and** threading a real `RoadID` plus an
unstaffed `solar-panel` (`n_workers = 0`, `power_production = "10kW"`, no
recipe — cannot perturb other markets), the same test oscillated for real:
runs `[(3,39),(4,334),(3,66),(4,1)]` with flour actually produced.

**Why:** the interesting half of a production test is consumption — steady-state
oscillation, surplus draw-down, storage-cap halts. A no-humans or blacked-out
scenario proves none of it while still going green, because the initial
`buy_until` alone satisfies any assertion whose lower bound sits above the
honest value.

**How to apply:** when a diff adds a scenario claiming to exercise the production
loop, check staffing AND `connected_road`/power before believing any
"converges / oscillates / consumes" claim. The cheap check is to probe the
produced item's capital and `comp.progress`; if flour is 0 and progress is 0.0,
`recipe_act` never ran. Assert **both** bounds and sample a window — a single
end-of-run snapshot cannot distinguish real oscillation from a pinned value.
Related: [[sim-test-harness-quirks]], [[market-exttrade-seam]],
[[dispatch-truck-park-seam]].
