# Documentation framework and patterns for soviet-simulator

**Kind:** research report
**Authority:** advisory — findings for the lead to disposition
**Status:** draft
**Owner:** researcher
**Last verified:** 2026-08-28

---

## Bottom line

The project already has most of the right instincts — a metadata header block, the
charter-binds-scope / spec-binds-mechanism / bd-binds-task hierarchy, evidence labels, a
`doc-reality-auditor` in the roster — but lacks three things: (1) a brief/story document type
that agents can self-service from, (2) a machine-checkable freshness mechanism beyond `Last
verified`, and (3) explicit rules that keep agent-facing docs short, tabular, and DRY. The
biggest anti-pattern is already documented in the audit trail: a stale string (`br` → `bd`) that
propagated into seven binding documents because there was no freshness check on the thing doing
the checking.

---

## Part 1 — Catalogue of document types

Each entry covers: purpose, canonical template fields, lifecycle states, who writes/reads it,
where it lives, staleness risk, and citation.

### 1.1 Charter / product contract

**Purpose.** Binds scope and cut line. Not a spec; does not assert implementation state.

**Fields.**
- Title and version
- Authority block (Kind/Authority/Status/Owner/Last verified)
- Purpose and identity (what the product is, its non-negotiable pillars)
- Scope table (commitment rows; each row is a what, not a how)
- Post-1.0 and Never lists (absolute exclusions)
- Scope change protocol (how the scope can be amended)

**Lifecycle.** `draft` → `ratified` → `amended` (tracked by a ratified amendment ADR, never by
silent edits). A charter does not become `superseded`; it becomes `amended` with a dated note.

**Writes / reads.** Lead writes; all agents read as first authority.

**Location.** `docs/plan/charter-<version>.md`

**Staleness risk.** Very low — scope-only, implementation-agnostic. Highest blast radius when
stale because it is the root of every agent's scope check. Freshness check: grep for the tracker
tool name and any versioned tool references on each iteration boundary.

**Citation.** This project's own `docs/plan/charter-1.0.md` [live, read 2026-08-28]; CLAUDE.md
authority hierarchy [live].

---

### 1.2 Architecture Decision Record (ADR)

**Purpose.** Records a single, significant design choice and its rationale so future agents do not
re-litigate it unknowingly.

**Fields (MADR 4.0 template).**
- Title phrased as problem-solution ("Use X for Y")
- YAML front matter: status, date, decision-makers, consulted, informed
- Context and Problem Statement
- Decision Drivers
- Considered Options (with pros/cons)
- Decision Outcome
- Consequences (positive and negative)
- Confirmation (how to verify the decision has been implemented or remains appropriate)
- More Information (links to specs, fact-sheets)

**Lifecycle.** `proposed` → `accepted` → `deprecated` | `superseded by ADR-N`

An accepted ADR is immutable except for a `superseded by` pointer. Superseding requires a new
ADR; the old one keeps its text.

**Writes / reads.** Lead or domain advisor writes after a significant choice; implementers and
gate agents read before touching the affected seam.

**Location.** `docs/decisions/ADR-NNNN-<slug>.md`

**Staleness risk.** Medium. A decision can be overtaken by a substrate change. Mitigation: the
Confirmation field says what evidence verifies continued applicability; `doc-reality-auditor`
checks that the cited code still matches the stated decision.

**Citation.** MADR 4.0, released 2026-09-17 [source: adr.github.io/madr]; `docs/decisions/README.md`
[live]; `docs/templates/decision.md` [live]. Note: the existing template already matches MADR's
core fields; the one gap is that `Confirmation` is present but the lifecycle states are only four
(`Draft/Accepted/Superseded/Archived`) vs. MADR's five (`proposed/accepted/deprecated/superseded`).
The project's "Archived" conflates deprecated (still in force, just old) with superseded (replaced
by another decision). Recommend adding `deprecated` as a distinct state.

---

### 1.3 Specification

