# Report 14 — Matt-skills engineering skills: adopt, adapt, discard

**Kind:** gate-report (research)
**Authority:** advisory
**Status:** active
**Verified-at:** 974c932 (matt-skills repo)
**Last verified:** 2026-08-28
**Scope:** four skills in `skills/engineering/` — `improve-codebase-architecture/`, `codebase-design/`,
`domain-modeling/`, `diagnosing-bugs/`; GOSPLAN §3.5 stages 1–3, 6; §4.2; Appendix A.
**Prior report:** 08 covered `to-spec → to-tickets → to-goal`; not repeated here.

Source files verified at the SHA above:
- `skills/engineering/improve-codebase-architecture/SKILL.md` (74 lines)
- `skills/engineering/improve-codebase-architecture/HTML-REPORT.md` (124 lines)
- `skills/engineering/codebase-design/SKILL.md` (115 lines)
- `skills/engineering/codebase-design/DEEPENING.md` (38 lines)
- `skills/engineering/codebase-design/DESIGN-IT-TWICE.md` (45 lines)
- `skills/engineering/domain-modeling/SKILL.md` (75 lines)
- `skills/engineering/domain-modeling/ADR-FORMAT.md` (48 lines)
- `skills/engineering/domain-modeling/CONTEXT-FORMAT.md` (59 lines)
- `skills/engineering/diagnosing-bugs/SKILL.md` (141 lines)
- `skills/engineering/diagnosing-bugs/scripts/hitl-loop.template.sh` (45 lines)

---

## Skill 1: `codebase-design`

### What it is

A **vocabulary document**, not a protocol. Trigger: any design or restructuring discussion, or when
another skill needs the deep-module lexicon. No steps, no stop conditions, no handoff. It defines
eight terms — module, interface, implementation, depth, seam, adapter, leverage, locality — with
one principle each, four design tests (deletion test, "interface is the test surface", one adapter =
hypothetical seam, two = real), and three dependency categories (`DEEPENING.md`). The design-it-twice
pattern (`DESIGN-IT-TWICE.md`) spawns 3+ parallel sub-agents with different interface constraints,
collects their outputs, compares on depth/locality/seam placement, and issues a recommendation.

Load-bearing line: `SKILL.md:23` — "A module is deep when a large amount of behaviour sits behind a
small interface, shallow when the interface is nearly as complex as the implementation."

Load-bearing line: `SKILL.md:65` — "One adapter means a hypothetical seam. Two adapters means a
real one. Don't introduce a seam unless something actually varies across it."

The skill produces no artifact on its own; it is referenced by `improve-codebase-architecture` and
by `domain-modeling`.

### What is genuinely good

The vocabulary is tight and self-consistent. "Seam" is explicitly disambiguated from DDD's "bounded
context" (SKILL.md:22) — a real collision this project has already navigated. The one-adapter rule
is falsifiable and actionable. The dependency-category taxonomy in DEEPENING.md (in-process /
local-substitutable / remote-owned / true-external) maps cleanly onto testing strategy without
requiring mocks: each category determines *which* adapter fills the seam in tests. Design-it-twice
with parallel sub-agents is the only thing here that would cost meaningful tokens.

### What is generic filler

