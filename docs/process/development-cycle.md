# The development cycle

**Kind:** process
**Authority:** operational
**Status:** active
**Owner:** project lead
**Last verified:** 2026-08-26

How one iteration of this project gets built, who does each part, and what each phase exists to
prevent. **Every phase here was added because something specific went wrong** — the failure is
named in each section, so nobody deletes a phase without knowing what it was buying.

Authoritative for process. `bd` is authoritative for task state. The
[`1.0 charter`](../plan/charter-1.0.md) is authoritative for scope.

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

Sixteen agents in `.claude/agents/`, invoked by name. Every one exists because something specific
went wrong; each prompt carries that evidence, so the trap is inherited rather than rediscovered.

| Phase | Agent | Tier | Owns |
|---|---|---|---|
| 0 | `substrate-cartographer` | opus | What the code, the Lua and the reference game actually do |
| 0 / 4 | `kornai-economist` | opus | Shortage economy, queue-clearing, the dishonest enterprise |
| 0 / 4 | `logistics-modeller` | opus | Dispatch, vehicles, routing, congestion |
| 0 / 4 | `utilities-modeller` | opus | Power, water, sewage, heat, waste, weather |
| 0 / 4 | `settlement-modeller` | opus | Citizens, households, needs, services |
| 0 / — | `soviet-authenticity` | opus | The fantasy and the look; judges from frames |
| 2 | `sim-implementer` | opus | `simulation/` — ~17,700 lines |
| 2 | `ui-implementer` | opus | `native_app/` — ~10,100 lines |
| 2 | `data-implementer` | opus | `base_mod/*.lua` ~950 + `prototypes/` ~2,790 — ~3,740 lines |
| 2 | `engine-implementer` | opus | `engine/` ~12,500 + `engine_demo/` ~520 — ~13,000 lines |
| 2 | `geom-implementer` | opus | `geom/` — ~10,500 lines |
| 2 | `widget-implementer` | opus | `goryak/` ~5,250 + `egui-inspect*` ~1,410 + `assets_gui/` ~1,165 — ~7,800 lines |
| 2 | `net-implementer` | opus | `networking/` — ~2,050 lines |
| 2 | `common-implementer` | opus | `common/` ~1,290 + `headless/` ~80 — ~1,370 lines |
| 3 | `evidence-auditor` | opus | The tests, not the code. Every guard seen failing |
| 4 | `wiring-auditor` | opus | Is it reachable from the running game? |
| 4 | `ledger-invariant-checker` | opus | Is quantity conserved? Economy diffs only |
| 4 | `reviewer` *(global)* | opus | General adversarial gate |
| — | `debugger` | opus | Root cause of a concrete misbehavior: diagnosis + minimal failing repro, never the fix. On demand, any phase |
| 6 | `doc-reality-auditor` | opus | Docs, agents and tickets vs the code |
| 7 | `release-engineer` | opus | Reproducible builds, pinning, licence |
| 7 | `perf-engineer` | opus | The five bench gates at 250k |

Tiering is now uniform opus/high across all 16 in-repo agents (user decision 2026-08-27;
supersedes the earlier uniform-sonnet policy). The standing opus review gate (`reviewer`, global)
remains the quality lever. The gate stays mandatory —
a high implementer tier does not replace it; the measured result (an opus reviewer caught a bug
an opus implementer shipped) was measured at opus tier.

A codex cross-vendor gate is **planned, not built**. `.codex/agents/` mirrors 15 roles as `.toml`
adapters but contains no reviewer and no gate entry point (verified 2026-08-27: `ls .codex/agents`
shows no `reviewer.toml`). Do not skip arranging the opus gate on the belief that a second one
already exists.

## Starting an iteration

```bash
bd ready                                    # what is unblocked, ranked
cat docs/plan/iterations/RESUME.md          # where the last session stopped
```

Then Phase 0: dispatch `substrate-cartographer` on every seam the iteration touches, and the domain
advisor for its cluster. **No brief gets written until the fact-sheet exists.**

Tracking is two-layer, and `bd` is the only surface every agent can reach — see `CLAUDE.md`:

| | Where | Who writes |
|---|---|---|
| Macro — goal, why, traps | a `bd` issue | lead creates, anyone updates |
| Micro — progress, findings | `bd comments add --author` | the worker itself |
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

`bd` issues carry the macro goal and the traps. Claude tasks mirror them as the user's dashboard.
See `CLAUDE.md` for the two-layer protocol.

---

## Phase 2 — BUILD

Parallel implementers on disjoint files:

