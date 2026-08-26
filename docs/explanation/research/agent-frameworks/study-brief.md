# Brief — study four agent-engineering frameworks, then design ours

**Kind:** research brief
**Authority:** explanatory
**Status:** historical input
**Owner:** project lead
**Last verified:** 2026-08-24

Paste this whole file as the opening prompt of a fresh session, run from
`/home/caio/soviet-simulator`. Budget generously; this is a deep study, not a survey.

---

## Your job

1. **Study four agent-engineering frameworks from their current upstream source.**
2. **Design a custom framework for this project** that takes the best of each.

The output is a design document, not code and not an installed plugin.

---

## Step 1 — fetch each fresh. Do not study the local caches.

Local copies are pinned versions and some are months old. **Every one of these repos is actively
developed; the cached copy is a snapshot, not the framework.** Clone or fetch each to a scratch
directory outside this repo (e.g. `/tmp/fw-study/`), and record the commit SHA and date you read.

| Framework | Upstream | Author |
|---|---|---|
| **superpowers** | `https://github.com/obra/superpowers` | Jesse Vincent |
| **iterative-development** | `https://github.com/prime-radiant-inc/iterative-development` | Jesse Vincent |
| **mattpocock/skills** | `https://github.com/mattpocock/skills` | Matt Pocock |
| **compound-engineering** | `https://github.com/EveryInc/compound-engineering-plugin` | Kieran Klaassen, Tom Chow |

Note **superpowers and iterative-development share an author.** iterative-development's own manifest
calls itself *"an iterative implementation methodology that pairs with superpowers… designed for
comprehensive or ambiguous specs where the upfront writing-plans flow loses the plot."* They are not
rivals; one is the same author's deliberate alternative at a different scale. Read them as a pair.

**Read the skills themselves, not the READMEs.** A README states intent; the SKILL.md files state
mechanism. Where they disagree, the mechanism is the truth. For each framework, read at minimum its
router/orchestrator skill and every skill in its main flow.

---

## Step 2 — the project you are designing for

A Soviet planned-economy city-builder. Rust, ECS, a hard fork of Egregoria taken 2026-08-22. The
Bevy track that preceded it was discarded; the repo is GPL-3.0 by inheritance.

Scale and shape, because these decide which framework properties matter:

- **130 scheduled stories across 13 remaining iterations**, from a corpus of 149 stories / 370 ACs
  in the now-archived legacy iteration corpus.
- Single developer plus agents. **No users, no telemetry, no browser, no App Store, no PR review
  flow.** Task tracking is `br` (beads), not GitHub issues.
- `simulation/` ~15,400 lines · `native_app/` ~3,600 · `base_mod/*.lua` ~950.
- The design pillars: **nothing teleports** (goods move physically or not at all) and **never game
  over** (failure degrades into queues and shortages, never terminates).
- Determinism matters: the sim bincode-round-trips and hash-compares every tick.

Read these before designing: `CLAUDE.md`, `docs/process/development-cycle.md`,
`docs/plan/charter-1.0.md`, `CONTEXT.md`, `docs/plan/iterations/RESUME.md`, and `.claude/agents/`
(fifteen agent definitions).

---

## Step 3 — prior findings: test these, do not inherit them

A previous session reached the conclusions below. **They are leads, not facts.** This project has
repeatedly shipped ratified documents describing things that were never built, so re-derive
anything you intend to rely on. If you disagree with a conclusion, say so and show why — that is a
more valuable result than agreement.

**The decisive property was regression coverage.** With 13 iterations left, iteration 11 can
silently break iteration 2. Only iterative-development has an answer: a sentinel behavior corpus
re-run every iteration, and a definition of done as *"passing behavior evidence at the correct seam
for every externally observable requirement."* mattpocock and compound-engineering are both
per-feature flows with no cross-iteration regression story. That single property decided the
ranking. **Test it — check whether the current upstream of the other three has added one.**

**iterative-development's structural blind spot.** It extracts requirements from spec *prose* and
never checks them against code. Concretely, measured in this repo:

```
370 ACs carry a [SUBSTRATE: …] tag       103 of them cite an exact file:line
265 ABSENT   33 CONFLICTS   26 PROVIDED   19 PARTIAL   17 UNAUDITED   10 OURS
```

**Not one has ever been verified.** The format built the slot — it even has a tag named `UNAUDITED` —
and no step in the pipeline fills it. Every serious failure this session traced to a wrong substrate
claim.

**Its adversarial review is single-axis.** Sixteen PAR reports (2 reviewers × 8 domains) all asked
*"what is missing?"*. None asked *"is this true?"*. Two reviewers agreeing an AC is complete says
nothing about whether it is correct.

**It is autonomous with no approval gate.** Its loop has `check_for_human_interrupt()` but nothing
that stops it building 130 stories of the wrong thing. superpowers' `brainstorming` has a HARD GATE
exactly there.

**Three independent convergences** — treat as evidence these ideas are real:

