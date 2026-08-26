---
name: sim-test-setup-traps
description: Substrate facts about building scenario tests in simulation/src/tests — footprints, truck spawning, and unwrap panics that masquerade as real failures
metadata:
  type: project
---

Verified the hard way while writing round-4 tests for sov-dispatch-wedge-ab4
(2026-08-26).

**Every `kind = "factory"` company spawns its own trucks.**
`goods_company.rs:129-137` spawns `proto.n_trucks` when
`ckind == CompanyKind::Factory`. In `base_mod/companies.lua`, BOTH
`cereal-farm` and `flour-factory` are `kind = "factory"` with `n_trucks = 1`.
So `setup_real_seller_buyer` (ledger.rs) yields a city with TWO trucks before
you spawn any yourself. Any test premised on "exactly one truck" is wrong.
(Stores get none — that half of the known trap is still true.)

**Company footprints come from Lua and they are large.** `cereal-farm` is
`size = 120.0`, `flour-factory` is `size = 80.0`. `setup_real_seller_buyer`
places them at x=30 and x=150. Placing a second pair between them overlaps and
`build_company_at` panics.

**`build_company_at` unwraps `build_special_building`** (`hoarding.rs:40`). A
placement that does not fit surfaces as
`called Option::unwrap() on a None value: hoarding.rs:40` — which looks like a
real test failure but is a setup error. If a new test goes red there, suspect
geometry before suspecting the code under test.

**A red test for the wrong reason is not a red test.** Both of the above
produced convincing-looking failures that had nothing to do with the defect.
Always read the panic location, not just the fact of failure.

Related: [[dispatcher-truck-pool]].
