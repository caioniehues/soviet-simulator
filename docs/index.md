# Soviet Simulator documentation

**Kind:** index
**Authority:** operational
**Status:** active
**Owner:** project lead
**Last verified:** 2026-09-03

Five minutes here answers what this project is, what is binding, what exists, and where to go.

## What is Soviet Simulator?

A socialist planned-economy city, infrastructure, logistics and society simulator, written in
Rust as a hard fork of Egregoria (2026-08-22). Its subject is **coordination under physical
scarcity**. Cities: Skylines gives you a city; Workers & Resources gives you a planned economy;
this game gives you the Plan as a *force* that deforms every physical system it touches, and a
society that must be physically reproduced for the economy to continue.

## Why is it unusual?

- **The dishonest enterprise is the core loop.** An enterprise requests more input than its recipe
  consumes and hoards the surplus. The player catches it from observable state. There is no hidden
  honesty flag — only discrepancies.
- **Nothing teleports.** Goods move on real trucks and trains or they do not move.
- **Never game over.** Failure becomes queues, shortages, substitution, colder homes and going
  without. It never ends the game.
- **No domestic price.** Scarcity clears by queue, allocation, substitution and going without. The
  rouble exists only at the border.
- **Information is a resource.** What the Planner knows, what enterprises report, what is
  physically true, and what households live are four different things.

## What is the player's role?

**THE PLANNER.** Not a mayor, not an entrepreneur, not a market actor. The Planner sets quotas,
priorities, allocation policy, construction programmes and reserves; institutions and citizens
adapt; the Planner reads imperfect reports and plans again. See [the player role](product/player-role.md).

## What does "physical planned economy" mean?

Every macroeconomic number resolves into physical or institutional state. A steel shortfall is a
missing input, worker, watt, litre, dock or route. A cold apartment is a heating chain. See
[simulation philosophy](product/simulation-philosophy.md) and
[physical causality](simulation/concepts/physical-causality.md).

## What is 1.0?

Fifteen domestic resources plus import-only Medicine; physical production and logistics; Food and
Meat as separate needs; persistent citizen identities grouped into households, with a housing
queue, an observable housing shortage and explicit going without; electricity, water (with static
head and tank storage), heating and waste as utilities; demographics including death; two
education tiers; healthcare; construction Sites; terrain, reservoir graph and hydro; minimal
freight rail; border trade in one rouble; three authored Plans on one save; 250,000 citizen
identities at 60 fps as the performance target. The charter binds it: [1.0 portal](product/scope-1.0.md),
[charter](plan/charter-1.0.md). Its eleven-row table is the complete list of 1.0 commitments; the
explicit cuts, and anything the table does not name, are [Post-1.0](product/post-1.0.md).
Households and citizens and Utilities became charter rows on 2026-09-03
([ADR-0001](decisions/0001-households-and-utilities-are-1.0-scope.md)).

## Where is what

| Question | Go to |
|---|---|
| What is binding, and in what order? | [Document authority](meta/document-authority.md) |
| How should a subsystem behave? | [Specifications](reference/specifications/README.md) — all currently `draft` |
| What does the code implement today? | Source and tests establish this. Start with [Current substrate](architecture/current-substrate.md) and its cited [fact-sheets](research/fact-sheets/wave1-economy.md) |
| What is the target architecture and how do we get there? | [Architecture handbook](architecture/index.md), [migration sequence](architecture/migration-sequence.md) |
| Why is a mechanic designed this way? | [Simulation knowledge tree](simulation/index.md), [concepts](simulation/concepts/index.md), [causal loops](simulation/causal-loops.md) |
| What does new code have to do? | [Engineering standards](engineering/index.md) |
| Where is the historical and technical evidence? | [Research](research/index.md) |
| Which mechanic lives where? | [Mechanics index](reference/mechanics-index.md), [authority index](reference/authority-index.md), [invariants](reference/invariants.md) |
| What was decided? | [Decisions](decisions/README.md) — [ADR-0001](decisions/0001-households-and-utilities-are-1.0-scope.md) is accepted; [proposals](plan/proposals/gosplan.md) are advisory |
| What is being worked on? | `bd ready`, `bd show <id>` — task state lives only in `bd` |
| How does work get done here? | [Development cycle](process/development-cycle.md), [`CLAUDE.md`](../CLAUDE.md), [`AGENTS.md`](../AGENTS.md) |
| What does a word mean? | [Glossary](reference/glossary.md) |

## How does a new contributor begin?

1. [Getting started](developer/getting-started.md) — build, run, test.
2. [Repository tour](developer/repository-tour.md) — the crates and where the simulation lives.
3. [How to read the docs](developer/how-to-read-the-docs.md) — the five states of knowledge.
4. Read the [charter](plan/charter-1.0.md) and the [glossary](reference/glossary.md).
5. Pick a task from `bd ready`, read its spec, then [add a system](developer/adding-a-system.md) or
   [write an evidence test](developer/writing-evidence-tests.md).

## For coding agents

Inspect source before claiming implementation. Read charter, glossary and the spec before changing
a subsystem. Check `docs/decisions/` and the tests. Use `bd` for task state. Research pages are
not binding. Update the docs — especially [current substrate](architecture/current-substrate.md)
— when you change a documented contract. The full rule is in `AGENTS.md`.
