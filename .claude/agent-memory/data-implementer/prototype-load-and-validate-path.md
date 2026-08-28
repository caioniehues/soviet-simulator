---
name: prototype-load-and-validate-path
description: How base_mod Lua actually reaches Rust — the parse/validate seam in prototypes/src/load.rs, what validate() checks, and the two test entrypoints (test_prototypes panics, try_parse_prototypes returns)
metadata:
  type: project
---

The whole data layer funnels through `prototypes/src/load.rs`.

- `load_prototypes(base)` — the real game path. Sets `package.path` to
  `base_mod/?.lua`, executes `base_mod/data.lua`, parses each entry of the Lua
  `data` global, then calls `validation::validate`, then installs into the
  process-wide `PROTOTYPES` OnceLock.
- `test_prototypes(lua)` — thread-local fixture path used by `market.rs` unit
  tests. **It `.unwrap()`s the parse result**, so a value `validate()` rejects
  becomes a panic, not an observable error. It also installs into
  `TEST_PROTOTYPES`, which is thread-local and leaks across tests sharing a
  thread — never call it on a success path in a test that only wants to check
  refusal.
- `try_parse_prototypes(lua)` — added 2026-08-28 for sov-k3w. Same real parse +
  validate path, returns `Result<(), PrototypeLoadError>`, installs nothing.
  This is the entrypoint for asserting a bad data value is REFUSED.

`validate()` (`prototypes/src/validation.rs`) checks, per goods-company:
n_trucks vs CompanyKind; that consumption/production item ids resolve; the sign
of power_consumption/power_production; and (since sov-k3w)
`recipe.request_multiplier >= 1` plus every consumption/production
`amount >= 1`. `ValidationError::InvalidField(company, field, message)` is the
right variant for any new numeric range check — it already carries exactly
(company name, field name, what was wrong).

**Which command proves what.** `cargo test -p prototypes` runs `tests::test_base`,
which DOES load the real `base_mod` through `load_prototypes("../")` — so it is
the cheapest proof that base_mod itself still validates. It is NOT proof about
simulation-side consumers: a bad value can pass parse and only explode in
`simulation`. Always run both, and read the `running N tests` line.

Related: [[recipe-numeric-fields-are-consumed-unsigned]]
