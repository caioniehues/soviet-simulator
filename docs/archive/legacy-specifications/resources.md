# Resource Ontology

> Superseded by ../../reference/specifications/resources.md — provenance only.

**Status:** draft model (grounded in research)
**Phase:** 1
**Primary inspiration:** W&R physical typed commodities, generalised + deepened (OURS)
**Evidence:** see [research/resources.md](../research/resources.md) for W&R-data and CS1-code sources.

> Every resource is a **physical typed commodity**: it has mass and volume, occupies a specific kind of storage, moves only on compatible transport, and is produced/consumed by recipes. There is no abstract "money buys it" resource. This is the vocabulary the whole physical economy (`production`, `logistics`, `construction`, `trade`) is written against.

## Purpose

Define the closed set of resources and the metadata each carries, so that:
- every `production` recipe references resources that exist here,
- `logistics` can decide which vehicle/container/network can carry a given resource,
- `construction` can consume real materials, and
- storage buildings can accept only resources of a compatible class.

The ontology is the **noun list** of the economy; production/logistics/construction are the **verbs**.

## The two source models

Grounded in the two research labs (to be confirmed by `research/resources.md`):

- **W&R** — a resource is a *physical commodity* with a storage class (aggregate / covered / liquid / cooled / open / special) and a compatible transport class (dumper, tanker, covered truck, refrigerated, conveyor, pipe, rail wagon, ship). This is our primary model.
- **CS1** — a resource is a `TransferManager.TransferReason` token (a reason a vehicle makes a trip); industry goods are classed raw / processed / luxury. Coarser than W&R; we take the *trip-generating* idea, not the abstraction.

### The confirmed resource sets (from `research/resources.md`)

**W&R — ~60 movable commodities** (CONFIRMED from `resources/` icons; grouping INFERRED):

- **Raw / extracted:** rawcoal, rawiron, rawbauxite, rawgravel, oil, uranium, plants, livestock, wood, water
- **Processed ores & metals:** coal, iron, bauxite, gravel, alumina, aluminium, steel
- **Construction:** cement, concrete, asphalt, bitumen, bricks, boards, prefabpanels
- **Chemicals & liquids:** chemicals, plastics, fuel, fertiliser, fertiliser_liquid, explosives
- **Consumer goods:** food, meat, clothes, fabric, alcohol
- **Electronics & components:** eletronics, ecomponents (electronic), mcomponents (mechanical)
- **Nuclear chain:** uranium → yellowcake → uf6 → nuclearfuel → nuclearfuelburned (spent)
- **Vehicles (finished machines, themselves resources):** vehicles, airplanes, helicopters, ships, trains
- **Energy & utility flows:** eletric (electricity), heat, water, usagewater (grey/waste water)
- **Waste (10 sorted classes):** waste_aluminium, waste_ash, waste_bio, waste_burnable, waste_gravel, waste_mixed, waste_other, waste_plastic, waste_steel, waste_toxic
- **Labour / abstract:** workers (labour as a transferable), service_material (generic shop supply)

**CS1 — cargo subset of `TransferManager.TransferReason`** (CONFIRMED, one flat enum that also mixes in citizen-dispatch reasons):
Oil, Ore, Logs, Grain, Coal (raw) · AnimalProducts, Flours, Paper, PlanedTimber, Petroleum, Plastics, Glass, Metals, Fish (processed) · Goods, Food, Petrol, Lumber, LuxuryProducts (finished) · Mail chain · Garbage/Snow. CS1's ground deposits (`NaturalResourceManager.Resource`) and service coverage (`ImmaterialResourceManager.Resource`) are **separate** enums — the latter is the coverage-scalar model we explicitly reject for physical goods.

> **Key finding:** the two games are complementary, not redundant. W&R has the *physical rigour* (typed transport class, container, connection medium) that CS1 lacks; CS1 has the *tier semantics* (raw → processed×2 → luxury value ladder) that W&R leaves implicit. **We take both.**

## Resource metadata (the ontology fields)

Each resource carries:

