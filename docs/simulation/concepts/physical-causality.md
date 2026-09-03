# Physical causality

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** simulation
**Last verified:** 2026-08-28

Scope: **1.0 binding** — the no-teleport pillar and its consequences are charter commitments
(charter §Identity, lines 29–30; design laws 1–6).

## What this is

Goods move physically or they do not move. Allocation, matching, payment, route creation, and
reservation never teleport stock. This is the first design law and the foundation that every
domain mechanic rests on.

The law has six consequences, each a distinct rule:

1. **Goods move physically or do not move.** (charter)
2. **Request, allocation, reservation, pickup, custody, delivery, on-hand, and consumption
   are separate states.** No state is evidence that another occurred
   ([`SPEC-PRODUCTION-003`](../../reference/specifications/production.md#spec-production-003);
   glossary "Request").
3. **Failure persists.** A missing stock, vehicle, route, dock, worker, watt, litre, dwelling,
   school place, or clinic slot creates a visible waiting, partial, stalled, substitution, or
   going-without state. It never ends the game. (charter)
4. **No silent deletion.** Goods, demand, citizens, vehicles, queues, and sites do not vanish
   because a transaction failed. (design law 4)
5. **No domestic price clearing.** Scarcity resolves by policy, queue, priority, substitution,
   rationing, reserve, adaptation, or going without. The rouble is border foreign currency
   only. (charter, lines 32–33)
6. **Physical opportunity cost is visible.** Prioritising one use removes actual capacity,
   materials, labour, transport, housing, or service access from another. (design law 6)

## 1.0 requirement

The charter binds rules 1, 3, and 5 directly. The production specification binds the
eight-state chain:

> Requested, received, consumed, on-hand, reserved, in-custody, and surplus quantities are
> distinct. A reported request is never proof of consumption.
> — [`SPEC-PRODUCTION-003`](../../reference/specifications/production.md#spec-production-003)

The logistics specification binds pickup and delivery conservation:

> For pickup quantity `x`, the atomic transition is `H_source -= x`, `R_source -= x`,
> `C_haul += x`.
> — [`SPEC-LOGISTICS-006`](../../reference/specifications/logistics.md#spec-logistics-006)

The trade specification binds settlement to physical customs clearance:

> Domestic matching, allocation, reservation, dispatch, production, and consumption MUST NOT
> debit, credit, rank by, or otherwise clear through roubles.
> — [`SPEC-TRADE-001`](../../reference/specifications/trade.md#spec-trade-001)

## Target design

The design proposes that every flow through the simulation respects the eight-state chain.
Input flow is: request → allocation → reservation → pickup → custody → delivery → on-hand →
consumption. Output flow begins at production, remains in producer custody until a separate
logistics delivery, and never becomes another holder's stock at match time (design bible §6).

Custody conservation means that post-pickup cancellation cannot "release" cargo. The goods stay
in custody until physical return, reassignment, or delivery
([`SPEC-LOGISTICS-006`](../../reference/specifications/logistics.md#spec-logistics-006)).

## Current substrate

The truck dispatch leg is the code's actual strength. `DispatchState` sequences
ToSource → Loading → ToDestination → Unloading, with seller capital debited at Loading and
buyer capital credited at Unloading
(`simulation/src/economy/market.rs`, `advance_dispatches`).
Twelve ledger tests and fourteen retail tests prove this path
(Lane E, E-128).

The live violation is the **export-side teleport**. At `market.rs:774`, `*cap -= qty_sell`
debits seller capital at match time. No `Dispatch` is created for the export trade. Goods
vanish from the seller's stock the moment a match is found, without a truck driving them
anywhere. The import side was fixed by `sov-abs`: imports now go through dispatch from the
freight station.

Three further violations exist:

- Domestic money gates building construction (`world_command.rs:225`), worker wages
  (`economy/mod.rs:54`), and train spawning — contradicting rule 5.
- Auto-generated roadside lots (`map/map.rs:682-720`) contradict the planned-construction
  principle.
- `request_multiplier` is static per prototype, not adaptive — contradicting rule 4's
  requirement that demand persists and adapts.

## Related

- [Authority](authority.md) — one owner per state prevents two truths for one quantity.
- [Scarcity](scarcity.md) — non-price clearing is the domestic consequence of rule 5.
- [Reserves](reserves.md) — reserve classes must sum to physical stock (conservation).
- [Enterprise behavior](../planned-economy/enterprise-behavior.md) — the eight-state chain
  applied to the dishonest enterprise.
- [Design bible §2](../../vision/design-bible.md) — the twenty design laws.
- [Logistics specification](../../reference/specifications/logistics.md) — custody contract.
