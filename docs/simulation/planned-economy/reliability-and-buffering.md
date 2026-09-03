# Reliability and buffering

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** economy
**Last verified:** 2026-09-03

| Scope | Label |
|---|---|
| Shortage propagation and going without | 1.0 — charter identity + Resources row |
| Adaptive request inflation, planning credibility, the ratchet | Post-1.0 hook |

The shortage spiral is the game's core mechanic and ships in 1.0. The three named mechanisms
below are PLAUSIBLE design proposals with CONFIRMED historical grounding, deferred as hooks by
[ADR-0001](../../decisions/0001-households-and-utilities-are-1.0-scope.md).

## What this is

This is the domain instance of [reliability](../concepts/reliability.md) applied to the
planned economy. Enterprises, households, hospitals, wagon fleets, and construction sites
all buffer against unreliable delivery. The spiral runs in both directions: unreliable delivery
causes hoarding, which worsens scarcity; reliable delivery reduces hoarding, which releases
stock.

Three domain-specific mechanisms drive the spiral:

### Adaptive request inflation (Lane A, section 3a)

The design proposes per-enterprise state that drives request inflation from experienced
reliability:

```text
per enterprise:
  reliability_memory: f32      // EMA of fulfillment_rate
  effective_multiplier: f32    // base_multiplier / max(reliability_memory, floor)
```

After each recipe cycle, `fulfillment_rate` updates from received / requested. Low reliability
raises the effective multiplier; high reliability lowers it. The test that proves it:
two enterprises start with `request_multiplier = 1`; one has deliveries artificially delayed;
after N cycles, the delayed enterprise's effective multiplier is strictly higher.

### Planning credibility (Lane A, section 3d)

The design proposes (concept CONFIRMED — Gregory and Harrison 2005; numbers UNSUPPORTED) a
global credibility record:

```text
global (in PlanningAuthority):
  credibility: f32               // EMA, range 0 to 1
  fulfilled_promise_count: u32
  broken_promise_count: u32
  confiscation_memory: f32
  mid_period_revision_count: u32

per enterprise:
  trust_in_plan: f32             // blend of global credibility and own reliability_memory
```

Low `trust_in_plan` increases `effective_multiplier` (more hoarding), decreases voluntary
reporting accuracy, and increases propensity for local workshops. The test: three confiscations
measurably drop credibility and raise subsequent requests from the confiscated enterprises.

The four behavioural indicators: if the Plan always cuts requests, enterprises inflate; if the
Planner confiscates reserves, enterprises hide stock; if overfulfillment raises the next quota,
enterprises conceal capacity; if delivery is reliable, safety stocks shrink.

### The ratchet (Lane A, section 3b)

Heroic overfulfilment in one period raises the quota for the next. Revealed slack becomes
obligation. Enterprises conceal capacity to avoid the ratchet (CONFIRMED — Weitzman 1980;
Berliner 1957).

```text
per enterprise, per plan period:
  quota: u32
  actual_output: u32
  quota_history: RingBuffer<(u32, u32), 8>
```

Auto-quota: `max(quota, actual_output) * growth_factor`. The staircase is visible in a
timeline. The Planner can override below the ratcheted level to rebuild trust, at the cost
of output.

**Era caveat** (Lane G-32): the ratchet was strongest under Stalinist planning (1930s-1950s)
and weakened after the 1965 Kosygin reforms. The game's fixed 1950s-60s era means the ratchet
is strong. The design notes this historical context.

### Indicator design (CONFIRMED — Nove)

What the Plan measures becomes what enterprises optimise: tonnage targets produce heavy goods;
unit targets favour simple variants; fulfilment percentage drives storming; rail tonnage
neglects awkward consignments. The design proposes introducing an indicator only when its
physical consequence is represented (design bible section 5.11).

## Current substrate

The spiral seed exists. `request_multiplier` is a static `i32` on the `Recipe` prototype
(`prototypes/src/types/recipe.rs:52`), set to 4 for `flour-factory` and 3 for `meat-facility`
(`base_mod/companies.lua:40,582`), defaulting to 1 for all others
(`slaughterhouse` declares no multiplier, `base_mod/companies.lua:526-543`). It is wired
end-to-end in `recipe_init` (`simulation/src/souls/goods_company.rs:22-26`).

No `reliability_memory`, `fulfillment_rate`, credibility record, quota, or ratchet state exists
in the simulation. `Government` holds only `money: Money`
(`simulation/src/economy/government.rs:9-11`). No plan period exists.

The storage-capacity floor on hoarding is CONFIRMED:
`recipe_should_produce` (`goods_company.rs:44-47`) refuses to buy above
`amount * (storage_multiplier + 1)`.

## Open questions

- Adaptive multiplier or Planner-set request limits?
- Are plan periods player-defined or emergent?
- Three confiscations versus a continuous credibility function?

## Related

- [Reliability](../concepts/reliability.md) — the general concept.
- [Enterprise behavior](enterprise-behavior.md) — the dishonest enterprise.
- [Reserves](reserves.md) — confiscation as a credibility-costly Planner act.
- [Storming](storming.md) — storming interacts with the ratchet.
- [Plan cycle](plan-cycle.md) — period boundaries drive credibility and ratchet updates.
- [Design bible section 5.2-5.4, 5.11-5.12](../../vision/design-bible.md).
