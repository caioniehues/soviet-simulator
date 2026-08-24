# Framework design — what to take from four agent frameworks, and what to build instead

Study run 2026-08-24 against fresh upstream clones. Tracked as `sov-framework-design-study-823`.
Two parts: **Part 1** is the comparison and the reasoning, for a human reader. **Part 2** is the
operational spec, for agents to execute. Read Part 1 to judge the calls; Part 2 to run the process.

**The headline, up front.** The prior session ranked `iterative-development` first on one property —
cross-iteration regression coverage. That property is real and that framework does describe it best.
But the mechanism was *already adopted in this repo*, and it produced a corpus of 153 scenarios with
**zero runnable commands**, six named sentinels with **zero implementing tests**, and a derivation
rule that leaves **33% of the corpus unreachable**. The ranking identified the right property and
mistook a well-written prose design for a working mechanism.

The finding that reorganises the whole study is not about any framework. It is that **this repo
currently ships twelve documents that assert something their own source disproves.** Eleven are
arithmetic. The twelfth is not, it was missed by this study's own method and caught by its review
gate, and it is worth more than the other eleven together: `RESUME.md:117` — the file every agent
reads first — tells them a bincode round-trip is *"a real determinism check."* It is not.

Of the four frameworks, only `compound-engineering` has any mechanism for checking a produced
document against code, and it has two — both single-subagent, both explicitly not hard gates.

**And the design that follows is smaller than the one this study first wrote.** The first draft's
centrepiece was a validator that policed those documents. An adversarial gate established that none
of the eleven arithmetic findings would have cost an hour of real work, that a validator over
hand-editable markdown decays into `|| true`, and that no amount of counting reaches the twelfth. The
revision cuts nine mechanisms and replaces the principle: **delete the surface that can hold a false
claim rather than policing it.**

---

# PART 1 — THE STUDY

## 1.1 What was read, and how

Four repos cloned fresh from upstream into `/tmp/fw-study/` on 2026-08-24. Local plugin caches were
not read; every claim below traces to a file at a pinned SHA.

| Framework | Path | SHA | HEAD date | SKILL.md files |
|---|---|---|---|---|
| superpowers | `/tmp/fw-study/superpowers` | `b36e0829c6d0140e93cfef2ca599b1b07d4a7797` | 2026-08-12 | 14 |
| iterative-development | `/tmp/fw-study/iterative-development` | `c05889aeb28f1f2c93f88232236e6ed906d32a6f` | 2026-06-06 | 6 |
| mattpocock/skills | `/tmp/fw-study/skills` | `5b15a47f2d7150f545fbcacbfe381787fc0230dc` | 2026-08-21 | 36 |
| compound-engineering | `/tmp/fw-study/compound-engineering-plugin` | `a32c9474c658f3e33b6e3615a5d51089046d4c79` | 2026-08-23 | 34 |

**Method.** Six readers — one per framework, plus a second independent adversarial reader on
`superpowers` and `iterative-development` whose only job was to adjudicate the prior session's
claims. Three verifiers re-derived this repo's own numbers. Rules given to every reader: a skill's
`description:` frontmatter states intent, the body states mechanism, **the body wins**; no claim may
cite a README; quote verbatim where the quote carries the argument. Reader reports are in
`/tmp/fw-study/reports/`.

Where a result below is marked ★ it was verified by the lead directly, not taken from a worker.

## 1.2 The four frameworks, on the four questions that matter

### How do you know you are done?

| | Definition of done | Where it lives | Enforced by |
|---|---|---|---|
| **superpowers** | Fresh command output plus an independent reviewer verdict, per task and again per branch | `verification-before-completion/SKILL.md:12-14`; `finishing-a-development-branch/SKILL.md:15-19` | prose |
| **iterative-development** | *"passing behavior evidence at the correct seam for every externally observable requirement — not just that stories are marked done"* (`iterative-development/SKILL.md:10`) | the corpus table | prose |
| **mattpocock** | Per-skill; only `diagnosing-bugs` has a real checklist. `code-review` explicitly is **not** a gate — a report with no APPROVE/BLOCK field | `diagnosing-bugs/SKILL.md` Phase 1/6 | prose |
| **compound-engineering** | Per-skill completion contracts; `ce-work` blocks shipping until a review receipt or an authorized skip state is recorded | `ce-work/SKILL.md` Phase 3-4 | prose |

`iterative-development`'s is the best sentence anyone wrote. It is the one to keep.

### What if the spec is wrong?

Three distinct things get conflated. Separating them is the crux of the study:

- **(a) spec-vs-HUMAN** — a person approves the plan before building.
- **(b) spec-vs-CODE** — something checks that a document's assertions about the codebase are *true*.
- **(c) impl-vs-spec** — compliance checking.

| | (a) human gate | (b) grounding | (c) compliance |
|---|---|---|---|
| **superpowers** | **Yes, twice** — `brainstorming` `<HARD-GATE>`, plus a spec-review gate | **None** | extensive |
| **iterative-development** | **No — actively suppressed** (below) | **None** | the central machine |
| **mattpocock** | soft, conversational (`grilling`) | none standing; `triage` verifies one issue's claim once | `code-review`'s two axes |
| **compound-engineering** | yes, default; stripped in pipeline mode | **YES — the only one** | `ce-code-review` Stage 2 |

Everybody does (c). Almost nobody does (b). That gap is what has cost this project most.

`iterative-development` does not merely lack the human gate — it overrides ours by name.
`iterative-development/SKILL.md:140`:

> When running autonomously, this orchestrator **takes precedence over interactive-gate skills**
> (e.g., `brainstorming` which requires design approval before implementation). … **Do not block on
> skills that assume a human is present to approve each step.**

`check_for_human_interrupt()` sits at `SKILL.md:45` — the first statement *inside* `while True:`,
i.e. after extraction and scoping already ran. It has **no function definition anywhere in the
repo**. Escalation is catastrophe-only and explicitly excludes reviewer findings, audit gaps, a
BLOCKED implementer, and spec ambiguity.

### The regression story

| | Mechanism | Verdict |
|---|---|---|
| **superpowers** | The host project's own suite, run at branch boundaries and pre-commit | Real but wholly inherited. No corpus, no sentinel, no cross-plan concept. Reviewers are **forbidden** to re-run the suite (`task-reviewer-prompt.md:74-80`) |
| **iterative-development** | Sentinel corpus, baseline-before / compare-after, three-tier audit | The best *design*. Unenforced — see below |
| **mattpocock** | One regression test per diagnosed bug | None cross-feature |
| **compound-engineering** | `ce-dogfood` explicitly *"diff-scoped, not whole-app"* | None cross-feature |

The prior session's ranking rests entirely on row 2. Two things break it.

**It is prose, and that was proven by mutation.** `id-adversary` fed
`validate_iteration_log.py` an iteration log whose Scenarios field reads *"SENTINEL CORPUS: 7 of 10
FAILING. JOURNEY-0001 REGRESSED … Regressions knowingly shipped."* Result: `OK: … rc=0`. The wrap-up
gate passes an iteration that confesses to shipping seven regressions. Across all five validators,
`grep -rn "subprocess|os.system|popen" --include=*.py` returns **nothing** — no validator executes
anything. The framework concedes it at `scoping-the-simplest-core/SKILL.md:112`: *"The validator
checks format only. The PAR scope review is the real structural gate."*

**Its own test suite is red at HEAD, and has been for 4½ months.**
`python3 -m unittest discover -s tests -q` → `Ran 37 tests … FAILED (failures=2)`. Both are
`test_valid_example_passes` — the repo's canonical *"valid"* fixtures fail its own validators.
Traced to `6413000` (2026-04-11), which added required fields to two validators and touched only the
`.py` files. Four later commits shipped over the red suite, including `516189e chore: prep for public
release`. The project whose distinguishing claim is cross-iteration regression detection did not
detect its own cross-commit regression, in a suite that runs in 0.64 seconds.

**Sentinel promotion is not curation.** `extracting-requirements/SKILL.md:158-162` — journey
scenarios become sentinels automatically, surface scenarios do not, *"refined during scoping"*, and
that refinement is implemented nowhere. Membership is decided by **which spec folder a behavior was
written up in**. For a greenfield project with a `journeys/`-structured spec that is free and
sensible. For a fork of Egregoria it means no sentinel covers the union-find electricity substrate
or the `market.rs` teleport paths — the things that actually break here.

### Scale and context assumptions

| | Assumes |
|---|---|
| **superpowers** | git hardcoded throughout; stack detection is a 4-marker enum (`Cargo.toml` included); one leaked `gh api` call at `receiving-code-review/SKILL.md:145-147`; no issue-tracker integration at all; single operator |
| **iterative-development** | **Greenfield.** The walking skeleton's first task builds the E2E harness "before implementing any product features"; no step anywhere inventories an existing codebase or marks a story done because the capability already exists. Spec must be organised into `journeys/`/`contracts/`/`domains/`/`test-vectors/` |
| **mattpocock** | GitHub/GitLab first-class; `.out-of-scope/mainstream-issue-trackers-only.md` explicitly excludes a CLI tracker like `br`; TS/npm in examples; PR-as-request-surface in `triage` |
| **compound-engineering** | GitHub PRs load-bearing in the shipping tail; a browser (`ce-test-browser`, `ce-polish`); Xcode; users and telemetry (`ce-product-pulse` → PostHog/Mixpanel/Sentry/Stripe); a team leaving PR comments |

Roughly a third of `compound-engineering`'s `lfg` pipeline is inapplicable here. `iterative-development`'s
inapplicability is more serious and less obvious: **its bootstrap is where its distinctive value is
created, and this project's bootstrap is finished.**

### Enforcement — the axis nobody advertises ★

This is the finding that decides the design, and it cuts against all four equally.

- **superpowers** ships exactly one hook. `hooks/hooks.json:3-14` is `SessionStart`, and
  `hooks/session-start` cats `using-superpowers/SKILL.md` into context. `grep -rn
  "PreToolUse\|PostToolUse\|UserPromptSubmit\|\"Stop\""` over the whole tree returns **no output**.
  The `<HARD-GATE>` tag is an XML-looking string in a markdown file, and `using-superpowers/SKILL.md:60-62`
  explicitly subordinates it: *"User instructions … take precedence over skills."*