| Agent | Owns |
|---|---|
| `sim-implementer` | `simulation/src/**` — ECS, economy, souls, map_dynamic |
| `ui-implementer` | `native_app/src/**` — panels, readouts, tools |
| `data-implementer` | `base_mod/*.lua`, `prototypes/src/**` |
| `engine-implementer` | `engine/src/**`, `engine_demo/**` — wgpu pipelines, passes, drawables, shaders, GPU timing, frame capture, input |
| `geom-implementer` | `geom/src/**` — vectors, matrices, quaternions, splines, volumes, frustum culling. Determinism-critical |
| `widget-implementer` | `goryak/src/**`, `egui-inspect*/**`, `assets_gui/src/**` — reusable yakui/egui widgets, theme, asset viewer |
| `net-implementer` | `networking/src/**` — connections, authentication, packets, world-send, catch-up replication |
| `common-implementer` | `common/src/**`, `headless/src/**` — timestep, saveload, rand, hashing. Tiny surface, enormous blast radius |

**Every workspace crate now has an owner** (measured 2026-08-27: 34,762 lines were previously
unowned, not the ~26,000 this table used to claim — it omitted `goryak/`, `assets_gui/` and the
`egui-inspect` pair). The global `implementer` is no longer the fallback for any crate in this
repo. If a task spans two lanes, split it; two agents must never own one file.

Each logs progress with `bd comments add <id> "…" --author <name>` as it goes, especially when it
discovers its brief was wrong. Evidence tasks are interleaved with code tasks, never trailed.

Implementers are **opus** (user decision 2026-08-27). The quality lever is still the review gate,
not implementer tier — this is measured, not assumed. A high tier does not earn a gate skip; every
non-trivial diff still runs the chain.

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
| 1 | `wiring-auditor` | opus | Is every new API actually called from production code? |
| 2 | `ledger-invariant-checker` | opus | Is quantity conserved across this economic seam? Run only when the diff touches the economy |
| 3 | `reviewer` | opus | General adversarial gate — re-derives from source, never from a worker's summary |
| 4 | domain advisor | opus | Sign-off, their cluster only — and only when the diff diverged from their Phase 0 answer (new mechanic, changed clearing rule, contradicted fact-sheet). A diff that lands exactly what Phase 0 approved skips this row |

**Splitting row 4 for `simulation/src/economy/market.rs`.** Three agents can each claim that file,
and the row is singular, so name the advisor by what the diff touches rather than by cluster:

| What the diff changes in `market.rs` | Advisor |
|---|---|
| The `Dispatch` state machine, routing, truck assignment | `logistics-modeller` |
| Capital, `reserved`, `requested` arithmetic — any quantity or money seam | `ledger-invariant-checker` (already unconditional at row 2; no separate sign-off) |
| Clearing policy, shortage behaviour, what an enterprise may request | `kornai-economist` |

A diff can hit two rows; then it gets two sign-offs. `sov-jcl` is the worked example — it lands in
`Market::advance_dispatches`, so logistics signs off and the ledger gate runs at row 2.

**Why the order — and it is not about tokens.** A reachability defect makes every later review
moot: there is no point auditing the logic, conservation or design of code that nothing calls. Run
the mechanical filter first so the expensive gates are reviewing code that actually runs.

Proven on the first real run: dispatched blind, `wiring-auditor` independently reproduced two
findings the ~112k opus gate had produced, and added one it had missed — `Market::dispatches()`
has no in-game observation surface, so the story's own promise that "the planner catches it from
observable state" is unmet outside `cargo test`.

Every gate re-derives from primary sources. A verifier that reads the producer's summary is
grading its own work.

See also: [process-layer drift review](review-2026-08-26-vs-swarmforge.md) for unresolved findings
against this roster and gate chain.

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
- A `bd` ticket sat open in the ready queue after its work had shipped.
- A legacy `RESUME.md` reported generated counts that did not match its corpus. Rebuild handoffs
  from `bd`, commits, executed commands, and current generated artifacts instead of copying them.

Then update the re-derived requirements and evidence inputs, run their documented `--check`
commands, regenerate [`the roadmap`](../generated/roadmap.md) with
`python3 docs/plan/iterations/build_roadmap.py --requirements-dir docs/plan/iterations/requirements --extract docs/plan/iterations/extract/requirements.json --evidence docs/generated/evidence/target-scenarios.json --output docs/generated/roadmap.md`,
and confirm every promoted scenario runs a non-zero test filter. A generated roadmap reports
status; it never closes work in place of `bd`.

(The scribe transcript-mining pass was retired 2026-08-26; durable learnings are recorded directly as they land.)

---

## Phase 7 — SHIP (per release, not per iteration)

