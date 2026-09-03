# State storage

**Kind:** architecture
**Authority:** advisory
**Status:** draft
**Owner:** architecture
**Last verified:** 2026-08-28

## Current substrate

- Entities: array-of-structs in `HopSlotMap<ID, Ent>` per type (`world.rs`); `HopSlotMap` is not
  contiguous.
- Stock: `Market.markets: BTreeMap<ItemID, SingleMarket>`, each with `BTreeMap<SoulID, i32>` for
  capital and similar maps for reserved and requested (`economy/market.rs`).
- Hash maps everywhere use `FxHasher` (`common/src/hash.rs`).
- No SoA, no bitsets, no fixed resource arrays, no compile-time size assertions.
- `PersonalInfo.name: String` is boxed on every human — a heap allocation per citizen.

## Target design

**Dense cores plus sparse side stores.** Hot 250k-scale state is contiguous and small; rare
states live beside it:

```text
CitizenCore[]  CitizenActivity[]  CitizenResidence[]
PregnancyStore  IllnessEpisodeStore  EducationEnrollmentStore  HousingQueueStore  SocialEdgeStore  CitizenBodyStore
```

Illustrative core (not a ratified definition):

```rust
struct CitizenCore { household: HouseholdId, birth_day: Day, workplace: Option<WorkplaceId>,
                     qualification: QualificationCode, activity: ActivityCode, next_event: SimTime, flags: CitizenFlags }
```

Names and presentation metadata go to cold or interned stores. Benchmark hand-written SoA against
`soa-rs`/`soa_derive` before adopting a crate (`bytemuck` is already a transitive dependency for
POD casting).

**Fixed resource arrays.** The 1.0 catalogue is small and fixed: `EnumMap<Resource, Qty>` per
holder replaces `HashMap<Resource, f32>` — deterministic iteration, cache locality, no hashing, no
allocation, compact serialisation. `enum-map` is already a transitive dependency via `egui_extras`
(Lane C1 §4.2) — possibly the single highest-impact data-structure change for the economy.

**Bitset society.** Derived cohort indexes:

```text
working_age ∩ technical_qualified ∩ district_7 ∩ available ∩ reachable
```

Only the narrowed candidates run expensive household or employment evaluation — *filter cheaply,
think expensively.* `fixedbitset` for dense populations, `roaring` for sparse memberships. Needs
dense citizen IDs ([entity identity](entity-identity.md)).

**Budget.** Lane G's arithmetic: ~320 bytes per citizen for the proposed state → 80 MB at 250k,
larger than any cache; a naïve full pass costs ~2.7 ms sequential and 3–8× worse scattered. Set
explicit byte budgets for `CitizenCore`, `HouseholdCore`, vehicle hot state, `Haul`, scheduled
event, causal fact, route-cache entry; assert them at compile time; make them an accepted decision
after profiling ([performance](performance.md)).

## Migration

1. `EnumMap` for per-holder stock in `SingleMarket` (independent; changes serialisation).
2. Record/body split ([entity identity](entity-identity.md)).
3. SoA `CitizenCore` once the record exists; measure against AoS.
4. Bitset indexes maintained from the [change journal](change-journal.md).

## Open decisions

- Hand-written SoA versus a crate — decide by benchmark.
- Byte budgets — decide after profiling, record as a decision.

## Related

- [Entity identity](entity-identity.md)
- [Performance](performance.md)
- [Rust standard](../engineering/rust.md)
- [Rust crates research](../research/engineering/rust-architecture-crates.md)
