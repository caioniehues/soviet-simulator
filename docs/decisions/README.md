# Decision register

**Kind:** reference
**Authority:** operational
**Status:** active
**Owner:** project lead
**Last verified:** 2026-09-03

This directory contains decisions ratified for the Rust/Egregoria fork. A decision is binding only
when its status is **accepted** and it records its context, decision, consequences, and
confirmation evidence.

## Pre-fork ADR status

All ADRs written before the fork are historical records. They are archived pending individual
re-ratification and do not establish current mechanism, scope, or completion. A useful old idea
may be re-ratified only by a new or explicitly superseding decision that checks it against the
current substrate and charter.

No new decision is created merely to preserve an old ADR number or conclusion. The charter binds
scope; the glossary binds terminology; specifications bind in-scope mechanism; `bd` binds task
state.

## Lifecycle

1. Draft records an unresolved choice and its evidence.
2. Accepted records the chosen option, consequences, owner, and confirmation method.
3. Superseded points to the accepted decision that replaced it.
4. Archived preserves historical rationale without current authority.

Use [the decision template](../templates/decision.md). Do not use an ADR to document a task,
generated status, or an unverified code observation.

## Accepted decisions

| Id | Title | Date |
|---|---|---|
| [ADR-0001](0001-households-and-utilities-are-1.0-scope.md) | Households and Utilities are 1.0 charter rows | 2026-09-03 |
| [ADR-0002](0002-fixture-world-is-a-materialised-replay.md) | The fixture world is a materialised replay, never an authored save | 2026-09-03 |
