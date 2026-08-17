# Phase 1b + Phase 2 review — the recipe contract, and the four lookups read the table

**Verdict: PASS**, with one significant finding that should be closed before Phase 3 commits.

The refactor is behaviour-preserving and I proved it rather than accepting it. All 13 rows ×
4 columns are identical to what the deleted `match` blocks held; I confirmed the storage-band
column with a reconstructed-oracle test rather than by eye. Every gate was run by me on the
restored tree: **138 passed / 0 failed**, **9 clippy warnings** (the same nine, none in a
touched file), `cargo fmt --check` clean.

The finding is not a regression in behaviour. It is that this phase removed the last
independent witness for the storage-band column, and unlike the other three columns it was
not given a replacement.

---

## Findings, most severe first

### F1 — the storage-band column now has no witness, and even band *membership* is unguarded

**Where:** `src/sim/catalogue.rs:417` (`default_policies_folds_every_band_the_row_lists`),
against the band column at `catalogue.rs:117,132,147,165,187,207-211,284`.

**What it was:** at Phase 1, `default_policies_matches_the_hand_written_lookup` compared the
table's triples against `storage::default_policies()`, which was an independent hand-written
`match`. Two sources, genuine equivalence.

**What it is now:** `storage::default_policies()` *is* the fold of the table's triples
(`storage.rs:78-83`). The test compares the table against a fold of the table. Note the test
body did not change — the diff on it is a rename and a doc comment. It went vacuous because
its oracle was deleted underneath it, without a single line of the test being touched.

**Concrete input that shows the difference.** I applied four band corruptions at once and ran
the full suite:

| row | from | to |
|---|---|---|
| Quarry Gravel | `0.0, 0.05` | `0.0, 0.5` |
| Dwelling Goods | `0.5, 1.0` | `0.15, 1.0` |
| HeatPlant Coal | `0.6, 1.0` | `0.9, 1.0` |
| Depot | `&[]` (no bands) | `&[(Coal, 0.3, 0.7)]` |

