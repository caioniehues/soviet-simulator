# Workplace representation — the representation-error idea

**Kind:** concept
**Authority:** advisory
**Status:** draft
**Owner:** society
**Last verified:** 2026-09-03

| Scope | Label |
|---|---|
| Representation error | Post-1.0, HYPOTHESIS |
| Institutional confidence | Post-1.0, HYPOTHESIS |

All content on this page is Post-1.0 and HYPOTHESIS.

## What this is

The Planner does not observe the world directly. Every piece of information passes through an
institutional channel — enterprise reports, union reports, local-Soviet reports — and each
channel introduces bias. The representation-error model measures that bias: the gap between
what is physically happening and what the Planner sees.

This is the four-realities model applied to institutions. It gives the
[information concept](../../concepts/information.md) a testable form.

## Target design

### Six channels — HYPOTHESIS (bible §8.12)

The design proposes six channels through which workplace conditions reach the Planner:

| Channel | Source | Bias |
|---|---|---|
| Physical reality | The simulation state | Ground truth; the Planner cannot read it directly |
| Lived experience | What workers actually experience | Accurate for the individual; incomplete aggregate |
| Enterprise report | What management reports upward | Inflated output; suppressed problems; storming hidden |
| Union report | What the trade-union committee reports | Welfare-focused; may amplify housing/safety; may suppress production issues |
| Local-Soviet report | What the district deputy reports | Service-focused; captures complaints management ignores |
| Planner belief | What the Planner actually sees | The result of filtering through all channels |

Each channel has a measurable bias. The enterprise report inflates output figures (confirmed:
CIA documented grain-output overstatement of up to 53 %; Lane G-16, B2-16). The union report
amplifies welfare issues. The local-Soviet report captures what both miss. The Planner's
belief is never the full picture.

### Institutional confidence — HYPOTHESIS (bible §8.13)

When a citizen or institution submits a complaint through a channel, one of two things
happens: action is taken, or it is not.

- **Effective channels** encourage further reporting. Citizens who see their complaints acted
  on continue to report.
- **Ineffective channels** encourage exit, absenteeism, and informal adaptation. Citizens who
  see complaints ignored stop reporting and adapt privately — through blat, through changing
  jobs, through reducing effort.

The design proposes that institutional confidence is channel effectiveness measured over time:
complaints submitted versus complaints acted on. A channel with high confidence aggregates
better information. A channel with low confidence produces silence — and silence looks like
agreement to the Planner.

This is a generalisation of [planning credibility](../../planned-economy/reliability-and-buffering.md#planning-credibility)
concept applied to citizen-state interaction. It is never loyalty, never a happiness meter,
never mind-reading.

### Connection to the four realities

The representation error IS the gap between:
- **Actual physical** — what the simulation computes
- **Reported institutional** — what enterprise and institutional reports say
- **Planner knowledge** — what the Planner's UI displays
- **Household lived experience** — what citizens actually experience

Each institution is a filter with known, measurable distortions. The design proposes that the
Planner's information quality depends on the institutional channels available and their
confidence levels. Investing in institutional capacity (union independence, local-Soviet
standing commissions, safety inspection) improves the Planner's information — but each
investment costs resources and creates new biases.

## Current substrate

No representation error exists. One reality: `Simulation` state, read directly by the UI.
No `PlannerSnapshot`, no information restriction, no report filtering. The Planner has
perfect information about everything.

## Research basis

- Bible §8.12: the six-channel model as the strongest new idea in §8.
- Bible §8.13: institutional confidence as channel effectiveness.
- Synthesis §3.5: the lead's verification of §8 as "historically defensible."
- CIA, "Soviet Statistical Falsification" (CIA-RDP85T01058R000507850001-1): enterprise-level
  falsification documented.
- Harrison (2011), "Forging Success: Soviet Managers and False Accounting, 1943 to 1962."

## Open questions

- How many channels does 1.0 need? The design proposes six; a minimal version might have
  only two (enterprise report and physical reality) with the gap as the observable.
- How does the Planner improve channel confidence? Direct investment in institutions, or
  emergent from how the Planner responds to reports?
- Should the inspector explicitly show "enterprise reports X; physical state is Y" as a
  standard contrast? The causal-inspector proposal adds a provenance column for this.

## Related

- [Soviet workplaces](soviet-workplaces.md)
- [Trade unions](trade-unions.md)
- [Local Soviets](local-soviets.md)
- [Information concept](../../concepts/information.md)
- [Labour](../labor.md)
- [Workplaces](../workplaces.md)
