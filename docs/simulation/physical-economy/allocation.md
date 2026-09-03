# Allocation

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** economy
**Last verified:** 2026-09-03

| Scope | 1.0 — charter row Transport and border |

## What this is

This page is a pointer, not a second definition. The allocation mechanic — how Logistics
selects a source without price — is documented once, canonically, in
[planned-economy allocation](../planned-economy/allocation.md). That page covers the target
deficit-first design (`SPEC-LOGISTICS-005`/`010`) and the current distance-only substrate
(`Market::make_trades`, `simulation/src/economy/market.rs:511-551`, domestic
`money_delta: Money::ZERO` at `market.rs:544`).

In the physical-sequence reading path, allocation sits between [Requests](requests.md) and
[Reservation](reservation.md): allocation selects the source; reservation encumbers its
stock. No separate physical-side allocation state machine exists in code — there is one
`make_trades`, and it is described on the canonical page.

## Open questions

See the canonical page. The physical-side open question is only how the deficit scale
interacts with the hoarding floor in `recipe_should_produce`
(`simulation/src/souls/goods_company.rs:32-50`).

## Related

- [Requests](requests.md)
- [Reservation](reservation.md)
- [Logistics](logistics.md)
- [Logistics spec](../../reference/specifications/logistics.md#spec-logistics-010)
- [Glossary](../../reference/glossary.md)
