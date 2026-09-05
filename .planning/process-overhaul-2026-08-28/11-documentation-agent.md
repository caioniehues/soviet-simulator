# Documentation agent design

**Kind:** research report
**Authority:** advisory — findings for the lead to disposition
**Status:** draft
**Owner:** researcher
**Last verified:** 2026-08-28

---

## Answer (one paragraph)

Design one agent — `doc-agent` — that owns all three surfaces: rustdoc code comments, the
requirements-traceability chain, and the browsable project wiki. Splitting it into two agents
would violate gate independence only if both ran on the same story diff; in practice the three
surfaces have disjoint file ownership and the agent's method per surface is serial, so one agent
with a clear refusal boundary is cheaper and less ambiguous than two with a coordination seam.
The agent runs at Plan Review (same slot as `drift-auditor`, after it), and on demand when a new
spec, fact-sheet, or ADR is written. It does not touch code, does not set mechanisms, and does
not duplicate `drift-auditor`'s staleness sweep or `substrate-cartographer`'s fact-sheets; it
consumes both as inputs. Its three freshness metrics — doc coverage %, uncovered REQ-* IDs,
stale wiki pages (SHA-diff count) — are the numbers it reports each Plan.

---

## 1. Rust code-docs tooling

### 1.1 `cargo doc`

`cargo doc -p simulation --no-deps` builds docs for the simulation crate in 1.77 s. [live,
run 2026-08-28]

```
cargo 1.97.1 (c980f4866 2026-06-30)
Documenting simulation v0.1.0
Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.77s
Generated /home/caio/soviet-simulator/target/doc/simulation/index.html
```

No `[workspace.lints]` section exists in `Cargo.toml`. No `missing_docs` lint is declared in
any crate's `lib.rs`. [live, grep 2026-08-28]

### 1.2 Doc coverage (nightly)

Nightly toolchain `nightly-x86_64-unknown-linux-gnu` is installed. [live]

`cargo +nightly rustdoc -p simulation -- -Z unstable-options --show-coverage` produces a
per-file table. Summary row: [live, run 2026-08-28]

```
| Total  |  61  |  7.4%  |  0  |  0.0%  |
```

61 of ~825 documentable items carry a doc comment. Coverage is 7.4 %. Notable gaps:

| File | Documented | % |
|---|---|---|
| `simulation/src/economy/market.rs` | 5 | 14.7 % |
| `simulation/src/economy/ecostats.rs` | 2 | 16.7 % |
| `simulation/src/economy/government.rs` | 1 | 50.0 % |
| `simulation/src/lib.rs` | 0 | 0.0 % |
| `simulation/src/init.rs` | 0 | 0.0 % |
| `simulation/src/world.rs` | 3 | 4.2 % |
| `simulation/src/world_command.rs` | 1 | 1.4 % |
| `simulation/src/map_dynamic/dispatch.rs` | 4 | 33.3 % |

Command to reproduce: `cargo +nightly rustdoc -p simulation -- -Z unstable-options --show-coverage`
Output is written to `target/doc/simulation.txt`. [live]

### 1.3 `#![warn/deny(missing_docs)]`

The lint is stable. Enabling `#![warn(missing_docs)]` in `simulation/src/lib.rs` makes the
compiler warn on every undocumented public item. Upgrading to `#![deny(missing_docs)]` turns
warnings to errors. Neither is currently declared. Adding a graduated approach — `warn` first,
`deny` per-crate after coverage reaches a threshold — is the standard incremental gate. [source:
rustdoc book, doc.rust-lang.org/rustdoc/lints.html; verified via `rustdoc --help` output, live]

The `rustdoc::broken_intra_doc_links` lint catches broken `[TypeName]` cross-references. It is
stable since Rust 1.52 (2021). Enable with `#![warn(rustdoc::broken_intra_doc_links)]` or via
workspace lints (`[workspace.lints.rust]`). [source: doc.rust-lang.org/rustdoc/lints.html;
**unverified** against the exact lint name on cargo 1.97.1 — confirm with `RUSTDOCFLAGS="-W
rustdoc::broken-intra-doc-links" cargo doc -p simulation --no-deps 2>&1 | head -5`]

