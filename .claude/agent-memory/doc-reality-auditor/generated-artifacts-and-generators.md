---
name: generated-artifacts-and-generators
description: The standing map of generated docs to their generator commands, and which artifacts carry a self-describing Generator header that drifts after a path move
metadata:
  type: project
---

Four generated documentation artifacts exist, plus two generated JSON inputs. Each Markdown one
carries a `**Generator:** ...` header line that the generator itself emits, so a path move breaks
the header unless the generator source string is edited too.

| Artifact | Generator source | Notes |
|---|---|---|
| `docs/generated/roadmap.md` | `docs/plan/iterations/build_roadmap.py` (header string at line 65) | `--check` compares byte-for-byte, so a stale header passes the check |
| `docs/generated/evidence/coverage.md` | `docs/plan/iterations/evidence/build_evidence.py` (header at line 253) | |
| `docs/generated/evidence/current-regression-inventory.md` | same, header at line 292 | |
| `docs/generated/evidence/target-scenarios.json` | same, written at line 380 | roadmap reads this as `--evidence` |
| `docs/generated/evidence/current-regression-inventory.json` | same, line 381 | |
| `docs/plan/iterations/extract/requirements.json` | `docs/plan/iterations/extract/build_extract.py` | |

**Why:** `build_evidence.py:350-356` (`write_or_check`) writes all four of its outputs into one
`--output-dir`. Splitting the `.md` outputs into `docs/generated/evidence/` while the `.json`
outputs stay in `docs/plan/iterations/evidence/` makes the documented `--check` command fail and
would fork the JSON on the next real run.

**RESOLVED 2026-08-27** (lead, same session this memory was written): all four outputs now
live in `docs/generated/evidence/`. `build_roadmap.py:65`, `RESUME.md`, `development-cycle.md`
and `.claude/agents/evidence-auditor.md` were repointed, and both files were regenerated. All
four `--check` commands in RESUME's block now exit 0. Inputs (`build_evidence.py`,
`evid-spec-bindings.json`) deliberately stayed under `docs/plan/` — generators and their inputs
live in `docs/plan/`, generated artifacts in `docs/generated/`.

**How to apply:** whenever a generated artifact moves, check three places, not one — the artifact's
own Generator header, the generator's header string constant, and every doc that quotes the
command (`docs/plan/iterations/RESUME.md`, `docs/process/development-cycle.md` Phase 6).

A byte-for-byte `--check` that passes proves reproducibility, never correctness of the header.

Related: [[sweep-uncommitted-docs-2026-08-27]].
