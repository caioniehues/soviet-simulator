---
name: phase-log-data-driven-buildings
description: Per-phase verdicts and findings for the data-driven buildings refactor — what each phase got wrong, so a repeat is visible
metadata:
  type: project
---

# Phase log — data-driven buildings

**Why:** a refactor repeats its own mistakes. The arm dropped in Phase 2 is the arm dropped
in Phase 6. Recording the *shape* of each finding, not a summary of how the phase went.

**How to apply:** before reviewing phase N, read every entry below and check whether the
same shape recurs.

## Phase 1 — the spec shape + the table — **PASS** (2026-08-17)

Scope: new `src/sim/catalogue.rs`, `src/sim/mod.rs` +1 line. Nothing reads the table.

**No regressions.** All 13 rows × 8 fields matched the live source exactly. `ALL`'s order
matched `save.rs`'s discriminants at all 13 positions. Every arm of every collapsed match
mapped to a row.

**Findings were all about test strength:** the recipe test asserted only 5 of 13 kinds and
compared constants against themselves; the power/water/heat tests re-transcribe the match;
the save round-trip hash is blind to a symmetric reorder.

**Report accuracy:** honest and substantively correct; one loose count ("the four warnings"
while running `--tests`, which shows nine).

## Phases 1b + 2 — the recipe contract + the four lookups read the table — **PASS** (2026-08-17)

Scope: 4 files in `src/sim/` — `buildings.rs` (+5/−34), `catalogue.rs` (+208/−56),
`labour.rs` (+5/−18), `storage.rs` (+10/−22). Gates verified by me: 138 green, 9 clippy
(same nine, none in a touched file), fmt clean.

**No behaviour regression.** All 13 rows × 4 columns identical to the deleted matches. None
of the four matches had a wildcard arm — all exhaustive per-kind — so the wildcard-becomes-
default hazard had no surface this phase. Signatures byte-identical; no production call site
moved. `Without<ConstructionSite>` gap intact in `run_heat_plants` and `solve_water`; both
files untouched. Zero intentional behaviour changes, as contracted.

**F1 (significant) — a column lost its witness with no replacement.** The storage-band column
is now unguarded: four simultaneous corruptions, *including adding a band to Depot which has
never had one*, gave 138 green. See [[vacuous-checks-data-driven-buildings]] §2. Values are
correct today — proven by a reconstructed-match oracle, which I then proved non-vacuous.
The implementer disclosed this but understated it as "the numbers are not guarded"; the real
scope is that band *membership* is unguarded too. The phase's own newly-approved witness rule
("How a column keeps a witness") was applied to three columns of four.

**F2 (minor, second occurrence of a pattern) — counts quoted are pre-change.** "54 references,
none of which changes" was measured before deletion; it is 51 now, because the three
equivalence tests stopped calling the live functions. Substance held — no production call site
moved — and the deltas were exactly self-consistent. **Pattern: this implementer's numbers are
approximate, their reasoning is reliable.** Verify counts, trust arguments.

**F3 (recorded, not actionable) — cross-swap of equal-valued sibling constants stays invisible.**

**What the implementer got right and should keep doing:** proved their own new test by
mutation before trusting it, and quoted the failure text; caught that their own deletion had
turned three tests into tautologies and said so in the report rather than shipping the green;
declared the gap they chose not to close and deferred the decision rather than quietly
skipping it; migrated the five *why* comments from the deleted arms onto the rows that now own
the fact instead of dropping them.

## Phase 3 — the band pin + save discriminant by position — **PASS** (2026-08-18)