### 1.4 Workspace-level lint configuration

Cargo 1.97.1 supports `[workspace.lints]`. Adding:

```toml
[workspace.lints.rust]
missing_docs = "warn"

[workspace.lints.rustdoc]
broken_intra_doc_links = "warn"
```

to `Cargo.toml` propagates the lint to every workspace member that adds `[lints] workspace =
true` to its own `Cargo.toml`. This is the lowest-friction enforcement path. [source:
doc.rust-lang.org/cargo/reference/workspaces.html#the-lints-table; **unverified** whether every
crate in this workspace already opts into workspace lints — check with `grep -r "workspace =
true" */Cargo.toml`]

### 1.5 Tools on crates.io (release status verified via `cargo search`, 2026-08-28)

| Tool | Latest version | Installed? | Purpose | Verdict |
|---|---|---|---|---|
| `cargo-deadlinks` | 0.8.1 | No | Check HTML doc output for broken links | Use after `cargo doc`; installable |
| `cargo-spellcheck` | 0.15.7 | No | Spell-check doc comments | Installable; requires hunspell dict |
| `cargo-rdme` | 2.2.2 | No | Sync `README.md` from crate-level doc comment | Useful for public crates; low priority here |
| `cargo-readme` | 3.4.0 | No | Generate README from doc comments | Similar to rdme; low priority |
| `mdbook` | — | **Yes** (0.5.4) | Build in-repo wiki from Markdown | Already installed |

[live, `mdbook --version` output 2026-08-28; `cargo search` output 2026-08-28]

### 1.6 Doc coverage gate (per-crate)

A practical gate: `cargo +nightly rustdoc -p simulation -- -Z unstable-options --show-coverage
2>&1 | awk '/\| Total/ { gsub(/%/,"",$7); if ($7+0 < THRESHOLD) exit 1 }'`. Set THRESHOLD=10
for Plan 01, raise by 5 each Plan until reaching 30. Starting at 7.4 % actual, a threshold of 10
is achievable in one Plan with targeted module-doc additions. [live — derived from coverage
output; threshold values are recommendations, unverified against team appetite]

Doctests as executable docs: the simulation crate has 0 doctests today (`Percentage: 0.0%` for
Examples column). A doctest in `market.rs` illustrating `Dispatch::new` would serve as both
specification and regression. [live — coverage output]

---

## 2. Wiki tooling

### 2.1 `mdbook` (already installed)

`mdbook 0.5.4` is installed. [live] An in-repo `docs/wiki/` directory with a `book.toml` and
`src/SUMMARY.md` builds to HTML with `mdbook build`. The standard plugins:

- `mdbook-linkcheck` — checks internal and external links in the built output. Installable via
  `cargo install mdbook-linkcheck`. [source: github.com/Michael-F-Bryan/mdbook-linkcheck;
  **unverified** that a release exists compatible with mdbook 0.5.x — check before installing]
- `mdbook-mermaid` — renders mermaid diagram fences inside Markdown. [source:
  github.com/badboy/mdbook-mermaid; **unverified** compatibility with 0.5.4]

An mdBook wiki lives at a path and is built to `docs/wiki/book/` (gitignored build output).
Pages are hand-maintained Markdown files. This is the right fit for this repo: the docs are
already Markdown, the tooling is installed, and `mdbook build` is a one-command CI step.

Diátaxis-shaped wiki means: four section types (Tutorials, How-to, Reference, Explanation)
with distinct tones and structures. For this repo, Tutorials are out (agents learn by doing);
Explanation and Reference dominate.

### 2.2 code-review-graph wiki

The `.code-review-graph/wiki/` directory already contains **201 pages** generated by
`generate_wiki_tool`. [live, `ls | wc -l`, 2026-08-28]. Sample page inspected (read-only via
`get_wiki_page_tool`):

- `economy-capital`: 38-node community centered on `Market`/`SingleMarket` in
  `simulation/src/economy/market.rs`. Lists members with file:line, execution flows
  (`advance_dispatches`, `routing_update_system`, `settle_retail`), incoming/outgoing
  dependency edges. [live, `get_wiki_page_tool` result]
