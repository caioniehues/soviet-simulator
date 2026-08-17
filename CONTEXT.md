# Soviet Simulator

A socialist planned-economy city, infrastructure, logistics and society simulator. The
player is **the Planner**: quotas arrive from above, means are scarce below, and nothing
in the world teleports. This file is the glossary — what the project's words mean, and
which words we have decided not to use. It holds no implementation detail; the
[1.0 charter](docs/charter-1.0.md) holds scope and [`docs/adr/`](docs/adr/) holds decisions.

## The Plan

**Plan**:
An authored sequence of quota periods running on the game clock. Three ship in 1.0, on one
continuous save, before procedural endless mode.
_Avoid_: campaign, scenario, mission

**Quota**:
A production or housing target the state sets for the current period.
_Avoid_: goal, objective, target

**Tranche**:
The allocation of roubles the state pays out at period rollover, scaled by how well the
quota was met. Missing a quota means a leaner tranche, never a loss.
_Avoid_: budget, grant, payout

**Rouble**:
The single foreign currency, spendable only at the border. There is no internal money —
the domestic economy is allocated, not traded.
_Avoid_: cash, money, funds

## Placement — the planner's hands

**Ghost**:
The preview of a thing about to be placed, shown under the cursor before the click commits.
It carries its footprint, its material bill, and its refusal reason if it has one.
_Avoid_: preview, blueprint, phantom

**Verdict**:
The judgment on a proposed placement: approved, or refused with a reason. One verdict
answers both the ghost and the commit, so the two can never disagree. See ADR 0009.

**Refusal**:
The game declining a player action, always with a stated physical reason. A refusal is a
designed teaching moment, not an error condition.
_Avoid_: error, invalid, failure

**Material bill**:
What a site demands in matter to complete, and the only sense in which a building has a
cost — buildings are never priced in roubles. See ADR 0010.
_Avoid_: price, build cost

**Setback**:
The clear ground a building keeps from its neighbours and from road corridors. Also the
fixed distance frontage snap holds a building back from the road it aligns to.

**Rescind**:
Withdrawing a construction order before ground is broken on it. Distinct from demolition,
which destroys something physical and costs accordingly. See ADR 0008.
_Avoid_: undo, cancel, revert

**Rebuild**:
Restoring the road segment most recently cut, re-paying its material. The opposite
operation to rescind, and deliberately not merged with it.
_Avoid_: undo, redo, restore

**Ground broken**:
The threshold a construction site crosses when the first materials are delivered to it or
the first work is done. Before it, a site is only an order; after it, it is physical.

**Frontage snap**:
Placement orienting a building to a nearby road's tangent at a fixed setback, so buildings
line a street rather than scattering. See ADR 0007.

**Node snap**:
A new road segment's endpoint grabbing an existing road node within a radius, so the road
topology never acquires near-miss junctions. See ADR 0007.

## The world

**Site**:
A placed but unfinished building, consuming delivered materials until it completes.
_Avoid_: construction, works, build

**Dispatcher**:
The system that assigns vehicles to haulage jobs. Nothing moves without one dispatching it.

**Border**:
Where the planned economy meets the outside world: a customs office that sells imports and
buys exports at fixed prices, in roubles. Everything bought there physically drives in.
_Avoid_: market, trade hub, port
