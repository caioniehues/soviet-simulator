# Writing evidence tests

**Kind:** guide
**Authority:** operational
**Status:** active
**Owner:** project lead
**Verified-at:** `4e9e930b2a73`
**Last verified:** 2026-08-28

An evidence test proves one specification claim and is seen failing against the wrong
implementation the spec names. The chain `test ↔ invariant ↔ specification ↔ mechanic` must be
followable in both directions.

## Name and place

- File: `simulation/src/tests/scenarios/<subsystem>.rs`; declare `mod <subsystem>;` in
  `scenarios/mod.rs`.
- Function: `evid_<subsystem>_<claim>` for a spec's `EVID-*` row; `scenario_<nnnn>_<behaviour>`
  for a story scenario; `sov_<id>_<behaviour>` for a `bd` fix.
- Doc-comment: the `SPEC-*` anchor it proves, the invariant from the [invariants index](../reference/invariants.md),
  and the causal chain it guards (the `sov_ahw_*` test's comment is the model).

## Shape

```rust
#[test]
fn evid_logistics_pickup_delivery_cancel_conservation() {
    let mut ctx = TestCtx::new();
    let (seller, buyer, ..) = setup_seller_buyer(&mut ctx, 300.0);     // scenarios/mod.rs helpers
    // arrange stock, post a request
    // act: advance until the truck is Loading, then cancel
    assert!(drain_dispatches(&mut ctx, 2_000));
    // assert the identity: source + destination + custody + embedded + sinks == initial + sources
}
```

`TestCtx::tick()` runs the round-trip determinism check every tick; `advance_ticks(n)` every 25.
Use `build_house_at` (explicit position) rather than `build_house_near` (depends on auto-lots,
which are slated for removal).

## See it fail

Break the mechanism the test protects — credit the destination at reservation, satisfy the need at
route arrival, activate a Site before completion, double-apply a meter delta — run the test, paste
the red output into the `bd` comment, revert. If the test stays green, it is not evidence
([testing standard](../engineering/testing.md)). For eligible changes, cargo-mutants does this
mechanically ([mutation policy](../process/mutation-policy.md)).

## Assert behaviour, not arithmetic

Assert the story's claim (the buyer goes without; the truck returns physically; the surplus is
still on hand) rather than a number that happens to hold. Assert conservation explicitly for any
economy seam. Test idempotency by applying a transition twice.

## Filters must run something

```bash
cargo test -p simulation evid_logistics -- --nocapture   # confirm "running N tests", N ≥ 1
```

## Close the loop in the docs

The spec's `EVID-*` row now points at a real test; update its "Current substrate" section; the
generated roadmap counts it after the `--check` commands in Phase 6 of the development cycle.

## Related

- [Testing standard](../engineering/testing.md)
- [Simulation transitions standard](../engineering/simulation-transitions.md)
- [Invariants index](../reference/invariants.md)
- [Adding a specification](adding-a-specification.md)
