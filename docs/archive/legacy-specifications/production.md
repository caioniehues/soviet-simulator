# Production

> Superseded by ../../reference/specifications/production.md — provenance only.

**Status:** draft model (grounded in research)
**Phase:** 1
**Primary inspiration:** W&R transformation process, deepened; CS1 worker-scaling for comparison (OURS synthesis)
**Evidence:** see [research/production.md](../research/production.md); resource vocabulary in [spec/resources.md](resources.md).

> A factory is a **transformation process**, not a money printer. Given inputs + power + water + workers + machinery + time it yields outputs + byproducts. Any missing factor throttles or stops the process — and that failure is legible and cascades (`spec/needs.md`'s core loop). Nothing is produced merely because the treasury can afford it.

## Purpose

Define the model by which a building consumes resources (from `spec/resources.md`) and labour and produces outputs, and — crucially — **how each production factor gates the rate**. This is the verb layer over the resource nouns: every `$CONSUMPTION`/`$PRODUCTION` recipe references resources defined in the ontology.

## The transformation model

```
output_rate = base_rate
            × f_labour(workers_present / workers_needed, skill_mix)
            × f_power(electricity_available)
            × f_inputs(min over inputs of available / required)      // Liebig's-law bottleneck
            × f_machinery(equipment_present, condition)
            × f_water_quality(quality ≥ required ? 1 : penalty)
            × f_output_space(output_storage_not_full ? 1 : 0)
```

The multiplicative form is the point: the **scarcest factor wins** (a Liebig's-law bottleneck). Full staffing + full power + no iron = no steel. This is where W&R's causality lives and where CS1's "money → goods" model is rejected.

### What the research confirmed about each factor (`research/production.md`)

The multiplicative/bottleneck form is **validated by CS1 source**: `ProcessingFacilityAI.ProduceGoods` throttles `finalProductionRate` down to `stock·100/inputRate` for *each* input independently — the scarcest ingredient caps the rate, exactly the `min()` above (CONFIRMED, `ProcessingFacilityAI.cs:471`). Per-factor behaviour we adopt:

- **`f_labour` — scales with staffing fraction × worker quality.** W&R: the `$PRODUCTION` number is documented **per-worker** (`water_well_*.ini`: `//production for water well is not per worker, but whole` marks the *exception*), so output ∝ workers present / `$WORKERS_NEEDED` (INFERRED linear). CS1 makes the curve explicit: `rate = 2·e − 200·e/(staffFrac+100)` where `e` = average per-worker health/education efficiency (CONFIRMED, `PrivateBuildingAI.cs:397`). **We take CS1's curve** as the concrete law W&R only gestures at.
- **`f_power` — a gating input, not a bill.** This is the headline W&R↔CS1 difference: W&R makes power a `$CONSUMPTION_PER_SECOND eletric` **input that gates output**; CS1 computes power as an **output** of the rate and bills it after (`GetConsumptionRates`). **We keep W&R's direction** — no power ⇒ no output, regardless of budget. W&R also carries idle-vs-producing draw fallbacks (`$ELETRIC_WITHOUT_WORKING_FACTOR`, modal 0.4).
- **`f_inputs` — Liebig bottleneck, CONFIRMED.** Each input throttles rate to its own stock; any input at zero ⇒ stop.
- **`f_output_space` — CONFIRMED backpressure.** Output buffer full ⇒ production halts (`ProcessingFacilityAI.cs:523`, raises `NoPlaceforGoods`). This is our cascade engine.
- **No `f_funding`.** CS1 multiplies output by a funding slider (`rate × budget/100`); W&R has **no money gate on output at all** (CONFIRMED absent). **We drop the funding knob entirely** — it violates the project's one rule.
- **Time-of-day / renewables:** CS1 halves output at night; W&R modulates renewables by `$PRODUCTION_CONNECT_TO_WIND/SUN`. Both are OURS-optional.

## Confirmed recipe grammar (W&R, from `spec/resources.md §B` + `research/production.md`)

```
$PRODUCTION <resource> <rate>              // output per production tick
$CONSUMPTION <resource> <rate>             // input per tick
$CONSUMPTION_PER_SECOND <resource> <rate>  // continuous draw (electricity)
$CONSUMPTION_WATER_REQUIRED_QUALITY <0..1> // input purity gate
$PRODUCTION_SEWAGE_POLLUTION <rate>        // byproduct emission
$WORKERS_NEEDED <n>                        // labour input
$PROFESORS_NEEDED <n>                      // skilled/educated labour (education buildings)
```

Deeper W&R production tokens the research confirmed (`research/production.md §A–C`):

```
$WORKERS_NEEDED <n>                      // 160 files — per-worker denominator (rate is per-worker)
$PROFESORS_NEEDED <n>                    // 34 files — university-educated staff (high-tech)
$WORKING_VEHICLES_NEEDED <n>             // 66 files — on-site equipment to operate (→ machinery factor)
$ELETRIC_WITHOUT_WORKING_FACTOR <0..1>   // idle power draw (modal 0.4); producing draw is higher
$ELETRIC_WITHOUT_LIGHTING_FACTOR <0..1>  // power fraction with lighting off
$PRODUCTION_CONNECT_TO_WIND / _TO_SUN    // renewable output modulated by weather/day-cycle
$PRODUCTION_DECREASE_ACCORDING_YEAR      // output drifts over the calendar (depletion/ageing)
$POLLUTION_SMALL | _MEDIUM | _HIGH       // air pollution as a NAMED TIER, not a number
$WASTE_PRODUCTION_ASH                    // combustion → waste_ash byproduct
$WASTE_EXTRACTION <wasteClass> <yield>   // recycling recovers a fraction, e.g. waste_steel 0.98
$TYPE_PRODUCTION_LINE                     // final-assembly plants (most inputs)
```

**Notable confirmed recipe shapes** (verbatim quotes in `research/production.md §D`):
- **Extraction, no inputs** — `iron_mine.ini`: `labour → rawiron 4` + `$POLLUTION_MEDIUM`. Mines/fields have *no* `$CONSUMPTION`.
- **Co-products** — `oil_rafinery.ini`: one `oil` input → **two** outputs (`fuel` + `bitumen`), each with its own pinned export bucket. Recipes can be many-to-many.
- **Power-intensive** — `aluminium_plant.ini`: `$CONSUMPTION_PER_SECOND eletric 2.35` (~12× the steel mill) — smelting is the game's hungriest step.
- **Most inputs** — `production_vehicle.ini`: `steel+plastics+mcomponents+ecomponents+fabric+eletronics + 400 workers + 100 professors → vehicles` (1:1:1:1:1:1→1, requires educated staff).

## Factor gates (each is a way production can fail)

Mirrors `needs.md`'s consequence-coupling style — every factor is a distinct, inspectable failure mode:

| Factor | Source of the number | Failure behaviour | Evidence |
|---|---|---|---|
| **Labour** | `$WORKERS_NEEDED` (per-worker) + skill mix | staffing fraction scales rate; zero workers → stop | CS1 curve CONFIRMED; W&R per-worker INFERRED |
| **Power** | `$CONSUMPTION_PER_SECOND eletric` | **gating input** — no power ⇒ no output (idle-draw fallbacks exist) | W&R CONFIRMED (as input); we keep this direction |
| **Inputs** | each `$CONSUMPTION <resource>` | each input independently throttles rate to its stock; any at zero → stop | CS1 CONFIRMED (Liebig); W&R INFERRED |
| **Water quality** | `$CONSUMPTION_WATER_REQUIRED_QUALITY` | below threshold → recipe blocked | W&R CONFIRMED |
| **Machinery** | `$WORKING_VEHICLES_NEEDED` (+ OURS wear) | missing equipment → reduced/zero rate | W&R token CONFIRMED; wear is OURS |
| **Output space** | output `$STORAGE_EXPORT` bucket full | backpressure → production halts until freight clears | CS1 CONFIRMED (`NoPlaceforGoods`); W&R INFERRED |

The **output-space gate is the cascade engine**: rail jammed → export bucket fills → mill stops → downstream construction starves. This is `needs.md`'s coal-train cascade, made mechanical — and CS1's code confirms the mechanism (rate clamped to remaining output space, `NoPlaceforGoods` problem raised). Our `bottleneck` field is the direct analogue of CS1's `NoResources`/`NoInputProducts`/`NoPlaceforGoods` problem flags, surfaced to the player.

**One factor we deliberately omit: funding.** CS1 multiplies output by a budget slider; W&R has no such gate. We follow W&R — physical supply is the only throttle.

## Byproducts (production isn't clean) — CONFIRMED
Three byproduct channels, all confirmed in W&R data:
- **Sewage** — `$PRODUCTION_SEWAGE_POLLUTION <rate>` (numeric); routed via `$CONNECTION_SEWAGE_OUTPUT`. `$WATER_NOT_PRODUCE_SEWAGE_FROM_PRODUCTION` opts out.
- **Air/ground pollution** — a **named tier** `$POLLUTION_SMALL|_MEDIUM|_HIGH` (not a number; radius/decay is native). Ties to `$ATTRACTIVE_FACTOR_POLLUTION` → lowers nearby liveability (`needs.md` §B2).
- **Solid waste** — `$WASTE_PRODUCTION_ASH` (combustion → `waste_ash`); and the reverse, `$WASTE_EXTRACTION <class> <yield>`, where recycling plants recover a fraction (e.g. `waste_steel 0.98`, `waste_plastic 0.9`). Byproducts are first-class outputs from `spec/resources.md` and must physically go somewhere — the waste chain is not free disposal.

## Open questions

**Resolved by research** (moved to draft model above):
- ~~Rate scaling shape~~ → labour scales with staffing fraction × worker quality (CS1 curve, adopted); inputs/output-space throttle to stock (CS1 CONFIRMED); power is a gating input (W&R). Funding dropped.
- ~~Does partial staffing scale output~~ → yes (W&R per-worker + CS1 curve).

**Still open:**
- **Tick vs continuous / time base.** `$PRODUCTION` is per production-step, `$CONSUMPTION_PER_SECOND` continuous. **W&R's tick length is native-only — not in the data** (confirmed gap). We must define our own canonical sim-time unit (`architecture/simulation-clock.md`) and re-derive rates against it, since W&R's raw numbers aren't time-anchored.
- **Skill/education depth.** Both games agree on ~2 tiers (`$WORKERS_NEEDED`/`$PROFESORS_NEEDED` ≈ CS1's edu levels). Do we go deeper (Dwarf-Fortress skills per citizen) or hold at 2 tiers? Lean: 2 tiers now, per-citizen skill later. Should under-qualification **throttle** output (OURS) or merely warn (CS1 base)? Lean throttle.
- **Machinery/equipment wear.** `$WORKING_VEHICLES_NEEDED` is confirmed; do we add equipment **condition/wear** as a rate factor (OURS, deeper than W&R)? Lean yes for the depth goal — but it's the biggest new-scope item; defer to a later pass.
- **Continuous vs batch processes.** Smelting is continuous; a harvest is batch (W&R `$TYPE_FIELD` grows on the tile with no `$PRODUCTION` line). One model with rate=0 between batches, or two? Lean: one continuous model; fields are a special extraction type.
- **Renewable/depletion modifiers.** Adopt `$PRODUCTION_CONNECT_TO_WIND/SUN` and `$PRODUCTION_DECREASE_ACCORDING_YEAR` (depletion)? Both fit the physical-causality theme.

## Data (draft)
```
Recipe {
  inputs: [{ resourceId, ratePerStep }]          // from $CONSUMPTION (per production step)
  continuousInputs: [{ resourceId, ratePerSec }] // from $CONSUMPTION_PER_SECOND (power/heat)
  outputs: [{ resourceId, ratePerStep }]         // from $PRODUCTION — MULTIPLE allowed (co-products, e.g. fuel+bitumen)
  byproducts: [{ resourceId, ratePerStep }]      // sewage / ash / waste
  pollutionTier?    // small | medium | high   (categorical, from $POLLUTION_*)
  workersNeeded, professorsNeeded                // two labour tiers
  workingVehiclesNeeded?                         // $WORKING_VEHICLES_NEEDED — machinery factor
  waterQualityMin?                               // from $CONSUMPTION_WATER_REQUIRED_QUALITY
  // NB: rates are re-derived against our sim clock; W&R's raw numbers are not time-anchored
}
ProductionState {
  recipe
  currentRate0to1   // = min over active factor gates (Liebig) × labour curve × power gate
  bottleneck        // which factor limits NOW → NoResources/NoInputProducts/NoPlaceforGoods (CS1) → player UI
}
```
Recipes are **many-to-many** (multiple inputs, multiple outputs) — confirmed by the oil refinery's fuel+bitumen co-products and the vehicle plant's 6 inputs. Extraction buildings (mines/fields) have **no inputs** — `labour → output`.
`bottleneck` is deliberately surfaced so the player can *see* why a factory is slow — the emotional core of the design.

Production runs at **medium** frequency (see `architecture/simulation-clock.md`); factor re-evaluation on input/power/staffing change.

## Evidence log
| Claim | Evidence level | Source | Notes |
|---|---|---|---|
| Recipe grammar `$PRODUCTION`/`$CONSUMPTION`/`$CONSUMPTION_PER_SECOND`/`$WORKERS_NEEDED` | CONFIRMED | W&R `buildings_types/*.ini` | research/production.md §A |
| W&R `$PRODUCTION` rate is **per-worker** (water wells flagged as the exception) | CONFIRMED (comment) / INFERRED (mechanism) | W&R `water_well_*.ini` | research/production.md §A2 |
| W&R has **no funding/budget gate** on output | CONFIRMED (absent) | W&R `*.ini` grep | research/production.md §F |
| CS1 worker curve `rate = 2·e − 200·e/(staffFrac+100)`, e = health/edu efficiency | CONFIRMED | CS1 `PrivateBuildingAI.cs:397` | research/production.md §E1 — adopted |
| Each input independently throttles rate to its stock (Liebig) | CONFIRMED | CS1 `ProcessingFacilityAI.cs:471` | validates our `min()` formula |
| Output-storage-full halts production (`NoPlaceforGoods`) | CONFIRMED (CS1) / INFERRED (W&R) | CS1 `ProcessingFacilityAI.cs:523`; W&R bucket structure | the cascade engine |
| W&R power is a **gating input**; CS1 power is an **output** of rate | CONFIRMED | W&R `$CONSUMPTION_PER_SECOND eletric`; CS1 `GetConsumptionRates` | research/production.md §F — we keep W&R's direction |
| Water-quality gate (`$CONSUMPTION_WATER_REQUIRED_QUALITY`) | CONFIRMED | W&R `chemical_plant.ini`, `food_factory.ini` | research/resources.md §C |
| Byproducts: sewage (numeric), pollution (tier), ash; recycling recovery yields | CONFIRMED | W&R `*.ini` | research/production.md §C |
| Recipes are many-to-many (co-products); extraction has no inputs | CONFIRMED | W&R `oil_rafinery.ini`, `iron_mine.ini` | research/production.md §D2, §D5 |
| Rate = labour-curve × min(input gates, output-space) × power gate (Liebig synthesis) | OURS | — | combines CS1 math + W&R power-gating |
| Drop CS1's funding multiplier; surface `bottleneck` to player | OURS | — | project's one rule + legibility goal |
| W&R rate **time base is native-only** (not time-anchored in data) | CONFIRMED (gap) | research/production.md Gaps | must re-derive against our sim clock |

## Related
- ../research/production.md · ../spec/resources.md · ../spec/logistics.md · ../spec/construction.md · ../spec/needs.md · ../spec/education.md · ../architecture/simulation-clock.md