```
test result: ok. 138 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

A Quarry that offers gravel only above 50% instead of 5%, a Dwelling whose goods demand drops
from 50% to 15%, and a **Depot that gains a storage band it has never had** — all invisible.
The Depot case is the important one: it is an added arm, the mirror of the dropped arm this
review exists to catch, and nothing in the tree sees it.

**Coverage that does exist**, and it is thinner than it looks — three incidental literals in
`storage.rs`'s own tests: PowerPlant's `min_pct == 0.6` (`storage.rs:188`), Mine's
`max_pct == 0.05` (`storage.rs:195`), and Warehouse's three `(0.2, 0.6)` pairs
(`storage.rs:221`). Everything else is unwitnessed: Quarry, Factory, Dwelling, HeatPlant,
PowerPlant's `max`, Mine's `min`, and the *absence* of bands on all six no-band kinds.

**This is not a live regression.** I verified the values are correct by adding a temporary
test to `storage.rs` that reconstructs the deleted match verbatim from
`git show HEAD:src/sim/storage.rs` and compares it against the new fold across all 13 kinds ×
3 resources. It passed. I then proved the oracle was not itself vacuous by corrupting
Dwelling's band, which it caught (`left: Some((0.5, 1.0)), right: Some((0.15, 1.0))`) — a
corruption the shipped 138-test suite does not. Oracle removed afterwards; tree restored.

So: **the fold is faithful, the numbers survived, and the guard is gone.**

**Why it matters now rather than later.** `task_plan.md`'s "How a column keeps a witness" —
written and approved this phase — says structural constants get pinned and balance numbers get
grounded in behaviour. The band column got neither. It fell through the rule on the same phase
the rule was created. Extending `PINNED_COLUMNS` to a fourth element is the mechanical fix the
implementer already scoped, and it would restore membership checking (the Depot case) as well
as the values.

### F2 — the "54 references, none changed" claim is a pre-change count

**Where:** `reports/phase2-catalogue.md:105-112`.

The report gives 11 / 13 / 9 / 21 = 54 references and says "every one a call site or an
import, none of which changes because no signature changes". Measured with `findReferences`
on the current tree:

| function | report | actual now | delta |
|---|---|---|---|
| `BuildingKind::footprint` | 11 / 7 files | 10 / 6 files | −1 |
| `BuildingKind::inventory_capacity` | 13 / 11 files | 12 / 10 files | −1 |
| `BuildingKind::workers_needed` | 9 / 4 files | 8 / 3 files | −1 |
| `storage::default_policies` | 21 / 10 files | 21 / 10 files | 0 |

Three references did change: the three catalogue equivalence tests stopped calling the live
functions when they were re-pointed at `PINNED_COLUMNS`. `default_policies` is unchanged
because its test still calls the live function. The deltas are exactly self-consistent, and
**no production call site moved** — which is the substance of the claim. But the number quoted
is the pre-deletion one, and the claim "none of which changes" is not literally true.

This is the second phase running where the implementer's *counts* were loose while the
*substance* was sound (Phase 1: "the four warnings" while running `--tests`, which shows nine).
Worth reading their numbers as approximate and their reasoning as reliable.

### F3 — a rate can still name the wrong sibling constant invisibly, if the swap is a *cross*-swap

**Where:** `catalogue.rs:120` and `:135`.

The new behavioural test closes the Phase 1 hole in the direction that matters, but its exact
limit is worth recording. Three variants, measured:

- Mine's row → `QUARRY_GRAVEL_RATE` alone: suite green, **but rustc emits
  `warning: unused import: MINE_COAL_RATE`** — an unclaimed extra guard, because each constant
  is named exactly once in the table.
- Mine ↔ Quarry **cross**-swap (each row names the other's constant): **138 green, no warning
  at all.** Fully invisible.
- Mine's row → `QUARRY_GRAVEL_RATE` *plus* rebalancing `QUARRY_GRAVEL_RATE` to `0.09`: fails
  correctly — `Mine / Coal: one frame moved 0.049999952, the catalogue claims 0.09`.

So the cross-swap is latent, not live: it is behaviourally a no-op today and becomes a loud,
correctly-named failure the moment either constant is rebalanced. That is acceptable, and it is
the property "ground balance numbers in behaviour" is supposed to buy. Recorded so nobody later
mistakes the green for proof that the row names the right constant.

---

## Claims checked, and how

### 1. The new behavioural test is genuinely non-vacuous — **confirmed, by six mutations**

`one_frame_moves_exactly_what_the_recipe_claims` (`catalogue.rs:540`) loops all 13 kinds × 3
resources with no skip, no `continue`, no early exit. Verified by mutation, not by reading:

| # | mutation | result |
|---|---|---|
| A1 | Warehouse gets the Phase 1 fabricated recipe (`in Coal 99 / out Goods 99`) | **FAILS** — `Warehouse / Coal: one frame moved 0, the catalogue claims -99` |
| A2b | Mine's row names `QUARRY_GRAVEL_RATE`, which is rebalanced to `0.09` | **FAILS** — `Mine / Coal: one frame moved 0.049999952, the catalogue claims 0.09` |
| A2d | legitimate rebalance: `QUARRY_GRAVEL_RATE = 0.09`, row unchanged | passes — correctly, no false positive |
| A5 | rogue `inventory.add(Gravel, 0.5)` for Warehouse in `extract_resources` | **FAILS** — `Warehouse / Gravel: one frame moved 0.5, the catalogue claims 0` |
| A6 | rogue `inventory.take(Coal, 0.25)` for **Depot**, a zero-capacity kind | **FAILS** — `Depot / Coal: one frame moved -0.25, the catalogue claims 0` |
| A2a/A2c | sibling swaps without rebalance | see F3 |

A5 and A6 are the ones that answer the lead's question directly: **the eight recipe-less kinds
are asserted to move nothing, not skipped**, and the assertion is live in both directions —
a rogue output *and* a rogue draw are caught. A6 also validates the fixture's yard-widening
justification: the seed genuinely lands on a kind whose real capacity is `0.0`, so
"consumes nothing" is not holding vacuously for want of stock. That was the subtlest vacuity
risk in the fixture and it is closed.

The five producer rates are now behaviourally witnessed as a side effect: the test only passes
because PowerPlant and HeatPlant really burn `0.02`, Factory really makes `0.03`, and Mine and
Quarry really make `0.05`. That is strictly stronger than the Phase 1 test it replaced.

### 2. The four deleted `match` blocks lost nothing — **confirmed, every arm, all 13 kinds**

Enumerated from `git show HEAD:src/sim/{buildings,labour,storage}.rs` and checked against the
table row by row. **None of the four matches had a wildcard arm** — all were exhaustive
per-kind — so the "wildcard silently became a different default" failure mode had no surface
here. (`extract_resources`'s `_ => {}` is untouched.)

| # | Kind | footprint | capacity | workers | bands |
|---|---|---|---|---|---|
| 0 | Mine | 14×14 | 60 | 6 | Coal 0.0–0.05 |
| 1 | Quarry | 16×12 | 60 | 4 | Gravel 0.0–0.05 |
| 2 | PowerPlant | 18×14 | 40 | 8 | Coal 0.6–1.0 |
| 3 | Factory | 20×16 | 40 | 10 | Goods 0.0–0.1 |
| 4 | Dwelling | 12×10 | 10 | 0 | Goods 0.5–1.0 |
| 5 | Warehouse | 20×12 | 120 | 0 | Coal+Gravel+Goods each 0.2–0.6 |
| 6 | Depot | 22×26 | 0 | 0 | none |
| 7 | BusStop | 5×3 | 0 | 0 | none |
| 8 | ConstructionOffice | 20×22 | 0 | 0 | none |
| 9 | WaterPump | 10×8 | 0 | 0 | none |
| 10 | SewagePlant | 16×12 | 0 | 0 | none |
| 11 | HeatPlant | 18×12 | 40 | 0 | Coal 0.6–1.0 |
| 12 | CustomsOffice | 22×14 | 120 | 0 | none |

Every value agrees with the deleted arms and with the Phase 1 verified baseline. The collapsed
`|`-chains all map correctly: `Mine | Quarry => 60.0`, `WaterPump | SewagePlant => 0.0`, the
8-kind zero-vacancy chain, and the 5-kind no-band chain. Warehouse's `ResourceKind::ALL` fold
unrolls to three triples in `ALL`'s exact order (Coal, Gravel, Goods).

The seven production constants in `buildings.rs` are unchanged in value (only their line
numbers moved, 110→81 etc., from the 29 deleted lines). `heat.rs` and `water.rs` are untouched.

I also compared the top-level symbol multiset of each changed file against `HEAD`: identical
for `buildings.rs`, `labour.rs` and `storage.rs`. `catalogue.rs` differs only inside
`mod tests`; no production item was added or removed.

### 3. `default_policies`' fold is faithful — **confirmed by oracle, see F1**

Same start point (`StoragePolicies::default()`), same `.with()` path, therefore the same
`StorageBand::new` clamp. The clamp alters nothing on the way through: every value in the table
is already inside `[0,1]` with `min <= max`, so `min_pct.clamp(0.0, 1.0)` and
`max_pct.clamp(min_pct, 1.0)` are identity for all 13 rows. Order is irrelevant because no row
lists a resource twice — had one, `with` → `set` would let the last write win, but none does.

### 4. `PINNED_COLUMNS` earns its place — **confirmed**

- **Test-only:** declared at `catalogue.rs:353` inside `#[cfg(test)] mod tests`, used only at
  `:377`, `:389`, `:401`, all in the same module. Not reachable from production code and not
  compiled into it.
