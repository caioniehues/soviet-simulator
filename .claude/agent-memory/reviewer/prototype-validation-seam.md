---
name: prototype-validation-seam
description: Where Lua data invariants are enforced (validate()), which numerics are still unguarded, and the get_lua unwrap_or type-swallow trap
metadata:
  type: project
---

`prototypes/src/validation.rs::validate()` is the ONLY invariant gate on base_mod data,
and it is reached by every parse path — `load_prototypes`, `test_prototypes`, and (since
sov-k3w) `try_parse_prototypes` all funnel through `parse_prototypes_boxed` in
`prototypes/src/load.rs`. So a fixture in a unit test is validated too.

`try_parse_prototypes(lua) -> Result<(), PrototypeLoadError>` is the non-installing test
entry: fresh `Lua`, drops the Box, never writes `PROTOTYPES` (lib.rs:105) or
`TEST_PROTOTYPES` (lib.rs:111) — those are written only at load.rs:33 and load.rs:12.
It is the right way to assert a data value is REFUSED without panicking.

**Guarded as of sov-k3w:** n_trucks, referenced item ids, power sign,
`request_multiplier >= 1`, every consumption/production `amount >= 1`.

**Still unguarded, same defect shape (verified 2026-08-28):**
- `storage_multiplier` — `souls/goods_company.rs:64` `(item.amount * storage_multiplier) as u32`
  wraps on negative; `:46` `item.amount * (storage_multiplier + 1)` gives 0 → silent halt.
- `recipe.duration` — `souls/goods_company.rs:205` divides by `duration.seconds() as f32`;
  0 gives +inf progress (runaway, no panic).

**Trap — `get_lua(...).unwrap_or(default)` swallows TYPE errors, not just absence.**
`prototypes/src/types/recipe.rs:63` reads `request_multiplier` that way, so
`request_multiplier = true` loads as 1 (honest) and passes validation — the core loop is
silently deleted. The correct form is `get_lua_opt(table, "x")?.unwrap_or(d)`, as used at
`prototypes/src/prototypes/goods_company.rs:41-42`. Grep for `get_lua(` + `.unwrap_or(`
whenever reviewing a data-layer diff.

Related: [[review-method-patterns]], [[repo-has-no-test-ci]].