```
Resource {
  id                 // stable identifier, e.g. "steel", "iron_ore", "fuel"
  displayName
  category           // raw / processed-material / construction / consumer-good / liquid / energy / waste  (see below)
  tier               // raw | intermediate | finished | luxury  — economic ladder (OURS, from CS1 semantics)

  // physical
  mass               // per unit (t)
  volume             // per unit (m³) — drives truck/wagon fill
  bulkDensity        // INFERRED from mass/volume where W&R doesn't state it

  // handling classes — determine compatible storage & transport
  storageClass       // aggregate | covered | liquid | cooled | open | special(grid/pipe)   <- from W&R tokens
  transportClass     // set of compatible vehicle/network kinds                              <- from W&R tokens
  containerClass?    // typed container where applicable (steel/plastic/bio/toxic/aluminium/open)

  // lifecycle
  shelfLife?         // for perishables (food, meat) — OURS unless W&R states it
  hazardClass?       // toxic / radioactive (uranium, nuclear fuel, some waste)

  // economy
  recipes[]          // production recipes that OUTPUT this (defined in spec/production.md)
  tradeable          // can be imported/exported (spec/trade.md)
}
```

### How W&R actually encodes this (CONFIRMED tokens, from `research/resources.md §B`)

W&R has **no global resource enum** — a building names a resource by bare lowercase string in its recipe (`$PRODUCTION steel 0.086`), and physical handling is declared by a **transport class** on each storage bucket. Our `storageClass`/`transportClass` fields generalise these real tokens:

- **`transportClass` ← `RESOURCE_TRANSPORT_*`** (16 confirmed classes): `GRAVEL` (loose bulk: ore/coal/gravel), `OIL` (liquid fuels), `CEMENT` (powder tanker), `COOLER` (refrigerated), `COVERED` (dry boxed goods), `OPEN` (flatbed: steel/boards/bricks), `WATER`, `SEWAGE`, `LIVESTOCK`, `VEHICLES`, `ELETRIC`, `NUCLEAR`, `CONCRETE` (mixer), `WASTE`, `GENERAL`, `PASSANGER` (people — separate from cargo, unlike CS1).
- **`storageClass` / buckets ← `$STORAGE*`**: `$STORAGE <class> <cap>`, `$STORAGE_IMPORT`/`_EXPORT`, `$STORAGE_IMPORT_SPECIAL <class> <cap> <resource>` (a bucket pinned to one named resource), `$STORAGE_DEMAND_BASIC/_ADVANCED/_HOTEL/_PRISON` (consumer-demand buffers in shops).
- **connection medium ← `$CONNECTION_*`**: conveyor (bulk solids), pipe/waterpipe/sewage/steam (liquids/gas), bulk chute, road/rail/water/air/cableway, eletric low/high, heating big/small, pedestrian. → this maps to our `transportClass` set membership for network-borne resources.
- **`containerClass` ← typed container models**: `container_big_{aluminium,bio,construction,plastic,steel,toxic}` + small variants — a container's material encodes what it may legally carry.
- **recipe grammar ← `$PRODUCTION` / `$CONSUMPTION`** (per tick), `$CONSUMPTION_PER_SECOND` (continuous, used for `eletric`), `$CONSUMPTION_WATER_REQUIRED_QUALITY <0..1>` (input purity gate), `$PRODUCTION_SEWAGE_POLLUTION` (byproduct), `$WORKERS_NEEDED` (labour input). Detailed in `spec/production.md`.

Caveat: the *grouping* and each transport class's exact *meaning* are INFERRED from which goods use each token; the token strings themselves are CONFIRMED. W&R exposes no central resource→class manifest, and the numeric ordering of `RESOURCE_TRANSPORT_*` lives in the native binary (not recoverable from `.ini`).

## Categories (grouping)

Membership CONFIRMED from W&R asset names; category boundaries and typical-transport mappings INFERRED (see caveat above):

| Category | Members (W&R) | Typical `RESOURCE_TRANSPORT_*` |
|---|---|---|
| Raw / extracted | rawcoal, rawiron, rawbauxite, rawgravel, oil, uranium, plants, livestock, wood, water | GRAVEL, OIL, LIVESTOCK, WATER |
| Processed metal | coal, iron, bauxite, gravel, alumina, aluminium, steel | GRAVEL, OPEN |
| Construction | cement, concrete, asphalt, bitumen, bricks, boards, prefabpanels | CEMENT, CONCRETE, OPEN |
| Chemicals & liquids | chemicals, plastics, fuel, fertiliser(+_liquid), explosives | OIL, COVERED |
| Consumer good | food, meat, clothes, fabric, alcohol | COVERED, COOLER |
| Electronics | eletronics, ecomponents, mcomponents | COVERED |
| Nuclear chain | uranium, yellowcake, uf6, nuclearfuel, nuclearfuelburned | NUCLEAR |
| Vehicles (finished) | vehicles, airplanes, helicopters, ships, trains | VEHICLES |
| Energy / flows | eletric, heat, water, usagewater | ELETRIC, (heat pipe), WATER, SEWAGE — network-borne |
| Waste (10 classes) | waste_{aluminium,ash,bio,burnable,gravel,mixed,other,plastic,steel,toxic} | WASTE |

