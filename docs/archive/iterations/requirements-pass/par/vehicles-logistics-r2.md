## F1 — `$LIFESPAN` vehicle-retirement token has no AC or scenario
- classification: omission
- source: spec/vehicles.md:26
- verbatim: "$LIFESPAN 35                        // ONLY on the 12 trolleybuses — the fleet's lone longevity token"
- what should have covered it: no story / no scenario
- why it matters: the one authored aging/retirement token in the source data is never turned into a requirement — vehicle end-of-life (age-based retirement, distinct from wear-based scrapping) is unaddressed.

## F2 — Scrapyard end-of-life conversion (vehicle → waste_steel + waste_aluminium) has no AC
- classification: omission
- source: spec/vehicles.md:75
- verbatim: "scrapyard: vehicle in → waste_steel + waste_aluminium out"
- what should have covered it: "Manufacture vehicles as a real production chain" covers acquisition only, not the corresponding disposal leg / no scenario
- why it matters: the spec states a full "cradle-to-grave" (line 76) lifecycle; the extraction's stories stop at manufacture/import and never test the grave end — a vehicle that reaches zero condition has no specified fate.

## F3 — Driver-binding open question resolved with a lean, but the lean itself is never turned into an AC
- classification: omission
- source: spec/vehicles.md:95
- verbatim: "Lean: bind for private cars + key services, abstract for bulk freight pools initially."
- what should have covered it: "Model the vehicle as an owned physical asset" AC-4 only asserts a driver reference field exists, not the differentiated binding rule / no scenario
- why it matters: AC-4 treats "driver reference" as a uniform field, but the spec's own resolved lean is that binding is conditional (private/key-service vehicles bind a citizen-driver, bulk freight pools do not) — the differentiation is a distinct, testable behavior that's dropped.

## F4 — Passengers-as-cargo-class (shared capacity/class model with freight) has no AC or scenario
- classification: omission
- source: spec/vehicles.md:36
- verbatim: "Passengers are literally a cargo class (`RESOURCE_TRANSPORT_PASSANGER`, bus capacity 138 persons) — people ride the same capacity/class model as freight."
- what should have covered it: no story / no scenario (all extracted stories/scenarios are framed purely around freight resources)
- why it matters: this is an explicit design confirmation that passenger transport reuses the same capacity/cargoClass gating as freight vehicles (F-covered by "Gate cargo assignment by resource transport-class compatibility") — but no AC or scenario ever exercises a passenger vehicle or a person as a transported unit, so the reuse claim is untested.

## F5 — Locomotive/wagon split (tractive vs cargo assets, bought as consists) has no AC
- classification: omission
- source: spec/vehicles.md:37
- verbatim: "Trains split tractive vs cargo assets: locomotive = power, no cargo; wagon = capacity, no propulsion (`$PURCHASE_EXCLUDE` — bought as consists)."
- what should have covered it: no story / no scenario
- why it matters: this is a distinct vehicle-composition rule (a "vehicle" for rail is actually two coupled asset types with different capability profiles) that none of the extracted Vehicle-as-asset ACs account for — the single-entity `Vehicle {}` schema in the extraction implicitly assumes one vehicle = one capacity + one propulsion.

## F6 — Currency-bloc split (RUB vs USD) on vehicle price has no AC
- classification: omission
- source: spec/vehicles.md:65
- verbatim: "`$COST_RUB` is the placeholder `4300` in 409/454 files (a truck and an ocean tanker share it) — real prices are native. The meaningful data is the **currency split** (bloc) and `$AVAILABLE` window."
- what should have covered it: "Manufacture vehicles as a real production chain" AC-2 covers price-from-BOM but not the bloc/currency distinction / no scenario
- why it matters: the spec explicitly flags currency-bloc (Eastern vs Western) as "the meaningful data," directly tying vehicles.md to trade.md's bloc mechanic, yet no AC tests that import price/currency differs by vehicle origin bloc.

## F7 — `$AVAILABLE` historical production window (era-gating which vehicles can be bought) has no AC
- classification: omission
- source: spec/vehicles.md:25
- verbatim: "$AVAILABLE <from> <to>              // historical production window, e.g. 1976 2001"
- what should have covered it: no story / no scenario
- why it matters: this is a concrete, named field the spec calls out as confirmed data (era arc gating), but nothing in the extraction tests that a vehicle outside its available window cannot be imported/manufactured — an observable acceptance boundary is entirely unaddressed.

## F8 — Fuel granularity / propulsion-typing open questions have no AC even as a "lean"
- classification: omission
- source: spec/vehicles.md:99-100
- verbatim: "**Fuel granularity.** One `fuel` resource, or petrol/diesel split by era (1917 start → coal/steam → diesel → electric)? The historical arc suggests era-varying fuel/propulsion." and (line 58) "**Propulsion typing** (electric tram/trolleybus vs diesel vs 1917 horse/steam) — W&R leaves this native; we make it data, tied to the era arc."
- what should have covered it: "Model the vehicle as an owned physical asset" AC-1 covers a generic fuelType field but never tests propulsion-type variation (electric vehicles bypass fuel entirely per line 5: "or eletric for trams/trolleybuses") / no scenario
- why it matters: the spec explicitly states electric vehicles consume no fuel resource at all — AC-1's blanket rule ("an empty tank halts movement") has no stated exception for electric-typed vehicles, an observable behavioral gap.

