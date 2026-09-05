# Software Development Cycles: Scrum and Adjacent Frameworks
## Research for Process Overhaul — 2026-08-28

**Audience:** One human lead + N stateless LLM subagents, token-budget-constrained, no persistent memory, no clock. `bd` (beads) is the tracker. Work is gate-gated across 8 phases. Purpose: feed a redesign decision.

**Sources:** Scrum Guide 2020 (scrumguides.org), Shape Up (basecamp.com/shapeup), Kanban Guide May 2025 (kanbanguides.org), XP (Beck; secondary via nimblework.com/agile/extreme-programming-xp), DORA (dora.dev), Team Topologies (teamtopologies.com/key-concepts), Scrum.org pathology articles. All claims tagged by source.

---

## 1. Scrum from the Primary Source (Scrum Guide 2020)

Source: https://scrumguides.org/scrum-guide.html [doc]

### Three Empirical Pillars

| Pillar | What it prevents | Human assumption | Holds for LLM agents? |
|---|---|---|---|
| **Transparency** — work visible to performers and stakeholders | Decisions made on hidden state; invisible debt | Humans can forget what's happening; visibility requires discipline | **Partially.** Agents have no ambient awareness; everything must be explicit in a brief. But agents also cannot deceive themselves — their "state" is the brief plus file output. Transparency must be structural (files, `bd` comments), not social. |
| **Inspection** — regular examination of artifacts and progress | Drift goes undetected; small problems compound | Humans need scheduled forcing functions or they avoid review | **Yes, but differently.** An agent inspects its own output only within one run. Cross-run inspection must be forced by the lead (a gate agent, a reviewer spawn). |
| **Adaptation** — immediate adjustment when limits exceeded | Sunk-cost continuation of wrong paths | Humans resist changing course; self-managing teams adapt faster | **Structurally blocked.** A mid-sprint agent cannot adapt; it runs to completion. Adaptation is a dispatch decision by the lead, not a team decision. |

### Five Values

Commitment, Focus, Openness, Respect, Courage. [doc, scrumguides.org]

These values exist to build **trust** inside a persistent team. Agents have no trust relationship; they have a brief and a context window. The values are not transferable, but their **structural outputs** are: Focus → narrow brief; Openness → `bd` comments; Commitment → acceptance criteria with evidence.

### Roles

**Developers**
- Accountability: usable Increment each Sprint.
- Failure prevented: diffuse responsibility with no clear owner of working software.
- Assumption: stable, persistent team with daily communication.
- Agent verdict: No persistence. Replace with "whoever holds a `bd` issue is the developer for that scope."

**Product Owner (PO)**
- Accountability: Product Backlog ordering; Product Goal.
- Failure prevented: backlogs that grow without value judgment; conflicting priorities.
- Assumption: single human with business authority, always reachable.
- Agent verdict: **The human lead IS the PO.** No agent can hold this role. PO absence is a known pathology (see §3); here it is structural — the lead must write Product Goal and Sprint Goal themselves.

**Scrum Master (SM)**
- Accountability: Scrum as defined; team effectiveness; impediment removal.
- Failure prevented: ceremony drift; team overwhelmed by organizational friction.
- Assumption: persistent human who coaches culture over time.
- Agent verdict: **DROP as a role.** No agent can coach culture across sessions. The SM's process functions (timebox enforcement, impediment triage) transfer to the human lead or to gate agents.

### Events

**Sprint (1 month or less)**
- Failure prevented: open-ended scope creep; infinite planning; invisible progress.
- Assumption: continuous calendar time; team members always available during sprint.
- Agent verdict: **TRANSFORMS.** Calendar time is irrelevant. The Sprint's real function is a **scope box**: a fixed, agreed body of work. Replace with an **Iteration scope gate** (already exists as the 8-phase cycle). The commitment is a `bd` milestone, not a two-week clock.

**Sprint Planning (max 8 hrs)**
- Three questions: Why is this Sprint valuable? What can be done? How will it be done?
- Failure prevented: teams start work with no shared understanding of goal or approach.
- Assumption: real-time collaborative deliberation; verbal negotiation.
- Agent verdict: **TRANSFORMS.** Replace with a **brief-writing ritual** by the human lead: a single planning document (the pitch) that answers all three questions before any agent is spawned. Agents cannot negotiate in real time.

