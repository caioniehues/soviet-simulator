# Proposal — the causal inspector and the Planner information boundary

**Kind:** decision (draft)
**Authority:** advisory — binds nothing until accepted as a numbered decision
**Status:** proposed
**Owner:** project lead
**Date:** 2026-08-28
**Feeds:** the charter's "persistent identities and observable state let the Planner understand and correct the dishonest-enterprise loop"; `SPEC-PRODUCTION-009`; the missing observability specification

## Context

The dishonest enterprise is wired (`request_multiplier` → `recipe_init` → `set_requested`) and the
Planner cannot see it: no UI reads `Market::requested()`. The design thread proposes a
STATUS / CAUSE / TREND / POLICY / PHYSICAL CHAIN inspector and a drillable material balance; no
specification covers either. Lane G found that "every aggregate clickable down to real trains"
has an uncosted index behind it, and the lead found that the thread's discrepancy example shows
consumed and on-hand quantities as bare facts, contradicting its own "reports are not truth" law.

## Decision proposed

1. **Every Planner-visible value carries provenance**: measured · reported by enterprise ·
   aggregated · observed via institution · estimated · unknown. A panel line without provenance
   is a defect. This is the code-level form of the four realities.
2. **The inspector contract** for any significant object is the five lines; the physical chain is
   walked from recorded causal facts with parent links, never reconstructed.
3. **First increment:** requested vs received vs consumed vs on-hand vs oldest-open-request on the
   building inspector, each line labelled — "reported" for requested, "measured at delivery" for
   received, "inferred from stock change" for consumed. About thirty lines of UI plus one
   accessor. This is the minimum viable core loop.
4. **Drill-down** links aggregates to representative physical examples first; full entity
   drill-down is budgeted separately.
5. **Notifications** derive from causal state; the first two are request-inflation rising and
   repeated period-end storming (once periods exist).

## Alternatives

- Show physical truth in the UI (today's behaviour). Rejected by design law 9; collapses the
  information game.
- Build the full observatory before any panel. Rejected: the first increment needs no
  infrastructure and proves the thesis.

## Consequences

Provenance becomes a field on every snapshot value type; the observatory and change journal
follow ([architecture](../../architecture/observatory.md)); a specification for observability is
owed ([missing specs](../../developer/adding-a-specification.md)).

## Validation

A scenario that inflates one enterprise's request and asserts the inspector model exposes the
discrepancy with the right provenance labels; an inspected frame of the panel.

## Open for the Planner

Which resources may the Planner never see directly? Does the Planner see `capital` (physical) or
only reports plus deliveries?

## Related

- [Causality (architecture)](../../architecture/causality.md) · [Snapshots](../../architecture/snapshots.md) · [Reports and information (design)](../../simulation/planned-economy/reports-and-information.md) · [Lane E §3](../../research/conversation-mining-2026-08-28/E-code-gap-matrix.md)
