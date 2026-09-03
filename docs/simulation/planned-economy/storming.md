# Storming

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** economy
**Last verified:** 2026-09-03

Scope: 1.0 — charter row Resources and production — storming is a charter-scope temporal
mechanic (production with plan periods); no specification commits to it yet.

## What this is

Shturmovshchina is the three-phase monthly production cycle documented across Soviet industry
(CONFIRMED — multiple sources):

| Phase | Name | Character |
|---|---|---|
| First third | *Spyachka* (hibernation) | Slow start; inputs not yet arrived; previous-period cleanup |
| Middle third | *Goryachka* (hot time) | Accelerating; inputs arriving; pressure building |
| Final third | *Likhoradka* (feverish frenzy) | Maximum intensity; overtime; quality risks |

Late inputs, taut quotas, or delayed effort concentrate output near the period end. Storming
generates cascading consequences:

- **Freight pulses** — dispatch requests spike in the final third, overloading the transport
  system.
- **Overtime and fatigue** — workers extend shifts, reducing next-period productivity.
- **Rail and road congestion** — concentrated demand creates bottleneck queues.
- **Quality and rework risk** — rushed output fails quality checks, consuming additional inputs.
- **Household time pressure** — overtime reduces household discretionary time.
- **Distorted reports** — period-end output spikes make reporting unreliable.

Storming propagates **upstream recursively** (CONFIRMED as a valid construction from the
documented three-phase cycle, Lane A, A-09): a downstream enterprise's end-of-period spike
creates demand for inputs from an upstream enterprise, which then storms to meet that demand,
cascading further upstream. Space programme demand → electronics → wire → copper → mine → rail
congestion.

## 1.0 requirement

No specification commits to storming. It depends on plan periods, which do not exist in the
simulation. The charter's commitment to production and quotas implies that storming is in scope
once plan periods exist.

## Target design

The design proposes (Lane A, section 3c) a per-enterprise temporal demand profile:

```text
per enterprise:
  plan_period_ticks_remaining: u32
  plan_period_target: u32
  period_output_so_far: u32
  storming_state: enum { Normal, Storming }
  storming_multiplier: f32    // 1.0 normal, up to 1.5 under storming
```

When `period_remaining / total_period < threshold` AND `output_so_far / target < shortfall`,
the enterprise enters storming. Storming multiplies productivity by `storming_multiplier` but
also multiplies input consumption by the same factor — driving larger raw-material requests
into the logistics system — and degrades quality (if quality grades exist).

The test: an enterprise with a quota it cannot meet at normal rate enters storming in the
final third. Its input request rate increases. A downstream enterprise that was previously
adequately supplied now faces a demand spike and begins to lag on its own quota.

### Freight-plan stability

Two plans with equal annual tonnage need different fleets if one is smooth and one is pulsed.
The design proposes tracking mean corridor load, peak load, period variance, emergency-dispatch
share, empty repositioning, dock waiting, and missed loading windows — as metrics, without
invented values. The right fix may be the Plan, not more track (design bible section 5.6,
PLAUSIBLE).

## Current substrate

Production is continuous. `company_system` (`simulation/src/souls/goods_company.rs:192-218`)
advances `progress` by `productivity * DELTA / recipe.duration.seconds()` every tick. There is
no plan-period awareness, no deadline behaviour, no temporal bunching. No `storming_state`,
`storming_multiplier`, or period tracking exists.

`Government` holds only `money` (`simulation/src/economy/government.rs:9-11`). No quota system,
no plan period, no performance tracking.

## Research basis

Shturmovshchina is one of the best-documented phenomena of Soviet management (CONFIRMED).
The three-phase monthly cycle — spyachka, goryachka, likhoradka — is described in multiple
sources (Lane A, A-09, citing GlobalSecurity.org and Wikipedia on shturmovshchina). The
recursive upstream propagation is the design's own construction, but it follows logically
from documented supply-chain coupling and is consistent with the cascade pattern
(design bible section 11).

## Open questions

- How does storming interact with quality grades (OTK, Post-1.0)?
- Is the storming multiplier per-enterprise or per-recipe?
- How does the Planner diagnose storming from freight metrics alone?

## Related

- [Plan cycle](plan-cycle.md) — storming depends on plan periods.
- [Reliability and buffering](reliability-and-buffering.md) — storming and the ratchet interact.
- [Enterprise behavior](enterprise-behavior.md) — storming as an enterprise temporal response.
- [Phase lag](../concepts/phase-lag.md) — freight pulses propagate with delay.
- [Queues](../concepts/queues.md) — dock and corridor queues spike during storming.
- [Design bible section 5.5](../../vision/design-bible.md).