- `economy-dispatch`: 2-node community (`Dispatch`, `truck`). Small and precise. [live]

**What the graph wiki provides:** structural (community-based) pages with member lists,
flow criticality scores, and dependency edge counts. It does NOT provide: domain explanation,
how-things-work narrative, glossary, ADR cross-references, or freshness dates. It is a code
index, not an architecture guide.

**`generate_wiki_tool` is a write operation** (writes to `.code-review-graph/wiki/`). It must
not be run without the Planner's authority. [source: tool schema — `"Pages are written to
.code-review-graph/wiki/ inside the repository"`; live, schema loaded 2026-08-28]

The graph wiki regenerates from the code graph (built at commit `4e9e930b2a73`, HEAD matches).
It is self-refreshing as long as the graph hook fires on file changes. [live, wiki page metadata]

### 2.3 Arc42 / C4 as skeleton

Arc42 provides 12 sections; C4 provides 4 levels (Context, Container, Component, Code). Both
are frameworks for architecture documentation. For this repo, the relevant arc42 sections are:

- §1 Introduction and Goals (maps to charter)
- §3 System Scope and Context (maps to substrate.md)
- §5 Building Block View (maps to graph wiki communities)
- §6 Runtime View (maps to scenario tests and execution flows)
- §9 Architecture Decisions (maps to `docs/decisions/`)
- §11 Technical Risks (maps to gate reports and retro findings)

Using arc42 section numbering as the wiki skeleton gives a cold agent a predictable navigation
path. [source: arc42.org/overview; **unverified** against the current arc42.org page — read
before adopting the section numbers, as they may have changed]

### 2.4 ADR index page

An `docs/decisions/README.md` exists but is empty. A populated ADR index with one row per ADR
(number, title, status, date, what it decided) is the cheapest wiki mechanism that a cold agent
can use to avoid re-litigating settled questions. The existing template is `docs/templates/
decision.md`. Zero ADRs are ratified today. [live, `ls docs/decisions/`]

### 2.5 Freshness mechanisms (what actually works in comparable repos)

Report 07 §2.3 documents three levels of freshness checking [live, report 07]:

1. **`Verified-at: <SHA>` + `git diff --name-only <sha> HEAD -- <file>`** — a script can
   check every document that carries this field. Already adopted in GOSPLAN §5.1.
2. **CI link check** — `mdbook-linkcheck` or `lychee` in a GitHub Actions step. Not yet
   implemented.
3. **Grep-based retired-tool-name scan** — already the basis of `doc-check.sh` in GOSPLAN §3.7.

The `doc-reality-auditor` (to be renamed `drift-auditor`) already runs `doc-check.sh` at Plan
Review. The documentation agent's wiki freshness pass is distinct: it checks whether wiki page
content is consistent with the `Verified-at` SHA of the fact-sheets it draws from, not just
whether tool names have changed.

---

## 3. Requirements traceability

### 3.1 Existing chain

The repo already has a four-script Python chain: [live, scripts read 2026-08-28]

| Script | Input | Output | Check command |
|---|---|---|---|
| `build_requirements.py` | Charter + SPEC anchors (hardcoded catalogue) | `requirements/*.md` + `story-migration.md` | `--check` flag |
| `build_extract.py` | `requirements/*.md` | `extract/requirements.json` | **unverified** — file exists |
| `build_evidence.py` | `extract/requirements.json` + `docs/reference/specifications/*.md` + `evid-spec-bindings.json` | `generated/evidence/target-scenarios.json` | **unverified** — file exists |
| `build_roadmap.py` | `requirements/*.md` + `extract/requirements.json` + `generated/evidence/target-scenarios.json` | `generated/roadmap.md` | `--check` flag |

The chain enforces: every `REQ-*` has `SPEC-*` anchors (in the catalogue), every `SPEC-*` cited
in `evid-spec-bindings.json` must resolve in the specs directory, every `REQ-*` must have at
least one `EVID-*` target, and every target must be either `IMPLEMENTED` or `UNIMPLEMENTED`. The
`build_roadmap.py` fails with `"requirements without evidence"` if any `REQ-*` lacks a bound
target. [live, `build_roadmap.py` source read]

