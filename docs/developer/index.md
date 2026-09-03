# Developer guide

**Kind:** index
**Authority:** operational
**Status:** active
**Owner:** project lead
**Last verified:** 2026-08-28

Task-oriented how-tos. Each page answers "how do I…" and links out for the why.

## Belongs here

Getting started, the repository tour, reading the docs, and one page per recurring task.

## Does not belong here

Architecture explanation, design rationale, standards (those pages say *what* must be true; these
say *how to do it*), process orchestration (`docs/process/`).

## Pages

| Task | Page |
|---|---|
| Build, run, test | [Getting started](getting-started.md) |
| Find your way around the crates | [Repository tour](repository-tour.md) |
| Read the documentation without being misled | [How to read the docs](how-to-read-the-docs.md) |
| Add a simulation system | [Adding a system](adding-a-system.md) |
| Add a resource (item) | [Adding a resource](adding-a-resource.md) |
| Add a building or company | [Adding a building](adding-a-building.md) |
| Write or revise a specification | [Adding a specification](adding-a-specification.md) |
| Write a test that proves a spec claim | [Writing evidence tests](writing-evidence-tests.md) |
| Find why two runs diverge | [Debugging determinism](debugging-determinism.md) |
| Profile the simulation or renderer | [Profiling](profiling.md) |
| Benchmark a change | [Benchmarking](benchmarking.md) |

## Reading path for a new contributor

1. Getting started → 2. Repository tour → 3. How to read the docs → 4. the
[charter](../plan/charter-1.0.md) and [glossary](../reference/glossary.md) → 5. the
[development cycle](../process/development-cycle.md) → 6. `bd ready`.

## Related

- [`CLAUDE.md`](../../CLAUDE.md), [`AGENTS.md`](../../AGENTS.md) — the agent entry points
- [Engineering standards](../engineering/index.md)
- [Architecture handbook](../architecture/index.md)