- **iterative-development**'s validators check markdown shape (proven above).
- **mattpocock**'s only blocking code in the entire corpus is `diagnosing-bugs/scripts/hitl-loop.template.sh`,
  two bash helpers wrapping `read -r -p`.
- **compound-engineering** has real validator scripts, but a skill is *asked* to run them.

**Every gate in all four frameworks is a sentence addressed to a model.**

Meanwhile `~/.claude/settings.json` on this machine wires 14 hooks, five of them blocking
`PreToolUse` guards. One of them (`lsp-first-read-guard.js`) blocked a Read attempted during this
very study and forced an LSP call first. **This project can build gates that the frameworks it is
copying cannot.** And `soviet-simulator/.claude/settings.local.json` currently contains only
`permissions` — zero project-scoped hooks. The substrate is present and entirely unused.

## 1.3 What each does better than the others — named mechanism, not vibe

**compound-engineering — grounding validation at write time.** `ce-compound/references/grounding-validation.md`.
Before a learning doc is finalised, a mechanical script (`validate-doc-claims.py`) catches dead paths
and SHAs, then a dispatched **read-only** subagent must *"locate the defining source in the current
tree and quote the defining line(s) with file:line. Verdict: verified (with quote), contradicted
(with the quote showing otherwise), or unverifiable."* On contradiction: *"fix the doc using the
quoted evidence (the quote, not the conversation, is authoritative)."* Paired with
`ce-compound-refresh`, which re-checks the whole store later. Its rule, at
`ce-compound-refresh/references/classify.md:15`: *"Match docs to reality, not the reverse. When code
and doc disagree, the doc is what changes."* — the clause *"and the code does not"* appears
separately at `ce-compound-refresh/SKILL.md:54`. (An earlier draft welded the two files into a single
quotation. The substance held; the verbatim did not, and in a document arguing that quotes are
authoritative and paraphrase is not, that is the failure it prosecutes.)

**This is the single most valuable mechanism found in the study** — the only direct answer to the
question the brief names as most valuable. Three qualifications, all of which matter for adoption:

- **It is explicitly not a hard gate.** `grounding-validation.md` opens: *"Neither pass is a hard
  gate — every flag is adjudicated, because solution docs legitimately cite deleted paths and
  pre-fix states."* Reasonable for a learnings store; also why it does not, alone, stop a false claim
  shipping.
- **It is one subagent, unreplicated** — unlike `ce-code-review`, which requires independence before
  counting agreement as corroboration.
- **There is a second one, and it is the closer analogue.**
  `ce-doc-review/references/personas/feasibility-reviewer.md` — the file this study's first draft
  admitted it had not read — grounds a *plan* rather than a learning: *"What already exists? … Does
  it assume greenfield when reality is brownfield?"* and states plainly, **"This check requires
  reading the codebase alongside the plan."** Its confidence rubric anchors on code evidence
  (*"Specific technical constraint blocks the approach and you can cite it concretely"*) and it
  carries a false-positive suppression rule against theoretical concerns with no baseline data.

So the accurate statement is: **compound-engineering is the only one of the four that grounds
documents against code, and it does so twice** — at learning-doc write time and at plan-review time.
Plan-review grounding is exactly what this project's failure #1 needed and did not have.

