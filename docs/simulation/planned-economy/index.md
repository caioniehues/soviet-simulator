# Planned economy

**Kind:** index
**Authority:** advisory
**Status:** draft
**Owner:** economy
**Last verified:** 2026-09-03

## What belongs here

This section describes how the planned economy works as a simulation domain: the control loop
between the Planner and the enterprises, the mechanisms that distort it, and the instruments
the Planner uses to manage it. Every page names one major mechanic, separates design proposal
from current code, and labels its evidence.

These pages are domain instances of the cross-cutting concepts in
[`docs/simulation/concepts/`](../concepts/index.md). Enterprise buffering is one instance of
[reliability](../concepts/reliability.md). Priority dispatch is one instance of
[scarcity](../concepts/scarcity.md). The concept pages explain the general form; these pages
apply it.

## What does not belong here

Architecture proposals belong in `docs/plan/proposals/`. Historical research belongs in
`docs/research/`. Specifications belong in `docs/reference/specifications/`. Society-side
mechanics (households, housing, labour, demographics) belong in `docs/simulation/society/`.

## Reading path

1. [Plan cycle](plan-cycle.md) — the control loop.
2. [Material balance](material-balance.md) — the accounting identity.
3. [Enterprise behavior](enterprise-behavior.md) — the dishonest enterprise as the core loop.
4. [Reports and information](reports-and-information.md) — what the Planner sees.
5. [Reserves](reserves.md) — custody states that sum to physical stock.
6. [Priorities](priorities.md) — priority relocates scarcity.
7. [Reliability and buffering](reliability-and-buffering.md) — the spiral and its instruments.
8. [Storming](storming.md) — shturmovshchina and temporal demand profiles.
9. [Allocation](allocation.md) — how allocation clears without price.

## Authoritative documents

- [Charter 1.0](../../plan/charter-1.0.md) — binding scope (Resources and production; Transport
  and border).
- [Glossary](../../reference/glossary.md) — binding terms (Planner, Quota, Rouble, Request).
- [Production specification](../../reference/specifications/production.md) — draft.
- [Resources specification](../../reference/specifications/resources.md) — draft.
- [Logistics specification](../../reference/specifications/logistics.md) — draft.
- [Trade specification](../../reference/specifications/trade.md) — draft.

## Related

- [Simulation concepts](../concepts/index.md) — the cross-cutting ideas this section applies.
- [Design bible §5](../../vision/design-bible.md) — the planned economy as a control system.
- [Lane A](../../research/conversation-mining-2026-08-28/A-economy-control-loop.md) — mechanism
  validation and data-structure sketches.
- [Economy fact-sheet](../../research/fact-sheets/wave1-economy.md) — current substrate.