**The chain does not check:** whether test code contains a function matching the `EVID-*`
scenario ID, whether a `SPEC-*` anchor referenced in the catalogue actually exists in the spec
file as a named heading, or whether the evidence status reflects the current code state (it is
a manual declaration).

### 3.2 EARS syntax

EARS (Easy Approach to Requirements Syntax) provides six sentence patterns: Ubiquitous
(`The system shall…`), Event-driven (`When X, the system shall Y`), State-driven (`While X,
the system shall Y`), Unwanted (`If X, then the system shall Y`), Optional (`Where X is
included, the system shall Y`), Complex (combination). [source: alistairmavin.com/ears; verified
2026-08-28 per report 07]. The `build_requirements.py` catalogue already uses EARS-shaped
criteria strings (e.g. `"Only received physical material and recorded work make a Site ground
broken or complete"`). [live, script source]

### 3.3 External traceability tools

| Tool | Markdown-native? | Lightweight enough? | Assessment |
|---|---|---|---|
| `doorstop` | Partial (YAML+Markdown) | Yes | File-based; Python; last release 2022. **Unverified** active maintenance. Would conflict with the existing `REQ-*`/`EVID-*` scheme by imposing its own ID system. |
| `StrictDoc` | Yes (RST+SDFormat native; Markdown import beta) | Yes | Python; actively maintained 2024. Outputs HTML traceability matrix. Its SDFormat is a DSL; importing from Markdown requires custom parser. **Not a drop-in** over the existing chain. |
| `OpenFastTrace` | No (Java; reads structured markup in code/docs) | No | Java dependency; overkill for this repo. |
| `reqflow` | No (C; reads structured text) | No | Too low-level for LLM-agent use. |
| `sphinx-needs` | No (RST/Sphinx) | No | RST ecosystem; not compatible. |

**Verdict:** None of these tools extend the existing chain without replacing it. The existing
Python scripts are lightweight, repo-local, already verified to run, and produce machine-readable
JSON output. The right extension is to add a fifth script, `check_traceability.py`, that closes
the three gaps in §3.1. [unverified — this script does not yet exist; this is a recommendation]

### 3.4 What a complete traceability check would add

```
check_traceability.py
  1. For each REQ-* in extract/requirements.json:
     a. Verify every SPEC-* anchor cited in the catalogue is a ## heading in the spec file.
     b. Verify every EVID-* target scenario in target-scenarios.json has a matching test
        function name in tests/scenarios/**/*.rs.
  2. Report: N REQ-* with unresolved SPEC-* anchors; M EVID-* with no test function found.
```

This closes the EVID→test gap. The SPEC-anchor check is a grep; the test-name check is also a
grep. No new dependencies needed. [recommendation, unverified until implemented]

---

## 4. Existing "documentation agent" patterns in the wild

Searched GitHub repos cited in report 03 and the GOSPLAN evidence base.

**wshobson/agents `docs-*`:** No `docs-` agent found by that specific path; report 03 did not
cite one. The closest pattern found in comparable repos (report 03 §tier-1) is harness
(revfactory) which has a "technical-writer" role that writes briefs, not documentation.
**Unverified** — GitHub search not run this session due to token budget; based on report 03
which ran `gh api` searches.

**VoltAgent/awesome-claude-code-subagents:** The pattern from comparable repos (report 03) is
that 80 % use file-based artifact handoff with a human-reviewed plan before code. Documentation
agents specifically are rare; most repos use a "docs writer" role inline in a coding agent.
**Unverified** against the current awesome-claude-code-subagents list.

**Anthropic guidance on agent-facing docs:** From GOSPLAN §4.1 and report 02 §7.2: descriptions
must be "pushy" — trigger situations + near-miss exclusions + re-trigger keywords. Agent body
≤ 150 lines. Tables over prose. Progressive disclosure: index entry → page → source.
[source: GOSPLAN §4.1, live; report 02 §7.2, live]

