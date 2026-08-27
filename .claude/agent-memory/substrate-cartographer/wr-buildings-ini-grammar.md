---
name: wr-buildings-ini-grammar
description: Where W&R's building catalogue physically lives, its verified grammar, and a false claim in this project's own findings.md about file count
metadata:
  type: reference
---

## Location (primary source, verified 2026-08-17)

`~/.local/share/Steam/steamapps/common/SovietRepublic/media_soviet/buildings_types/`

**1472 files total, but only 488 are `.ini`.** The rest: 490 `.bbox`
(collision/footprint geometry), 481 `.fire` (flammability), 8 `.nmf` (mesh), 5
`.naf` (animation). `490+488+481+8+5 = 1472` exactly.

**False claim to correct if it resurfaces:** this project's
`.planning/2026-08-17-data-driven-buildings/findings.md` (and the task brief
that quoted it) both say "1472 `.ini` files." That's wrong — it's 1472 files
of mixed type. Every count/analysis should use the 488 figure. Filed a
correction in `.planning/2026-08-17-data-driven-buildings/reports/phase1-research.md`.

## Grammar, verified by grep across all 488 `.ini` files

- Directive family prefixes collide: `^\$PRODUCTION` matches
  `$PRODUCTION_SEWAGE_POLLUTION`, `$PRODUCTION_DECREASE_ACCORDING_YEAR`,
  `$PRODUCTION_CONNECT_TO_SUN/WIND` too. `^\$CONSUMPTION` matches
  `$CONSUMPTION_PER_SECOND`, `$CONSUMPTION_WATER_REQUIRED_QUALITY`,
  `$CONSUMPTION_INCREASE_ACCORDING_YEAR` too. **Always grep
  `^\$PRODUCTION[[:space:]]` / `^\$CONSUMPTION[[:space:]]` (exact directive,
  space-delimited) or you overcount.**
- `$PRODUCTION` distribution (exact): 407 files have 0, 75 have 1, 4 have 2, 2
  have 3. Real multi-output example: `oil_rafinery.ini` (`fuel` + `bitumen`
  from one `oil` input — genuinely two products). The other 2-and-3-line files
  (`nuclear_fuel_plant.ini`, `uranium_conversion.ini`, both
  `powerplant_nuclear_*.ini`) have a `$PRODUCTION vehicles 1.0` line that is
  **not a real second product** — it's engine bookkeeping to spawn a shipping
  container (the file's own comment says so). Don't read those as evidence for
  N-ary outputs; `oil_rafinery` is the real evidence.
- `$CONSUMPTION` distribution (exact): 418 have 0, 30 have 1, 23 have 2, 10
  have 3, 1 has 4, 1 has 5, 4 have 6 (all `$TYPE_PRODUCTION_LINE` vehicle
  assembly buildings), 1 has 7 (`production_airplane.ini`).
- **Workers are never a `$CONSUMPTION` line.** `grep -iE
  '^\$CONSUMPTION[[:space:]_]*(worker|workers)' *.ini` → zero hits across all
  488 files. Headcount is always the separate `$WORKERS_NEEDED` key (156
  files).
- **Electricity, water, fuel(coal/gas/nuclearfuel) are ordinary
  `$CONSUMPTION <res> <n>` lines** — same grammar as any material. `heat` is
  likewise an ordinary `$PRODUCTION heat <n>` line
  (`heating_plant_big.ini`/`heating_plant_small.ini`). The type tag
  (`$TYPE_HEATING_PLANT`/`$TYPE_POWERPLANT`/`$TYPE_FACTORY`) drives behavior,
  not the production/consumption grammar — nothing about "power" or "heat" is
  special-cased in the directive syntax itself.
- **Electricity's encoding is internally inconsistent** — worth knowing before
  citing an example. `asphalt_plant.ini`'s `$CONSUMPTION eletric 3` (the line
  quoted in this project's findings.md) is the *only* file in the whole corpus
  using that exact form (`grep -lE '^\$CONSUMPTION[[:space:]]+eletric'` → 1
  file). All 38 other electricity-consuming buildings use
  `$CONSUMPTION_PER_SECOND eletric <n>` instead. Treat `asphalt_plant.ini` as
  an outlier, not the representative case, if anyone reaches for it as "the"
  canonical electricity example again.
- **No `fuel` resource is ever consumed at the building level**
  (`grep -lE '^\$CONSUMPTION[[:space:]]+fuel\b' *.ini` → 0 files). `fuel` is
  something `oil_rafinery.ini` *produces*; vehicles consume it elsewhere, not
  buildings.
- **Renewables (solar/wind) have no `$CONSUMPTION` line at all** —
  `powerplant_solar.ini`/`powerplant_wind1.ini` use `$PRODUCTION_CONNECT_TO_SUN`/
  `$PRODUCTION_CONNECT_TO_WIND` as an output-side multiplier instead of a
  consumed input. A fourth distinct gating mechanism, alongside consumed
  resources, `$WORKERS_NEEDED`, and `$CONSUMPTION_WATER_REQUIRED_QUALITY`
  (a quality *threshold*, separate from consumed quantity).
- **~80 distinct `$TYPE_*` tags** exist (`grep -ohE '^\$TYPE_[A-Z_0-9]*'
  *.ini | sort | uniq -c`). Notably 9 separate `$TYPE_MINE_*` tags, one per
  mineral (coal/iron/bauxite/uranium/wood/oil/gravel/water/water_surface) —
  strong evidence the reference game doesn't generalize "mine" as one
  machine-type + a resource parameter; the product is baked into the type tag.

## Inheritance and versioning — confirmed absent/flat

Grepped all 488 files for `$INHERIT`/`$BASE`/`$PARENT`/`$EXTENDS`/`$INCLUDE`/
`$COPY_FROM` — **zero matches, no inheritance mechanism exists.** Where W&R
has near-duplicate buildings (27 `_v2`/`_v3`-suffixed files), it's full file
duplication (diffed `powerplant_coal.ini` vs `_v2`: identical
production/consumption, only geometry differs), and the superseded original
is tagged `$OBSOLETE` (17 files) and **kept, never deleted** — presumably so
old saves referencing the old type name still resolve. No numeric type ID
field exists anywhere in the grammar; type identity in every file opened is
the filename stem, a string. (This last point is inference from data shape,
not proven — can't read the closed-source engine's save format.)

## Hot-reload — no evidence either way

Searched modding wiki/forums for whether `.ini` changes take effect without a
restart. Found nothing. Report as unknown, don't assert it doesn't exist.
