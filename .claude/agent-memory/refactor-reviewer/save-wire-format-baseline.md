---
name: save-wire-format-baseline
description: The save wire format's verified baseline — the 13 building discriminant bytes, every *_from_u8 out-of-range policy, and what now depends on BuildingKind::ALL's order
metadata:
  type: project
---

# Save wire format — verified baseline

Verified at Phase 3 (2026-08-18) by an **out-of-tree differential oracle**: the `7c3b4ce`
match transcribed verbatim into a standalone `rustc` program alongside the new positional
impl, compared over all 13 encode inputs and all 256 decode inputs. 0 mismatches. Then the
oracle was proved non-vacuous by swapping two `ALL` entries in the standalone copy (2 encode
+ 2 decode mismatches reported).

**Why out-of-tree:** the wire-format reviewer is read-only. Copying both implementations into
`/tmp` and running `rustc --edition 2021` gives the full oracle strength with zero risk to a
dirty tree holding another track's untracked files. **This is the technique to reuse** —
it is cheaper than a mutation-and-restore cycle and cannot corrupt anyone's work.

## Building discriminants (unchanged since 7c3b4ce; `SAVE_VERSION` = 5)

0 Mine · 1 Quarry · 2 PowerPlant · 3 Factory · 4 Dwelling · 5 Warehouse · 6 Depot ·
7 BusStop · 8 ConstructionOffice · 9 WaterPump · 10 SewagePlant · 11 HeatPlant ·
12 CustomsOffice · 13..=255 → `None` (row skipped, entity left empty — pre-existing).

`SAVE_VERSION` staying 5 is **correct**, not an oversight: `from_bytes` hard-rejects a
version mismatch (`save.rs:1046`), so bumping it with an identical byte stream would break
every existing save for nothing.

## Out-of-range policy per decoder — the inventory, unchanged since 7c3b4ce

Four return `Option`/`None`, three silently substitute a default:

| decoder | policy |
|---|---|
| `kind_from_u8` | `None` |
| `class_from_u8` | `None` |
| `resource_from_u8` | `None` |
| `transport_class_from_u8` | `None` |
| `net_kind_from_u8` | **defaults** to `NetKind::Power` |
| `location_from_u8` | **defaults** to `CitizenLocation::AtHome` (also lossy in range: bytes 1 `ToWork` and 3 `ToHome` both decode to `AtHome`) |
| `phase_from_u8` | **defaults** to `FreightPhase::ToPickup` |

The Phase 3 report got this table wrong — see [[phase-log-data-driven-buildings]]. Read it
from the code, not from a report.

## What depends on `BuildingKind::ALL`'s ORDER — the number to watch

| | 7c3b4ce | after phases 3+5 |
|---|---|---|
| `ALL` references | **9, all in `src/sim/catalogue.rs`** | **21, across 4 files** |

At 7c3b4ce `ALL`'s order was catalogue-private. It is now simultaneously:

1. the **on-disk save discriminant** (`save.rs:244,248`),
2. the **Digit-3 tool cycle** order (`game/tools.rs:53`),
3. the **BUILD flyout** listing order (`game/art.rs`, `game/toolbar.rs`).

So a cosmetic UI reorder now rewrites every save file's meaning. Guarded by
`building_discriminants_are_pinned_to_the_wire_format`, which fails loudly — but the failure
text is `"Mine encodes to the wrong byte"`, which reads like "update the pin". **If a later
phase's diff reorders `ALL`, or edits that pin's literals, that is a save-breaking change
regardless of how the phase describes it.**

Two orderings are also now conflated: `spec()` is `BUILDINGS[kind as usize]` (enum
declaration order) while the save is `ALL` position. They agree today. A reorder of the enum
declaration alone is caught by `PINNED_COLUMNS`; a reorder of `ALL` alone by the pin test.

## `position(...).unwrap()` is NOT type-safe — know what actually holds it up

`kind_to_u8` = `ALL.iter().position(|&k| k == kind).unwrap()`. Nothing in the type system
makes that unreachable: `ALL: [BuildingKind; COUNT]` with `COUNT = 13` a literal constrains
length, not exhaustiveness or distinctness. `ALL` could list `Mine` twice and omit
`CustomsOffice` and still compile; `kind_to_u8(CustomsOffice)` would then panic. The pin test
does **not** catch a 14th enum variant left out of `ALL` (its length assertion compares
against `COUNT`, which would still be 13).

What actually still forces exhaustiveness crate-wide is **one unrelated compiler-exhaustive
13-arm match: `parts()` at `src/game/buildings.rs:213`** (the silhouette geometry). It is the
last `match` over `BuildingKind` in the tree with no wildcard. **If a later presentation
phase tables `parts()`, adding an enum variant stops being a compile error and becomes a
runtime panic on save + an out-of-bounds in `spec()`.** Require a replacement guard in that
phase (an exhaustiveness assertion over `ALL`).
