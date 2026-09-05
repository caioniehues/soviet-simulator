# Report 09 — code-review-skill and socratic

**Kind:** explanation
**Authority:** advisory
**Status:** active
**Owner:** researcher
**Last verified:** 2026-08-28

Repos cloned shallow into scratchpad. All file:line citations reference the cloned trees unless
noted.

---

## Part A: awesome-skills/code-review-skill

### Repo metadata [source]

- URL: https://github.com/awesome-skills/code-review-skill
- SHA: 277f4793018448eb08289bd54ab3c0e241a02b7b (2026-08-23)
- Stars: 1,839 / Forks: 192
- License: MIT

### A1. What it does — protocol, step by step [source: SKILL.md]

The skill is a **Claude Code skill** (frontmatter `name: code-review-skill`), loaded by a user
invoking it in a Claude Code session. It does **not** run tests itself; it has `Bash` in its
allowed-tools list ("run lint/test/build commands to verify code quality") but there is no
mandatory test-run step — test-running is optional and checker-prompted. It does **not** use
subagents; the workflow is single-agent.

**Phase 1 — Context Gathering (2-3 min)**
Read the PR description, check PR size (flag >400 lines; ask to split), check CI/CD status,
understand the business requirement, note architectural decisions.
For large diffs: pipe through `scripts/pr-analyzer.py` (`git diff main...HEAD | python
scripts/pr-analyzer.py`) to triage complexity before reading — SKILL.md:105.

**Phase 2 — High-Level Review (5-10 min)**
Architecture & Design (SOLID, coupling/cohesion, anti-patterns; consult
`reference/architecture-review-guide.md` for significant changes).
Performance Assessment (algorithm complexity, N+1, memory; consult
`reference/performance-review-guide.md`).
File organization. Testing strategy.

**Phase 3 — Line-by-Line Review (10-20 min)**
Per file: Logic & Correctness, Security, Performance, Maintainability, Reuse.
"Reuse" check is explicit: search the tree for existing utilities/helpers that could replace new
code — SKILL.md:125. References `reference/code-quality-universal.md` for anti-patterns
(parameter sprawl, leaky abstractions, nested conditionals, stringly-typed code, TOCTOU,
no-op updates).

**Phase 4 — Summary & Decision (2-3 min)**
Summarise key concerns, highlight positives, make a clear decision (Approve / Comment /
Request Changes), offer to pair if complex.

**Severity scheme** — three blocking tiers plus three annotation markers (SKILL.md:170-178):
- 🔴 `[blocking]` — must fix before merge
- 🟡 `[important]` — should fix, discuss if disagree
- 🟢 `[nit]` — nice to have, not blocking
- 💡 `[suggestion]`, 📚 `[learning]`, 🎉 `[praise]` — non-blocking annotations

**Large diff handling:** the `scripts/pr-analyzer.py` script (Python, ~120 lines) reads the diff
from stdin, classifies files by complexity, and prints a triaged review order with risk scores.
No automatic splitting; the reviewer must ask the author to split if >400 lines.

**No language-specific dispatch** in the main protocol — the reviewer is expected to consult
`reference/<lang>.md` files themselves. The Rust file (`reference/rust.md`) exists and is
substantial (847 lines): ownership/borrow, unsafe with SAFETY comments, async correctness,
cancellation safety, spawn vs await, error handling (thiserror vs anyhow split), performance
(Cow, iterator laziness, string allocation), trait design, and a full checklist.

**Triggering:** the skill is loaded by the user; it has no hooks or automated triggers.

---

### A2. Comparison against our gate chain [source: reviewer.md, gate-review.js, wiring-auditor.md, ledger-invariant-checker.md, evidence-auditor.md]

#### What CRS checks that we do not (or do more explicitly)

| CRS feature | Where in CRS | Our gap |
|---|---|---|
| Reuse audit — explicit search for existing utilities that could replace new code | SKILL.md:125, `code-quality-universal.md` | `reviewer.md` has no explicit reuse step; our gates check correctness, wiring, conservation |
| `pr-analyzer.py` — automated diff triaging before reading | SKILL.md:105, `scripts/pr-analyzer.py` | No equivalent; our reviewer reads the whole diff |
| Architecture & design as a named Phase 2 | SKILL.md:108-116 | `reviewer.md` §2 checks acceptance criteria and correctness but no named arch step |
| Performance as a named Phase 2 | SKILL.md:112-114 | Not a named check in `reviewer.md` |
| Rust-specific: cancellation safety, `select!` safety, structured concurrency, `spawn` vs `await` | `reference/rust.md:280-550` | Our `reviewer.md` has no Rust-specific checks; `wiring-auditor.md` has four house defect shapes (silent default, panic on live path, unseen-fail guard, search-tool absence) but not async cancellation |
| Rust-specific: `unsafe` block must carry `SAFETY` comment | `reference/rust.md:102-187` | Not in any gate file |
| Rust-specific: `thiserror` vs `anyhow` split (library vs application) | `reference/rust.md:560-575` | Not in any gate file |
| Praise / learning annotations in output | SKILL.md:175-178 | Not in our review format |
| Offer to pair if complex | SKILL.md:133-135 | Not in our reviewer |

