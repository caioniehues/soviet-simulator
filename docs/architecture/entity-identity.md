# Entity identity

**Kind:** architecture
**Authority:** advisory
**Status:** draft
**Owner:** architecture
**Last verified:** 2026-08-28

## Current substrate

All entities are generational slot-map keys: `VehicleID`, `TrainID`, `HumanID`, `WagonID`,
`FreightStationID`, `CompanyID` via `slotmapd::new_key_type!` (`simulation/src/world.rs`).
`slotmapd` is Uriopass's fork of `slotmap` whose reason to exist is that a serialise → deserialise
cycle keeps key generation and iteration order identical — essential for saves that use keys as
persistent identity. It has zero GitHub stars and its author says "use the original if you do not
need this invariant" (Lane C1 §2). A slot is reused after deletion with a bumped generation.

`HumanEnt` is one struct holding physics, routing, decision, desires and a boxed `PersonalInfo`
(`name: String, age, gender`). There is no citizen without a body.

Typed IDs exist; typed **units** do not beyond `Money` (i64 fixed-point) and `Power`. Mass, volume,
energy and quantities are bare `i32`/`f32`.

## Target design

**Two identity families.**

| Family | For | Why |
|---|---|---|
| Append-only dense typed IDs (`CitizenId`, `HouseholdId`, major institutions) | things whose historical identity must never be reused | biographies, chronicle, causal facts: a dead Citizen #N stays #N |
| Generational slot-map handles | bodies, vehicles, itineraries, hauls, active movement objects | reuse is safe and cache-friendly |

Non-zero integer IDs keep `Option<Id>` at integer size (niche layout); assert it at compile time.

**Citizen materialisation levels** (target; none implemented):

```text
L0 CitizenRecord       persistent biography, membership, core state
L1 scheduled actor     next relevant event
L2 active activity     service, queue, route interaction
L3 CitizenBody         physical movement state
L4 render instance     GPU representation (bounded, per charter)
```

Identity survives every level transition.

**Typed IDs and units at authority boundaries:** `HaulId`, `ProductionRunId`, `DeliveryId`,
`WaterTransferId`, `Mass`, `Volume`, `Energy`; fixed/integer for conserved quantities. Selective
dimensional typing (`uom`-style) at physics boundaries only, if profiling and ergonomics allow.

## Migration (Lane C2 §3.2)

1. Extract `PersonalInfo`, `Home` and employment history into a `CitizenRecord` store in
   `Resources`, keyed by a new dense `CitizenId`; `HumanEnt` keeps hot movement and decision
   state and holds its `CitizenId`. `spawn_human` writes both. **This changes the save format** —
   the [migration seam](persistence.md) comes first.
2. Add `Mass`, `Volume` newtypes following the `Power`/`Money` pattern; convert at seams.
3. Dense IDs unlock [bitset cohorts](state-storage.md).

## Open decisions

- Vendor `slotmapd` or upstream its determinism fix to `slotmap`, if the fork goes unmaintained.
- Whether "new save required" is acceptable pre-1.0 for the record/body split.

## Related

- [State storage](state-storage.md)
- [Persistence](persistence.md)
- [Citizens (design)](../simulation/society/citizens.md)
- [Citizen architecture proposal](../plan/proposals/citizen-architecture.md)
- [Rust standard](../engineering/rust.md)
