---
name: vacuous-checks-data-driven-buildings
description: Tests in this refactor that pass without proving anything — which ones died, how they died, and which were repaired. Re-check every phase.
metadata:
  type: project
---

# Checks that look green but prove less than they appear to

**Why:** each of these was demonstrated vacuous by planting a deliberate error and watching
the suite stay green. A refactor repeats its own mistakes; the gap that hid a bad row in
Phase 1 will hide one in Phase 6.

**How to apply:** when an implementer's evidence is "test X is green", check X against this
list first. If it is here, the green is not evidence and you need to verify by hand.

## 0. THE MECHANISM TO WATCH — a test goes vacuous without its own text changing

Learned the hard way at Phase 2. I had written "`default_policies_matches…` is strong — do
not let anyone *simplify* it". Nobody simplified it. Its diff was a rename and a doc comment.
It died anyway, because the **oracle it compared against was deleted underneath it**: the
hand-written `match` became a fold of the very table the test compares to.

**So: never assess a test by its own diff.** Assess it by what its two sides resolve to
*after* the phase. Every phase that deletes a per-kind `match` silently converts that column's
equivalence test into `table == f(table)`. This will recur at Phase 6 for power/water/heat.

The cheap detector: corrupt a table row and run the suite. If nothing fails, the column has
no witness.

## 1. `round_trip_preserves_the_sim_state_hash` cannot catch a symmetric reorder

`save.rs`. Encode (`kind_to_u8`) and decode (`kind_from_u8`) run in the **same binary**.
Reorder both consistently and the hash round-trips perfectly while every save file on disk
decodes to the wrong building kinds.

This is the strongest test in the tree and it is *blind* to exactly the failure Phase 3
risks. The only real guard is a discriminant test asserting against **hard-coded integers**
(`Mine == 0` … `CustomsOffice == 12`), never against `ALL`'s own positions.

Same trap for `restore_into_the_same_world_is_identical`.

**Also confirmed at Phase 2:** the save suite asserts nothing about footprint, capacity or
vacancies. Corrupting Mine's three columns at once left all 6 save tests green. The save
tests exercise the changed path but are not evidence for it.

## 2. The storage-band column had NO witness at Phase 2 — CLOSED at Phase 3

**Status 2026-08-18: fixed and verified by mutation, including the case the implementer
did not test (deleting a band).** See [[baseline-building-numbers]] for the six mutations
and their verbatim output. Keep the history below — the *shape* of the gap is the reusable
lesson, and Phase 6 will recreate it for power/water/heat.

### History (Phase 2)

Proven: corrupted Quarry's max (0.05→0.5), Dwelling's min (0.5→0.15), HeatPlant's min
(0.6→0.9) **and added a Coal band to Depot, which has never had one** — all four at once —
and got `138 passed; 0 failed`.

`default_policies_folds_every_band_the_row_lists` (`catalogue.rs:417`) is now
table-vs-fold-of-table. The only band literals left anywhere are incidental, in `storage.rs`'s
own tests: PowerPlant `min == 0.6`, Mine `max == 0.05`, Warehouse's three `(0.2, 0.6)`.
Unwitnessed: Quarry, Factory, Dwelling, HeatPlant, PowerPlant's max, Mine's min, and the
*absence* of bands on all six no-band kinds.

The added-band-on-Depot case is the one to remember: **membership** is unguarded, not just
values. That is the added/dropped arm shape, undetectable.

Values are nonetheless correct today — verified by oracle (see below). Flagged as F1 at
Phase 2 with the fix scoped (extend `PINNED_COLUMNS` with a 4th element).

## 3. `recipe_rates_match_the_named_production_constants` — REPLACED at Phase 1b, hole closed

Was vacuous two ways (only 5 of 13 kinds; both sides named the same constant). Replaced by
`one_frame_moves_exactly_what_the_recipe_claims`, which I verified non-vacuous by six
mutations. It genuinely catches: a fabricated recipe on a non-producer, a wrong-constant row
once either constant is rebalanced, a rogue system output on a recipe-less kind, and a rogue
system *draw* on a zero-capacity kind. All 13 kinds, both directions.

