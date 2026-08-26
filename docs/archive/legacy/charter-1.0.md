# The 1.0 Charter

**Status:** the plan of record for shipping soviet-simulator 1.0. Authored 2026-08-17
as the destination of the wayfinder map
[the Road to 1.0](https://github.com/caioniehues/soviet-simulator/issues/81) — 28
decision tickets, every one closed. Each section links the ticket that holds its
reasoning; **this document gists, the tickets hold the detail.**

A charter is finite by construction. `ROADMAP.md` used to run to open-ended fog;
from here to 1.0 there is a fixed rung list, a fixed cut line, and a fixed
acceptance bar. Nothing in this document requires a further decision of scope or
taste before someone builds it.

---

## 1. Identity (settled long before this charter — never re-litigated)

A large-scale **socialist planned-economy** city, infrastructure, logistics and
society simulator: CS2's authoring, legibility and scale + W&R's physical
causality + Dwarf Fortress's persistent individual identities.

- **Player fantasy: THE PLANNER.** Quotas from above, scarce means below.
  Queue-as-demand, allocation-not-markets, brownout-before-blackout as policy.
- **The one rule:** nothing teleports; every effect has a physical cause. Fiat
  exists only as an explicitly marked bootstrap.
- **Never game over:** failure is leaner tranches, colder homes, longer queues.
- Money is the **foreign rouble at the border**; internally the economy stays
  planned.

## 2. Audience — who 1.0 is for
[#87](https://github.com/caioniehues/soviet-simulator/issues/87)

**Self and friends, shipped as an unlisted itch build.** Steam is a possible
future effort, not a line item here.

What that fixes: EN only; no accessibility line item; minimal settings; **fixed
keybindings**; local panic log + autosave-on-crash instead of telemetry;
CI-built Linux and Windows binaries; no trailer.

**The load-bearing split:** the *shell* bar is friends-grade, but the **visual and
game-feel bar is stranger-grade** — "would a stranger think this looks
finished?" — even though no stranger will see the build. This is the clause that
answers the playtest verdict that started the map.

## 3. Posture
[#90](https://github.com/caioniehues/soviet-simulator/issues/90)

> **Lean systems, maximal polish, opportunistic breadth.**

Cut discipline for systems (texture is cut); polish weighted co-equal with
systems (the braid enforces it); breadth admitted only where research proved
reuse makes it cheap — rail rides the existing lane graph, S15 rides the
dispatcher and customs. **One deliberate exception:** terrain with water and
hydro dams, mandated as part of what 1.0 *is*.

## 4. What "polished" means
[#89](https://github.com/caioniehues/soviet-simulator/issues/89)

> **1.0 is polished when a stranger can boot it, learn it from the First Plan
> alone, play for two hours without outside help, and at no point see anything
> that looks unfinished.**

Nine shell items carry that definition, each with its own bar: app-state retrofit
(the keystone, built first and alone); main menu; saves UX (3 rotating
period-end autosaves + named manual saves); pause/date/speed UI; minimal
settings; notifications + event log (action-needed toasts only, no spam); camera
polish; the in-HUD onboarding objective strip; packaging + crash posture.
**Cut:** performance-scaling options — a single pinned target replaces them.

## 5. The braid
[#88](https://github.com/caioniehues/soviet-simulator/issues/88)

1. **No two consecutive B-rungs** (systems) without a G or P rung between them.
2. **Shell work counts as G**, so the braid pulls it forward instead of letting
   it pool into a terminal death-march.
3. **The ladder opens in debt:** the first rung is G/P, paying down the felt-debt
   the playtest recorded. The braid prevents *new* unfelt depth; this clause
   clears the existing arrears.

Origin: after B8 the sim was deep and unfelt. That is doctrine now, not a lesson
to re-learn per milestone.

## 6. The cut line
[#90](https://github.com/caioniehues/soviet-simulator/issues/90)

### Ships in 1.0

| System | Scope | Ticket |
|---|---|---|
| Resource tree | **S15 "Steel and Bread"** — 15 resources, 12 new recipe buildings; Goods gains a real steel+boards recipe; Food and Meat become separate dwelling needs; water stays a utility, never cargo | [#92](https://github.com/caioniehues/soviet-simulator/issues/92) |
| Agriculture | **Field-cycle** farming on the existing climate sinusoid (stockpile before winter); livestock as continuous conversion; no perishables, no granary kind | [#93](https://github.com/caioniehues/soviet-simulator/issues/93) |
| Services (B9) | All four loops: demographics **including death**, education at **two tiers** (School + Technical Institute), healthcare, waste with **both** landfill and incinerator. **Medicine is a 16th, import-only resource** | [#94](https://github.com/caioniehues/soviet-simulator/issues/94) |
| Terrain & water | **The posture exception.** Heightfield terrain, reservoir-graph hydrology, hydro dams — three stages, procedural seed maps, ore-deposit siting, minimal bridges, no free terraform | [#96](https://github.com/caioniehues/soviet-simulator/issues/96) |
| Rail | **Minimal freight** — 3 buildings, 1 loco + 1 wagon, fixed consists bought at customs, built with the ordinary spline and construction tools | [#91](https://github.com/caioniehues/soviet-simulator/issues/91) |
| Border (B10) | All 16 resources trade both ways at **fixed per-kind prices** (no market); single rouble; multiple customs offices; **one fixed 1950s–60s era**, flat catalogue | [#95](https://github.com/caioniehues/soviet-simulator/issues/95) |
| Pollution | Output-scaled emissions into a 64² decaying field, coupled to **sickness, crop yield and basin water**. The lesson is siting and throttling | [#97](https://github.com/caioniehues/soviet-simulator/issues/97) |
| Campaign | **Three authored plans on one continuous save**, then procedural endless scaled off the player's own output | [#102](https://github.com/caioniehues/soviet-simulator/issues/102) |
| Art | Zero spend. Grounding pass, palette factory, UI redraw, juice, ground dressing, weathering, **bounded visible citizens, day/night, visible seasons** | [#105](https://github.com/caioniehues/soviet-simulator/issues/105) |
| Audio | Zero-spend CC0, three layers: UI feedback first, then ambience, menu-only music | [#104](https://github.com/caioniehues/soviet-simulator/issues/104) |
| Shell + UX | The nine items of §4; the three-tier UX ledger of §8 | [#89](https://github.com/caioniehues/soviet-simulator/issues/89), [#103](https://github.com/caioniehues/soviet-simulator/issues/103) |

### Post-1.0 (committed direction, not designed now)

Loyalty / legitimacy / broadcast / monuments (the crown jewel, gets its own
design effort, [#99](https://github.com/caioniehues/soviet-simulator/issues/99)) ·
vehicle lifecycle including fuel-as-commodity
([#101](https://github.com/caioniehues/soviet-simulator/issues/101)) · B11 crime ·
era calendar from 1917, vehicle manufacture, voltage tiers, dual currency ·
ships/docks, pipelines, cableways, containers, **airplanes and helicopters** ·
S25 petrochemical tree · grid depth (transformers, treatment tiers, CHP,
electric-heating fallback) · passenger rail, signals, electrification ·
free terraform, cell-level water · kindergarten, deathcare, epidemics ·
perishables and refrigerated transport · Steam and all marketing.

### Never

Tourism, hotels and attractions
([#100](https://github.com/caioniehues/soviet-simulator/issues/100)) — antithetical
to the fantasy. Fires and disasters
([#98](https://github.com/caioniehues/soviet-simulator/issues/98)) — random
destruction is not this game's pressure source; scarcity is.

## 7. The ladder

Sixteen rungs, braid-compliant by construction (no two B's adjacent; opens G/P).
Each rung keeps the standing discipline: charted as its own wayfinder map, a
bench gate where it adds cost, and an acceptance capture judged side-by-side
against the previous recording.

| # | Rung | Type | Content |
|---|---|---|---|
| **R0** ✅ | **The State Document** | G+P | **Done** ([#110](https://github.com/caioniehues/soviet-simulator/issues/110)). Grounding pass (SSAO + contact shadows + TAA + cascade tuning), sky/horizon gradient, **material factory enforcing the palette** (fixes the lawn-green violation), UI redraw to a drawn state-document system, HUD hierarchy, **critical warnings in signal-fail colour + toast**, refusal feedback, first juice. *Mandatory opening rung.* |
| **R1** | The Planner's Hands | G | **Specced** ([#117](https://github.com/caioniehues/soviet-simulator/issues/117), ADRs 0007–0011). Placement snapping + ghost preview showing the **material bill** and refusal reason, **rescind** (not a general undo), tooltips everywhere + toolbar icons, inspect-depth panel ("why is this not working"), camera easing + zoom-to-cursor; building gains a rotation, **save v6**. *Before S15 multiplies the cost of bad placement.* Selection outlines struck — shipped in R0.5. |
| **R2** | Real Matter | B | The S8 construction layer: Wood, Boards, Cement, Concrete, Bricks + 5 buildings; construction sites consume real materials. |
| **R3** | The Sound of Work | P | Audio layers 1–2 (UI feedback, then ambience) + geometry weathering (base bands, window repetition, rust roof edges). |
| **R4** | Steel and Bread | B | The rest of S15: Iron, Steel, Prefab panels, Crops, Food, Livestock, Meat + 7 buildings; **field-cycle farming**; Goods' real recipe; Food/Meat as separate needs. |
| **R5** | The Second Plan | G | Authored Plan II teaching the S15 economy and the winter deadline; objective strip + hint lines in the state's voice. |
| **R6** | Care of the State | B | B9: demographics with ageing and death, education at two tiers, healthcare with imported medicine and ambulance dispatch, waste with landfill + incinerator. `bench_services` at 250k. |
| **R7** | Faces in the Crowd | P | Bounded visible citizens with walk/queue/work states, day/night cycle, powered-state window glow, ambient life. |
| **R8** | The Land | B | Terrain T1: heightfield mesh + ray-marched cursor, `Terrain` sampler, road draping + grade gates, cut/fill earthworks, slope refusals, **procedural seed maps**, **ore-deposit siting**, minimal bridges, save v7 (R1 took v6). Slope refusals arrive as new variants on R1's placement verdict. |
| **R9** | The Living Land | P | Ground dressing (scatter, worn yards, road shoulders), snow cover on the sinusoid, fields cycling green → gold → bare. |
| **R10** | The Waters and the Air | B | Terrain T2 (basins, stage–volume curves, D8 spill edges, seasonal inflow) + T3 (**the dam**, turbine/spillway, hydro into the untouched pool solver) + the pollution field and its three couplings. |
| **R11** | The Shell | G | App-state retrofit (keystone, alone), main menu, new game with seed presets, load/save screen, period-end autosaves, pause/date/speed, settings, notifications + event log, **machine-kind save fix + save-soak hash gate**. |
| **R12** | The Iron Road and the Border | B | Minimal rail (depot, cargo station, customs hookup, loco + wagon, block reservation, consists) + full 16-resource two-way trade at fixed prices. |
| **R13** | The Third Plan | G | Authored Plan III teaching terrain, the dam, rail and services; procedural endless mode; the skip path from the new-game screen. |
| **R14** | Ship Shape | P | Packaging CI (Linux + Windows → unlisted itch), crash posture, camera final pass, remaining juice, menu music if a CC0 track fits, provenance table complete. |
| **R15** | **1.0** | gate | The save line in the sand; all bench gates green at 250k / 60 fps; the stranger test on a still frame *and* ten seconds of idle motion; a full three-plan campaign playthrough into endless. |

**Honest estimate: roughly 65–90 agent sessions.** The largest single item is
terrain (R8 + R10, ~10–14), then the shell (R11, ~6–9 after the audience cuts)
and services (R6, ~6–8).

## 8. The UX ledger
[#103](https://github.com/caioniehues/soviet-simulator/issues/103)

**Tier 1 — legibility defects, not taste** (all in R0): critical warnings must be
visually critical; palette enforcement; refusals need feedback beyond text; HUD
information hierarchy. Two of these are regressions against our own written
intent, which is why they lead the ladder.

**Tier 2 — the interaction debt W&R is criticised for** (all in R1): placement
snapping and preview, undo, tooltips, inspect depth, selection feedback.

**Tier 3 — comfort** (rides the shell rungs): camera feel, toolbar icons,
citizen legibility.

## 9. Engineering posture

- **Performance target** ([#107](https://github.com/caioniehues/soviet-simulator/issues/107)):
  **250k citizen identities at 60 fps** on the dev machine, full six-band
  SimTick. Five new bench gates re-anchored to that scale: `bench_services`,
  `bench_terrain`, `bench_chains`, `bench_rail`, `bench_save`. Sim gates stay
  headless. A gate regression blocks a rung from closing.
- **Saves** ([#106](https://github.com/caioniehues/soviet-simulator/issues/106)):
  version-gated hard breaks through development (v6 terrain, v7 S15, v8
  services, v9 rail), then **one line in the sand at 1.0-rc** — from a released
  version onward, saves keep working. The machine-kind bug is a fix, not a known
  issue.
- **Acceptance** per rung: a verified capture video (or a played session for
  G-rungs), judged against the previous recording.
- **Evidence discipline, no magic, local over global, benchmark before scale** —
  unchanged standing rules.

## 10. Research banked while charting

Four research branches hold the measurements this charter rests on:

- `research/rail-minimum` — W&R's rail data is 2 vehicle types and a
  parameterless track layer; minimal rail is small because nearly everything
  reuses what exists.
- `research/resource-closures` — the complete W&R production graph; growth is
  chain-shaped, with closure frontiers at 8, 15 and 25 resources.
- `research/game-shell` — per-item shell costs on Bevy 0.19; native `bevy_ui`
  throughout; the app-state retrofit is the keystone.
- `research/visual-polish` — the ranked ten techniques; the grounding pass is
  the highest lift per unit effort.
- `research/terrain-water-hydro` — W&R ships **no hydro at all**; our codebase is
  already Vec3-native, so the pool solver needs zero changes and terrain is
  ~6–10 sessions.