| Idea | compound-engineering | mattpocock | this repo |
|---|---|---|---|
| A glossary as a durable layer | `CONCEPTS.md` | `CONTEXT.md` | `CONTEXT.md` exists, aged best of any doc |
| Doc staleness needs its own tool | `/ce-compound-refresh` | — | `doc-reality-auditor` |
| Learnings must compound | `docs/solutions/` | — | per-agent memory ×15 |

**A known flaw in what this project already built.** Its fifteen agents each hold *private* memory.
`ledger-invariant-checker` learned that `sell_all` re-posts off full capital and forgets
reservations; `sim-implementer` cannot read that. compound-engineering's shared `docs/solutions/`
pool does not have this problem. Decide how the custom framework handles it.

**Empirical failure inventory** — the framework must survive these, since they all really happened:

- A brief asserted "extend `map_dynamic::Dispatcher`" for trucks. Truck registration was commented
  out; the dispatcher had never seen a truck. The agent built the mechanism the AC forbade.
- The next brief said "copy the `freight_station.rs` train pattern." Trains have no parking; trucks
  only move when `Driving` with a collider. The premise was false.
- `base_mod/items.lua` sets `optout_exttrade = true` on one item of twenty-one. That line falsified
  three claims in a landed commit. No agent had ever read the Lua layer.
- `cargo test -p simulation sentinel` was documented as the regression runner. No test matched. It
  ran **0 tests and exited 0** — a green check with no subject.
- `CLAUDE.md` told every agent to read a `bevy.md` that did not exist. Four agent definitions
  targeted paths deleted five days before they were written.
- The economy credited buyers goods for free, every tick, whenever no freight station existed — the
  normal early game. Scarcity, the design's entire pressure source, was switched off, and no
  general code review found it. A specialist tracing conservation did.

---

## Step 4 — what already exists here. Do not reinvent it.

- **`docs/process/development-cycle.md`** — an 8-phase cycle (GROUND → PLAN → BUILD → PROVE → GATE → DISPOSITION →
  WRAP → SHIP) where every phase names the failure it prevents. **Phase 0 (GROUND) has no equivalent
  in any of the four frameworks** and was invented here.
- **Fifteen agents** in `.claude/agents/`, tiered deliberately: sonnet implements, opus reviews and
  advises. Two are calibrated against known answers; thirteen are unproven.
- **A two-layer tracking protocol** in `CLAUDE.md`: `br` issues for macro, `br comments` for micro.
  Critically — **subagents cannot see Claude's built-in task tools**; `br` is the only surface every
  agent reaches. Verified by direct probe.
- **Standing rules**: re-derive never inherit · evidence not assertion · an honest partial beats a
  broken whole · depth is never capped to save tokens (scope is narrowed, depth never is).

Your design should say, for each existing piece: **keep, modify, or replace** — with a reason.

---

## Step 5 — the deliverable

Write `docs/framework-design.md`. It must contain:

1. **A comparison grounded in fetched source**, with the commit SHA and date for each framework.
   For each: its theory of *how you know you are done*, its answer to *what if the spec is wrong*,
   its context/scale assumptions, and its regression story if any.
2. **What each does better than the others**, with the specific mechanism named — not a vibe.
3. **The design**: phases or flows, the artifacts each produces, the gates, and where human approval
   is required.
4. **Provenance for every element** — which framework it came from, or "new, because <evidence>".
5. **What you deliberately rejected** from each, and why. This section is as important as the rest;
   a design that takes everything is not a design.
6. **A migration path** from the current state — 130 stories mid-flight, an existing corpus, an
   existing roadmap. **A design requiring the corpus to be thrown away and re-extracted is almost
   certainly wrong**; say so explicitly if you conclude otherwise.
7. **How it fails.** Name the failure modes your own design is blind to. Every one of the four has a
   blind spot that only showed up in practice; yours will too. Find it before it finds us.

---

## Discipline

- **Fetch fresh and cite commits.** A claim about a framework must be traceable to a file at a SHA.
- **Read mechanisms, not summaries.** Do not describe a skill from its `description:` frontmatter.
- **Quote verbatim** when a framework's own words carry the argument.
- **Depth is not capped.** Take the tool calls and the time this needs. An under-read comparison
  producing confident wrong conclusions is the failure mode here.
- **Disagree where warranted.** The prior findings above were reached under time pressure by an
  agent that had just built fifteen agents and may be attached to them.
- **Do not install anything, do not modify `.claude/agents/`, and do not change the running project.**
  This is a design study. Its output is one markdown file.

---

## What would make this excellent rather than adequate

A design that is *smaller* than the sum of the four, not larger. Four frameworks offering ~90 skills
between them, and this project needs perhaps a dozen moves that actually fire. The hard work is
deciding what not to take.

The single most valuable thing you could produce: **a mechanism that stops a document asserting
something the code does not do.** All four frameworks generate documents. None verifies them. That
gap has cost this project more than every other failure combined.