#### What we do that CRS does not

| Our feature | Where ours | CRS gap |
|---|---|---|
| **Blindness** — reviewers never see the builder's summary or each other's findings until dedup | `gate-review.js:59-89` | CRS is single-reviewer; no anchoring protection |
| **Double skeptic verify** — every finding challenged on "is it true?" and "does it matter?" before surviving | `gate-review.js:103-165` | No equivalent; CRS reviewer rates severity but no one refutes |
| **Completeness critic** — a separate agent asks what dimension nobody checked | `gate-review.js:171-204` | No equivalent |
| **Re-derive from source, never from the builder's summary** | `reviewer.md:3` ("verifies line by line"), `gate-review.js:108-113` (skeptic reads actual file) | CRS has no explicit prohibition; single reviewer, may rely on PR description |
| **Wiring gate** — distinct agent asking only "is this reachable?" before the expensive gate | `wiring-auditor.md` | CRS has no reachability check |
| **Ledger gate** — conservation of quantity and money across every seam | `ledger-invariant-checker.md` | Not present; CRS is domain-agnostic |
| **Evidence gate** — every guard seen failing before believed | `evidence-auditor.md` | CRS's testing checklist does not require mutation testing |
| **Structured finding schema** with `file`, `line`, `severity`, `claim`, `evidence`, `reproduce` | `gate-review.js:18-42` | CRS has labels but no structured output schema |
| **Verdict cap** — PASS / PASS-WITH-FINDINGS / BLOCK only on CONFIRMED blocker | `gate-review.js:240-246` | CRS verdict is Approve / Comment / Request Changes; no confirmation requirement |
| **Four project-specific defect shapes** embedded in gate agents | `wiring-auditor.md:130-188` | CRS has generic red flags; none specific to ECS/Egregoria patterns |

---

### A3. Steal list [source: SKILL.md, reference/rust.md]

**Steal — high value**

1. **Rust async/cancellation checklist** (`reference/rust.md:280-395`).
   We have ECS systems that use async. Neither `reviewer.md` nor any specialist gate checks
   `select!` safety, `read_exact` vs `read` cancellation, or `tokio::pin!` re-use. This is a
   compiler-invisible defect class.
   → Land in: `reviewer.md` body, under a "Rust-specific checks" section. Could also become a
   `.claude/rules/simulation.md` trap triggered whenever `simulation/**` is read.

2. **Explicit reuse audit step** (`SKILL.md:125`): before accepting new code, search adjacent
   files and shared modules for existing utilities. We have code-quality problems documented
   in report 06; this step is cheap and structural.
   → Land in: `reviewer.md` §2, between correctness and scope creep. One line: "Before accepting
   new functions, grep tree for existing helpers that could replace them."

3. **Unsafe SAFETY comment requirement** (`reference/rust.md:102-187`).
   ECS and low-level game code uses `unsafe`. No gate currently checks for missing `# Safety`
   doc or `// SAFETY:` block comments. Compiler-invisible defect.
   → Land in: `reviewer.md` body; or a `.claude/rules/` trap for any crate with `unsafe`.

4. **`pr-analyzer.py` diff triaging** (`scripts/pr-analyzer.py`).
   Our gate-review.js already caps verify at 8 findings (`VERIFY_CAP`, line 95). A triager
   that risk-scores files before the blind review pass would let dimension prompts focus on
   high-risk files first.
   → Land in: `gate-review.js`, as an optional pre-phase that produces a file-risk list injected
   into each dimension's `COMMON` context. MIT license allows it.

5. **`thiserror` vs `anyhow` split** (`reference/rust.md:560-575`). Library crates must not
   expose `anyhow::Error` to callers. This codebase has multiple crates with public APIs.
   → Land in: `reviewer.md` Rust section.

**Steal — lower priority**