**Purpose.** Defines how an in-scope system must behave. Binding on mechanism once ratified. Not
an implementation claim.

**Fields (current template, `docs/templates/specification.md`).**
- Purpose (charter outcome this makes precise)
- Scope and exclusions (charter citation + Non-goals + Post-1.0 cuts)
- Invariants (stable named anchors, e.g. `SPEC-SUBSYSTEM-001`)
- Model and state (authoritative state, ownership, transitions, persistence)
- Failure behavior (queues, shortage, substitution, going without — explicit)
- Observability (what the Planner can inspect)
- Acceptance evidence (executable checks, mutation proof, player-facing proof)
- Substrate and decisions (fact-sheet anchors, ratified ADRs, gaps)
- Open questions

**RFC 2119 authority marking.** `MUST`/`SHOULD`/`MAY` have force only when the BCP 14
boilerplate is explicitly declared in the document body (RFC 8174 §2) [source: rfc-editor.org,
live-checked 2026-08-24 per `docs/explanation/research/documentation-architecture.md`].

**Lifecycle.** `draft` → `review` → `ratified` → `superseded` (pointing to successor)

A ratified spec binds mechanism from the ratification date. It does not become binding
retrospectively. `doc-reality-auditor` checks ratified specs against code at Phase 6.

**Writes / reads.** Domain advisor (e.g. `kornai-economist`) writes; implementers and gate
agents read.

**Location.** `docs/reference/specifications/<domain>.md`

**Staleness risk.** High — the spec claims what the code does; code drifts constantly. The
`Acceptance evidence` section and the `Last verified` date are the two staleness signals; the
auditor's job is to catch the gap.

**Citation.** `docs/templates/specification.md` [live]; `docs/reference/specifications/README.md`
[live]; RFC 2119 [doc: rfc-editor.org/rfc/rfc2119]; RFC 8174 [doc: rfc-editor.org/rfc/rfc8174].

---

### 1.4 Fact-sheet (substrate map)

**Purpose.** Records what the code ACTUALLY provides for a given seam — verified observations
with file:line citations. Input to specs and briefs; not itself binding on behavior.

**Fields.**
- Seam / domain
- Authority block (Kind: fact-sheet / Authority: observed / Status / Owner / Last verified with commit SHA)
- Summary table: concept | file:line | evidence label (CONFIRMED/OBSERVED/INFERRED)
- Known gaps (what was not found)
- Verification commands (the exact commands that produced the findings)

**Lifecycle.** Fact-sheets are per-wave artifacts. `draft` → `verified` (once the cartographer
confirms sources) → `stale` (once the commit they pin has diverged enough). A stale fact-sheet is
not deleted; it is marked `Status: stale, superseded by <newer wave>`.

**Writes / reads.** `substrate-cartographer` writes; domain advisors and implementers read.

**Location.** `docs/research/fact-sheets/wave<N>-<domain>.md`

**Staleness risk.** Very high — tied to a specific commit. The `Last verified` SHA is the only
reliable signal. Freshness rule: any brief that cites a fact-sheet more than one iteration old
triggers a re-cartograph request.

**Citation.** `docs/research/fact-sheets/wave1-substrate.md` [live]; development-cycle.md §Phase
0 [live].

---

### 1.5 Agent brief (story/task brief)

**Purpose.** Gives a single agent enough context to execute one bounded task without
rediscovery. The agent's entry document. This is the type most urgently missing a template in the
current project.

