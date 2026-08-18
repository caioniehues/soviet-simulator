# Phase 3 + Phase 5 review — the wire format from `ALL`, and the art table

**Verdict: PASS WITH FINDINGS.**

Both tracks are behaviour-preserving and I did not take that on trust. All 13 building
discriminants are byte-identical to `7c3b4ce` for every one of the 256 possible bytes, proved
by an out-of-tree differential oracle rather than by reading the diff; all 78 art values
(13 kinds × 2 surfaces × 3 numbers) plus 13 heights, 13 shipped products and 13 toolbar
captions were compared against `7c3b4ce` twice, once by hand and once by a parser that read
the two new tables as separate datasets. Twelve mutations against the sim guards and thirteen
against the art guards were watched fail, each naming the right kind and the right column.
No observable behaviour changed anywhere in the diff, which is the contract: the one sanctioned
change belongs to Phase 4 and is absent here.

It is not a clean PASS for two reasons. First, the presentation track cannot be committed with
`git commit -am` — `src/game/art.rs` is untracked while its `pub mod art;` declaration is a
tracked modification, and a commit that takes one without the other does not compile at all.
That is a packaging defect, not a code defect, and it is a one-command fix, but it is the last
thing between this tree and a broken `main`. Second, Phase 5 shipped 409 new lines, six new
guards and two 13-row tables with no self-report and no prior review, so this gate had to
derive from scratch what a report would have stated. Nothing in it was wrong. That was
unestablished until now, and on the evidence of every earlier phase in this refactor — each of
which had at least one column test turn out vacuous on inspection — the assumption was not
free.

---

## Can these two tracks be committed, and as what?

Yes, as two commits, and the file sets are disjoint so they can land in either order.
`git diff --name-only 7c3b4ce | grep -v '^src/'` returns nothing; no file is touched by both
tracks.

**Sim track — commit as-is.** `src/sim/catalogue.rs`, `src/sim/save.rs`. Both are tracked and
modified, so nothing special is required. The entire `catalogue.rs` diff lives inside
`#[cfg(test)] mod tests` (`diff` of everything above `#[cfg(test)]` is silent), and the entire
production half of `save.rs` differs from `7c3b4ce` only in the two function bodies plus one
added doc comment. Add `.planning/…/reports/phase3-catalogue.md` to this commit — the five
earlier phase reports are all committed and this one is not.

**Presentation track — commit only after one explicit `git add`.** `src/game/art.rs`,
`buildings.rs`, `mod.rs`, `toolbar.rs`, `tools.rs`, `vehicles.rs`. The exact edit required
before committing:

```
git add src/game/art.rs .planning/2026-08-17-data-driven-buildings/reports/phase3-catalogue.md
```

Name the two paths. **Do not use `git add -A` or `git add .`**: an untracked 184,748-byte
Claude Code session transcript sits at the repo root
(`2026-08-17-215718-local-command-caveatcaveat-the-messages-below.txt`) and matches no ignore
rule anywhere, so a blanket add sweeps it into the commit.

No further fix is required of either track. Everything else below is recorded, not blocking.

---

## Findings, most severe first

`CONFIRMED` means both adversarial skeptics held the finding. `PLAUSIBLE` means one refuted it,
and the refutation is summarised with the finding — in every case here the refutation attacked
the *consequence*, never the facts, and in every case the facts survived.

### F1 — `src/game/art.rs` is untracked while `pub mod art;` is a tracked modification `CONFIRMED` · commit gate

**Where:** `src/game/mod.rs:3` against an untracked `src/game/art.rs`.

`git commit -am` stages tracked modifications only and never adds untracked files.
`src/game/mod.rs:3` (`+pub mod art;`) *is* a tracked modification and would be committed;
`src/game/art.rs` — 409 lines, 6 tests — would not. Three further tracked-and-modified files
import from the missing module: `buildings.rs:9` and `vehicles.rs:7` (`use super::art::art;`)
and `toolbar.rs:10` (`use super::art::BUILDING_ART;`).

```
$ git ls-files --error-unmatch src/game/art.rs
error: pathspec 'src/game/art.rs' did not match any file(s) known to git
$ git cat-file -e :src/game/art.rs
fatal: path 'src/game/art.rs' exists on disk, but not in the index
$ git check-ignore -v src/game/art.rs
(no output, exit 1)
```

The first skeptic did not stop at that. They copied `.git/index` to a scratchpad, ran
`GIT_INDEX_FILE=<scratch> git add -u` and `git write-tree` — touching nothing in the repo — and
inspected the resulting tree `4015064826908203f30edfb6aea82dc211cfb67f`:
`git ls-tree <tree> src/game/art.rs` is **empty**, while blob `6710376d` for `mod.rs` has
`pub mod art;` at line 3. They then reproduced the consequence in an isolated minimal crate:

```
error[E0583]: file not found for module `art`
 --> game/mod.rs:1:1
```

`src/lib.rs:1` is `pub mod game;`, so the real crate reaches it the same way, and every one of
the 19 binary targets (18 under `src/bin/`, plus `src/main.rs`) fails with the lib.

