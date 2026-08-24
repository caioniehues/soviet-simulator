# The 1.0 charter

**Kind:** plan
**Authority:** binding
**Status:** active
**Owner:** project lead
**Last verified:** 2026-08-24

## Purpose and authority

This charter defines the 1.0 destination and cut line for the Rust/Egregoria hard fork. It binds
scope, not implementation status or mechanism. The current code and cited substrate fact-sheets
describe what exists; ratified specifications define how an in-scope system must behave; `br`
defines task state.

Earlier completion markings, rung status, estimates, and implementation assertions from the
discarded track do not transfer to this fork. An item is complete only when current implementation
and its required evidence establish it. This is a reset of completion evidence, not a change to
the identity or cut line below.

## Identity

Soviet Simulator is a socialist planned-economy city, infrastructure, logistics, and society
simulator. The player is the **Planner**: quotas arrive from above and scarce means constrain the
plan below.

The binding pillars are:

- Goods move physically or do not move; matching, payment, or allocation never teleports stock.
- Failure becomes queues, shortages, substitution, colder homes, and going without; it never ends
  the game.
- Domestic clearing uses queue, allocation, substitution, and going without, never price.
- The rouble is a single foreign currency used only at the border.
- Persistent individual identities and observable state let the Planner understand and correct the
  dishonest-enterprise loop.

## 1.0 scope

1.0 includes the following product commitments. Their exact mechanisms require a current,
ratified specification and evidence; this table does not claim that any are currently provided.

| Area | Binding 1.0 commitment |
|---|---|
| Resources and production | A fifteen-resource domestic tree and twelve new recipe buildings; Food and Meat are separate dwelling needs; Water is a utility, never cargo; Medicine is a sixteenth, import-only resource. |
| Agriculture and services | Field-cycle farming, livestock conversion, demographics including death, two education tiers, healthcare, landfill, and incinerator. |
| Planner interaction | Placement snapping and rotation; a ghost that exposes footprint, material bill, and refusal; one coherent placement verdict; rescind before ground is broken; selection and inspect-depth feedback; tooltips, toolbar icons, and polished camera movement. There is no general undo. |
| Terrain and environment | Procedural seed maps, heightfield terrain, reservoir-graph water, hydro dams, ore siting, minimal bridges, and pollution coupled to sickness, crop yield, and basin water. |
| Transport and border | Minimal freight rail with three buildings, one locomotive type, and one wagon type; fixed-consist border purchase; multiple customs offices; all sixteen resources are tradable at fixed per-kind prices in one rouble; Water is never cargo; trade mechanisms require a ratified specification; one fixed 1950s–60s era and flat catalogue. |
| Plans and onboarding | Three authored plans on one continuous save, then procedural endless mode; the First Plan alone must teach a new player to play for two hours without outside help. |
| Shell and comfort | Main menu; named manual saves plus three rotating period-end autosaves; pause, date, and speed controls; minimal settings; action-needed notifications and event log; camera polish; an in-HUD onboarding strip; local panic log and autosave-on-crash. |
| Presentation and audio | Zero-spend art and audio; grounded palette-controlled presentation; bounded visible citizens; day/night and visible seasons; legible state feedback and refusal feedback; UI feedback, ambience, and optional menu-only music. |
| Distribution and audience | English-only, fixed keybindings, no separate accessibility line item, no telemetry, and CI-built Linux and Windows binaries shipped as an unlisted itch build. The shell may be friends-grade; visuals and game feel must pass a stranger-grade finish bar. |

The implementation posture is lean systems, maximal polish, and opportunistic breadth only where
reuse makes it cheap. Terrain, water, and hydro are the deliberate breadth exception. Performance
targets 250,000 citizen identities at 60 fps on the development machine; the relevant
implementation and release plans define the benchmark gates. A green test suite or save
round-trip alone does not establish this target.

During development, save formats may take explicit version-gated hard breaks. From the 1.0 release
candidate onward, released saves remain compatible. Every completed product rung requires current
implementation evidence and an inspected capture or played-session acceptance appropriate to the
work; prose, task state, or inherited completion markings cannot substitute for that evidence.

## Explicit cuts

The following are committed Post-1.0 direction and cannot receive 1.0 acceptance criteria:

- loyalty, legitimacy, broadcast, monuments, crime, vehicle manufacture, and vehicle fuel lifecycle;
- voltage tiers and grid-depth features including transformers, treatment tiers, and combined heat
  and power; electric-heating fallback; passenger rail, signals, and electrification; ships, docks,
  pipelines, cableways, containers, aircraft, and petrochemicals;
- era calendar, dual currency, free terraform, cell-level water, kindergarten, deathcare,
  epidemics, perishables, refrigerated transport, Steam, and marketing.

Tourism, hotels, and attractions are never in scope. Fires and disasters are never in scope:
scarcity, not random destruction, is the intended pressure source.

## Scope discipline

An in-scope system cannot be marked implemented from a legacy document, a generated roadmap, a
task handoff, or an old ADR. Completion requires current implementation evidence plus the
acceptance evidence required by its specification. A proposed feature outside this charter belongs
in Post-1.0 direction or a future charter revision, never in a 1.0 requirement by implication.

## Current-reality boundary

Current substrate evidence already records gaps against this destination: the economy fact-sheet
labels trade and persistent unmet demand as violations, while the logistics fact-sheet labels a
physical truck transfer seam as provided but custody, recovery, and coherent fulfillment as
partial, conflicting, or absent. See [economy fact-sheet](../research/fact-sheets/wave1-economy.md)
and [logistics fact-sheet](../research/fact-sheets/wave1-logistics.md). These findings constrain
the rewrite; they do not narrow the binding scope.
