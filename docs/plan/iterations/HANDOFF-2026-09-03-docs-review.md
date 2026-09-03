# Handoff — 7-slice documentation review, 2026-09-03

**Kind:** plan handoff
**Authority:** operational handoff only; `bd` remains task-state authority
**Status:** review complete, findings decomposed into `bd`; no fixes started
**Owner:** project lead
**Last verified:** 2026-09-03

> Read this, then run `bd ready` — the queue is authoritative, this page is context.

## What happened this session

Seven read-only reviewer agents each audited one slice of the (mostly uncommitted) `docs/` rewrite
against the code at `b25a04b` plus the working tree. Every finding is cited `path:line` for both
docs and code. Totals: 9 high, 49 medium, 22 low (overlapping across slices).

Per-slice reports and the cross-slice synthesis are committed under
[`reviews/docs-review-2026-09-03/`](reviews/docs-review-2026-09-03/):

| File | Slice |
|---|---|
| `review-synthesis.md` | nine ranked cross-cutting themes + suggested fix order — **start here** |
| `review-authority.md` | index, SUMMARY, meta, templates, checker |
| `review-product.md` | product, vision, charter, proposals, roadmap |
| `review-economy.md` | planned/physical economy, transport, glossary, invariants, mechanics-index |
| `review-society.md` | society, infrastructure, national-projects, specifications |
| `review-architecture.md` | architecture pages, both substrate maps, wave1 fact-sheet |
| `review-engineering.md` | engineering, developer, process, dependency policy, dev-cycle |
| `review-research.md` | research corpus, explanation, archive leakage, generators |

The reports are observational research artifacts, not binding; they are frozen at the review commit
and must not be "updated" — file follow-ups in `bd`.

## Decomposition (all in `bd`, label `docs-review-2026-09-03`)

Sixteen beads, each owning a disjoint file set so they can run concurrently. Doc tasks are children
of the in-progress `sov-6pr` epic.

**Code defects newly filed**

- `sov-uo5` — external import has no accountable border source stock (FreightStation is counters only)
- `sov-bub` — bounded Loading/Returning route failure deletes cargo after the seller was debited; no loss sink

Already tracked, given new evidence in comments: `sov-7f7`, `sov-20g`, `sov-5ut`, `sov-91e`
(may already be fixed — verify), `sov-n8v`/`sov-y66`, `sov-z9x` (its row-63 wording has
*regressed* in `reference/architecture/substrate.md`), `sov-journey-sentinels-rxa`.

**Decision needed from the lead**

- `sov-m7r` — Households/citizens, Utilities and Water pressure/tank are bound as 1.0 by
  `product/scope-1.0.md`, `vision/design-bible.md` and `docs/index.md` but absent from the charter's
  nine rows. Add charter rows or demote everywhere. `sov-8d1` and `sov-6uy` leave scope labels
  untouched until this is decided.

**Doc/process tasks (parallel-safe)**

| Bead | Owns |
|---|---|
| `sov-0kc` | the four determinism pages |
| `sov-bu6` | testing.md, writing-evidence-tests, getting-started, AGENTS.md test lines, `scenarios/mod.rs:5-12` comment |
| `sov-3mi` | development-cycle.md ↔ gate-chain formula ↔ `.claude/agents/*` ↔ dev-cycle skill |
| `sov-8d1` | design-bible trim + two product low fixes (P3) |
| `sov-brv` | research-lane metadata, AGENTS.md header, SUMMARY orphan, archived-agent deps |
| `sov-rut` | templates / document-authority / documentation-model taxonomy; canonical substrate map |
| `sov-ik2` | widen `scripts/check_docs.py` — **blocked on `sov-brv` + `sov-rut`** |
| `sov-a2p` | `wave1-economy.md` in-place rewrite |
| `sov-9mz` | architecture pages + `substrate.md` content (row 63 back to SPLIT) |
| `sov-6uy` | planned-economy / physical-economy / transport pages |
| `sov-bpp` | mechanics-index, invariants, glossary |
| `sov-kvn` | society / infrastructure / national-projects pages |
| `sov-ipc` | dependencies, dependency-policy, code-intelligence, bd-survey (P3) |

## Facts worth knowing before touching anything

- The four model rules: border-rouble rule is broken in code for **both** halves of external trade
  (`economy/mod.rs:103-104` applies `money_delta` at match, before `advance_dispatches`). Docs
  currently claim the import half is fixed. It is not.
- `test_world_survives_serde` runs an **empty** schedule and `is_equal` never compares `World`.
  Four doc pages describe it as a full repeat-run gate.
- `cargo test -p simulation sentinel` and `evid_logistics` match **zero** tests; 0 `evid_` tests
  exist. `AGENTS.md` still documents both as runnable.
- `scripts/check_docs.py` reports 0 errors because it checks metadata only on specs + 8 wiki
  sections; research, explanation and root entrypoints are outside its scope.
- `process/development-cycle.md` still dispatches agents that `b25a04b` archived.

## Suggested next wave

Dispatch via the dev-cycle, one implementer per bead, no shared files:
`sov-0kc sov-bu6 sov-a2p sov-6uy sov-bpp sov-kvn` (six page sweeps), then `sov-brv sov-rut` →
`sov-ik2`, then `sov-3mi`. Code defects `sov-uo5`/`sov-bub` go through the full gate chain with
ledger-invariant-checker on Phase 4.

## Working-tree state

The docs rewrite (≈75 modified/untracked paths, see `git status`) was **not committed** this
session; nothing here was committed either. The `reviews/` directory and this handoff are new
untracked files. Committing is the lead's call.

## Environment notes (omp, not project)

`web_search` chain was set to `[exa, parallel, firecrawl, anthropic, startpage, duckduckgo]`,
`codex` excluded, `providers.fetch = parallel`. Exa and Parallel keys are stored; Firecrawl is
not. Anthropic was still the provider answering probes at end of session — re-probe in a fresh
process before relying on Exa/Parallel.
