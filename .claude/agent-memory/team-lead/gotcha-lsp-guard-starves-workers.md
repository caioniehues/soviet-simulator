---
name: gotcha-lsp-guard-starves-workers
description: SUBAGENTS HAVE NO LSP — proven by probe 2026-08-27, suspected since 2026-08-23. Never brief a worker to use or warm LSP; resolve symbols yourself and paste file:line. Also: the read-guard budget is shared per cwd
metadata:
  type: project
---

## PROVEN 2026-08-27: subagents have NO LSP, and it is not recoverable

A `wiring-auditor` probe whose definition listed `LSP` got, verbatim:

    Error: No such tool available: LSP. LSP is disabled for this session, in subagents as well as here.
    ToolSearch("select:LSP,ListAgents")  ->  No matching deferred tools found

The message is misleading: LSP kept working in the MAIN session minutes later. Measured subagent
toolset: `Read`, `Bash`, `ToolSearch`, `Skill`, `Write`, `Edit`, `SendMessage` (deferred).
Absent: `LSP`, `ListAgents`, `Grep`, `Glob`, `Agent`, `WebFetch`.

**Never brief a worker to use LSP or to "warm" it. Resolve symbols yourself in the main session
and paste `file:line` into the brief.** A structural graph over MCP is the only code-intelligence
tool a worker can reach.

**The expensive part is that this note already said so on 2026-08-23** — see the worker quote
below — and I briefed 4 workers with "your LSP is preloaded, warm it with documentSymbol" anyway.
Rediscovering it cost ~330k tokens across three agents. Read this file before writing a brief.

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

**The fix IS installed** (confirmed 2026-08-26 by reading `~/.claude/hooks/lsp-first-read-guard.js`
directly — this memory previously said "not installed"; that was stale). It adds a per-file
consecutive-block counter (`RELENT_AFTER = 2`) that relents at every gate (Gate 1 warmup, Gate 4
nav, Gate 5 surgical) with a loud `emitWarning`, so an agent that genuinely cannot call LSP is
never deadlocked, while an agent that does call LSP never reaches it. The warning text itself says
"say so in your report rather than falling back to `bash cat`" — so a worker reporting "no LSP
tool available, using Bash/grep" is the fix working as designed, not a new failure.

**Session-scale facts (2026-08-27, sov-00c review wave, 7/7 workers):** when the LSP tool is
disabled for the SESSION ("LSP is disabled for this session, in subagents as well as here"),
every subagent lacks it — do not put an LSP-warmup instruction in briefs then; instead state
the working read path: Read relents on the **third** attempt per file (`n > RELENT_AFTER`,
RELENT_AFTER=2 — not the second, as this note previously implied), and `grep -n` via Bash is
sanctioned by the guard's own message. All 7 workers completed with grep-based evidence.

**How to read a worker reporting this:**

- Not sloppiness, not a new bug — the escape hatch fired because either the LSP tool was genuinely
  absent from that agent's toolset, or two block-and-retry cycles happened first (rare in practice
  since ToolSearch("select:LSP") usually succeeds).
- Still worth checking per-brief: did the brief instruct the LSP warmup call up front? Omitting it
  is a lead-side gap (the global tool-discipline rule lives in the lead's context, not
  auto-inherited by a subagent) — the guard rescues the agent from a deadlock, it does not make the
  agent call LSP in the first place.
- The flag is still keyed on cwd only (shared across concurrent agents in one repo) — later workers
  in a large parallel wave can still be more restricted than earlier ones. That part of the
  original finding stands.

See [[feedback-agent-thoroughness-over-cost]] — a hook that silently caps a worker's reads is the
same harm as a budget in the prompt, arriving through the harness instead.
