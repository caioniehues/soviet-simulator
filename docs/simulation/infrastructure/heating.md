# Heating

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** infrastructure
**Last verified:** 2026-09-03

| Scope | 1.0 — charter row *Utilities* |

## What this is

Heating is a finite thermal-flow utility. It transfers produced heat through its own
connected infrastructure — distinct from the road graph, distinct from the electricity
network. Shortfall becomes observable warmth shortage. Electricity cannot substitute for
physical heat.

Heating has the longest inertia of the piped utilities. When the coal supply stops, the
heating plant's output drops. Hot water already in the pipe continues flowing for a while
(pipe FIFO). Building interiors cool slowly (thermal mass). Hours pass before apartments
get cold. That delay is the mechanic: the Planner has time to act, but must act before
the consequences propagate.

## 1.0 requirement

`SPEC-HEATING-001` — Heating SHALL be sole authority for thermal topology, endpoint
attachment, generation offer, buffer, pipe/pump capacity, declared loss, transfer
progress, and served/unmet thermal rate. Electricity MUST NOT satisfy heat shortfall.

`SPEC-HEATING-002` — `G + D = V + C + L`. Buffer updates as `B_next = B + C - D`.

`SPEC-HEATING-003` — variable temperature demand MAY be evaluated only after ratified
Weather supplies the referenced observation. Before then, Heating uses declared static
demand.

`SPEC-HEATING-006` — one immutable `HeatAllocationID` per network per tick. Replay is
a no-op.

`SPEC-HEATING-007` — generation `G` derives from a Production-owned plant result
accepted once.

## Target design

Pipe FIFO + building thermal ODE (PLAUSIBLE, D §3.6):

**Per pipe:** `FIFO<(temperature: f32, volume: f32)>`

**Per building:** `{T_interior: f32, C_thermal: f32, Q_demand: f32}`

**Tick:**
1. Source pushes `(T_source, flow_rate * dt)` into pipe FIFO
2. Pipe FIFO pops from the other end when total volume > pipe volume (transport delay)
3. Building receives heat: `Q_in = flow_rate * c_p * (T_supply - T_return)`
4. Building thermal ODE: `T_interior += (Q_in - Q_loss) / C_thermal * dt`
5. `Q_loss = U * A * (T_interior - T_exterior)` (heat loss to environment)

The Planner sees: coal supply stops → source temperature drops → pipe FIFO delivers old
hot water for a while → building interior temperature drops slowly → eventually apartments
are cold. Hours of delay before visible failure.

Coal grades (HYPOTHESIS, D §4.8): not all coal is equal. Lignite has roughly half the
calorific value of anthracite. The current prototype and resource model treats coal as
uniform.

## Current substrate

No heating kind or registered heating/weather system exists
(`simulation/src/map/objects/building.rs:17-37`, `simulation/src/init.rs:52-70`).
The only utility flow is aggregate binary electricity blackout. This is entirely greenfield.

No weather system or spec exists. `SPEC-HEATING-003` requires a ratified Weather interface
before variable demand. The weather/hydrology spec is identified as missing in SYNTHESIS §7.

## Research basis

District heating node method (ResearchGate, Benonysson et al.): tracks transport delay by
calculating time for water mass to move from source to consumer. Inlet temperature
propagated with pipe heat loss (PLAUSIBLE, D §3.6).

W&R reference: heating plants consume coal and produce heat. Buildings have
`$HEATING_ENABLE/$HEATING_DISABLE`. No pressure or temperature model — binary connectivity
only.

## Open questions

- Which heat sources and endpoint classes are 1.0?
- Which Weather observation contract is required before variable demand?

## Related

- [Electricity](electricity.md)
- [Water](water.md)
- [Production](../physical-economy/production.md)
- [Network architecture](network-architecture.md)
- [Heating spec](../../reference/specifications/heating.md)
- [Phase lag](../concepts/phase-lag.md)
