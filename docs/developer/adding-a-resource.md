# Adding a resource

**Kind:** guide
**Authority:** operational
**Status:** active
**Owner:** project lead
**Verified-at:** `4e9e930b2a73`
**Last verified:** 2026-08-28

## Scope check first

The charter fixes the 1.0 catalogue at fifteen domestic resources plus import-only Medicine, and
Water is a utility, never cargo. `base_mod/items.lua` currently declares **21** items, several of
which are not in the charter's list. Adding an item is a scope question before it is a data change
— cite the charter row or the decision that admits it.

## Declare it

In `base_mod/items.lua`, inside `data:extend { … }`:

```lua
{
    type = "item",
    name = "copper",          -- the id; lowercase, hyphenated
    label = "Copper",
    optout_exttrade = false,  -- true removes it from border trade (exactly one item sets this today)
},
```

Parsed by `prototypes/src/prototypes/item.rs` (`ItemPrototype { id, label, optout_exttrade }`).
There is **no** mass, volume, unit, storage class or transport class yet — the resources
specification asks for them; adding a field means extending `ItemPrototype::from_lua` and every
consumer that should read it.

## Give it a producer and a consumer

A resource nobody produces or consumes is vocabulary, not a mechanism. Add it to a recipe in
`base_mod/companies.lua` (`consumption = {{"copper", 2}}`, `production = {{"wire", 1}}`), or make
it importable (it is, unless `optout_exttrade`). Validation refuses recipe numbers below 1
(`prototypes/src/validation.rs`, `sov-k3w`).

## Prove it

- A scenario test that produces or consumes it through `recipe_act` and asserts stock changes at
  physical endpoints, never at match time.
- `cargo test -p prototypes` for parsing and validation; `cargo test -p simulation` for behaviour.
- If it should reach citizens, it needs a desire (`souls/desire/`) — today only bread has one.

## Document it

The resources specification's substrate section and [current substrate — prototypes](../architecture/current-substrate.md#prototypes)
count items; update both. Add the mechanic row to the [mechanics index](../reference/mechanics-index.md)
if it introduces a new one.

## Related

- [Resources (design)](../simulation/physical-economy/resources.md)
- [Resources specification](../reference/specifications/resources.md)
- [Adding a building](adding-a-building.md)