**Key method lines extracted from comparable-repo patterns (report 03 tier-1):**
- harness: 500-line cap with `references/` overflow; CLAUDE.md changelog table; Phase-0 drift
  audit of the doc agent's own outputs.
- Wirasm/PRP pattern: the brief contains the verification command + expected output shape. For
  a doc agent, the verification command is `cargo +nightly rustdoc --show-coverage` + threshold,
  not a test.
- disler hooks: enforcement via `PreToolUse`/`PostToolUse` for completion. The doc agent's
  natural hook is `PostToolUse` on `Edit` to any `.md` under `docs/reference/specifications/`
  — trigger a `check_traceability.py` run.

---

## 5. One agent or two?

**Arguments for one agent:**
- File ownership is cleanest with one owner. The three surfaces (`simulation/src/**/*.rs` doc
  comments, `docs/plan/iterations/**/*.{py,md,json}` traceability chain, `docs/wiki/**/*.md`
  wiki pages) never overlap — an agent that owns all three never contends with itself.
- A code-docs pass and a wiki pass share context: the doc-coverage output tells the agent which
  modules need architectural explanation in the wiki; the wiki pages tell the agent where doc
  comments need cross-references.
- Gate independence requires only that the documentation agent never reviews its own output in
  the same Plan. That is trivially true: its output is docs, not code; the code gates
  (`wiring-auditor`, `ledger-invariant-checker`) do not touch docs.

**Arguments for two agents:**
- The rustdoc pass requires Rust toolchain knowledge; the wiki pass requires domain narrative
  knowledge. These are different competencies.
- If the agent's context fills with coverage tables, it has less room for wiki page composition.

**Verdict: one agent.** The competency gap is smaller than it appears: rustdoc coverage is a
script output the agent reads and acts on with targeted `///` comments; it does not require
Rust compilation knowledge. The shared context benefit (coverage gaps → wiki explanation gaps)
is real and measurable. The two passes run sequentially in one invocation, separated by a
report section boundary. The agent refuses to touch code logic, specs, or fact-sheets.

---

## 6. Agent design

### 6.1 Name and description

```
Name: doc-agent
Tier: sonnet (doc coverage is a script + targeted comment additions; wiki is prose generation
      from existing content; neither requires adversarial reasoning)
```

**Description (pushy):**

> Owns three doc surfaces for this repo: (1) rustdoc coverage — reads `--show-coverage` output,
> adds `///` comments to public items in `simulation/` and `common/`, and raises the per-crate
> threshold by 5 pp each Plan; (2) requirements traceability — runs `check_traceability.py` and
> `build_roadmap.py --check`, reports uncovered `REQ-*` IDs and `EVID-*` with no test match;
> (3) project wiki — updates `docs/wiki/` pages that carry a stale `Verified-at` SHA (detected
> by `doc-check.sh`). Trigger when: a new spec, fact-sheet, or ADR is written; at Plan Review
> after `drift-auditor`; or when `doc-check.sh` reports STALE pages. Do NOT trigger for code
> reviews, gate decisions, architecture rulings, or any change to `docs/reference/specifications/`
> (that is advisor territory). Near-miss: a request to "update the docs" after a code change
> belongs to the code-writing agent, not here — this agent documents the stable state, not the
> in-flight change.

### 6.2 What it owns

| Surface | Files owned | Files it reads but does not write |
|---|---|---|
| Code docs | `simulation/src/**/*.rs` (doc comments only), `common/src/**/*.rs` (doc comments only) | `target/doc/simulation.txt` (coverage output) |
| Traceability | `docs/generated/roadmap.md`, `docs/plan/traceability/story-migration.md` | `docs/plan/iterations/**/*.{py,json,md}`, `docs/reference/specifications/*.md`, `tests/scenarios/**/*.rs` |
| Wiki | `docs/wiki/**/*.md` | `docs/reference/fact-sheets/*.md`, `docs/decisions/*.md`, `docs/reference/substrate.md`, graph wiki pages (read via `get_wiki_page_tool`) |

