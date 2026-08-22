# Resume — requirements extraction COMPLETE, roadmap is next

Read this before acting; do not re-derive state.

## Where things stand — verified, not remembered

`extracting-requirements` finished on 2026-08-22 and is **committed**. All eight domains
extracted, adversarially reviewed for omissions, remediated, aggregated, and gated.

| artifact | state |
|---|---|
| `requirements/` | 35 epics, 139 stories, 361 ACs |
| `behavior-scenarios.md` | 151 scenarios — 57 surface, 53 contract, 41 failure-recovery |
| `behavior-corpus.md` | 151 rows; 5 promoted to `sentinel`, rest `iteration`; every `Command` is `TBD` |
| `coverage-ledger.md` | 23 chunks: 22 covered, 1 non-normative. **0 gaps, 0 story-only** |
| `extract/*.json` | the eight per-domain sources; `extract/validate.py` is the schema gate |
| `par/*.md` | 16 omission-review reports (2 independent reviewers × 8 domains), kept as audit trail |

`extract/validate.py <file>` checks schema, `[SUBSTRATE: …]` tags, enum values, duplicate
titles/AC-ids, and that every `owning_story_titles` resolves. It has been mutation-tested.
Re-run it after any hand-edit to an extraction JSON.

## What was decided along the way

- **Single rouble for 1.0.** `spec/trade.md:26-31` marks dual currency "CONFIRMED, adopted";
  `docs/charter-1.0.md:108` puts it in Post-1.0. The charter wins. The dual-currency work is
  **captured, not dropped** — story "Offer per-currency loans with interest and borrowing caps"
  carries `deferred: true`. Do not schedule it for 1.0; do not delete it either.
- **Fix-everything disposition.** All 172 PAR findings were remediated, including numeric
  constants as ACs (water-quality thresholds 0.93/0.97/0.60, hospital beds 100, serve-rate 3,
  seat formula StudentCount×5/4, route modifiers ×7.5/×0.95, and so on). The user chose this
  over deferring thin-ACs. Do not "simplify" these back out.
- **Epic consolidation.** 46 epics → 35 by merging per-sub-mechanic splinters (Education/
  Healthcare/Crime were 9 epics for 10 stories) and the cross-domain duplicate
  `Physical foreign trade` ≡ `Physical border trade`.

## Step 1 — the next step

Run `iterative-development:scoping-the-simplest-core` against `requirements/` to pick the
walking-skeleton iteration and order the rest into `roadmap.md`.

The walking skeleton is **the dishonest enterprise** — a plant that misreports output to the
planner. `SCENARIO-0090 "Player detects a hoarding enterprise from its inspection panel alone"`
is already in the corpus at `e2e` seam and marked `sentinel`; it is the natural first journey.

## Step 2 — the decision waiting at the roadmap

The corpus roughly doubled during remediation, so the roadmap will be large. **If it is bigger
than you want to schedule for 1.0, the lever is the `deferred` flag, not deleting requirements** —
same mechanism already used for per-currency loans. Raise this with the user at roadmap
review, with a concrete proposed cut, rather than silently trimming.

## Known gap, deliberately accepted

There are **no `JOURNEY-NNNN` scenarios**. The extraction taxonomy generates them from a
`spec/journeys/` directory and this project's spec is 23 flat domain files. The sentinel corpus
was seeded instead by promoting five existing `e2e` scenarios: SCENARIO-0009, 0015, 0090,
0115, 0118. `scoping-the-simplest-core` should author the walking-skeleton journey properly
and add it to the sentinel set.

## Standing obligations

- **Visual proof is owed.** Per `CLAUDE.md`, work is not done until the user has seen it running —
  a 15–20s video once the first Soviet-side change lands. A prior screenshot attempt captured
  the wrong monitor.
- Egregoria pins git *branches* (`egui` master, a personal `yakui` fork's `dev`); lock to commits
  before any distribution.
- `Lot::generate_along_road` is disabled; nothing may depend on auto-lot generation.
- Prefer `integration` seams anchored on `TestCtx` (`simulation/src/tests/test_iso.rs`).
