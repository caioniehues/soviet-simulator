# Simulation concepts

**Kind:** index
**Authority:** advisory
**Status:** draft
**Owner:** simulation
**Last verified:** 2026-08-28

## What belongs here

This section defines the cross-cutting ideas that recur across every simulation domain. Each
concept page explains one idea, states the design principle it serves, and links to the domain
pages that apply it. A domain page names its instance and links back here; it does not re-explain
the general form.

These are design concepts, not specifications. They bind nothing. The charter binds scope, the
glossary binds terms, and ratified specifications bind mechanism.

## What does not belong here

Domain-specific mechanics (enterprise buffering, household scheduling, dispatch priority) belong
in their own subtrees (`planned-economy/`, `society/`, `infrastructure/`). Architecture proposals
belong in `docs/plan/proposals/`. Historical research belongs in `docs/research/`.

## Reading path

1. [Authority](authority.md) — the one-authority-per-transition law.
2. [Physical causality](physical-causality.md) — goods move or do not; states are distinct.
3. [Scarcity](scarcity.md) — non-price clearing and the mechanisms that replace it.
4. [Queues](queues.md) — queues as first-class scarcity objects.
5. [Reserves](reserves.md) — the five-purpose taxonomy and storage as a floor on hoarding.
6. [Phase lag](phase-lag.md) — physical momentum in disruption and recovery.
7. [Reliability](reliability.md) — the reliability-buffering spiral.
8. [Information](information.md) — the four realities and reports as resources.
9. [Adaptation](adaptation.md) — every actor adapts; stable things sleep.
10. [Social reproduction](social-reproduction.md) — the Plan-to-labour loop.

## Authoritative documents

- [Charter 1.0](../../plan/charter-1.0.md) — binding scope.
- [Glossary](../../reference/glossary.md) — binding terms.
- [Design bible](../../vision/design-bible.md) — curated vision (explanatory, not binding).
- [Specification register](../../reference/specifications/README.md) — the authority table.

## Related

- [Planned economy](../planned-economy/index.md) — domain instance of most concepts here.
- [Design bible §2](../../vision/design-bible.md) — the twenty design laws.
- [Architecture proposals](../../plan/proposals/) — how concepts map to code.
