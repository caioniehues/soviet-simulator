# Resources

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** economy
**Last verified:** 2026-09-03

| Scope | 1.0 — charter row Resources and production |

## What this is

A resource is a named physical thing the simulation tracks. The Planner builds an economy
around fifteen domestic resources, plus import-only Medicine. The granularity rule says:
split a resource only when it changes routing, storage, substitution, bottleneck, allocation,
quality, timing, or consequences. Gosplan tracked roughly 1,943 product categories (CONFIRMED,
Lane A-01); this game tracks far fewer, but each distinction must earn its place.

Water is a utility, never cargo. It crosses the border only through a physical metered utility
connection.

## 1.0 requirement

`SPEC-RESOURCES-001` — the ratified catalogue SHALL contain exactly the charter's fifteen
domestic resources plus Medicine. Every identity SHALL have a declared unit and handling/storage
compatibility before a recipe, storage, or haul can use it.

`SPEC-RESOURCES-002` — stock is an owned physical quantity. Request, allocation, reservation,
pickup, custody, delivery, and consumption are distinct records.

`SPEC-RESOURCES-003` — Water MUST NOT enter cargo stock, vehicle custody, a freight station,
or a trade haul.

`SPEC-RESOURCES-005` — a quantity cannot be silently deleted or created during a failed
request, reservation, transfer, or consumption.

## Target design

The design proposes handling classes (HYPOTHESIS, bible §6.8): bulk, unit, liquid, heavy.
Each class constrains which vehicles and storage types can carry and hold a resource. This
requires the item metadata that the current substrate lacks (E-003).

## Current substrate

`base_mod/items.lua` declares 21 items. Each has `name`, `label`, and optionally
`optout_exttrade`. The only item with `optout_exttrade = true` is `job-opening`.

`prototypes/src/prototypes/item.rs`, `ItemPrototype`: fields are `base` (name, label),
`id: ItemID`, and `optout_exttrade: bool`. No mass, volume, unit, storage class, transport
class, or capacity metadata exists.

The 21 items are: job-opening, cereal, flour, bread, vegetable, carcass, raw-meat, meat,
tree-log, wood-plank, iron-ore, metal, gold, high-tech-product, furniture, flower, wool,
cloth, oil, coal, polyester.

The charter names fifteen domestic resources. The current 21 includes items not in the
charter scope (gold, high-tech-product, flower, wool, cloth, polyester) and one non-physical
item (job-opening). The catalogue must be reconciled.

Medicine is ABSENT: the charter requires it as a sixteenth, import-only resource
(`docs/plan/charter-1.0.md:44`), but no `medicine` entry exists in `base_mod/items.lua:1-108`
and the item prototype carries only name/label/id/`optout_exttrade`
(`prototypes/src/prototypes/item.rs:7-14`). Until it is declared, the 1.0 catalogue gap is
open — no recipe, storage, or haul can reference it.

## Open questions

- Which fifteen names satisfy the charter catalogue, and what are their units?
- Which handling classes apply to each resource?
- Which substitutions are legal for dwelling needs and production inputs?

## Related

- [Requests](requests.md)
- [Storage](storage.md)
- [Production](production.md)
- [Logistics](logistics.md)
- [Resources spec](../../reference/specifications/resources.md)
- [Glossary](../../reference/glossary.md)
