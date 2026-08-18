# F1 close-out + Phase 3 report — the band witness, and the save discriminant

## Baseline

`cargo test --lib` at start: **138 passed; 0 failed** (commit `7c3b4ce`). This work takes it
to **140**, both additions being guards rather than behaviour.

**Read the gate section before comparing numbers.** Partway through this phase the
presentation track landed `src/game/art.rs`, so a whole-lib run is now 146 tests and is
currently **red on one test that is not mine**. My track is green in isolation; details and
a coordination hazard are at the end.

---

# F1 — the band column has a witness again

## What was added

`PINNED_COLUMNS` grew a fifth tuple element holding the `(resource, min_pct, max_pct)`
triples, and `default_policies_column_holds_its_pinned_bands` compares the table's band
column against it per resource. Extracted a `PinnedRow` type alias, since a five-element
tuple in an array type is past the point where it reads.

Compared **per resource rather than slice against slice**, deliberately: the fold is
order-insensitive (`StoragePolicies` is a dense per-resource array), so a row listing its
bands in a different order behaves identically, and a golden record should not fail on a
difference the game cannot see. `Option` comparison is what makes membership checkable —
`None` vs `Some(..)` is the Depot case.

The old fold test is kept and re-documented as the other half of a chain of custody: the
pin proves the row still lists the right triples, the fold test proves
`storage::default_policies` turns those triples into the right `StoragePolicies`.

## Proof: the review's own four-way corruption, each one caught

The review applied four band corruptions at once and got 138 green. Re-applied to this
tree, each is now named. The assert aborts at the first mismatch, so I peeled them back one
at a time rather than claiming all four from one run:

| corruption | output |
|---|---|
| Quarry Gravel `0.0,0.05 → 0.0,0.5` | `Quarry / Gravel  left: Some((0.0, 0.5))  right: Some((0.0, 0.05))` |
| Dwelling Goods `0.5,1.0 → 0.15,1.0` | `Dwelling / Goods  left: Some((0.15, 1.0))  right: Some((0.5, 1.0))` |
| HeatPlant Coal `0.6,1.0 → 0.9,1.0` | `HeatPlant / Coal  left: Some((0.9, 1.0))  right: Some((0.6, 1.0))` |
| **Depot gains a Coal band it has never had** | `Depot / Coal  left: Some((0.3, 0.7))  right: None` |

The Depot line is the one that matters — it is the added-arm case, the mirror of a dropped
arm, and it was invisible to the entire suite before. All four reverted; the band column is
byte-identical to `7c3b4ce`.

## A second lesson from widening the tuple, worth recording

Widening `PINNED_COLUMNS` from four elements to five broke two of the three existing pinned
tests *at compile time* — and one of them, `workers_needed`, would have broken **silently**
had the types happened to line up: it destructured with `let (pinned_kind, .., workers)`,
where `..` means "everything in the middle", so `workers` silently re-bound from the vacancy
count to the band slice. It only failed because `u32` and `&[(ResourceKind, f32, f32)]` are
not comparable.

Both patterns are now positional and explicit (`(pinned_kind, _, _, workers, _)`). The
general point is the same one the review made about the fold test: `..` in a tuple pattern
is a silent re-binding hazard the moment the tuple's shape changes.

---

# Phase 3 — the save discriminant by position

## The change

```rust
fn kind_to_u8(kind: BuildingKind) -> u8 {
    BuildingKind::ALL.iter().position(|&k| k == kind).unwrap() as u8
}

fn kind_from_u8(v: u8) -> Option<BuildingKind> {
    BuildingKind::ALL.get(v as usize).copied()
}
```

Byte-for-byte the shape `resource_to_u8`/`resource_from_u8` (`save.rs:303-309`) already
uses, which is what the plan asked for. 31 lines of hand-paired match deleted. `SAVE_VERSION`
stays **5** — untouched.

## The emitted bytes did not change — proved exhaustively, not by eye

I did not trust the transcription. I extracted the committed functions verbatim
(`git show 7c3b4ce:src/sim/save.rs`), pasted them into the test module under
`committed_kind_to_u8`/`committed_kind_from_u8`, and ran a differential:

```rust
for kind in BuildingKind::ALL { assert_eq!(kind_to_u8(kind), committed_kind_to_u8(kind)); }
for v in 0..=u8::MAX { assert_eq!(kind_from_u8(v), committed_kind_from_u8(v)); }
```

```
test sim::save::tests::temporary_differential_against_the_committed_encoder ... ok
```

All 13 encodes agree and **all 256 possible decode inputs agree**, including the entire
out-of-range region 13..=255 mapping to `None`. That is a stronger statement than the pinned
test alone makes: the wire format is identical for every byte that could appear on disk, not
just for the thirteen that are meant to. Oracle removed afterwards; `grep` confirms no trace
remains.

## The pinned test, and why it is the only thing standing here

`building_discriminants_are_pinned_to_the_wire_format` asserts hard-coded `0u8..=12`
against `kind_to_u8`, and `kind_from_u8(byte) == Some(kind)` in the other direction, plus
`kind_from_u8(13) == None` and `kind_from_u8(u8::MAX) == None`. It also asserts its own
length equals `BuildingKind::COUNT`, so appending a kind without pinning it fails rather
than passing vacuously. **No reference to `ALL`'s positions anywhere** — that would be
circular in exactly the way F1 was.

**The demonstration the lead asked for.** I reordered `BuildingKind::ALL`, swapping
`WaterPump` and `SewagePlant`:

