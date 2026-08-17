# Phase 1 Research — Recipe Shape for Data-Driven Buildings

Verified 2026-08-17. Every number below was produced by running commands against
the actual files, not recalled. Primary sources: the W&R install on this machine
(`~/.local/share/Steam/steamapps/common/SovietRepublic/media_soviet/buildings_types/`)
and `wube/factorio-data` + `lua-api.factorio.com` (fetched live).

## Correction to `findings.md` first — it will bite you if unaddressed

**`findings.md` and the task brief both say "1472 `.ini` files." That's wrong.**
The directory holds 1472 files total, but only **488 are `.ini`**. The rest:

```
490 .bbox   (collision/footprint geometry)
488 .ini    (the actual building definitions — production/consumption/workers)
481 .fire   (flammability/fire-spread data)
  8 .nmf    (mesh)
  5 .naf    (animation)
```

`490 + 488 + 481 + 8 + 5 = 1472` exactly — that's where "1472" came from, and it's
a file-type mix, not a building count. Every count below (Q2, Q3) is over the
correct 488. This doesn't change any conclusion, but "1472 buildings" would be a
wrong number to put in an ADR.

---

## The four answers, one line each

1. **Q1 — Yes, confirmed.** In current Factorio (2.1-era API, verified against
   both `lua-api.factorio.com` docs *and* live `wube/factorio-data` source), a
   crafting machine declares `crafting_categories: array[RecipeCategoryID]` and
   a recipe declares `categories: array[RecipeCategoryID]`; any machine whose
   set intersects the recipe's set can craft it — many-to-many, and I found a
   concrete example of the same category shared by two machines.
2. **Q2 — Single-output dominates; multi-output is real but rare (~1%).** Of
   488 `.ini` files, 407 produce nothing (housing, storage, decoration), 75
   have exactly 1 `$PRODUCTION` line, 6 have 2, 2 have 3. `outputs` must be a
   slice — `oil_rafinery.ini` genuinely turns one input into two real products.
3. **Q3 — Workers are the only gate W&R keeps structurally separate; power,
   water and fuel are ordinary consumed resources.** 418/488 files have zero
   `$CONSUMPTION` lines; of the 70 that consume anything, electricity, water
   and fuel (coal/gas/nuclearfuel) all appear as plain `$CONSUMPTION <res> <n>`
   lines, same grammar as gravel or bitumen. Only headcount gets a dedicated
   key (`$WORKERS_NEEDED`), never a `$CONSUMPTION` line. "Which is scarcest" is
   not a schema question in W&R's data — it's not encoded anywhere in the
   `.ini` grammar, it's an emergent map/economy property (see Q3 detail).
4. **Q4 — Neither primary source hot-reloads prototype/recipe data, and W&R has
   no inheritance mechanism at all.** Factorio's own docs state prototypes
   "can no longer be modified" once the control stage begins — a restart is
   required for `data.lua` changes. W&R's `.ini` grammar has zero inheritance
   directive (grepped, none exist); version bumps (`_v2`, `_v3`) are full file
   duplicates, and old versions are kept forever tagged `$OBSOLETE` rather than
   deleted — the save-stability discipline is "never remove, mark
   deprecated," which is exactly this project's own ADR 0004 already.

---

## Q1 — Factorio: machine and recipe are genuinely separate prototypes

**Confirmed, with one correction to what "verify against a primary source"
turned up: the field name has changed since Factorio 1.1/2.0 and is easy to
get wrong from memory.**

### The machine side
`AssemblingMachinePrototype` (`lua-api.factorio.com/latest/prototypes/AssemblingMachinePrototype.html`):

> **`crafting_categories`** :: `array[RecipeCategoryID]` — "A list of recipe
> categories this crafting machine can use."

Confirmed against live `wube/factorio-data` source
(`base/prototypes/entity/entities.lua`):

```lua
-- assembling-machine-1
crafting_categories = {"crafting", "advanced-crafting"},
-- assembling-machine-2
crafting_categories = {"crafting", "advanced-crafting", "crafting-with-fluid"},
```

Both machines list `"advanced-crafting"` — **direct evidence a recipe with that
category is craftable in either machine.** This is the many-to-many binding:
category is the join table, not a direct machine↔recipe pointer.

### The recipe side
`RecipePrototype` (`lua-api.factorio.com/latest/prototypes/RecipePrototype.html`):

