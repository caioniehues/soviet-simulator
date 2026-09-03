# Reserves

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** simulation
**Last verified:** 2026-08-28

Scope: **1.0 candidate** — the charter requires physical stock and inspectable surplus;
the five-class taxonomy is a PLAUSIBLE design proposal (Lane A §3e).

## What this is

A reserve is stock held against future need. Every actor in the simulation holds reserves for
different reasons: an enterprise buffers against unreliable delivery, a household stores food
against shop shortages, a hospital stockpiles medicine, a wagon fleet keeps spare vehicles, and
the state maintains strategic stocks.

The design proposes a five-purpose taxonomy. All five classes sum to physical stock; none can go
negative; transfer between classes is an explicit action, not a balance:

| Class | Purpose | Who draws it | Planner visibility |
|---|---|---|---|
| **Operating** | Current recipe cycle consumption | Automatic (recipe_act) | Full |
| **Safety** | Minimum buffer below which requests increase | Automatic (with a recorded event) | Full |
| **Enterprise** | Hidden surplus the enterprise does not report | Never automatic — it *is* the hoard | Inferrable from inspection |
| **State** | Planner-allocated strategic reserve | Planner action only | Full |
| **Project** | Earmarked for a national project | Project system only | Full |

This taxonomy is PLAUSIBLE. Soviet practice had state reserves, enterprise buffer stocks, and
operational inventory (Lane A). The exact five-class split is the design thread's invention.
The open question is whether five classes are too many for the player; two or three may carry
the dishonest-enterprise loop equally well. The hidden enterprise reserve is the essential class
for the core gameplay loop.

## 1.0 requirement

The production specification requires that requested, received, consumed, on-hand, reserved,
and surplus are distinct quantities
([`SPEC-PRODUCTION-003`](../../reference/specifications/production.md#spec-production-003)).
The dishonest-enterprise inference rule requires that the Planner can detect excess on-hand
stock against consumption
([`SPEC-PRODUCTION-009`](../../reference/specifications/production.md#spec-production-009)).
The five-class breakdown is not yet in a specification.

## Target design

The design proposes (Lane A §3e) that per enterprise, per item:

```text
operating + safety + enterprise_reserve + state_reserve + project_reserve
    == physical_stock
```

`recipe_act` consumes from operating first. If operating is depleted, the enterprise draws from
safety (with a credibility penalty — this signals unreliability to the Planner).
`enterprise_reserve` is never drawn automatically — it is the hoard. `state_reserve` moves only
by a Planner action (confiscation or reallocation). `project_reserve` moves only by a
national-project system.

The Planner can compute the hidden reserve from physical stock minus the four declared
classes — if they inspect closely enough. The enterprise's own report omits it. That is
[`SPEC-PRODUCTION-009`](../../reference/specifications/production.md#spec-production-009) made
concrete.

Confiscation is a Planner act with a credibility cost: seizing enterprise reserves degrades
planning credibility, which increases future hoarding
(see [reliability and buffering](../planned-economy/reliability-and-buffering.md)).

### The network-reserves table

The reserve concept extends beyond goods to every network in the simulation. Each network has
a natural-unit reserve measure:

| Network | Reserve measure |
|---|---|
| Roads | Unused lane capacity (hours at current load) |
| Rail | Headway slack, spare wagons |
| Logistics | Inventory days, fleet idle fraction |
| Water | Tank volume (hours at current draw) |
| Sewage | Empty buffer capacity, treatment headroom |
| Heating | Hot-water thermal mass (hours at current burn) |
| Electricity | Generation reserve, ramping headroom |
| Gas | Linepack (hours at current draw) |
| Reservoir | Stored head (days at current release) |
| Society | Discretionary time, spare housing and service capacity |

"Coal bunker: 18 hours at current burn" tells the Planner more than a percentage bar. A
republic can function while every reserve is dangerously low — but one disruption away from
cascade failure. The expert Planner manages reserves across all networks, in natural units
(design bible §10, Lane D §4.18).

## Current substrate

`SingleMarket` tracks `capital` (on-hand stock as `i32`), `reserved` (matched but not yet
picked up, as `u32`), and `requested` (declared need, as `u32`)
(`simulation/src/economy/market.rs:39-53`). There is no distinction between types of on-hand
stock. A single `i32` represents all stock at an enterprise.

The storage-capacity floor on hoarding is CONFIRMED in code. `recipe_should_produce`
(`simulation/src/souls/goods_company.rs:44-47`) refuses to buy when:

```
capital - reserved >= amount * (storage_multiplier + 1)
```

An enterprise cannot hoard what it cannot store. Storage construction is therefore a
Planner-visible hoarding signal: an enterprise that keeps enlarging its warehouse while
reporting shortage is telling on itself (SYNTHESIS §3.2, lead finding).

## Open questions

- Five reserve classes or three? The essential class for the core loop is the hidden enterprise
  reserve. Operating and safety may suffice for the Planner's inspection; state and project
  reserves add depth but also cognitive load.
- How does the Planner confiscate reserves? A direct action, or an allocation rule that draws
  from enterprise surplus above a threshold?

## Related

- [Scarcity](scarcity.md) — reserves are a tool against scarcity.
- [Reliability](reliability.md) — unreliable delivery drives reserve accumulation.
- [Physical causality](physical-causality.md) — reserves sum to physical stock (conservation).
- [Enterprise behavior](../planned-economy/enterprise-behavior.md) — the dishonest enterprise
  hides surplus in the enterprise reserve class.
- [Planned-economy reserves](../planned-economy/reserves.md) — the domain instance.
- [Production specification](../../reference/specifications/production.md) — SPEC-PRODUCTION-009.
