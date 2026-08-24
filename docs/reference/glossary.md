# Soviet Simulator glossary

**Kind:** reference
**Authority:** binding
**Status:** active
**Owner:** project lead
**Last verified:** 2026-08-24

This glossary defines project terms. It does not establish implementation behavior, acceptance
criteria, or completion status; those belong to the charter, specifications, and substrate
evidence.

## The Planner and the plan

**Planner**
: The player role: an author of planned allocation and construction decisions under scarcity.
  _Avoid:_ mayor, entrepreneur, market actor

**Plan**
: An authored sequence of quota periods on one continuous save.
  _Avoid:_ campaign, scenario, mission

**Quota**
: A state-set production, provision, or housing requirement for a plan period.
  _Avoid:_ goal, objective, target

**Tranche**
: A period-end allocation of border roubles scaled by quota performance; it may be leaner, but it
  is never a game-over condition.
  _Avoid:_ budget, grant, payout

**Rouble**
: The single foreign currency used only at a border customs clearance. Domestic clearing is not a
  monetary market.
  _Avoid:_ domestic money, cash, funds

## Placement and construction

**Ghost**
: The uncommitted placement representation that shows a footprint, material bill, and any refusal
  reason.
  _Avoid:_ blueprint, phantom

**Verdict**
: One stated judgment of a proposed placement: approved or refused with a physical reason.
  _Avoid:_ validation result

**Refusal**
: A rejected player action with an explicit physical reason.
  _Avoid:_ error, invalid action, failure

**Material bill**
: The physical resources required to complete a site; it is not a domestic rouble price.
  _Avoid:_ price, build cost

**Site**
: A placed but unfinished building that receives material and work until completion.
  _Avoid:_ works, build

**Ground broken**
: The threshold after which a site has received its first material or work and is physical rather
  than only ordered.

**Rescind**
: Withdraw a construction order before ground is broken.
  _Avoid:_ undo, cancel, revert

## Production, allocation, and movement

**Binding constraint**
: The explicitly identified scarcest physical gate on a process, such as labour, input, power, or
  water.
  _Avoid:_ bottleneck, blocker

**Request**
: A stated demand for a physical resource. A request is distinct from allocation, reservation,
  receipt, consumption, and stock on hand.
  _Avoid:_ purchase order

**Custody**
: The accountable holder of a physical quantity between pickup and delivery.

**Dispatcher**
: The service that reserves and assigns a vehicle identity to a haulage job.

**Border**
: The customs boundary where imports and exports clear at fixed per-kind rouble prices and goods
  must physically cross.
  _Avoid:_ market, trade hub, port

**Going without**
: An explicit unmet-need outcome under scarcity. It is a simulation state, not game termination.
  _Avoid:_ game over

## Decision inputs

**Policy**
: A planner-authored value read by simulation decisions and persisted with the save.
  _Avoid:_ setting, preference, config

**Player control**
: A presentation or pacing control, such as simulation speed, that is not a policy unless the
  simulation reads and persists it as one.