## F9 — Cargo stations as medium-transfer nodes (rail↔road↔ship↔air) have no AC or scenario
- classification: omission
- source: spec/logistics.md:31
- verbatim: "**Cargo stations** (`$TYPE_CARGO_STATION`, 43 files) are the medium-transfer nodes (rail↔road↔ship↔air)."
- what should have covered it: no story / no scenario
- why it matters: transshipment between transport media (a truck handing cargo to a train) is a named node type in the confirmed vocabulary but is entirely absent from the extracted requirements — multi-modal logistics trips are unaddressed.

## F10 — Fixed conveyance (conveyor/pipe/cable/heat links moving goods with no vehicle) has no AC beyond the network-resource exclusion
- classification: thin-AC
- source: spec/logistics.md:38
- verbatim: "**Two kinds of edge (CONFIRMED):** vehicle-served docks (road/rail/water/air + loading rate) **vs fixed conveyance** — conveyor/bulk-chute/pipe/cable/heat links that move goods **with no vehicle at all**. Adjacent mine→processor belts bypass the truck fleet entirely."
- what should have covered it: "Exclude network-borne resources from the vehicle scheduler" only covers electricity/heat/water/sewage, not solid-goods fixed conveyance (ore belts, bulk chutes) / no scenario
- why it matters: the spec distinguishes fixed conveyance for ordinary cargo (ore, bulk goods) from network-borne utility flows — these are different mechanisms (LogisticsEdge.medium=CONVEYOR vs a grid), and only the utility-flow half got an AC; conveyor-adjacent goods transfer with no vehicle is untested.

## F11 — Loading draws power (`$ELETRIC_CONSUMPTION_LOADING_FIXED`) has no AC
- classification: omission
- source: spec/logistics.md:29
- verbatim: "(loading also draws power: `$ELETRIC_CONSUMPTION_LOADING_FIXED`)"
- what should have covered it: "Model loading/unloading dock throughput as a real bottleneck" covers the rate cap but not the power-draw coupling / no scenario
- why it matters: this is a named cross-domain coupling (logistics dock activity consumes the electrical grid) that the extraction's dock-throughput story doesn't mention at all.

## F12 — Per-frame material round-robin as a scheduler rate-limit (explicitly "adopted") has no AC
- classification: omission
- source: spec/logistics.md:82
- verbatim: "Rate limiting: materials are round-robined across frames (`GetFrameReason`) — a proven amortisation trick we adopt."
- what should have covered it: no story / no scenario
- why it matters: this is marked "adopted" (a committed design decision, not an open question) in the evidence log ("Per-frame material round-robin as scheduler rate-limit | CONFIRMED (CS1) → adopted") yet no AC constrains scheduler tick behavior to this amortisation pattern.

## F13 — Dispatch trip state machine (travel→load→travel→unload) is only partially covered
- classification: thin-AC
- source: spec/logistics.md:44
- verbatim: "Dispatch { vehicleId; request; state }                    // travel→load→travel→unload"
- what should have covered it: no story covers the full 4-state dispatch lifecycle as an observable sequence; "Model loading/unloading dock throughput as a real bottleneck" only tests the load-rate cap in isolation / no scenario exercises travel-to-load-to-travel-to-unload as a sequence
- why it matters: the data model names a specific ordered state machine for every delivery; none of the extracted ACs assert that a vehicle actually transitions through all four states in order (e.g., that a vehicle can't unload before it has traveled and loaded).

## F14 — Scheduler granularity vs performance / spatial partitioning open question has no AC
- classification: omission
- source: spec/logistics.md:76-77
- verbatim: "**Scheduler granularity vs performance.** A global solve every tick over thousands of stores is infeasible. CS1's confirmed trick — round-robin materials across frames — plus spatial partitioning (`architecture/` rules: never globally solve what's local). How local can matching be without starving distant needs?"
- what should have covered it: no story / no scenario
- why it matters: flagged as still-open in the source, so its absence from ACs is defensible as unresolved design — noted for completeness, not scored as a hard omission (see classification note below).

## F15 — Wiring: explicit vs automatic source→destination routing open question has no AC
- classification: omission
- source: spec/logistics.md:78
- verbatim: "**Wiring: explicit or automatic?** W&R makes the player wire source→destination manually. Do we require wiring (planning gameplay) or auto-match within a district with optional overrides? Lean: auto-match by default, player overrides as the planning tool."
- what should have covered it: no story / no scenario
- why it matters: unlike F14, this open question carries a resolved "Lean" (auto-match by default with player overrides) — the same pattern the extraction did turn into ACs elsewhere (e.g., driver-binding lean in F3) but skipped here entirely.

## F16 — Customs vehicle bay / physical clearing mechanics beyond "a trip happens" has no AC
- classification: thin-AC
- source: spec/logistics.md:34
- verbatim: "plus vehicle bays where trucks clear customs. Utilities cross at dedicated border buildings ... the good's transport medium decides which border building it crosses at, exactly as domestically."
- what should have covered it: "Make the external partner a finite, physical customs crossing" ACs test throughput cap and delay, but never that the crossing point is selected by the good's transport medium (a road good crosses at a road border building, a rail good at a rail one) / no scenario
- why it matters: the spec asserts medium-specific border routing as a hard constraint ("exactly as domestically"), i.e. the same transport-class/medium gate from vehicles applies at the border — this specific coupling is untested even though the general customs-delay behavior is.

## F17 — Fleet as dense-array ECS entities (80k vehicles as data) — architecture note, not behavior
- classification: intentional-exclusion
- source: spec/vehicles.md:104
- verbatim: "Vehicles are dense array entities (`architecture/ecs.md`) — 80k vehicles as data, not GameObjects."
- what should have covered it: n/a
- why it matters: this is an implementation/architecture directive, not an observable behavioral requirement — correctly out of scope for story/scenario extraction.

TOTAL FINDINGS: 15