**Refuses:** touching `docs/reference/specifications/` (advisor territory); changing test code;
writing fact-sheets (cartographer territory); setting any mechanism or making any architecture
ruling; writing briefs or gate reports.

### 6.3 Method per surface

**Surface 1 — rustdoc coverage**

```
1. cargo +nightly rustdoc -p simulation -- -Z unstable-options --show-coverage 2>&1 > /tmp/cov.txt
2. Parse /tmp/cov.txt: extract files with 0 % and files with public API at < 20 %.
3. For each file below threshold: read the public items (structs, traits, functions, enums).
   Add /// doc comments to all undocumented public items. One sentence minimum; link to the
   relevant SPEC-* anchor if one governs the item.
4. Re-run coverage; verify Total % has increased.
5. Report: before/after Total %; N items added; which files still below threshold.
```

Lint gate: after adding comments, `RUSTDOCFLAGS="-W rustdoc::broken-intra-doc-links" cargo doc
-p simulation --no-deps 2>&1 | grep "^warning:"` must be empty for the new comments.

**Surface 2 — requirements traceability**

```
1. python3 docs/plan/iterations/requirements/build_requirements.py --check
2. python3 docs/plan/iterations/build_roadmap.py --requirements-dir ... --extract ... --evidence ... --output ... --check
3. Run check_traceability.py (when it exists): report N REQ-* with unresolved SPEC-* anchors,
   M EVID-* with no matching test function.
4. For each EVID-* with no test match: add a bd comment to the relevant open issue flagging the
   gap (do not write the test; that is evidence-auditor territory).
5. Report: chain integrity (pass/fail per script); gap table.
```

**Surface 3 — project wiki**

```
1. Run doc-check.sh: collect STALE pages (those with Verified-at SHA where git diff is non-empty).
2. For each STALE page: read the current fact-sheet(s) it draws from (substrate-cartographer
   output). Update the wiki page to reflect the current state. Update Verified-at to HEAD.
3. For any new spec or ADR without a wiki page: create a stub in docs/wiki/ that links to the
   authoritative document and summarizes the decision in ≤ 3 sentences.
4. Run mdbook build; verify exit 0.
5. Report: N pages refreshed; N stubs added; N pages still stale (with reason).
```

For the graph wiki: read the relevant community pages via `get_wiki_page_tool` to extract
current execution flows and member lists; use them to populate the wiki's "how it works" section
for that module. Do not duplicate the graph wiki — link to it as a structural index.

### 6.4 GOSPLAN cycle placement

| When | What | Why |
|---|---|---|
| Plan Review, after `drift-auditor` | Full three-surface pass | drift-auditor identifies STALE pages; doc-agent acts on the list |
| On new spec/ADR ratification | Surface 3 only (wiki stub + spec page refresh) | Keeps wiki current with decisions as they happen |
| On `check_traceability.py` failure in CI | Surface 2 only (gap report, no auto-fix) | Fast feedback on broken traceability without a full doc pass |
| On explicit Planner request | Any surface | Exploratory or catch-up pass |

The agent does NOT run per-story. Running it per-story would generate noise and compete with
the code gates for context. One pass per Plan is the right cadence; a new spec ratification
triggers a targeted partial pass.

**Does NOT run** in: Ground (cartographer's lane), Gate (gate agents' lane), Build (builders'
lane).

### 6.5 Coordination

| Other agent | Relationship | How |
|---|---|---|
| `drift-auditor` | Upstream: drift-auditor produces STALE page list; doc-agent acts on it | drift-auditor's report is a file (`docs/generated/drift-report.md` or equivalent); doc-agent reads it |
| `substrate-cartographer` | Upstream: fact-sheets are doc-agent's source of truth for wiki accuracy | doc-agent reads fact-sheets; never writes them; cites their `Verified-at` SHA in updated wiki pages |
| `evidence-auditor` | Peer: evidence-auditor owns test coverage; doc-agent flags EVID-* gaps but does not write tests | doc-agent files a bd comment; evidence-auditor decides disposition |
| `wiring-auditor` | Downstream: wiring-auditor checks that code is reachable; doc-agent adds doc comments that may reference APIs — doc-agent must not assert an API is wired if it is not | doc-agent uses `#[cfg_attr(…)]` and hedged language in doc comments for speculative paths |
| `kornai-economist` et al. | Advisors: doc-agent may update wiki pages about economy; the economist is the authority on correctness | doc-agent flags proposed economy wiki edits in its report; economist can veto via bd comment |

