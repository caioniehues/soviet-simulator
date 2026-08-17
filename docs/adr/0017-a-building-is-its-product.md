# A building is its product; recipes bind to kinds, not to generic machines

**Status:** decided 2026-08-17, **partly built** — `Recipe` and the `BUILDINGS` table exist
(`src/sim/catalogue.rs`, phase 1); nothing reads them yet.

There are two ways a simulation game models what a building makes, and both reference
implementations were checked directly rather than recalled.

**Factorio separates them.** `AssemblingMachinePrototype` declares
`crafting_categories: array[RecipeCategoryID]`, `RecipePrototype` declares
`categories: array[RecipeCategoryID]`, and any machine whose set intersects the recipe's can
craft it. The category is a join table, not a pointer: `assembling-machine-1` and
`assembling-machine-2` both list `"advanced-crafting"`, so a recipe in that category runs in
either. An assembler is a generic box; the player slots the recipe in.

**Workers & Resources bakes the product into the type.** Its 488 `buildings_types/*.ini`
files carry nine distinct mine tags — `$TYPE_MINE_COAL`, `_IRON`, `_BAUXITE`, `_URANIUM`,
`_WOOD`, `_OIL`, `_GRAVEL`, `_WATER`, `_WATER_SURFACE` — where Factorio would have one
mining drill and nine recipes. A W&R building *is* what it makes.

**We take the W&R shape.** Factorio's separation exists for two forces we do not have:
mod support, where machines and recipes are authored by different people and the category is
the only contract between them, and machine tiers, where a recipe must stay valid as
assembler 1 → 2 → 3 unlock more categories. This project ships no mod support in 1.0, and the
charter already commits R4 to twelve recipe buildings for thirteen resources — one building
per product, which is the W&R shape stated in the plan before anyone looked at either game.
A `crafting_categories` indirection would be machinery for a problem we don't have.

`Recipe` stays its own type inside `BuildingSpec` rather than being melted into it. That costs
nothing today and keeps the Factorio split a move rather than a rewrite, if a later rung ever
wants one building to make more than one thing.

## What the reference data settled about the shape

**`outputs` is a slice, not a scalar.** Of 488 `.ini` files, 407 produce nothing, 75 produce
exactly one thing, 4 produce two and 2 produce three. `oil_rafinery.ini` is the real case —
`$CONSUMPTION oil 0.5` becomes `$PRODUCTION fuel 0.25` and `$PRODUCTION bitumen 0.15`, two
independently useful products. `inputs` likewise: of the 70 files that consume anything, 40
consume three or more.

**Only staffing earns a dedicated field.** Across all 488 files there is not one
`$CONSUMPTION worker` line; headcount is always `$WORKERS_NEEDED`, its own key. Everything
else that gates production — electricity, water, fuel — is an ordinary consumption line with
the same grammar as gravel. `heating_plant_big.ini` is `$PRODUCTION heat 350` /
`$CONSUMPTION coal 0.28`: heat is a resource name, not a special case, and the `$TYPE_*` tag
is what drives behaviour. That is the argument for eventually carrying power, heat and water
as ordinary `(ResourceKind, f32)` entries rather than as three bespoke fields.

**We are not there yet, deliberately.** Our power is a pooled network solve, water is a
boolean `Watered` component and heat is a pair of components; none are `ResourceKind`
members. Phase 1 therefore transcribes today's model faithfully — `power`, `water` and `heat`
are separate typed fields on `BuildingSpec` — and the unification waits for R4, when
`ResourceKind` grows from 3 to 16 and the question has to be answered anyway. Recording the
target now so the separate fields read as a staging post rather than the design.

**Do not copy W&R's own inconsistency.** `$CONSUMPTION eletric 3` appears in exactly one file
in the corpus (`asphalt_plant.ini`, and it is the line this project's notes had quoted as
canonical); the other 38 electricity consumers use `$CONSUMPTION_PER_SECOND eletric`, a
different directive with a different unit. A schema written from scratch gets one unit for
all inputs.

## Two things neither reference implementation offers

**Prototype inheritance does not exist in either.** W&R's grammar has no `$INHERIT`,
`$BASE`, `$PARENT`, `$EXTENDS` or `$INCLUDE` — checked across all 488 files, zero matches.
Near-duplicate buildings are full file copies with a `_v2`/`_v3` suffix, and the superseded
original is tagged `$OBSOLETE` and *kept* rather than deleted, so old saves still resolve.
That is the same never-reuse discipline as ADR 0004, reached independently by a shipped game
with a far larger save-compatibility burden. Factorio's `categories` is a many-to-many join,
not inheritance either. If a later design wants "the same recipe with different rates", that
is a novel decision, not a precedent-backed one.

**Neither hot-reloads prototype data.** Factorio's own lifecycle documentation states
prototypes "can no longer be modified" once the control stage begins — changing a recipe's
numbers needs a restart. For W&R no citation either way was found, so it is recorded as
unknown rather than asserted. The consequence is that a Rust `const` table requiring
`cargo run` after a balance edit is not a regression against how these games actually work;
it is what they do. If balancing twelve buildings in R4 turns painful, the answer is a RON
override loader on top of the table — a loader, not a reshape, which is why the spec types
carry serde derives.

Evidence gathered 2026-08-17 against the W&R install on this machine and against live
`wube/factorio-data` plus `lua-api.factorio.com`; full counts and quotations in
`.planning/2026-08-17-data-driven-buildings/reports/phase1-research.md`. One correction fell
out of it: the "1472 buildings" figure this project had been repeating is the whole
directory's file count across `.ini`, `.bbox`, `.fire` and meshes. The building count is 488.
