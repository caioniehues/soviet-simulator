# Foreign Trade

> Superseded by ../../reference/specifications/trade.md — provenance only.

**Status:** draft model (grounded in research)
**Phase:** 1
**Primary inspiration:** W&R physical dual-currency trade, with a published (non-opaque) market model (OURS)
**Evidence:** see [research/trade.md](../research/trade.md); transport side in [research/logistics.md](../research/logistics.md) §C/§D/§E5.

> Money exists mainly for imports/exports/loans/foreign labour — the accounting boundary of the republic. Inside the border the economy is physical; at the border, goods become currency and currency becomes goods — but only when a real vehicle clears a real crossing.

## Purpose

Make foreign trade the *only* place money touches the physical economy — and even there, money is never sufficient. Buying a resource creates an order at customs; a vehicle must physically haul it inland. W&R states our rule verbatim in its own tutorial: *"You cannot instantly purchase resources in buildings. (You need bring them from customs.)"* (research/trade.md §B1 — CONFIRMED).

## Draft model

### Border crossings (from W&R — CONFIRMED grammar)

Trade endpoints are built, typed border buildings (research/trade.md §A):

- **Customs house** — per-transport-class pass-through (1-unit buffer per cargo class — a throughput point, not a stockpile), vehicle bays, an inward domestic edge and an outward border edge (`$CONNECTION_ROAD/RAIL_BORDER`). Road, rail, and air variants; player-buildable variants exist (`$SUBTYPE_OWN_CUSTOM`).
- **Utility border buildings** — electricity import/export transformers and border pipelines are separate typed endpoints (§A3). The transport medium of a good decides *which* border building it crosses at, same as domestically (INFERRED).
- Throughput is physical: bay count, fleet, and border-edge capacity throttle trade — no numeric cap token exists (INFERRED §E).

Contrast dropped: CS1's map edge as `OutsideConnectionAI` posting **unlimited priority-0 offers** — an infinite fixed-price shop-of-last-resort with no building to construct (§D1). That is the abstraction we reject.

### Dual currency (from W&R — CONFIRMED, adopted)

- Two ledgers: **roubles** (domestic/Eastern bloc) and **dollars** (hard currency). Every W&R vehicle costs `$COST_RUB` xor `$COST_USD` — 294 vs 62 files, zero both (§C1): currency is a property of a good's *origin*, not a runtime choice.
- Separate loans per currency with own interest and borrowing caps (§C2 — CONFIRMED from UI strings).
- The design consequence: you can be rouble-rich and dollar-poor and unable to buy Western equipment until you export for hard currency — "having money" is insufficient in a second, sharper way (§G).
- Exchange rate / conversion mechanic: not visible in W&R data (Gaps); our lean — no free conversion, dollars only via export or dollar loans.

### Pricing: dynamic world market, published model (W&R substrate + OURS fix)

- W&R prices are a **dynamic global market**, entirely native — zero price tokens in 488 .ini files; UI confirms live computation and event shocks (OPEC stabilises oil) (§C4 — CONFIRMED presence, CONFIRMED absence from data).
- CS1's alternative is a hardcoded constant table (`GetResourcePrice`: Grain 200 … LuxuryProducts 10000) — too static (§D2).
- **OURS:** adopt the moving market (reacts to era, world events, and our own export volume) but **publish the model** — expose the price curve and its drivers to the player, fixing W&R's black box (§G).

### Settlement (CS1 implementation pattern + W&R semantics)

Book trade as an explicit paired ledger entry (CS1's `EconomyManager` credit-seller/debit-buyer at delivery is a clean pattern — §D3), but: split by currency, tag import/export, and settle **only on physical border clearance** — treasury and simulation never diverge (§G).

### Catalogue gating (from W&R — CONFIRMED, adopted)

Goods and vehicles carry availability era + origin country (`$AVAILABLE 1969 1987`, `$COUNTRY` — §C1). The trade catalogue changes across the campaign timeline and with bloc alignment — trade as a geopolitical lever.

### Asset resale (from W&R — CONFIRMED, adopted)

Exported vehicles fetch full price only when new; condition discounts resale (§C3). Trade value = f(condition, market, era) — no churning new vehicles across the border for free money.

### Data (draft)

```
BorderCrossing { mode: road|rail|air|wire|pipe; bays[]; classBuffers[class→1u]; domesticEdge; borderEdge }
Treasury { roubles; dollars; loans: [{currency, principal, rate, penaltyRate}] }
Market { pricePerGood(t, currency); drivers: [era, events, ourExportVolume]; published: true }
TradeOrder { good; qty; direction; currency; priceAtOrder?; status: ordered→atCustoms→cleared }
```

## Open questions
- ~~Two currencies or one foreign-currency pool?~~ → two (roubles/dollars), per W&R's confirmed model.
- ~~Import contracts planned or reactive?~~ → partially open: orders are placed at customs (W&R shape), but *who* places them — plan quotas, or deficit-driven like spec/logistics.md dispatch? Lean: plan sets standing import/export contracts; logistics fulfils.
- Price at order time or at clearance time? (Moving market makes this a real hedging question.)
- Foreign labour (W&R has it) — defer to a later spec batch?
- Loans as pressure mechanic: adopt W&R's per-currency interest + penalty + cap as-is?

## Evidence log
| Claim | Evidence level | Source | Notes |
|---|---|---|---|
| W&R: resources must be physically hauled from customs; money alone does nothing | CONFIRMED | `sovietEnglish.btf` tutorial strings | research/trade.md §B1-B2 |
| W&R customs = per-class pass-through + border/domestic edges + vehicle bays | CONFIRMED | `zoll_sahy.ini:12-42` etc. (5 files) | §A2 |
| W&R utility trade via dedicated border transformers/pipelines | CONFIRMED | `eletric_transformator_custom*.ini`, `foreign_pipe_player.ini` | §A3 |
| W&R dual currency: every vehicle `$COST_RUB` xor `$COST_USD` (294/62/0) | CONFIRMED | `vehicles/*/script.ini` census | §C1 |
| W&R separate rouble/dollar loans with own interest | CONFIRMED | `sovietEnglish.btf` UI strings | §C2 |
| W&R prices are dynamic global market; zero price data on disk | CONFIRMED (+absence) | .ini census + UI strings | §C4 |
| W&R catalogue gated by era + country | CONFIRMED | `$AVAILABLE`, `$COUNTRY` | §C1 |
| W&R condition-dependent resale | CONFIRMED | `sovietEnglish.btf` | §C3 |
| CS1 border: unlimited priority-0 offers, budget/pathfind throttled | CONFIRMED | `OutsideConnectionAI.cs:967-1097` | §D1 — rejected |
| CS1 prices hardcoded constants; single currency | CONFIRMED | `IndustryBuildingAI.cs:1514-1538` | §D2 |
| CS1 settlement: paired EconomyManager credit/debit at delivery | CONFIRMED | `IndustryBuildingAI.cs:1424-1465`, `EconomyManager.cs` | §D3 — pattern adopted |
| Exchange rate / conversion mechanic | — (gap) | not in W&R data | native; our design open |
| Published (non-opaque) market model | OURS | — | §G |

Evidence levels: CONFIRMED · OBSERVED · INFERRED · SPECULATIVE · OURS (see [spec/README](README.md)).

## Related
- ../research/trade.md · ../research/logistics.md · ../spec/logistics.md · ../spec/vehicles.md · ../spec/resources.md · ../spec/electricity.md
