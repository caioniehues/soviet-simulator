# Electricity

> Superseded by ../../reference/specifications/electricity.md — provenance only.

**Status:** draft model (grounded in research)
**Phase:** 1
**Primary inspiration:** W&R explicit wire network + voltage tiers; CS1 pulse only as an amortisation pattern
**Evidence:** see [research/utilities.md](../research/utilities.md) §A/§C/§G/§H; power-as-gating-input in [research/production.md](../research/production.md) §A4.

> Physical electrical grid graph: generation depends on fuel + workers + equipment condition. Power is a gating input to nearly everything (spec/production.md). Blackouts are physical causality, not a coverage toggle.

## Purpose

A building has power only if it is physically wired — through hops the player laid — to a producer with spare capacity. Plants are full recipe buildings (fuel + workers + water → electricity + byproducts). No coverage field.

## Draft model

### Generation: plants are recipe buildings (from W&R — CONFIRMED)

W&R power plants are ordinary production buildings emitting `eletric` (research/utilities.md §C2): coal plant `$CONSUMPTION coal 1.2` → `$PRODUCTION eletric 70` with 20 workers; nuclear consumes fuel + quality-gated cooling water + workers + professors and outputs electricity + spent fuel + sewage pollution. Adopted: generation obeys spec/production.md wholesale — no fuel, no staff, no cooling water ⇒ no output. (CS1's plants are also fuel-throttled but funding-scaled — the funding gate is dropped, as everywhere.)

### Transmission: explicit two-tier network (from W&R — CONFIRMED grammar)

- Two voltage tiers, HIGH (transmission) and LOW (distribution), with directional typed connection points (`$CONNECTION_ELETRIC_HIGH/LOW_INPUT/OUTPUT` — §C1).
- Typed chain: `plant → HIGH lines → transformer → LOW lines → substation → buildings` (§C2, topology INFERRED from types).
- **Dropped: CS1's cell-grid model** — 256×256 cells, flood-fill pulse, "powered iff the pulse reaches your cell", power lines as capacity-free conductivity brushes (§A1–A4). Coverage-and-connectivity-only is the abstraction we reject.

### Capacity and loss (OURS — W&R hints, not file-confirmed)

W&R has no `$CAPACITY`/`$LOSS` token (Gaps) — the solver is native. We make it explicit: each line class has a throughput rating and per-km transmission loss; HIGH lines lose less. Plant siting and substation placement then matter physically (§H).

### Consumption and shortfall (from W&R — CONFIRMED, via production.md)

Consumers draw `$CONSUMPTION_PER_SECOND eletric` continuously, with idle/lighting fallback draws (research/production.md §A4 — not re-derived). Deficit allocation among consumers on a starved subnetwork is native in W&R (Gaps) — **OURS:** planner-set priority classes (hospitals > housing > industry), with brownout before blackout.

### Solver performance (CS1 pattern — CONFIRMED, adopted as implementation note)

CS1 amortises its grid solve over a 256-frame cycle (§A3). Whatever our graph solver is, it must be budgeted per tick the same way (§H). See `architecture/simulation-clock.md`.

### Border trade

Electricity crosses the border as a wired utility link via dedicated import/export transformer buildings (§C4) — pricing/currency owned by spec/trade.md.

### Data (draft)

```
Plant → ordinary ProductionBuilding emitting `electricity` onto its HIGH output points
Line { tier: HIGH|LOW; capacity; lossPerKm }
Transformer { HIGH↔LOW; capacity }   Substation { LOW input; served consumers }
Consumer += { drawPerSecond; idleDraw; priorityClass }
```

## Open questions
- ~~Graph flow model: real load-flow or capacity buckets?~~ → capacitated graph flow per subnetwork (not electrical load-flow physics; not CS1 coverage). Exact solve: per-subnetwork balance with edge capacities.
- Deficit allocation: priority classes (lean) vs proportional brownout everywhere?
- Time-of-day demand curve — couple to citizen schedules (spec/citizens.md) or a simple day/night factor first?
- Storage (none in either game's data) — pumped hydro/batteries as era tech?

## Evidence log
| Claim | Evidence level | Source | Notes |
|---|---|---|---|
| CS1: 256×256 cell grid, flood-fill pulse; powered iff pulse reaches cell | CONFIRMED | `ElectricityManager.cs:327-1067` | §A1-A3 — rejected as model, kept as amortisation pattern |
| CS1: power lines paint conductivity; no per-line capacity | CONFIRMED (absence) | `PowerLineAI` → `ElectricityManager.cs:1069` | §A4 |
| CS1: plant output worker/funding-scaled, fuel-throttled | CONFIRMED | `PowerPlantAI.cs:191-201` | §A5 |
| W&R: two voltage tiers, directional typed connection points | CONFIRMED | `$CONNECTION_ELETRIC_*` census | §C1 |
| W&R: plants are full recipe buildings (fuel+workers+water→power+byproducts) | CONFIRMED | `powerplant_coal.ini:3-18`, `powerplant_nuclear_single.ini:9-34` | §C2 |
| W&R: transformer/substation chain | CONFIRMED types, INFERRED topology | `eletric_transformator.ini`, `eletric_substation.ini` | §C2 |
| W&R: power is a continuous gating input to consumers | CONFIRMED | `$CONSUMPTION_PER_SECOND eletric` (39 files) | production.md §A4 |
| W&R: per-edge capacity/loss absent from data (solver native) | CONFIRMED (absence) | no `$CAPACITY`/`$LOSS` token | Gaps |
| Explicit line capacity + distance loss; priority-class deficit allocation | OURS | — | §H |

Evidence levels: CONFIRMED · OBSERVED · INFERRED · SPECULATIVE · OURS (see [spec/README](README.md)).

## Related
- ../research/utilities.md · ../research/production.md · ../spec/production.md · ../spec/heating.md · ../spec/waste.md · ../spec/trade.md