**Fields (derived from agent-orchestration literature and the project's own failure record).**
- Brief ID (the `bd` issue id, e.g. `sov-hoard-panel-mko`)
- Role (which agent type receives this)
- Acceptance criteria (observable outcomes — the test that proves it done, as exact commands)
- Scope boundary (what is out of scope, to prevent scope creep)
- Traps (known wrong paths; the most important field for agents — inherited from issue description)
- Substrate pointers (file:line for the seams to touch; never a directory)
- Spec and decision pointers (which ratified spec and ADR govern this seam)
- Verification command (exact command to run; agent closes with its output)
- Handoff note (what the next agent needs to know that is not obvious from the code)

**Lifecycle.** `assigned` → `in-progress` → `closed` (with commit SHA and verification output)
Never `done` without evidence; see `bd close --reason`.

**Writes / reads.** Lead writes; the assigned agent reads. Other agents read only the `bd`
tracker summary; the brief is the agent's primary context document.

**Location.** Brief text goes in the `bd` issue description (`-d`); traps go in the description.
Acceptance criteria go in `--acceptance`. Handoff notes go in `bd comments add`. Very long briefs
(>300 lines) may have a companion brief file at `.planning/<session>/<id>-brief.md`.

**Staleness risk.** High if brief lives outside `bd`. The issue description is the single source
of truth; companion files must reference the issue id and must be explicitly marked `provisional`.

**Citation.** CLAUDE.md §Task tracking conventions [live]; `docs/process/development-cycle.md`
[live]; INVEST criteria (Independent, Negotiable, Valuable, Estimable, Small, Testable)
[unverified — widely cited practitioner standard; no primary paper pinned in sources]; Given/When/
Then BDD style [source: cucumber.io/docs, unverified for this session]; EARS syntax — six
patterns (Ubiquitous, Event-driven, State-driven, Unwanted, Optional, Complex) [doc:
alistairmavin.com/ears, wikipedia.org/wiki/Easy_Approach_to_Requirements_Syntax, verified 2026-08-28].

---

### 1.6 Process document (runbook / playbook)

**Purpose.** Repeatable, step-by-step instructions for a human or agent to execute a known
procedure. Procedural, not explanatory.

**Fields (current template, `docs/templates/process.md`).**
- Trigger (when this applies)
- Preconditions
- Steps (numbered, one bounded action each)
- Verification (exact command or observation)
- Failure and recovery
- Owner and related documents

**Lifecycle.** `draft` → `active` → `superseded` | `archived`

**Writes / reads.** Lead or senior agent writes; any agent executing the procedure reads.

**Location.** `docs/process/<name>.md`

**Staleness risk.** Medium-high. Process docs reference tool names (e.g. `br` → `bd`) that
change. Mitigation: every tool name in a process doc is a staleness point; CI grep for retired
tool names is the cheapest check.

**Citation.** `docs/templates/process.md` [live]; `docs/process/development-cycle.md` [live].

---

### 1.7 Research / explanation note

**Purpose.** Records a bounded question, method, findings, and implications. Not binding; feeds
decisions and specs. Maps to Diátaxis "explanation" quadrant.

**Fields (current template, `docs/templates/research.md`).**
- Question
- Method and sources (including revision/commit of local sources inspected)
- Findings (evidence-labelled: CONFIRMED / OBSERVED / INFERRED / SPECULATIVE / OURS)
- Implications (what findings constrain; no binding mechanism here)
- Uncertainties
- Related documents

**Lifecycle.** `draft` → `active` → `superseded` | `archived`

**Writes / reads.** Researcher or domain advisor writes; lead and domain advisors read before
making the related decision.

**Location.** `docs/explanation/research/<slug>.md`

**Staleness risk.** Low once findings are archived — they are historical claims, not current
assertions.

**Citation.** Diátaxis "explanation" [doc: diataxis.fr, verified 2026-08-28]; `docs/templates/research.md`
[live].

---

### 1.8 Generated status document

**Purpose.** Derived output from authoritative inputs. Records what the generator computed;
cannot establish scope, task completion, or mechanism.

**Fields (current template, `docs/templates/generated.md`).**
- Authority block (Kind: generated / Authority: derived)
- Generator: `<command>`
- Authoritative inputs: `<paths>`
- Regeneration command: `<exact command>`
- Generation boundary (what the generator derives and what it cannot establish)
- Output

**Lifecycle.** Always `active` while the generator is live; becomes `stale` when inputs change.
No manual lifecycle management — regeneration is the lifecycle.

**Writes / reads.** Generator scripts write; lead reads as a reporting surface. Agents must not
act on generated status as evidence of implementation; they must read the authoritative inputs.

**Location.** `docs/generated/<name>.md`

**Staleness risk.** Very high without CI enforcement. Current mitigation: the four Python
generators are documented with exact `--check` commands in `RESUME.md` [live].

**Citation.** `docs/templates/generated.md` [live]; Diátaxis "reference" guidance on generated
vs handwritten material [doc: diataxis.fr].

---

### 1.9 Gate / review report

**Purpose.** Records what a gate agent examined, what it found, and its verdict
(approve/approve-with-fixes/send-back). Auditable record for a closed iteration.

**Fields.**
- Gate type (wiring-audit / ledger-invariant / evidence-audit / reviewer)
- Scope (commit range or diff reviewed)
- Ticket/spec reviewed against
- Findings (ranked by severity; each with file:line)
- Verdict and conditions
- Agent and date

**Lifecycle.** Immutable once written. `active` while the iteration is open; `archived` at
Phase 6 wrap.

**Writes / reads.** Gate agent writes; lead reads.

**Location.** `.planning/<session>/gate-<type>-<id>.md`

**Staleness risk.** None — point-in-time record; validity expires with the diff.

**Citation.** `docs/process/development-cycle.md` §Phase 4 [live].

---

### 1.10 Handoff / RESUME

**Purpose.** Lets the next session pick up without re-deriving current state. Describes where
the last session stopped, what is verified, and what the natural next story is.

**Fields.**
- Session end state (commit SHA, test count, what is wired)
- What was left open (issue ids and their state)
- The natural next story (specific `bd` id and why)
- Blockers and known risks
- Reproduction commands (exact commands to verify the described state)

**Lifecycle.** `active` for one session; becomes `archived` when the next handoff supersedes it.
The critical rule: re-derive the queue with `bd ready` rather than trusting the handoff's Next
work section, which goes stale within one session [source: `RESUME.md` line 73, live].

**Writes / reads.** Lead writes at session close; lead reads at session open.

**Location.** `docs/plan/iterations/RESUME.md` (one live file; prior content archived to
`.planning/<session>/HANDOFF-<date>.md`)

**Citation.** `docs/plan/iterations/RESUME.md` [live]; `docs/plan/iterations/HANDOFF-2026-08-27-tooling-wave.md`
[live].

---

## Part 2 — Recommended taxonomy and authority/lifecycle header scheme

### 2.1 The minimal set of doc kinds for this process

| Kind | Authority | Lifecycle states | Hand-written? |
|---|---|---|---|
| `charter` | `binding` | `draft → ratified → amended` | Yes |
| `specification` | `binding` | `draft → review → ratified → superseded` | Yes |
| `decision` (ADR) | `binding` | `proposed → accepted → deprecated \| superseded` | Yes |
| `fact-sheet` | `observed` | `draft → verified → stale` | Yes (by cartographer) |
| `brief` | `operational` | `assigned → in-progress → closed` | Yes (lead) |
| `process` | `operational` | `draft → active → superseded \| archived` | Yes |
| `explanation` | `explanatory` | `draft → active → superseded \| archived` | Yes |
| `generated` | `derived` | `active` (regenerate-only) | No — script output |
| `gate-report` | `advisory` | `active → archived` | Yes (gate agent) |
| `handoff` | `operational` | `active → archived` | Yes (lead) |

Ten kinds. Every document in the repo should match exactly one. If a document cannot be
classified, it is either two documents fused together (split it) or it is transitional session
debris (delete it).

### 2.2 Improved authority/lifecycle header

Current block:
```
**Kind:** <value>
**Authority:** <value>
**Status:** <value>
**Owner:** <value>
**Last verified:** YYYY-MM-DD
```

Recommended additions and changes:

```
**Kind:** charter | specification | decision | fact-sheet | brief | process | explanation | generated | gate-report | handoff
**Authority:** binding | operational | observed | explanatory | derived | advisory
**Status:** <lifecycle state from table 2.1 — no ad-hoc values>
**Owner:** <role, not a person name — so it survives team changes>
**Verified-at:** <commit SHA> <!-- replaces "Last verified" for anything citing code -->
**Last verified:** YYYY-MM-DD  <!-- kept for non-code time-sensitive claims -->
**Supersedes:** <path> <!-- when this doc replaces another -->
**Superseded by:** <path> <!-- when this doc has been replaced; do not delete the doc -->
```

Key changes from current:
1. `Verified-at: <commit SHA>` is added for any document that cites code behavior. A SHA is
   machine-comparable; a date is not.
2. `Status` is locked to the enumerated values in Table 2.1. Ad-hoc values like "superseded
   migration research" or "cutover complete" are not valid status fields — they belong in the body.
3. `Owner` is a role (e.g. `project lead`, `substrate-cartographer`, `kornai-economist`), not a
   person name, so the field retains meaning after roster changes.
4. `Supersedes` and `Superseded by` make the succession chain navigable without grepping.

### 2.3 What a freshness check could look like

**Level 1 — Grep-based (zero infra cost).**
```bash
# Check for retired tool names in active docs
grep -rn '\bbr\b' docs/ --include='*.md' | grep -v archive/
# Check for undefined spec anchors cited in briefs
grep -oh 'SPEC-[A-Z]*-[0-9]*' docs/reference/specifications/*.md | sort -u > /tmp/defined_anchors
grep -roh 'SPEC-[A-Z]*-[0-9]*' docs/ | grep -v archive | cut -d: -f2 | sort -u | comm -23 - /tmp/defined_anchors
```

**Level 2 — SHA-based staleness (one shell function).**
For any document with `Verified-at: <SHA>`, check whether the files it cites have changed since
that SHA:
```bash
git diff --name-only <verified-sha> HEAD -- <cited-file>
```
If the diff is non-empty, the document is stale. This is runnable by `doc-reality-auditor` at
Phase 6 without any additional tooling.

**Level 3 — Executable acceptance criteria (current practice, extend it).**
The spec template already has an `Acceptance evidence` section. Any spec in `ratified` status
should have at least one executable command in that section. `doc-reality-auditor` verifies that
the command exists and, on demand, runs it.

**Level 4 — Link checker in CI (deferred).** The existing documentation-architecture research
note recommends adding CI checks for Markdown linting, local-file links, anchors, and external
links [live: `docs/explanation/research/documentation-architecture.md` §Links and validation].
This remains the right call; it is not yet implemented.

---

## Part 3 — Directory layout and naming rules

### 3.1 Current layout (as of 2026-08-28)

```
docs/
  README.md                    authority map
  SUMMARY.md                   navigation index
  archive/                     historical material (122 files)
  decisions/                   ADRs (empty except README)
  explanation/                 research notes, technical-stack reports
  generated/                   roadmap, evidence
  plan/                        charter, controlled-rewrite plan, iterations/
  process/                     development-cycle, policies, audits
  reference/                   specs, architecture/substrate, art-direction, glossary
  research/                    fact-sheets, awesome-rust note
  templates/                   five templates
.planning/                     session artifacts: plans, agent reports, research HTML
```

The layout is largely correct. Two structural mismatches remain:

1. `docs/research/` and `docs/explanation/research/` are parallel trees. Fact-sheets live in
   `docs/research/fact-sheets/`; research notes live in `docs/explanation/research/`. This split
   is motivated by kind (`fact-sheet` vs `explanation`) but the directory names do not make the
   distinction legible to a cold agent. **Recommendation:** rename `docs/research/` to
   `docs/reference/fact-sheets/` — fact-sheets are reference material (observed substrate state),
   not explanation.

2. `.planning/` mixes session HTML reports (non-Markdown, not indexed anywhere) with Markdown
   reports that agents write. **Recommendation:** add a `.planning/README.md` naming what lives
   there and how to navigate it. Session HTML files are ephemeral and should not be cited by
   other documents.

### 3.2 Naming rules

| Document kind | Pattern | Example |
|---|---|---|
| Charter | `charter-<version>.md` | `charter-1.0.md` |
| Specification | `<domain>.md` (flat, no prefix) | `logistics.md` |
| ADR | `ADR-<4-digit-N>-<kebab-slug>.md` | `ADR-0018-retail-two-leg-model.md` |
| Fact-sheet | `wave<N>-<domain>.md` | `wave3-corpus.md` |
| Process | `<verb-noun>.md` | `development-cycle.md` |
| Explanation | `<topic>.md` (no prefix) | `documentation-architecture.md` |
| Generated | `<derived-artifact>.md` | `roadmap.md` |
| Gate report | `gate-<type>-<bd-id>.md` | `gate-wiring-sov-abc.md` |
| Handoff | `HANDOFF-<YYYY-MM-DD>-<slug>.md` | `HANDOFF-2026-08-27-tooling-wave.md` |
| RESUME | `RESUME.md` (exactly one) | `RESUME.md` |
| Research report (this kind) | `<NN>-<topic>.md` under session dir | `07-framework-and-documentation-patterns.md` |

Numeric prefixes on research reports (`07-`) indicate sequencing within a session. Do not use
numeric prefixes outside session directories — they impose ordering where none is needed and
create renaming churn.

### 3.3 Migration from current layout

| Current location | Issue | Action |
|---|---|---|
| `docs/research/fact-sheets/` | Lives under `research/` not `reference/` | Move to `docs/reference/fact-sheets/`; update all relative links |
| `docs/research/awesome-rust-project-fit.md` | This is an explanation note, not a fact-sheet | Move to `docs/explanation/research/awesome-rust-project-fit.md` |
| `.planning/research-synthesis-2026-08-27.html` | Non-Markdown, not indexed, not cited | Add a `.planning/README.md` entry; do not cite the HTML from any `.md` |
| `docs/decisions/` | Empty except README; no ADRs yet | When the first ADR is ratified, use `ADR-0001-<slug>.md` |
| `docs/plan/proposals/mcp-test-harness.md` | No templates/kind match for "proposal" | Reclassify as `explanation` (design rationale) or convert to an ADR if a decision was made |
| Brief text | Currently only in `bd` issue descriptions | Keep in `bd`; add `.planning/<session>/<id>-brief.md` companion only when >300 lines |

---

## Part 4 — Anti-patterns tied to sources

### AP-1: Documents asserting what the code does (the project's stated failure mode)

**Pattern:** A spec or process doc claims that system X behaves in way Y. The code diverges. The
doc remains binding. Agents act on the doc, not the code.

**Mechanism:** No freshness check exists that ties the document to a code revision. `Last
verified: YYYY-MM-DD` is a date, not a commit; the code may have changed the same day.

**Fix:** Add `Verified-at: <commit SHA>` to every document that cites code behavior. `doc-reality-auditor`
diffs cited files against HEAD at Phase 6. Any document without a SHA in `Verified-at` cannot
claim `Authority: binding` for a code-level claim.

**Source:** This project's own failure record — `docs/process/doc-audit-2026-08-26.md` §1-2 [live:
`br` → `bd` propagated into seven binding documents because no freshness mechanism caught it].
Also: Martraire (2019), *Living Documentation* — "documentation written once and never updated is
misinformation" [unverified: the specific chapter is not confirmed; the principle is widely
attributed to Martraire's DEVOXX talks and the book].