**Residual blind spot, acceptable:** a Mine↔Quarry *cross*-swap (each row naming the other's
equal-valued constant) is 138-green with no warning. A one-sided swap at least trips
`warning: unused import`. Both become loud failures the moment a constant is rebalanced, so
this is latent, not live.

## 4. The three equivalence tests died at Phase 2 and were repaired with `PINNED_COLUMNS`

`footprint`/`inventory_capacity`/`workers_needed` equivalence tests became `table == table`
the moment the matches were deleted. Verified: corrupting Mine's row left all three passing at
the old shape. Now pinned to a golden literal row in `mod tests`, and verified that all three
fail on that same corruption, and that the `ALL`-alignment assertion fires on a row reorder.

**`PINNED_COLUMNS` is now itself a thing to protect.** If a later phase "deduplicates" it
against the table, all three guards die at once and silently.

## 5. The utility tests re-transcribe the match instead of calling it

`catalogue.rs` — power, water and heat each rebuild the expected match **inside the test
body**. They catch table typos but not a mis-reading reproduced in both places. Still
correctly the transcribed-match kind at Phase 2, because `solve_power`, `solve_water` and
`attach_heat_components` still own their matches. **Phase 6 deletes those matches and these
three go circular exactly as the lookup tests did.** Whoever gates Phase 6 must require a
replacement witness *in the same phase*.

## 6. `Surface::mat()` in `game/art.rs` is the unpinned bridge — PROVEN vacuous at Phase 5

The six art pins compare the **tuple** `(role, shade, metallic)` read straight off the
`Surface` struct fields. They never compare a built `Mat`. `Surface::mat()` is the only code
that turns a pinned row into the material the game actually renders
(`kind_material` = `art(kind).wall.mat()`, `roof_material` = `art(kind).roof.mat()`).

Proven 2026-08-18: deleting the single line `.shade(self.shade)` from `mat()` — which
discards every wall and roof shade for all 13 kinds, the whole point of the column — gives
`148 passed; 0 failed`. Deleting both `.shade` and `.metallic` also gives 148 green.

**Same shape as F1**: the values have literals, the *consumer* of the literals has none. The
generic form of this gap: *a pin that reads private struct fields cannot witness the accessor
that everything else in the codebase goes through.* Look for it wherever a table column is
read via a method rather than directly. Closing it needs one assertion on a built material
(e.g. `art(Mine).wall.mat().build().base_color != Role::SootBrick.color()` at some shade, or
a pin on `.build().metallic` for RUST_ROOF).

## 7. Where the art pins are genuinely strong (do not re-test these)

All eight of the following were established by mutation on 2026-08-18 and each named the
right kind and value in its failure text — see [[phase-log-data-driven-buildings]] Phase 5.
Wall `Role`, wall shade at 0.02 resolution, roof surface, roof metallic (via `RUST_ROOF`),
height, shipped product, shipped *membership* (Depot given Coal), label string. A wholesale
row swap fails as `row 6 drifted out of position`, and the "keep them aligned" variant —
swapping the table rows **and** their pins together — fails all six via
`pinned row 6 drifted out of ALL's order`. **The art pin is not circular**, unlike the
hazard in §1: both `BUILDING_ART` and `PINNED_ART` carry an explicit `kind` field asserted
against `BuildingKind::ALL[i]`, so the pin cannot be dragged into alignment with a bad table.
That two-sided `kind` column is the pattern to demand of every future pin.

## 8. The phase 6 utility witnesses are circular against column mutations — and priority
## has no witness at all (PROVEN LIVE, 2026-08-19)