### 6.6 Report format

```
## doc-agent report — Plan NN, YYYY-MM-DD

### Surface 1: rustdoc coverage
- simulation/: 7.4 % → X % (+N items documented)
- Files still below 10 % threshold: [list]
- Broken intra-doc links: [0 | list]

### Surface 2: traceability chain
- build_requirements.py --check: PASS | FAIL (reason)
- build_roadmap.py --check: PASS | FAIL (reason)
- check_traceability.py: N REQ-* with unresolved SPEC-* anchors; M EVID-* with no test match
  - [table of gaps]

### Surface 3: wiki
- N pages refreshed (Verified-at updated)
- N stubs added: [list with links to authoritative docs]
- N pages still stale (reason: [no current fact-sheet | advisor territory | out of scope])

### What the next doc-agent needs
- [unclosed gaps, pending advisor review, deferred items]
```

### 6.7 Freshness/coverage numbers reported each Plan

| Metric | Source command | Target |
|---|---|---|
| `simulation/` doc coverage % | `cargo +nightly rustdoc --show-coverage` | +5 pp per Plan from 7.4 % baseline |
| Uncovered REQ-* IDs | `build_roadmap.py --check` (fails if any uncovered) | 0 |
| EVID-* with no test match | `check_traceability.py` | 0 |
| Stale wiki pages | `doc-check.sh` STALE count | 0 at end of pass |
| mdbook build exit code | `mdbook build docs/wiki` | 0 |

---

## 7. Proposed wiki structure for this repo

Seeded from what exists in `docs/` (read 2026-08-28). The wiki is distinct from `docs/` — it
is the browsable, narrative layer. `docs/` holds authoritative documents; the wiki synthesises
them for a cold reader.

```
docs/wiki/
  book.toml
  src/
    SUMMARY.md
    introduction.md          — what the game is; the two pillars; "nothing teleports, never game over"
    architecture/
      overview.md            — arc42 §1+3: context, C4 System diagram, crate map
      substrate.md           — synthesised from docs/reference/architecture/substrate.md
      economy.md             — Kornai model; dishonest enterprise; queue clearing
      transport.md           — logistics model; truck dispatch; retail two-leg model
      settlement.md          — citizens, households, needs
      utilities.md           — electricity (union-find → wire); water; heating
    how-it-works/
      dispatch-cycle.md      — what happens when a factory requests goods; truck lifecycle
      retail-flow.md         — factory → store → shopper; the two-leg model
      dishonest-enterprise.md — the detection mechanic; observable state the Planner reads
      map-generation.md      — procgen pipeline; terrain → roads → lots → buildings
    decisions/
      index.md               — ADR index table (one row per ADR: number, title, status, date, what)
    glossary.md              — one definition per term; links to canonical source
    reference/
      crate-map.md           — which crate owns which concern; who calls whom (from graph wiki)
      spec-index.md          — SPEC-* anchors, which spec they live in, their binding status
      req-index.md           — REQ-* IDs, their EVID-* targets, IMPLEMENTED/UNIMPLEMENTED status
```

Pages that already have source material to draw from (confirmed 2026-08-28):
- `architecture/economy.md` ← `docs/reference/specifications/logistics.md`, `economy.md`,
  `docs/explanation/research/`, graph wiki `economy-capital` page
- `architecture/substrate.md` ← `docs/reference/architecture/substrate.md`
- `how-it-works/dispatch-cycle.md` ← `simulation/src/economy/market.rs` (graph wiki:
  `economy-capital`, execution flow `advance_dispatches`), `docs/reference/specifications/logistics.md`