**Both skeptics held the facts and both argued the severity down.** Their case is worth
recording, because it is right: the gate's rubric defines `blocker` as "observable behaviour
changed, or a save/wire format broke", and this is neither — the working tree is sound and
self-consistent, and it is the *packaging* that is incomplete. The failure is maximally loud
(`cargo check` on the committer's own machine, before any push), fixed by `git add` plus
`--amend`, and cannot let a regression through unnoticed. The second skeptic also showed the
premised workflow is contradicted by this refactor's own history one commit back:

```
$ git show --stat --format="%h %s" d019ca8 -- src/sim/mod.rs src/sim/catalogue.rs
 src/sim/catalogue.rs | 450 +++++...
 src/sim/mod.rs       |   1 +
```

That is the identical shape — a new untracked module file plus its one-line `pub mod`
declaration in a tracked `mod.rs`, landed together — and `git commit -am` is incapable of
producing it. So the hazard is live on a plausible commit path, not a certainty.

Two corrections to the original write-up, neither weakening it: `src/bin/` holds 18 files, not
19 (`src/main.rs` is the 19th target); and the trigger is not `-am` specifically but any path
that stages tracked modifications only (`git add -u`, an IDE "stage all modified"). The
"148 → 142 tests" consequence in the original is incoherent — a crate that does not compile has
no test count.

**Second limb, same fix.** `.planning/…/reports/phase3-catalogue.md` is also untracked and also
unignored, while `git ls-files .planning/` shows five committed phase reports. The convention
is broken by the same omission. Both paths go in the one explicit `git add` above.

### F2 — `Surface::mat()` is the only bridge from the pinned art data to the rendered material, and no test invokes it `PLAUSIBLE` · minor

**Where:** `src/game/art.rs:42-46`.

The six art pins compare `(surface.role, surface.shade, surface.metallic)` read off `Surface`'s
private fields via `surface_of` (`art.rs:366-368`) and never build a `Mat`. `grep -rn '\.mat()'`
returns exactly two call sites, `buildings.rs:15` and `:19`, and the fields are private, so
nothing else can read them. `buildings.rs` has no test module; palette's six tests construct
`Mat` directly; toolbar's two read `.label`/`.kind` only. Consequently no test result can depend
on `mat()`'s body:

```
  pub fn mat(&self) -> Mat {
      Mat::new(self.role)
-         .shade(self.shade)
          .metallic(self.metallic)
  }

test result: ok. 148 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.59s
```

The stronger mutation (`Mat::new(self.role)` alone) is also 148 green. The defect that hides is
real — `Mat::new` defaults shade to 1.0 (`palette.rs:165`), applied as a linear multiplier at
`palette.rs:238-241`, and every value in the table sits inside the `(0.05, 0.85)` albedo clamp,
so dropping the shade collapses 13 distinct wall surfaces onto 5 role colours and 3 roof
surfaces onto 3 roles. Most visibly a Concrete wall and a `CIVIC_ROOF` become identical and
Quarry, Depot and ConstructionOffice render as one flat colour.

**One skeptic refuted the consequence, correctly.** The gap is pre-existing and this refactor
strictly shrank it. `git show 7c3b4ce:src/game/buildings.rs | grep -c '#\[test\]'` returns 0 —
`buildings.rs` had zero tests at baseline, and the conversion lived inline in four expressions
(`Mat::new(role).shade(shade)` at `7c3b4ce:buildings.rs:34`, the rust-roof chain at `:48`).
The same mutation was equally green then, and at baseline the *data* was unwitnessed too:
mutating `BuildingKind::Mine => (Role::SootBrick, 1.0)` was green. This diff took the art
column from 0 of 78 values guarded to 78 of 78, and collapsed the untested conversion surface
from four expressions to one three-line forwarder. Filing it against the change that improved
it inverts the direction of the delta.

The F1-of-Phase-2 analogy in the original is also wrong and was retracted by both skeptics:
F1 was a self-comparing assertion that proved nothing, whereas these pins fail loudly on any
mistyped row. The residual gap is a missing witness on a downstream accessor, not a vacuous
guard. **Recorded at `minor`, no action required.** The residual risk worth remembering is the
opposite shape from the one filed: a fourth field added to `Surface` and not forwarded in
`mat()` would be silent, and the suggested built-material assertions would not catch that
either.

### F3 — four of the five compiler-exhaustive matches over `BuildingKind` are gone; one survives `PLAUSIBLE` · minor

**Where:** `src/sim/save.rs:244`, and the sole survivor at `src/game/buildings.rs:213`.

`kind_to_u8` is now `BuildingKind::ALL.iter().position(|&k| k == kind).unwrap() as u8`. That
`.unwrap()` is **not** provable panic-free from the type. `pub const ALL: [BuildingKind; Self::COUNT]`
with `COUNT = 13` a hand-written literal (`catalogue.rs:23`) constrains length only — not
distinctness, not coverage. An `ALL` that listed `Mine` twice and omitted `CustomsOffice` would
compile.

At `7c3b4ce` five `match` sites over `BuildingKind` had no wildcard, so "add a 14th variant" was
a non-exhaustive-patterns compile error:

| site at 7c3b4ce | what it was | deleted by |
|---|---|---|
| `src/sim/save.rs:236` | `kind_to_u8` | Phase 3 |
| `src/game/buildings.rs:19` | `kind_material` | Phase 5 |
| `src/game/buildings.rs:81` | `kind_height` | Phase 5 |
| `src/game/vehicles.rs:56` | `shipped_resource` (13 patterns, 3 arms) | Phase 5 |
| `src/game/buildings.rs:257` | `parts()` | **survives, at `buildings.rs:213`** |

(`roof_material`, also deleted by Phase 5, was never a guard — it ended in `_ =>`.) A skeptic
re-established the survivor independently with a brace-matching scanner over every `.rs` file
under `src/`, and hand-checked the four other wildcard-free hits (`sim/buildings.rs:160`,
`sim/transit.rs:130`, `sim/vehicles.rs:219`, `sim/zoning.rs:23`) — each matches a *different*
enum. There are no `.rs` files outside `src/`.

The pin test does not restore the lost guarantee: `wire_format.len() == BuildingKind::COUNT`
(`save.rs:1195-1199`) compares a 13-element array literal's length against the literal 13, so a
variant added to the enum without touching `ALL` or `COUNT` passes `13 == 13` vacuously.

**One skeptic refuted, on grounds that hold.** The finding's own concrete example is caught
loudly: a duplicated `Mine` fails `building_discriminants_are_pinned_to_the_wire_format`
(which hand-names all 13 variants), `sim::catalogue::tests::every_row_sits_at_its_own_kind_s_position`
and the identical test in `art.rs` — three CI failures. And a variant outside `ALL` has no door
into the game at all: the BUILD flyout is `[(&str, ToolAction); COUNT]` built from
`BUILDING_ART`, the number-key cycle is `ALL[(kind as usize + 1) % COUNT]`, and the load path is
`ALL.get(v)`. If one were constructed anyway, `spec()` (`&BUILDINGS[kind as usize]`) panics out
of bounds at placement, long before a save is written. Also fair: `ALL`'s exhaustiveness was
never compiler-enforced at `7c3b4ce` either.

**No defect today — `ALL` is exhaustive, distinct and in enum-declaration order, verified.**
This survives as a standing requirement, carried into §5 below.

### F4 — three pinned tests destructure from the end of the tuple, against the phase's own written rule `PLAUSIBLE` · minor

**Where:** `src/game/art.rs:389`, `src/game/art.rs:397`, `src/sim/catalogue.rs:498`.

```rust
art.rs:389        let (.., height, _) = pinned(i, kind);
art.rs:397        let (.., shipped)   = pinned(i, kind);
catalogue.rs:498  let (pinned_kind, .., bands) = PINNED_COLUMNS[i];
```

against the leading-anchored form used elsewhere: `art.rs:373` `(_, _, wall, ..)`, `:381`
`(_, _, _, roof, ..)`, `:405` `(_, label, ..)`, `catalogue.rs:445` `(pinned_kind, footprint, ..)`,
`:457`, `:469`. The Phase 3 report closes with the instruction these violate:

> The `..` tuple-pattern hazard described under F1 above will bite again if `PINNED_COLUMNS`
> grows a sixth element. The three pinned tests now use explicit positional patterns; keep
> them that way.

**The consequence is much narrower than filed, and one skeptic proved it with rustc.** Only
`shipped` is exposed. Appending any 7th column makes `height` re-bind to the `ResourceKind`
slot and the assertion fails to compile:

```
error[E0308]: mismatched types
 --> t.rs:10:29 | expected `f32`, found `ResourceKind`
```

The `shipped` case compiles only if the appended column is itself a `ResourceKind`, and even
then it fails loudly the moment it is added:

```
pin now compares against: Gravel
thread 'main' panicked at s.rs:10:5: assertion `left == right` failed: shipped pin
  left: Coal  right: Gravel
```

It goes quietly vacuous only if the new column's 13 values coincide with the shipped column's.
Worth noting in both directions: the leading-`_` form is safe against *appending*, not against
*insertion* — inserting a `&'static str` at index 1 would silently re-bind `label` in
`(_, label, ..)`. The rule is directional, not absolute. And the sim track does not meet its own
standard either, which is why `catalogue.rs:498` is folded in here rather than filed separately.

**One fix, three sites**, and it is free: `(_, _, _, _, height, _)`, `(_, _, _, _, _, shipped)`,
`(pinned_kind, _, _, _, bands)`. Do it when someone next touches those files; it is not worth a
round trip now.

### F5 — nothing asserts which toolbar `Category` owns the build tools `PLAUSIBLE` · minor

**Where:** `src/game/toolbar.rs:372`, against `listed_build_tools()` at `:356-366`.

`the_build_flyout_offers_the_whole_catalogue_and_nothing_else` asserts
`listed_build_tools() == BUILDING_ART.map(|row| (row.label, row.kind))`, but the left side is
itself generated from `BUILDING_ART` by the const block at `toolbar.rs:47-58`. As a *value* pin
it is `table == f(table)` — its own docstring concedes this. Its real assertion is membership,
and `listed_build_tools()` flat-maps across all categories and discards category identity. So
setting `"BUILD [3]"`'s `tools` to `&[]` and `"NETWORKS [4]"`'s to `&BUILD_TOOLS` leaves the
filtered sequence byte-identical:

```
$ cargo test --lib -- game::toolbar
test result: ok. 2 passed; 0 failed
```

while `spawn_toolbar` (`toolbar.rs:178`) renders an empty BUILD flyout and
`handle_category_clicks` (`toolbar.rs:231`) makes clicking BUILD fall through to Inspect.
`grep -rn 'CATEGORIES\|BUILD_TOOLS' src/` confirms both are referenced only inside `toolbar.rs`,
so no other test in the crate can catch it.

**One skeptic refuted, and the load-bearing correction is this:** `git show 7c3b4ce:src/game/toolbar.rs | grep -c '#\[test\]'`
returns 0. Baseline `toolbar.rs` had no tests at all, so the same misplacement — cut-and-pasting
the inline 13-entry literal into another category — was equally expressible and equally
undetected. This diff took toolbar coverage from 0 tests to 2; it is a gap in new coverage, not
a regression. The skeptic also verified all 13 label/kind pairs are unchanged from the baseline
inline list, in the same order, so the player sees an identical flyout. And the order guard is
not circular: `the_digit_3_cycle_walks_the_build_flyout_in_order` compares the flyout against
`next_building_kind`, which indexes `BuildingKind::ALL` — an independent source.

Cheap closure for whoever next opens the file: `assert!(!CATEGORIES[2].tools.is_empty())` and
`assert_eq!(CATEGORIES[2].label, "BUILD [3]")`.

### F6 — the two out-of-range assertions pin a shape, not a value `PLAUSIBLE` · minor, orientation note only

**Where:** `src/sim/save.rs:1207-1208`.

```rust
assert_eq!(kind_from_u8(BuildingKind::COUNT as u8), None);
assert_eq!(kind_from_u8(u8::MAX), None);
```

Because `ALL` is declared `[BuildingKind; Self::COUNT]`, its length equals `COUNT` by the type,
so while the body stays `ALL.get(v as usize).copied()` no change to `ALL`'s *contents* — a full
reorder that silently rewrites every save on disk included — can make either line fail. An
out-of-tree oracle with a deliberately swapped table confirms it: both lines report `None` while
the pinned literal catches the swap (`to_u8(L) = 11` against pinned `10`).

**One skeptic refuted, correctly, and their argument is the more useful record.** These lines
are not dead weight and the suggested remedy is affirmatively harmful. They fire loudly if the
body is later "simplified" to `Some(ALL[v as usize])` — the implementer demonstrated
`index out of bounds: the len is 13 but the index is 13` — and they are the *only* guard on that
path anywhere in the tree (`grep -rn 'COUNT as u8\|u8::MAX' src/sim/` returns exactly these two
lines), which protects the drop-the-row degradation at `save.rs:748`. The proposed literal `13`
would be worse, not equal: modelling the append the tree's own doc comment calls safe,

```
tree spelling  kind_from_u8(COUNT=14) = None       -> holds
literal 13     kind_from_u8(13)       = Some(NEW)  -> assert_eq!(.., None) FAILS
```

`COUNT as u8` keeps testing one-past-the-end in perpetuity; a frozen `13` fires spuriously on a
documented-safe operation and then stops testing the boundary at all. The doc-comment
inconsistency alleged in the original does not exist — the "transcribed once" sentence is scoped
to the `wire_format` array, and these two lines carry their own comment at `save.rs:1204-1206`
describing them accurately as a no-panic policy pin.

**No edit.** Recorded so a future reader does not mistake them for boundary-value coverage of the
discriminant table: only the 0..=12 literals defend the wire format.

### F7 — a 185 KB session transcript sits untracked and unignored at the repo root `PLAUSIBLE` · minor

`2026-08-17-215718-local-command-caveatcaveat-the-messages-below.txt`, 184,748 bytes / 3,394
lines, opening with the Claude Code CLI banner and `/clear`, `/model`. `git check-ignore -v`
exits 1 with no output — and that covers `.gitignore` (no `*.txt`, no matching name),
`.git/info/exclude` (no active patterns) and the XDG global ignore (one line,
`**/.claude/settings.local.json`). It is therefore in the set `git add -A` stages.

A skeptic refuted it as a gate finding and the reasoning is sound: it is not compiled (no
`build.rs`, no `include_str!`/`include_bytes!` anywhere in `src/`), predates the baseline commit
by nearly two hours so it is not collateral of this refactor, contains no credentials or PII
(scanned), and the three commits made since it appeared were all hand-curated narrow file sets —
one of them, `1cf5d97`, being itself a `.gitignore`-editing commit that left it alone. The
predicted accident has had three opportunities and has not occurred.

**It survives here only as the caveat on F1's fix**, which is why the two are stated together:
the reflexive way to pick up `art.rs` is `git add -A`, and that is the one command that sweeps
this in. Gitignore it or delete it when convenient.

---

## Process findings

These assert missing coverage or a documentation error, not a defect, and were not put through
adversarial verification.

**P1 — no bench gate ran, no capture ran, and nothing in the tree renders a building.** The plan
names the seven bench gates as part of this refactor's proof contract (`task_plan.md:14-15`) and
`findings.md:118-124` adds "the game launches and reaches a window". Neither ran, by anyone.
`grep -rn 'GamePlugin' src/` returns `src/lib.rs:50`, `src/game/mod.rs:24`/`:26`, a comment, and
three capture binaries — **no test target anywhere adds it**. Phase 5 put an `art()` table read
inside two per-frame `Update` systems (`draw_spans` and `draw_power_lamps`, `wires.rs:39-49`,
calling `kind_height` at `:151` and `:189`) and inside the per-frame placement ghost
(`kind_color` at `buildings.rs:170`), and `kind_height` went from a match over `f32` immediates
to an indexed load into a `[BuildingArt; 13]` whose rows carry a 16-byte `&'static str` and two
12-byte `Surface`s — a much larger stride, unmeasured. The only perf argument on record covers
the sim track only (`phase3-catalogue.md:187-189`, "nothing here is on a hot path"); Phase 5 has
no report and so no argument at all. Precedent that a green suite does not imply a running game
is five commits back: `41635f2`, "fix: the game could not start at all", whose message says
"Nothing caught it because nothing builds the shipped plugin set" — and whose remedy,
`the_shipped_sim_plugin_set_registers_without_duplicates` (`lib.rs:63-68`), calls
`add_sim_plugins` and never `GamePlugin`. It guards the sim half; this diff is entirely in the
unguarded presentation half. Mitigating: this diff adds no plugin, no `Resource` and no system —
the only structural addition is `pub mod art;` — so the launch risk is low, and the plan does
defer visual proof to Phases 4 and 7. That deferral is defensible for aesthetic judgement, since
no art value changed. It is not defensible as existence proof, and the deferral target is two
unstarted phases away while the commit lands now.

**P2 — Phase 5 shipped 409 new lines, six new guards and five rewritten call-site files with no
self-report.** The reports directory holds phase1-research, phase1-catalogue, phase1-review,
phase2-catalogue, phase2-review and phase3-catalogue — nothing for phase 5, while
`task_plan.md:86` marks it complete. There was no declared file list (which is what let F1
survive), no disclosure of the wildcard-arm collapse in `roof_material`, and no mutation evidence
for the five new pins. The gate derived all of it. Write the report before committing, even a
short one.

**P3 — `art.rs`'s module doc cites ADR 0017 for a claim ADR 0017 does not make.** `art.rs:5-6`
says the reference implementation "keeps meshes and materials out of its building files for the
same reason (ADR 0017)", and `task_plan.md:95-96` states it more strongly. But
`grep -in 'mesh\|material\|texture' docs/adr/0017-a-building-is-its-product.md` returns exactly
one line, `:86`, an appendix correction to a *file count* — and never mentions materials. The
primary-source report records only "8 .nmf (mesh)", which for 488 building types argues meshes
are *shared*, not excluded by design. The architectural decision is independently sound on ADR
0003, which the plan cites accurately; fix the citation rather than the decision. Separately,
ADR 0017's own status line still reads "nothing reads them yet", which went stale at `7c3b4ce`.

**P4 — the Phase 3 report's decoder-divergence table is wrong.** Its first claim (that
`net_kind_to_u8`, `class_to_u8` and `transport_class_to_u8` remain hand-paired matches) is true.
Its second is not: `class_from_u8` (`save.rs:258`) returns `Option<RoadClass>` with
`_ => return None` and degrades exactly like `kind_from_u8`. The report also misses the two
decoders that genuinely do default — `location_from_u8` (→ `AtHome`, and lossy even in range:
bytes 1/`ToWork` and 3/`ToHome` both decode to `AtHome`) and `phase_from_u8` (→ `ToPickup`).
The true split is 4 `Option`/`None` (kind, class, resource, transport_class) against 3 defaulting
(net_kind, location, phase). No behavioural consequence: every decoder body is textually
identical to `7c3b4ce`, and the diff arguably *reduces* the stylistic split by converging
`kind_*` onto the positional shape `resource_*` already used. This is also the third phase running
where the implementer's citations drift while their arguments hold — the report cites `save.rs:769`
for a consumer that is now at `:748` (769 minus the 21 lines the phase itself deleted) and
`:303-309` for functions at `:282-288`. Settled pattern: verify their line numbers and counts,
trust their reasoning and mutation proofs.