**Daily Scrum (15 min)**
- Failure prevented: individual drift from Sprint Goal; impediments hidden for days.
- Assumption: daily clock; co-located or synchronous team; human memory.
- Agent verdict: **DROP.** No clock; no persistent agents. Replace with **mandatory `bd comment` at every agent run-end**: last known state, blockers, and what assumption changed. This achieves Transparency without a meeting.

**Sprint Review (max 4 hrs)**
- Failure prevented: building in a vacuum; features that miss stakeholder needs.
- Assumption: live stakeholders can attend and give feedback.
- Agent verdict: **TRANSFORMS.** The human lead reviews every completed iteration gate output (running game, not just clean build — per CLAUDE.md). The "stakeholder" is the lead themselves plus the running sim.

**Sprint Retrospective (max 3 hrs)**
- Failure prevented: teams repeat the same mistakes; process never improves.
- Assumption: persistent team with shared memory of what happened.
- Agent verdict: **TRANSFORMS into a lead-authored post-mortem.** Agents have no retrospective memory. The lead writes a brief structured post-mortem per iteration (what broke, what the next brief must say differently). Store in `.planning/` — NOT delegated to an agent (see §3, retro pathology).

### Artifacts and Commitments

| Artifact | Commitment | Failure prevented | Agent relevance |
|---|---|---|---|
| Product Backlog | Product Goal | Disconnected backlog items; no north star | KEEP: the `bd` backlog IS the Product Backlog; the charter IS the Product Goal |
| Sprint Backlog | Sprint Goal | Team works disconnected tasks with no unifying why | TRANSFORMS: iteration `bd` milestone is the Sprint Goal; the brief answers "why" |
| Increment | Definition of Done (DoD) | Vague done; false progress; quality debt | KEEP exactly: the existing gate checklist IS the DoD; gate agents enforce it |

---

## 2. Adjacent Frameworks

### Extreme Programming (XP) — Beck 1999/2004

Source: secondary via nimblework.com/agile/extreme-programming-xp [source unverified against primary book]

| Practice | Failure prevented | Agent assumption check |
|---|---|---|
| **TDD** | Regressions; design smell; fear of change | **KEEP.** `cargo test -p simulation` already runs. Tests are the agent's only feedback mechanism. TDD is more important for agents than humans — agents can't eyeball behavior. |
| **Pair Programming** | Knowledge silos; review blindness; poor design | **TRANSFORMS → reviewer agent.** One implementer + one reviewer is the structural analogue. Simultaneous pairing impossible. |
| **Collective Code Ownership** | Bottlenecks; tribal knowledge | **KEEP implicitly.** Agents have no territory. Any agent can touch any file. The risk is the inverse: no agent knows a file's history without reading it. |
| **Continuous Integration** | Long-lived branches; integration surprises | **KEEP.** `cargo build` + `cargo test` at every gate. Already enforced. |
| **Small Releases** | Late feedback; integration risk; feature bloat | **KEEP.** Iteration scope boxes; gate before next work begins. |
| **On-site Customer** | Building wrong thing; slow feedback | **TRANSFORMS → human lead is the customer.** The lead must be reachable to answer product questions that arise mid-brief. |
| **40-Hour Week / Sustainable Pace** | Burnout; quality decay under pressure | **IRRELEVANT for agents.** Token budget is the relevant constraint. |
| **Simple Design** | Over-engineering; unnecessary complexity | **KEEP.** Agents tend toward over-engineering (they pattern-match to complex solutions). Explicit "simplest that works" instruction in briefs helps. |
| **Refactoring** | Accumulated design debt | **KEEP — but as a dedicated task.** Agents cannot refactor safely without reading the full context first. Never sneak refactors into feature briefs. |
| **Coding Standards** | Inconsistency; review friction | **KEEP.** Clippy + rustfmt. Already enforced. |

**XP values: Communication, Simplicity, Feedback, Courage, Respect** [source: secondary]

### Kanban

Source: kanbanguides.org/html-kanban-guide/ (May 2025) [doc]

