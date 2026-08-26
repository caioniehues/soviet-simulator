# Memory Index

- [Single rouble for 1.0](decision-single-rouble.md) — dual currency deferred to Post-1.0; charter beats spec/trade.md on scope conflicts
- [Completeness over lean specs](feedback-completeness-over-lean-specs.md) — in spec/requirements work capture everything incl. numeric constants; ponytail still governs code
- [Inherited claims are untrusted](gotcha-inherited-claims.md) — two false substrate facts from RESUME.md reached ~20 briefs; verify handoff claims against code first
- [Thoroughness over cost](feedback-agent-thoroughness-over-cost.md) — never cap an agent's depth to save tokens; narrow in scope, never in depth
- [Specialist gates beat general ones](feedback-specialist-gates.md) — measured: 100.8k vs 112k and 5 findings vs 2 on the same seam
- [Price a pivot against an open goal](feedback-goal-drift.md) — say what a scope change costs the standing commitment, then do it; label what each ✅ is a check of
- [Never verdict before reading](gotcha-verdict-before-reading.md) — I ranked three frameworks from frontmatter and reversed after reading; always pick, and open the source first
- [The LSP guard starves workers](gotcha-lsp-guard-starves-workers.md) — hook deadlocks agents with no LSP tool and shares one read budget per cwd; tested fix not yet installed
- [Plain markdown, not Artifacts](feedback-plain-markdown-docs.md) — process docs are read by agents; repo `.md` + a CLAUDE.md pointer or it doesn't exist
- [The dev cycle](../../../docs/dev-cycle.md) — 8 phases, each naming the failure it prevents; agents are authored to fit its slots
- [No polling herdr teammates](feedback-herdr-no-polling.md) — brief workers to message the lead on finish/block; never background-wait on panes
- [Herdr for codex only](feedback-herdr-codex-only.md) — Claude teammates run as Agent-tool subagents; herdr panes ignore model pins and cost too much
- [Herdr/orchestration friction log](gotcha-herdr-friction.md) — lost prompts on restart, pull-don't-expect reports, waits settle on momentary idle
- [Codex-luna team](codex-luna-team.md) — 15-role .codex/agents mirror on luna/medium, codex exec transport, HEAD-scope-check every cross-vendor finding