## The value/tier ladder (OURS — combining both games)

W&R encodes *physical* handling but leaves economic tier implicit; CS1 encodes an explicit **raw → processed → luxury** worth ladder (`IndustryBuildingAI.cs`: Logs 200 / Ore 300 / Oil 400 → processed ~1500–3000 → LuxuryProducts 10000 — CONFIRMED). We overlay CS1's *tier semantics* onto W&R's *physical set*: each resource additionally carries a `tier` (raw / intermediate / finished / luxury). This drives planning priority and trade value without reintroducing money as a build trigger.

```
tier ∈ { raw, intermediate, finished, luxury }   // OURS — economic classification over the physical set
```

## Storage & transport compatibility (the point of the classes)

The ontology exists so this rule is enforceable:

> A resource can only sit in a storage of a compatible `storageClass`, and only move on a `transportClass` it belongs to.

Consequences that make it matter (mirrors `needs.md`'s causal style):
- Refrigerated goods (meat) rot without a cooled store / refrigerated transport → `shelfLife` expires → food need unmet.
- Liquids need tankers or pipelines; a covered truck cannot carry fuel.
- Energy (electricity, heat) never rides a vehicle — it flows on its own network, so it's modelled but excluded from the logistics vehicle scheduler.

## Open questions
- **Granularity ceiling.** Transcript wants "deeper than W&R, curated." Where do we stop splitting? (e.g. do we model machine tools / spare parts / intermediate chemicals as distinct resources, or fold them?) Decide per-chain, not globally.
- **Units.** W&R stores many bulk goods in tonnes and some in m³/units. Do we normalise everything to a single unit or keep per-resource units? Lean: keep native units, carry mass+volume so logistics can convert.
- **Electricity/heat as "resources."** They share the recipe grammar (inputs→output) but not the transport grammar. Model as resources with `storageClass = special` and no vehicle transport? (Provisional: yes.)
- **Waste as first-class resource** (so it must physically go somewhere) vs a sink. Lean first-class, matching W&R's sorted-waste chain.
- **Containers.** Are typed containers themselves resources (they are assets in W&R) or a property of transport? Provisional: property of transport, not consumed resources.

## Data (draft)
See the `Resource {}` block above; final field list frozen once `research/resources.md` confirms which metadata W&R actually encodes vs what is OURS.

## Evidence log
| Claim | Evidence level | Source | Notes |
|---|---|---|---|
| W&R names resources by bare string in recipes; handling set by a transport class per storage bucket | CONFIRMED | W&R `buildings_types/*.ini` (488 files) | research/resources.md §B |
| 16 `RESOURCE_TRANSPORT_*` classes (GRAVEL/OIL/CEMENT/COOLER/COVERED/OPEN/…) | CONFIRMED | W&R `buildings_types/*.ini` | class *meaning* is INFERRED; token strings CONFIRMED — §B1 |
| ~50–60 movable commodities exist in W&R | CONFIRMED | W&R `media_soviet/resources/` (63 icons) | full list research/resources.md §A |
| CS1 represents movable goods as `TransferManager.TransferReason` tokens | CONFIRMED | CS1 `TransferManager.cs` | cargo subset in research/resources.md §D1 |
| CS1 raw→processed×2→luxury worth ladder (Logs 200 … LuxuryProducts 10000) | CONFIRMED | CS1 `IndustryBuildingAI.cs` ~L1520 | source of our `tier` field — §D2 |
| Storage/transport containers are themselves typed (steel/plastic/bio/toxic/aluminium/open) | CONFIRMED | W&R `resources/container_big_*` | research/resources.md §B4 |
| Metadata field set (mass/volume/storageClass/transportClass/shelfLife/hazard/recipes) | OURS | — | generalisation over both games |
| `tier` (raw/intermediate/finished/luxury) overlaid on the physical set | OURS | — | CS1 semantics over W&R physical rigour |
| Electricity & heat modelled as network-only resources | OURS | — | share recipe grammar, not transport grammar |
| CS1 `ImmaterialResource` coverage-scalar model rejected for physical goods | OURS | CS1 `ImmaterialResourceManager.cs` | scalars reserved for intangibles (loyalty/wellbeing) |

## Related
- ../research/resources.md · ../spec/production.md · ../spec/logistics.md · ../spec/construction.md · ../spec/trade.md · ../spec/needs.md
