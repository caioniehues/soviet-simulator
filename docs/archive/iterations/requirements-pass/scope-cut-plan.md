# Wave plan — map charter Post-1.0 items to STORY-IDs

Goal: clear the P1 gate `sov-scope-cut-1p6` with a concrete, charter-grounded deferral list.
Lead judges; miners only extract. Nothing in the repo is written by a miner.

Source of truth: `docs/superpowers/iterations/requirements/EPIC-*.md` (STORY-IDs + titles + ACs).
Charter list: `docs/charter-1.0.md:102-118` (Post-1.0, then Never).

| Phase | Owner | Reads | AcceptanceCheck |
|---|---|---|---|
| P1 people/services sweep | miner-A (sonnet) | EPIC-016..021, 029, 030 | every STORY-ID in range classified; each HIT carries a verbatim quote |
| P1 vehicles/logistics/production sweep | miner-B (sonnet) | EPIC-007, 013, 014, 015, 022..028, 035, 036 | same |
| P1 utilities/economy/construction sweep | miner-C (sonnet) | EPIC-001..006, 008..012, 031..034 | same |
| P2 judge + propose | lead | the three reports | re-grep every quoted string before it enters the proposal |
| P3 user approval | user | proposal | explicit yes on the deferral list |
| P4 apply | lead | requirements/, build_roadmap.py | `build_roadmap.py` re-run, counts reconcile, `sov-scope-cut-1p6` closed with evidence |

File ownership: no miner writes files. Lead is sole writer of `requirements/`, `roadmap.md`,
`RESUME.md`, this plan.

Status: P1 dispatched.
