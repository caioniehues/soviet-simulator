# Multi-Agent Team Patterns for LLM-Driven Development

**Brief:** Design space of "team patterns" for LLM agents — from fan-out/gate to genuine collaboration —
with evidence, cost multipliers, and verdicts for a token-constrained team of one human lead + N subagents.

**Date:** 2026-08-28  
**Author:** researcher agent  
**Decision feeds:** soviet-simulator process overhaul, process-overhaul-2026-08-28

---

## 1. Pattern Catalogue

### 1.1 Orchestrator–Worker

**Mechanism:** A central LLM decomposes a task dynamically at runtime, delegates subtasks to worker
LLMs in parallel, then synthesises results. The orchestrator uses extended thinking (Anthropic) or
planning steps to determine worker count and boundaries before spawning.

**What it buys:** Parallelism where subtasks are not pre-definable. Scales with task complexity.
Anthropic's internal research system uses this as its primary pattern: 3–5 subagents for direct
comparisons, 10+ for complex research.

**Cost multiplier:** ~4× tokens vs a single chat interaction; up to 15× vs chat for a full multi-agent
run [live, Anthropic multi-agent research system blog post,
https://www.anthropic.com/engineering/multi-agent-research-system].

**Evidence:** Anthropic's multi-agent research system with Claude Opus 4 orchestrator + Sonnet 4
subagents outperformed single-agent Opus 4 by 90.2% on their internal research eval [source,
https://www.anthropic.com/engineering/multi-agent-research-system]. Token usage explained 80% of
performance variance — more tokens, more performance, but with diminishing returns.

**Known failure modes (MAST):** Step repetition (15.7%), unaware of termination conditions (12.4%),
disobey task specification (11.8%), reasoning-action mismatch (13.2%) [source, arxiv 2503.13657,
https://arxiv.org/abs/2503.13657]. Orchestrators spawning 50 subagents for simple queries was
an explicit early failure at Anthropic [source, anthropic blog].

**Needs live messaging?** No. Works with file-based results + a synthesis pass by the lead.

---

### 1.2 Pipeline / Assembly Line

**Mechanism:** Fixed sequence of specialist agents where each stage's output becomes the next stage's
input. Roles are pre-assigned: product manager → architect → engineer → tester (MetaGPT's model
[source, arxiv 2308.00352v6, https://arxiv.org/html/2308.00352v6]).

**What it buys:** Predictable structure; easy to audit; catches role-specific concerns in sequence.

**Cost multiplier:** ~3–6× depending on pipeline length. One pass per stage.

**Evidence:** MetaGPT achieves average score 3.9 on HumanEval-style tasks; ChatDev (chat-chain
variant) scores 2.1 on same benchmark but outperforms on quality metrics by cooperative
communication [unverified exact benchmark; source is emergentmind.com report on MetaGPT vs
ChatDev comparison]. MetaGPT shows 60–68% fewer system-design and inter-agent-misalignment
failures vs ChatDev but 1.56× higher task-verification failures [source, MAST study,
arxiv 2503.13657].

**Known failure modes:** FC3 verification failures dominate — MetaGPT's chess program passes
compilation checks while containing runtime bugs (MAST, concrete example). Sequential coupling
means an error in stage 2 propagates unchanged into stage 3.

**Needs live messaging?** No. File-based handoff per stage is the canonical design.

---

### 1.3 Planner–Executor–Critic

**Mechanism:** Three roles in a structured cycle. Planner produces a spec/plan. Executor implements
against it. Critic checks the output against the spec and either approves or returns a critique for
the next Executor iteration.

**What it buys:** Explicit feedback loop; critic holds a different context from the executor.
Structurally equivalent to Anthropic's Evaluator–Optimizer pattern [source,
https://www.anthropic.com/engineering/building-effective-agents]. Works best when evaluation
criteria are clear and "iterative refinement provides measurable value."

**Cost multiplier:** 2–4× per round-trip, multiplied by iteration count (typically 1–3 rounds until
approval or escalation).

**Evidence:** Anthropic notes this is "particularly effective when we have clear evaluation criteria"
[source, building-effective-agents]. MAST: incorrect verification (9.1%) and premature termination
(6.2%) are the primary failure modes for critic-equipped pipelines. The critic accepting a wrong
result is more common than it not running at all.

**Known failure modes:** Critic bias toward the executor's framing once it has read the solution.
See §4.4 on LLM-as-judge bias.

**Needs live messaging?** No. File-based; critic writes findings to a review file.

---

### 1.4 Generator–Verifier (Blind)

**Mechanism:** Generator produces N candidate solutions independently; a separate verifier (which has
NOT seen the generator's reasoning, only its output) selects the best or flags failures.

**What it buys:** Asymmetry: verification is easier than generation. Empirically confirmed —
verification accuracy (87.0%) is more than twice generation accuracy (32.4%) on hard problems
[source, LLM-as-a-Verifier, arxiv 2607.05391, https://arxiv.org/html/2607.05391v1]. On
SWE-Bench Verified with N=3 candidates, a verifier achieved 78.2% vs 76.1% mean Pass@1 of the
candidate pool [source, https://arxiv.org/html/2607.05391v1].

**Cost multiplier:** N× for generation + ~1× for verification. For N=3: ~4× total.

**Known failure modes:** Verifier sees only outputs, not generator reasoning — can miss subtly wrong
solutions that pass surface checks. LLM verifiers share the judge bias problem (§4.4).

**Needs live messaging?** No. Generator writes candidate files; verifier reads them.

---

### 1.5 Debate / Adversarial

**Mechanism:** Two or more agents take opposing positions (or are randomly assigned to argue for
different answers) and exchange rounds of critique. Typically 2–3 rounds; winner determined by
convergence or a separate judge.

**What it buys:** Reduces factual errors on reasoning tasks. Du et al. (2023) showed multi-agent
debate improves performance on arithmetic and strategic reasoning benchmarks vs single-agent
[source, ICML 2023, "Improving factuality and reasoning in language models through multiagent
debate," referenced in https://arxiv.org/html/2509.05396v1]. "Talk Isn't Always Cheap: Understanding
Failure Modes in Multi-Agent Debate" (2025) documents when debate hurts.

**Cost multiplier:** ~4–8× (rounds × agents).

**Known failure modes:** Confident-liar problem: a high-confidence wrong answer can flip correct
agents [source, "The Confident Liar," arxiv 2606.10296,
https://arxiv.org/pdf/2606.10296]. Persuasion is positional or stylistic, not epistemic.
Debate does not reliably detect wrong reasoning — it detects weak argument presentation.

**Needs live messaging?** No, but benefits from structured turn protocol. Files with per-round
responses work; a lead arbitrates.

---

### 1.6 Reflection / Self-Critique

**Mechanism:** A single agent (or same agent in a second pass) generates a response, critiques its
own output, then refines. Reflexion (Shinn et al. 2023): verbal reinforcement learning, 91% pass@1
on HumanEval vs GPT-4's 80% [source, referenced in arxiv 2405.06682]. Self-Refine (Madaan et al.
2023): iterative feedback on code optimization, dialogue, sentiment tasks [source, arxiv, referenced
in search results].

**What it buys:** Can cheaply catch surface errors without a second agent.

**Cost multiplier:** ~1.5–2× per reflection round. Very cheap.

**Known failure modes:** Cannot reliably identify its own errors without a ground-truth oracle.
Diminishing returns after the first round. Can degrade performance on easier prompts or with
already-capable base models. A model convinced of a wrong answer does not self-correct via
reflection alone [source, arxiv 2405.06682].

**Needs live messaging?** No. Single agent, sequential passes.

---

### 1.7 Best-of-N + Judge

**Mechanism:** Run the same prompt N times (with different temperature/seed), collect N outputs,
pass all to a judge that selects the best.

**What it buys:** Improves recall at the cost of precision. Oracle Pass@3 upper bound on
SWE-Bench Verified was 84.4% vs 76.1% mean Pass@1 [source, arxiv 2607.05391]. Cheap and
embarrassingly parallel.

**Cost multiplier:** Exactly N× generation + ~1× judge.

**Known failure modes:** Judge bias (§4.4). Convergent wrong answers: if the base model has a
systematic error, all N outputs share it and the judge cannot distinguish. No mechanism for the
judge to prefer a subtly correct minority answer.

**Needs live messaging?** No. Files suffice.

---

### 1.8 Tournament

**Mechanism:** N candidates compete pairwise; winners advance. Used for evaluation and ranking
rather than task completion.

**What it buys:** More robust ranking than a single judge pass — bias is distributed. Used in
preference dataset curation.

**Cost multiplier:** O(N log N) comparisons.

**Known failure modes:** Position bias (earlier candidate favored), verbosity bias, self-preference
bias all documented in LLM judges [source, "LLM-as-a-Judge: A Systematic Literature Review,"
researchgate.net/publication/406281480, and "Anchoring Bias in LLM-as-a-Judge,"
arxiv 2608.25869]. Reproduced independently across studies.

**Needs live messaging?** No. File-based comparisons; lead advances winners.

---

### 1.9 Blackboard / Shared Workspace

**Mechanism:** All agents read from and write to a shared, persistent artifact (a file, a structured
document, a wiki). No direct messaging — agents coordinate through the shared state. Readers
summarize what they see; writers append or update.

**What it buys:** Each agent sees the full accumulated work, not just a passed message. No
coordinator bottleneck. Supports async work.

**Cost multiplier:** ~1.5× overhead for reading the shared document. Scales well per-agent.

**Known failure modes:** Stale reads if agents write concurrently (file lock required). The blackboard
grows — agents reading a 50k-token document in every round hits context limits. Cognition's "Don't
Build Multi-Agents" argument is precisely that a shared scratchpad partially addresses context
fragmentation but summaries are still lossy [source, cognition.com/blog/dont-build-multi-agents].

**Needs live messaging?** No. File-based. This is the current soviet-simulator file-based handoff
model.

---

### 1.10 Market / Auction

**Mechanism:** Tasks are posted as work items; agents bid (or are assigned by capability), then
execute. A coordinator tracks assignments and resolves conflicts.

**What it buys:** Dynamic load balancing. Handles variable task difficulty.

**Cost multiplier:** High coordination overhead. Complex to implement correctly.

**Evidence:** No rigorous coding-task benchmark evidence found. Primarily a research curiosity.
**[unverified — no controlled study found]**

**Needs live messaging?** Yes — bidding requires real-time coordination.

---

### 1.11 Pair: Driver / Navigator

**Mechanism:** Driver executes (writes code, runs tools); Navigator holds the high-level spec and
plan, critiques Driver's moves turn-by-turn, and steers direction. Role-specific context: Navigator
does NOT write code; Driver does NOT plan.

**What it buys:** Simultaneous implementation and spec compliance. Human pair programming
meta-analysis (Hannay et al., 2009, ScienceDirect 0950-5849): positive effect on quality at high
task complexity, at the cost of ~2× effort [source, sciencedirect.com/science/article/abs/pii/
S0950584909000123].

**PairCoder** (ASE 2024) implements this pattern with two LLM agents: Navigator proposes solution
plans and selects the current optimal; Driver implements, tests, and refines. "Superior code
generation accuracy across benchmarks" vs baselines [source, arxiv 2409.05001,
https://arxiv.org/abs/2409.05001].

**Cost multiplier:** ~2× (two agents run per turn, alternating).

**Known failure modes:** Navigator and Driver can develop conflicting mental models when the
Navigator cannot see the Driver's intermediate state (context fragmentation). If the Navigator
only sees summaries of Driver output, information loss applies.

**Needs live messaging?** Requires structured turn protocol. Works with files if Navigator reads
Driver's output files and vice versa, but a turn protocol must enforce alternation.

---

### 1.12 Mob / Rotating Driver

**Mechanism:** All agents see the full shared workspace. One is designated Driver (holds the
keyboard / writes the file). Others observe and make suggestions. Driver role rotates on a timer
or trigger. Human mob evidence: 15% slower initial development, 28% faster overall delivery due
to fewer defects and no knowledge transfer delays [source,
futurice.com/blog/mob-programming]. Continuous code review as a side effect.

**Cost multiplier:** N× observation tokens per turn (every agent reads the workspace). High.

**Known failure modes:** For LLM agents: no attention signal differentiates "driver output"
from "observer suggestion" without explicit protocol. High token cost per round. If agents
cannot hold each other accountable (no persistent memory), mob discipline degrades.

**Needs live messaging?** Structured turn protocol required. File-based if the driver writes
and observers append suggestions to a separate comments file.

---

### 1.13 Parallel-Independent-then-Merge

**Mechanism:** N agents independently tackle the same (or complementary) aspects of a problem with
no knowledge of each other's work until completion. A lead then merges or selects. This is the
current fan-out-then-gate shape in soviet-simulator.

**What it buys:** Zero inter-agent contamination. Maximum independence = maximum diversity of
outputs. Cheap to implement.

**Cost multiplier:** N× for the parallel work. Low merge overhead.

**Evidence:** Anthropic explicitly recommends this for "parallelization" tasks: "multiple content
reviewers flagging different vulnerabilities independently" [source, building-effective-agents].
The independence is the feature: contamination is eliminated.

**Known failure modes:** Agents may duplicate work or leave gaps without task boundaries. Merger
(the lead) becomes the bottleneck and must synthesise without seeing reasoning. Anthropic
documented both issues in their research system: "subagents duplicate work, leave gaps, or fail
to find necessary information without detailed task descriptions" [source, multi-agent blog].

**Needs live messaging?** No. Core file-based pattern.

---

### 1.14 Hierarchical Teams

**Mechanism:** Multiple layers of orchestrators and workers. Top orchestrator delegates to
sub-orchestrators; each sub-orchestrator manages its own worker pool.

**What it buys:** Handles massive scope. Scales to many-agent deployments.

**Cost multiplier:** Exponential in theory; practical implementations cap at 2–3 levels to avoid
coordination collapse.

**Known failure modes:** Inter-agent misalignment across levels (MAST FC2). The top orchestrator
sees summaries of summaries — each summarization step is a lossy compression [source, Cognition
blog, and "lost in the middle" finding in context research].

**Needs live messaging?** Depends on implementation. File-based hierarchies possible.

---

### 1.15 Handoff / Swarm

**Mechanism:** An agent executes until it determines a different specialist is better suited, then
transfers control with a context handoff object. No central coordinator. OpenAI Swarm (Oct 2024,
educational) / Agents SDK (Mar 2025, production): handoff is a primitive returning a target agent
[source, openai.github.io/openai-agents-python, github.com/openai/swarm]. LangGraph swarm:
~40% reduction in end-to-end response time vs supervisor pattern [source,
dev.to/focused_dot_io/multi-agent-orchestration-in-langgraph].

**What it buys:** Low coordination overhead; no bottleneck supervisor.

**Known failure modes:** Specialist agent's internal loop outputs do not propagate to parent —
"customers see the handoff but lose the specialist's response" [source, LangGraph guide]. Infinite
loops possible; requires explicit recursion limits.

**Needs live messaging?** Requires live agent-to-agent transfer. Not compatible with pure
file-based + lead model.

---

## Summary Pattern Table

| Pattern | Mechanism | Cost × | Evidence quality | Known failure | Needs live? |
|---|---|---|---|---|---|
| Orchestrator–Worker | Lead decomposes, workers execute in parallel | 4–15× | High (Anthropic production) | Overscaling, step repetition | No |
| Pipeline / Assembly Line | Fixed specialist sequence | 3–6× | Medium (MetaGPT/ChatDev) | Verification theatre | No |
| Planner–Executor–Critic | Spec → impl → feedback loop | 2–4× per round | Medium (Anthropic blog) | Critic bias, premature accept | No |
| Generator–Verifier (Blind) | N candidates, separate verifier | 4× for N=3 | High (arxiv 2607.05391) | Judge bias | No |
| Debate / Adversarial | Structured argument rounds | 4–8× | Medium (Du et al 2023) | Confident-liar flip | No |
| Reflection / Self-Critique | Agent critiques own output | 1.5–2× | Medium (Reflexion 2023) | No oracle, diminishing returns | No |
| Best-of-N + Judge | N independent runs, judge selects | N× | High (SWE-Bench data) | Systematic errors in all N | No |
| Tournament | Pairwise elimination | O(N log N) | Low for coding | Position/verbosity bias | No |
| Blackboard / Shared WS | All read/write shared file | 1.5× | Low empirical | Context bloat, stale reads | No |
| Market / Auction | Agents bid on tasks | High | Very low | Complex coordination | Yes |
| Pair: Driver/Navigator | Driver implements; Navigator steers | 2× | Medium (PairCoder ASE 2024) | Context fragmentation | Structured turns |
| Mob / Rotating Driver | All observe, one drives, rotates | N× | Low (human proxy) | No LLM attention signal | Structured turns |
| Parallel-Independent-Merge | N independent, lead merges | N× | High (Anthropic rec.) | Gaps/duplicates without tight briefs | No |
| Hierarchical Teams | Multi-level orchestrators | 3–10× | Medium (frameworks) | Summary-of-summary loss | No |
| Handoff / Swarm | Agent-to-agent control transfer | 1–2× | Medium (LangGraph) | Lost specialist outputs | Yes |

---

## 2. Collaborative Build Stage Designs for Coding Agents

### 2.1 Design A: Implementer + Spec-Guardian (Driver/Navigator)

**Roles:**
- **Driver (sim-implementer):** Reads the brief, writes Rust code, runs `cargo test -p simulation`,
  appends findings and test output to a shared scratchpad file.
- **Navigator (spec-guardian):** Does NOT write code. Holds the specification and acceptance
  criteria. After each Driver commit/checkpoint, reads the scratchpad and the diff, then writes
  a structured critique: spec deviations, missing acceptance checks, suggested next step.

**Turn protocol:**
1. Lead writes brief → Driver brief.
2. Driver runs until: all acceptance criteria pass, or 3 rounds, or a blocker.
3. Driver writes `DRIVER_PASS_N.md` with diff path, test output, open questions.
4. Navigator reads brief + `DRIVER_PASS_N.md`, writes `NAVIGATOR_PASS_N.md`:
   - PASS: lists criteria met, flags residual risks.
   - RETURN: lists specific deviations, provides the correction target.
5. If RETURN: Driver starts pass N+1. If PASS: Lead reviews and gates.

**File lock:** Driver owns the source files; Navigator is read-only on source. No conflicts.

**Disagreement resolution:** Navigator's spec-deviation verdict is final unless it misread the spec.
Lead arbitrates on spec ambiguities only.

**Evidence basis:** PairCoder (ASE 2024) shows navigator-guided multi-plan exploration produces
better code than driver-only; human pair programming meta-analysis shows quality gains at high
complexity. Spec-guardian role is novel framing — unverified at LLM scale.

**Token multiplier:** ~2.5× a solo implementer (two agents, each running for 2–3 turns, reading
shared context each turn). Estimate per ticket: if solo implementer costs 80k tokens, this costs
~200k.

---

### 2.2 Design B: Implementer + Live Test-Writer Racing

**Roles:**
- **Implementer:** Writes production code to satisfy the spec.
- **Test-Writer:** Reads the same spec simultaneously, writes adversarial tests (including edge
  cases the implementer is unlikely to consider), targeting the acceptance criteria and known
  invariants. The Test-Writer does not see the implementation until tests are written.

**Turn protocol:**
1. Lead issues brief to both agents simultaneously (parallel launch).
2. Implementer writes `src/...` files; test-writer writes `tests/...` files.
3. Both finish and signal done to the lead via a sentinel file.
4. Lead runs `cargo test -p simulation` — this is the integration point.
5. Red tests → Implementer gets the test output and fixes. Test-writer may add a follow-up test
   if fix reveals new behaviour.
6. All green → evidence-auditor gate (existing Phase 3).

**File lock:** Source and test directories are disjoint. No lock needed during parallel phase.

**Disagreement resolution:** The tests are ground truth. Implementer must make tests pass; test-writer
cannot change tests to match a wrong implementation.

**Evidence basis:** AgentCoder (arxiv 2312.13010) demonstrates independent test-designer improves
code quality vs single agent; "test design handled independently of code authoring to ensure
objectivity." The generator-verifier asymmetry evidence (87% vs 63% accuracy) supports the value
of a verifier who did not participate in generation.

**Token multiplier:** ~2× (parallel, so wall-clock is the same as a solo run; tokens are doubled).
Test-writer run is typically shorter than the implementer's full implementation run.

---

### 2.3 Design C: Two Implementers + Judge (Best-of-2)

**Roles:**
- **Implementer-A and Implementer-B:** Given the same brief independently, each produces a
  complete implementation without seeing the other's work.
- **Judge (lead or a reviewer agent):** Reads both implementations, runs both test suites, and
  selects the better one — or cherrypicks parts.

**Turn protocol:**
1. Parallel brief dispatch to A and B.
2. Both write to isolated directories: `src_a/` and `src_b/`.
3. Lead runs tests for each, writes pass/fail to `judge_input.md`.
4. Judge (or lead) reads `judge_input.md` + both diffs, picks one or merges, writes rationale.
5. Merged result enters Phase 4 (reviewer gate).

**File lock:** Two independent directories — no conflict.

**Disagreement resolution:** Judge is final. If both fail tests, lead picks the least-bad for a
second round, or escalates.

**Evidence basis:** Best-of-N on SWE-Bench Verified: oracle Pass@3 = 84.4% vs mean Pass@1 = 76.1%;
a judge captured 78.2% [source, arxiv 2607.05391]. The gain is real (2.1pp over mean Pass@1) but
modest relative to the cost.

**Token multiplier:** ~2.5× (two full implementations + one judge pass). For tickets where first-
attempt pass rates are low (complex sim logic), the diversity of solutions provides the biggest gain.

**Caution:** The gain is smaller when both agents share the same base model and will converge on
the same wrong approach. Use this pattern only when ticket complexity is high.

---

### 2.4 Design D: Spec-Mob (Three Amigos Variant)

**Roles (pre-implementation, planning stage):**
- **Domain advisor** (e.g. kornai-economist or logistics-modeller agent): Reads the raw ticket,
  checks it against domain invariants, flags contradictions or missing constraints.
- **Implementer** (sim-implementer): Reads the ticket, writes an implementation sketch — what
  systems it will touch, what edge cases it sees, what it doesn't know.
- **Evidence-auditor / test planner:** Reads ticket + implementer sketch, writes acceptance
  criteria and test cases that would prove the feature works — before any code is written.

**Turn protocol:**
1. Lead creates ticket with title + description only (no pre-baked acceptance criteria).
2. All three agents run in parallel against the ticket text.
3. Each writes to: `amigos_domain.md`, `amigos_impl.md`, `amigos_test.md`.
4. Lead reads all three, resolves conflicts, writes the authoritative brief with acceptance
   criteria derived from the three-way review.
5. Implementation proceeds under Design A or B.

**File lock:** All three write to separate files. No conflict.

**Disagreement resolution:** Lead synthesises. Any domain-advisor veto on a constraint is binding
(same authority as kornai-economist gate sign-off on economy work).

**Evidence basis:** Human Three Amigos literature documents the pattern's value in surfacing
requirement gaps before build, but finds no controlled quantitative study on defect reduction
[source: Wrike/Agile Alliance descriptions; George Dinwiddie 2014]. The LLM analogue for
domain-advisor + implementer pre-brief is novel — **unverified at LLM scale** beyond this
researcher's reasoning from first principles.

**Token multiplier:** ~1.5× a solo planner pass (three shorter reads, not a full implementation).
This replaces the lead's planning pass, not adds to it, so marginal cost is the domain-advisor and
test-planner runs only.

---

## 3. Collaboration in Other Stages

### 3.1 Planning: Three Amigos (see Design D above)

The human Three Amigos practice (business analyst + developer + tester, introduced by George
Dinwiddie, 2014) maps cleanly to LLM agent roles: domain advisor + implementer + test-planner.
No controlled LLM evidence found. **[unverified]**

### 3.2 Review: Independent Blind vs Sequential

**Human evidence (Microsoft):** Two experienced reviewers is the empirically optimal number;
adding more degrades per-reviewer engagement [source, Porter et al, cited in
michaelagreiler.com/code-review-best-practices]. Google's practice: 75% of changes reviewed by
one reviewer [source, same]. Reviewed code: 20–30% fewer production defects vs unreviewed [source,
microsoft.com/en-us/research/wp-content/uploads/2016/02/bosu2015useful.pdf].

**Independent blind vs sequential for LLM reviewers:** MAST finding: FC2.5 (ignored other agent's
input) occurs in 1.9% of traces, and FC2.3 (task derailment) in 7.4%. Sequential reviewers
exhibit anchoring — the second reviewer's feedback converges toward the first's framing [source,
"Anchoring Bias in LLM-as-a-Judge," arxiv 2608.25869]. **Recommendation: run two reviewers
in parallel (blind), then have the lead merge findings.** The cost is ~2× review time; the gain
is independence.

**LLM-as-judge bias inventory [source, arxiv 2411.16594, arxiv 2606.19544]:**
- Position bias: earlier-presented candidate favored
- Verbosity bias: longer answers rated higher
- Self-preference: model prefers own outputs
- Authority bias: outputs attributed to prestige sources rated higher

Mitigation: blind review (no author identity), structured rubric, calibration passes where judge
scores known-bad examples before live use.

### 3.3 Retrospective

**Human analogy:** Agile retrospective — what worked, what didn't, what to change.

**LLM evidence:** Self-reflection studies show LLMs can identify surface errors but cannot reliably
diagnose deep failure causes without an oracle [source, arxiv 2405.06682]. A retrospective agent
reading session transcripts and ticket histories can pattern-match on known failure taxonomies
(MAST 14 failure modes) but may confabulate explanations. **Verdict: useful for flagging known
failure patterns; unreliable for root-cause attribution. Run as an input to the lead's synthesis,
not as a standalone artefact.** [Partially verified; no controlled study on LLM retrospective
quality found.]

### 3.4 Backlog Refinement

The Spec-Mob (Design D) is the direct LLM analogue of backlog refinement. The three-amigos pre-
brief pattern is this researcher's primary recommendation for applying collaboration to the planning
stage. Cost is low; the constraint-surfacing value is highest before code is written.

---

## 4. Context-Passing Discipline

### 4.1 Summaries vs Raw Sources

"Lost in the middle" problem: LLMs cannot properly use relevant information buried in vast contexts
even with windows exceeding 1M tokens [source, arxiv 2508.21433]. Observation masking (structured
filtering, not LLM summarization) is 52% cheaper than raw-baseline context while improving solve
rates by 2.6%; LLM summarization is more expensive and no more effective [source, arxiv 2508.21433,
"The Complexity Trap"].

**Rule: prefer structured excerpts over LLM-generated summaries for context passing.** Cite the
source file and lines; do not paraphrase. When a summary is necessary (context budget), use a
structured schema (JSON or markdown headers) rather than free-text prose.

### 4.2 Memory Contamination

Toxic or adversarial context compressed into memory summaries can remain below standard toxicity
detectors while still influencing downstream generations [source, arxiv 2605.16746]. Cross-user
contamination in shared state: "persistent state becomes shared across users, creating opportunities
for cross-user information transfer" [source, arxiv 2604.01350].

**Rule:** Agent A's prior reasoning must not be passed to Agent B if B is intended to be an
independent reviewer. Use separate context windows for independent agents.

### 4.3 "The Brief is the Contract"

Anthropic's multi-agent system failure analysis: "subagents duplicate work, leave gaps, or fail
to find necessary information without detailed task descriptions." Each subagent needs: objective,
output format, tool guidance, and clear task boundaries [source, multi-agent research blog]. This
aligns with the soviet-simulator CLAUDE.md traps-in-description convention.

### 4.4 LLM-as-Judge Bias (Summary)

Documented biases: position, verbosity, self-preference, authority, anchoring (prior scores
contaminate independence) [sources: arxiv 2608.25869, arxiv 2606.19544, arxiv 2411.16594].
Mitigation: blind evaluation, structured rubric, calibration. Cross-study comparison of LLM
evaluation results is "effectively impossible" [source, arxiv 2606.19544].

---

## 5. Verdict: Ranked by Value-per-Token for a Small Token-Budget Team

**Tier 1: Worth their cost, adopt now**

1. **Parallel-Independent-then-Merge (current model):** Zero contamination, cheap, proven.
   Improvement target: tighter task boundaries to eliminate duplicate work.

2. **Generator–Verifier (Blind), N=2 or N=3:** The verification asymmetry is real and measured.
   Use for high-complexity, low-first-pass-rate tickets (complex sim logic). Cost: ~3–4×.

3. **Spec-Mob / Three Amigos (pre-brief only):** Low cost (~1.5×), highest value before code is
   written. Catches constraint gaps the lead misses. Recommended as a Phase 0 add for any ticket
   touching economy, logistics, or settlement invariants.

4. **Blind parallel review (two reviewers, independent):** Replace sequential review with parallel.
   Human evidence strong; LLM anchoring bias evidence strong for why sequential is worse. Cost: 2×
   review, same wall-clock time.

**Tier 2: Use selectively, high complexity only**

5. **Implementer + Live Test-Writer (Design B):** Strongest evidence of defect reduction for
   coding agents (AgentCoder; generator-verifier asymmetry). Best for tickets with testable
   acceptance criteria. Cost: ~2×.

6. **Driver/Navigator (Design A):** PairCoder evidence is promising but limited to code-generation
   benchmarks; spec-guardian framing is novel. Use when a spec is complex and the implementer
   has previously misread it. Cost: ~2.5×.

7. **Planner–Executor–Critic:** Use when evaluation criteria are binary and testable. Avoid when
   criteria are aesthetic or complex — critic bias and incorrect verification (9.1% of MAST
   failures) make it unreliable.

**Tier 3: High cost, limited payoff at current model capability**

8. **Best-of-2 Implementers + Judge (Design C):** 2.1pp gain on SWE-Bench vs mean; real but
   modest. Worth it only when first-pass failures are expensive to fix in Phase 4.

9. **Orchestrator–Worker:** Essential for research/information tasks (90.2% gain at Anthropic);
   less clearly superior for implementation tasks where subtasks share state.

10. **Debate / Adversarial:** Confident-liar failure mode is severe. Use only when agents have
    been verified to hold independent positions (not contaminated by each other's priors). No
    coding-specific controlled study found. **Mostly theatre for implementation tasks.**

**Tier 4: Not recommended on this token budget**

11. **Mob / Rotating Driver:** Human evidence is promising; no LLM-specific evidence; very high
    token cost (N× per turn for N observing agents). Skip unless budget relaxes significantly.

12. **Tournament:** O(N log N) comparisons; position/verbosity bias undercuts the premise. Use
    only for evaluation dataset curation, not task completion.

13. **Market / Auction:** No rigorous evidence; high coordination complexity. Skip entirely.

14. **Handoff / Swarm:** Requires live messaging infrastructure not in the current file-based
    model. Adoption cost is high; gain is speed, not quality.

---

## 6. Gaps and Open Questions

- **Spec-Mob (Design D) at LLM scale:** The three-amigos pre-brief is the highest-ROI
  recommendation with the weakest empirical backing. A controlled within-project A/B test (50
  tickets with vs without the amigos pass, measure Phase 4 send-back rate) would close this.

- **MAST failure mode distribution by pattern type:** The MAST paper identifies
  14 failure modes with frequency counts but the paper was not fully read; FM-1.3 step-repetition
  (15.7%) and FM-1.5 termination-unaware (12.4%) are the top two. A full read of the HTML version
  would surface more pattern-specific evidence.

- **Debate for complex invariant checking (ledger, economy):** Multi-agent debate shows reasoning
  improvement on arithmetic tasks. The ledger-invariant-checker agent's adversarial role is
  adjacent to this pattern. No controlled test of whether debate between a ledger-checker and an
  implementer reduces conservation errors more than a single checker. Hypothesis: worth testing on
  one ledger-touching ticket.

- **Context window practical limits for blackboard pattern:** At what shared document size does
  reading overhead exceed the gain from shared state? This depends on model context efficiency.
  No measurement for Claude Sonnet 4 on Rust diffs found.

---

## Sources

- Anthropic, "Building Effective Agents": https://www.anthropic.com/engineering/building-effective-agents
- Anthropic, "How we built our multi-agent research system": https://www.anthropic.com/engineering/multi-agent-research-system
- MAST taxonomy, "Why Do Multi-Agent LLM Systems Fail?" (NeurIPS 2025): https://arxiv.org/abs/2503.13657
- MAST HTML version: https://arxiv.org/html/2503.13657
- LLM-as-a-Verifier (Stanford, 2026): https://arxiv.org/html/2607.05391v1
- "The Complexity Trap" (observation masking): https://arxiv.org/html/2508.21433v1
- PairCoder (ASE 2024): https://arxiv.org/abs/2409.05001
- AgentCoder (2023): https://arxiv.org/abs/2312.13010
- MetaGPT: https://arxiv.org/html/2308.00352v6
- AutoGen: https://arxiv.org/pdf/2308.08155
- CAMEL (NeurIPS 2023): https://arxiv.org/abs/2303.17760
- Cognition, "Don't Build Multi-Agents": https://cognition.com/blog/dont-build-multi-agents
- OpenAI Swarm: https://github.com/openai/swarm
- OpenAI Agents SDK: https://openai.github.io/openai-agents-python/
- LangGraph supervisor vs swarm: https://dev.to/focused_dot_io/multi-agent-orchestration-in-langgraph-supervisor-vs-swarm-tradeoffs-and-architecture-1b7e
- "Talk Isn't Always Cheap: Failure Modes in Multi-Agent Debate": https://arxiv.org/html/2509.05396v1
- "The Confident Liar" (debate failure): https://arxiv.org/pdf/2606.10296
- Self-Reflection in LLM Agents: https://arxiv.org/pdf/2405.06682
- Anchoring Bias in LLM-as-a-Judge: https://arxiv.org/html/2608.25869
- LLM-as-a-Judge Systematic Review: https://arxiv.org/pdf/2411.16594
- State Contamination in Memory-Augmented LLM Agents: https://arxiv.org/abs/2605.16746
- Pair programming meta-analysis (Hannay et al. 2009): https://www.sciencedirect.com/science/article/abs/pii/S0950584909000123
- Mob programming evidence: https://www.futurice.com/blog/mob-programming
- Microsoft code review characteristics: https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/bosu2015useful.pdf
- Du et al. 2023, multi-agent debate (ICML): referenced in https://arxiv.org/html/2509.05396v1
- Reflexion (Shinn et al. 2023): referenced in https://arxiv.org/pdf/2405.06682