---

### AP-2: Mixing Diátaxis quadrants in one document

**Pattern:** A spec that also tutorializes. A how-to that explains the theory. A charter that
tracks task state.

**Mechanism:** Mixed-purpose documents satisfy no reader fully. An agent reading a reference
document does not need narrative explanation; it needs scannable facts. An agent following a
process does not need architecture rationale inline.

**Fix:** One document, one kind. The ten kinds in Table 2.1 map to Diátaxis quadrants as follows:

| Diátaxis | This project's kinds |
|---|---|
| Reference | `specification`, `fact-sheet`, `charter`, `decision` |
| How-to / Runbook | `process`, `brief` |
| Explanation | `explanation`, `gate-report`, `handoff` |
| Tutorial | none — agents learn by doing; tutorial docs are for human onboarding |
| Generated | `generated` (cross-cutting, not a Diátaxis type) |

**Source:** Diátaxis — "the number one mistake is mixing the types on the same page"
[doc: diataxis.fr, verified 2026-08-28].

---

### AP-3: Long prose where agents need tables

**Pattern:** A 200-line narrative specification when a 20-row table with CONFIRMED/OBSERVED labels
would answer the agent's question in two reads.

**Mechanism:** Agent performance degrades as context length increases; effective context is the
length over which quality holds, and "performance drops 39% on average from single-turn to
multi-turn interaction" for agents working over long documents [source: Microsoft Research /
Salesforce study cited in datanorth.ai/blog/context-length, 2026; exact study unverified for
this session]. LLM agents scan tables faster than prose; an indexed table is recoverable if the
agent loses context mid-read. Anthropic's own CLAUDE.md guidance states: "if your CLAUDE.md is
too long, Claude ignores half of it because important rules get lost in the noise" — ruthlessly
prune [live: Anthropic engineering best practices, code.claude.com, confirmed by fork research
2026-08-28].

