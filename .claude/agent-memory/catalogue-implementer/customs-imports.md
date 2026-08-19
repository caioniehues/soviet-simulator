---
name: customs-imports
description: How customs.rs imports were wired onto the existing StoragePolicies band mechanism, and the catalogue default row that keeps old sell-everything behaviour
metadata:
  type: project
---

Implemented 2026-08-19: `buy_imports` in `src/sim/customs.rs`, same
`SimStage::ProductionAndUtilities` stage as `sell_exports`, same
`Without<ConstructionSite>` filter. Both systems now read `&StoragePolicies`
on the CustomsOffice entity instead of `sell_exports` draining the yard
unconditionally.

**The trap**: `StoragePolicies::surplus`/`deficit` return `0.0` via
`map_or(0.0, ...)` when a resource has **no band at all** (`None`), not just
when the band is `(0,0)`. The CustomsOffice catalogue row in
`src/sim/catalogue.rs` had `default_policies: &[]` — no bands, not
zero-bands. Wiring `sell_exports` straight onto `.surplus()` without fixing
the catalogue row would have silently stopped the office from selling
anything by default and broken the existing "goods hauled in become
roubles" test. Fix: catalogue row now bands all three `ResourceKind::ALL` at
`(0.0, 0.0)` explicitly — min 0 means no default import demand, max 0 means
everything already-in-yard reads as surplus, reproducing the old
sell-everything behaviour exactly. Two `PINNED_COLUMNS` test tables in
catalogue.rs both needed updating for this row (one drives
`default_policies_column_holds_its_pinned_values` or similar — check both
before assuming one edit covers it).

Prices: `import_price(kind) = export_price(kind) * 2.0`, no separate price
table. Rate: `IMPORT_RATE = SALE_RATE` (0.25 t/tick), one dock counter
shared by both directions conceptually but tracked as two separate
per-office ticks (sell budget and buy budget don't share the 0.25, each
system gets its own).

Ping-pong avoidance is automatic, not something to special-case: because
`StorageBand::new` clamps `min_pct <= max_pct`, a resource can never be both
below its min and above its max at once, so `sell_exports` and
`buy_imports` never fight over the same tonnage in the same tick.

Treasury never goes negative: `buy_imports` bounds the request by
`treasury.roubles / price` (`affordable`) before calling `inventory.add`,
so even though `add()` can return less than requested (shared-capacity
cap), it can never return more — the debit is always ≤ what's on hand.

See [[catalogue-field-design]] and [[catalogue-layout]] for the wider
catalogue conventions this fits into.
