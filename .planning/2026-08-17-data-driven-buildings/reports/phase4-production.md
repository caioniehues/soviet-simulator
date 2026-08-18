# Phase 4 — four production systems become two, and the inertness bug dies

`cargo test --lib`: **153 passed, 0 failed** (from 148 at `ea85655`). Five tests added, all
five watched failing first. Seven bench gates re-run and green. `capture_g1` re-run and
watched back.

## The change

`extract_resources`, `run_power_plants`, `run_factories` and `run_heat_plants` are gone.
`src/sim/production.rs` holds `produce_flows` and `produce_goods`, both driven by
`spec.recipe`, plus the `Gated { rate, bound_by }` component ADR 0014 asked for.

## Why two systems and not one

The plan said "one generic production pass". It cannot be one, and the reason is in the
schedule rather than in the code:

```rust
solve_power.after(run_power_plants).before(run_factories)   // wires.rs
```

The grid solve sits *between* generation and consumption so the plant has this tick's output
before the factory's gate is read. A single system cannot be both sides of that sandwich.

The split is data, not a match: a kind with a `flow_output` feeds a utility net and must
produce before the solves; a kind without one fills a yard and produces after. That is a new
column, `flow_output: Option<FlowOutput>`, holding `PLANT_OUTPUT_MW` and `HEAT_PLANT_OUTPUT`
— the two numbers the old systems had hard-coded. It is a balance number, so per the plan's
own rule it is grounded in behaviour (`a_fuelled_plant_pours_its_nominal_flow_and_an_empty_
one_pours_none`) rather than pinned.

## Hazard 1 from the gate: the heat plant's asymmetry dissolved

The gate warned that `run_heat_plants` burns flat with **no labour factor** while the other
three scale with staffing, so a generic loop would silently apply the labour curve to it.

It does apply the curve, and that is behaviour-preserving, because `labour_factor` returns
**1.0 when `workers_needed == 0`** (`labour.rs:62`) and the HeatPlant's row says
`workers_needed: 0`. The asymmetry was never in the curve; it was in which systems consulted
it. Running everything through one curve reproduces today exactly.

**The latent change, recorded rather than hidden:** if R4 gives the HeatPlant workers, its
burn starts scaling where before it would not have. That is almost certainly the wanted
behaviour, but it is a decision R4 now inherits rather than one this phase made.

## The sanctioned behaviour change, witnessed failing first

Both halves were written as tests before any production code moved, and both were watched
red against `ea85655`:

```
a_heat_plant_under_construction_burns_nothing_and_warms_nobody
  left: 9.959999   right: 10.0     ← 0.04 coal = HEAT_PLANT_COAL_BURN × 2 ticks
a_pump_under_construction_supplies_nothing
  left: true       right: false    ← the factory was watered by a pump on the scaffolding
```

The heat half is fixed structurally: both production queries carry `Without<ConstructionSite>`
once, where the old code applied it by hand at four sites and missed one.

The pump half could **not** be fixed there, and this is worth stating plainly because it
contradicts the plan's expectation. A pump has no recipe — it is a supplier, not a producer —
so it never passes through the production pass at all. `solve_water`'s own `plants` query
carries the filter instead. A structural fix in one place does not reach a system that
gathers its own entities.

## `Gated`, and why it is a component

ADR 0014's argument held up: the query requires `&mut Gated`, so a producer that never earned
one is skipped rather than run ungated. `a_producer_stripped_of_its_gate_produces_nothing`
proves it by removing the component from a live, producing mine and watching output stop.
Failing closed is the property a helper function every producer must remember to call cannot
have.

`availability` never touches an inventory — ADR 0014's seam, kept. The producer still burns
its own coal in `run_recipe`, including the partial-take behaviour a fuel-short plant has
always had: it burns what it got and generates nothing. Moving the shortfall check before the
take would leave that coal in the yard, which is a different game.

## Non-vacuity

Every new test was watched failing. Two mutations after the fact, on the code rather than the
pins:

| mutation | caught by |
|---|---|
| the power gate returns `Bound::Labour` instead of `Bound::Power` | `the_gate_names_what_is_binding`: `left: Labour  right: Power` |
| `FlowOutput::Power(mw)` pours `0.0` | `a_fuelled_plant_pours…`: `left: 0.0  right: 10.0` |

## Gates

```
cargo test --lib                    153 passed; 0 failed
bench_chain     mean 0.0730 ms  (gate 0.33 ms)   PASS
bench_citizens  mean 0.5293 ms  (gate 2 ms)      PASS
bench_dispatch  mean 0.3411 ms  (gate 2 ms)      PASS
bench_networks  mean 0.0438 ms  (gate 1 ms)      PASS
bench_sites     mean 0.0943 ms  (gate 2 ms)      PASS
bench_traffic   mean 1.1245 ms  (gate 16 ms)     PASS
bench_transit   mean 0.2368 ms  (gate 2 ms)      PASS
clippy --lib --tests: the same nine pre-existing warnings, none in changed files
```

`bench_networks` is the gate this phase could plausibly have moved — 220 powered, 220 watered,
200 heated. It reads **0.0438 ms** against the README's recorded 0.044 ms.

## The strongest evidence: the plan lands identically

`capture_g1` re-run and compared frame-for-frame against the committed reference at
`screenshots/result/g1`:

| | committed reference | this build |
|---|---|---|
| Coal | 40 t — 100% of 40 | 40 / 40 t |
| Households housed | 8 — 100% of 8 | 8 / 8 |
| Fulfilment | 100% | 100% |
| Treasury | 68 roubles | 68 roubles |

380 frames of simulation through a rewritten production pass converge on the same treasury
value. Frame 60 shows `Coal 0/40` with "3 of 3 sites blocked" — the inertness contract
visible in the running game.

**A trap for whoever captures next:** run captures with `cargo run --release --bin capture_g1`,
never the raw `./target/release/capture_g1`. The raw binary loses the asset root and renders
with no text and untextured ground. That looks exactly like an asset regression and is not one.

## Found, not acted on

- **`solve_power` has no `ConstructionSite` filter either**, but does not need one the same
  way: a plant that becomes a site is dropped by `produce_flows` and keeps its *last*
  `PowerOutput` rather than being zeroed. That is `run_power_plants`' pre-existing behaviour,
  reproduced deliberately. A demolish-to-site could leave stale megawatts on the grid for as
  long as the site stands.
- **`parts()` at `src/game/buildings.rs:213` is still the last wildcard-free match over
  `BuildingKind`.** This phase did not touch it, so the crate's only compile-time
  exhaustiveness guard survives — but `flow_output` is now a fourth column a new kind must
  fill, and nothing forces that beyond the struct literal.
- **`Bound` has no consumer yet.** R1's inspect panel is what it was built for; until then it
  is written every frame and read only by tests.
