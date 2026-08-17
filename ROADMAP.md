# Roadmap — Bevy track

Re-derived from the 22 specs for the Bevy track (charter map [#1](https://github.com/caioniehues/soviet-simulator/issues/1), ladder ticket [#6](https://github.com/caioniehues/soviet-simulator/issues/6)). The derivation method carries over from the Unity track: **order by the specs' hard-dependency layering; every rung ships a demonstrable playable increment; deliberate bootstrap stubs that later rungs replace; each rung is charted as its own wayfinder map and closed against its acceptance demo and benchmark gate.**

Two deliberate departures from the Unity ladder ([soviet/ROADMAP.md](https://github.com/caioniehues/soviet)):

1. **Traffic-at-scale moves from rung 7 to rung 4.** W&R's traffic collapse is this project's founding grievance; pathfinding is its own engineering project. The *representation* risk (two-layer roads, lane graph, incremental recompile) dies in B1; the *scale* risk (congestion, reservation, background pathfinding) dies in B4 — before construction, housing, and utilities multiply the traffic load on top of it.
2. **Public transit gets a named rung (B5).** Transit dominance is a pillar; the Unity ladder never shipped a transit line. Citizens commuting by tram is the game's identity, not a follow-on.

There is no rendering rung: Bevy 0.19's GPU-driven instancing works from day 1, so rendering stays a cross-cutting track.

Legend: `[x]` done · `[~]` charted/underway · `[ ]` planned

---

## Bootstrap cycles

- **Electricity ⇄ production:** power gates production, but a plant *is* a production building. B1 stubs the gate (plant burns fuel unstaffed); B2 closes the labour half, B8 the utility half.
- **Construction ⇄ everything:** construction needs vehicles/roads/materials, which come to exist through construction. B1's instant dirt roads and single-quantum paved build are the stub; B6 replaces it with the real phased process.
- **Transit ⇄ construction:** B5 places stops/depots via the same bootstrap building stub every pre-B6 building uses; B6 retrofits them into the phased pipeline.

---

### B1 — The First Chain `[~]`

**Destination** — a standalone Linux build proving the one rule end to end in Bevy: coal physically hauled by truck powers a plant that powers a factory; cut the road and the blackout cascades. Ships the road representation *done right* per `spec/roads.md`: player-drawn spline centerlines compile into the one unified lane graph (node/segment/per-lane buffer — the lane network *is* the routing graph), with incremental 1-ring recompile and `modificationIndex` invalidation, so B4 scales a sound foundation instead of replacing a naive one.
**Acceptance demo** — RTS camera over terrain; road tool (dirt instant, paved from delivered gravel); truck shuttling mine→plant; power gate on the factory; cut/rebuild hotkeys showing the cascade.
**Specs consumed** — `roads.md` (stage 1), `resources.md` (stage 1), `production.md` (stage 1), `electricity.md` (stage 1), `vehicles.md` (movement only).
**Bootstrap stubs left behind** — unstaffed buildings; office-less single-quantum construction; ad-hoc shuttle hauling; electricity-only utilities.
**Benchmark gate** — ms/tick at 100 concurrent chains within 2% of a 60 fps frame (Unity reference: 0.34 ms). Headless, graphics-independent.

---

### B2 — Hands: Citizens and Labour `[x]`

**Destination** — nothing runs unstaffed anymore: persistent citizens with households live somewhere, are allocated to workplaces by a planning pass, commute over the road network, and their presence gates every building's output.
**Acceptance demo** — dwellings house households; workers commute to the mine/plant/factory; production scales with staffing; pull the workers or block the commute and the chain degrades exactly as cutting the road did.
**Systems in scope** — citizen identity + household unit (ship together); labour planning pass with commute feasibility; needs stage 1 (needs only); staffing factor in production; simple dwellings; save/load first cut (custom serde columns — identities make saves meaningful).
**Specs consumed** — `citizens.md` (stage 1), `households.md` (stage 1), `needs.md` (stage 1), `production.md` (labour factor).
**Depends on** — B1.
**Out of scope** — education tiers (B9), demographics (B9), housing-queue politics (B7), shift schedules.
**Benchmark gate** — 50k identities + the B1 chain under 2 ms/tick.

---

### B3 — The Dispatcher: Logistics and the Fleet `[ ]`

**Destination** — hauling becomes policy, not hand-wired shuttles: the player sets target stock levels per storage; a shared dispatcher matches deficits to supply and sends an owned, finite vehicle — no free trucks, ever.
**Acceptance demo** — a multi-storage network self-balances to targets; all vehicles busy ⇒ deliveries queue visibly; a vehicle is a depot-parked asset you can count.
**Systems in scope** — generalized dispatcher (deficit→match→dispatch, transport-class gating) replacing B1 shuttles; vehicles as owned assets stage 1 (capacity, class, depot slots; fuel/wear stubbed); storage buckets; dispatch readout.
**Specs consumed** — `logistics.md` (stages 1–2), `vehicles.md` (stage 1), `resources.md` (transport classes).
**Depends on** — B2 (drivers are citizens with jobs).
**Out of scope** — rail/fixed conveyance (fog rung), fuel + wear (B8), vehicle manufacture (B10).
**Benchmark gate** — dispatcher cost sub-linear in storages; 1k concurrent freight orders within tick budget.

---

### B4 — Streets Alive: Traffic at Scale `[ ]`

**Destination** — movement gets real at scale, five rungs earlier than the Unity track dared: background hierarchical pathfinding with ref-counted shared paths, emergent congestion from local lane rules, stall-as-planning-signal instead of despawn.
**Acceptance demo** — a congested junction backs up visibly; a stalled corridor is a dashboard signal, not a vanished truck; thousands of vehicles at 60 fps.
**Systems in scope** — pathfinding engine (off-main-thread via `AsyncComputeTaskPool`, per-edge congestion scalar, path sharing by origin/destination area); traffic stage 1 (lane reservation, waits, re-route); curved roads (node-fan tangents).
**Specs consumed** — `pathfinding.md`, `traffic.md` (stage 1), `roads.md` (stages 2–3).
**Depends on** — B3 (real fleets to stress it with).
**Out of scope** — signals/parking polish, pedestrians-on-network (B5 gives them transit first).
**Benchmark gate** — 10k simultaneous vehicles, 60 fps, pathfinding off the main thread; no tick-budget regression for B1–B3 systems.

---

### B5 — The Lines: Public Transit `[ ]`

**Destination** — the pillar becomes playable: player-drawn tram and bus lines with stops, schedules and depot-owned vehicles; citizens plan multi-leg trips (walk → stop → ride → walk) and mode choice makes transit culturally dominant when the network deserves it.
**Acceptance demo** — draw a tram line past the dwellings and the factory; commuters visibly switch from walking/trucks to riding; delete the line and staffing at the far factory degrades; overcrowded trams leave people at stops.
**Systems in scope** — line/stop/schedule model; transit vehicles as owned assets on the traffic network; multi-leg trip planner atop B4 pathfinding; mode-choice cost model (time/comfort/transfer weights per vision doc); stops/depots via bootstrap building stub.
**Specs consumed** — `traffic.md` (transit stage), `pathfinding.md` (multi-modal), `citizens.md` (trip model), `needs.md` (mobility coupling).
**Depends on** — B4, B2.
**Out of scope** — metro/rail (fog rung), fares (planned economy — free at point of use), era vehicle catalogue (B10).
**Benchmark gate** — 5k concurrent riders with multi-leg trips inside band budgets.

---

### B6 — Building for Real: Phased Construction `[ ]`

**Destination** — the construction stub dies: a blueprint becomes a building only through the phase ladder (groundworks → structure → wiring → rooftop), each phase consuming delivered materials and machine-work from a construction office's fleet.
**Acceptance demo** — watch a building rise phase by phase; starve it of steel or the excavator and the site stalls with the bottleneck named on screen; roads and transit stops build through the same pipeline.
**Systems in scope** — construction office + machine fleet (skill-matched); bill-of-quantities; building lifecycle with activation gating; zoning stage 1 (land-use overlay + siting validity — *intent, never a spawn trigger*); demolition first cut.
**Specs consumed** — `construction.md` (stages 1–2), `buildings.md` (stages 1–2), `zoning.md` (stage 1), `vehicles.md` (construction machines).
**Depends on** — B3 (material delivery), B2 (construction workers).
**Out of scope** — renovation-in-place, rubble demolition, informal construction.
**Benchmark gate** — 100 concurrent sites without breaking the tick budget.

---

### B7 — A Roof Over Every Head: Housing and the Plan `[ ]`

**Destination** — housing is allocated, never bought: the visible housing queue run by the housing office *is* residential demand, and the planner answers it by physically building dwellings staffed and supplied by everything before.
**Acceptance demo** — families join the queue; queue length drives build decisions; a finished block fills by allocation policy; overcrowding and eviction-to-queue work.
**Systems in scope** — housing queue + allocation policy; household stage 2; shortage/plan-fulfillment dashboard (zoning stage 2); dedicated UI pass (tool palette, inspection panels via `bevy_egui`).
**Specs consumed** — `households.md` (stage 2), `zoning.md` (stage 2), `needs.md` (housing coupling).
**Depends on** — B6, B2.
**Out of scope** — demographics (B9), hostels, political loyalty.
**Benchmark gate** — hold the B2 citizen gate with allocation running.

---

### B8 — The Web: Utilities `[ ]`

**Destination** — the utility-network solver generalizes: water⇄sewage (one cycle), district heating with temperature-driven demand, electricity upgraded to tiers + priority-class deficit allocation; utilities gate production per the full factor stack.
**Acceptance demo** — winter raises heat demand; an overloaded grid browns out by priority class (hospitals last); a backed-up drain shuts water consumers; treat-or-discharge pollutes or costs.
**Systems in scope** — shared capacitated-network solver, amortised per clock bands; pipe networks + quality grades; heating plant + electric fallback; production stage 2 (full Liebig stack); vehicle fuel becomes a real commodity.
**Specs consumed** — `water.md` + `sewage.md` (stages 1–2), `heating.md` (stages 1–2), `electricity.md` (stage 2), `production.md` (stage 2), `vehicles.md` (fuel).
**Depends on** — B6 (networks are built), B3 (tanker fallback).
**Out of scope** — CHP, storage tech, storm load, contamination coupling.
**Benchmark gate** — full-city solve within its clock-band budget.

---

### B9 — Care of the State: Services `[ ]`

**Destination** — no coverage auras, ever: education is enrolment with seat-time progression feeding worker qualification; sickness is an acute event cured by a dispatched, staffed, supplied hospital; waste is collected by the dispatcher and recycled, burned, or dumped.
**Acceptance demo** — a school cohort graduates and unlocks skilled staffing; a sick worker is ambulanced, cured over days, returns to work; uncollected bins breed sickness; the incinerator feeds the grid.
**Systems in scope** — education ladder (entangled with qualification tiers); healthcare loop (medicine as commodity); waste loop (typed containers → sort/incinerate/landfill); needs stage 2 (wants); demographics stage 1 (birth/ageing so schools matter).
**Specs consumed** — `education.md` (stages 1–2), `healthcare.md` (stages 1–2), `waste.md` (stages 1–3), `citizens.md` (stage 2), `needs.md` (stage 2), `households.md` (stage 3 partial).
**Depends on** — B8, B3, B7.
**Out of scope** — epidemics, deathcare, adult re-education, crime (B11).
**Benchmark gate** — service events at **250k identities** within band budgets.

---

### B10 — The Border: Trade and the Calendar `[ ]`

**Destination** — the outside world exists and time passes: dual-currency physical border trade (customs haul, no infinite edge), the calendar drives era progression from 1917, and the vehicle/goods catalogue changes by year — importing to bootstrap becomes a real strategy.
**Acceptance demo** — import electricity to start your first plant; sell surplus for hard currency; watch the buyable catalogue change across decades; horses before lorries.
**Systems in scope** — trade (border stations, published market, rouble/dollar split); calendar + era gating; vehicle manufacture (closing the vehicles loop) and era-typed propulsion; year-drift modifiers.
**Specs consumed** — `trade.md`, `vehicles.md` (stage 3), `resources.md` (stage 3).
**Depends on** — B8 (something to trade), B3 (customs haulage).
**Out of scope** — diplomacy/blocs beyond pricing.
**Benchmark gate** — hold the 250k gate with trade + calendar running.

---

### B11 — Order and Discontent `[ ]`

**Destination** — the society pushes back: crime emerges from neglect and is cleared only by physical arrest→court→fed prison; the black market leaks state inventory under chronic shortage; aspirations complete the needs model.
**Acceptance demo** — a neglected district's crime rises; an arrest consumes police capacity and a fed prison bed; chronic shortage visibly leaks goods; aspiration satisfaction becomes a long-run score.
**Systems in scope** — crime loop; black market stage 1; needs stage 3 (aspirations, very-low-frequency band); zoning stage 3 (planning-office proposals).
**Specs consumed** — `crime.md`, `needs.md` (stage 3), `zoning.md` (stage 3).
**Depends on** — B9, B7.
**Benchmark gate** — the **500k → 1M** identity ladder begins here; every new system behind the gate.

---

## Polish ladder (parallel P-track)

Systems rungs prove mechanics; polish rungs make them look and feel like the finished product. Same wayfinder-map discipline — each P-rung is charted, has an acceptance demo (always a recaptured video judged against the previous one), and closes. Anchored to B-rungs so art never waits until "later" again. Art direction: **W&R-like industrial realism** — gritty, weathered, Soviet-era material honesty — *studied from reference, never ripped*; assets are our own procedural meshes, CC0 libraries (ambientCG, Poly Haven, Kenney), and (once approved) generated models.

### P1 — First Light `[x]` (anchor: after B1 — runs now, on the M1 scene)

**Destination** — the M1 demo scene stops looking like a debug view: real lighting and atmosphere, materially-honest ground and roads, buildings with silhouettes and detail instead of colored boxes, trucks that read as trucks, HUD that reads as a designed interface. Zero asset spend: procedural detail + CC0 textures/models only.
**Acceptance demo** — recapture the B1 acceptance video with identical script; side-by-side with the M1 capture it must read as "a game" vs "a prototype".
**In scope** — art-direction doc (reference study, palette, material rules); lighting/atmosphere (sun angle, shadows, ambient, tonemapping, bloom/AO, distance fog, sky); terrain material (tiled CC0 ground with variation, road wear decals); building meshes v2 (multi-part procedural: walls/roofs/windows/chimneys, plant smoke VFX); road/wire pass (textured ribbons, catenary sag poles); truck model (CC0 or procedural v2, wheel spin); UI theme (font, panel styling, iconographic key legend).
**Out of scope** — paid generation (separate spend decision), character art (P2), audio (P3).

### P2 — Faces in the Crowd `[ ]` (anchor: after B2)

Citizens and dwellings get the same treatment: readable people at RTS zoom, animation states (walk/work/queue), dwelling architecture pass, day/night cycle with window glow.

### P3 — The Republic Sounds `[ ]` (anchor: after B5)

The identity slice: tram/transit visual pass, full audio (ambient industry, vehicles, UI), weather/seasonal dressing first cut, camera feel (easing, cinematic zoom levels).

### P4 — Ship Shape `[ ]` (anchor: pre-release fog)

Menus, settings, onboarding, loading, performance-scaled visual options, trailer-grade capture tooling. Charted when the frontier approaches.

---

## Rungs 12+ (named fog — direction without fake precision)

- **Rail** — freight + passenger rail, signalling, stations; the dispatcher and transit models both extend.
- **Seasons & Agriculture** — growing cycles, harvest logistics, food chains deepen; heating demand already seasonal from B8.
- **Political Legitimacy** — loyalty, perceived fairness, the state's mandate as an emergent consequence of everything above.

Each is charted only when the frontier reaches it.

---

## Cross-cutting tracks (attached to milestones, never their own)

- **Save/load** — first cut in B2 (custom serde-column format per ecosystem survey); hardened whenever a rung adds persistent state.
- **UI** — every rung ships its demo's minimum; the dedicated pass is B7.
- **Rendering** — Bevy 0.19 native (GPU instancing, `VisibilityRange` LOD) from day 1; look-and-feel lives in the P-ladder above (P1 sets the art direction).
- **Benchmarks** — identity ladder: 50k (B2) → 250k (B9) → 500k–1M (B11+). Simulation always benchmarked headless, independent of graphics.
- **Simulation clock** — systems adopt the six-band registry as their cost demands it (the B8 solver is the forcing function).

## Standing rules

- **No magic.** Every feature respects the one rule (README of the spec set).
- **Evidence discipline.** Tag everything; never confuse what a game appears to do with how it does it.
- **Local over global.** No feature ships if it forces a global recompute that could be local.
- **Benchmark before scale.**
- **Milestones are wayfinder maps.** Chart, then work; one map at a time.
