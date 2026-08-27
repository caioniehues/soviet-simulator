---
name: sweep-agent-roster-2026-08-27
description: Sweep of all 16 agent definitions and development-cycle.md roster at commit 8531d3c plus uncommitted working tree. Model tier contradiction is the top finding.
metadata:
  type: project
---

## Sweep scope
- All 16 `.claude/agents/*.md` files (working tree)
- `docs/process/development-cycle.md` roster section (working tree)
- Commit: HEAD is `8531d3c` with extensive uncommitted changes

## Top findings

1. **Model tier contradiction**: all 16 agent YAMLs say `model: opus` (working tree) while dev-cycle.md says "uniform sonnet." Committed state (8531d3c) had `model: sonnet` in all agents.
2. **Line count drift**: `simulation/src/` is 17,746 lines (agents claim ~15,400); `native_app/src/` is 10,085 lines (agents claim ~3,600); `assets/` has 97 PNGs (agents claim 2,584).
3. **kornai-economist stale substrate claims**: `set_requested` now has a production caller; `Market::remove` now clears `reserved`, `requested`, and dispatches.
4. **Evidence-auditor ToolSearch typo**: `ToolSearc` (missing `h`) at line 11.
5. **Story counts in dev-cycle roster**: "25 stories", "26 stories", "24 stories" — legacy corpus numbers; current system has 21 requirements.

## Drift-prone artifacts
- Line counts in agent descriptions (simulation, native_app)
- Model tiers (agents vs dev-cycle)
- Substrate claims about market.rs internals (change frequently)

## Generated files and their generators
- `docs/generated/roadmap.md` ← `docs/plan/iterations/build_roadmap.py`
- `docs/plan/iterations/extract/requirements.json` ← `extract/build_extract.py`
- `docs/generated/evidence/` ← `evidence/build_evidence.py`
