# The production gate is a component computed once, not four hand-written chains

**Status:** decided 2026-08-17, **not yet built** — R1, [#117](https://github.com/caioniehues/soviet-simulator/issues/117).

Four production sites each re-derived their own Liebig chain and returned a bare scalar:
mine and quarry from staffing alone, the power plant from the minimum of staffing and fuel,
the factory from power AND water AND staff, the heat plant from fuel alone. Every one
computed which factor was scarcest and then discarded it, so no caller could learn why
output was zero — and R1's inspect panel needs exactly that.

Worse, the `Without<ConstructionSite>` inertness filter was applied by hand at each site
and applied inconsistently: mine, quarry, power plant, factory and labour assignment
carried it; `run_heat_plants` and `solve_water` did not. **A heat plant that was still a
construction site burned coal and heated homes at full rate**, as did a water pump. Nothing
would have failed to compile if the next production system forgot the filter too.

One gate system now runs before the producers and writes a `Gated { rate, bound_by }`
component onto every producer. Production systems multiply by the rate; the inspect panel
reads the bound factor without recomputing anything. A pure function each site *calls* was
rejected for the same reason the old code failed: it still requires every future producer
to remember to call it and to filter construction sites, where a component makes a
forgotten producer one with no `Gated` at all — no output, failing loudly rather than
silently working.

The seam is availability, not consumption. The gate reads the staffing curve, the power and
water gates, and whether fuel is present, and names the scarcest; it never takes anything
from an inventory. The producer still burns its own coal. That keeps the gate testable in
isolation and leaves consumption where it belongs.