The testability section (`SKILL.md:70–95`, TypeScript examples: "accept dependencies, don't create
them") is textbook DI advice. It adds nothing for a Rust/ECS codebase and uses the wrong idiom
(class constructor injection vs Rust trait objects and component parameters).

### Fit in GOSPLAN

This vocabulary already exists in spirit across our house rules. The delta is naming: we say
"seam" and "deep module" informally; this formalises it. The two-adapter rule codifies a real
discipline — our code has several single-adapter abstractions that have cost merges.

**Where it lands:**
- The eight terms + deletion test + two-adapter rule → a few lines in `.claude/rules/house-rules.md`
  under "Architecture vocabulary". This is a shared-block candidate.
- Design-it-twice → a `planner` play for L-lane interface design. When a stage-2 Spec-Mob surfaces
  competing interface shapes, `planner` (resumed) can run the pattern and fold the winner into the
  brief. No new agent needed.
- Dependency categories (DEEPENING.md) → `brief.md` template under `Verify:` or `Traps:` — the
  brief should state which dependency category applies at each seam the builder will cross.

### Cost

Loading the vocabulary into `house-rules.md` costs nothing at runtime — it is already in context.
A design-it-twice sub-agent fan-out costs ~60–100k per L-lane story it runs on; it replaces the
Spec-Mob's interface-shape step rather than adding to it.

---

## Skill 2: `improve-codebase-architecture`

### What it is

A three-step protocol: explore → present candidates → grilling loop.

**Trigger:** architectural friction identified by the user or by a post-bug handoff from
`diagnosing-bugs` (SKILL.md:141: "hand off to `/improve-codebase-architecture`").

**Step 1 — Explore (SKILL.md:19–35).** Scope by git hot spots (commit history or user direction).
Spawn a sub-agent to walk the codebase. Apply the deletion test. Look for shallow modules, leaking
seams, locality failures. Domain model (`CONTEXT.md`) and ADRs constrain the search.

**Step 2 — Present candidates (SKILL.md:44–62).** Output: one card per candidate, each with files,
problem (one sentence), solution (one sentence), before/after, recommendation strength (`Strong` /
`Worth exploring` / `Speculative`), and a "Top recommendation" section. Strictly forbids proposing
interfaces at this stage. Asks the user: "Which of these would you like to explore?"

**Step 3 — Grilling loop (SKILL.md:64–74).** User picks a candidate. Run `/grilling` (not
in-scope for this report). Side effects: update `CONTEXT.md` for new terms; offer an ADR only when
the rejection reason would help a future explorer.

**Stop conditions:** only one — the user has not picked a candidate yet (explicit between steps
2 and 3). Otherwise continuous until the user stops.

**Artifacts:** none formally defined; HTML report is optional (written to `$TMPDIR`).

**Verification:** none — this skill produces recommendations, not committed code. The grilling loop
is where verification would happen, but it is deferred to `/grilling` (not in this skill).

### What is genuinely good

The git-hot-spots heuristic ("where have things changed recently") is a concrete, cheap, correct
proxy for where deepening pays off. The YAGNI framing — "deepening pays off by making future
changes easier, so weight parts that have recently changed" (SKILL.md:23) — is exactly the right
argument against premature abstraction.

The "ADR conflicts" handling (SKILL.md:58): surface a candidate that contradicts an ADR only when
the friction is "real enough to warrant revisiting the ADR." That is precisely the standard we use
for reopening settled questions.

The candidate card format forces a before/after contrast — not just "this module is shallow" but
"here is what it looks like now and what it would look like after."

### What is generic filler

The entire HTML report section (HTML-REPORT.md, 124 lines) is irrelevant here. We do not use
Claude Code to generate interactive HTML reports for architecture reviews. The CDN instructions,
Tailwind, Mermaid patterns, and visual style guidance all assume a web-facing output that we never
produce.

The `/grilling` skill reference is a dead end: it is not in the four skills under review, not in
our roster, and the brief does not define it.

### Fit in GOSPLAN

The skill maps cleanly onto two points in our cycle:

1. **Stage 2 Refine, L-lane.** The Spec-Mob's `refine_advisor.md` already asks for "constraints
   missing, model consistency." The hot-spots exploration + candidate card format gives the
   substrate-cartographer and domain advisors a concrete output shape for structural findings. The
   advisor writes one candidate card (problem/solution/before-after/strength) for each architectural
   concern they find, rather than a free-form note. This raises the signal quality of Spec-Mob
   output at no token cost.

2. **Stage 6 Dispose / stage 7 Review, post-bug.** When the debugger delivers a diagnosis, the
   recommended handoff is currently open-ended. The protocol here gives it a concrete destination:
   a candidate card filed by the debugger or the owning implementer, reviewed by `substrate-cartographer`
   or the relevant domain advisor before the next Plan's Bet. This becomes the architectural
   backlog mechanism that GOSPLAN currently lacks.

**Where it lands:**
- Candidate card format (files, problem, solution, before/after, strength) → `docs/templates/arch-candidate.md`.
  Referenced in Spec-Mob instructions (stage 2) and in `debugger.md` under "if the cause is
  architectural, file a candidate card."
- Hot-spots exploration → a note in `substrate-cartographer.md`: "check `git log --oneline -60`
  for the seam under study before writing the fact-sheet; if the seam is not in the hot 20 files,
  state that."
- The `/grilling` step is not adopted — we do not have the tool, and the Planner's sign-off at
  stage 2 exit serves the same function.

### Cost

Writing a candidate card costs ~5–10k tokens. Spawning a hot-spots sub-agent costs ~20–40k. On an
L-lane story this replaces ad-hoc exploration that currently costs comparable tokens with no
structured output.

---

## Skill 3: `domain-modeling`

### What it is

An **active glossary discipline**, not a reference lookup. Trigger: any time a term is being
resolved, a new concept is being named, or model consistency is being checked. Five behaviours:
(1) challenge glossary conflicts immediately; (2) sharpen vague terms by proposing canonical names;
(3) stress-test domain relationships with concrete scenarios; (4) cross-reference code to surface
contradictions; (5) update `CONTEXT.md` inline, never batched.

**ADR protocol (SKILL.md:68–74):** three gates — hard to reverse AND surprising without context
AND result of a real trade-off. All three required. The ADR body (`ADR-FORMAT.md`) is a single
paragraph by default; sections are optional.

**Artifacts:** `CONTEXT.md` (glossary, no implementation detail), `docs/adr/*.md`.

**Stop conditions:** none (continuous discipline, not a phase).

**File structure:** a root `CONTEXT.md` or a `CONTEXT-MAP.md` for multi-context repos. Lazily created.

### What is genuinely good

The three-gate ADR threshold is exactly right. Our ADR history is zero files today (GOSPLAN §2);
that is the correct state for a repo where no decision was hard-to-reverse AND surprising without
context AND a real trade-off between explicit alternatives — until GOSPLAN itself, which meets all
three. The threshold prevents the ADR as a ritual.

The "challenge immediately, update inline" rhythm prevents terminology drift — the exact disease
identified in GOSPLAN §2 ("11 contradictions between `development-cycle.md` and the agent files it
describes").

Cross-referencing code against verbal claims ("your code cancels entire Orders, but you just said
partial cancellation is possible") is exactly the substrate-cartographer's method: three sources,
they must agree.

### What is genuinely limited here

The `CONTEXT.md` / CONTEXT-MAP.md file system assumes a TypeScript-style bounded context structure.
We do not have `CONTEXT.md`. We have:
- `docs/reference/specifications/` for ratified mechanism specifications (authority: binding)
- Domain advisor agents whose memory files hold model knowledge
- `docs/reference/architecture/substrate.md` for structural facts

The skill's glossary file does not map onto our authority structure. A CONTEXT.md created lazily
here would have ambiguous authority: is it a specification? a memory? a process document? Without
an explicit authority field it is a third copy of knowledge that already lives in two authoritative
places.

The ADR format is a near-match for our `decision.md` template (GOSPLAN §5.2). The three-gate
threshold is already implied in GOSPLAN's standing rule "re-derive; never inherit."

### Interaction with ratified specs and domain advisors

This is the critical fit question. In matt-skills, `CONTEXT.md` is a mutable glossary updated
inline during sessions. In GOSPLAN, the analogous artifact is a **specification** in
`docs/reference/specifications/`, which is written by an advisor, reviewed in stage 2, and ratified
by the Planner before it binds code. Allowing any agent to update a specification inline during
design violates the spec's authority — it makes the spec a living document without a ratification
event.

The right mapping: the inline-update discipline (challenge immediately, sharpen fuzzy terms)
belongs in the **Spec-Mob** at stage 2, with the advisor running the challenge protocol and the
output going into `refine_advisor.md`. If the result is a new term or a renamed concept, the advisor
drafts a spec amendment, and the Planner ratifies it before the builder touches code. Not inline;
one ratification event.

**Where it lands:**
- The challenge-and-sharpen protocol → `kornai-economist.md`, `logistics-modeller.md`,
  `settlement-modeller.md`, `utilities-modeller.md`: each advisor gains a few lines describing
  this behaviour during Spec-Mob. Not a new file.
- The three-gate ADR threshold (verbatim) → `docs/templates/decision.md`. It is a better
  expression of the same intent than what we have.
- `CONTEXT.md` as a living glossary file: **do not adopt**. The spec system plus agent memory
  already covers this. A third store would drift.
- The concrete-scenarios discipline (stress-test with edge cases) → already described in
  `kornai-economist.md` but not in the other advisors. Add one line to each.

### Cost

The challenge discipline costs nothing — it changes how the advisor speaks during stage 2. The
ADR template replacement is a one-time edit to `docs/templates/decision.md`. No runtime cost.

---

## Skill 4: `diagnosing-bugs`

### What it is

A six-phase protocol: (1) build a feedback loop; (2) reproduce + minimise; (3) hypothesise;
(4) instrument; (5) fix + regression test; (6) cleanup + post-mortem.

**Trigger:** user says "diagnose"/"debug this", or reports something broken/slow.

**Phase 1 is the skill (SKILL.md:18–66).** The phrase "This is the skill. Everything else is
mechanical" is load-bearing. Ten loop-building strategies in priority order: failing test → curl
script → CLI invocation → headless browser → replay a trace → throwaway harness → fuzz loop →
bisection harness → differential loop → HITL bash script (last resort). Completion criterion: a
single runnable command, red-capable, deterministic, fast, agent-runnable. "If you catch yourself
reading code to build a theory before this command exists, stop." (SKILL.md:66)

**Phase 3 (SKILL.md:89–98):** generate 3–5 ranked falsifiable hypotheses, show them to the user
before testing. Each must state: "If X is the cause, then changing Y will make the bug disappear."

**Phase 5 (SKILL.md:117–129):** write the regression test *before* the fix — but only at a *correct
seam*. "If no correct seam exists, that itself is the finding." (SKILL.md:122)

**Phase 6 (SKILL.md:133–141):** after cleanup, "ask: what would have prevented this bug?" If
architecture, hand off to `/improve-codebase-architecture`.

**HITL script (hitl-loop.template.sh):** a structured bash template for bugs that require human
clicks, so the loop is still structured and its captured output feeds back to the agent.

### Comparison against our `debugger.md`

Our debugger is more mature than this skill in every domain-specific dimension:

| Dimension | matt-skills `diagnosing-bugs` | Our `debugger.md` |
|---|---|---|
| Reproduce first | Phase 1 (extensive) | Step 1: "Before any theory" |
| Causal chain | Phase 4 instrumentation | Step 2: "Follow the actual code path with the graph" |
| Mutation confirmation | implied in Phase 5 | Step 4: explicit — "flipping the suspected cause flips the symptom" |
| 3-strike rule | not present | explicit — "never rerun the same failing probe unchanged" |
| Sibling sweep | not present | Step 6: "grep every other caller of the broken seam" |
| Hypothesis ranking | Phase 3: 3–5 ranked | implicit in chain tracing |
| Report template | not present | explicit: SYMPTOM / ROOT CAUSE / CHAIN / CONFIRMED / REPRO / SIBLINGS / SUGGESTED |
| "Diagnosis never the fix" | Phase 5 fixes the bug | explicit: "You find why, not fix" |
| Four house defect shapes | not present | explicit: silent defaults, panics on live paths, unverified checks, false search zeros |

The critical difference: matt-skills `diagnosing-bugs` *includes the fix* (Phase 5). Our
`debugger.md` explicitly refuses it — the fix goes to the owning implementer lane. This is not a
design preference; it is how we prevent a writer-collision and keep the diagnosis auditable.

### What matt-skills does better

**The "correct seam" concept (SKILL.md:117–122).** "If the only available seam is too shallow
(unit test that can't replicate the chain that triggered the bug), a regression test there gives
false confidence. If no correct seam exists, that itself is the finding." This formulation is more
precise than anything in our debugger. Our debugger says "leave a minimal failing repro" but does
not give the agent a principled way to recognize when a seam is wrong vs merely hard.

**The ten loop-building strategies in priority order.** Our debugger's "reproduce first" is
correct but thin. The ordered list — failing test first, HITL last — gives a new agent a concrete
decision tree for how hard to work before declaring the loop non-constructible.

**Phase 3 hypothesis format.** "If X is the cause, then changing Y will make the bug disappear /
changing Z will make it worse." This falsifiability framing is stronger than our implicit practice.
We do this but do not require stating the prediction before probing.

**Phase 6 post-mortem handoff.** The explicit instruction to ask "what would have prevented this
bug?" and route architecture problems forward prevents findings from dying in closed issues. Our
debugger's report template has no equivalent step.

### What is generic filler

Phases 2–4 in matt-skills are standard debugging advice that our debugger already exceeds. The
tool-preference list in Phase 4 ("debugger/REPL inspection first") assumes a JS/Python environment
where a REPL is available; in Rust cargo build is the instrument.

The HITL bash script is a good artefact for generic use but we have not had a bug that required
it. Keep it as reference only.

### Fit in GOSPLAN

**Where it lands — four concrete steals:**

1. **"Correct seam" framing → `debugger.md` step 5 (REPRO).** Replace "leave a minimal failing
   repro (or why none is possible)" with: "Leave a minimal failing repro at the *correct seam* —
   the seam where the real bug pattern occurs as it does at the call site. If the only available
   seam is too shallow, that is the finding: name which seam would be correct and why it does not
   exist. A shallow seam repro gives false confidence."

2. **Loop-building priority order → `debugger.md` step 1 (REPRODUCE).** Add a parenthetical
   decision tree: "failing test at any seam → CLI invocation with fixture → throwaway harness →
   fuzz/property loop → bisection → HITL script. Never skip to theory before the loop is red."
   This does not change the protocol; it gives the agent a concrete decision path when the obvious
   repro does not exist.

3. **Falsifiable hypothesis format → `debugger.md` step 3 (implied between CHAIN and CONFIRMED).**
   Add: "Before probing, enumerate 3–5 ranked hypotheses. Each must state: 'If X, then changing Y
   flips the symptom.' A hypothesis you cannot state as a falsifiable prediction is a vibe — sharpen
   or discard it." This upgrades the implicit practice to a required step.

4. **Post-mortem handoff → `debugger.md` after SIBLINGS.** Add after SUGGESTED: "ARCHITECTURE: if
   the root cause is architectural (no correct seam exists, hidden coupling, tangled callers), file
   an `arch-candidate.md` from the template and cite it in the bd comment. Do not apply the change;
   the candidate enters the next Plan's Bet."

### Cost

These four changes are four paragraphs in `debugger.md`. No runtime cost.

---

## Ranked steal list

| Rank | Mechanism | Source file:line | GOSPLAN destination | Why |
|---|---|---|---|---|
| 1 | Correct-seam framing for regression tests | `diagnosing-bugs/SKILL.md:117–122` | `debugger.md` step 5 | Plugs the "false confidence seam" gap; directly applicable to our Rust test harness |
| 2 | Falsifiable hypothesis format ("If X then Y") | `diagnosing-bugs/SKILL.md:89–98` | `debugger.md` between CHAIN and CONFIRMED | Strengthens our implicit practice; cheap to add |
| 3 | Loop-building priority order (10 strategies) | `diagnosing-bugs/SKILL.md:27–35` | `debugger.md` step 1 | Gives a concrete decision tree when the obvious repro fails |
| 4 | Post-mortem architecture handoff | `diagnosing-bugs/SKILL.md:140–141` | `debugger.md` after SIBLINGS as ARCHITECTURE field | Closes the finding-to-backlog gap |
| 5 | Three-gate ADR threshold (verbatim) | `domain-modeling/ADR-FORMAT.md:30–37` | `docs/templates/decision.md` | More precise than our current implied standard; one-time edit |
| 6 | Candidate card format (problem/solution/before-after/strength) | `improve-codebase-architecture/SKILL.md:44–54` | `docs/templates/arch-candidate.md` (new template) | Gives the debugger and Spec-Mob a structured output for architectural findings |
| 7 | Git hot-spots scope heuristic | `improve-codebase-architecture/SKILL.md:19–26` | `substrate-cartographer.md` (one sentence note) | Cheap signal for where to focus; cartographer already does this informally |
| 8 | Two-adapter rule | `codebase-design/SKILL.md:65` | `.claude/rules/house-rules.md` architecture vocabulary | Codifies a real discipline; prevents single-adapter indirection |
| 9 | Dependency category taxonomy (in-process / local-substitutable / remote / external) | `codebase-design/DEEPENING.md:9–28` | `docs/templates/brief.md` under Traps | Brief should state the dependency category at each seam the builder will cross |
| 10 | Concrete-scenarios discipline for advisors | `domain-modeling/SKILL.md:54–58` | `logistics-modeller.md`, `settlement-modeller.md`, `utilities-modeller.md` (one line each) | Already in `kornai-economist.md`; propagate to the three others |
| 11 | Challenge-and-sharpen protocol during Spec-Mob | `domain-modeling/SKILL.md:44–53` | Domain advisor bodies; Spec-Mob output format | Makes advisor `refine_advisor.md` sharper; no new agent |
| 12 | Design-it-twice parallel sub-agent pattern | `codebase-design/DESIGN-IT-TWICE.md:16–45` | `planner` play option for L-lane interface design during Spec-Mob | Replaces ad-hoc interface exploration with structured parallel comparison |

---

## Do-not-steal list

| Mechanism | Reason |
|---|---|
| `CONTEXT.md` living glossary file | We have two authoritative stores (specs + advisor memory). A third creates a drift surface. The `domain-modeling` skill's inline-update discipline is valuable; the file is not. |
| `diagnosing-bugs` Phase 5 (fix + regression test) | Contradicts the core rule of our `debugger`: diagnosis never the fix. The fix belongs to the implementer lane. Stealing Phase 5 would collapse the write-boundary and make the diagnosis non-auditable. |
| `improve-codebase-architecture` grilling loop | References `/grilling`, which is not defined in our roster and not in this skill. The Planner sign-off at stage 2 exit and the ADR mechanism serve the same function. |
| HTML report generation (HTML-REPORT.md) | Wrong output format for our process. Candidate cards are markdown files in `.planning/plans/plan-NN-<slug>/`. No browser, no CDN. |
| `codebase-design` TypeScript testability examples (SKILL.md:70–95) | Wrong language and idiom. Rust testing through trait objects and direct ECS component inspection is covered by our existing house rules. |
| Design-it-twice as a routine Spec-Mob step | Only warranted when a competing interface shape has been surfaced by the advisor; not every L-lane story has this. Make it an option, not a required play. |
| HITL bash script as a required artefact | We have not had a bug requiring human-in-the-loop reproduction. Keep the template as reference material under `docs/reference/`; do not install it as a protocol step. |

---

## Stage design changes GOSPLAN should make

### One new template: `arch-candidate.md`

Filed by debugger or implementer when a bug's root cause is architectural, or by an advisor during
Spec-Mob when a structural concern is found. Enters the next Plan's Bet queue.

Fields:
```
Candidate: <short name>
Files: <list>
Problem: <one sentence — what causes friction>
Solution: <one sentence — what changes>
Before: <current structure or module relationship>
After: <structure after deepening>
Strength: Strong | Worth exploring | Speculative
Filed-by: <agent>
Filed-at: <bd issue id>
```

No grilling loop, no Mermaid. Reviewed at stage 0 Bet by gosplan + Planner.

### One new paragraph in `debugger.md`

Add the correct-seam framing, the hypothesis format, the loop priority order, and the
ARCHITECTURE handoff field to the existing report template. See steals 1–4 above. Total
addition: ~12 lines. No structural change to the agent's role.

### Candidate cards in Spec-Mob output (L-lane)

`refine_advisor.md` already has the implicit structure. Make it explicit: "If you find a
structural concern (shallow module, leaking seam, locality failure), write one candidate card
in the `arch-candidate.md` format. Do not expand this into a redesign proposal — that is the
grilling loop, which happens in a subsequent Plan."

### No new stage

The four skills do not justify a new "design" stage between Decompose and Build. The design work
they describe maps onto: stage 1 (Decompose, where interface shape is proposed), stage 2 (Refine,
where Spec-Mob challenges it), and the architectural backlog (arch-candidate.md, reviewed at Bet).
Adding a stage between them would lengthen the cycle for every story, including S-lane patches where
no design work is needed.

---

## Gaps

1. The `/grilling` skill referenced in `improve-codebase-architecture/SKILL.md:65` is not in
   this repo or our roster. Its role (walking a decision tree with the user) is played by the
   Planner's stage 2 sign-off on L briefs. No gap in our process, but the reference in the skill
   is a dead end.

2. The design-it-twice pattern (`DESIGN-IT-TWICE.md`) is verified as a described protocol but
   not as a measured outcome. The claim (~1.5× improvement from parallel interface design) is
   unverified for our codebase. It should be treated as a candidate play for Plan 02, not a default.

3. The domain-modeling challenge discipline (steal 11) requires all four advisors to be consistent
   in how they run stage 2. Currently `kornai-economist.md` is the most detailed; the other three
   are thinner. Propagating the protocol requires editing three agent files. Logged, not done here.