6. **Severity label vocabulary** (🔴/🟡/🟢) — our gate-review.js uses blocker/major/minor/process
   in the schema but our reports do not consistently surface the labels in a scannable way.
   Aligning report format to the label vocabulary makes findings triage faster.
   → Land in: `gate-review.js` synthesis prompt, `reviewer.md` report section.

**Do not steal**

- The four-phase process as a replacement for our gate chain. CRS is a general reviewer skill;
  our gates are adversarial specialists with measured 7-finding track record. We are strictly
  stronger on correctness assurance; CRS is broader on language breadth and review culture.
- The "praise" annotation. This project's review culture is adversarial-gates-then-merge, not
  collaborative mentoring. Adding praise to gate output would dilute the signal.
- The "offer to pair if complex" gesture. The Pair play already exists as a lane decision
  (gosplan.md §3.6); a reviewer offering it ad hoc conflicts with the routing authority.
- CRS's single-reviewer structure. Our blind parallel review + skeptic verify is measured (87%
  vs 63% accuracy asymmetry, report 05 §2.2). Do not collapse it.

---

## Part B: m4vic/socratic

### Repo metadata [source]

- URL: https://github.com/m4vic/socratic
- SHA: 3cfaf6e73a8be29d4a8ed5ca80aa03a05f9aad7f (2026-08-14)
- Stars: 114 / Forks: 17
- License: MIT

### B1. What it is [source: SKILL.md, PROMPT.md, agents/openai.yaml]

Socratic is a **Claude Code skill** (also ships a portable system-prompt variant `PROMPT.md` for
ChatGPT/Gemini/local models). It is neither an agent nor a multi-agent system. It defines a
**self-interrogation method** — not a user interview — executed by the agent that loads it.

**The 697-question bank** is split into 15 domain files (`questions/00-requirements.md` through
`questions/14-team-maintenance.md`), with a `questions/core/` subtree of shorter versions.
**Ten knowledge packs** (`packs/`) provide domain-specific overlays for agent design, AI
engineering, data systems, domain modeling, legacy change, operations, product discovery, software
design, testing design, threat modeling.

**Question protocol — Mode A (default)**
1. Build a domain set dynamically from request signals. Always include requirements + testing.
   Add domains as mid-task signals emerge. (`SKILL.md:23-43`)
2. Choose Core (routine/prototype) or Full (production/PII/money) question depth. (`SKILL.md:47-53`)
3. Load relevant packs (0-2 per task) from `packs/registry.md`. (`SKILL.md:59-82`)
4. Optionally activate a grade (MVP / production / enterprise) which changes the stopping
   condition. (`SKILL.md:83-89`)
5. Self-answer every selected question using: codebase first, then engineering defaults, then
   escalate to user only for authority decisions. (`SKILL.md:93-99`)
6. Sufficiency check: stop when outcome is clear, material risks are mitigated, no contradiction
   changes the plan, and the next question would not materially change anything. (`SKILL.md:101-115`)
7. Emit output contract once: Domains considered / Self-answered highlights / Assumed / Open
   questions (ideally 0-3) / Top risks / Plan. (`SKILL.md:117-128`)
8. Build and verify. (`SKILL.md:133-135`)

**Question ordering:** Priority 1 questions (requirements domain) must run first. Within a domain,
questions run in file order. No explicit stop condition per question — the sufficiency check (step
6) operates at the domain/cluster level.

**Output format:** one structured block before implementation, not a dialogue. The user sees the
contract, not the raw question bank.

**Mode B — Interactive** (opt-in only, `SKILL.md:138-144`): one yes/no question at a time, each
with a recommended default. Budget: 0-2 for throwaway, 3-6 prototype, 8-15 production, 15-25
for money/PII/health.

**Stop conditions:** the sufficiency check's five conditions (SKILL.md:107-114), or the active
grade's gate. The skill explicitly says "do not stop merely because a token budget is low."

**What it produces:** a structured pre-build contract (assumptions, open questions, top risks,
plan). No code, no artifact — it feeds the build phase.

**Triggering:** loaded by the user via the skill invocation system. `$socratic lite` = core;
`$socratic deep` / `$socratic full` / `$socratic audit` = full. Preset domain combinations exist
as a sanity-check table (`SKILL.md:147-160`).

---

### B2. Where it fits in GOSPLAN [source: gosplan.md §3.5, development-cycle.md]

#### (a) Bet/Refine — interrogating a story before it becomes a brief

**Fit:** Very high. Socratic's Mode A matches what a brief author (gosplan) should do silently:
inspect the codebase, apply engineering defaults, surface only the 0-3 decisions that require
the Planner. The output contract maps onto the brief template fields (gosplan.md §5.2): problem,
assumed substrate, traps (= top risks), acceptance criteria, out-of-scope list.