```
test sim::save::tests::building_discriminants_are_pinned_to_the_wire_format ... FAILED
test sim::save::tests::round_trip_preserves_the_sim_state_hash ... ok
test sim::save::tests::restore_into_the_same_world_is_identical ... ok
test sim::save::tests::file_round_trip_and_version_gate ... ok
test sim::save::tests::loaded_world_resumes_production_and_commutes ... ok
test sim::save::tests::mid_trip_save_keeps_the_cargo_and_the_order ... ok
test sim::save::tests::mid_commute_save_normalizes_travellers_home ... ok

assertion `left == right` failed: WaterPump encodes to the wrong byte
  left: 10
 right: 9
```

Then I made it worse, because a partial reorder is the easy case. I applied a **fully
self-consistent** reorder — the enum declaration in `buildings.rs`, `ALL`, the `BUILDINGS`
rows and the `PINNED_COLUMNS` rows, all four moved together, the shape a tidy-up commit
would actually take:

```
test result: FAILED. 139 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

**139 of 140 green.** Every catalogue test, every position test, and all six save tests
including the round-trip hash passed while every save file on disk silently changed meaning.
One test in the tree caught it, and it is the one this phase added. That is the concrete
form of the review's F3 warning, and it is now closed.

## `.get()` versus indexing is load-bearing, not stylistic

Confirmed by substituting `Some(BuildingKind::ALL[v as usize])`:

```
thread '...building_discriminants_are_pinned_to_the_wire_format' panicked at src/sim/save.rs:248:10:
index out of bounds: the len is 13 but the index is 13
```

A malformed or future-version save turns into a panic on the player's file instead of the
dropped row `load_snapshot` already handles (`save.rs:769`, `let Some(kind) = … else`).
Reverted; `.get().copied()` is what shipped, and the `None` assertions in the test pin that
behaviour so it cannot be "simplified" later.

`spec()` still indexes `BUILDINGS[kind as usize]` directly and correctly — its argument is a
`BuildingKind`, which is in range by construction. The distinction is that a byte off disk
is not a `BuildingKind` and must not be treated as one.

---

## Gates

**Scoped to my track**, because the whole-lib run is not currently mine alone:

```
$ cargo test --lib sim::
test result: ok. 126 passed; 0 failed; 0 ignored; 0 measured; 22 filtered out

$ cargo test --lib sim::catalogue
test result: ok. 10 passed; 0 failed

$ cargo test --lib sim::save
test result: ok. 7 passed; 0 failed
```

`cargo clippy --lib --tests` — **nine warnings, the same nine**, none in `catalogue.rs` or
`save.rs`: `game/juice.rs:125`, `game/vehicles.rs:423` (moved from `:436` by the
presentation track), `sim/dispatch.rs:179`, `sim/households.rs:129`, `sim/network.rs:116`,
`:117`, `:132`, `sim/water.rs:175`, `sim/wires.rs:348`.

`rustfmt --edition 2024 --check src/sim/catalogue.rs src/sim/save.rs` — clean.

Bench gates not re-run this phase: nothing here is on a hot path. `kind_to_u8` runs once per
building per *save*, and the linear `position()` scan over 13 elements is the same cost
`resource_to_u8` has always paid on a much hotter column.

## Scope

Two files, both `src/sim/`: `catalogue.rs` (the band pin) and `save.rs` (the discriminant).
Zero intentional behaviour changes — the wire format is proved identical for all 256 byte
values. `SAVE_VERSION` is still 5. The `Without<ConstructionSite>` gap in `run_heat_plants`
and `solve_water` is untouched, still Phase 4's.

---

## Found, not acted on

- **A cross-track hazard, and I tripped it.** `cargo fmt` at the repo root reformats the
  *whole crate*, including another agent's untracked, in-flight files. `src/game/art.rs`
  appeared mid-phase as untracked while I was running bare `cargo fmt` after each edit. I
  have switched to `rustfmt --edition 2024 <my files>` and recommend the same for anyone
  else working in parallel — a repo-wide formatter run is a write to someone else's track.
- **The whole-lib suite is currently red, and not from this work.**
  `game::art::tests::shipped_column_holds_its_pinned_products` fails with
  `HeatPlant shipped product: left: Coal, right: Goods` (`src/game/art.rs:396`). That file
  is untracked and being written right now, so this is almost certainly work in flight
  rather than a defect — flagging it only so the number `145 passed; 1 failed` is not read
  as mine. Every sim test passes.
- **`net_kind_to_u8`, `class_to_u8`, `transport_class_to_u8` and the rest are still
  hand-paired matches** in the same section of `save.rs`. They have the same silent-failure
  shape `kind_to_u8` had, and `NetKind`/`RoadClass`/`TransportClass` have no `ALL` array to
  index. Out of scope here — recording it because "the discriminant maps are now
  position-based" would be false if said generally; only `BuildingKind` and `ResourceKind`
  are.
- **`net_kind_from_u8` and `class_from_u8` degrade differently from `kind_from_u8`**: they
  return a *default* variant for an unknown byte (`_ => NetKind::Power`) rather than `None`.
  So an unknown net kind silently becomes a power line, where an unknown building kind drops
  the row. Not a regression and not mine to change, but the two policies disagree and only
  one of them is written down.
- **The `..` tuple-pattern hazard** described under F1 above will bite again if
  `PINNED_COLUMNS` grows a sixth element. The three pinned tests now use explicit positional
  patterns; keep them that way.