**Fix:** For reference documents, lead with a table. For process documents, use numbered steps.
Reserve prose for explanation documents. Every spec's Invariants section is already a list of
named anchors — extend that habit to every reference section. Apply progressive disclosure: an
index entry names what a document covers; full content loads on demand; source files load only
when the agent must verify code behavior. Measured production effect of demand-loading: 41-80%
cost reduction, 13-31% latency improvement [source: arXiv 2607.17598 and practitioner reports
cited in fork research; exact values unverified against primary paper for this session].

**Source:** Practitioner guidance from Agent-Friendly Docs (dacharycarey.com, 2026) [doc,
verified 2026-08-28]: "short, dense files beat long comprehensive ones"; tables and lists outperform
prose for machine readers. Martraire (2019), *Living Documentation* — "documentation written once
and never updated is misinformation" [source: book and DEVOXX talks; specific chapter unverified].

---

### AP-4: DRY violations across agent-facing documents

**Pattern:** The same fact — a tool name, a file path, a constraint — appears in CLAUDE.md,
AGENTS.md, development-cycle.md, and three specs. When the fact changes, it changes in some
places and not others.

**Mechanism:** The `br` → `bd` migration [live: `docs/process/doc-audit-2026-08-26.md`] showed
exactly this: one tool rename required seven edits in binding documents. The audit caught six of
seven; one more could easily be missed.