Reviewed as the *sim-guard mutation* dimension of a five-reviewer blind gate (I owned
`catalogue.rs` + `save.rs` only; `src/game/art.rs` was another reviewer's, concurrent).

**Zero behaviour change, proved not argued.** The only production edit in either file is
`kind_to_u8`/`kind_from_u8`. Wire format proved identical for **all 256 byte values** by
parsing `git show 7c3b4ce:src/sim/save.rs`'s two matches into dicts and comparing against
`BuildingKind::ALL`'s enumeration — encode map equal, decode map equal, `>= 13 -> None`
both sides. `SAVE_VERSION` still 5. `catalogue.rs`'s production half is **byte-identical**
(`diff` of everything above `#[cfg(test)]` is silent); the whole 163-line diff is inside
`mod tests`. Both production call sites unchanged (`save.rs:407` encode, `:748` decode,
confirmed by LSP `findReferences`: 3 refs on `kind_to_u8`, 5 on `kind_from_u8`, all others
in the new test).

**F1 closed properly.** Six band mutations, each named by the new pin, including the two
membership cases (Depot *gains* a band; Mine *loses* one). The implementer only tested the
added-arm direction; the dropped-arm direction works too.

**All four report claims re-established independently:** `.get()` vs `ALL[…]` panics at
`save.rs:248:10` and *only* `building_discriminants_are_pinned_to_the_wire_format` catches it
(6 other save tests green) — the `None` asserts are load-bearing. That test is also the
**only** assertion in the whole tree against hard-coded building bytes (`grep`: `save.rs:1181`
is the sole `0u8` pin), which is why a fully self-consistent reorder trips exactly one test.

**F-A (minor, and it is a REPEAT of the phase's own lesson) — the new pin test carries the
`..` re-binding hazard the report says it eliminated.** `catalogue.rs:498` is
`let (pinned_kind, .., bands)`, suffix-anchored. Only 1 of the 4 pin tests is actually
explicit. Reproduced a 10-passed/0-failed run in which the band pin asserted an empty
column for all 13 kinds. See [[vacuous-checks-data-driven-buildings]] §6.
*Shape to remember: the implementer correctly diagnosed a hazard, wrote it down, and then
reintroduced it in the same commit.*

**F-B (process) — the pin is now a single point of failure for two columns.** A footprint or
capacity mutation fires exactly one test each. Provenance is intact (I hand-compared all
four columns to `d019ca8`), but "deduplicating" `PINNED_COLUMNS` would kill four guards.

**F-C (process, third occurrence) — the equal-valued-constant cross-swap is still green.**
Swapping `MINE_COAL_RATE` <-> `QUARRY_GRAVEL_RATE` in the two rows: `126 passed; 0 failed`.
Still latent, still becomes loud on any rebalance. R4 will open it.

**Report accuracy:** substantively excellent — the strongest report of the refactor so far,
and its four headline claims all reproduce. Two nits: the "three pinned tests are now
explicit" claim is false (F-A), and the 139/140 reorder demo needed a *third* file
(`buildings.rs`'s enum declaration) that the report's Scope section does not mention. That
was a reverted mutation, not a scope breach — `buildings.rs` is unmodified in the tree.
Within the phase's own two files a reorder trips 2 tests (`ALL` + pin) or 3 (`ALL` +
`BUILDINGS` + pin), never 1, because `every_row_sits_at_its_own_kind_s_position` couples
`ALL` to `kind as usize`.
*Pattern holds from Phase 2: this implementer's numbers are approximate, their reasoning is
reliable.*

### Original watch-list (all six discharged)

1. **`ALL[v as usize]` replacing `kind_from_u8`'s `None` fallback with a panic** on an
   out-of-range byte. Must be `ALL.get(v as usize)`. A malformed save file is the input that
   shows it. The implementer already flagged this themselves in their Phase 2 report.
2. **A discriminant test that asserts `ALL` against its own positions** — circular. It must
   assert against hard-coded integers 0..=12.
3. **The round-trip hash is not evidence here** — encode and decode share a binary, so a
   symmetric reorder passes. See [[vacuous-checks-data-driven-buildings]] §1.
4. The oracle technique applies directly: reconstruct the deleted `kind_to_u8`/`kind_from_u8`
   from `git show HEAD:src/sim/save.rs` and compare across all 13 kinds *and* the out-of-range
   bytes 13..=255.
5. Check `PINNED_COLUMNS` has not been "deduplicated" against the table — it is now the only
   witness for three columns.
6. Whether F1 was closed. If the band column is still unwitnessed after Phase 3, say so again.

Outcome: 1 correct (`.get()` shipped, panic reproduced), 2 correct (pin uses hard-coded
`0u8..=12`, no reference to `ALL`'s positions), 3 confirmed (round-trip hash green under
every reorder variant), 4 done by parser instead of an in-tree oracle — cheaper and leaves
no leftover (grep for `committed_kind`/`temporary_differential` in both files: NONE),
5 `PINNED_COLUMNS` intact and *widened*, 6 closed.

## Phase 4 — recipes replace four hand-written systems (50031a1) — **PASS** (2026-08-19)

Scope: new `src/sim/production.rs` (369 lines), deletions in `buildings.rs`/`heat.rs`,
ordering edits in `wires.rs`/`water.rs`, `flow_output` column added to the catalogue
(`Power(PLANT_OUTPUT_MW)` on PowerPlant, `Heat(HEAT_PLANT_OUTPUT)` on HeatPlant, `None`
elsewhere — all correct vs baseline). Reviewed post-commit; suite 153 green, run by me.

**The one sanctioned change shipped with real behavioural witnesses**:
`a_heat_plant_under_construction_burns_nothing_and_warms_nobody` (heat.rs) and
`a_pump_under_construction_supplies_nothing` (water.rs) — both are genuine before/after
behaviour tests, not pins. `solve_water` gained the supplier-side `Without<ConstructionSite>`
explicitly (suppliers have no recipe, so the structural filter in production.rs can't reach
them — the comment says so, correctly).

**The labour asymmetry watch-item resolved by data, verified**: `labour_factor` returns 1.0
when `workers_needed == 0` (`labour.rs:62-63`) and HeatPlant's workers column is 0, so
running the heat plant through the generic curve is a no-op. Correct today, but note:
**HeatPlant's flat burn is now an emergent property of the workers column** — if R4 ever
gives HeatPlant workers, its output silently starts scaling by staffing, which the old code
never did. Not a defect, but a coupling nobody's report names.

Parity checked by hand across all four deleted systems: partial-take-then-zero-output on
fuel shortage preserved; `run_factories`'s required-`&Powered` skip reproduced as
`Bound::Power` (same observable: no goods); `watered.is_none_or` semantics reproduced;
f<=0 power plant zeroes output without burning, same both sides. The scheduling sandwich
(`produce_flows → solve_power/solve_heat → produce_goods`, `solve_water` before
`produce_goods`) reproduces the old chain order for every pair that interacts.

New surface (not a regression, worth knowing): `Gated` component, attached by observer to
producers; an entity stripped of it is silently inert (tested, deliberate, fail-closed).

## Phase 6 — utility solves read the table (e4c7fba) — **BLOCKED** (2026-08-19)

**F1 (BLOCKER) — Dwelling's power priority flipped Housing → Industry in the catalogue, in
the same commit that deleted the only test pinning it.** `catalogue.rs` Dwelling row,
`power: Some(UtilityDemand { rate: DWELLING_DEMAND_MW, priority: PriorityClass::Industry })`
— pre-refactor `solve_power` said `PriorityClass::Housing` (visible in the phase 6 diff's own
deleted lines, `wires.rs`), and the deleted
`power_demand_matches_solve_power_s_per_kind_match` expected Housing. The dwelling's *water*
row still says Housing — the inconsistency is the tell. Not in any plan, report, or the
implementer's memory notes (which document every other phase 6 decision).

**Why 153 stayed green — an arithmetic accident worth remembering.**
`starved_grid_serves_homes_before_factories` (wires.rs) is the one cross-class contention
test: 10 MW pool, 3×4 MW factories + 2×1 MW homes. With everyone Industry, sort falls to
BuildingId; factories (placed first) take 8, the third factory's 4 doesn't fit, and the
leftover 2 covers both homes — **both assertion counts (2 homes, 2 factories) coincide with
the Housing-first result.** Concrete exposing input: 1 plant, 2 factories placed first,
8 dwellings — old: 8 homes lit + 0 factories... (homes first); new: both factories lit,
6 homes dark. The new witness is blind to priority by construction (single consumer, no
contention).

**Everything else in phase 6 is clean**: `solve_power`/`solve_water`/`attach_watered`/
`attach_heat_components`/`buildings.rs` spawn-attach all map old arms to column reads
faithfully (checked arm by arm; the `FlowOutput::Power(_)` variant-match for `PowerOutput`
attach is correct — presence alone would wrongly give HeatPlant a PowerOutput). No constants
changed across either phase (grepped the combined diff). Scope clean: outside `src/sim/`
only implementer memory + the phase 4 report. Fix: one word, `Industry` → `Housing`, plus a
priority witness (see F2).

**F3 (minor, process) — phase 6 committed unformatted code.** `cargo fmt --check` fails on
four of its own new lines (`buildings.rs:178`, `heat.rs:141`, `heat.rs:361`, `water.rs:370`),
all pure line-width reflows. The fmt-clean gate had held at every prior phase. Clippy still
exactly the baseline 4 warnings.

**Environment note (2026-08-19):** an untracked `tests/scratch_review_probe.rs` (another
reviewer's save/restore probe, not phase scope) sits in the tree — it reds `cargo fmt --check`
and would be swept by `git add -A`. Same shape as the Phase 3+5 transcript residue.

**F2 (major, the predicted circularity, now proven live)** — the three replacement witnesses
derive their *expected* membership and rates from `spec(kind)` itself, and the spawn/attach
code reads the same column, so a column mutation is self-consistent end to end. Priority has
**no witness at all** — F1 is the live proof. Details in
[[vacuous-checks-data-driven-buildings]] §8.

## Phase 5 — the art catalogue (`src/game/art.rs`) — mutation dimension, **PASS with one major** (2026-08-18)

Scope reviewed: the six pins in `src/game/art.rs` only (other reviewers held the rest).
Shipped with **no self-report at all**, unlike every earlier phase.

**No regression in the data.** All 13 rows × 5 columns match 7c3b4ce in both the table and the
pin, parsed independently — see [[baseline-building-numbers]] "The art columns".

**The pins are strong, and now proven so.** Eight distinct table mutations each failed the
right test naming the right kind; both reorder variants failed, including the "swap the rows
and their pins together" edit that would have exposed a circular pin. Details in
[[vacuous-checks-data-driven-buildings]] §7. **This is the first phase in the refactor whose
new guards were non-vacuous on first inspection** — a change from Phase 1/1b/2, where every
column test needed repair.

**F4 (major) — the accessor between the pin and the game is unwitnessed.** The pins read
`Surface`'s private fields; `Surface::mat()` is what the game actually calls, and no test
touches it. Deleting `.shade(self.shade)` from it — discarding every wall and roof shade for
all 13 kinds — gives `148 passed; 0 failed`. See
[[vacuous-checks-data-driven-buildings]] §6.

**The shape to carry forward: F1 and F4 are the same defect.** F1 = the band *column* lost its
literals. F4 = the art column kept its literals but the *reader* of them has none. Both are
"the guard stops one hop short of the thing that runs". At every remaining phase, ask not only
*is this value pinned* but *is the function that consumes it pinned* — for Phase 6 that means
`solve_power`/`solve_water`/`attach_heat_components` must be exercised, not just their table
rows compared.

**Type-level guards in force here, worth reusing:** `[BuildingArt; BuildingKind::COUNT]` and
`[PinnedRow; BuildingKind::COUNT]` with `COUNT` a hard-coded `13` (`sim/catalogue.rs:23`) —
dropping a row from *either* array is `E0308: expected an array with a size of 13, found one
with a size of 12`, so length and completeness need no test. `shipped: ResourceKind` is
non-`Option`, matching 7c3b4ce's non-`Option` return, so "a kind that ships nothing" is
unrepresentable rather than untested.

## Phase 5 — the art catalogue (`src/game/art.rs`) — **PASS on my dimension** (2026-08-18)

Reviewed blind, as one of five reviewers, on the uncommitted tree. My dimension: does the art
table say what the deleted code said? Scope: new `src/game/art.rs` (409 lines) +
`buildings.rs` −48/+4, `vehicles.rs` −16/+2, `toolbar.rs` +101/−31, `tools.rs` +18/−17,
`mod.rs` +1. All inside `src/game/`. No `src/bin/`, no ADR.

**No regression found. All 13 rows × 5 columns identical to 7c3b4ce**, plus the Digit-3 cycle
and the BUILD flyout order/labels. Numbers recorded in
[[baseline-building-numbers]] § "The art columns".

**The one thing that could have been a blocker and wasn't.** The Phase 3 report recorded a
red test in flight: `shipped_column_holds_its_pinned_products` failing
`HeatPlant shipped product: left: Coal, right: Goods`. `left` is the table, `right` is the
pin — so the **pin** was the wrong side, and 7c3b4ce says `Mine | PowerPlant | HeatPlant =>
Coal`. The fix corrected the pin to the correct value. Had it "fixed" the table to Goods
instead, every test would still have been green and heat plants would have shipped the wrong
product. **Lesson to reuse: when a pinned test goes red mid-phase, always resolve which side
was wrong against `git show <baseline>:`, never against whichever side is easier to edit.**

**Recurrence of a Phase-3 hazard the Phase-3 report itself warned about.** `art.rs`'s pinned
tests destructure a 6-tuple with `let (.., height, _)` and `let (.., shipped)`. Phase 3's F1
was exactly this `..` pattern silently re-binding when the tuple grew, and its own report
closed with "the three pinned tests now use explicit positional patterns; keep them that way".
Two of `art.rs`'s five did not. Minor today (a 7th column of a different type would fail to
compile), latent if a column of the same type is appended.

**Process:** Phase 5 shipped with **no self-report** while Phase 3, half the size, has a
detailed one. And a stray `2026-08-17-215718-local-command-caveat….txt` sits untracked at the
repo root.

## Phase 3 — save discriminant by position — **PASS (wire-format dimension)** (2026-08-18)

Reviewed read-only, as one of five blind reviewers on a combined phase 3 (sim) + phase 5
(presentation) tree. Scope of this entry: `src/sim/save.rs` only.

**No regression. The wire format is provably identical.** Proof chain, all four links closed:

1. The whole **production half** of `save.rs` (everything before `mod tests`) is *textually
   identical* to `7c3b4ce` apart from the two function bodies and one added doc comment —
   `diff` of the two halves, no filters. Nothing in snapshot/restore moved. Report claim
   confirmed exactly.
2. Every `#[derive(Serialize` struct definition in the file is textually identical, so the
   postcard encoding is a pure function of unchanged types.
3. `catalogue.rs`'s production half is *also* textually identical (all 5 diff hunks are at
   line ≥332; `#[cfg(test)]` starts at 314), so `BuildingKind::ALL` and `BUILDINGS` did not
   move either.
4. The two changed mappings agree with `7c3b4ce`'s matches on all 13 encodes and **all 256
   decodes** — out-of-tree oracle, see [[save-wire-format-baseline]].

`SAVE_VERSION` 5 unchanged is correct and argued, not assumed. Oracle scaffolding
(`committed_kind_to_u8` etc.) genuinely gone; no `#[allow]` anywhere in `save.rs`. Production
call sites: LSP says 1 encoder (`:407`) and 1 decoder (`:748`), same as `7c3b4ce`; the
reference delta is +1/+3 and decomposes exactly into the new pin test's uses.

**The pin test closes the Phase 1/2 F3 gap.** `building_discriminants_are_pinned_to_the_wire_
format` asserts hard-coded `0u8..=12` in both directions and touches `ALL`'s positions
nowhere. Its `len() == COUNT` assertion is genuinely non-vacuous for an *appended* or
*inserted* kind. This is the guard the phase-log demanded and it arrived in the right shape.

**Findings (none blocking):**

- **W1 (minor) — new cross-track coupling nobody's report names.** `BuildingKind::ALL` went
  from 9 references in 1 file to 21 across 4: its order is now the save discriminant *and*
  the toolbar/tool-cycle order. `game/tools.rs:47-51`'s doc comment invites reordering `ALL`
  to reorder the UI; `save.rs:239-242`'s says reordering it rewrites every save. Neither
  cross-references the other. Details in [[save-wire-format-baseline]].
- **W2 (minor) — the pin's two out-of-range assertions are tautologies.**
  `kind_from_u8(BuildingKind::COUNT as u8) == None` cannot fail while the impl is `.get()`,
  because `COUNT` *is* `ALL.len()` by the array type. They earn their place as a regression
  guard against a future `ALL[v]` impl (which the implementer demonstrated panics), but they
  pin no value. A literal `13` would be equally strong and consistent with the test's own
  stated doctrine.
- **W3 (minor) — `.unwrap()` is not type-safe**, and what holds it up is one unrelated
  exhaustive match in the *other* track's file. See [[save-wire-format-baseline]].
- **W4 (process) — the report's decoder table is wrong.** It says `class_from_u8` returns a
  default variant; it returns `None`. It missed `location_from_u8` and `phase_from_u8`, which
  do default. Divergence is unchanged from `7c3b4ce` either way, so no defect.
- **F2 pattern, THIRD occurrence — cited line numbers are pre-change.** Report cites
  `save.rs:769` for the `let Some(kind) … else` consumer (actually `:748`) and `:303-309` for
  `resource_to_u8` (actually `:282-288`). Both are exactly the pre-deletion numbers (769−21).
  **Settled pattern: this implementer's line numbers and counts drift; their reasoning and
  their mutation proofs are reliable.** Verify citations, trust arguments.

**F1 is closed** and closed well: `PINNED_COLUMNS` grew the band column, all 13 rows kept,
and the implementer re-ran my own four-way corruption including the added-Depot-band case.
`PINNED_COLUMNS` was *not* deduplicated against the table.

**What the implementer got right and should keep doing:** built the differential oracle
against `git show 7c3b4ce:` unprompted and then deleted it; proved `.get()` vs `[]` was
load-bearing by substituting the indexing form and quoting the panic; ran the *fully
self-consistent* four-file reorder — the hard case, not the easy one — and reported that
139 of 140 tests passed through it.

### Phase 3+5 — scope / collateral / commit-readiness dimension — **BLOCKED** (2026-08-18)

Separate reviewer, same tree, different dimension from the art-values pass above.

**The blocker is not in the code, it is in git.** `src/game/art.rs` (409 lines, 6 tests) is
**untracked** while `src/game/mod.rs`'s `pub mod art;` is a *tracked* modification.
`git commit -am` stages tracked modifications only, so the commit declares a module whose
file is absent (rustc E0583) and three tracked files that `use super::art::…`
(`buildings.rs:9`, `vehicles.rs:7`, `toolbar.rs:10`) stop compiling. Result: lib + all 19
`src/bin/` binaries + the game fail to build for anyone who clones. Proof that needs no
build: `git cat-file -e :src/game/art.rs` → *"exists on disk, but not in the index"*.
**Reusable rule: whenever a phase adds a NEW file, `git cat-file -e :<path>` it. A green
suite in a dirty tree says nothing about whether the file is in the index.**

**Formatter collateral: none, and here is the discriminator that proved it.**
`git diff 7c3b4ce --numstat` was byte-identical to `git diff 7c3b4ce -w --numstat` on all 7
rows → zero whitespace-only changed lines. Second, independent check: every pre-change file
was *already* rustfmt-canonical (`git show 7c3b4ce:f > /tmp/f; rustfmt --check /tmp/f`), so
the bare `cargo fmt` the Phase 3 report admits running was a no-op. Caveat that cannot be
closed: art.rs was untracked at the time, so if `cargo fmt` reflowed it there is no git copy
to compare against. **Artefact to expect: `rustfmt --check` on a copied-out `mod.rs` always
fails with "failed to resolve mod X" — that is the copy, not the file.**

**Test arithmetic closed exactly.** 138 → 148. `#[test]` count at 7c3b4ce over `src/` minus
`src/bin/` was 138, i.e. equal to the lib test count, so the attribute count is a valid
proxy. +6 art.rs, +2 toolbar.rs (presentation), catalogue.rs 9→10, save.rs 6→7 (sim). Zero
deletions (`git diff 7c3b4ce -- src/ | grep '^-.*#\[test\]'` empty).

**Two near-misses that turned out clean and are worth not re-litigating.**
1. `Surface::mat()` calls `.metallic(self.metallic)` where the old wall and civic-roof code
   called no `.metallic()` at all. Not a change: `Mat::new` (`palette.rs:170`) initialises
   `metallic: 0.0`.
2. `next_building_kind`'s `_ => BuildingKind::Mine` wildcard covered *two* things — every
   non-Building tool mode **and** CustomsOffice (which had no arm of its own). The new
   `ALL[(kind as usize + 1) % COUNT]` reproduces both: (12+1)%13 = 0 = Mine.

**catalogue.rs's whole diff is inside `#[cfg(test)] mod tests`** (first hunk `@@ -332`). No
production line moved there this phase — worth knowing before re-deriving it.

**Process residue at the repo root:** a 185 KB Claude Code transcript
`2026-08-17-215718-local-command-caveat….txt`, untracked and NOT gitignored, so `git add -A`
sweeps it into the commit. Also `.planning/…/reports/phase3-catalogue.md` is untracked while
`.planning/` is tracked, and `.claude/agent-memory/` is untracked while `.gitignore` explicitly
un-ignores it (`!.claude/agent-memory/`) — both need an explicit `git add` or they are lost.
