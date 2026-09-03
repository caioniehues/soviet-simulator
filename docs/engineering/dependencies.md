# Dependency standard

**Kind:** standard
**Authority:** operational (the [dependency policy](../process/dependency-policy.md) and its CI job are the enforced baseline)
**Status:** active
**Owner:** project lead
**Last verified:** 2026-08-28

## Rules

1. **Must:** `cargo-deny check` (pinned 0.20.2) passes locally and in CI before a lockfile change
   lands. Re-record the policy document whenever `Cargo.lock` changes.
2. **Must:** only crates.io and the two allowed git sources (`emilk/egui`, `Uriopass/yakui`).
   No new git or path sources without a policy change.
3. **Should:** pin the two git sources to a `rev`. Today neither is pinned; `cargo update` advances
   them silently and the build is not reproducible (release-engineer finding; Lane C1-01/02).
4. **Must:** before adding a crate, check whether it is already a transitive dependency
   (`cargo tree -i <crate>`): `enum-map`, `bytemuck`, `arc-swap`, `tracing`, `quickcheck`,
   `fixedbitset` are.
5. **Must:** a new crate states what it buys — correctness, performance or leverage — with a
   measurement or a concrete gap; "popular" is not a reason.
6. **Must not:** adopt an ECS (`bevy_ecs`, `hecs`, `shipyard`), an async runtime per actor, a
   concurrent map for authoritative state, or nightly `std::simd` as foundational architecture.
7. **Should:** prototype before adopting: `typed-index-collections`, `fixed`, `soa-rs`/`soa_derive`,
   Salsa, `good_lp` backends, `rkyv`, `postcard + zstd`. Adopt-class candidates (Rayon, ArcSwap,
   FixedBitSet, Roaring, SmallVec, bytemuck, Shuttle, Criterion) still need a use and a benchmark.
8. **Must:** every workspace member carries `publish = false` (the omission once made two
   `cargo-deny` checks inert).
9. **Must:** licence obligations are tracked; the repository is GPL-3.0 by inheritance, permanently.

## Reference

The crate-by-crate verified findings — versions, licences, maintenance, fit — are in
[Rust crates research](../research/engineering/rust-architecture-crates.md) and the earlier
[project-fit survey](../research/awesome-rust-project-fit.md).

## Related

- [Dependency policy](../process/dependency-policy.md)
- [Rust standard](rust.md)
- [Technical stack research](../explanation/research/technical-stack-upstream-2026-08-24.md)
