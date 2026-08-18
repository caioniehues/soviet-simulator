---
name: hot-reload-neither-source-has-it
description: Neither Factorio nor W&R hot-reloads prototype/recipe balance numbers at runtime — plain-text data files don't imply live-editable
metadata:
  type: reference
---

Verified 2026-08-17. Don't let "it's just data files" imply "therefore
hot-reloadable" without checking — neither reference implementation for this
project's catalogue research actually does it.

**Factorio — confirmed, citable.** `lua-api.factorio.com/latest/auxiliary/data-lifecycle.html`:
prototypes load once in the **Prototype (Data) Stage** at startup
(`data.lua`/`data-updates.lua`/`data-final-fixes.lua`); the **Control Stage**
begins on new game or save load, and prototypes "can no longer be modified"
once it begins. Confirmed by a modding-community web search too: changing a
recipe's numbers needs a full restart; only `control.lua` (runtime scripting)
and sprites (`runtime-sprite-reload`, an explicit special case) reload live.

**W&R — no evidence either way, reported as unknown, not as absent.**
Searched the modding wiki/forums for a `.ini` reload-without-restart workflow,
found nothing. The closed-source engine means this can't be settled from the
data files alone. If asked again, don't assert W&R lacks hot-reload — say
"not found, would need engine internals or a developer statement."

**Implication for this project:** a `&'static [BuildingSpec]` Rust const
table requiring a `cargo run`/recompile to rebalance (soviet-simulator's
actual plan, per `task_plan.md`) is not a regression against either reference
implementation — both bake prototype data at startup too.