`the_grid_serves_exactly_the_draw_the_power_column_claims` (wires.rs) and its water/heat
siblings read `spec(kind)` for the *expected* side: gate-presence expectation, and the pool
they build is `demand.rate` off the same column. The spawn/attach code reads the same column.
So: give Mine a fabricated `power: Some(…)` row → `Powered` attaches, the witness expects it
and serves it, green. Take `heat: Some(Consumer)` off Dwelling → no `Heated`, the witness
expects none, green — while every home freezes. **Membership and rate mutations of the
power/water/heat columns are catchable only by a historical diff.** (Rates get partial
secondary coverage from the Liebig/starved-grid tests that mix real constants; membership
gets none.)

**Priority is worse: zero witnesses, and the gap shipped a real regression.** Phase 6 flipped
Dwelling's power priority Housing → Industry and 153 stayed green. The one contention test,
`starved_grid_serves_homes_before_factories`, passes by arithmetic accident: 10 MW pool,
3×4 MW factories + 2×1 MW homes — with everyone in one class the third factory doesn't fit
and the leftover 2 MW lights both homes anyway, so both asserted counts coincide. **A
contention test whose starved consumer's demand exceeds the leftover pool is the only shape
that pins a priority.** E.g. 2 factories + 3 homes on 9 MW: Housing-first = 3 homes + 1
factory; flat = 2 factories + 1 home.

## Mutation testing works well here and is cheap

`cargo test --lib` runs in ~0.46s. Back up all touched files (`cp` + `md5sum`), plant one
error per field group, run, confirm each names the right kind, restore, re-verify md5 **and**
`git diff --numstat` against the pre-review counts.

**The oracle technique is the strongest instrument found so far.** To prove a deleted match
was faithfully replaced, temporarily add a test that reconstructs the deleted match verbatim
from `git show HEAD:path` and compares it against the new implementation across all kinds ×
all resources. Then corrupt one row to prove the oracle itself is not vacuous. This settled
the `default_policies` fold at Phase 2 in two minutes and caught what 138 tests could not.

## 6. `art.rs`'s `PINNED_ART` is genuinely non-vacuous — and I got the proof for free

At Phase 5 another reviewer was mutating `src/game/art.rs` concurrently while I read it. I ran
the scoped suite against *their* live mutations instead of writing my own, and watched three
pins fail by name:

```
roof_column_holds_its_pinned_surfaces   Mine roof       left: (RustedSteel,0.75,0.0) right: (…,0.3)
shipped_column_holds_its_pinned_products Depot shipped   left: Coal  right: Goods
wall_column_holds_its_pinned_surfaces   Warehouse wall  left: (WornEarth,0.88,0.0)  right: (…,0.9)
```

Two properties worth remembering. `PINNED_ART` spells roofs out as `(Role, f32, f32)` literals
rather than referencing `RUST_ROOF`/`CIVIC_ROOF`/`TARRED_ROOF`, so it catches a change to the
*shared* roof constant, not just to a row — that is why the Mine-roof mutation was caught.
And `pinned(i, kind)` asserts `PINNED_ART[i].0 == BuildingKind::ALL[i]` against a hard-coded
row order, which makes it a real pin on `ALL`'s order, not a self-comparison.

**Technique worth reusing: if another reviewer is mutating the file you are reviewing, run the
scoped suite while their mutation is live.** It is free non-vacuity evidence and needs no write
from you. Verify the file's md5 back to pristine afterwards.

## 7. The toolbar's order witness is a three-link chain, not a direct pin

`the_build_flyout_offers_the_whole_catalogue_and_nothing_else` compares `CATEGORIES`' build
entries against `BUILDING_ART` — but `BUILD_TOOLS` is *generated from* `BUILDING_ART` in a
`const` block, so on its own it is `table == f(table)` and pins neither order nor captions.
What actually pins the on-screen order is the chain: that test (flyout == table order) +
`every_row_sits_at_its_own_kind_s_position` (table order == `ALL`, and `art(kind).kind == kind`,
which is what guards the `kind as usize` indexing) + `PINNED_ART`'s hard-coded row order and
label column. The chain closes. Do not let a later phase break a link thinking the wiring test
covers it.

**One live blind spot in that chain:** `listed_build_tools()` deliberately flat-maps across
*all* categories, so moving `&BUILD_TOOLS` from the `"BUILD [3]"` category onto, say,
`"NETWORKS [4]"` leaves every test green while the BUILD flyout renders empty. Nothing asserts
which `Category` owns the build entries.

