# The development cycle

How one iteration of this project gets built, who does each part, and what each phase exists to
prevent. **Every phase here was added because something specific went wrong** — the failure is
named in each section, so nobody deletes a phase without knowing what it was buying.

Authoritative for process. `br` is authoritative for task state. `docs/charter-1.0.md` is
authoritative for scope.

---

## The shape

```
0 GROUND  →  1 PLAN  →  2 BUILD  →  3 PROVE  →  4 GATE  →  5 DISPOSITION  →  6 WRAP
                                                                                  ↓
                                                                    7 SHIP (per release)
```

Phases 0, 3, 4 and 6 are the ones that catch defects. Phases 1 and 5 are the lead's and are
never delegated.

## The roster

Fifteen agents in `.claude/agents/`, invoked by name. Every one exists because something specific
went wrong; each prompt carries that evidence, so the trap is inherited rather than rediscovered.

| Phase | Agent | Tier | Owns |
|---|---|---|---|
| 0 | `substrate-cartographer` | opus | What the code, the Lua and the reference game actually do |
| 0 / 4 | `kornai-economist` | opus | Shortage economy, queue-clearing, the dishonest enterprise |
| 0 / 4 | `logistics-modeller` | opus | Dispatch, vehicles, routing, congestion — 25 stories |
| 0 / 4 | `utilities-modeller` | opus | Power, water, sewage, heat, waste, weather — 26 stories |
| 0 / 4 | `settlement-modeller` | opus | Citizens, households, needs, services — 24 stories |
| 0 / — | `soviet-authenticity` | sonnet | The fantasy and the look; judges from frames |
| 2 | `sim-implementer` | sonnet | `simulation/` — ~15,400 lines |
| 2 | `ui-implementer` | sonnet | `native_app/` — ~3,600 lines |
| 2 | `data-implementer` | sonnet | `base_mod/*.lua`, `prototypes/` — ~950 lines |
| 3 | `evidence-auditor` | sonnet | The tests, not the code. Every guard seen failing |
| 4 | `wiring-auditor` | sonnet | Is it reachable from the running game? |
| 4 | `ledger-invariant-checker` | opus | Is quantity conserved? Economy diffs only |
| 4 | `reviewer` *(global)* | opus | General adversarial gate |
| 6 | `doc-reality-auditor` | sonnet | Docs, agents and tickets vs the code |
| 6 | `scribe` *(global)* | sonnet | Mines raw transcripts into durable learnings |
| 7 | `release-engineer` | sonnet | Reproducible builds, pinning, licence |
| 7 | `perf-engineer` | sonnet | The five bench gates at 250k |

Tiering is deliberate and measured, not cosmetic: **sonnet implements, opus reviews and advises.**
The quality lever is the review gate, never the implementer's tier. Do not "upgrade" an implementer
to fix quality — add the gate.

## Starting an iteration

```bash
br ready                                    # what is unblocked, ranked
cat docs/superpowers/iterations/RESUME.md   # where the last session stopped
```

Then Phase 0: dispatch `substrate-cartographer` on every seam the iteration touches, and the domain
advisor for its cluster. **No brief gets written until the fact-sheet exists.**

Tracking is two-layer, and `br` is the only surface every agent can reach — see `CLAUDE.md`:

| | Where | Who writes |
|---|---|---|
| Macro — goal, why, traps | a `br` issue | lead creates, anyone updates |
| Micro — progress, findings | `br comments add --actor` | the worker itself |
| Dashboard | Claude tasks | **main session only** — subagents cannot see it |

---

## Phase 0 — GROUND

**Nothing is briefed until the ground is mapped.**

| Who | Does |
|---|---|
| `substrate-cartographer` (opus) | Maps the seam across all three sources and returns a cited fact-sheet |
| domain advisor for the cluster | Reads the iteration's stories and answers: is this mechanic model-consistent? |

The cartographer answers three questions that must agree, and the whole point is that they are
only meaningful **together**:

1. What does our Rust actually provide? (`simulation/`, `native_app/`, `prototypes/`)
2. What does our Lua actually declare? (`base_mod/*.lua`)
3. What does the reference implementation actually do? (the Workers & Resources install,
   1,472 `.ini` files — see the cartographer's own definition for the path)

**What this prevents — three real failures, all from one root:**

- An agent was told to "extend `map_dynamic::Dispatcher`" for trucks. Truck registration was
  commented out; the dispatcher had never seen a truck. It built the parallel mechanism the
  acceptance criterion forbade, because the criterion assumed a substrate that did not exist.
- A follow-up brief said "copy the `freight_station.rs` train pattern." Trains have no parking
  concept; trucks carry `VehicleState` and only move when `Driving` with a collider. The brief's
  central premise was false and the agent found it only by reading `road.rs` directly.
- `base_mod/items.lua` sets `optout_exttrade = true` on exactly one item out of 21. That single
  line falsified three claims in a commit that had already landed. **No agent had ever read the
  Lua layer.**

**Exit gate:** a fact-sheet exists, with file:line citations, for every seam the iteration touches.

Fact-sheets persist in the cartographer's memory. The second brief on the same seam is nearly free.

---

## Phase 1 — PLAN (lead only)

Decompose into TDD-sized tasks. For each: scope, acceptance criteria, the verification command,
and **explicit file ownership**.

**Watch the shared files.** Every agent adding a scenario must also edit
`simulation/src/tests/scenarios/mod.rs` to declare its module. An ownership table that assigns
each agent "its own file" and forgets the shared declaration file is wrong. This nearly caused a
clobber; only dispatch timing prevented it. Either serialise on the shared file or have the lead
pre-declare every module.

`br` issues carry the macro goal and the traps. Claude tasks mirror them as the user's dashboard.
See `CLAUDE.md` for the two-layer protocol.

---

## Phase 2 — BUILD

Parallel implementers on disjoint files:

| Agent | Owns |
|---|---|
| `sim-implementer` | `simulation/src/**` — ECS, economy, souls, map_dynamic |
| `ui-implementer` | `native_app/src/**` — panels, readouts, tools |
| `data-implementer` | `base_mod/*.lua`, `prototypes/src/**` |

Each logs progress with `br comments add <id> "…" --actor <name>` as it goes, especially when it
discovers its brief was wrong. Evidence tasks are interleaved with code tasks, never trailed.

Implementers are **sonnet**. The quality lever is the review gate, not implementer tier — this is
measured, not assumed. Do not "upgrade" an implementer to fix quality; add the gate.

---

## Phase 3 — PROVE

`evidence-auditor` (sonnet). One rule: **every new guard must be seen failing.**

Mutate the thing it protects, watch the test go red, paste the real output, revert. An assertion
never observed failing proves nothing.

**What this prevents:** `cargo test -p simulation sentinel` was documented as the sentinel-set
runner. No test function contained `sentinel`. It ran zero tests and exited zero — a green check
whose subject did not exist. Separately, four of five "proofs" of provided behaviour were never
mutation-tested, and one asserted arithmetic rather than the behaviour its story claimed.

---

## Phase 4 — GATE

**Ordered cheap to expensive. This ordering is the point.**

| # | Agent | Tier | Asks |
|---|---|---|---|
| 1 | `wiring-auditor` | sonnet | Is every new API actually called from production code? |
| 2 | `ledger-invariant-checker` | opus | Is quantity conserved across this economic seam? Run only when the diff touches the economy |
| 3 | `reviewer` | opus | General adversarial gate — re-derives from source, never from a worker's summary |
| 4 | domain advisor | opus | Hard sign-off, their cluster only |

**Why the order — and it is not about tokens.** A reachability defect makes every later review
moot: there is no point auditing the logic, conservation or design of code that nothing calls. Run
the mechanical filter first so the expensive gates are reviewing code that actually runs.

Proven on the first real run: dispatched blind, `wiring-auditor` independently reproduced two
findings the ~112k opus gate had produced, and added one it had missed — `Market::dispatches()`
has no in-game observation surface, so the story's own promise that "the planner catches it from
observable state" is unmet outside `cargo test`.

Every gate re-derives from primary sources. A verifier that reads the producer's summary is
grading its own work.

---

## Phase 5 — DISPOSITION (lead only)

Every finding gets an explicit disposition: **fixed / accepted / filed**. No orphans.

**Re-verify each finding against the current commit before acting on it.** A finding is filed
against a snapshot. In one wave an agent rewrote 359 lines of the reviewed file between the gate
filing three findings and the lead reading them; all three had to be re-checked against the new
code. Two survived. Acting on a stale finding is as bad as missing a live one.

---

## Phase 6 — WRAP

`doc-reality-auditor` (sonnet) sweeps every doc, agent definition and open ticket against the
code, and reports what is stale.

**What this prevents, all found in a single session:**

- `CLAUDE.md` instructed every agent to "Read `bevy.md`" — the file did not exist, and the engine
  had been discarded months earlier.
- Four agent definitions targeted `src/sim/` and `src/game/`, paths deleted five days before the
  agents were written.
- A `br` ticket sat open in the ready queue after its work had shipped.
- `RESUME.md` claimed 35 epics and 139 stories. The real counts were 36 and 149.

Then: mark stories `done:ITER-NNNN`, update `behavior-scenarios.md` and `behavior-corpus.md`,
regenerate `roadmap.md` with `build_roadmap.py` **from the repo root**, append and validate
`iteration-log.md`, and `grep` for `TODO(ITER-NNNN)` — a hard gate, the iteration is not done
while its own markers remain.

Finish with a `scribe` pass over the raw transcripts, not over the lead's digest.

---

## Phase 7 — SHIP (per release, not per iteration)

| Agent | Does |
|---|---|
| `release-engineer` | Pin dependencies to commits. `egui` is currently `git = "…"` with **no branch or rev at all**, tracking upstream HEAD; `yakui` points at a personal fork's `dev` branch. The build is not reproducible |
| `perf-engineer` | The five charter bench gates at 250k scale: `bench_services`, `bench_terrain`, `bench_chains`, `bench_rail`, `bench_save` |

Then the visual proof. Per `CLAUDE.md`, work is not done until the user has seen it running: a
15–20s video, watched back before calling it done. A prior attempt captured the wrong monitor.

---

## Domain advisors

Five advisors, sized off the actual roadmap rather than intuition:

| Advisor | Cluster | Scheduled stories |
|---|---|---|
| `utilities-modeller` | ITER-0008, 0009, 0010 | 26 |
| `logistics-modeller` | ITER-0003, 0006 | 25 |
| `settlement-modeller` | ITER-0007, 0011 | 24 |
| `kornai-economist` | ITER-0012 + the core loop everywhere | 13 |
| `soviet-authenticity` | presentation, all iterations | — |

**They never write code.** They answer whether a mechanic is consistent with the model. They
advise on request during Phase 0, and hold a hard sign-off gate in Phase 4 for iterations in their
own cluster only.

`kornai-economist` is special: the dishonest enterprise is the core loop of the whole game, so it
is consulted far outside ITER-0012.

---

## What a phase has cost

**These are observations for planning, not budgets.** No agent is held to them, and no agent prompt
should ever cap depth to hit one. An audit that stops early to save tokens produces a cheaper,
worse answer — `wiring-auditor`'s first run went well past its designed target and that extra depth
is exactly what surfaced a finding the more expensive general gate had missed. Narrow the scope of
a role; never narrow its depth.

Measured from real dispatches, not estimated:

| | tokens |
|---|---|
| miner / extraction | 65–85k |
| sonnet implementer | 110–155k |
| opus reviewer | 105–113k |
| narrow sonnet auditor | 15–30k |

Which puts one full iteration at roughly:

```
Ground ~80k · Build ~360k · Prove ~40k · Gate ~165k · Wrap ~30k   ≈ 675k
```

Thirteen iterations remain after ITER-0000. That number is why cheap-filter ordering in Phase 4
and persistent cartographer memory are load-bearing, not polish.

---

## Briefing an agent

- **Never write "report to team-lead".** There is no reachable agent by that name; the session has
  its own name (`soviet-simulator-NN`). Both agents in the first real wave burned a failed
  `SendMessage` on it before falling back. **An agent's final message IS its report** — say that
  instead, and the lead receives it automatically.
- Give the brief the raw sources, not your conclusions. A pre-digested brief caps what the worker
  can find — and if your premise is wrong, the worker inherits the error. Say explicitly when you
  are deliberately *not* telling it what to look for; a blind audit is stronger evidence than a
  confirmed one.
- Name the verification command, and require real output rather than a claim.
- Tell it what NOT to touch, and who owns the files it must not write.

## How this relates to the `iterative-development` skill

Two loops, different scopes. They do not compete.

| | `iterative-development` skill | This document |
|---|---|---|
| Scope | The **outer** loop across the whole project | The **inner** loop of one iteration |
| Answers | *What* to build and in what order | *Who* does each part and *how* it is proven |
| Artifacts | `requirements/`, `roadmap.md`, `behavior-corpus.md`, `iteration-log.md` | the agent roster, the gates, the phase order |

The skill's `running-an-iteration` gives the canonical steps — sentinel baseline, citation check,
scope review, decompose into code *and evidence* tasks, dispatch, post-iteration runs, resolve
`TODO(ITER-NNNN)` markers, wrap up. **Keep following those.** This document refines them with two
things the generic skill cannot know:

1. **Phase 0 does not exist in the skill.** It was added here because three failures in one session
   all traced to briefs asserting substrate that did not exist. The skill assumes the requirements
   describe reality; on a hard fork they frequently do not.
2. **The skill says "dispatch an implementer"; this says which one, and which gates follow.**

Where the two genuinely conflict, **this document wins**, because it is project-specific and
evidence-backed. Then fix whichever artifact drifted — do not route around it.

`br` remains authoritative for task state over any plan file, including this one.

## Standing rules that cut across every phase

- **Re-derive, never inherit.** A claim from a brief, a memory file, a previous session or a peer
  agent is untrusted until checked against code. Two false substrate claims once reached roughly
  twenty dispatches before anyone verified either.
- **Evidence, not assertion.** "Tests pass" is not a result. The command and its real output is.
- **An honest partial beats a broken whole.** Two agents stopped mid-task and reported an accurate
  map instead of half-landing a rewrite. Both were right to, and both saved the next agent more
  than they cost.
- **`cargo test -p simulation -- --test-threads=1`.** Parallel runs segfault intermittently on a
  pre-existing unsynchronised `static mut` race in `init.rs` (`sov-test-race-initfuncs-qt6`). A
  green parallel run proves little.