- `decisions/index.md` ← `docs/decisions/` (currently empty; stubs are added as ADRs are ratified)
- `glossary.md` ← `docs/reference/glossary.md` [live, file confirmed]
- `reference/req-index.md` ← `docs/generated/roadmap.md` (generated; doc-agent regenerates before
  copying counts)

Pages that need fact-sheet or advisor input before writing (deferred):
- `how-it-works/retail-flow.md` ← `retail-two-leg-model.md` ADR (ratified 2026-08-26 per memory);
  needs logistics-modeller review
- `architecture/utilities.md` ← utilities-modeller review

---

## 8. Gaps and what would close them

| Gap | Closer |
|---|---|
| No `check_traceability.py` script | Write it (≤ 60 lines; two greps — SPEC-* in spec files, test function names in scenarios); run in CI |
| No `#![warn(missing_docs)]` in any crate | Add to `[workspace.lints.rust]` in `Cargo.toml`; each crate opts in with `workspace = true` |
| No `docs/wiki/` directory or `book.toml` | Create skeleton (one Wave-1 task, S lane) |
| `rustdoc::broken_intra_doc_links` lint name not confirmed on cargo 1.97.1 stable | Run `RUSTDOCFLAGS="-W rustdoc::broken-intra-doc-links" cargo doc -p simulation --no-deps` |
| mdbook-linkcheck compatibility with mdbook 0.5.4 unverified | `cargo install mdbook-linkcheck && mdbook-linkcheck docs/wiki` |
| `doc-check.sh` exists in GOSPLAN §3.7 as proposed but not yet implemented | Wave-1 scripts task |
| `drift-auditor` report format not yet standardised | Coordinate output format with drift-auditor in Wave-1 roster rewrite |
| 0 ADRs ratified | Lead writes ADR-0001 (gosplan) at Wave-0 conclusion; doc-agent creates wiki stub |
| Workspace lints section may break git-dep crates (egui, yakui pinned to git) | Test `cargo check --workspace` after adding `[workspace.lints]` — git-dep crates are members and inherit lints |

---

## Sources

All commands run on 2026-08-28 at HEAD `4e9e930b2a73` unless noted.

- `cargo doc -p simulation --no-deps` [live]
- `cargo +nightly rustdoc -p simulation -- -Z unstable-options --show-coverage` → `target/doc/simulation.txt` [live]
- `cargo search cargo-deadlinks` → 0.8.1 [live]
- `cargo search cargo-spellcheck` → 0.15.7 [live]
- `cargo search cargo-rdme` → 2.2.2 [live]
- `cargo search cargo-readme` → 3.4.0 [live]
- `mdbook --version` → 0.5.4 [live]
- `ls .code-review-graph/wiki/ | wc -l` → 201 [live]
- `mcp__code-review-graph__get_wiki_page_tool economy-capital` [live]
- `mcp__code-review-graph__get_wiki_page_tool economy-dispatch` [live]
- `cat docs/plan/iterations/build_roadmap.py` [live]
- `head -60 docs/plan/iterations/requirements/build_requirements.py` [live]
- `head -60 docs/plan/iterations/evidence/build_evidence.py` [live]
- `cat simulation/src/lib.rs | head -30` — no missing_docs lint declared [live]
- `cat Cargo.toml | grep -A20 workspace` — no `[workspace.lints]` [live]
- `ls docs/reference/` [live]
- `.planning/process-overhaul-2026-08-28/07-framework-and-documentation-patterns.md` [live]
- `docs/plan/proposals/gosplan.md` §5 [live]
- doc.rust-lang.org/rustdoc/lints.html — `missing_docs`, `broken_intra_doc_links` [**unverified** against 1.97.1; verify before use]
- arc42.org/overview — section numbering [**unverified** current state; check before adopting]
- alistairmavin.com/ears — EARS patterns [verified 2026-08-28 per report 07]
- github.com/Michael-F-Bryan/mdbook-linkcheck — compatibility with 0.5.x [**unverified**]
- doorstop, StrictDoc, OpenFastTrace, reqflow, sphinx-needs — assessed from known project characteristics [**unverified** current release status of doorstop and StrictDoc]