**Fix:**
1. Each stable fact lives in exactly one authoritative document. Other documents cite it with a
   path, not a repetition.
2. For facts that MUST appear in multiple documents (e.g. the tracker tool name in CLAUDE.md and
   the charter), a CI grep is the freshness check: `grep -rn '\bbr\b' docs/ --include='*.md' |
   grep -v archive/`.
3. AGENTS.md and CLAUDE.md are already the designated cross-tool single sources for agent entry
   conventions [source: AGENTS.md convention guide, morphllm.com/agents-md-guide, 2026]; other
   documents must cite them, not repeat their content.

**Source:** "One fact once" principle; software DRY applied to documentation.

---

### AP-5: Briefs without file:line pointers

**Pattern:** A brief tells an agent to "update the economy module." The agent spends 40% of its
budget searching for the right file.

**Mechanism:** Agents have no LSP tool in subagent context [CLAUDE.md: "Subagents have no LSP
tool. Resolve symbols in the main session and paste `file:line` into briefs."]. A cold language
server answers `findReferences` with "No references found" [live: `docs/reference/code-intelligence.md`].

**Fix:** Every brief must include the substrate pointers field with file:line resolved by the
lead before dispatch. The cartographer's fact-sheet is the source; the brief pastes from it.

**Source:** CLAUDE.md §Subagents [live]; `docs/reference/code-intelligence.md` §cold server trap
[live].

---

### AP-6: Treating generated status as authoritative

**Pattern:** An agent reads `docs/generated/roadmap.md`, sees "21 requirements, 107 targets, 0
implemented", and concludes that the implementation status is known. It is not — the roadmap
derives from evidence targets, not from code inspection.

**Mechanism:** Generated documents synthesize their inputs; they cannot observe the code. The
generation boundary is not visible to an agent reading only the output.

**Fix:** The `generated` template already includes a Generation boundary section. Every generated
document must state explicitly: "This document cannot establish implementation status." Gate
agents must read spec invariants and run the verification commands, not read the generated
roadmap.

**Source:** `docs/templates/generated.md` §Generation boundary [live]; `docs/plan/iterations/RESUME.md`
line 36 ("Requirements derive scope... schema validity is not target proof") [live].

---

### AP-7: Status values that are not lifecycle states

**Pattern:** A document has `Status: superseded migration research` or `Status: cutover complete`.
These are prose annotations, not lifecycle states.

**Mechanism:** An agent reading the Status field expects to determine: can I act on this? A
non-enumerated value forces the agent to parse prose, which is error-prone under prompt pressure.

**Fix:** Lock the Status field to the enumerated values in Table 2.1. All explanatory context
goes in the document body under a "Note" or "History" heading.

**Source:** `docs/process/doc-survey-2026-08-26.md` — the survey found multiple partial headers
and ad-hoc status values [live]; MADR 4.0 uses an explicit enumeration of five states [source:
adr.github.io/madr, verified 2026-08-28].

---

## Gaps and what would close them

1. **No ADRs yet in `docs/decisions/`.** The directory is empty except README. The retail two-leg
   model, the `bd` tool choice, and the opus-uniform tier decision are all significant enough to
   warrant ADRs. Closing this gap: the lead writes ADR-0001 through ADR-0003 at the next
   iteration boundary, using the existing template.

2. **No brief template.** The `docs/templates/` directory has five templates; a `brief.md`
   template is absent. Closing this gap: add `docs/templates/brief.md` using the fields in §1.5.

3. **`Verified-at: <SHA>` field is not yet in any template.** Closing this gap: add it to
   specification, fact-sheet, and decision templates.

4. **No CI link checker.** `docs/explanation/research/documentation-architecture.md` recommended
   this in 2026-08-24 and it remains unimplemented [live]. Closing this gap: a `markdownlint` +
   `lychee` or `markdown-link-check` GitHub Actions step.

5. **`docs/research/` vs `docs/explanation/research/` split.** Not yet resolved. Closing this
   gap: the migration action in §3.3.

6. **EARS syntax for acceptance criteria.** The spec template has an `Acceptance evidence`
   section but uses free prose. EARS patterns (When/While/If-then/Ubiquitous) would make
   acceptance criteria machine-parseable and mutation-testable [source: alistairmavin.com/ears,
   wikipedia.org/wiki/Easy_Approach_to_Requirements_Syntax, verified 2026-08-28]. Closing this
   gap: update the spec template's `Acceptance evidence` section with a EARS pattern table.

---

*Sources used: diataxis.fr [live]; adr.github.io/madr [live]; rfc-editor.org/rfc/rfc2119 and
rfc8174 [verified 2026-08-24 per project docs]; alistairmavin.com/ears [live]; morphllm.com/
agents-md-guide [live]; dacharycarey.com/2026/02/18/agent-friendly-docs [live]; datanorth.ai/blog/
context-length [live]; wikipedia.org/wiki/Easy_Approach_to_Requirements_Syntax [live]; all
project `.md` files cited as [live] were read directly from the repo at HEAD, 2026-08-28.*
