# Information

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** simulation
**Last verified:** 2026-08-28

Scope: **1.0 — charter row Planner interaction** (inspect-depth feedback); the four-realities
model and representation error are design proposals.

## What this is

Reports are not truth. The simulation holds four distinct realities:

| Reality | What it holds | Example |
|---|---|---|
| **Physical** | Stocks, custody, citizens, households, buildings, sites, routes, queues, runs, utility flows, actual attendance and illness | 35 tonnes of copper sitting in a plant's yard |
| **Institutional** | What organisations declare, request, record, or believe | The plant's reported requirement of 140 tonnes |
| **Planner** | What reaches THE PLANNER through reports, dashboards, inspection, measurement | "Requested 140, received 126, discrepancy flagged" |
| **Lived** | What people experience: search, queues, warmth, commute, crowding, childcare, health access, fatigue, workplace pressure, informal access, opportunity | 33 hours per week to fill a food basket (CIA, 1954 Moscow) |

A formal Plan can report success while lived conditions deteriorate. Plots, blat, local
adaptation, and enterprise welfare can make life tolerable while the metrics look weak. The
game must enforce this separation.

Information is a resource. Better reporting, monitoring, and reliable institutions improve
planning quality without magically improving supply (design law 8). The Planner's instruments
for learning the physical truth are inspection, measurement, institutional channels, and time —
never an omniscient debug view of `Simulation`.

## 1.0 requirement

The production specification requires inspectable discrepancies:

> The Planner SHALL infer suspected deception from inspectable request, receipt, consumption,
> on-hand, surplus, and outstanding-request-age discrepancies; no authoritative `dishonest`
> flag may replace those observations.
> — [`SPEC-PRODUCTION-009`](../../reference/specifications/production.md#spec-production-009)

This rule means the Planner's view exposes quantities, not verdicts. The Planner compares
numbers and draws conclusions; the simulation does not label an enterprise "dishonest".

## Target design

The design proposes that every Planner-visible value declares **how it is known** — measured,
reported, aggregated, observed via an institution, estimated, or unknown (design bible §4,
§13.20). The normal UI consumes a `PlannerSnapshot`, not raw `Simulation` state. No
omniscient player UI is permitted (design law 9).

```text
Material-balance inspector — one line per entry:
  Item  | Reported  | Physical | Source      | Provenance
  Steel | 140 t     | 126 t    | Enterprise  | reported
  Steel | 126 t     | 126 t    | Inspector   | measured
```

A discrepancy between "reported" and "measured" values is the Planner's primary signal for
institutional dishonesty. The Planner sees the discrepancy, never the reason. An enterprise
that inflates needs has reasons — remembered reliability, reserve targets, bargaining history —
and those live in the simulation, hidden from the Planner's view, not absent
(SYNTHESIS §6, "enterprise intent state" conflict resolution: rich hidden *state*, no hidden
*verdict*).

### Representation error (HYPOTHESIS)

The design thread proposes comparing physical reality, lived experience, enterprise report,
union report, local-Soviet report, and Planner belief as six channels, each with a measurable
**representation error** (design bible §8.12). Political institutions become a sensor network,
not a popularity minigame. This is an extension of the four-realities model and remains a
hypothesis; no specification commits to it.

## Current substrate

One reality exists: `Simulation` state, read directly by the UI. `native_app/` holds
`Arc<RwLock<Simulation>>` and reads approximately forty resources directly
(`native_app/src/game_loop.rs:33`, Lane E, E-036). No `PlannerSnapshot` exists; no information
restriction exists; no provenance metadata is stored.

The building inspector (`inspect_building.rs:150-267`) shows workers, productivity, power,
progress, and storage capital per item. The human inspector (`inspect_human.rs:17-80`) shows
location, destination, house, last-ate, and work building. No STATUS/CAUSE/TREND is displayed.
`Market::requested()` is a public API that `native_app/` does not call.

## Research basis

The four-realities model captures a structural fact of Soviet planning: reported output could
diverge from physical output by substantial margins. Harrison (2011) documents
plan-fulfilment falsification at the enterprise level, including grain overstatement of up to
53%. The CIA analytic record (Lane B2) distinguished between physical data and reported data
when assessing Soviet economic performance. The game's information model is grounded in this
documented reality.

## Open questions

- Which resources may the Planner *not* see directly? The specification requires inspectable
  discrepancies, not omniscience.
- How does the provenance column interact with the `PlannerSnapshot` architecture? Each
  snapshot value needs a provenance tag at the type level.

## Related

- [Authority](authority.md) — one owner per value; information has its own authority chain.
- [Physical causality](physical-causality.md) — the eight states that reports can misrepresent.
- [Adaptation](adaptation.md) — enterprises adapt their reports as well as their buffers.
- [Reports and information](../planned-economy/reports-and-information.md) — the domain instance.
- [Design bible §4](../../vision/design-bible.md) — the four realities.
- [Causal inspector proposal](../../plan/proposals/causal-inspector.md) — worked examples.