| Practice | Failure prevented | Agent verdict |
|---|---|---|
| **WIP limits** | Context-switch thrash; started-not-finished pile | **TRANSFORMS.** WIP for the lead: how many parallel agent waves are in flight. One wave at a time is the practical limit for a solo lead to review. |
| **Pull principle** | Push-driven overload; work queued faster than done | **KEEP conceptually.** No new agent brief until the gate for the previous one clears. Already enforced by the phase-gate structure. |
| **Flow metrics** (WIP, throughput, cycle time, work item age) | Invisible bottlenecks; decisions without data | **TRANSFORMS partially.** Cycle time per `bd` issue (open→close delta) is measurable. Throughput = issues closed per iteration. Age = stale issues. `bd stale` already exists. |
| **Visualized workflow** | Shared misunderstanding of state | **TRANSFORMS.** `bd list` + `bd show` is the board. No Kanban board needed if the lead reviews `bd` before each wave. |
| **Definition of Workflow (DoW)** | Ambiguity about when work starts/stops | **KEEP.** The 8-phase cycle already defines this. |

**Flow metrics are mandatory per the Kanban Guide [doc].** The guide does not mandate a specific cadence (no Sprints) — it demands continuous measurement. This is the key Kanban advantage: it fits a team of one human reviewing waves of agents, because there is no sprint commitment to manage.

### Shape Up (Basecamp)

Source: basecamp.com/shapeup/webbook [doc]

| Element | Failure prevented | Agent verdict |
|---|---|---|
| **Appetite** (not estimates) | Endless scope expansion; estimates that inflate | **DIRECTLY APPLICABLE.** "We have a token budget and one iteration for this feature" is an appetite. Agents cannot estimate; they can only be given scope constraints. |
| **Pitches** | Vague feature requests entering execution | **DIRECTLY APPLICABLE.** A pitch IS the brief format: problem, appetite, proposed solution, rabbit holes to avoid, hard constraints. The lead should write pitches, not just `bd` titles. |
| **Betting Table** | Backlog rot; arbitrary prioritization | **TRANSFORMS.** The lead's iteration planning session replaces the betting table. One human, no vote. But the forcing function — "what are we betting THIS iteration?" — is valuable. |
| **Hill Charts** | Misleading task-completion metrics; hidden unknowns | **DROP.** Agents report blockers and evidence via `bd` comments. Hill charts require ongoing human judgment to place dots. |
| **Cool-Down** | Burnout; no space for discovered bugs | **TRANSFORMS.** Between waves: time for the lead to review, write next pitches, run `bd stale`. Not a two-week calendar break — a gate between phases. |
| **Circuit Breaker** | Scope inflation; no-extension discipline | **DIRECTLY APPLICABLE and critical.** If an agent brief is not resolved in one run, it is CANCELLED and rewritten (simpler, smaller) — never extended. This prevents runaway agent cost. |

Shape Up's appetite + circuit breaker is the most directly applicable framework for agent teams. Its model assumes a small, trusted team — the human lead plus spawned agents fits better than a standing Scrum team.

### Lean / Toyota Production System

Sources: training data only [unverified against primary source — Toyota Production System book, Ohno 1988]

| Concept | Failure prevented | Agent verdict |
|---|---|---|
| **Andon cord** (stop the line on defect) | Defects propagating downstream | **DIRECTLY APPLICABLE.** A gate agent that fails STOPS the wave. Already in the 8-phase model. This is the most important Lean principle for agent work. |
| **Jidoka** (automation with human intelligence) | Blind automation continuing despite errors | **KEEP.** Gate agents are jidoka: automated check + stops for human judgment when uncertain. |
| **Kaizen** (continuous improvement) | Stagnation; no learning cycle | **TRANSFORMS → lead-authored post-mortem.** Agents cannot kaizen. The lead accumulates learning across iterations. |
| **Single-piece flow / no WIP inventory** | Batch-and-queue delays | **KEEP.** Issue-by-issue dispatch, not batch dispatch where possible. |
| **Muda (waste elimination)** | Token waste on low-value work | **DIRECTLY APPLICABLE.** Every agent prompt is waste-audited: is this brief precise enough to not require a follow-up? |