**P5 — the report's "139 of 140 green" reorder demonstration is not reproducible from the two
files the phase owns.** It requires editing the enum declaration in `src/sim/buildings.rs`, which
the report's own Scope section ("Two files, both `src/sim/`") does not mention. That was a
reverted mutation, not a scope breach — `src/sim/buildings.rs` is unmodified in the tree — but a
reader checking the claim would fail. Within the phase's two files a reorder trips 2 or 3 tests,
never 1, because `every_row_sits_at_its_own_kind_s_position` couples `ALL` to `kind as usize`.
The substance of the claim was verified another way and is correct.

**P6 — `switch_tool` has no test, so the Digit-3 *binding* is unwitnessed even though the cycle
now is.** The refactor extracted `next_building_kind` and gave it a good test — order, start
element and wrap-around all covered — but nothing instantiates `switch_tool` or presses a key, so
nothing catches Digit-3 being rebound, the else-if chain being reordered, or the `Building` arm
being dropped. Pre-existing rather than caused by this diff, but the diff edited that exact arm,
which is the moment the question is cheapest to answer. The missing assertion is a Bevy-app test
that inserts `ButtonInput<KeyCode>` with Digit3 just-pressed, runs `switch_tool`, and asserts
`*ToolMode == ToolMode::Building(BuildingKind::Mine)`.

