---
name: factorio-recipe-machine-binding
description: How Factorio actually binds recipes to crafting machines (categories, plural, as of 2.1) — verified against live docs and source, includes a version-drift trap
metadata:
  type: reference
---

## Verified 2026-08-17, against lua-api.factorio.com/latest AND live wube/factorio-data source

Machine and recipe are separate prototypes, bound many-to-many through a
shared category set — not a direct pointer either direction.

- `AssemblingMachinePrototype.crafting_categories` :: `array[RecipeCategoryID]`
  — "A list of recipe categories this crafting machine can use."
- `RecipePrototype.categories` :: `array[RecipeCategoryID]` optional, default
  `{"crafting"}` — "Controls which machines can craft this recipe."
- A recipe is craftable in a machine iff their category sets intersect.
- Confirmed live in `wube/factorio-data`, `base/prototypes/entity/entities.lua`:
  `assembling-machine-1` has `crafting_categories = {"crafting", "advanced-crafting"}`,
  `assembling-machine-2` adds `"crafting-with-fluid"`. Both share
  `"advanced-crafting"` — direct proof one recipe category is craftable in
  more than one machine.
- Confirmed live in `base/prototypes/recipe.lua`: `categories = {"oil-processing"}`
  on the basic-oil-processing recipe.

## Version-drift trap — flag this every time Factorio recipe fields come up

**The field is `categories` (plural, array), not `category` (singular).**
Training data through Factorio 1.1/early 2.0 will say `category` (singular) —
that was real, but Factorio 2.0.49 added a second field
`additional_categories` to avoid breaking mods, then **2.1 unified both into
the single plural `categories`**, which is what `lua-api.factorio.com/latest`
documents and what current `factorio-data` source uses. If this comes up
again: verify the live docs/source before writing `category` anywhere, it's
probably stale.

## Why this doesn't transfer to soviet-simulator

Factorio's machine/recipe split exists to serve **moddability and machine
tiers** — recipes must stay valid as machines are added/upgraded/modded
independently. This project has no mod support in 1.0 and ~13→26 building
kinds total; `task_plan.md` (`.planning/2026-08-17-data-driven-buildings/`)
already correctly decided `Recipe` lives inside `BuildingSpec`, one recipe per
building kind, not a shared catalogue slotted into generic machines. A
`crafting_categories`-style indirection would solve a problem this project
doesn't have — don't recommend it without a mod-support requirement first.
