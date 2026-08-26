---
name: ruling-inflation-source
description: sov-lpj verdict — request-inflation is a static Lua request_multiplier on the RECIPE; emergent hoarding rejected for 1.0; the hoard is BOUNDED not growing
metadata:
  type: project
---

Ruled 2026-08-26, Phase 0 consult on **sov-lpj**. Full text is the
`bd comments sov-lpj` entry authored `kornai-economist`.

## The ruling

**Static Lua-declared `request_multiplier` on the RECIPE table** (not the
company prototype — recipe is where `consumption` and `storage_multiplier`
already live, and `recipe_init`/`recipe_act` take `&Recipe` and nothing else).
Optional, **defaults to 1**; a required parse breaks all 26 companies.
Multiplier not absolute, so honesty stays expressible as exactly one value
that survives recipe rebalancing.

**Why:** the sim reads it once at `recipe_init` via `Market::set_requested`,
giving the panel a single readable source. Both existing
`.requested(soul, item.id).unwrap_or(item.amount as u32)` sites
(`goods_company.rs:24`, `:57`) become that value.

**Rejected — pure emergent inflation** (sim raises requests in response to
experienced shortage). Kornai is theoretically on its side (hoarding IS a
response to supply uncertainty), but: (a) every enterprise then inflates
identically, so the discrepancy becomes a constant offset rather than a
detection signal — the Planner cannot distinguish a liar; (b) it is a feedback
loop reading a shortage signal that is currently untrustworthy (see
[[finding-exttrade-teleport]]); (c) charter scope — it's a system, not a field.

**Rejected — per-company random roll at spawn.** Gives variance but nothing
learnable; the player must re-check every enterprise every time. Chores, not
inference.

**Post-1.0 path is deliberately preserved:** `request_multiplier` can later
become the FLOOR/propensity that the sim raises under experienced shortage,
with no data migration.

## The dynamics finding that shapes every acceptance criterion

**With `buy_until` (market.rs:414-420) a static multiplier produces a BOUNDED
hoard, not a growing one.** `buy_until` tops up to N and stops. Steady state
for consumption `c`, multiplier `k`: stock oscillates in `[k*c - c, k*c]`,
surplus is `(k-1)*c`, **constant**, reached after one delivery cycle.
A test asserting unbounded growth will fail.

True to Kornai anyway: the firm's target reserve is a stock, not a flow. What
grows without bound in Kornai is the *shortage the hoard causes elsewhere*.
The permanent withdrawal of `(k-1)*c` per hoarder is the gameplay.

## Five discrepancies (REQ-PRODUCTION-001) — 3 of 5 for 1.0

- **request** — `Market::requested()`, exists, this ticket gives it a value.
- **consumption** — derived from `recipe.consumption[i].amount`, no new state.
- **surplus** — derived `capital - reserved - consumption`. **No stored field**:
  a stored surplus can drift, and the point is that it is inferred.
- *deferred* **receipt** (cumulative-delivered accumulator, new save surface;
  not needed since surplus alone separates honest=0 from dishonest=(k-1)*c).
- *deferred* **age** — needs per-lot timestamps, i.e. capital becomes a queue
  not a scalar. Most expensive of the five, and the right long-term signal.

## Accepted weakness, stated openly

A static multiplier is not an *incentive*, it is a fact — the enterprise does
not choose to lie, it was authored lying. The Planner-facing half of the core
loop is fully real; the enterprise-facing half ("honesty gets you shorted") is
post-1.0. Accepted for 1.0; must be named in the bead, not discovered later.

## Hard constraints carried to sov-hoard-panel-mko

No honesty flag, no stored surplus field, no pre-computed inflation percentage.
The panel shows requested and true consumption as separate numbers; **the
player does the subtraction**. A badge would turn detection into a readout.

## W&R

No precedent field for "requests more than it consumes" — our invention,
confirmed. But W&R's split of `$CONSUMPTION` (rate) from `$STORAGE_IMPORT`
(inbound buffer) is the precedent for keeping declared request separate from
declared consumption. We add a third quantity W&R lacks: the *reported*
requirement.

Related: [[numbers-base-mod]], [[finding-exttrade-teleport]],
[[ruling-retail-dispatch]]