**P7 — the equal-valued-constant cross-swap is still invisible, third confirmation.** Because
`MINE_COAL_RATE == QUARRY_GRAVEL_RATE == 0.05`, two rows can name each other's constant with the
whole sim suite green: `one_frame_moves_exactly_what_the_recipe_claims` compares observed yard
delta against the recipe's claimed number, and equal numbers are indistinguishable. Phase 3's new
pins do not touch the recipe column, so nothing changed. Not live today — the values are correct —
but the moment R4 rebalances either constant a swapped pair silently produces the wrong rate for
two kinds.

**P8 — two mutation reviewers were live-editing `src/` during this gate window, so any suite
result quoted without an md5 witness in the same shell invocation is not evidence.** `art.rs` is
untracked, so `git status --porcelain` cannot reveal that it has been altered and git holds no
copy to restore from. Its md5 changed at least five times during the window. One reviewer's
first two `cargo test --lib` runs reported `147 passed; 1 failed` on
`default_policies_column_holds_its_pinned_bands` — another reviewer's in-flight mutation of
`catalogue.rs`, correctly diagnosed as such and not a defect. Every result quoted in this report
was taken md5-fenced. Both files are pristine now (see §Tree state).

---

## What is now proved, and what is merely green

This is the section to read before touching any of this code. The distinction is between a guard
someone watched fail, and a test that has only ever passed.

