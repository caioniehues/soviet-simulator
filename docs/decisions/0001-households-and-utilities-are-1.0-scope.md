# ADR-0001: Households and Utilities are 1.0 charter rows

**Kind:** decision
**Authority:** binding
**Status:** accepted
**Owner:** project lead
**Last verified:** 2026-09-03
**Date:** 2026-09-03
**Decision makers:** project lead (caioniehues), interviewed by the lead agent

## Context and problem

The 1.0 charter table ([`charter-1.0.md`](../plan/charter-1.0.md) §1.0 scope) had nine rows and
no row for Households/citizens or for Utilities. Yet the charter's own identity section presupposes
both: "colder homes" and "going without", "persistent individual identities", "Water is a utility,
never cargo", "landfill and incinerator", and a 250,000 citizen-identity performance target. The
charter also states that a feature outside the table is "never in a 1.0 requirement by
implication". The charter was therefore self-inconsistent.

Downstream pages resolved the inconsistency in different directions:

- [`product/scope-1.0.md`](../product/scope-1.0.md), [`vision/design-bible.md`](../vision/design-bible.md)
  §17 and [`index.md`](../index.md) bound households, housing shortage and electricity, water,
  sewage, heating and waste as 1.0.
- [`product/post-1.0.md`](../product/post-1.0.md) deferred water pressure, head and tank storage,
  while [`simulation/infrastructure/water.md`](../simulation/infrastructure/water.md) and the
  design bible treated the static-head solver as 1.0 binding.
- [`simulation/planned-economy/reliability-and-buffering.md`](../simulation/planned-economy/reliability-and-buffering.md)
  claimed "1.0 binding" for adaptive request inflation, planning credibility and the ratchet, which
  `post-1.0.md` defers; [`reserves.md`](../simulation/planned-economy/reserves.md) used
  "1.0 candidate", a label defined nowhere.

The 2026-09-03 documentation review filed this as `sov-m7r`. Substrate evidence gathered for the
decision (read-only, 2026-09-03):

| Area | State | Evidence |
|---|---|---|
| Citizens in houses | System runs each tick; no capacity, queue or shortage | `simulation/src/init.rs:111-114`, `simulation/src/map_dynamic/binfos.rs:45`, `simulation/src/souls/desire/home.rs:24` |
| Electricity | Scheduled; binary blackout halts productivity | `simulation/src/init.rs:53`, `simulation/src/map_dynamic/electricity.rs:43-93`, `simulation/src/tests/scenarios/inflation.rs:142-153` |
| Water, sewage, heating, waste | Absent: no building kind, system, prototype or test | `simulation/src/map/objects/building.rs:18-23`, `base_mod/companies.lua`, `simulation/src/init.rs:25-95` |
| Water pressure, tanks, reservoir/hydro | Absent; heightfield terrain exists without hydrology | same |
| Glossary | No `Household`, `Utility`, `Water` or `Reservoir` entry | `docs/reference/glossary.md` |

Nothing in question is implemented, so the decision is about commitment, not about protecting
existing code.

## Decision drivers

- The charter's identity section and model rules bind the game's shape; a table that omits what
  the identity requires cannot be satisfied.
- "Lean systems, maximal polish": every added network is a new solver, a new building kind and a
  new failure surface.
- Domestic clearing is queue, allocation, substitution and going without, never price; a shortage
  without a queue would violate that rule.
- Scope labels must be checkable: a page's scope claim should be traceable to a charter row or it
  is wrong by construction.

## Considered options

1. **Revise the charter**: add a Households/citizens row and a Utilities row so the table matches
   the identity section.
2. **Demote everywhere**: strip households, housing shortage and non-water utilities from every
   page and keep only what the nine rows literally say.
3. **Reinterpret**: declare the identity section binding on its own and leave the table as is.

## Decision outcome

Option 1, with these bindings:

1. **Households and citizens row.** Persistent citizen identities grouped into households;
   residence assignment with a housing queue and an observable housing shortage; household
   consumption with explicit going without. Housing tiers (kommunalka to separate flat), propiska
   and household time budgets are Post-1.0.
2. **Utilities row.** Electricity, water, heating and waste. **Sewage is Post-1.0** and joins the
   explicit cuts; it has no charter anchor, and pollution-to-basin coupling can source from industry
   directly.
3. **Water in 1.0 includes static head and tank storage.** Connection alone must not guarantee
   service; the tree-based static-head solver and drain-before-failure tank delay are the mechanism
   that distinguishes water from electricity.
4. **Reliability and reserves are split.** Shortage propagation and physical stock with inspectable
   surplus are 1.0 (already bound by the charter). Adaptive request inflation, planning credibility,
   the ratchet and the five-class custody-state model are Post-1.0 hooks.
5. **Scope vocabulary.** A page's scope line uses exactly one of two labels: **1.0**, which must
   name the charter row it derives from, or **Post-1.0**, which may carry a *hook* note meaning
   "avoid an architectural dead end; build nothing". The label "1.0 candidate" is retired.
   "Never in scope" remains for the charter's permanent exclusions.

Why: demoting would make the charter's own identity statements unfulfillable and leave the 250k
performance target with nothing to count; reinterpreting keeps the contradiction forever. Four
networks with charter anchors is already the ceiling of the lean posture, and sewage is the one
whose absence no player would notice in a two-hour First Plan.

## Consequences

- The charter table gains two rows and one explicit cut; `scope-1.0.md`, `index.md`, `post-1.0.md`
  and the design-bible scope rows are aligned to it.
- Pages whose scope line said "1.0 candidate" or claimed 1.0 for deferred proposals are corrected
  by the owning documentation sweeps (`sov-6uy`, `sov-8d1`, `sov-kvn`, `sov-bpp`).
- The glossary gains `Household` and `Utility`.
- Cost: four utility networks and a household model are now committed work with no substrate
  behind them. The migration sequence already places them in phases 5 and 7.
- Risk: the Utilities row is the largest single addition to 1.0; if it proves too expensive, the
  charter revision path is heating or waste to Post-1.0, never water.

## Confirmation

- `charter-1.0.md` §1.0 scope contains a Households and citizens row and a Utilities row naming
  exactly electricity, water, heating and waste; sewage appears under Explicit cuts.
- No active page under `docs/` carries the label "1.0 candidate", and every "1.0" scope line
  names a charter row (`python3 scripts/check_docs.py` remains green; the sweeps cite this record).
- The decision remains appropriate while the charter's identity section still names colder homes,
  persistent identities and Water as a utility.

## More information

- `bd show sov-m7r` and its comments record the interview and the substrate evidence.
- [`docs/architecture/migration-sequence.md`](../architecture/migration-sequence.md) phases 5 and 7.
- [`docs/simulation/infrastructure/water.md`](../simulation/infrastructure/water.md) §Target design
  for the static-head solver.