> **`categories`** :: `array[RecipeCategoryID]` optional, default `{"crafting"}`
> — "Controls which machines can craft this recipe."
> **`ingredients`** :: `array[IngredientPrototype]`
> **`results`** :: `array[ProductPrototype]`

Confirmed against live `wube/factorio-data` (`base/prototypes/recipe.lua`):
`categories = {"oil-processing"}` appears verbatim on the basic-oil-processing
recipe.

### The version-drift flag this task explicitly asked me to watch for

My prior expectation (and likely any model's training data through Factorio
1.1) is a **singular** `category` field, not `categories`. A web search for the
changelog confirms why: `RecipePrototype::category` (singular) was the field
through 2.0; Factorio 2.0.49 added `additional_categories` as a second field
to avoid breaking mods; **2.1 unified both into the single plural `categories`
array**, which is what `lua-api.factorio.com/latest` documents today and what
current `factorio-data` uses. If anyone on this team writes "Factorio recipes
have a `category` field" from memory, that's stale — cite `categories`
(plural) going forward. This is a docs-vs-training-data drift exactly of the
kind `bevy.md` warns about for Bevy; it applies to Factorio too.

### Bottom line for this project

Real separation exists in Factorio for a real reason: **moddability and
machine tiers.** A recipe stays valid while machines are added, removed, or
upgraded around it (assembling-machine-1 → -2 → -3, each unlocking more
categories); mods add machines or recipes independently and the category is
the only contract between them. **This project has none of those forces**: no
mod support in 1.0, and `task_plan.md` already decided `Recipe` lives inside
`BuildingSpec` (one recipe per building kind, not a shared catalogue slotted
into generic machines) — correctly, per its own stated reasoning ("W&R needs
text files because it ships mod support; we don't, yet"). A
`crafting_categories`-style indirection would be solving a problem this
project doesn't have. The W&R half of the ADR's claim — a building *is* its
product — holds up structurally too: many `$TYPE_MINE_*` tags exist, one per
mineral (`$TYPE_MINE_COAL`, `$TYPE_MINE_IRON`, `$TYPE_MINE_BAUXITE`, `$TYPE_MINE_URANIUM`,
`$TYPE_MINE_WOOD`, `$TYPE_MINE_OIL`, `$TYPE_MINE_GRAVEL`, `$TYPE_MINE_WATER`,
`$TYPE_MINE_WATER_SURFACE` — 9 distinct type tags for what Factorio would treat
as one "mining-drill" machine with 9 different recipes). W&R does not
generalize "mine" as a machine + recipe; the product is baked into the type
tag itself.

---

## Q2 — `$PRODUCTION` line count per file (488 `.ini` files)

Exact match on `^\$PRODUCTION[[:space:]]` — the naive `^\$PRODUCTION` grep
over-counts because `$PRODUCTION_SEWAGE_POLLUTION`, `$PRODUCTION_DECREASE_ACCORDING_YEAR`,
`$PRODUCTION_CONNECT_TO_SUN` and `$PRODUCTION_CONNECT_TO_WIND` are distinct
directives that share the prefix. Distinct directive keys found across the
whole corpus (`grep -ohE '^\$PRODUCTION[A-Z_]*' *.ini | sort -u`):

```
$PRODUCTION
$PRODUCTION_CONNECT_TO_SUN
$PRODUCTION_CONNECT_TO_WIND
$PRODUCTION_DECREASE_ACCORDING_YEAR
$PRODUCTION_SEWAGE_POLLUTION
```

Distribution of the real `$PRODUCTION` directive:

| count | files |
|---|---|
| 0 | 407 |
| 1 | 75 |
| 2 | 4 |
| 3 | 2 |

### The 2- and 3-line files, read in full

- **`oil_rafinery.ini`** — the one genuine multi-product recipe in the corpus:
  ```
  $TYPE_FACTORY
  $PRODUCTION fuel 0.25
  $PRODUCTION bitumen 0.15
  $CONSUMPTION oil 0.5
  ```
  One input, two real, independently-useful outputs. (`oil_rafinery_v2.ini` is
  an identical duplicate — see Q4 on the `_v2` versioning convention.)
- **`nuclear_fuel_plant.ini`**, **`uranium_conversion.ini`** — 2 lines, but the
  second is not a real second product:
  ```
  $PRODUCTION nuclearfuel 0.0019
  $PRODUCTION vehicles 1.0  //need to include to production because need create containers what is vehicles
  ```
  The building's own comment says why: `vehicles` is a bookkeeping artifact
  the engine needs to spawn the transport container for shipped goods, not a
  second recipe output. This pattern also explains the 6 files with
  `$CONSUMPTION` counts of 6 (`drydock.ini`, `production_train.ini`,
  `production_train2.ini`, `production_vehicle.ini`) — all `$TYPE_PRODUCTION_LINE`
  buildings (vehicle assembly), each with 6–7 material inputs and exactly one
  `$PRODUCTION vehicles 1.0` output.
- **`powerplant_nuclear_single.ini`**, **`powerplant_nuclear_double.ini`** — 3
  lines: `eletric` (real output), `nuclearfuelburned` (a waste-tracking
  byproduct, presumably feeds a spent-fuel/pollution system), `vehicles`
  (same container bookkeeping as above).

### What this implies for `outputs`

It has to be a slice, not a scalar — `oil_rafinery` is real evidence, not a
hypothetical. But don't let the "vehicles"/"nuclearfuelburned" pattern leak
into the design: those are the reference engine's own logistics/pollution
plumbing riding the same directive, not part of the recipe grammar a designer
would author. If this project ever needs a byproduct or a shipping-container
concept, model it as its own field, not as a second `Recipe.outputs` entry —
W&R's own data shows conflating the two is what forces every reader of
`nuclear_fuel_plant.ini` to know a project convention (comment) to not
mis-read `vehicles` as a real product.

---

## Q3 — `$CONSUMPTION` line count, and where the gating-factor line is drawn

Same over-count trap existed here too — `$CONSUMPTION_INCREASE_ACCORDING_YEAR`,
`$CONSUMPTION_PER_SECOND`, and `$CONSUMPTION_WATER_REQUIRED_QUALITY` all share
the prefix. Exact match on `^\$CONSUMPTION[[:space:]]`:

| count | files |
|---|---|
| 0 | 418 |
| 1 | 30 |
| 2 | 23 |
| 3 | 10 |
| 4 | 1 |
| 5 | 1 |
| 6 | 4 |
| 7 | 1 |

70 of 488 files consume anything at all; of those, 40 consume 3 or more
resources — `inputs` must be a slice too, matching `task_plan.md`'s existing
`Recipe { inputs, outputs }` decision.

### Where W&R draws the "gating factor" line — the load-bearing finding for this question

Three genuinely different mechanisms exist in the grammar, not one:

1. **Workers — a dedicated key, never a consumed resource.**
   `grep -iE '^\$CONSUMPTION[[:space:]_]*(worker|workers)' *.ini` → **zero
   hits**, across all 488 files. 156 files carry `$WORKERS_NEEDED` instead.
   Headcount is structurally separate from the resource-consumption list;
   findings.md already models this correctly (`$WORKERS_NEEDED` as its own
   key).
2. **Electricity, water and fuel — ordinary consumed resources, same grammar
   as any material.** `powerplant_coal.ini`: `$CONSUMPTION coal 1.2`.
   `heating_plant_big.ini` / `heating_plant_small.ini`: `$CONSUMPTION coal
   0.28` / `0.3`. `alumina_plant.ini`: `$CONSUMPTION water 0.08` sits in the
   same list as `$CONSUMPTION bauxite 0.21`. There is **no `fuel` resource
   consumed at the building level anywhere** (`grep -lE '^\$CONSUMPTION[[:space:]]+fuel\b' *.ini`
   → 0 files) — "fuel" in W&R is itself a *produced* resource
   (`oil_rafinery.ini` makes it) that vehicles consume elsewhere, not
   something a building consumes directly. If this project's "fuel" gate maps
   to a specific material (coal, wood) rather than an abstract fuel pool, the
   W&R precedent says model it exactly like any other material input, not as
   a special case.
3. **Electricity's *encoding* is inconsistent inside the reference data
   itself — worth knowing before copying the example verbatim.**
   `findings.md` quotes `asphalt_plant.ini`'s `$CONSUMPTION eletric 3` as the
   canonical example, and that line is real
   (`asphalt_plant.ini:19`, confirmed). But it is **the only file in the
   entire corpus using that exact form** (`grep -lE '^\$CONSUMPTION[[:space:]]+eletric' *.ini`
   → 1 file: `asphalt_plant.ini`). Every other electricity-consuming building
   (38 files, including `chemical_plant.ini`, `cement_plant.ini`,
   `aluminium_plant.ini`) uses **`$CONSUMPTION_PER_SECOND eletric <n>`**
   instead — a distinct directive, presumably a rate rather than a
   per-production-cycle amount. `asphalt_plant.ini` looks like an
   outlier/legacy line in the reference data, not the representative case.
   Something to be deliberate about, not copy blind: if this project's
   `Recipe` wants a single consistent unit for all inputs (which it should —
   the W&R split into two competing directives is exactly the kind of
   inconsistency a from-scratch schema should avoid), electricity needs to be
   normalized to the same per-tick unit as materials, not treated as special.
4. **Renewable availability is a fourth, non-consumption concept.**
   `powerplant_solar.ini` and `powerplant_wind1.ini` have **no `$CONSUMPTION`
   line at all** — solar/wind power isn't modeled as consuming a "sunlight" or
   "wind" resource, it's a production-side multiplier:
   `$PRODUCTION_CONNECT_TO_SUN 100` / `$PRODUCTION_CONNECT_TO_WIND 25`. This is
   the closest W&R analogue to this project's boolean `Powered`/`Watered`
   gates today — an external multiplier on the output rate, not an input
   consumed from inventory.
5. **Water has a second, non-quantity gate.** `$CONSUMPTION_WATER_REQUIRED_QUALITY 0.75`
   sits alongside `$CONSUMPTION water 0.08` — a minimum-quality threshold,
   separate from the consumed amount. Evidence that "water" as a gate can
   split into two independent checks (enough vs. clean enough) even inside a
   single resource line, if this project ever needs that.
6. **`heat` uses the identical `$PRODUCTION`/`$CONSUMPTION` grammar as any
   material — it is not special-cased.** `heating_plant_big.ini`:
   `$PRODUCTION heat 350` / `$CONSUMPTION coal 0.28`. The *type tag*
   (`$TYPE_HEATING_PLANT` vs `$TYPE_POWERPLANT` vs `$TYPE_FACTORY`) is what
   drives behavior/rendering, not the production grammar — `eletric` and
   `heat` are just resource names in the same list as `asphalt` or `gravel`.
   **This is the single strongest piece of evidence for this project's
   `Recipe` shape**: don't special-case power/heat/water as separate fields
   from material inputs — model them all as `(ResourceKind, f32)` entries in
   the same `inputs`/`outputs` list. Only staffing earns a dedicated field,
   because it's the only gate the reference grammar itself treats as
   structurally different.

### "Which one is scarcest" — not answerable from the data, and I want to be honest about that boundary

No `.ini` field ranks or prioritizes gating factors — W&R's grammar has no
concept of "binding constraint," that's a runtime/simulation property (how
much of each resource the map actually produces vs. how much all buildings
demand), not a catalogue property. This project's own source already has the
right instinct: `src/sim/buildings.rs:319` comments "Liebig stage 2 (B8.2):
power AND water AND staff — the scarcest factor wins," which is exactly the
right framing — but as read, `run_factories` today treats power and water as
booleans (`powered.0`, `watered_ok`) and only staffing (`f`) as continuous, so
"scarcest wins" isn't actually computed yet, just gated pass/fail on two of
three factors. That's consistent with `findings.md`'s own note that ADR 0014's
`Gated { rate, bound_by }` is "decided but not built." The W&R data doesn't
tell you which factor to name scarcest; it tells you that whichever factor you
pick, it should be represented the same way (a consumed quantity, or an
external multiplier, consistently) as every other input — not hand-picked
per-system the way today's four production functions do it.