### Proved — a mutation was applied and the guard was watched to fail, naming the right thing

| guard | mutation | observed failure |
|---|---|---|
| `default_policies_column_holds_its_pinned_bands` | Quarry Gravel `(0.0,0.05)`→`(0.0,0.5)` | `Quarry / Gravel  left: Some((0.0, 0.5))  right: Some((0.0, 0.05))` |
| " | Dwelling Goods `(0.5,1.0)`→`(0.15,1.0)` | `Dwelling / Goods  left: Some((0.15, 1.0))  right: Some((0.5, 1.0))` |
| " | HeatPlant Coal `(0.6,1.0)`→`(0.9,1.0)` | `HeatPlant / Coal  left: Some((0.9, 1.0))  right: Some((0.6, 1.0))` |
| " | **Depot gains** a Coal band (added arm) | `Depot / Coal  left: Some((0.3, 0.7))  right: None` |
| " | **Mine loses** its Coal band (dropped arm) | `Mine / Coal  left: None  right: Some((0.0, 0.05))` |
| `building_discriminants_are_pinned_to_the_wire_format` | `ALL` Depot↔BusStop swapped + pins swapped | `Depot encodes to the wrong byte  left: 7  right: 6` |
| " | `.get()` → `Some(ALL[v as usize])` | `panicked at save.rs:248:10: index out of bounds: the len is 13 but the index is 13` |
| `one_frame_moves_exactly_what_the_recipe_claims` | Mine coal rate → `0.07` | fails at `catalogue.rs:640:17`, names kind and resource |
| `footprint_column_holds_its_pinned_values` | Mine 14×14 → 14×15 | `left: Vec2(14.0, 15.0)  right: Vec2(14.0, 14.0)` |
| `inventory_capacity_column…` | Mine 60 → 65 | `left: 65.0  right: 60.0` |
| `workers_needed_column…` | Mine 6 → 7 | `left: 7  right: 6`, **plus** `sim::labour::tests::output_scales_with_the_cs1_staffing_curve` |
| `wall_column_holds_its_pinned_surfaces` | Quarry `Concrete`→`Timber` | `Quarry wall  left: (Timber, 0.8, 0.0)  right: (Concrete, 0.8, 0.0)` |
| " (shade alone) | Warehouse `0.9`→`0.88` | `Warehouse wall  left: (WornEarth, 0.88, 0.0)  right: (WornEarth, 0.9, 0.0)` |
| `roof_column_holds_its_pinned_surfaces` (metallic alone) | `RUST_ROOF` `.metallic(0.3)`→`0.0` | `Mine roof  left: (RustedSteel, 0.75, 0.0)  right: (RustedSteel, 0.75, 0.3)` |
| " (whole surface) | PowerPlant `RUST_ROOF`→`CIVIC_ROOF` | `PowerPlant roof  left: (Concrete, 0.55, 0.0)  right: (RustedSteel, 0.75, 0.3)` |
| `height_column_holds_its_pinned_values` | Factory `9.0`→`9.5` | `Factory attach height  left: 9.5  right: 9.0` |
| `shipped_column_holds_its_pinned_products` | HeatPlant `Coal`→`Gravel` | `HeatPlant shipped product  left: Gravel  right: Coal` |
| " (membership) | Depot gains `Coal`, which it has never shipped | `Depot shipped product  left: Coal  right: Goods` |
| `label_column_holds_its_pinned_captions` | Depot `"DEPOT"`→`"GARAGE"` | `Depot toolbar label  left: "GARAGE"  right: "DEPOT"` |
| `every_row_sits_at_its_own_kind_s_position` | Depot/BusStop rows swapped, pins untouched | `row 6 drifted out of position  left: BusStop  right: Depot` |