**Unverified claim:** "Andon cord" as described comes from secondary sources. The original Toyota mechanism is verified in Ohno's book but not from a live primary source here.

### Team Topologies

Source: teamtopologies.com/key-concepts [doc]

| Concept | Failure prevented | Agent verdict |
|---|---|---|
| **Stream-aligned team** | Handoff delays; no end-to-end ownership | **TRANSFORMS.** Each `bd` issue is a temporary stream-aligned agent: owns its scope end-to-end for one run. |
| **Enabling team** | Capability gaps; platform adoption barriers | **TRANSFORMS.** The researcher agent is an enabling team of one. The substrate-cartographer is an enabling agent for brief-writers. |
| **Complicated-subsystem team** | Diluted specialized expertise | **TRANSFORMS.** Domain-specialist agents (kornai-economist, logistics-modeller) are complicated-subsystem roles. Already in the roster. |
| **Platform team** | Cognitive overload on stream-aligned teams | **TRANSFORMS.** The `bd` CLI + the gate framework + the code-review-graph is the platform. It must be maintained so implementers can trust it. |
| **Collaboration mode** | Exploration stalls without cross-functional input | **TRANSFORMS → parallel agent spawn.** Two agents exploring the same problem and reporting back is structurally equivalent. |
| **X-as-a-Service mode** | High-coordination overhead for stable interfaces | **KEEP.** Domain advisor agents serve X-as-a-Service: ask once, get a judgment. |
| **Facilitation mode** | Teams stuck on process problems | **TRANSFORMS → gate agent.** Gate agents facilitate without implementing. |
| **Cognitive load** | Teams failing because they own too much complexity | **CRITICAL for brief design.** A brief that dumps the entire codebase context on an agent exceeds its cognitive load. Narrow scope = the key design constraint. |

Cognitive load is the most transferable Team Topologies concept: **keep agent briefs narrow enough that the agent can hold the relevant context in one window.**

### DORA Four Keys + SPACE

Source: dora.dev/research/ [doc], secondary for exact definitions [source: launchdarkly.com, getdx.com]

**Four Keys:**
- **Deployment Frequency** — how often you release to production.
- **Lead Time for Changes** — commit to production.
- **Change Failure Rate** — % of deploys causing production failures.
- **Time to Restore Service** — recovery time after a failure.

**SPACE framework** (Forsgren et al., 2021): Satisfaction, Performance, Activity, Communication/Collaboration, Efficiency. Designed to prevent single-metric gaming (e.g. pure velocity).

**Agent verdict:** The Four Keys are **production-deployment metrics** — not directly applicable to a solo game-dev project with no production deploys. But the underlying model transfers:
- Deployment Frequency → **iteration close rate** (how often a phase completes).
- Lead Time → **`bd` issue open-to-close delta**.
- Change Failure Rate → **gate-fail rate** (how often a gate rejects before accepting).
- Time to Restore → **how long from a gate failure to a passing re-run**.

DORA's finding that **cultural factors outweigh technical ones** [doc, dora.dev] is irrelevant for a purely agent team — there is no culture to shape. The human lead's process design is the entire culture.

### Other Practices

**Definition of Ready (DoR):** A `bd` item is ready when: problem described, appetite stated, rabbit holes named, acceptance criteria written, relevant file paths provided. An agent that starts without DoR will produce output the lead cannot accept. DoR is the brief-quality gate.

**INVEST** (Independent, Negotiable, Valuable, Estimable, Small, Testable): For agent briefs: Independent and Small are critical — an agent that depends on another in-flight agent produces a race condition. Estimable is irrelevant (agents cannot estimate). Testable is mandatory.

**Three Amigos / BDD:** Three perspectives (product, dev, test) on one story. TRANSFORMS: the human lead writes the story; the implementer agent writes the code; the evidence-auditor agent takes the tester role. Three sequential runs, not one conversation.

**Backlog Refinement:** KEEP. The lead reviews `bd` items before each wave. Items without DoR are blocked before dispatch.

**Mob / Ensemble Programming:** DROP. Not physically possible with stateless agents.

**Swarming:** Multiple agents on one blocker simultaneously. KEEP for investigation (parallel forks). DANGEROUS for implementation (conflicting edits).

