# Documentation architecture — consolidation decision

**Kind:** explanation
**Authority:** explanatory
**Status:** superseded migration research
**Owner:** project lead
**Last verified:** 2026-08-24

**Scope:** repository Markdown and generated planning artifacts, not Rust API documentation. This
records the pre-cutover rationale and inventory; the active execution plan is
[`../../plan/controlled-documentation-rewrite.md`](../../plan/controlled-documentation-rewrite.md).

## Recommendation

Adopt a single `docs/` root for project documentation, retaining only repository-entry and legal
files at root: `README.md`, `AGENTS.md`, `CLAUDE.md`, `LICENSE`, and `NOTICE.md` (the latter is
the licence/provenance companion). Move `CONTEXT.md`, `ROADMAP.md`, `spec/`, `architecture/`,
`research/`, and the remaining status/history notes beneath `docs/` in one planned migration.

Use small, explicit metadata rather than imposing the same template on every document:

```markdown
**Kind:** reference | specification | explanation | decision | process | plan | generated | historical
**Authority:** binding | operational | reference | explanatory | historical | derived
**Status:** draft | active | accepted | superseded | archived
**Owner:** <code area, process, or role>
**Last verified:** YYYY-MM-DD   <!-- only when a claim needs an expiry signal -->
```

`Authority` answers whether a reader may act on the document; `Kind` keeps reference, task
guidance, decisions, and rationale from being conflated. This fits Diátaxis: reference describes
the system authoritatively, while explanation supplies context and rationale
([reference](https://diataxis.fr/reference/), [explanation](https://diataxis.fr/explanation/)).

Use BCP 14 keywords only in a document that deliberately declares the RFC 8174 boilerplate.
Uppercase **MUST**, **SHOULD**, and **MAY** otherwise have no special force. RFC 8174 makes that
interpretation applicable only when the words are capitalised and the convention is invoked;
RFC 2119 defines their requirement levels ([RFC 8174 §2](https://www.rfc-editor.org/rfc/rfc8174.html#section-2),
[RFC 2119](https://www.rfc-editor.org/rfc/rfc2119.html)).

## Target tree

```text
README.md                    repository entry, build/run and one-sentence doc pointer
AGENTS.md / CLAUDE.md        root-only agent entrypoints; link to docs/process/
LICENSE / NOTICE.md          root-only legal and provenance material
docs/
  README.md                  navigable map, authority rules, templates and validation command
  reference/
    glossary.md              current CONTEXT.md vocabulary
    architecture/            current system/reference maps
    specifications/          subsystem mechanism specifications and their index
  decisions/                 ADR-0001-...md; only durable, accepted design decisions
  plan/
    charter-1.0.md           binding scope
    iterations/              handwritten requirements, scenarios and process inputs
  generated/
    iterations/              roadmap and other derived planning outputs only
  process/                   development cycle, contribution and agent-operating guidance
  explanation/               research, vision, framework/design rationale, art direction
  archive/
    bevy-track/              immutable prior-track documents
```

This is intentionally a shallow information architecture, not a full arc42 transcription. arc42
does support the relevant architecture areas—building blocks, cross-cutting concepts, quality,
decisions, and glossary—and recommends ADRs for important, expensive, or risky decisions
([template overview](https://arc42.org/overview/), [decisions](https://docs.arc42.org/section-9/),
[glossary](https://docs.arc42.org/section-12/)).

## Formats and migration rules

1. **Reference versus explanation.** Put binding scope in `plan/charter-1.0.md`; mechanism in
   `reference/specifications/`; current vocabulary in `reference/glossary.md`; and research,
   rationale, and historical discussion in `explanation/` or `archive/`. A `spec` claim remains
   evidence-tagged using the existing `CONFIRMED`/`OBSERVED`/`INFERRED`/`SPECULATIVE`/`OURS`
   vocabulary; no status word substitutes for evidence.

2. **ADRs.** Keep one Markdown file per decision, renumber only if a dedicated migration chooses
   to. New ADRs use `Status`, `Date`, `Decision makers`, `Context and problem statement`,
   `Decision drivers`, `Considered options`, `Decision outcome`, `Consequences`, `Confirmation`,
   and `More information`. MADR provides this lifecycle and template, including `proposed`,
   `accepted`, `deprecated`, and `superseded by ADR-xxxx`
   ([pinned template](https://raw.githubusercontent.com/adr/madr/d1698d0b8b6b8ef83a0a255d3e3920cbcda159ba/template/adr-template.md),
   [MADR guidance](https://adr.github.io/madr/)). Existing free-form ADRs should receive missing
   metadata only after their current decision and implementation state are verified.

3. **Generated versus handwritten.** Generated files live only in `docs/generated/`; their first
   lines name the generator, authoritative inputs, and regeneration command. Handwritten sources
   never include generated blocks without sentinels. Keep architectural decisions, rationale,
   specifications, and process documents handwritten: generated reference can remain faithful to
   code, but autogenerated material alone is insufficient documentation
   ([Diátaxis reference](https://diataxis.fr/reference/)). This directly separates the current
   generated iteration roadmap from its requirements and avoids stale hand-edited totals.

4. **Archive, do not leave ambiguous history.** An archived document moves as a whole under
   `docs/archive/<track-or-era>/`, retains its original date, starts with `Kind: historical` and
   `Status: archived`, and names its successor. It is excluded from current navigation except a
   short archive index. Do not silently delete superseded rationale; Git history remains the
   record of ordinary revisions.

5. **Links and validation.** Use portable inline links such as `label (relative-path)` and relative links for
   repository files; GitHub documents both this form and generated section anchors
   ([GitHub Markdown links](https://docs.github.com/en/get-started/writing-on-github/getting-started-with-writing-and-formatting-on-github/basic-writing-and-formatting-syntax#links)). Add CI checks for Markdown linting, local-file links, anchors, and external links with retry/cache policy. Docs-as-code explicitly supports version control, review, and automated tests
   ([Write the Docs](https://www.writethedocs.org/guide/docs-as-code/)); no primary source found
   mandates a particular link checker.

## Options and trade-offs

| Option | Benefits | Cost / verdict |
|---|---|---|
| Keep the current split roots | No move work | Authorities and history remain hard to locate; generated and handwritten artifacts stay mixed. Reject. |
| One `docs/` tree, Markdown on GitHub | One discoverable home; preserves current review and links; low operational cost. **Recommend.** | Requires an inventory, atomic path/link update, and clear index. |
| One `docs/` tree plus a site generator now | Navigation, search, and versioned published docs are possible. | Adds a build/deploy surface before an external documentation audience is established. Defer. |

A generator becomes warranted when a public/user documentation audience needs search, versions,
or a rendered navigation experience that GitHub cannot provide. If that trigger occurs, record the
tool and deployment choice in an ADR; neither Diátaxis nor arc42 prescribes one.

## Current-tree evidence and proposed destination

| Current material | Observed drift | Destination |
|---|---|---|
| Root `CONTEXT.md`, `CLAUDE.md`, `README.md` | Glossary, operational instructions, and repository entry are co-located at root; only the latter two are true entrypoints. | Move `CONTEXT.md` to `docs/reference/glossary.md`; retain root `README.md` and agent files. |
| Root `ROADMAP.md`, `progress.md`, `findings.md`, `task_plan.md` | Bevy-era/history and active planning are mixed at root; `ROADMAP.md` identifies itself as Bevy track. | Archive the historical material; move live planning to `docs/plan/` after authority review. |
| `spec/` and `architecture/` | Both are documentation outside the proposed root; `spec/README.md` already defines an evidence discipline. | `docs/reference/specifications/` and `docs/reference/architecture/`. |
| `docs/adr/` | 17 ADRs exist, but several have no status line and none consistently use decision sections. | `docs/decisions/`; migrate metadata conservatively. |
| `docs/superpowers/iterations/` | 59 Markdown files include requirements, scenarios, PAR reports, resume state, and a generated roadmap in the same tree. | Separate handwritten inputs in `docs/plan/iterations/` from derived outputs in `docs/generated/iterations/`. |
| `docs/archive/`, `docs/wayfinder-brief.md`, `docs/vision/` | Some history is clearly labelled; legacy Bevy guidance and a large vision transcript need the same lifecycle rule. | `docs/archive/` and `docs/explanation/vision/`, with successor links/status. |
| Root `research/` and research-like `docs/*.md` | Research is split by provenance rather than reader purpose. | `docs/explanation/research/`; retain source dates and citations. |

The strongest existing authorities are already explicit: charter for scope, `spec/` for mechanism,
`br` for task state, and code for substrate (`CLAUDE.md`, `docs/dev-cycle.md`). The migration
should preserve that precedence in `docs/README.md`; it must not invent a competing plan of
record.

## Safe migration sequence

1. Approve this tree and the metadata vocabulary; designate one owner for the migration.
2. Inventory every Markdown file as `active`, `generated`, or `historical`, with a successor or
   source-of-truth for each non-active item.
3. Move one bounded category at a time, update all relative links in the same change, and add
   root pointers only where a root exception remains intentional.
4. Run the new link/anchor validation and the documented iteration generator; verify generated
   output is reproducible before removing its old path.
5. Add `docs/README.md` as the only navigation index, then make the site-generator decision only
   if its stated trigger is met.

## URL verification and unresolved gaps

All external URLs cited above were live-checked on 2026-08-24 and returned HTTP 200. No primary
standard dictates a universal archive layout, universal ordinary-document ownership field, or a
specific site generator/link checker; those are deliberately labelled project policy above.

## Repository files inspected

`AGENTS.md`; `CLAUDE.md`; `CONTEXT.md`; `README.md`; `ROADMAP.md`; `NOTICE.md`; `findings.md`;
`progress.md`; `task_plan.md`; `research/bevy-ecosystem.md`; `architecture/ecs.md`;
`architecture/simulation-clock.md`; `spec/README.md`; `docs/charter-1.0.md`;
`docs/dev-cycle.md`; `docs/art-direction.md`; `docs/framework-design.md`;
`docs/framework-study-brief.md`; `docs/egregoria-architecture.md`;
`docs/egregoria-substrate-audit.md`; `docs/wayfinder-brief.md`;
`docs/archive/bevy.md`; `docs/archive/bevy-track-README.md`;
`docs/superpowers/iterations/RESUME.md`; `docs/superpowers/iterations/roadmap.md`;
`docs/superpowers/iterations/task_plan.md`; `docs/superpowers/iterations/behavior-corpus.md`;
`docs/superpowers/iterations/build_roadmap.py`; `docs/superpowers/iterations/extract/validate.py`;
and `docs/adr/0001-simtick-substep-schedule.md` through
`docs/adr/0017-a-building-is-its-product.md`.