Three results in that table deserve to be called out.

**The art pin is not circular, and this was tested directly.** The careless "keep them aligned"
edit — swapping the two table rows *and* their two pinned rows together, so both arrays stay
internally consistent — fails **all six** art tests. Five fail inside the `pinned()` helper with
`pinned row 6 drifted out of ALL's order / left: BusStop / right: Depot` (`art.rs:362`), the
sixth at `art.rs:216`. The mechanism is that both `BUILDING_ART` and `PINNED_ART` carry an
explicit `kind` field asserted against `BuildingKind::ALL[i]`, so neither array can be dragged
into agreement with the other without disagreeing with the enum. **This is the pattern to demand
of every future pin in this refactor.**

**A reorder between two similar kinds is caught by the positional assertion, not by the column
pins.** In the Depot/BusStop swap, `shipped_column_holds_its_pinned_products` *passed*, because
both ship Goods. `every_row_sits_at_its_own_kind_s_position` is therefore load-bearing and must
not be deleted as a duplicate.

**Row completeness is guarded at the type level, not by an assertion, and that is sound.**
Deleting a row from `BUILDING_ART` gives `error[E0308]: expected an array with a size of 13,
found one with a size of 12` at `art.rs:79:62`; deleting one from `PINNED_ART` gives the same at
`art.rs:251:58`.

### Proved by comparison against `7c3b4ce`, not by a test

A pin transcribed out of the new table proves only that the table has not moved since the pin was
written. These closures are what actually establish fidelity to the deleted code:

- **All 256 decode bytes and all 13 encode values.** A standalone oracle parsed `7c3b4ce`'s two
  13-arm matches and the current `ALL`, then compared exhaustively:
  `encode/roundtrip mismatches over 13 kinds: 0`, `decode mismatches over all 256 bytes: 0`,
  `new_from_u8(13)=None new_from_u8(255)=None old_from_u8(13)=None old_from_u8(255)=None`. Then
  the oracle itself was proved non-vacuous by swapping SewagePlant/HeatPlant in its own copy of
  `ALL`: `DECODE MISMATCH at 10: old=Some(SewagePlant) new=Some(HeatPlant)`, 2 mismatches each way.
- **All 78 art surface numbers, both tables, two-sided.** A python parser extracted `BUILDING_ART`
  and `PINNED_ART` as *separate* datasets and compared each against `7c3b4ce`'s five sources
  (`kind_material`, `roof_material`, `kind_height`, `shipped_resource`, the hand-written BUILD
  list): `MISMATCHES: 0` for the pin, `TABLE-vs-7c3b4ce MISMATCHES: 0` for the table. The
  two-sided parse is the part that matters — it establishes the pin is an independent
  transcription of the old code, which a hand-compare cannot.
- **The wildcard arm was expanded faithfully.** `7c3b4ce`'s `roof_material` had
  `_ => Mat::new(Role::RustedSteel).shade(0.75).metallic(0.3)`. By set membership that arm covered
  exactly Mine, PowerPlant, Factory, Warehouse, WaterPump, SewagePlant, HeatPlant — and
  `RUST_ROOF` sits on exactly those seven rows. The two named arms map to `TARRED_ROOF` (Dwelling,
  BusStop, CustomsOffice) and `CIVIC_ROOF` (Quarry, Depot, ConstructionOffice). 3+3+7 = 13. This
  is the **first wildcard the refactor has collapsed** — the four sim matches at Phase 2 were all
  exhaustive — so it was the one place a kind could silently gain or lose a fact, and it did not.