The `/grill` verb in the retired `compass` plugin (`development-cycle.md`: "grill a claim before
it becomes a decision") was nominally similar but is dead — compass was retired with the Bevy
track. Socratic is a working replacement, but **much more structured**: it does not grill the
user; it grills the brief internally.

For **L-lane stories** (new mechanic, cross-lane), Socratic's domain-selection logic would drive
which advisors the Spec-Mob should include: signals for "economy seam" → `packs/data-systems` →
`kornai-economist`; signals for "agent design" → `packs/agent-design`. This makes Spec-Mob
participant selection mechanical rather than gosplan judgment.

For **M-lane stories**, Socratic's Mode A (silent, one contract) is the brief-writing step
gosplan already does informally. Making it explicit and skimmable by the Planner reduces the
DoR review cost.

**Evidence from literature:** "Ask-before-Plan" (arXiv:2406.12639) shows proactive clarification
before planning reduces downstream replanning; "Structured Uncertainty guided Clarification"
(ResearchGate) shows uncertainty-guided clarification substantially improves task success while
asking fewer questions. Both support the Refine step. Note: both papers study interaction with
external users, not self-interrogation; the Socratic Mode A self-answer removes the latency cost
but also removes the signal of a human reply. **Unverified whether self-interrogation quality
equals interactive clarification quality at the scale of LLM planning tasks.**

**Cost:** loading core domain files (~5 files × ~50 questions = ~250 questions) + one output
contract. At sonnet, probably 30-60k tokens per brief. At the current brief-writing cost (no
measurement exists), this may be neutral or slightly cheaper because it prevents a send-back.
**Unverified.**

#### (b) Spec-Mob

**Fit:** Partial. The Spec-Mob already runs three agents in parallel (advisor + builder +
evidence-auditor), each reading the raw ticket. Socratic would be a layer *within* the advisor's
pass: the advisor could self-interrogate using domain files specific to its cluster before writing
`refine_advisor.md`. This is implicit in how the Spec-Mob works but not made explicit. Worth
naming in the advisor agent bodies.

#### (c) Retro

**Fit:** Low. Socratic is pre-build; retro is post-evidence. The sufficiency check ("no
unresolved contradiction changes the plan") cannot run after the fact because the plan has
already executed. A retro needs root-cause analysis, not requirement interrogation. gosplan.md
§3.5 already says "an LLM retro cannot root-cause novel failures (05 §3.3)"; Socratic would not
improve that. Do not add it here.

#### (d) Planner ratifying a mechanism ruling

**Fit:** None. A mechanism ruling is a decision already framed; Socratic interrogates
underspecified requests. The dispute procedure (gosplan.md §3.8) and the ADR template already
structure this. Adding Socratic here would add questions to a phase already well-defined.

#### (e) Dispute procedure

**Fit:** None. Disputes are adversarial (builder vs gate finding); Socratic is cooperative
self-interrogation. The dispute procedure already has a defined path (blind skeptic on the
specific finding). Do not mix them.

---

### B3. Evidence on Socratic questioning and LLM planning outputs [live search]

Papers found (all **unverified** as directly applicable to LLM brief quality):

- **arXiv:2411.00750** (Nov 2024): Socratic-guided sampling improves LLM self-improvement by
  avoiding tail-narrowing in training distributions. Relevant to model training, not to
  brief-writing quality.
- **arXiv:2512.24103** (Dec 2025): Intrinsic self-critique enhances LLM planning on benchmarks
  via iterative correction. Closest to our use case (planning quality); improvement is on
  structured planning tasks, not on engineering brief writing. **Unverified whether this
  generalises to our domain.**
- **arXiv:2406.12639** (Jun 2024, Ask-before-Plan): Proactive agents that ask clarifying
  questions before planning reduce replanning in real-world tasks. **Indirect support**:
  interrogating the brief before build reduces send-backs. No LLM-specific numbers on brief
  quality.
- **ResearchGate: Structured Uncertainty guided Clarification**: uncertainty-guided strategy
  outperforms prompting/uncertainty baselines on task success while minimising questions asked.
  **Indirect support** for Socratic's 0-3 open questions discipline.
- **arXiv:2508.03682** (Self-Questioning Language Models, 2025): self-questioning improves
  reasoning performance. Tangentially relevant.

**Summary:** there is directional evidence that pre-planning interrogation reduces downstream
errors. No paper directly measures Socratic self-interrogation against brief quality in an
engineering-agent context. gosplan.md §3.10 already recommends measuring Spec-Mob send-back
rates in Plans 01 and 02; the same measurement would capture any Socratic benefit.

---

### B4. Steal list, ranked [source: SKILL.md, packs/]

**Steal — high value**

1. **Mode A self-interrogation as the explicit brief-writing discipline** (SKILL.md:14-128).
   gosplan already writes briefs but with no named protocol. Adopting Socratic's contract
   format (domains / self-answered highlights / assumed / open questions / top risks / plan)
   gives the Planner a skimmable artefact at DoR review instead of a prose brief. The 0-3
   open-questions discipline is the key rule: if gosplan has more than 3 blockers for the
   Planner, the brief is underspecified and should not be dispatched.
   → Land in: `docs/templates/brief.md` — add a "Pre-build contract" header that mirrors the
   Socratic contract output. Include in the DoR hook check: if no "Assumed:" section, brief
   is not ready.

2. **Domain-signal table for Spec-Mob participant selection** (SKILL.md:25-42).
   The current Spec-Mob selection is "advisor + builder + evidence-auditor" with no rule for
   which advisor. Socratic's domain signals ("economy seam" → data-systems pack → kornai-economist;
   "agent" → agent-design pack → no current GOSPLAN equivalent) make participant selection
   auditable.
   → Land in: `docs/process/gosplan.md` §3.5 Refine row, as a mapping table.

3. **packs/agent-design/core.md** — the pack's six questions (should this be one agent or
   several? should the doer check its own work? minimal tool set? model tiering? user-facing
   vs dispatched? how to know it worked?) directly apply to every new agent we add to the
   roster. The pack was derived from 34 published Claude Code agent systems.
   → Land in: `docs/templates/agent.md` as a "Design checklist" section. Also useful as a
   pre-Spec-Mob check when the story involves a new agent.

