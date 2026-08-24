# Omission review — vehicles & logistics (reviewer 1)

## F1 — Fixed conveyance edges (no-vehicle transfer) have no story or scenario
- classification: omission
- source: spec/logistics.md:58
- verbatim: "**Two kinds of edge (CONFIRMED):** vehicle-served docks (road/rail/water/air + loading rate) **vs fixed conveyance** — conveyor/bulk-chute/pipe/cable/heat links that move goods **with no vehicle at all**. Adjacent mine→processor belts bypass the truck fleet entirely. CS1 has no fixed-edge layer (every transfer spawns a vehicle); we keep W&R's."
- what should have covered it: no story / no scenario
- why it matters: this is a second, explicitly-kept transport mode (goods moving without a vehicle) that is distinct from the "network-borne resources excluded" story (which only covers electricity/heat/water/sewage on utility grids) — solid/liquid goods on conveyors/pipes/chutes between adjacent buildings are unaddressed by any AC, and it directly interacts with the nothing-teleports rule (a fixed-edge good still must not teleport, it must flow at a rate).

## F2 — Round-robin/frame rate-limiting of scheduler matching has no proof obligation
- classification: omission
- source: spec/logistics.md:72, spec/logistics.md:99
- verbatim: "Rate limiting: materials are round-robined across frames (`GetFrameReason`) — a proven amortisation trick we adopt." / "**Scheduler granularity vs performance.** A global solve every tick over thousands of stores is infeasible."
- what should have covered it: no story / no scenario
- why it matters: this is stated as an adopted mechanism ("we adopt"), not an open question, yet no AC requires the scheduler to actually amortise matching across frames — an implementer could legally build a full-solve-every-tick matcher and no extracted AC would catch it.

## F3 — Dispatch state machine (travel→load→travel→unload) has no scenario covering the full cycle
- classification: missing-scenario
- source: spec/logistics.md:111
- verbatim: "Dispatch { vehicleId; request; state }                    // travel→load→travel→unload"
- what should have covered it: no story owns the Dispatch state machine itself / no scenario exercises the full travel→load→travel→unload cycle
- why it matters: extracted stories cover pieces (vehicle asset fields, waiting when no vehicle idle, dock loading rate) but no AC or scenario asserts the four-phase sequencing itself — e.g. that a vehicle must physically travel to source before loading, and travel to destination before unloading (the concrete falsifiable form of "nothing teleports").

## F4 — Empty return trips are not addressed
- classification: omission
- source: spec/logistics.md:31 (implicit in "travel → load → travel → unload" round trip, spec/logistics.md:111) and dispatch policy discussion (spec/logistics.md:5-15)
- verbatim: "score = f(priority, distance)" context and "assign a compatible idle vehicle from a depot/office\n        ↓  dispatch: travel → load → travel → unload" (spec/logistics.md:15, 111)
- what should have covered it: no story / no scenario
- why it matters: the spec's own dispatch pipeline implies a vehicle returns (empty or reassigned) after unloading, but no extracted AC states what a vehicle does immediately after a delivery completes (return-empty vs. seek next assignment) — this is a real dispatch-policy gap the reviewer brief specifically calls out.

## F5 — Vehicle taxonomy: capacity units, speed/power/weight, historical availability window, skill roles, lifespan token are entirely unrepresented
- classification: omission
- source: spec/vehicles.md:33-41
- verbatim: "$RESOURCE_CAPACITY <n>              // tonnes, or persons for PASSANGER (431 files)\n$RESOURCE_TRANSPORT_TYPE <class>    // exactly ONE cargo class per vehicle (419) — the hard gate\n$MOVEMENT_SPEED / _POWER_KW / _EMPTY_WEIGHT\n$COST_RUB | $COST_USD               // ruble vs dollar = Eastern vs Western bloc (ties to trade.md)\n$AVAILABLE <from> <to>              // historical production window, e.g. 1976 2001\n$SKILL_*                            // working roles: construction skills + PERSONAL/FIRETRUCK/AMBULANCE/…\n$LIFESPAN 35                        // ONLY on the 12 trolleybuses — the fleet's lone longevity token"
- what should have covered it: "Model the vehicle as an owned physical asset" AC-1..AC-4 cover fuel/wear/cargoClass/capacity/owner/driver only
- why it matters: capacity is only generically named ("capacity field") with no AC pinning that capacity is tonnes-or-persons; speed/power/weight (movement physics) have zero AC; the RUB/USD currency-bloc split (era/politics-relevant) is dropped in favor of a generic "price derived from BOM" AC; the $LIFESPAN-only-on-trolleybuses asymmetry (most vehicles have no authored lifespan) is lost entirely.

## F6 — Passengers-as-cargo-class (buses carry persons under the same capacity/class model as freight) has no AC
- classification: omission
- source: spec/vehicles.md:48
- verbatim: "Passengers are literally a cargo class (`RESOURCE_TRANSPORT_PASSANGER`, bus capacity 138 persons) — people ride the same capacity/class model as freight."
- what should have covered it: no story / no scenario
- why it matters: this is a named, confirmed design unification (passenger transport = freight transport under one vehicle/cargo model) that no extracted AC states or tests; an implementer could build passenger transport as an entirely separate system without violating any AC.