- **The band pin's literals are not circular.** All 13 pinned band rows were hand-compared against
  `git show d019ca8:src/sim/storage.rs`'s hand-written match, the last commit that still held it —
  13 arms, every arm maps to a row, zero wildcards, no value changed.
- **The Digit-3 cycle.** All 14 transitions reconstructed from `7c3b4ce`'s 13-arm match:
  Mine→Quarry→…→HeatPlant→CustomsOffice, CustomsOffice→Mine (the old wrap was *implicit in the
  wildcard*, and `(12+1)%13 = 0` reproduces it), and every non-Building mode → Mine.
- **The BUILD flyout.** 13 captions, byte-identical strings in identical order to the hand-written
  list at `7c3b4ce`.
- **`.metallic(0.0)` on walls is a provable no-op**, not a default being relied on: `Mat::new`
  initialises `metallic: 0.0` (`palette.rs:170`), `metallic()` is a plain field assignment
  (`:203-206`), `shade` and `metallic` are independent fields consumed only in `build()`, and
  `build()` clamps to `MAX_METALLIC = 0.6` with `RUST_ROOF`'s 0.3 well inside. Builder order is
  irrelevant for the same reason, and matches `7c3b4ce`'s order anyway.
- **`SAVE_VERSION` is still 5, and that is correct — argued, not assumed.** `from_bytes`
  (`save.rs:1046`) hard-rejects any version mismatch, so bumping it against a provably
  bit-identical byte stream would make every existing v5 save unloadable for nothing. The stream
  is bit-identical because every `#[derive(Serialize)]` struct in `save.rs` is textually identical
  to `7c3b4ce` (145-line extract, `diff` silent), so the postcard encoding is a pure function of
  unchanged types.
- **`parts()` is byte-identical**, md5 `be27b59d675d0beb7c98c57ef898d763` on both sides.

### Merely green — passing, and not yet evidence of anything

- **`Surface::mat()`.** Deleting `.shade(self.shade)` leaves 148 green. See F2.
- **The save round-trip hash tests.** Confirmed blind, not assumed: under both reorder variants
  `round_trip_preserves_the_sim_state_hash`, `restore_into_the_same_world_is_identical`,
  `file_round_trip_and_version_gate`, `loaded_world_resumes_production_and_commutes`,
  `mid_trip_save_keeps_the_cargo_and_the_order` and `mid_commute_save_normalizes_travellers_home`
  all **passed**. Their green is not evidence about discriminants and must never be read as such.
  They do exercise the changed path — `full_town` places 5 buildings that flow through
  `kind_to_u8` and `kind_from_u8` — but the other 8 kinds appear in the save test module only
  inside the new pin's `wire_format` array. That array is the sole 13-of-13 byte-level witness in
  the tree (`grep -rn ', 0u8)' src/` → `save.rs:1181` only).
- **`save.rs:1207-1208`.** Cannot fail while the body is `.get()`. See F6.
- **`the_build_flyout_offers_the_whole_catalogue_and_nothing_else` as a value pin.** `table == f(table)`.
  Its captions and order are witnessed, but only through a three-link chain to `PINNED_ART`. See F5.
- **The equal-valued-constant cross-swap.** Green, and will stay green until R4. See P7.
- **The whole rendering path.** No test adds `GamePlugin`; no bench, capture or launch ran. See P1.

### Clean, and worth not re-deriving

Scope is exactly 7 tracked-modified files plus 1 untracked, all under `src/sim/` or `src/game/`.
No `src/bin/` file changed and `cargo build --bins` finished. No ADR, no README, no `Cargo.toml`,
no `Cargo.lock`. Zero formatter collateral, proved two ways: `git diff 7c3b4ce --numstat` is
byte-identical to `git diff 7c3b4ce -w --numstat` on all 7 rows, and every pre-change version was
already rustfmt-canonical, so the bare `cargo fmt` the Phase 3 report admits running was a no-op.
Clippy shows the same nine pre-existing warnings and no tenth — `vehicles.rs:423` looks new but is
`7c3b4ce:vehicles.rs:436` shifted by the file's net −13 lines, 366 lines below the changed hunk.
Test arithmetic closes exactly: 138 + 6 (art) + 2 (toolbar) + 1 (catalogue) + 1 (save) = 148, and
`git diff 7c3b4ce -- src/ | grep '^-.*#\[test\]'` is empty — no test was deleted. Nothing was
added that suppresses: zero `TODO`, `dbg!`, `println!`, `#[allow(`, `#[ignore]` in the diff, and
exactly one `.unwrap()`, discussed at F3. No second per-kind art fact survives anywhere: the HUD
renders kind names via `{:?}`, so `art.rs`'s `label` column has a single owner. And Phase 5
conforms to its plan exactly — all six deliverables at `task_plan.md:105-107` present, nothing
beyond them tabled, `parts()` deliberately left as code as mandated twice.

---

## Carried forward into Phase 4

Phase 4 replaces `extract_resources`, `run_power_plants`, `run_factories` and `run_heat_plants`
with one generic `spec.recipe`-driven pass, lands `Gated { rate, bound_by }`, and makes the
`Without<ConstructionSite>` inertness filter structural. Five of the findings above become live
hazards there. Stated as instructions, in the register of Phase 3's "`ALL.get(v)` not `ALL[v]`" —
which is the instruction that made Phase 3 get it right.

1. **`run_heat_plants` is not symmetric with the other three, and the generic pass must reproduce
   the asymmetry deliberately or change it deliberately.** It burns `HEAT_PLANT_COAL_BURN` flat,
   with **no labour factor**, while the other three scale with staffing. Collapsing four systems
   into one recipe loop will silently apply the labour curve to heat plants unless you decide not
   to. If you decide to, that is a **second** intentional behaviour change and it must be declared
   in the report, not discovered by a gate.