4. **Sufficiency stop condition** (SKILL.md:107-114).
   The five conditions (outcome clear, material risks mitigated, no unresolved contradiction,
   next question immaterial, riskiest assumption falsifiable) are exactly what gosplan should
   use to decide when a brief is ready. Currently the DoR check is mechanical (fields present
   or absent); adding a judgment condition ("next question would not materially change the
   design") prevents over-specified briefs on S-lane stories.
   → Land in: `docs/templates/brief.md` DoR narrative, and the `dor-gate.sh` hook comment.

**Steal — medium value**

5. **Grade system** (grades/mvp.md, grades/production.md, grades/enterprise.md).
   Grades change what "done" means for a run. Maps onto our lane routing: S-lane ≈ MVP grade
   (minimal gates), M-lane ≈ production (full gate chain), L-lane ≈ enterprise (all gates +
   advisor sign-off). Naming the grade in the brief makes the lane decision auditable.
   → Land in: `docs/templates/brief.md` as a field `grade: mvp | production | enterprise`.

**Do not steal**

- The interactive Mode B. We have a Planner who provides authority decisions at seven named
  points (gosplan.md §3.2). Mode B turns the brief into a dialogue, which is exactly the
  "PO-absence pathology" we are fixing. Mode A only.
- The full 697-question bank as a mandatory pass. gosplan.md §3.3 ("the appetite is a limit,
  not an estimate"). Running all 15 domains on an S-lane patch would be theatre. Socratic
  explicitly warns against this ("Do not continue merely to use more questions").
- The portable PROMPT.md system-prompt variant. Our agents use skill frontmatter. The PROMPT.md
  is for other providers.
- The openai.yaml agent definition. We are not using the OpenAI Responses API.

---

## Gaps and what would close them

1. **Measured cost of Socratic brief-writing** at sonnet. No token measurement exists for a
   Socratic Mode A pass on an M-lane brief. Run a shadow brief on the first two Plan stories
   and compare with the existing approach.

2. **Cancellation-safety send-back rate**. Before adding the Rust async checklist from CRS,
   run it against the last 5 committed diffs and count findings. If zero findings, the
   checklist is noise for this codebase (no async in sim core). If hits exist, it earns its
   place.

3. **Self-interrogation vs interactive clarification quality**. No paper directly measures
   Socratic Mode A on engineering brief quality. The planned Spec-Mob measurement (gosplan.md
   §3.10) would also capture this if a control group uses Socratic Mode A and the other group
   uses the current brief-writing protocol.

4. **CRS `pr-analyzer.py` portability** (Python 3, reads stdin). Confirm it runs on CachyOS
   before proposing it for gate-review.js integration.