**Retrospective formats (4Ls, Starfish):** TRANSFORMS → lead post-mortem. Agents cannot participate. The lead uses a simple format: What worked / What broke / What the next brief must say differently.

**Sprint Review as demo:** TRANSFORMS → the lead runs the game and watches 15–20 seconds of it (per CLAUDE.md delivery rule). The "demo" is the running sim, not a presentation.

---

## 3. Known Scrum Pathologies and Their Agent Analogues

| Pathology | Human form | Agent-team analogue | Severity |
|---|---|---|---|
| **Scrum-but** | "We do Scrum but skip retros" | "We have an 8-phase cycle but skip gates when pressed" | HIGH — gate integrity is the only quality lever |
| **Ceremony theatre** | Standups that report status upward instead of coordinating peers | A retro agent that writes generic lessons nobody reads; a planning session that produces a brief the agent ignores | HIGH — ceremony without substance wastes tokens |
| **Velocity gaming** | Inflating story points to look productive | An agent that closes issues without meeting acceptance criteria; evidence-free closes | HIGH — `bd close --reason "commit <sha>"` is the guard; audit it |
| **Cargo-cult standups** | 30-min status reports to SM instead of peer coordination | Daily agent runs that produce status reports nobody reads | MEDIUM — DROP Daily Scrum entirely (see §1) |
| **PO absence** | PO unreachable; team builds without value judgment | **The human lead goes dark between waves.** Agents proceed on stale briefs. | CRITICAL — the lead IS the PO; absence is the single most dangerous pathology |
| **Sprint without Goal** | Team picks disconnected backlog items; no unifying why | An iteration `bd` milestone with no explicit "why this, now" statement | HIGH — each wave needs a one-sentence Sprint Goal equivalent |
| **Retrospective that changes nothing** | Team writes improvements; nobody acts | A researcher produces a process report; no rule in CLAUDE.md or agent definition changes | HIGH — retros must produce a specific file change, not prose |
| **Zombie Scrum** | Mechanical motions; no value delivery | Running all 8 phases on features the running game doesn't show | MEDIUM — "prove it in the running game" (CLAUDE.md) is the guard |
| **Refinement theatre** | Tickets refined indefinitely; nothing shipped | Issues refined in `bd` without gate completion; backlog grows faster than it closes | MEDIUM — `bd stale` + `bd orphans` sweep is the guard |

---

## 4. Verdict Table

Scope: every named practice from §1–2.