2. **`extract_resources` has a `_ => {}` wildcard arm and Phase 5 proved a wildcard is where a
   kind silently gains or loses a fact.** Before you delete it, enumerate by set membership which
   kinds it currently covers and assert that each of them still moves exactly zero. Phase 2's
   `one_frame_moves_exactly_what_the_recipe_claims` already asserts the eight recipe-less kinds
   move nothing in both directions (a rogue `add` and a rogue `take` were both caught) — keep that
   test and keep it looping all 13 kinds × 3 resources with **no skip, no `continue`, no early
   exit**.

3. **The inertness fix must be witnessed by a test that fails on the old code, not by the absence
   of a failure.** The bug is that a heat plant and a water pump produce at full rate while still a
   `ConstructionSite`. The gate for it is a test that places a building, leaves it under
   construction, ticks a frame and asserts the yard did **not** move — and you must watch it fail
   against `7c3b4ce`'s query before you claim it. `run_heat_plants` (`heat.rs:108-110`) queries
   `(&Building, &mut Inventory, &mut HeatOutput)` with no filter today; `solve_water`
   (`water.rs:63-66`) carries only `Without<Watered>`. Both files are untouched by this diff, so
   the baseline is exactly what Phase 2 recorded.

4. **`Gated { rate, bound_by }` is a balance number in a structural wrapper. Ground `rate` in
   behaviour and pin `bound_by` to a hand-written golden row.** Apply the plan's own test:
   *would a deliberate rebalance have to edit this?* `rate` yes — so it belongs in
   `one_frame_moves_exactly_what_the_recipe_claims`'s observed-delta comparison, not in
   `PINNED_COLUMNS`. `bound_by` no — it is the identity of the binding constraint, a structural
   fact R1's inspect panel will render, so it gets a pinned column with a hand-typed literal per
   kind. Do not pin `rate` and do not behaviour-ground `bound_by`.

5. **If Phase 4 or any later phase tables `parts()`, it must land a replacement exhaustiveness
   guard in the same change.** `parts()` at `src/game/buildings.rs:213` is the **last**
   wildcard-free `match` over `BuildingKind` in the crate — verified by a brace-matching scan of
   every `.rs` file under `src/`, and there are no `.rs` files outside `src/`. It is the only thing
   that still makes "add a 14th variant" a compile error. `COUNT = 13` is a hand-written literal,
   so no `[T; COUNT]` type mismatch fires when the enum grows. Acceptable replacements: a
   `#[deny(non_exhaustive_omitted_patterns)]` match, a derived count (strum's `EnumCount`), or a
   test that walks the enum rather than a transcribed 13-row literal. A fourth pinned table is
   **not** a replacement.

6. **Any new pinned column must anchor its tuple pattern at the front, and must carry an explicit
   `kind` field asserted against `BuildingKind::ALL[i]`.** The front-anchoring is F4. The `kind`
   field is what makes the art pins non-circular under the "keep them aligned" edit, and it is the
   single most valuable structural property either table has. Widening `PINNED_COLUMNS` for
   `Gated` is exactly the operation F4 warns about.

7. **Do not read a green save round-trip as evidence about anything Phase 4 changes**, and do not
   read a green suite as evidence the game runs. P1 stands until someone launches it. The plan
   already schedules the bench gates and `capture_g1` for Phase 4 — run them there, and record the
   numbers, because Phase 5 put table reads inside two per-frame `Update` systems and nobody has
   measured the stride change.

---

## Tree state

Verified, not assumed. Both mutation reviewers restored what they touched, by `cp` from a
scratchpad copy, never by git. `src/game/art.rs` is untracked, so `git status --porcelain` cannot
reveal a mutation of it — the md5 is the only proof, and it matches the value captured before any
reviewer touched anything.

```
$ git status --short
 M src/game/buildings.rs
 M src/game/mod.rs
 M src/game/toolbar.rs
 M src/game/tools.rs
 M src/game/vehicles.rs
 M src/sim/catalogue.rs
 M src/sim/save.rs
?? .claude/agent-memory/
?? .planning/2026-08-17-data-driven-buildings/reports/phase3-catalogue.md
?? 2026-08-17-215718-local-command-caveatcaveat-the-messages-below.txt
?? src/game/art.rs

$ git diff --stat 7c3b4ce
 src/game/buildings.rs |  52 ++--------------
 src/game/mod.rs       |   1 +
 src/game/toolbar.rs   | 101 +++++++++++++++++++++++--------
 src/game/tools.rs     |  29 ++++-----
 src/game/vehicles.rs  |  17 +-----
 src/sim/catalogue.rs  | 163 ++++++++++++++++++++++++++++++++++++++++----------
 src/sim/save.rs       |  83 +++++++++++++++----------
 7 files changed, 279 insertions(+), 167 deletions(-)
```

Seven modified files, +279/−167, `src/game/art.rs` untracked — exactly as expected.

Final suite, md5-fenced on the same command line so the result is about the pristine tree and
nothing else:

```
$ md5sum src/game/art.rs src/sim/catalogue.rs src/sim/save.rs && cargo test --lib && md5sum ...
05d101aef2f709af6c0a8a593116b258  src/game/art.rs
3063645b8a43e8bc9606f25d20e832e1  src/sim/catalogue.rs
70b4b1dd85bb67ea72d240800cf34638  src/sim/save.rs

test result: ok. 148 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.45s

05d101aef2f709af6c0a8a593116b258  src/game/art.rs
3063645b8a43e8bc9606f25d20e832e1  src/sim/catalogue.rs
70b4b1dd85bb67ea72d240800cf34638  src/sim/save.rs
```

Identical either side of the run, and identical to the values every reviewer independently
recorded as pristine. The tree is committable.