- **Aligned by assertion, not hope:** I swapped the Depot and BusStop rows and got
  `assertion left == right failed: pinned row 6 drifted out of ALL's order / left: BusStop /
  right: Depot`. The alignment check fires before the value check in all three tests.
- **All three fail on a corrupt row:** with Mine set to `14×15 / 61.0 / 7`, all three pinned
  tests failed. This is the mutation that at Phase 1's shape left all three *passing* — the pin
  genuinely restores a guard that the deletion would otherwise have destroyed.
- Its 13 rows match the Phase 1 verified baseline exactly.

The judgement call was right, and the doc comment stating what it does *not* prove is the part
that will stop the next reader deleting it as a duplicate.

### 5. Signatures and call sites untouched — **confirmed**

`footprint(self) -> Vec2`, `inventory_capacity(self) -> f32`, `workers_needed(self) -> u32`,
`default_policies(kind: BuildingKind) -> StoragePolicies` — all four signatures byte-identical
to `HEAD`. No caller was edited; the diff touches four files and none of them contains a call
site outside the functions themselves. Reference deltas fully explained in F2.

### 6. Scope — **clean**

Four files, all in `src/sim/`: `buildings.rs` (+5/−34), `catalogue.rs` (+208/−56),
`labour.rs` (+5/−18), `storage.rs` (+10/−22). Nothing in `src/game/`, `src/bin/`, or
`docs/adr/`.