| Practice / Element | Verdict | Reasoning |
|---|---|---|
| **Sprint (time-box)** | TRANSFORMS → scope box | No clock; the iteration boundary is a `bd` milestone and a gate, not a calendar |
| **Sprint Planning session** | TRANSFORMS → lead-written pitch/brief | Agents cannot negotiate in real time; the brief IS the plan |
| **Daily Scrum** | DROP | No clock, no persistent agents; replaced by mandatory `bd comment` at run-end |
| **Sprint Review** | TRANSFORMS → lead runs the game | One human stakeholder; "demo" = 15–20s of running sim |
| **Sprint Retrospective** | TRANSFORMS → lead post-mortem file in .planning/ | Agents have no retrospective memory; must produce a specific file change to matter |
| **Product Owner role** | KEEP — held by human lead | Non-negotiable; PO absence is the #1 pathology risk |
| **Scrum Master role** | DROP | No persistent team to coach; gate agents cover process checks |
| **Developers role** | TRANSFORMS → issue assignee for one run | The agent claiming a `bd` issue is the developer for that scope |
| **Product Backlog** | KEEP — already `bd` backlog | Direct mapping |
| **Sprint Backlog** | TRANSFORMS → wave `bd` milestone | The set of issues targeted in this wave |
| **Increment** | KEEP — gate-passing, runnable output | Already enforced by gate structure |
| **Product Goal** | KEEP — charter 1.0 | Already exists |
| **Sprint Goal** | TRANSFORMS → iteration "why" statement | One sentence per wave; written by lead before dispatch |
| **Definition of Done** | KEEP — gate checklist | Already exists; gate agents enforce it |
| **Three empirical pillars** | TRANSFORMS structurally | Transparency via `bd` comments; inspection via gate agents; adaptation via lead post-mortem |
| **Five Scrum values** | DROP as culture; extract structure | Brief narrowness (Focus), `bd` openness (Openness), acceptance criteria (Commitment) |
| **XP TDD** | KEEP as-is | Most important agent constraint; agents cannot eyeball behavior |
| **XP Pair Programming** | TRANSFORMS → implementer + reviewer agent | Sequential, not simultaneous |
| **XP Collective Ownership** | KEEP implicitly | Agents have no territory; briefs must supply file context |
| **XP Continuous Integration** | KEEP as-is | cargo build + cargo test at every gate |
| **XP Small Releases** | KEEP — gate before proceeding | Already enforced |
| **XP On-site Customer** | TRANSFORMS → lead availability | Lead must be reachable to unblock product questions |
| **XP Sustainable Pace** | DROP — token budget replaces it | Announce scale before multi-agent runs |
| **XP Simple Design** | KEEP — explicit in briefs | "Use the simplest design that passes the gate" |
| **XP Refactoring** | KEEP as dedicated issue only | Never mixed into feature briefs |
| **Kanban WIP limits** | TRANSFORMS → max 1 wave in flight | One wave at a time for a solo reviewer |
| **Kanban Pull** | KEEP — no new wave until gate clears | Already enforced by phase structure |
| **Kanban Flow metrics** | TRANSFORMS → `bd` issue age/throughput | `bd stale` and close-delta are the metrics |
| **Kanban DoW** | KEEP — 8-phase cycle IS the DoW | Already exists |
| **Shape Up Appetite** | DIRECTLY APPLICABLE | Token budget + iteration scope = appetite; write it in every pitch |
| **Shape Up Pitch** | DIRECTLY APPLICABLE — use as brief format | Problem, appetite, solution sketch, rabbit holes, hard constraints |
| **Shape Up Betting Table** | TRANSFORMS → lead's iteration planning | One human, no vote; forcing function is the value |
| **Shape Up Hill Charts** | DROP | Replaced by `bd` comment state; no value added |
| **Shape Up Cool-Down** | TRANSFORMS → inter-wave review gate | Time for lead to review, write pitches, run `bd stale` |
| **Shape Up Circuit Breaker** | DIRECTLY APPLICABLE and critical | If an agent brief fails in one run, cancel and rewrite smaller — never extend |
| **Lean Andon Cord** | DIRECTLY APPLICABLE — already in place | Gate failure stops the wave. Enforce without exception. |
| **Lean Jidoka** | KEEP — gate agents | Automated check + stops for human judgment |
| **Lean Kaizen** | TRANSFORMS → lead post-mortem | Agents cannot learn; only the lead accumulates process knowledge |
| **Lean Muda** | DIRECTLY APPLICABLE | Brief quality is waste elimination; a bad brief is pure muda |
| **Team Topologies cognitive load** | DIRECTLY APPLICABLE — most important concept | Keep briefs narrow; an overloaded agent produces incoherent output |
| **Team Topologies stream-aligned** | TRANSFORMS → per-issue agent | Each `bd` issue is one temporary stream-aligned agent |
| **Team Topologies enabling** | TRANSFORMS → researcher/advisor agents | Already in roster (researcher, domain advisors) |
| **Team Topologies complicated-subsystem** | TRANSFORMS → domain specialist agents | Already in roster (kornai-economist, logistics-modeller, etc.) |
| **Team Topologies platform** | TRANSFORMS → bd + gates + graph | The platform is the tooling; must be maintained |
| **DORA Four Keys** | TRANSFORMS → bd metrics | Iteration close rate, issue age, gate-fail rate |
| **SPACE framework** | PARTIAL — satisfaction irrelevant | Efficiency (token/output ratio) and Activity (issues closed) are the useful dimensions |
| **Definition of Ready** | DIRECTLY APPLICABLE | Brief is not dispatched without DoR: problem, appetite, rabbit holes, acceptance criteria, file paths |
| **INVEST** | PARTIAL | Independent + Small + Testable are critical; Estimable irrelevant |
| **Three Amigos / BDD** | TRANSFORMS → sequential lead + implementer + evidence-auditor | Already in roster |
| **Backlog Refinement** | KEEP — lead-only | DoR check before every wave |
| **Mob Programming** | DROP | Impossible with stateless agents |
| **Swarming (investigation)** | KEEP for investigation | Parallel forks on a blocker; never parallel implementation on the same file |
| **Retrospective formats (4Ls, Starfish)** | DROP formats; keep output requirement | Lead uses simple What-worked/What-broke/What-to-change-in-brief |
| **Sprint Review as demo** | TRANSFORMS → lead runs the game | Already mandated by CLAUDE.md |
| **Velocity as metric** | DROP | Gaming risk is high; close rate + gate-fail rate are better signals |

