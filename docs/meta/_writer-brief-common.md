# Common brief — knowledge-base writers (2026-08-28)

You are one of three Opus writers producing pages of the repository's Markdown knowledge base
under `docs/`. The lead writes the front door, `SUMMARY.md`, the architecture handbook, the
engineering standards, the reference indexes, and the concept pages' *link targets*. You write the
subtree named in your lane brief and nothing else.

## Read first, in this order

1. `docs/plan/charter-1.0.md` — binding scope. The 1.0 table and the Explicit cuts.
2. `docs/reference/glossary.md` — binding terms. Use its words; avoid the words it says to avoid.
3. `docs/vision/design-bible.md` — the curated design synthesis. It carries evidence labels and
   code-status notes per section. It is being decomposed into the pages you write; it is your
   primary content source. It is NOT authority.
4. `docs/research/conversation-mining-2026-08-28/SYNTHESIS.md` — the reconciliation of the GPT
   design thread against ten validation lanes and the code. §3 (reconciliation ledger), §4 (code
   reality), §5 (what the thread missed), §6 (open conflicts), §7 (open questions).
5. The lane reports for your lane (named in your lane brief) — the validation detail, sources,
   data-structure sketches and tests.
6. The draft specifications for your subsystems under `docs/reference/specifications/`. Every one
   is `Status: draft`. They are the *target contract*; cite them, never restate their MUST/SHALL
   rules.
7. `docs/reference/architecture/substrate.md` and `docs/research/fact-sheets/wave1-*.md` — current
   code reality with `file:line`. Where SYNTHESIS §4 or §9 says a fact-sheet row is stale, the
   synthesis wins.

You have Bash, Read, Grep, Glob and the code-review-graph MCP tools. **Before you write "the code
does X", open the file.** Cite `path` and symbol (e.g. `simulation/src/souls/goods_company.rs`,
`recipe_should_produce`). Line numbers are allowed only in a "Current substrate" section and must
be ones you saw this session.

## The five states of knowledge — never blur them

Every page separates, with these exact H2 headings where the content exists:

```text
## What this is            (the game concept — why it matters to the player)
## 1.0 requirement         (only what the charter and draft specs commit to; cite SPEC anchors)
## Target design           (what the design thread and lane reports propose; label evidence)
## Current substrate       (what the committed code does today; path + symbol; cite fact-sheets)
## Research basis          (historical/technical evidence; cite the lane report or source)
## Open questions          (unresolved; make no implementation claim)
## Related                 (4–8 relative links)
```

Omit a heading only if the page truly has nothing under it. A page about a Post-1.0 mechanic says
so in the first paragraph and has no "1.0 requirement" section.

Scope label for every mechanic, in the first paragraph or a one-row table, exactly one of:
`1.0 — charter row <name>` (must name the row it derives from) · `Post-1.0` (charter cut or
beyond; may add `hook` when the data should be designed so it can arrive later) · `research`
(evidence only, no mechanic proposed). `1.0 binding`, `1.0 candidate` and bare
`architecture hook` are retired ([ADR-0001](../decisions/0001-households-and-utilities-are-1.0-scope.md)).

Evidence labels for design and research claims: `CONFIRMED` (cited source or code),
`PLAUSIBLE`, `HYPOTHESIS`, `UNSUPPORTED`. Copy the label the synthesis or the lane report gave;
do not upgrade it.

## Page header (exact form)

```markdown
# <Title as a noun phrase>

**Kind:** concept | current-state | research | index
**Authority:** advisory
**Status:** draft
**Owner:** <simulation | society | economy | transport | infrastructure>
**Last verified:** 2026-08-28
```

`Kind: concept` for design pages. `Kind: index` for the subtree `index.md`. Use `Kind: research`
only for a page that is evidence with no mechanic proposal.

## Writing rules

- One page answers one major question. 60–200 lines. No page for every enum.
- **Reserve MUST/SHALL/SHOULD for quoting a spec.** Design pages say "the design proposes", "the
  thread suggests", "historical research indicates". Never "enterprises SHOULD hoard".
- Cross-cutting ideas are explained ONCE on the concept pages the lead owns:
  `docs/simulation/concepts/{authority,physical-causality,scarcity,queues,reserves,phase-lag,reliability,information,adaptation,social-reproduction}.md`.
  Your page names the domain instance and links: "Household pantry buffering is one domain
   instance of [reliability → defensive buffering](../simulation/concepts/reliability.md)." Do not re-explain.
- Numbers: use only numbers with a source (Lane B2 §3 calibration table, Lane B1 §2, Lane A §2).
  The design thread's invented tables (72/141/24/18; 61/67/82/91) are banned.
- Historical institutions are separated by system and period: USSR 1930s–50s vs 1950s–60s (the
  game's fixed era) vs late-Soviet; Yugoslav self-management; Hungarian reform; Polish workplace
  politics. Never "socialist unions worked like X". Separate *formal institution* / *observed
  practice* / *research uncertainty* / *candidate mechanic*.
- Links are relative. To a spec: `../../reference/specifications/logistics.md#spec-logistics-005`
  (anchor = lowercase spec id). To the glossary: `../../reference/glossary.md`. To the current
  substrate page the lead is writing: `../../architecture/current-substrate.md`. To the mechanics
  index: `../../reference/mechanics-index.md`. Adjust `../` depth for your file's location.
- End every page with `## Related` — 4 to 8 links, no auto-generated clouds.
- Stable filenames as given in your lane brief. No dates in filenames. Do not create files not in
  your list; if a page is needed, say so in your final message.
- ASCII diagrams in fenced `text` blocks are welcome where the bible uses them.
- Do not edit any file outside your subtree. Do not run `bd` mutations. Do not commit.

## Your `index.md`

Explains: what belongs in this section; what does not; the recommended reading path (an ordered
list); the authoritative documents this section depends on (charter rows, spec files); related
sections. 40–90 lines.

## Final message to the lead (≤ 30 lines)

The list of files written with line counts; any page you could not source honestly and why; any
contradiction you found between the bible, the synthesis, a spec and the code (cite all sides);
any link target you needed that is not in the list above.
