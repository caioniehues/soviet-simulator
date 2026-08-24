---
name: gotcha-lsp-guard-starves-workers
description: The lsp-first-read-guard hook blocked Read for 4+ workers with no LSP tool and no way out, and its budget flag is shared by every concurrent agent in one cwd — workers silently fell back to `bash cat`
metadata:
  type: project
---

`~/.claude/hooks/lsp-first-read-guard.js` has **no LSP-availability check and no escape hatch.**
On 2026-08-23 this cost four workers real turns and pushed at least one into violating
`rules/tool-discipline.md`.

**Two distinct defects, both verified from the agent transcripts:**

**1. Deadlock when the LSP tool is absent.** Subagents do not reliably get the LSP tool. One
implementer reported:

> "LSP tool was unavailable this session (ToolSearch found no LSP tool at all, for any query) but
> the lsp-first-read-guard hook still blocked Read until warmup. Grep/Glob tools were also absent
> from ToolSearch. I fell back to `bash cat -n` / `grep` for all reading, since Bash worked and
> nothing else did."

The hook blocks Read and instructs the agent to call LSP; LSP does not exist; the only remaining
way to read a file is `bash cat`, which the same rules file forbids. There is no legal move.

**2. The read budget is shared across concurrent agents.** The state flag is
`~/.claude/state/lsp-ready-<md5(cwd)>` — keyed on **cwd only**. `agentId` is not in the PreToolUse
payload and all subagents share the session's `sessionId`, so there is no way for the hook to tell
them apart. Two agents dispatched in the same wave (`a7257c13…` and `ad87181a…`, identical
`promptId d2aea593…`, identical cwd, 27 seconds apart) drew from one counter. That is why one was
blocked at *"Gate 5 — Surgical Mode Required, Read #6 requires at least 2 LSP navigation calls"*
having made only one nav call itself: a sibling had spent the budget. In a 15-agent wave the later
workers arrive already in surgical mode through no fault of their own.

**Blast radius:** 4 of 15 workers hit hard blocks; 7 mentioned the guard or a missing LSP tool.
Nobody reported it as a blocker — they quietly degraded to `bash cat`, so the tool-discipline rule
was being violated wave-wide and invisibly.

**A tested fix exists and is NOT installed** (global user config — needs the user's own go-ahead).
It adds a per-file consecutive-block counter that relents after 2 blocks with a loud warning, so an
agent that genuinely cannot call LSP is never deadlocked, while an agent that does call LSP never
reaches it. Verified standalone: blocks twice, allows on the third attempt, and the normal
progression (md free · 2 free reads · warn at 3 · block at 4 · re-reads free) is unchanged.
Tracked as the `br` ticket `sov-lsp-guard-deadlock-*`.

**How to apply until it is fixed:**

- When a worker reports reading via `bash cat`/`grep`, do not treat it as sloppiness — check
  whether the guard left it any alternative.
- Ask workers to state LSP availability in their environment note, so degradation is visible.
- Expect later workers in a large parallel wave to be more restricted than earlier ones; it is a
  shared-counter artifact, not a difference in how they work.

See [[feedback-agent-thoroughness-over-cost]] — a hook that silently caps a worker's reads is the
same harm as a budget in the prompt, arriving through the harness instead.
