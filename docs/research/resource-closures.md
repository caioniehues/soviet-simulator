# Resource-tree closures from W&R chains

**Ticket:** [#83](https://github.com/caioniehues/soviet-simulator/issues/83) (part of map #81)
**Question:** What closed resource trees exist between our 3 resources and W&R's 45? Candidate closed sets at ~8, ~15, ~25 resources.
**Sources:** `$PRODUCTION`/`$CONSUMPTION` lines mined from all 1472 files in `~/.local/share/Steam/steamapps/common/SovietRepublic/media_soviet/buildings_types/*.ini` (W&R 1.0 install, read 2026-08-17); `spec/resources.md`; `spec/production.md`; `src/sim/resources.rs`; `src/sim/buildings.rs`.

**Definition of closed:** every input of every included resource's recipe is either producible domestically by an included building chain, or plausibly importable at the customs border (G1.2 `CustomsOffice`). Since customs makes *any* set trivially closed, the metric that matters is **domestic closure with a named, short import list** — imports are the pressure valve, not the plan.

---

## A. The complete W&R production graph (mined, CONFIRMED)

Every W&R building recipe, extracted verbatim from the inis (`awk` over `$PRODUCTION`/`$CONSUMPTION`; utility draws via `$CONSUMPTION_PER_SECOND`):

### Extraction — no material inputs (labour → resource)
| Building | Output |
|---|---|
| coal_mine | rawcoal |
| iron_mine | rawiron |
| gravelmine / gravel_mine_big | rawgravel |
| bauxite_mine | rawbauxite |
| oil_mine | oil |
| uranium_mine | uranium |
| farm (4 sizes) | plants |
| woodcutting_post | wood |
| water_well (2), water_to_water | water |
| powerplant_wind (2), powerplant_solar | eletric |

### Processing & manufacture (inputs → outputs)
| Building | Inputs | Outputs |
|---|---|---|
| coal_processing | rawcoal | coal |
| iron_processing | rawiron | iron |
| gravel_processing (2) | rawgravel | gravel |
| bauxite_processing | rawbauxite | bauxite |
| steel_mill (2) | coal + iron | steel |
| cement_plant (3) | coal + gravel | cement |
| concrete_plant (2) | gravel + cement + water | concrete |
| panels_factory (3) | cement + gravel | prefabpanels |
| brick_factory (2) | coal | bricks |
| sawmill | wood | boards |
| oil_rafinery (2) | oil | **fuel + bitumen** (co-products) |
| asphalt_plant | gravel + bitumen + eletric | asphalt |
| chemical_plant | gravel + wood + plants + oil + water | chemicals |
| plastics_factory | chemicals + oil | plastics |
| fabric_factory | plants + chemicals + water | fabric |
| clothing_factory | fabric | clothes |
| food_factory | plants + water | food |
| distillery | plants + water | alcohol |
| animal_farm | plants + water | livestock |
| slaughterhouse | livestock | meat |
| fertilizer | waste_bio + chemicals | fertiliser |
| fertilizer_liquid (2) | chemicals + water | fertiliser_liquid |
| explosive_factory | chemicals + gravel + wood | explosives |
| mechanical_components_factory | steel | mcomponents |
| eletronic_components_factory | plastics + steel + chemicals | ecomponents |
| eletronic_factory | ecomponents + plastics + mcomponents | eletronics |
| production_vehicle / _train (2) / drydock | steel + plastics + mcomponents + ecomponents + fabric + eletronics | vehicles |
| production_airplane | the above + **aluminium** | vehicles |
| alumina_plant | bauxite + coal + chemicals + water | alumina |
| aluminium_plant | alumina + chemicals | aluminium |
| uranium_processing → uranium_conversion → nuclear_fuel_plant | uranium → yellowcake (+chem) → uf6 (+chem) | nuclearfuel |
| powerplant_coal (2) / powerplant_gas | coal / oil | eletric |
| powerplant_nuclear (2) | nuclearfuel + water | eletric + nuclearfuelburned |
| heating_plant (2) | coal | heat |
| incinerator_heat / _powerplant | waste | heat / eletric |
| sewage/water_treatment (4) | usagewater + chemicals | water |
| waste\_\*recycling (4) | waste_{steel,plastic,gravel,aluminium} | steel / plastics / gravel / aluminium |
| waste_treatment_plant (2) | waste_toxic + chemicals | — |

### Structural facts that shape any closure
1. **Three universal hubs.** `eletric` is a `$CONSUMPTION_PER_SECOND` input to **38** building types; `water` is a material input to 12; **`chemicals` is the biggest material hub — 17 consumers**, feeding plastics, fabric, fertiliser, alumina, electronics components, nuclear conversion, and all water treatment. Chemicals is the keystone resource: below ~20 resources you cut every chain that needs it (or import it); at 25 it pays for itself because its own recipe (gravel+wood+plants+oil+water) is fully domestic once agriculture and oil exist.
2. **Raw→processed pairs are half of the small-set budget.** rawcoal→coal, rawiron→iron, rawgravel→gravel, rawbauxite→bauxite each spend two resource slots for one processing building. W&R keeps them for the dumper-traffic gameplay; at 8 and 15 we **collapse each pair to one resource** (mine outputs the processed form directly, as our Mine/Quarry already do) and can re-split later without touching downstream recipes.
3. **Chain closure costs, measured from the graph:**
   - **Construction materials** close almost free: coal + gravel (already ours) + wood cover cement, concrete, bricks, boards, prefabpanels — 5 new processing buildings, only one new extraction (wood).
   - **Food** closes with just plants + water: farm + food_factory. Meat adds livestock + slaughterhouse; both inputs stay domestic.
   - **Steel** needs exactly one new extraction (iron) + the mill; coal is already ours.
   - **Oil branch** is the widest single door: oil unlocks fuel, bitumen, asphalt, chemicals, plastics — 6 resources from one extraction, and chemicals then unlocks fabric/clothes/fertiliser.
   - **Electronics → vehicles** is the deepest cut: 4 more resources (mcomponents, ecomponents, eletronics, vehicles), 4+ factories, and it wants *everything* upstream (steel, plastics, fabric). W&R itself gates it behind educated labour (`$PROFESORS_NEEDED`). Endgame material.
   - **Aluminium** (3 resources, 3 buildings) feeds only airplanes — cheapest big-chain cut.
   - **Nuclear** (5 resources) and **sorted waste** (10 classes) are self-contained side systems, cuttable wholesale.
4. **"Goods" does not exist in W&R.** Shops demand concrete commodities (food, meat, clothes, eletronics, alcohol via `$STORAGE_DEMAND_BASIC/_ADVANCED`). Our `Goods` is a placeholder consumer basket; every growth step should either give it a real recipe or split it.

### Where we stand (`src/sim/resources.rs`, `src/sim/buildings.rs`)
- `ResourceKind`: **Coal, Gravel, Goods**. Electricity/heat/water are modelled as flows (wire/pipe modules), matching W&R's `eletric`/`heat` — they never spend a resource slot, in W&R or here. Not counted in set sizes below.
- Recipes today: Mine→Coal, Quarry→Gravel, PowerPlant burns Coal→power, HeatPlant burns Coal→heat, **Factory→Goods from no material input** (power+water+staff only — the fiat input the first growth step should close), construction consumes Gravel, Dwellings consume Goods, customs sells all three.

---

## B. Candidate closed sets

Naming: our ids; W&R equivalents in parentheses where they differ. All sets keep electricity/heat/water as flows (not counted). All sets collapse raw/processed ore pairs except where noted.

### Candidate S8 — "The Construction Republic" (8 resources)

> Coal, Gravel, Goods + **Wood, Boards, Cement, Concrete, Bricks**

| Resource | Recipe | Domestic? |
|---|---|---|
| Coal | mine (have) | yes |
| Gravel | quarry (have) | yes |
| Wood | woodcutting post (**new**) | yes |
| Boards | sawmill: wood (**new**) | yes |
| Cement | cement plant: coal + gravel (**new**) | yes |
| Concrete | concrete plant: gravel + cement + water (**new**) | yes |
| Bricks | brick factory: coal (**new**) | yes |
| Goods | factory: **boards → goods** (re-recipe, OURS — closes the fiat input) | yes |

- **Closure:** fully domestic; zero mandatory imports. Customs still useful for bootstrapping any of the five.
- **Chains implied:** construction materials only. No agriculture, no steel — Goods stays the abstract consumer basket.
- **Buildings to add (5):** WoodcuttingPost, Sawmill, CementPlant, ConcretePlant, BrickFactory (all reuse the existing recipe-building system; no new mechanics).
- **Why this 8:** it makes B6 construction consume *real* materials (W&R construction takes concrete/bricks/boards/prefabpanels/steel) — the single biggest depth-per-resource payoff, and every recipe is 1–2 inputs already in the graph.
- **Variant S8-agri:** swap Bricks+Boards for Crops+Food (farm + food factory) if "feed the people" beats "build from real matter" — but then construction stays fiat, and food competes with Goods for the same dwelling-need slot. Lean: construction first, agriculture at 15.

### Candidate S15 — "Steel and Bread" (15 resources)

> S8 + **Iron, Steel, Prefab panels, Crops (plants), Food, Livestock, Meat**

| New resource | Recipe | Domestic? |
|---|---|---|
| Iron | iron mine (**new**; raw/processed collapsed) | yes — or importable early |
| Steel | steel mill: coal + iron (**new**) | yes |
| Prefabpanels | panels factory: cement + gravel (**new**) | yes |
| Crops | farm (**new**) | yes |
| Food | food factory: crops + water (**new**) | yes |
| Livestock | animal farm: crops + water (**new**) | yes |
| Meat | slaughterhouse: livestock (**new**) | yes |

- **Closure:** fully domestic. Sensible import list while building out: iron (until the mine district exists), food (famine valve).
- **Chains implied:** construction (complete: concrete/bricks/boards/panels/steel is W&R's exact construction-material set), **steel**, **agriculture/food**. Goods can now mean "household goods" produced from steel+boards, while Food/Meat become their own dwelling needs (W&R shops sell both) — the first real split of the consumer basket.
- **Buildings to add (7 beyond S8, 12 total):** IronMine, SteelMill, PanelsFactory, Farm, FoodFactory, AnimalFarm, Slaughterhouse.
- **New mechanics implied:** perishables/refrigerated transport for meat (spec/resources.md shelfLife + `COOLER` class) — or defer by treating meat as covered cargo at first; field growth cycles for the farm (W&R `$TYPE_FIELD` batch harvest) — or defer with a continuous-rate farm.
- **This is the natural 1.0-shaped set:** every W&R construction input exists, both halves of "the people need feeding and housing" are real, and no hub resource (chemicals/oil) is needed anywhere.

### Candidate S25 — "The Petrochemical Republic" (25 resources)

> S15 + **Oil, Fuel, Bitumen, Asphalt, Chemicals, Plastics, Fabric, Clothes, Alcohol, Fertiliser**

| New resource | Recipe | Domestic? |
|---|---|---|
| Oil | oil rig (**new**) | yes — or the classic import |
| Fuel | refinery: oil (co-product) (**new**) | yes |
| Bitumen | refinery: oil (co-product) | yes |
| Asphalt | asphalt plant: gravel + bitumen (**new**) | yes |
| Chemicals | chemical plant: gravel + wood + crops + oil + water (**new**) | yes — every input already in S15+oil |
| Plastics | plastics factory: chemicals + oil (**new**) | yes |
| Fabric | fabric factory: crops + chemicals + water (**new**) | yes |
| Clothes | clothing factory: fabric (**new**) | yes |
| Alcohol | distillery: crops + water (**new**) | yes |
| Fertiliser | fertiliser plant: chemicals + water (**new**; W&R liquid variant — the solid one needs the waste chain) | yes |

- **Closure:** fully domestic given one new extraction (oil). Import valve: oil alone covers the whole branch (fuel/bitumen/chemicals importable individually too — all are border-tradeable in W&R).
- **Chains implied:** everything in S15 plus **petrochemical**, **consumer goods** (clothes/alcohol join food/meat — at this point `Goods` should dissolve into the W&R-style shop basket), **fuel economy** (vehicles burn fuel — our depot fuel-tank hook), **asphalt roads** (road classes get a material cost), **fertiliser → farm yield** (closes agriculture into a loop).
- **Buildings to add (9 beyond S15, 21 total):** OilRig, Refinery (first multi-output recipe), AsphaltPlant, ChemicalPlant, PlasticsFactory, FabricFactory, ClothingFactory, Distillery, FertiliserPlant.
- **New mechanics implied:** liquid transport class (tankers/pipelines — `OIL` class), co-product recipes (refinery), vehicle fuel consumption.
- **Rationale for the cut:** chemicals' recipe is the test — at S25 all five of its inputs are domestic, so the keystone hub closes and drags fabric/plastics/fertiliser in with it. This is the smallest set where the chemicals hub is worth owning rather than importing.

### Explicitly excluded even at 25 (and why)
| Excluded | Cost to include | Verdict |
|---|---|---|
| raw/processed ore split (4 pairs) | +4 resources, +4 processing buildings, doubles bulk traffic | re-split later per-chain if dumper logistics wants it |
| aluminium chain (bauxite, alumina, aluminium) | +3 resources, +3 buildings, W&R's hungriest power draw | feeds only airplane production — cut until aviation |
| electronics → vehicles (mcomp, ecomp, eletronics, vehicles) | +4 resources, +5 factories, needs educated labour tier | the true endgame chain; needs S25 as its supply base |
| nuclear (uranium, yellowcake, uf6, nuclearfuel, spent) | +5 resources | self-contained prestige system, post-1.0 |
| sorted waste (10 classes) | +10 resources + separation/recycling plants | its own subsystem (spec/waste.md), orthogonal to the ladder |
| explosives, fertiliser-solid, usagewater-as-commodity | niche consumers | fold into the systems that need them when those arrive |

---

## C. Answers for the map (#81)

1. **The three sizes are real closure frontiers, not arbitrary counts:** 8 closes construction, 15 closes construction+steel+food with zero hub resources, 25 is the smallest set where the chemicals hub closes domestically. Between 15 and 25 there is no stable resting point — adding oil without chemicals wastes the door it opens.
2. **Resource growth is chain-shaped, not list-shaped.** Each step is "adopt a chain whole": 5 buildings (S8), +7 (S15), +9 (S25). The recipe engine already supports all of them; only S15 (perishables, field cycles) and S25 (liquids, co-products, fuel burn) imply new *mechanics*, each deferrable.
3. **Feeds "Agriculture + food":** agriculture enters at S15 as crops→food and crops→livestock→meat (2 farms + 2 processors), and closes into a loop at S25 via fertiliser. It is never required by construction/steel — the two ladders braid but don't block each other.
4. **Feeds "Resource tree depth":** the factory's fiat Goods input is the first thing any growth step should close (S8 does it with boards→goods); the customs import list is the tuning knob that lets each chain be adopted incrementally without breaking closure.

## Evidence log
| Claim | Level | Source |
|---|---|---|
| Full W&R recipe table (§A) | CONFIRMED | awk over `$PRODUCTION`/`$CONSUMPTION` in all 1472 `buildings_types/*.ini`, W&R install |
| eletric consumed by 38 building types | CONFIRMED | `$CONSUMPTION_PER_SECOND` grep, same corpus |
| chemicals = 17 consumers, biggest material hub | CONFIRMED | `$CONSUMPTION` tally, same corpus |
| Shops demand concrete commodities, no "goods" resource | CONFIRMED | `$STORAGE_DEMAND_*` in shop/kiosk/hotel inis |
| Our current set + fiat Factory recipe | CONFIRMED | `src/sim/resources.rs`, `src/sim/buildings.rs` |
| Collapse of raw/processed pairs; set compositions S8/S15/S25 | OURS | curation over the confirmed graph |

## Related
- spec/resources.md · spec/production.md · spec/trade.md · docs/wayfinder-brief.md · issues #81, #83
