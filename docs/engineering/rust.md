# Rust standard

**Kind:** standard
**Authority:** operational
**Status:** active
**Owner:** project lead
**Last verified:** 2026-08-28

Rules for Rust in this workspace. **Must** rules are checked in review; **should** rules are the
default unless a stated reason overrides them. Rules that depend on an unaccepted architectural
decision are marked *(target)* and are recommended, not required, until that decision is accepted.

## Ownership and identity

- **Must:** every mutable authoritative field has one owning module ([authority](authority.md)).
- **Must:** entity and transaction identifiers are typed newtypes. Never pass a bare `u32`/`u64`
  where an ID is meant; never let a `HumanID` be used as a `VehicleID`. The existing
  `slotmapd::new_key_type!` keys are the pattern.
- **Should:** quantities at authority boundaries are typed (`Money` and `Power` exist; `Mass`,
  `Volume` follow the same pattern). Conserved quantities are integer or fixed-point.
- **Should** *(target)*: persistent social identities (citizens, households) use append-only dense
  IDs; reusable live entities use generational handles ([entity identity](../architecture/entity-identity.md)).

## Data layout for mass entities (250k scale)

- **Must not:** allocate on the heap per hot record by default — no `String`, `Vec`, `HashMap` or
  `Box` inside a record that exists once per citizen or vehicle in a hot store. Names and
  presentation metadata go to cold or interned stores.
- **Should:** use fixed dense indexing where the key space is fixed (`EnumMap<Resource, _>` for
  per-holder stock) instead of `HashMap`.
- **Should:** keep rare state in sparse side stores rather than as `Option` fields on every record.
- **Should:** assert the size of critical hot structs at compile time
  (`const _: () = assert!(size_of::<T>() <= N);`) once a budget is decided.
- **Should:** prefer contiguous storage for hot data **where a benchmark shows the gain**; do not
  adopt SoA for elegance.

## Control flow

- **Must:** authoritative behaviour of slow actors is an explicit state-machine enum, never a
  suspended `async` future. Enums serialise, hash, replay and inspect.
- **Must:** every equal-cost or equal-priority choice has a stable tie-break on immutable identity
  or declared order ([determinism](determinism.md)).
- **Must not:** iterate a `HashMap`/`HashSet` to make an authoritative decision. Sort first, or use
  a `BTreeMap`, or a dense index.
- **Should** *(target)*: systems receive narrow typed contexts, not `&mut Simulation`
  ([authority boundaries](../architecture/authority-boundaries.md)).
- **Must not:** mutate authoritative state from a parallel worker. Workers produce intents; the
  owning module commits ([parallelism](../architecture/parallelism.md)).

## Numerics

- **Must:** document any non-obvious numerical assumption (units, scale factors, fixed-point
  scale, the meaning of a multiplier) at the definition site.
- **Must not:** enable fast-math or any flag that breaks IEEE conformance.
- **Should:** keep `f32` out of conserved or accounting quantities.

## `unsafe`

- **Must:** every `unsafe` block states the invariant it relies on and why safe code cannot
  express it. `native_app/src/init.rs` still carries the `static mut` pattern that the simulation
  crate removed; do not copy it into new code.

## Dependencies

- **Must:** follow the [dependency policy](../process/dependency-policy.md) (`cargo-deny check`
  green; only the two allowed git sources).
- **Should:** add a crate only when it buys proven correctness, performance or leverage; check
  whether it is already a transitive dependency first (`enum-map`, `bytemuck`, `arc-swap`,
  `quickcheck` are). Do not migrate to an ECS, an async runtime, a concurrent map or nightly SIMD
  as foundational architecture ([dependencies](dependencies.md)).

## Style

- `cargo fmt` and `cargo clippy -p <crate>` clean before review.
- Name what a check proves in its test name and in the `bd` close reason.

## Related

- [Authority standard](authority.md)
- [Determinism standard](determinism.md)
- [State storage (architecture)](../architecture/state-storage.md)
- [Development cycle — Phase 2 lanes](../process/development-cycle.md)