**Phase 4's named exception is intact.** `run_heat_plants` (`heat.rs:108-110`) still queries
`(&Building, &mut Inventory, &mut HeatOutput)` with no `Without<ConstructionSite>`, and
`solve_water` (`water.rs:63-66`) still carries only `Without<Watered>`. Both files are entirely
untouched by this diff. `run_heat_plants` still burns `HEAT_PLANT_COAL_BURN` flat, with no
labour factor — the asymmetry Phase 4 must reproduce deliberately.

**Exactly zero intentional behaviour changes**, which is what this phase's contract asks. I
found no second one.

---

## Gates, run by me on the restored tree

```
cargo test --lib     138 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out   (0.46s)
cargo test --lib sim::save     6 passed; 0 failed
cargo clippy --lib --tests     9 warnings
cargo fmt --check              exit 0
```

The nine clippy warnings are `game/juice.rs:125`, `game/vehicles.rs:436`, `sim/dispatch.rs:179`,
`sim/households.rs:129`, `sim/network.rs:116`, `:117`, `:132`, `sim/water.rs:175`,
`sim/wires.rs:348` — the same nine as Phase 1, none in a file this phase touched.

Nine catalogue tests, the same count as Phase 1 (one out, one in), so the 138 total is
genuinely unchanged rather than padded.

**On the save tests:** they pass, but they are not evidence for this phase. When I corrupted
Mine's footprint, capacity and vacancies simultaneously, no save test failed — only the three
pinned tests and one labour test. The save suite exercises the changed path (it would panic if
`spec()` did) but asserts nothing about these columns. Consistent with the standing note that
the round-trip hash is blind to symmetric changes.

## Tree restoration

Every mutation was applied to a `cp`-backed copy and reverted. Final state verified by md5 and
by `git diff --numstat` matching the pre-review counts exactly:

```
0d49df7838e2b3cd99d021acdc0d7667  src/sim/catalogue.rs
15a4cbe92a03d43d80cbc8080e53b268  src/sim/buildings.rs
6708c7ddc73abfa473f49e1f4df9a575  src/sim/labour.rs
25bd5a7d41acf9799bd39a79137416d7  src/sim/storage.rs

5  34  src/sim/buildings.rs
208 56  src/sim/catalogue.rs
5  18  src/sim/labour.rs
10 22  src/sim/storage.rs
```

## Recommendation

Ship it, and close F1 before Phase 3 commits — extend `PINNED_COLUMNS` to a fourth element
holding the band triples. That is the fix the implementer already scoped and deferred to this
review, it restores membership checking as well as values, and it keeps the plan's own witness
rule true of every column rather than three of four.