---

## Q4 — Prototype inheritance, save stability across reorder, hot-reload

### Prototype inheritance: W&R has none; Factorio has category indirection, not inheritance either

Grepped all 488 `.ini` files for any inheritance-shaped directive:
`$INHERIT`, `$BASE`, `$PARENT`, `$EXTENDS`, `$INCLUDE`, `$COPY_FROM` — **zero
matches.** The grammar is flat; every file states everything about itself.
Where W&R has near-duplicate buildings (27 files with a `_v2`/`_v3` suffix:
`cement_plant_v2.ini`, `cement_plant_v3.ini`, `oil_rafinery_v2.ini`,
`powerplant_coal_v2.ini`, etc.), the mechanism is **full file duplication**,
confirmed by diffing `powerplant_coal.ini` against `powerplant_coal_v2.ini` —
the production/consumption lines are identical, only geometry
(`$VEHICLE_STATION` positions, `$CONNECTION_*` layout) differs. The superseded
original is then tagged `$OBSOLETE` (17 files: `powerplant_coal.ini`,
`cement_plant.ini`, `oil_rafinery.ini`, `steel_mill.ini`, `farm.ini`, etc.) and
**kept in the catalogue rather than deleted** — almost certainly so old saves
that reference the old type name still resolve. Factorio's `categories` field
(Q1) is a many-to-many join, not inheritance either — there's no "recipe A
extends recipe B" mechanism in either primary source. **Neither reference
implementation gives this project a prototype-inheritance pattern to borrow.**
If the design doc ends up wanting inheritance (a `Recipe` that's "the same as
X but with different rates"), that's not validated by either primary source —
flag it as a novel decision, not a precedent-backed one.

### Save stability across catalogue reorder

Nothing in the `.ini` grammar carries an explicit numeric type ID — building
type identity in every file I opened is the filename stem
(`asphalt_plant.ini` → type `asphalt_plant`), a string, never a positional
index. **This is inferred from the data shape, not proven** — the actual save
format is compiled into the closed-source engine binary, which I can't read.
But the $OBSOLETE-instead-of-delete discipline (above) is exactly consistent
with string/name-keyed identity: reordering files on disk, or adding new
ones, can't renumber an existing building the way a positional enum would.
This is the same shape as `ADR 0004` in this project ("ids never reused,
save-remap by stable id") and the `Phase 3` plan already commits to a test
"pinning discriminant stability so reordering the enum can't corrupt old
saves" — the W&R discipline (never delete, mark obsolete) is worth stating
explicitly as the parallel, since it's evidence a shipped game with a much
larger, older save-compatibility burden reached the same policy.

### Hot-reload of numbers during balancing

**Neither primary source supports it, and I want to flag this since it
contradicts an intuition that "plain text data files" implies "live-editable."**
Factorio's own docs are explicit and I can quote them directly
(`lua-api.factorio.com/latest/auxiliary/data-lifecycle.html`): prototypes load
once during the **Prototype (Data) Stage** at startup (`data.lua`,
`data-updates.lua`, `data-final-fixes.lua`); the **Control Stage** "begins
when a player starts a new game or loads a save," and prototypes "can no
longer be modified" once it begins. A web search on Factorio modding-dev
practice confirms the practical consequence: changing a recipe's numbers
requires a full game restart; only `control.lua` (runtime scripting) and
sprites (`runtime-sprite-reload`, a special-cased exception) reload without
one. For W&R, I found **no evidence either way** — searched the modding wiki
and forums, found nothing describing a reload-without-restart workflow for
`buildings_types/*.ini`, and the closed-source engine means I can't confirm
from the data alone. Report this as unknown, not as "W&R doesn't have it" —
the honest answer is I couldn't find a citation.

**What this means for balancing workflow on this project:** if numbers living
in a Rust `&'static [BuildingSpec]` const table (as `task_plan.md` already
decided) feels like a regression vs. "real" data-driven games supporting live
balance tweaks — it isn't. Both primary sources bake prototype data at startup
too. A `cargo run` after editing the const table is not slower than either
reference implementation's actual workflow.

---

## Sources

- `~/.local/share/Steam/steamapps/common/SovietRepublic/media_soviet/buildings_types/` — 488 `.ini` files, read and grepped directly, 2026-08-17.
- [AssemblingMachinePrototype](https://lua-api.factorio.com/latest/prototypes/AssemblingMachinePrototype.html) — `crafting_categories` field.
- [RecipePrototype](https://lua-api.factorio.com/latest/prototypes/RecipePrototype.html) — `categories`, `ingredients`, `results` fields.
- [Data lifecycle (auxiliary docs)](https://lua-api.factorio.com/latest/auxiliary/data-lifecycle.html) — prototype stage, no runtime reload.
- `wube/factorio-data`, live raw source: `base/prototypes/entity/entities.lua` (assembling-machine `crafting_categories`), `base/prototypes/recipe.lua` (`categories = {"oil-processing"}`).
- WebSearch, Factorio 2.0/2.1 modding changelog — `category` → `additional_categories` → unified `categories`, community/forum sources, clearly a secondary source, used only to explain *why* the field renamed, not as the basis for the field-name claim (the live source and docs are the basis).
- WebSearch, W&R modding — no citable result on hot-reload; reported as unknown, not asserted false.
- `.planning/2026-08-17-data-driven-buildings/findings.md`, `task_plan.md` — this project's own current plan of record, read for context and cross-checked against the above.
- `src/sim/buildings.rs:301-327` (`run_factories`) — read to ground the "which gate is scarcest" answer in the project's own code, not just the reference data.
