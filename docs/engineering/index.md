# Engineering standards

**Kind:** index
**Authority:** operational — standards state what new code must or should do; a standard becomes binding for a rule only where it says so and where a decision or the existing process backs it
**Status:** active
**Owner:** project lead
**Last verified:** 2026-08-28

These pages are rules, not essays. Each says what new code **must** or **should** do, why in one
line, and how it is checked. Where a rule depends on an unaccepted architectural decision, the
page says so and the rule is *recommended*, not required.

## Belongs here

Rules for writing Rust in this repository, for determinism, authority, transitions, testing,
performance, benchmarking, dependencies, serialisation, failure modelling, observability and
documentation.

## Does not belong here

Design rationale (simulation tree), architecture description (architecture handbook), process
(`docs/process/`), task-oriented how-tos (developer guide).

## The standards

| Standard | One line |
|---|---|
| [Rust](rust.md) | Typed IDs, explicit ownership, no hot-path heap, explicit state machines, measured dependencies, justified `unsafe` |
| [Authority](authority.md) | One owning module per mutable authoritative field; references, results, intents across seams |
| [Determinism](determinism.md) | Stable iteration and ties, keyed RNG, idempotent transitions, deterministic merge, canonical digest, repeat-run tests |
| [Simulation transitions](simulation-transitions.md) | Immutable IDs, atomic conservation, replay no-op, one authority per transition |
| [Testing](testing.md) | Every guard seen failing; evidence tests named for their spec; mutation policy; no vacuous filters |
| [Performance](performance.md) | Stable state sleeps; filter cheaply, think expensively; justify any full scan; cache by revision |
| [Benchmarking](benchmarking.md) | Whole-world before micro; recorded commands and results; no numbers without a run |
| [Dependencies](dependencies.md) | `cargo-deny` policy; pinned git sources; prove leverage before adding a crate |
| [Serialization](serialization.md) | Envelope, versioned migrations, `slotmapd` invariant, no silent defaults |
| [Failure model](failure-model.md) | What is waiting, why, since when, who owns it, what recovers it — never a bare `failed: bool` |
| [Observability](observability.md) | Provenance on every Planner-visible value; causal facts; no omniscient reads |
| [Documentation](documentation.md) | The five states; inspect before asserting; update current-substrate with the code |

## Existing policies these standards defer to

- [Development cycle](../process/development-cycle.md) — phases, roster, gates
- [Dependency policy](../process/dependency-policy.md) — `cargo-deny` baseline and CI
- [Mutation policy](../process/mutation-policy.md) — where mechanical evidence is worth its cost
- [Document authority](../meta/document-authority.md)

## Related

- [Architecture handbook](../architecture/index.md)
- [Developer guide](../developer/index.md)
