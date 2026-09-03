# Dependency standard

**Kind:** standard
**Authority:** operational (the [dependency policy](../process/dependency-policy.md) and its CI job are the enforced baseline)
**Status:** active
**Owner:** project lead
**Last verified:** 2026-09-03

## Rules

Each rule carries its enforcement status. **Gate-enforced** means `cargo-deny check`
(pinned 0.20.2, CI job `.github/workflows/dependency-policy.yml`) fails on violation,
with the enforcing `deny.toml` lines cited. **Manual** means no gate checks it; reviewers
enforce it by hand. Gate coverage was verified 2026-09-03 against `deny.toml`.

1. **Must [gate-enforced]:** `cargo-deny check` (pinned 0.20.2) passes locally and in CI
   before a lockfile change lands. (The second half — re-recording the policy document
   whenever `Cargo.lock` changes — is **manual**; no gate checks prose.)
2. **Must [gate-enforced, `deny.toml:82-89` `[sources]`]:** only crates.io and the two
   allowed git sources (`emilk/egui`, `Uriopass/yakui`). No new external git or registry
   sources without a policy change. Internal path dependencies between workspace members
   are separately permitted by `allow-wildcard-paths = true` (`deny.toml:74-80`), which
   covers only local path specifications, never registries — so a new *external* path
   source still needs a policy change.
3. **Should [manual — advisory only]:** pin the two git sources to a `rev`. Today neither
   is pinned; `cargo update` advances them silently and the build is not reproducible
   (release-engineer finding; Lane C1-01/02). Git revision pinning is tracked separately
   (`sov-buw` owns the rev-pin gate); this rule stays advisory and this document makes
   no gate claim about it.
4. **Must [manual — `cargo tree -i <crate>` by hand]:** before adding a crate, check
   whether it is already in the graph. Verified 2026-09-03 against the manifests and
   `Cargo.lock`: `bytemuck` is **direct** (`engine`, `native_app`), `arc-swap` is
   **direct** (`simulation` dependencies), `quickcheck` is **direct** (`simulation`
   dev-dependencies) — none of the three is transitive. `enum-map` and `tracing` are
   transitive-only. `fixedbitset` is in neither the manifests nor `Cargo.lock`; it is
   not a dependency at all — do not cite it as one.
5. **Must [manual]:** a new crate states what it buys — correctness, performance or
   leverage — with a measurement or a concrete gap; "popular" is not a reason.
6. **Must not [manual — no gate entry]:** adopt an ECS (`bevy_ecs`, `hecs`, `shipyard`),
   an async runtime per actor, a concurrent map for authoritative state, or nightly
   `std::simd` as foundational architecture. `deny.toml:70-80` `[bans]` carries only
   `multiple-versions = "warn"`, `wildcards = "deny"` and `allow-wildcard-paths`; there
   are no `[bans]` deny entries for these architectures, so reviewers enforce this rule.
7. **Should [manual]:** prototype before adopting: `typed-index-collections`, `fixed`,
   `soa-rs`/`soa_derive`, Salsa, `good_lp` backends, `rkyv`, `postcard + zstd`.
   Adopt-class candidates (Rayon, ArcSwap, SmallVec, bytemuck, Shuttle, Criterion) still
   need a use and a benchmark. (FixedBitSet and Roaring are not in the graph at all —
   see rule 4 — so a proposal for either starts from absence, not presence.)
8. **Must [gate-enforced indirectly]:** every workspace member carries `publish = false`
   (the omission once made two `cargo-deny` checks inert). The setting is load-bearing
   for `deny.toml:61-68` `[licenses.private]` and `deny.toml:74-80`
   `allow-wildcard-paths`; without it those gates go inert or error, so a violation
   surfaces as a gate failure — but no check asserts the metadata directly.
9. **Must [gate-enforced, `deny.toml:37-59` `[licenses]`]:** licence obligations are
   tracked; the repository is GPL-3.0 by inheritance, permanently. The allowlist plus
   the single narrow `epaint` exception is the gate; anything outside it fails.

### Gate settings with no rule above

These `deny.toml` settings are gate-enforced but were previously undocumented here;
they are recorded so the inventory of what the gate checks is complete:

- `[graph] all-features = true` (`deny.toml:3-4`) — the gate resolves the full feature
  union, not the default-feature graph.
- `yanked = "deny"` (`deny.toml:33`) — gate-enforced; the two locked yanked versions
  (`bytemuck@1.16.1`, `bytes@1.6.0`) pass only through explicit time-limited `ignore`
  entries, owned by the project lead, review 2026-11-25.
- `unmaintained = "all"` (`deny.toml:34`), `unsound = "all"` (`deny.toml:35`) —
  gate-enforced; findings require explicit handling, none are currently excepted.
- `confidence-threshold = 0.8` (`deny.toml:38`) — gate-enforced; licence detections
  below 0.8 do not satisfy the allowlist check.

## Reference

The crate-by-crate verified findings — versions, licences, maintenance, fit — are in
[Rust crates research](../research/engineering/rust-architecture-crates.md) and the earlier
[project-fit survey](../research/awesome-rust-project-fit.md).

## Related

- [Dependency policy](../process/dependency-policy.md)
- [Rust standard](rust.md)
- [Technical stack research](../explanation/research/technical-stack-upstream-2026-08-24.md)