**And `switch_tool` itself is untested.** The Digit-3 *cycle function* is now covered
(`the_digit_3_cycle_walks_the_build_flyout_in_order`, including wrap and start-from-Inspect),
but no test presses a key or instantiates `switch_tool`, so the Digit-3 *binding* — and the
Escape/Digit1..7 chain around it — remains witnessed only by playing the game.

## 6. §2's gap is CLOSED as of Phase 3 — but a new tautology shipped with the fix

The band column has a witness again: `PINNED_COLUMNS` grew a fifth element and
`default_policies_column_holds_its_pinned_bands` compares it per resource, so `Option`
comparison makes **membership** checkable (the Depot case). Verified closed at Phase 3.

New, milder instance of the same family, in `save.rs`'s otherwise-excellent pin test:

```rust
assert_eq!(kind_from_u8(BuildingKind::COUNT as u8), None);
assert_eq!(kind_from_u8(u8::MAX), None);
```

`ALL: [BuildingKind; COUNT]` means `COUNT == ALL.len()` *by the type*, so `ALL.get(COUNT)` is
`None` unconditionally. **These two lines cannot fail while the implementation is `.get()`.**
They are not worthless — they fail loudly if someone "simplifies" it to `ALL[v as usize]`
(demonstrated: `index out of bounds: the len is 13 but the index is 13`). But they pin a
*shape*, not a value, and the test's own doc comment claims its integers are transcribed
literals. Rule of thumb reinforced: **an assertion whose expected value is derived from the
same constant as the actual value is a tautology, even when the constant is only a length.**

## 7. The 13-arm `parts()` match is a load-bearing guard that looks like art code

`src/game/buildings.rs:213` is the last compiler-exhaustive `match` over `BuildingKind` in
the tree (every other one has a wildcard). It is what makes "add a 14th kind" a compile
error. Nothing names it as a guard, and it sits in the presentation track that is actively
being turned into tables. When a phase proposes tabling `parts()`, that phase must add an
explicit exhaustiveness guard over `ALL` — otherwise a new variant becomes a runtime panic in
`kind_to_u8`'s `.unwrap()` and an out-of-bounds in `spec()`. See [[save-wire-format-baseline]].

## 6. A suffix-anchored `..` in a tuple pattern makes a pin silently assert the wrong column

Phase 3's own report names this hazard — widening `PINNED_COLUMNS` from 4 to 5 elements
silently re-bound `workers` from a vacancy count to a band slice, caught only because `u32`
and `&[(ResourceKind,f32,f32)]` are not comparable — and then says "the three pinned tests
now use explicit positional patterns; keep them that way".

**That claim is false, and the test the phase *added* is the one carrying the hazard.**
As shipped (`src/sim/catalogue.rs`):

- `:445` `let (pinned_kind, footprint, ..)` — prefix-anchored, safe under widening
- `:457` `let (pinned_kind, _, capacity, ..)` — prefix-anchored, safe
- `:469` `let (pinned_kind, _, _, workers, _)` — explicit, the only one actually fixed
- `:498` `let (pinned_kind, .., bands)` — **suffix-anchored: `bands` binds to the LAST
  element, whatever that becomes**

Reproduction that made this concrete (all reverted): add a 6th, band-shaped column to
`PinnedRow`, move every row's real bands into it and leave slot 5 as `&[]`, then make the
one mechanical edit the compiler demands (`(pinned_kind, _, _, workers, _)` ->
`(…, _, _)`). Result: `cargo test --lib sim::catalogue` = **10 passed; 0 failed** while
`default_policies_column_holds_its_pinned_bands` was asserting an empty column for all 13
kinds. The `u32`-vs-slice type accident that saved the tree last time does not recur when
the new column is the same type as the old one.

**Detector for future phases:** in any pin test, a `..` before the bound variable is a
latent vacuity; a `..` after it is fine.