**compound-engineering — a shared, indexed learning pool.** `docs/solutions/<category>/<file>.md`,
one repo-tracked store, schema-validated, written only by the orchestrator (*"Only the orchestrator
writes product files"*), discoverable because `ce-compound` Phase 2.5 verifies the root instruction
file names the store. Not auto-loaded — grepped on demand behind a one-line pointer.

**iterative-development — the definition of done, and the cadence column.** The DoD sentence at
`SKILL.md:10` is the best in the study. `behavior-evidence-formats.md:125-129` gives a
`task`/`iteration`/`sentinel` cadence column — an explicit notion of *"this small set runs every
iteration regardless of what changed"*, which nothing else has. And
`running-an-iteration/SKILL.md:33-38` + `:88-90` give the **baseline-before / compare-after**
protocol, which is what makes attribution possible at all and costs one command.

**iterative-development — the proof-seam ladder with REJECT rules.**
`behavior-evidence-formats.md:15-23` (unit → integration → app-level → process-level → e2e) plus
`spec-compliance-reviewer-prompt.md:44-51`: *"REJECT: unit-only evidence for app-level or e2e
behavior"*, *"REJECT: inspection-only evidence without strong justification"*, *"REJECT: one-time
manual verification that did not update the behavior corpus."* Cheap to port, and it names the exact
failure this repo already committed (proving a UI story with a sim-level assertion).

**superpowers — the file-handoff triad and the scoped fix loop.**
`scripts/{sdd-workspace,task-brief,review-package}` hand subagents *file paths*, never pasted text,
with a quantified rationale (a real 42k-char dispatch that was 99% pasted history). The fix loop is
capped at five rounds, rounds 4-5 escalate to a fresh implementer one model tier up, and at the cap
the controller must adjudicate every open finding into a logged ruling — *"a silent discard is
forbidden."* Also the verbatim **no-subagents contract** baked into every dispatched prompt, which
stops duplicate review seats.

**superpowers — the approval gate as a concept.** `brainstorming/SKILL.md:13-19`, and the
three-path router added in this very release: classification is *"bounded means the flow you are
changing is already here to read. If there is no existing flow to change, the task is not bounded"*
— ceremony scales, the gate never does. Right idea, no enforcement.

**mattpocock — the two-axis review with a no-merge rule.** `code-review/SKILL.md` dispatches two
sub-agents with disjoint prompts (Standards, incl. a 12-item Fowler smell baseline pasted inline at
`:38`; and Spec, asking for missing/partial, scope creep, and *"requirements that look implemented
but where the implementation looks wrong"*), then reports them **side by side** — `code-review/SKILL.md:76,78`:
*"Do not merge or rerank findings… Don't pick a single winner across axes."*
The reasoning behind the rule is **not in the skill body** — it is in the prose companion at
`docs/engineering/code-review.md:72`: *"There is no convergence guarantee… do not run it in a loop
until it comes back clean, because it will not."* Flagging that explicitly because §1.1's own method
says the body wins and no claim may cite the prose layer: **the mechanism is in the body, the
justification is not.** An earlier draft of this document attributed that quote to the SKILL.md and
was wrong to.

**mattpocock — the glossary discipline.** `CONTEXT-FORMAT.md`: a glossary *"totally devoid of
implementation details"*. `ADR-FORMAT.md` gates an ADR on **three simultaneous conditions** — hard to
reverse, surprising without context, the result of a real trade-off — *"If any of the three is
missing, skip the ADR."* This repo's `CONTEXT.md` already follows it and is, as the brief notes, the
document that has aged best. It is also the framework most honest about its own weakness:
*"an unreviewed, agent-authored glossary is worse than none: it becomes confident-sounding lore that
later sessions treat as truth."*

## 1.4 Corrections to the prior findings

The brief said to test these rather than inherit them. Six of nine needed correction.

| Prior claim | Verdict | Evidence |
|---|---|---|
| "Only iterative-development has a regression answer" | **Right property, wrong conclusion** | Its answer is unenforced prose; mutation-proven `rc=0` on a log confessing 7 regressions; its own suite red 4½ months |
| "Its adversarial review is single-axis; none asked *is this true?*" | **REFUTED for the framework** | 4 of 5 reviewer roles instruct truth-checking. `spec-compliance-reviewer-prompt.md:21`: `## CRITICAL: Do Not Trust the Report` … *"Verify everything independently by reading the actual code"* |
| "…sixteen PAR reports all asked *what is missing?*" | **CONFIRMED for this repo** ★ | Every finding's `classification:` is `omission`/`missing-scenario`/`thin-AC`/`intentional-exclusion`. No truth-check category exists structurally. **Both claims are true of different objects** — the framework has truth axes; our 16 reports came from the one omission-scoped gate, `extracting-requirements`' omission reviewer. The brief conflated them |
| "The format has a tag named `UNAUDITED` and no step fills it" | **REFUTED — and the truth is worse** ★ | `grep -rni unaudited` over the whole framework tree → **0 hits**. It appears **17 times in our requirements**. We invented the audit slot ourselves and never built the step |
| "370 ACs, 265 ABSENT / 33 CONFLICTS / 26 PROVIDED / 19 PARTIAL / 17 UNAUDITED / 10 OURS" | **CONFIRMED exactly** | Independently recounted; sums to 370; no other tag values exist |
| "103 of them cite an exact file:line" | **Number probably right, framing wrong** | 215 ACs cite *some* `path:line`. Partitioned: **102 code**, 108 spec/doc markdown, 1 Lua, 1 `.ini`, 3 decompiled C#. The prior 103 was almost certainly the code count; the brief's framing implied all such citations were grounding. **Only 28% of ACs make a claim about code anyone can check** |
| "It is autonomous with no approval gate" | **CONFIRMED, and stronger** | `SKILL.md:140` suppresses interactive gates *by name* |
| "superpowers' brainstorming has a HARD GATE" | **PARTIALLY TRUE** | The tag is real and heavily reinforced; enforced by **nothing**; and single-shot — `subagent-driven-development/SKILL.md:16` *"Do not pause to check in with your human partner between tasks"*, four stop conditions, none of which is "the work looks wrong" |
| "All four generate documents; none verifies them" | **REFUTED for compound-engineering** | `ce-compound` Phase 2.45 grounding validation + `ce-compound-refresh` do exactly this. True for the other three — and superpowers' two document-reviewer prompt templates are **orphans at this SHA**, referenced only by release notes and the plan that created them |

Two further defects found in upstream source that landed here:

**`backlink_scenarios.py:56-63` appends `scenario_ids[0]` to every unlinked AC of a story owning
multiple scenarios.** Found by reading upstream; confirmed in our corpus **33 for 33, zero
exceptions** ★. STORY-0003 owns SCENARIO-0003/0004/0012 — all 6 of its ACs tag only SCENARIO-0003.
Consequence, since `roadmap.md:3` says impacted sets are *"DERIVED from its committed stories' AC
`scenario:` tags"*: **50 of 153 scenarios (33%) are unreachable from any AC tag** and can never enter
an impacted set, though every one is owned by a story.

## 1.5 ★ The finding that reorganises the study

Eleven documents in this repo assert something their own source disproves. Every one is arithmetic.

| # | Artifact | Asserts | Truth | Checkable by |
|---|---|---|---|---|
| 1 | `roadmap.md:3` | "plus 1 deferred" | 19 | its own generator, `build_roadmap.py:265` |
| 2 | 14 of 16 `par/*.md` | `TOTAL FINDINGS: N` | N+1 … N+12 | `grep -c '^## F'` |
| 3 | `behavior-corpus.md` | 151 scenarios, proof seams assigned | **0** runnable commands (153 `TBD`) | column scan |
| 4 | `scenarios/mod.rs:1-14` | six sentinels re-run every iteration | **0** such tests exist | fn-name grep |
| 5 | `roadmap.md:3` | impacted set derived from AC tags | 33% of corpus unreachable | set difference |
| 6 | `dev-cycle.md:207` | Phase 6 validates `iteration-log.md` | file does not exist | `test -f` |
| 7 | `RESUME.md` | "Agent T2b is doing that now" | landed at `b3857f5`, 80 min *before* RESUME.md's own last edit | `git log` |
| 8 | `RESUME.md` | `sov-scope-cut-1p6` open | closed 2026-08-22 | `br show` |
| 9 | `RESUME.md` | 53 contract scenarios | 54 | row count |
| 10 | `dev-cycle.md` Phase 0 | *"Fact-sheets persist in the cartographer's memory. The second brief on the same seam is nearly free."* | that directory was **empty** for the entire period the claim was asserted — see note | `ls` |
| 11 | `.claude/skills/` | — | two Bevy skills still installed for an engine discarded 2026-08-22, loaded into every session's routing surface | path check |
| **12** | **`RESUME.md:117`** (and `roadmap.md:28`) | *"serialize → deserialize → per-key hash compare every tick — **a real determinism check**"* | `tests/mod.rs:107-121` compares a decoded copy against the live state. That proves `encode ∘ decode` is the identity — a **save/load round-trip guard**. It cannot detect two runs diverging, and is vacuous for state `hashes()` omits | reading the function |

**#12 is the only one on this list that is not arithmetic, and it is worth more than the other
eleven combined.** It was missed by this study's own method and found by its review gate. It sits in
the file `CLAUDE.md` makes every agent read first, and it functions as a *permission*: an agent told
a determinism guard exists stops looking for one. The brief names determinism as load-bearing. See
§2.4.0.

`build_roadmap.py:192` is the whole thesis in one line:

```python
A(f'{total} scheduled stories across 14 iterations, plus 1 deferred. Generated by `build_roadmap.py` — '
```

`total` is interpolated. **"plus 1 deferred" is a hardcoded literal.** Line 265 of the same file
computes the truth and prints it to a console nobody reads: `scheduled 130 + 19 deferred = 149`.
The correct number was in a variable, two lines away, and nothing compared them.

`RESUME.md` records *"All 172 PAR findings remediated."* The real count of `## F` items is **203**.

**A note on #10, because this document is not exempt.** When I checked at 00:59 today,
`.claude/agent-memory/substrate-cartographer/` was empty and the agent's own prompt instructed it to
"Read `MEMORY.md` first" — a file that did not exist. At 01:08, mid-study, this wave's own
cartographer wrote two files there. The finding was true for every day `dev-cycle.md` asserted it and
is no longer true as of this session. I am leaving it in the table, corrected, rather than deleting
it: a document about stale claims that quietly drops its own stale claim has learned nothing. The
underlying defect — that nothing checked — is unchanged.

**The lesson is not "add a document auditor."** This repo already has `doc-reality-auditor` in its
roster, with a good prompt, and shipped all eleven anyway. The lesson is that a per-iteration LLM
sweep is the wrong instrument for a defect class that is arithmetic. You do not ask a language model
to count; you count.

## 1.6 Why this changes the ranking

The prior ranking was: `iterative-development` first, on regression coverage.

The corrected ranking is not a ranking of frameworks at all, because **the thing this project needs
most is possessed by none of them in usable form**:

- The best *stated* regression design (`iterative-development`) is unenforced prose whose own
  maintainer shipped over a red suite for four months, and whose bootstrap does not apply here.
- The best *document-grounding* mechanism (`compound-engineering`) is real, is the answer to the
  brief's most-valuable question, and is single-threaded LLM judgment where most of the work is
  arithmetic.
- The best *human gate* (`superpowers`) is enforced by nothing, and is explicitly overridden by name
  by the framework the prior session ranked first.
- The best *review shape* (`mattpocock`'s disjoint two-axis with no merge) is a report, not a gate,
  and sits inside a toolbox whose ticket flow this project already rejected.

And `docs/dev-cycle.md` — invented here — is better than all four on the axis they share: it names
the failure each phase prevents, orders its gates cheap-to-expensive with an argued reason, and
tiers its agents on measured evidence. Its Phase 0 (GROUND) has no equivalent in any of the four,
and it is the direct answer to the (b) spec-vs-CODE gap that all four share.

**So the design is not "adopt a framework." It is: keep dev-cycle.md, take five specific mechanisms,
and build the one thing none of them has — an executable check that a document is true.**

## 1.7 The empirical failure inventory, re-derived

The brief lists six failures the framework "must survive, since they all really happened." A
forensic pass (`failure-verifier`, read-only, `HEAD = fdfabca`) checked each against code and git
history. **Four survive; two are materially wrong; and one of the six is still live in the repo.**

| # | Claim | Then | Now |
|---|---|---|---|
| 1 | Commented-out truck registration; agent built the forbidden mechanism | **CONFIRMED** | fixed `35ce342`; the forbidden *shape* still exists at `market.rs:137-148`, with no recorded verdict |
| 2 | "Copy the `freight_station.rs` train pattern" — premise false | **CONFIRMED** | **STILL LIVE** — `RESUME.md:84` |
| 3 | `optout_exttrade` on 1 of 21 falsified **three** claims | **count exact, "three" unsupported** | count still exact |
| 4 | `cargo test -p simulation sentinel` ran 0 tests, exited 0 | **CONFIRMED** | **still true — reproduced** |
| 5a | `CLAUDE.md` pointed at a nonexistent `bevy.md` | CONFIRMED on substrate; **text unverifiable** | fixed `5cf7953` |
| 5b | Four agent defs targeted paths deleted **five days before** they were written | **REFUTED — time reversed** | n/a |
| 6 | Buyers credited free every tick with no freight station | **CONFIRMED** | fixed `fdfabca` |

Four of these change the design, so they are worth stating precisely.

**#5b is backwards, and the correction changes which mechanism you build.** Agent files written
2026-08-17 (`1cf5d97`); `src/sim/` and `src/game/` deleted 2026-08-22 (`68fe28c`, the fork). Five
days in the *opposite* direction. The agents were **correct when authored** and were invalidated by a
later event. Also "four" is three — `prototype-researcher.md` names neither path.
*"Agents authored against dead paths"* implies a fix at authoring time. *"Correct agents decayed when
the tree moved"* implies a fix at **tree-change** time. The brief points at the wrong one, and this
design would have built the wrong mechanism on it.

**#6's real lesson is disposition, not detection.** The free-credit bug was described correctly, in
the repo, **two days before** the specialist "found" it — `roadmap.md:23` names the same line range,
the same mechanism, and the same consequence, and `behavior-scenarios.md:16` restates it. It was
filed as a *test-fencing precondition* rather than a bug, so nobody acted. **Another detector would
not have helped.** What was missing was a step that gives every recorded anomaly an explicit verdict.

**#1 is unfair as stated.** The agent did build the forbidden mechanism — and disclosed it, at
length, in `6ea4553`'s own commit message: *"KNOWN GAP — STORY-0149 AC-4 is not met, deliberately and
visibly … a constant countdown, not a vehicle traversing a road."* Any mechanism justified by "the
agent hid the violation" is justified by something that did not happen. What is missing is a verdict
on the disclosure — again, disposition.

**#3 is inflated roughly threefold, and the inflation has already propagated.** `b3857f5`'s three
findings are F1 (the flag), F2 (`Market::remove` leaks `reserved`/`requested`/`dispatches`) and F3
(`set_requested` has zero production callers). Only F1 turns on `optout_exttrade`. The "three claims"
phrasing is now copied verbatim into `docs/dev-cycle.md:98` and
`.claude/agents/substrate-cartographer.md:28`.

**And #5a's durable lesson is not "check paths."** `CLAUDE.md` — the one file every agent
auto-loads — was `.gitignore`d for the project's entire life until `5cf7953`. No version of the
offending text is in history, so the claim rests on the self-report of the commit that removed it:
the weakest evidence class in the inventory, and precisely the class this study exists to distrust.
**No review mechanism catches a defect in a file it cannot see.**

### The tag shape that caused failure #1 — and it is still in the grammar

`requirements/EPIC-036.md:94`, verbatim:

> AC-4: The dispatch state machine is implemented as an extension of the existing
> `map_dynamic::Dispatcher` … **not a bespoke parallel trip mechanism**.
> `[SUBSTRATE: PARTIAL — map_dynamic::Dispatcher exists and is in use, souls/freight_station.rs:5-9]`

**Every word of that tag is true, and it caused the failure.** `Dispatcher` does exist and is in
use — *by trains*. The tag never says "for trucks", and the unstated inference is the entire defect.
The tell nobody caught: at the fork, `world.rs:81` already **un**registered trucks
(`DispatchID::SmallTruck(id)`) while nothing ever registered one.

**The `[SUBSTRATE: …]` grammar has no slot for "exists, but not for your case."** That is a
one-field fix to a format this project owns, and it is cheaper than any agent.

### The inventory is itself an instance of the failure it describes

Two of six claims are inflated in the direction that flatters the proposed solution — #3 threefold,
#5b with its arrow of time reversed and its count off by one. **Both are already copied verbatim
into `.claude/agents/`**, where they will be inherited by every future agent as settled fact. The
brief that opens by warning "this repo has repeatedly ratified documents describing things that were
never built" is, in two of six entries, doing it.

---

# PART 2 — THE OPERATIONAL SPEC

## 2.1 The design, and the eight moves

**Keep the eight phases. Delete the hand-editable surface that can hold a false claim rather than
policing it: generate what can be generated, test what must be tested, and let a document that
cannot be written wrong replace a validator that can be muted.**

### The eight moves

Each has an owner, a trigger, and a red-then-green demonstration. Nothing else is built.

| # | Move | Reaches |
|---|---|---|
| 1 | **~30 lines in `build_roadmap.py`** — interpolate its two hardcoded literals, assert its computed totals, emit `RESUME.md`'s state table and the corpus counts as generated blocks | findings #1, #7, #8, #9 |
| 2 | **The golden-hash determinism test** — fixed seed, N ticks, committed hash; plus a run from a decoded save | **#12**, the only unbounded one |
| 3 | **The sentinel guard test** — every minted `SENTINEL-NNNN` resolves to a test fn; selector non-empty | #4 |
| 4 | **Pre-commit path-deletion sweep** — `git diff --diff-filter=DR` against a reverse index | #5b, #11 |
| 5 | **Fix `RESUME.md:84` and `:117`** — the live trap, and the false determinism claim | #2, #12 |
| 6 | **One table cell**: give `soviet-authenticity` a Phase-4 gate | the project's #1 known defect |
| 7 | **`docs/solutions/INDEX.md`** — generated index over the memories that already exist | the memory-discovery failure |
| 8 | **Four one-shot fixes** — backlink bug, two dead paths, `iteration-log.md` line, `rm -rf` the Bevy skills | #3, #5, #6, #11 |

Everything in §2.4 beyond these eight is either a **discipline** (§2.4.6 disposition, §2.4.5
reviewer shape, §2.4.8's brief-template line) or a **burn-down that fails only when it worsens**
(§2.4.2b, §2.4.9). No standing validator. No new agent. Nothing re-extracted.

Three moves exist only because the failure inventory was re-derived rather than inherited (§1.7):
**disposition** (#6's bug was described correctly twice and nobody acted), **the deletion sweep**
(#5b's arrow of time is reversed), and **the brief-template rule** (#1's substrate tag was true in
every word). A fourth — the determinism test — exists only because the review gate found what I
missed.

### What this costs

`dev-cycle.md:302-312` makes cost the reason its gate ordering exists: ~675k per iteration × 13
remaining ≈ 8.8M tokens. A design that ignores that is not a design for this project. The eight moves
are one-shot engineering, not per-iteration overhead — **roughly one implementer's worth of work
total, and near-zero recurring cost.** Two things in §2.4 would have added real recurring cost and
are deliberately defused:

- Phase 0's `UNAUDITED` rule is **"resolve or record why not", not blocking.** As a blocking gate it
  would be 17 ACs × one opus cartographer at ~80k ≈ **1.36M tokens**, mandatory — more than the
  entire defect class it sits beside is worth.
- §2.4.5's no-merge reviewer rule means the lead reads unreduced reports. That is a real cost, and
  §2.8 item 6 already notes the lead is the bottleneck. Accepted, because merging is where findings
  get lost.

## 2.2 What already exists — keep / modify / replace

| Existing piece | Verdict | Reason |
|---|---|---|
| `docs/dev-cycle.md` 8 phases | **KEEP** | Better than all four on failure-naming and gate ordering. It is the spine |
| Phase 0 GROUND | **KEEP + FIX** | No framework equivalent; it is our answer to spec-vs-CODE. But its stated cost model is false — the cartographer memory it depends on is empty (finding #10). Fix the store, not the phase |
| Phase 1 PLAN (lead only) | **KEEP** | Correct; the shared-file ownership warning is hard-won |
| Phase 2 BUILD | **MODIFY** | The only phase that names no failure it prevents. Give it one: the shared `scenarios/mod.rs` declaration clobber it already describes elsewhere |
| Phase 3 PROVE / `evidence-auditor` | **KEEP** | "Every guard seen failing" is stronger than anything in the four frameworks |
| Phase 4 GATE, cheap→expensive | **KEEP + ONE CELL** | Ordering proven: `wiring-auditor` reproduced two findings of a 112k opus gate and added one it missed. **But `soviet-authenticity` is the only advisor marked `0 / —` in `dev-cycle.md:29` — every other advisor is `0 / 4`, i.e. Phase-0 advice *plus* a Phase-4 hard sign-off.** It has no gate at all, and it is the agent hired specifically because the standing verdict is "looks like something done by a child." `CLAUDE.md` already mandates the artifact it would judge (the 15–20s video), and `RESUME.md:107` records that video as still owed. Give it a `0 / 4` slot on any iteration whose diff touches `native_app/`. One table cell, zero new mechanism |
| Phase 5 DISPOSITION | **KEEP** | Re-verify-before-acting is correct and evidence-backed |
| Phase 6 WRAP / `doc-reality-auditor` | **MODIFY** | Its arithmetic duties disappear because the documents become generated (§2.4.1), not because a validator takes them over. Keep the agent for staleness that needs judgment — which is where all three expensive failures lived. Add one mechanical step: flip `ABSENT` → `OURS` on every `done:ITER-NNNN` story's ACs (§2.4.9) |
| Phase 7 SHIP | **KEEP** | Unaffected |
| 15 project agents, sonnet/opus tiering | **KEEP** | Confirmed 15 exact, 9 sonnet / 6 opus. Note it is really three tiers: 6 opus advisors, 4 sonnet read-only auditors, 5 sonnet writers |
| Two-layer `br` protocol | **KEEP + ENFORCE** | Mechanism confirmed (subagents genuinely cannot see Claude task tools — re-probed live). But only **2 of 17** issues carry a worker comment. The protocol is barely practised |
| 15 private agent memories | **REPLACE** | 3 orphaned dirs for deleted agents hold 20 unreachable files (a 4th is empty); `doc-reality-auditor`'s store is empty and `substrate-cartographer`'s was until this session. Isolation is *conventional, not enforced* — all 15 declare `memory: project` and any agent with Read/Glob can already read a peer's |
| `behavior-corpus.md` | **KEEP THE FILE, FIX THE COLUMN** | 153 rows of good scenario text. The `Command` column is the defect, not the corpus |
| Six named sentinels | **KEEP, RE-DERIVE MEMBERSHIP** | Keep JOURNEY-0001. Replace folder-derived membership with earned promotion |
| `.claude/skills/bevy-*` | **DELETE** | Two skills for a discarded engine, polluting every session's routing surface |
| `docs/wayfinder-brief.md` | **KEEP AS HISTORICAL** | Already marked superseded; leave it |

## 2.3 The phases, with the deltas

The eight phases are unchanged in name and order. Three carry new content:

```
0 GROUND → 1 PLAN → 2 BUILD → 3 PROVE → 4 GATE → 5 DISPOSITION → 6 WRAP → 7 SHIP
    ▲                                                                 │
    └── APPROVAL GATE (hook-enforced, once per iteration) ────────────┘
```

**Phase 0 GROUND — add the standing backlog.** The corpus already contains **17 `UNAUDITED` ACs**,
each stating precisely what was not checked (*"audit did not enumerate other callers of Lot
construction"*, *"no per-item network-borne flag exists"*). These are 17 pre-written cartographer
briefs. An iteration that touches a seam carrying an `UNAUDITED` AC resolves it in Phase 0 or does
not proceed. Exit gate additionally: the cartographer wrote a dated, commit-stamped fact-sheet **to
the shared pool** (§2.4.4), not to a private directory.

**Phase 4 GATE — no new gate; one missing cell.** The first draft added a validator as "gate 0"; that
is cut. What Phase 4 actually lacks is a slot for `soviet-authenticity`, the only advisor in
`dev-cycle.md:29` marked `0 / —` rather than `0 / 4`, and the one hired for the project's worst known
defect. Give it a sign-off on iterations touching `native_app/`.

**Phase 6 WRAP — the auditor stops counting, because there is nothing left to count.** The counts
move into generated blocks (§2.4.1), so `doc-reality-auditor` is not relieved by a validator — it is
relieved because the documents can no longer disagree with their source. It keeps what needs
judgment: does this document describe an architecture we abandoned, is this instruction still the
right instruction. It also gains one mechanical step, the `ABSENT` → `OURS` flip (§2.4.9).

## 2.4 The mechanisms

> **This section was rewritten after the review gate.** The first draft's centrepiece was a
> standing validator, `check_claims.py`, that policed the repo's hand-written documents. An opus
> critic established that (a) **none of the eleven findings would have cost an hour of real work**,
> (b) the two with any consequence want a *test* and a *one-line script fix* rather than a
> validator, (c) a validator over hand-editable markdown decays into `|| true` within about six
> loosenings — arriving at `validate_iteration_log.py`, the very exhibit this study prosecutes, by a
> different road, and (d) it found a **twelfth** finding, semantic and about determinism, that no
> amount of counting reaches. The draft had done exactly what its own §2.8 item 9 confessed:
> privileged the arithmetic defect because arithmetic was what it could verify, named the flaw, and
> kept the machine anyway. Naming a flaw is not mitigating it.
>
> The revised principle: **delete the surface that can hold a false claim, rather than police it.**
> A generated document cannot be hand-written wrong. That is the only form of document validation
> that does not decay.

### 2.4.1 Make the generated documents fully generated ★ REVISED

No new script. **Three changes to one file that already exists**, `build_roadmap.py`, which already
globs `requirements/EPIC-*.md` at `:79` and already computes every number in question at `:265-268`
before printing them to a console nobody reads:

1. **Interpolate the literals at `:192`.** It hardcodes *two* — `"plus 1 deferred"` and
   `"across 14 iterations"` — and embeds a reference to `sov-scope-cut-1p6`, a ticket closed on
   2026-08-22, which is finding #8 sitting inside the generator. (Kills #1.)
2. **`assert` its computed totals against the corpus** before emitting. A generator that can compute
   a number and states a different one should fail, not print.
3. **Emit `RESUME.md`'s state table and `behavior-corpus.md`'s counts as generated blocks**, between
   markers, so findings #7, #8 and #9 cannot be hand-written wrong. `RESUME.md:5` calls itself *"the
   narrative handoff"* — the prose stays hand-written and mutable, which is its purpose; only the
   state table is generated.

Then four one-shot fixes that need no standing mechanism at all: `test -f` or delete the
`iteration-log.md` line (#6); `rm -rf` the two Bevy skills (#11); fix `backlink_scenarios`' first-match
bug and re-run it (#5); correct `governance.rs` → `economy/government.rs` and `prototypes/load.rs` →
`prototypes/src/load.rs`.

That is **~30 lines in one existing file plus four commands**, and it reaches nine of the twelve
findings — by removing the hand-editable surface rather than adding a policeman to it.

**What is deliberately NOT built:** a standing validator, a mutation test per check family, a
validator self-count assertion, a `Stop` hook, and a Phase-4 gate 0. All were in the first draft.
The two findings with real consequence are handled below by a test and a script fix.

*Provenance:* the **idea** that produced documents must be checked is `compound-engineering`'s
`validate-doc-claims.py`. The **conclusion** — generate rather than validate — is **new**, and it is
the critic's, not mine.

### 2.4.0 The golden-hash determinism test ★ NEW — and the highest-value item here

**Finding #12, which the first draft missed entirely.** `RESUME.md:117` tells every agent:

> Its `tick()` does serialize → deserialize → per-key hash compare every tick — **a real determinism
> check.**

`roadmap.md:28` builds on it: *"`TestCtx::tick()` already asserts determinism per tick … that nearly
hands us the journey's determinism observable free."*

Verified at `simulation/src/tests/mod.rs:107-121`:

```rust
fn check_determinism(&self) {
    let serialized = common::saveload::Bincode::encode(&self.g).unwrap();
    let deserialized: Simulation = common::saveload::Bincode::decode(&serialized).unwrap();
    let testhashes = self.g.hashes();
    for (key, hash) in deserialized.hashes().iter() {
        assert_eq!(testhashes.get(key), Some(hash), ...);
    }
}
```

It encodes, decodes, and compares the decoded state's hashes to the live state's. **That proves
`encode ∘ decode` is the identity. It is a save/load round-trip guard, and a good one.** It is not a
determinism check: it cannot detect two runs from the same seed diverging, because there is only ever
one run. It is also vacuous for any state `hashes()` itself omits — both sides agree by construction.

Why this matters more than all eleven arithmetic findings combined: the brief names determinism as
load-bearing, the sim hash-compares every tick, and **`RESUME.md` is the first file every agent
reads**. The claim is a *permission* — an agent that reads it stops looking for a determinism guard,
because it has been told one exists.

**The mechanism:** one test, ~20 lines. Fixed seed, N ticks, `hashes()` compared against a committed
constant; plus a second run started from a decoded save, compared to the in-memory run at the same
tick. Fix `RESUME.md:117` and `roadmap.md:28` in the same commit.

*Provenance:* **new, because** a document asserted a guard the code does not provide, and this is the
one instance where that cost is unbounded rather than cosmetic.

### 2.4.2 The sentinel set becomes real, and earns its membership

Three changes, all small:

**(a) The named set must resolve — but fix the identity first.** There is a live contradiction at
this seam, independent of this design. `behavior-corpus.md:7-8`: *"Scenario IDs are reassigned on
every re-aggregation — **sentinels are tracked by TITLE, not ID**."* `scenarios/mod.rs:2-3`: each
test fn *"carries its **stable corpus ID**"*. Both cannot be true. Keying a guard on the ID, as the
first draft did, means that after the next re-aggregation `sentinel_scenario_0115_*` silently proves
a different behavior — a green check with a substituted subject, worse than finding #4 because it
actually runs.

So: **mint an immutable `SENTINEL-NNNN` key at promotion**, carried in the test fn name and in a new
corpus column, and reconcile `behavior-corpus.md:7` with `scenarios/mod.rs:2`. Then the guard test
asserts every minted key resolves to a test fn and that the selector matches a non-empty set — ~10
lines, and it converts finding #4 from a doc comment into a guard.

**(b) `Command` is filled or the row is not a sentinel.** A row whose `Command` is `TBD` may not
carry cadence `sentinel`. Today that fails on all six — so it ships with a burn-down, not as a
blocker: the check records `expected_failures` and **fails only when the number goes up**. Every
check in this design that starts red uses that shape; a check that is red on arrival and cannot go
green is a check someone will suffix with `|| true`.

**(c) Promotion is earned — and seeded, not started cold.** A scenario becomes a sentinel when it has
regressed **twice**, replacing `extracting-requirements/SKILL.md:158-162`'s `kind == journey` rule,
which for a brownfield fork selects by which spec folder a behavior was written up in. But "promote
on second regression" needs two observed regressions, which on day one means a set of size one.
**Seed it instead from evidence already in the repo:** the 33 `[SUBSTRATE: CONFLICTS]` ACs and the
seams with the highest defect density in `git log` — the places this project has already been shown
to break, rather than the places a spec folder happened to describe. That gives a non-trivial set
immediately without importing the greenfield assumption.

The counter needs somewhere to live: a committed `sentinel-ledger.md`, one row per minted key. The
first draft required this counter and specified no artifact for it.

**(d) Baseline-before / compare-after, with the defect inverted.** Run the sentinel set before the
iteration and again after; a scenario that passed at baseline and fails after is a regression and
blocks. **And unlike upstream, a pre-existing sentinel failure also blocks.** Upstream's
`running-an-iteration/SKILL.md:38` says *"the failure predates this iteration … proceed with the
iteration"*, and `id-adversary` traced the consequence: once a sentinel survives one iteration
boundary it is permanently exempt, and `progress.md` reads `3/10 passing` forever.

*Provenance:* cadence column and baseline protocol from `iterative-development`
(`behavior-evidence-formats.md:125-129`, `running-an-iteration/SKILL.md:33-38`, `:88-90`). Earned
promotion and blocking-on-pre-existing are **new, because** the folder rule mis-selects on a fork and
the log-and-continue rule decays to nothing.

### 2.4.3 One approval gate — on the artifact, not the dispatch ★ REVISED

**The first draft specified a `PreToolUse` hook on `Agent` that would refuse to dispatch a Phase-2
implementer "while the current `br` iteration issue lacks an approval comment." That is not
buildable, and the critic was right to call the §2.8 admission about it a decoy.** Two reasons:

1. **The hook cannot know which issue is current.** A `PreToolUse` hook on `Agent` receives
   `tool_name`, `tool_input.subagent_type`, `tool_input.prompt`, `session_id`, `transcript_path`,
   `cwd` (see `~/.claude/hooks/agent-routing-guard.js:71-86`). Nothing identifies an iteration, and
   `br` has no iteration entity — `br list --status open` returns nine issues with no `ITER-*` among
   them. The hook would have to guess, and fail either open (decorative) or closed (every dispatch
   refused).
2. **It gates one channel of many.** The same work lands via `subagent_type: implementer`,
   `general-purpose`, or the lead's own `Edit`. `agent-routing-guard.js` exists precisely because
   generic subagent types are routine here.

**What replaces it:** one P1 `br` issue per iteration (`sov-iterNNNN-approval-*`), and a pre-commit
check that fails while a commit touching `simulation/src/**` lands with that issue still open.

This gates the **artifact**, not the dispatch. It needs no session context, cannot be routed around
by changing `subagent_type`, and degrades correctly against `CLAUDE.md`'s delivery posture — *"A task
handed over as a finished brief to execute gets reasonable calls and steady progress, no blocking."*
A finished-brief run closes the approval issue up front in one command and never sees the gate again;
a collaborative run leaves it open and gets the checkpoint `CLAUDE.md` asks for. A `PreToolUse` hook
could not have told those two modes apart, because the mode lives in the human's framing and not in
the tool payload.

**What it asks**, unchanged: is the substrate fact-sheet present for every seam this touches; is the
scope inside the charter; is this the thing you want built next.

**What it does not do:** fire per task. `superpowers`' own release notes record a session that *"sat
blocked for almost nine hours"* on a question the controller could have decided. Between gates,
rulings not stalls.

*Provenance:* concept from `superpowers` `brainstorming/SKILL.md:14-20`. Artifact-level placement is
**new, because** all four frameworks state gates as prose, and because the enforcement surface this
machine actually offers does not carry the context a dispatch-level gate would need.

### 2.4.4 An index over the memories that exist — not a migration ★ REVISED

**The first draft proposed replacing `.claude/agent-memory/<agent>/` with `docs/solutions/`. That was
wrong on two counts the critic caught.** First, all **15 of 15** agent bodies name their own private
directory and say "Read `MEMORY.md` first" (`sim-implementer.md:103`,
`substrate-cartographer.md:125`, and 13 more), and `memory: project` auto-provisions it — so a
migration that does not also edit 15 agent files leaves **two** stores live, which is the failure
doubled. Second, the files to migrate include `catalogue-implementer/sim-plugins-group-shape.md`,
which opens *"`SimPlugins` (`src/lib.rs`) and `GamePlugins` (`src/game/mod.rs`) are Bevy
`PluginGroup`s… (ADR 0012, ticket #118)"* — Bevy, deleted paths, GitHub-era tickets. §1.5 finding #11
condemns two Bevy *skills* for polluting the routing surface; the draft would have moved nine files
of Bevy *lore* into a pool every agent greps, seeding the rot §2.8 item 5 predicts before the first
read.

There is also a shape error. `memory: project` is a **push** — the harness injects the store.
`docs/solutions/` is a **pull** — grepped on demand behind a pointer. The evidenced failure is a
*discovery* failure (`sim-implementer` cannot find `ledger-invariant-checker`'s `sell_all` learning),
and replacing push with pull makes discovery worse.

**So: move nothing.** Build only the index — which is what §2.4.4's own first draft admitted the
change really was.

- A generated `docs/solutions/INDEX.md` listing every `.claude/agent-memory/*/*.md` by its
  frontmatter `description:`, regenerated like any other generated document (§2.4.1).
- One pointer line in `CLAUDE.md`, which every agent auto-loads.
- `rm -rf` the three dead orphan dirs whose content is superseded, **keeping**
  `prototype-researcher/`'s W&R and Factorio files, which are still true and still useful.

Zero agent-file edits, zero migration, no new store, and the one evidenced failure is solved —
because access was never restricted in the first place.

**The evidence this is needed** is stronger than the brief's version. The brief cites
`ledger-invariant-checker` learning that `sell_all` re-posts off full capital while `sim-implementer`
cannot read it. Verified — that learning is real, at
`.claude/agent-memory/ledger-invariant-checker/break-families.md`. But the actual state is worse:
**4 memory directories belong to agents that no longer exist** (`catalogue-implementer`,
`presentation-implementer`, `prototype-researcher`, `refactor-reviewer`), holding 17 files no current
agent can reach; `doc-reality-auditor/` is **empty**, and `substrate-cartographer/` was empty for the entire period
`dev-cycle.md` asserted its cost model — until this study's own cartographer wrote two files there at
01:08 on 2026-08-24 (see the note in §1.5). Precise current state: 10 directories, of which 3 orphans
hold **20 files** (`catalogue-implementer` 10, `refactor-reviewer` 6, `prototype-researcher` 4), a
4th orphan (`presentation-implementer`) is empty, and `doc-reality-auditor` is empty.

**And the isolation was never real.** All 15 agents declare `memory: project`; global settings grant
unscoped `Read`; there is no project `settings.json` and no hook. `sim-implementer` could already read
a peer's memory file — nothing tells it to. So this is not an architecture change. **It is an index.**

*Provenance:* the **discoverability pointer** is `compound-engineering`'s Phase-2.5 check. The
**shared-pool-as-migration** idea was theirs too and is **rejected** here for the reasons above.
Retained from ours: the cartographer's "record claims that turned out to be false, dated and
commit-stamped" discipline, which is better than anything upstream.

### 2.4.5 Reviewer shape: disjoint axes, no merge

Phase 4's gates already run distinct lenses. Two refinements:

- **Reviewers report side by side and are never merged or reranked.** From `mattpocock`
  `code-review/SKILL.md:76,78`. Its justification lives one layer out, in
  `docs/engineering/code-review.md:72`: *"There is no convergence guarantee… do not run it in a loop
  until it comes back clean, because it will not."* The rule is adopted on the body's authority; the
  quote is context, not evidence.
- **The truth axis is named explicitly in every reviewer brief.** Our 16 PAR reports have a
  `classification:` vocabulary with no truth-check category — that is *why* they were all
  omission-shaped. Borrow the loudest sentence in the study,
  `spec-compliance-reviewer-prompt.md:21`: `## CRITICAL: Do Not Trust the Report` … *"Verify
  everything independently by reading the actual code."*

**Not adopted:** PAR's two-reviewers-with-identical-inputs pairing. That is variance sampling and it
doubles cost; `dev-cycle.md`'s cheap-to-expensive ordering with *different* lenses is strictly better
value, and is measured.

### 2.4.6 Disposition — every recorded anomaly gets a verdict ★ NEW

**This is the mechanism the failure inventory actually demanded, and I would not have built it from
the brief's version of the story.**

Failure #6 — the economy crediting goods for free every tick — was described correctly, in the repo,
**two days before** a specialist "discovered" it. `roadmap.md:23` names the line range, the mechanism
and the consequence. `behavior-scenarios.md:16` restates it. It was filed as a *test-fencing
precondition* rather than a bug, and nobody acted for two days. Failure #1 is the same shape: the
implementer disclosed the AC violation in its own commit message, and no verdict was ever recorded.

Adding another detector would have caught neither. Both were already detected.

**The rule:** any anomaly recorded anywhere — a reviewer finding, a commit message's own KNOWN GAP, a
roadmap caveat, a fencing precondition, a `[SUBSTRATE: CONFLICTS]` tag — carries exactly one of four
verdicts, in `br`, with an owner:

```
bug      → a br issue exists, linked to the recording document
accepted → a ruling with the reason, and the reason is not "for now"
fenced   → a test or precondition contains it, WITH AN OWNER AND AN EXPIRY DATE
filed    → deliberately deferred, with the br id that carries it
```

The pre-commit pass gains one check: **every anomaly-shaped record in a tracked document resolves to
a `br` id**, and a `fenced` verdict past its expiry fails. The 33 `[SUBSTRATE: CONFLICTS]` ACs are
the standing input — each is a recorded, undispositioned anomaly today — so this check also ships as
a burn-down (`expected_failures = 33`, fail when it rises), not as a wall.

**Its own most likely failure**, stated because it is the mechanism most prone to becoming a rubber
stamp: `accepted` requires no work, and the expiry on `fenced` is set by the person who wants the
fence. The failure signature is a run of `accepted` verdicts whose reasons are all a variant of "for
now", and no check can detect that, because the check is semantic. It is a discipline, not a guard,
and it should be described that way rather than dressed as enforcement.

Phase 5 already does this for *reviewer findings* and does it well. The change is that Phase 5's
discipline now covers anomalies recorded **outside** a review — which is where both real failures
were.

*Provenance:* the four-verdict discipline is **ours**, `dev-cycle.md` Phase 5. Extending it beyond
reviews is **new, because** both #6 and #1 were detected-and-ignored, not undetected. Nearest upstream
relative is `ce-brainstorm/references/settled-decisions.md`'s provenance classes, which stop an agent
re-litigating a decided question; this stops one from never deciding it.

### 2.4.7 Re-validate on tree change, not on authoring ★ NEW

Failure #5b, corrected: the four agent definitions were **right when written** on 2026-08-17 and were
invalidated on 2026-08-22 when the fork deleted `src/sim/` and `src/game/`. Failure #2 is live right
now for the same reason — `RESUME.md:84` still says *"`souls/freight_station.rs` is the ONLY correct
prior art for driving a dispatched delivery"*, with no parking warning, and `CLAUDE.md` tells every
agent to read `RESUME.md` **first**. The next agent on that seam walks into the identical trap.

Authoring-time care cannot fix either. The trigger must be the tree moving.

**Mechanism — corrected after the gate.** The first draft proposed a `PostToolUse` hook on
`Write|Edit`. **That hook is blind to the event class it was built for.** The fork deleted those
paths at git level — `git show --stat 68fe28c | tail -3` → *"577 files changed"* — and `PostToolUse`
on `Write|Edit` never fires for `git rm`, `git mv`, `mv`, a rebase, a merge, or a checkout. It would
have caught none of #5b, #2 or #11.

So: **a pre-commit pass only**, using `git diff --name-status --diff-filter=DR` to see deletions and
renames, against a reverse index from source paths to the documents citing them. Today the whole
index is **27 distinct code paths** cited by 102 ACs, plus the paths named in 15 agent files. One
mechanism, one trigger, and it genuinely covers #5b and #11.

**It does not cover #2**, and the first draft claimed it did. `freight_station.rs` never moved; what
changed was what it is *applicable to*. No path index detects that — see §2.4.8, and the brief-template
rule that actually addresses it.

*Provenance:* **new.** Nearest relative is `ce-compound-refresh`, which re-checks docs against code —
but on demand and by LLM re-investigation, not triggered by the change that invalidated them.

### 2.4.8 One field in the `[SUBSTRATE: …]` grammar

`[SUBSTRATE: PARTIAL — map_dynamic::Dispatcher exists and is in use, souls/freight_station.rs:5-9]`
was true in every word and produced the project's most expensive failure. "In use" carried an
unstated "for trains, not for your case."

Add a **for-what** field. `PARTIAL` and `PROVIDED` must state what the substrate is provided *for*:

```
[SUBSTRATE: PARTIAL — map_dynamic::Dispatcher exists; USED-BY: FreightTrain only;
            NOT-USED-BY: SmallTruck (registration commented, dispatch.rs:94-102)]
```

**But the tag grammar is the wrong home for the rule, and this is where the first draft misfired
worst.** `[SUBSTRATE: …]` exists only in `requirements/EPIC-*.md`. The string `SUBSTRATE` appears in
exactly one file outside the corpus — `.claude/agents/substrate-cartographer.md:78`. `RESUME.md` has
none. `CLAUDE.md` has none. Agent definitions have none. **The three document classes where this
failure actually occurred are all outside the grammar the fix extends.** Failure #1's tag was in the
corpus; failure #2 lives in `RESUME.md:84`, which has no tags at all.

So the rule goes in the **brief template**, where every document class passes through:

> Every prior-art citation in a dispatch brief states the consumer it was observed serving.
> *"`freight_station.rs` drives a dispatched delivery **for `FreightTrain`**; no truck path exists —
> verify before copying."*

One line in `dev-cycle.md`'s "Briefing an agent" section. Costs nothing, covers `RESUME.md`, agent
definitions and briefs as well as the corpus.

The `USED-BY` / `NOT-USED-BY` field on `PARTIAL`/`PROVIDED` tags stays as a **secondary, burn-down**
item (45 ACs: 26 PROVIDED + 19 PARTIAL) rather than a required field with no backfill owner. Note the
sharp edge the first draft walked into: the only producer of that grammar is
`extracting-requirements`, a skill §2.6 **rejects entirely**, and `extract/validate.py:25-26`
enforces the current form — so a required new field would have had no author.

### 2.4.9 The 265-claim class nobody is watching

**72% of the corpus asserts a negative existential** — `[SUBSTRATE: ABSENT — …]`, 265 of 370 — and 57
of those cite a code path anyway, e.g. *"ItemPrototype is {base, id, optout_exttrade} only
(`prototypes/src/prototypes/item.rs:8-12`), no item ontology per audit §3"* (true today, verified).

Two problems the first draft missed entirely:

1. **The existence check runs backwards on them.** "Does the cited file exist?" goes green exactly
   when the file is present, while the claim is about what the file does *not* contain.
2. **The class decays continuously by design.** Every iteration that builds something turns ABSENT
   tags false. Thirteen iterations is thirteen rounds of silent falsification of the dominant tag.

**Mechanism:** at Phase 6 wrap-up, for every story marked `done:ITER-NNNN`, flip its ACs' `ABSENT`
tags to `OURS`. Then one assertion: *a done story may have no AC still tagged ABSENT.* Mechanical,
one pass, and it covers 265 claims for less than the 45-AC backfill above.

*Provenance:* **new** — the critic's, not mine.

## 2.5 Provenance of every element

| Element | From | Mechanism cited |
|---|---|---|
| 8 phases, failure-per-phase, gate ordering | **ours** | `docs/dev-cycle.md` |
| Phase 0 GROUND | **ours** | no equivalent in any of the four |
| "every guard seen failing" | **ours** | `evidence-auditor` |
| `br` two-layer tracking | **ours** | `CLAUDE.md`; subagent tool-isolation re-probed live |
| Definition of done | iterative-development | `iterative-development/SKILL.md:10` |
| Cadence column (task/iteration/sentinel) | iterative-development | `behavior-evidence-formats.md:125-129` |
| Baseline-before / compare-after | iterative-development | `running-an-iteration/SKILL.md:33-38`, `:88-90` |
| Proof-seam ladder + REJECT rules | iterative-development | `behavior-evidence-formats.md:20-26`; `spec-compliance-reviewer-prompt.md:58-60` |
| `TODO(ITER-NNNN)` hard gate | iterative-development | `running-an-iteration/SKILL.md:92-100` |
| Grounding validation (quote file:line or unverifiable) | compound-engineering | `ce-compound/references/grounding-validation.md` — note: explicitly **not** a hard gate |
| Grounding a *plan* against the codebase | compound-engineering | `ce-doc-review/references/personas/feasibility-reviewer.md` — *"This check requires reading the codebase alongside the plan"* |
| Discoverability pointer in the auto-loaded instruction file | compound-engineering | `ce-compound` Phase 2.5 |
| "code wins over doc" refresh rule | compound-engineering | `ce-compound-refresh/references/classify.md:15` + `SKILL.md:54` (two files) |
| Approval gate (concept) | superpowers | `brainstorming/SKILL.md:14-20` |
| File-handoff for dispatches | superpowers | `scripts/{sdd-workspace,task-brief,review-package}` |
| Scoped fix loop, capped, ruling-or-park | superpowers | `subagent-driven-development/SKILL.md:388-421` |
| No-subagents contract in dispatch prompts | superpowers | `implementer-prompt.md` |
| Disjoint two-axis review, no merge | mattpocock | `code-review/SKILL.md:76,78` |
| "Do Not Trust the Report" truth axis | iterative-development | `spec-compliance-reviewer-prompt.md:21` |
| Glossary / ADR three-condition rule | mattpocock | `ADR-FORMAT.md:31-35`; the "skip the ADR" and "devoid of implementation details" sentences are at `domain-modeling/SKILL.md:74` and `:64`. **Cited but not used by any phase — see §2.8 item 13** |
| **Generate rather than validate** | **new** (the critic's) | `build_roadmap.py:192` vs `:265` |
| **Golden-hash determinism test** | **new** | finding #12 — `RESUME.md:117` claims a guard `tests/mod.rs:107-121` does not provide |
| **Immutable `SENTINEL-NNNN` key** | **new** | `behavior-corpus.md:7-8` and `scenarios/mod.rs:2-3` contradict each other on ID stability |
| **Earned promotion, seeded from CONFLICTS + defect density** | **new** | folder rule mis-selects on a brownfield fork; pure earning starts cold |
| **Pre-existing sentinel failure blocks** | **new** | upstream's log-and-continue decays to permanent exemption |
| **Artifact-level approval gate (`br` issue + pre-commit)** | **new** | a `PreToolUse` hook cannot see which iteration is current; `br` has no iteration entity |
| Four-verdict disposition (bug/accepted/fenced/filed) | **ours**, extended | `dev-cycle.md` Phase 5 |
| **Disposition extended beyond reviews; `fenced` carries owner + expiry** | **new** | failures #6 and #1 were detected and ignored, not undetected |
| **Pre-commit path-deletion sweep (`--diff-filter=DR`)** | **new** | failure #5b's arrow of time; `PostToolUse` is blind to `git rm` |
| **Consumer named on every prior-art citation, in the brief template** | **new** | failure #1's tag was true in every word; failure #2 is live in `RESUME.md:84` |
| **`ABSENT` → `OURS` flip at wrap-up** | **new** (the critic's) | 265 of 370 ACs assert a negative existential that decays every iteration |
| **Phase-4 gate cell for `soviet-authenticity`** | **new** (the critic's) | `dev-cycle.md:29` — the only advisor with no gate, hired for the project's #1 known defect |

## 2.6 Deliberately rejected

A design that takes everything is not a design. What was left on the table, and why:

**From `iterative-development`:**
- **`extracting-requirements` and `scoping-the-simplest-core` entirely.** Two of six skills, ~330
  lines and 7 of 9 scripts, exist to turn spec prose into stories and an ordering. That phase is
  finished here: 36 epics, 149 stories, 130 scheduled, a roadmap that regenerates byte-identical.
  Re-running it would destroy the corpus and re-import the greenfield assumption.
- **The autonomous loop and catastrophe-only escalation.** `SKILL.md:140` overrides interactive gates
  by name and `:144` excludes spec ambiguity from escalation. This directly contradicts `CLAUDE.md`
  ("checkpoint at decisions of taste, scope, or cost") and the standing rule that a wrong approach
  costs more than buggy code.
- **All five validators as-is.** They check markdown shape. Importing prose gates into a repo that
  already has executable ones (`gate-review.js`, `evidence-auditor`'s mutation discipline) is a
  downgrade dressed as a framework.
- **PAR's identical-input pairing.** Doubles cost for variance sampling.

**From `superpowers`:**
- **`subagent-driven-development`'s workspace and ledger.** `br` already persists, is shared, and
  survives; SDD's ledger is gitignored and `rm -rf`'d at the end, so review verdicts are never
  durably recorded. We would be replacing a durable store with a disposable one.
- **`writing-plans`' single-file plan format.** Its No-Placeholders rule forbids compression, so 130
  tasks means one file with every implementation inlined; its spec-coverage check is a 370×130 skim
  the author runs on itself; SDD's pre-flight conflict scan is pairwise, up to 8,385 rows before
  task 1.
- **`receiving-code-review`'s `gh api` path.** No PR flow here.

**From `mattpocock`:**
- **`to-tickets` / `to-spec` / `triage` / `wayfinder`.** `dev-cycle.md` already ruled on this and it
  is right: they duplicate `br` and the roadmap, and two competing ticket systems is worse than
  either. Reinforced by `.out-of-scope/mainstream-issue-trackers-only.md`, which excludes a CLI
  tracker like `br` from first-class support.
- **The TypeScript-specific skills.** Not a TS project.

**From `compound-engineering`:**
- **The entire shipping tail** — `ce-babysit-pr`, `ce-commit-push-pr`'s PR machinery,
  `ce-resolve-pr-feedback`. No PR flow.
- **`ce-test-browser`, `ce-polish`, `ce-test-xcode`, `ce-product-pulse`.** No browser, no iOS, no
  users, no telemetry.
- **The SKILL.md-shrinking / reference-indirection style.** 15+ recent commits pushing bodies into
  `references/` for multi-host portability. Real per-invocation cost for a project that ships to one
  host.
- **Single-subagent grounding validation, as a standing check.** Kept as the shape to imitate when a
  claim genuinely needs judgment. The arithmetic majority is handled by not letting the document be
  hand-written at all.
- **`docs/solutions/` as a migration target.** See §2.4.4 — adopted as an index, rejected as a move.

**Not adopted from anywhere: a new agent.** The roster is 15, two of them have empty memory stores,
and the one hired for the project's worst known defect has no gate. The gap is not a missing role.

**And, after the review gate, rejected from my own first draft:** a standing `check_claims.py`
validator, a mutation test per check family, a validator self-count assertion, a `Stop` hook, a
Phase-4 gate 0, a `PreToolUse` approval hook, a `PostToolUse` reverse index, a required `USED-BY`
field, and a 17-file memory migration with 15 accompanying agent-file edits. Nine mechanisms, cut.
The brief asked for a design smaller than the sum of the four; the first draft implied eighteen
artifacts, three new conventions and nine migration steps, several with no owner and no trigger. The
critic counted them, and it was right to.

## 2.7 Migration path

**The corpus is not thrown away, and a design requiring that would be wrong.** 36 epics, 149
stories, 370 ACs, 153 scenarios and a byte-reproducible roadmap represent the single largest
investment in the repo, and `harness-auditor` confirmed `build_roadmap.py` regenerates `roadmap.md`
with zero diff — it is not drifted. Everything below is additive and ordered so each step is
independently useful.

| Step | Work | Proves itself by |
|---|---|---|
| **0** | **Fix `RESUME.md:84` and `:117` today.** `:84` tells the next agent that `freight_station.rs` is the ONLY correct prior art, with no parking/collider warning. `:117` calls a bincode round-trip *"a real determinism check."* `CLAUDE.md` makes this the first file every agent reads | The live trap stops being armed; the false permission is withdrawn |
| **0b** | Correct the "falsified three claims" inflation in `docs/dev-cycle.md:98` and `.claude/agents/substrate-cartographer.md:28` (only F1 turns on the flag), and the reversed-time claim in `.claude/agents/doc-reality-auditor.md:24-25` | Two agent definitions stop teaching a false lesson |
| **1** | **The golden-hash determinism test.** Fixed seed, N ticks, committed hash; second run from a decoded save compared at the same tick. Also fix `roadmap.md:28`, which repeats the claim | Mutate a tick-order-dependent system; watch it go red. This is the only guard here whose absence is unbounded |
| **2** | **~30 lines in `build_roadmap.py`** — interpolate `:192`'s two literals and its stale `sov-scope-cut-1p6` reference, `assert` computed totals, emit `RESUME.md`'s state table and corpus counts as generated blocks | Hand-edit a generated block; regeneration overwrites it. #1/#7/#8/#9 become unwritable rather than unchecked |
| **3** | **Sentinel guard test** in `scenarios/mod.rs`, after minting `SENTINEL-NNNN` keys and reconciling `behavior-corpus.md:7` with `scenarios/mod.rs:2` | Currently fails on all six. **That failure is the deliverable** |
| **4** | Fill `Command` for JOURNEY-0001 only; demote the other five to `iteration` until a command exists. Seed the sentinel set from the 33 CONFLICTS ACs and `git log` defect density | One real sentinel beats six declared ones |
| **5** | Fix `backlink_scenarios`' first-match bug and re-run, or hand-fix the 33 stories | 50 unreachable scenarios drop toward 0 |
| **6** | Four one-shot fixes: `governance.rs` → `economy/government.rs`; `prototypes/load.rs` → `prototypes/src/load.rs`; the `iteration-log.md` line (create or delete); `rm -rf` the two Bevy skills | Each verifiable in one command |
| **7** | `docs/solutions/INDEX.md` generated over the existing `.claude/agent-memory/*/*.md`; one pointer line in `CLAUDE.md`; `rm -rf` the three superseded orphan dirs, keeping `prototype-researcher/`'s W&R and Factorio files | `sim-implementer` can find `ledger-invariant-checker`'s `sell_all` learning. **Zero agent-file edits** |
| **8** | Pre-commit: path-deletion sweep (`--diff-filter=DR`), the iteration-approval issue check, and the disposition burn-downs | Delete a cited file; watch the commit name every document that cited it |
| **9** | Edit `dev-cycle.md`: Phase 2's failure, Phase 6's narrowed auditor scope, the `soviet-authenticity` gate cell, and one line in "Briefing an agent" naming the consumer on every prior-art citation | The advisor with no gate gets one |

Steps 0 and 1 are the ones that matter, and step 0 costs minutes. Everything after step 3 is cleanup
that the first three make visible.

**Every check that starts red ships with a burn-down, never as a wall** — record `expected_failures`
and fail when the number *rises*. Today that means 45 (the `USED-BY` backfill), 153 (`Command`
cells), and 33 (undispositioned CONFLICTS). A check that is red on arrival with no path to green is a
check someone suffixes with `|| true`, and then it is `validate_iteration_log.py`.

**Not on the path:** re-extraction, a new agent, adopting any framework's loop, or a rewrite of
`dev-cycle.md`.

## 2.8 How this design fails

Every one of the four has a blind spot that only appeared in practice. These are mine.

**0. The first draft of this section was, in part, an inoculation — and that is the most useful thing
the review gate found.** Item 9 below said plainly that the design "privileges the arithmetic defect
because arithmetic is what I could verify." It was true. The correct response to writing that
sentence was to shrink the mechanism; instead the draft kept the mechanism and kept the sentence,
which converts a self-criticism into a shield. Two other items were straightforwardly decoys: item 4
named an unfalsifiable risk (the human disables the hook) while the buildable objection was that
**nobody could build the hook at all**; item 7 said "nothing detects presentation problems" while
`dev-cycle.md:29` shows the detector is hired and simply has no gate. A section that lists ten
failure modes reads as candour and can function as armour. Assume this revision has the same defect
somewhere and I have not found it.

**1. Generated documents fail differently, not less.** Making `RESUME.md`'s state table generated
means it cannot be hand-written wrong — and means a bug in the generator is now authoritative and
silent, where a hand-written error at least had an author. The blast radius moves from one line to
every regeneration. `build_roadmap.py` is 270 lines with no tests.

**2. Nothing here catches the twelfth-class defect, and #12 proves the class is real.** Finding #12
was semantic, sat in the file every agent reads first, and no amount of counting reaches it. It was
found because one agent went looking at git history and read a function body. The design's answer is
the review gate itself — an adversarial reader with an explicit mandate to refute — which is a
process, not a mechanism, and costs an opus pass per wave.

**3. Sentinel seeding from CONFLICTS and defect density is a guess about the future from the past.**
It covers the seams that have already broken. The seam that breaks in ITER-0009 for the first time is
covered by nothing, and "promote on second regression" means the *first* regression always escapes by
construction.

**4. The artifact-level approval gate is weaker than the hook it replaces, deliberately.** It gates
commits touching `simulation/src/**`, so it does not stop an agent from burning 360k tokens building
the wrong thing — it stops that work from *landing*. The tokens are already spent by then. This is a
real regression against the first draft's intent, accepted because the stronger version was not
buildable.

**5. Indexing the memories rather than migrating them keeps every defect the memories already have.**
`docs/solutions/INDEX.md` makes `catalogue-implementer`'s Bevy-era lore *more* discoverable, not
less — the index points at nine files describing `src/game/mod.rs`, a path deleted five days after
they were written. Pruning is a judgment call the index does not make, and I have not specified who
makes it or when. `ce-compound-refresh` exists upstream for exactly this and is not adopted, because
per-doc LLM re-investigation is expensive. So the pool rots; I have only made it easier to read while
it rots.

**6. Everything here still assumes the lead reads the reports.** The wave that produced this document
generated nine reports totalling well over a megabyte of findings. The synthesis is mine and
unverified by anyone. Three claims in this document rest on a single agent's reading of a file I did
not open myself — flagged inline where they do. That is the same structural position the prior
session was in when it produced the conclusions this study just corrected.

**7. Presentation gets a gate cell, and a gate cell is not a solution.** The first draft said "nothing
detects this," which was a decoy — `soviet-authenticity` exists and `dev-cycle.md:29` gives it no
Phase-4 slot. Wiring it is one table cell. But an agent judging frames against a Soviet-authenticity
brief is not the same instrument as a human saying "this looks like a child made it," and the video
`CLAUDE.md` mandates has been owed since before this study started (`RESUME.md:107`). The gate makes
the omission visible; it does not make the judgment.

**8. Disposition is the mechanism most likely to become a rubber stamp.** Four verdicts where one
(`accepted`) requires no work is an invitation. The expiry date on `fenced` is the only part with
teeth, and expiry dates are set by the person who wants the fence. The failure signature is a run of
`accepted` verdicts whose reasons are all a variant of "for now", and nothing can detect that
mechanically, because the check is semantic. §2.4.6 now says this in its own section rather than
only here.

**9. This design privileged the arithmetic defect because arithmetic is what I could verify — and the
first draft acted on that insight by ignoring it.** Eleven findings, all countable, and a validator
built from them. The defects that actually cost this project were not arithmetic: #1 was a substrate
tag true in every word, #6 a correct description filed under the wrong heading, #12 a false claim
about determinism in the file every agent reads first. All three were found by agents reading git
history and function bodies, not by counting. The revision cuts the validator and keeps the three
mechanisms that address them — but the underlying asymmetry is unchanged and permanent: **I can
specify a check for what I can count, and only a process for what I cannot.**

**10. Two of the six inventory claims were inflated toward the solution their author preferred, and
this document did the same thing at least three times.** §1.7 documents the pattern in the brief.
Then the claim-reviewer found it here: a quote justifying a design decision attributed to a skill
body when it lives in a prose essay the study's own method excludes; a quote welded from two files
under one citation; "four commits shipped over the red suite" when the number is eight — each error
in the direction that flatters the argument. The correction pass was one agent, once. **Nobody has
re-derived §1.5's twelve findings independently of me**, and I both selected them and designed the
mechanisms that key off them.

**11. The two review agents disagree about what this document should be, and I picked one.** The
claim-reviewer found the document *"substantially true"* and returned citation corrections. The
critic returned a FATAL and argued the whole centrepiece should be deleted. I took the critic's side
and cut nine mechanisms. That was a judgment call made by the person whose work was being criticised,
under no independent adjudication, in a single pass — which is structurally the same position the
prior session occupied when it produced the conclusions §1.4 corrects.

**12. Nothing in this design has been built, and the one thing that would settle it is cheap.** Step
0 costs minutes and step 1 is about twenty lines. Until the determinism test has been seen going red
against a mutated tick order, §2.4.0 is a claim of exactly the kind this document exists to
prosecute.

---

## Appendix — verification status of this document

| Claim class | Status |
|---|---|
| Framework mechanisms, all four | Read at pinned SHAs by six agents; every claim carries file:line; two frameworks additionally adjudicated by an independent opus reader |
| The eleven findings (§1.5) | **All eleven verified by the lead directly** — commands run, output read. Independently re-derived a second time by `claim-reviewer` from primary sources; every one confirmed exact |
| **Finding #12** (`RESUME.md:117` calls a bincode round-trip "a real determinism check") | Found by `design-critic`; **`tests/mod.rs:107-121` read and verified by the lead directly.** This is the finding the study's own method missed and its review gate caught |
| Part 2 as revised | Rewritten after the gate. Nine mechanisms cut. **The revision has not been reviewed by anyone** |
| AC census (370 / 6 tag values / 215 / 102 code) | Independently re-derived by a dedicated agent with commands pasted; tag distribution matched the prior claim exactly |
| Backlink bug, 33/33, 50 unreachable scenarios | **Verified by the lead directly** |
| iterative-development red suite, `rc=0` mutation | Run by `id-adversary`; **not re-run by the lead** |
| `ce-compound` grounding validation mechanics | From `ce-reader`, re-derived from source by `claim-reviewer`. **Gap closed:** `feasibility-reviewer.md` has now been read — it is a second, plan-level grounding mechanism, and it strengthens rather than refutes §1.2's verdict |
| superpowers orphaned reviewer prompts | From `sp-adversary`'s greps; **not re-run by the lead** |
| The six empirical failures in the study brief | Re-derived by `failure-verifier` against code + git history at `HEAD=fdfabca`. **Two materially corrected** (#3 inflated ~3×, #5b time-reversed and count off by one); one still live (#2). **Not independently re-run by the lead** except the three items spot-checked below |
| `RESUME.md:84` still names `freight_station.rs` as the only prior art, no parking warning | **Verified by the lead directly** |
| "falsified three claims" propagated into `dev-cycle.md:98` and `substrate-cartographer.md:28` | **Verified by the lead directly** |
| Finding #10's reversal mid-study (cartographer memory written 01:08 today) | **Verified by the lead directly** |

**Known gaps in this study, stated rather than omitted:**

- `failure-verifier` reported that `ToolSearch("select:LSP")` returned `No matching deferred tools
  found` across three attempts while the `lsp-first-read-guard.js` hook still blocked `Read` on
  `.rs` files. All its code reads were done via `nl -ba` / `git show`, and all reachability answered
  by grep rather than `findReferences`. That is a harness defect worth its own ticket, and it means
  the reachability claims in §1.7 rest on grep, not on LSP.
- ~~`feasibility-reviewer.md` was never read.~~ **Closed by the review gate** — read, and it does
  ground against code. §1.3 is corrected accordingly.
- The 215-citation sub-partition differs slightly between the two agents who computed it: 108 md / 3
  C# versus 106 md / 5 C#. Both sum to 215 and both agree on 102 code citations, so this is a
  tie-breaking difference on ACs citing several file types in one bracket, not an error either could
  build a counter-example from.
- **`superpowers`' `task-reviewer-prompt.md` says reviewers must not re-run the suite *to confirm the
  implementer's report*, with an explicit carve-out for doubt-driven focused tests.** §1.2 renders
  this as "forbidden", which overstates it by a degree.
- iterative-development's red test suite and the `rc=0` mutation were run by `id-adversary` and not
  reproduced by the lead. They are the load-bearing evidence for §1.2's enforcement verdict.
- The charter (`docs/charter-1.0.md`) was not read in this study. Scope claims here defer to it and
  do not re-derive it.
- No mechanism in Part 2 has been implemented or tested. Every "proves itself by" in §2.7 is a
  prediction, not a result.