## F7 — Train tractive/cargo split (locomotive vs. wagon, bought as consists) has no AC
- classification: omission
- source: spec/vehicles.md:49
- verbatim: "Trains split tractive vs cargo assets: locomotive = power, no cargo; wagon = capacity, no propulsion (`$PURCHASE_EXCLUDE` — bought as consists)."
- what should have covered it: no story / no scenario
- why it matters: this is a distinct vehicle sub-taxonomy (a vehicle "asset" can be a two-part consist) not representable by the flat `Vehicle{}` schema the extracted AC-1..AC-4 test; nothing in the extraction flags that rail cargo requires this split.

## F8 — Full lifecycle end state (scrapyard: vehicle → waste_steel + waste_aluminium) not covered
- classification: omission
- source: spec/vehicles.md:67, spec/vehicles.md:84
- verbatim: "→ scrapyard: vehicle in → waste_steel + waste_aluminium out" / "wear → garage repair (consuming components, W&R's `CARPLANT` bucket made explicit) → eventual scrapping into recoverable steel/aluminium."
- what should have covered it: "Model the vehicle as an owned physical asset" AC-2 only says wear is "exposed for later repair/scrap logic" — no AC or scenario asserts the actual scrap output (waste_steel/waste_aluminium) or the repair input (CARPLANT parts bucket)
- why it matters: this is the closing leg of the vehicle cradle-to-grave loop and ties back into the resource economy (scrapped material re-enters production) — currently unfalsifiable, no AC would catch a scrap implementation that destroys the vehicle with no material output.

## F9 — Depot/office as "generic hauler that stores nothing it moves" — dispatch-office abstraction assumption is unexamined
- classification: omission
- source: spec/logistics.md:53
- verbatim: "The office is a **generic hauler** — it stores nothing it moves (fuel only); trucks shuttle export-bucket → import-bucket per the player's wiring."
- what should have covered it: no story / no scenario
- why it matters: per the reviewer brief's note, a generic "dispatch office" abstraction may not exist in the Egregoria substrate; the extraction's "Bound depot capacity by physical parking slots" story treats depots only as parking-slot containers and never states or tests the office's role as the *hauler* that owns/dispatches vehicles between two other buildings' buckets — the entity that actually executes "trucks shuttle export→import" is unaddressed.

## F10 — Loading also draws power (`$ELETRIC_CONSUMPTION_LOADING_FIXED`) not covered
- classification: omission
- source: spec/logistics.md:24
- verbatim: "`$STORAGE RESOURCE_TRANSPORT_OPEN 330` + `$VEHICLE_LOADING_FACTOR 5.7` / `$VEHICLE_UNLOADING_FACTOR` (loading also draws power: `$ELETRIC_CONSUMPTION_LOADING_FIXED`)."
- what should have covered it: "Model loading/unloading dock throughput as a real bottleneck" AC-1 covers only the loading-rate cap, not the power draw
- why it matters: loading is stated as consuming electricity in addition to being rate-limited — a second, independent constraint (a dock without power cannot load even with idle capacity and stock) that no AC represents.

## F11 — Propulsion typing / era-varying fuel (electric tram/trolleybus vs diesel vs 1917 horse/steam) not covered
- classification: omission
- source: spec/vehicles.md:87
- verbatim: "**Propulsion typing** (electric tram/trolleybus vs diesel vs 1917 horse/steam) — W&R leaves this native; we make it data, tied to the era arc."
- what should have covered it: no story / no scenario
- why it matters: stated as something we explicitly "make... data" (not merely an open question — it's listed under "Fuel & wear... We author what W&R left native" as one of the three authored gaps alongside fuel_l_per_km and wear, both of which ARE covered by AC-1/AC-2 of the asset story); propulsion typing is the third authored gap and is dropped.

## F12 — Customs house per-class token-bucket structure ("stockpiles nothing") not represented
- classification: thin-AC
- source: spec/logistics.md:44
- verbatim: "`$TYPE_CUSTOMHOUSE` (5 files) is a border **pass-through**: one `$STORAGE <class> 1` token-bucket per transport class (it handles any cargo but stockpiles nothing), `$BORDER_BUILDING`, an inward `$CONNECTION_ROAD` and an outward `$CONNECTION_ROAD_BORDER`/`$CONNECTION_RAIL_BORDER`, plus vehicle bays where trucks clear customs."
- what should have covered it: "Make the external partner a finite, physical customs crossing" ACs cover throughput-cap and delayed-resolution but never the pass-through/no-stockpile structural rule (capacity 1 per class, not a stockpiling warehouse)
- why it matters: the AC's throughput-cap language is generic ("finite per-tick or per-customs-house throughput limit") and could be satisfied by a customs house that stockpiles goods, which contradicts the spec's explicit "stockpiles nothing" pass-through model.

## F13 — CS1 border's "unlimited priority-0 offer" anti-pattern is named as the thing being rejected, but no AC tests that the *player-side* internal market never grants that same infinite-supplier behavior for non-customs cases
- classification: intentional-exclusion
- source: spec/logistics.md:38, spec/logistics.md:67
- verbatim: "The map border (`OutsideConnectionAI`) posts **unlimited priority-0 offers** for every cargo — an infinite supplier/sink that local partners always outbid."
- why intentional: this is background/contrast material (describing CS1's mechanism for comparison) rather than a normative requirement of our own system; the corresponding requirement (customs is capacity-limited) is separately and correctly stated at spec/logistics.md:48-58, which the extraction does cover via "Make the external partner a finite, physical customs crossing." Listed for completeness, not as a gap.

TOTAL FINDINGS: 12