| Agent | Does |
|---|---|
| `release-engineer` | Pin dependencies to commits. `egui` is currently `git = "…"` with **no branch or rev at all**, tracking upstream HEAD; `yakui` points at a personal fork's `dev` branch. The build is not reproducible |
| `perf-engineer` | Five PROPOSED bench gates at 250k scale (`bench_services`, `bench_terrain`, `bench_chains`, `bench_rail`, `bench_save`) — **none exist yet, and the charter names none of them**; it delegates gate definition to the implementation plan (charter:55-57). `sov-1ae` is open to build the first |

Then the visual proof. Per `CLAUDE.md`, work is not done until the user has seen it running: a
15–20s video, watched back before calling it done. A prior attempt captured the wrong monitor.

---

## Domain advisors

Select the advisor by the re-derived requirement cluster, not inherited iteration numbering:

| Advisor | Cluster |
|---|---|
| `utilities-modeller` | power, water, sewage, heating, waste, weather |
| `logistics-modeller` | dispatch, finite vehicles, routing, congestion |
| `settlement-modeller` | citizens, households, needs, services |
| `kornai-economist` | shortage economy and the dishonest enterprise |
| `soviet-authenticity` | presentation and player-facing visual proof |

**They never write code.** They answer whether a mechanic is consistent with the model. They
advise on request during Phase 0, and hold a Phase 4 sign-off for their own cluster **only when
the diff diverged from what they approved in Phase 0** — a faithful implementation of an already-
approved design does not need the same opus mind to approve it twice.

`kornai-economist` is special: the dishonest enterprise is the core loop of the whole game, so it
is consulted wherever a contract touches allocation, shortages, or enterprise reporting.

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

The active requirement schedule and its target-evidence counts are generated, so do not copy
iteration totals into this process document. Cheap-filter ordering in Phase 4 and persistent
cartographer memory remain load-bearing, not polish.

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

## The skill layer, and which owns what

Two skill packs are active. The three toolkits earlier revisions named here — `superpowers`,
`mattpocock-skills`, `iterative-development` — were absorbed into or superseded by the `compass`
plugin on 2026-08-24 and are disabled; only their *artifacts* survive.

| Layer | Owner | Use it for |
|---|---|---|
| Verbs + role playbooks | `compass` plugin | `/compass` routes one hop at a time; `/grill`, `/spec`, `/implement`, `/review`, `/debug`, `/lead`, `/tickets` are its verbs; the role playbooks preload into the generic agents |
| How much to build | `ponytail` | The ladder: YAGNI, stdlib first, shortest working diff — governs every implementer |
| The iteration | **this document** | Who does each part, and what gates it. Sovereign inside this repo: where a generic verb and this document conflict, this document wins, then fix whichever artifact drifted |
| Outer loop | `docs/plan/iterations/` + `build_roadmap.py` | Requirements, evidence, the generated roadmap. The corpus and the Phase 6 regeneration command outlived the retired `iterative-development` skill and remain canonical |

**Why the interrogation habit stays.** This project's signature failure is documents asserting
things the code does not do — `CLAUDE.md` pointing at a `bevy.md` that never existed, four agents
targeting paths deleted days earlier, `RESUME.md` miscounting the corpus by ten stories. Planning
flows structurally cannot catch that: they *generate* those documents and assume requirements
describe reality; on a hard fork they frequently do not. So `/grill` a claim **before** it becomes
ratified, and let Phase 0 and Phase 6 catch what slips through.

**Ticket flows defer to `bd`.** Compass verbs that track work (`/spec`, `/tickets`, `/triage`,
`/wayfinder`) operate *through* the `bd` workspace in this repo, never beside it — two competing
ticket systems is worse than either. `bd` remains authoritative for task state over any plan file,
including this one.

## Standing rules that cut across every phase

- **Re-derive, never inherit.** A claim from a brief, a memory file, a previous session or a peer
  agent is untrusted until checked against code. Two false substrate claims once reached roughly
  twenty dispatches before anyone verified either.
- **Evidence, not assertion.** "Tests pass" is not a result. The command and its real output is.
- **An honest partial beats a broken whole.** Two agents stopped mid-task and reported an accurate
  map instead of half-landing a rewrite. Both were right to, and both saved the next agent more
  than they cost.
- **`cargo test -p simulation` runs parallel and is trustworthy** since the `static mut` race in
  `init.rs`/`prototypes` was removed (`sov-test-race-initfuncs-qt6`, fixed 2026-08-26). The same
  defect shape still exists in `native_app/src/init.rs:85-86` — UI crate, not linked into the test
  binary; do not copy that pattern into new code.