---

## 5. Time-Boxing When There Is No Clock

**The problem:** Scrum's Sprint is a calendar time-box. Agents have no clock, no days, no weeks. What prevents scope from inflating?

Three candidate replacements, with reasoning. **No primary source addresses this directly for LLM agent teams** — the following is analytical reasoning, marked as such.

### Option A: Scope Box
Fix the number of `bd` issues in a wave before dispatch. A wave closes when all its issues close or are explicitly deferred. The lead cannot add issues mid-wave.
- Prevents: the lead keeps inserting "just one more thing" until the wave is unmanageable.
- Risk: a scope box without a size limit still produces a 40-issue wave that costs as much as a month of human work.
- Verdict: **Necessary but not sufficient alone.**

### Option B: Token Box
Announce token budget before any multi-agent run (per CLAUDE.md). If the budget is exhausted before scope completes, remaining issues are deferred (`bd defer`), not extended.
- Prevents: runaway cost; invisible spend.
- Risk: agents do not self-report token consumption reliably. The lead must monitor externally.
- Shape Up analogy: the appetite IS the budget. A six-week cycle for a human team is a fixed cost box. A token budget is the same thing for agents.
- Verdict: **Use appetite framing in every pitch: "this is a 2-agent, ~30k-token problem."**

### Option C: Evidence Box
A wave ends when the evidence requirement is met, not when calendar time expires. The evidence requirement is the Definition of Done (gate checklist). When every issue in the wave either passes its gate or is explicitly killed, the wave ends.
- Prevents: open-ended work with no stopping condition.
- This is the cleanest fit for the existing 8-phase model: each phase has a gate; the gate IS the stopping condition.
- Verdict: **Already the dominant mechanism. Make it explicit.**

**Recommended blend:** Scope Box (fixed issue set per wave) + Evidence Box (gate as stopping condition) + Token budget announced upfront as an appetite estimate. The circuit breaker (Shape Up) closes the loop: if a wave's gate is not passed in one run, rewrite the brief smaller before the next attempt.

**No primary source verified for this specific blend.** It is derived reasoning from Shape Up's appetite [doc, basecamp.com/shapeup], the Kanban pull principle [doc, kanbanguides.org], and the Lean circuit-breaker analogy.

---

## Gaps and What Would Close Them

1. **XP primary source not verified.** Claims about XP come from secondary sources (nimblework.com). The original Beck books (*Extreme Programming Explained*, 1st and 2nd editions) are the authority. Verdict here is low-risk because XP practices are stable and well-attested, but the exact formulation of each practice should be cross-checked against the book if you adopt them formally.

2. **DORA report detail not verified.** dora.dev returned high-level findings; the exact definitions of the Four Keys come from secondary sources (launchdarkly.com, getdx.com). The original Accelerate book (Forsgren, Humble, Kim, 2018) and DORA annual reports are authoritative.

3. **Lean / Toyota primary source not verified.** Ohno's *Toyota Production System* (1988) is the primary source. All Lean claims here are from training-data recall. The concepts are stable and widely attested; the specific framing should be verified against the book if cited formally.

4. **SPACE framework.** Not verified against the original paper (Forsgren et al., ACM Queue 2021). Secondary only.

5. **"Token box" as a sprint replacement** is original reasoning with no primary citation. The closest citation is Shape Up's appetite concept [doc, basecamp.com/shapeup]. This gap would be closed by empirical evidence from other agent-team projects — none found in this search.
