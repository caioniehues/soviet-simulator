---
name: feedback-consult-memory-before-briefing
description: Read agent-memory BEFORE writing briefs, not after the work — a note from 2026-08-23 already held the answer I spent ~330k tokens rediscovering on 2026-08-27
metadata:
  type: feedback
---

Consult `.claude/agent-memory/team-lead/` **before** writing the first brief of a session, not
when updating memory at the end.

**Why:** on 2026-08-27 I briefed four workers with "your LSP tool is preloaded — warm it with one
`documentSymbol` call". Every one of those briefs was unfollowable. [[gotcha-lsp-guard-starves-workers]],
written 2026-08-23, already carried a worker's verbatim report that LSP, Grep and Glob were all
absent from a subagent. I had the answer and did not look. Confirming it took a probe plus two
opus reviewers, roughly 330k tokens, and I shipped the false instruction into 16 agent definitions
(commit `e8744c5`) before catching it.

**How to apply:** the trigger is *writing a brief or a rule about worker capability*, not "doing
memory work". Before asserting what a subagent can do, grep this directory for the tool name. Same
for any claim about the harness — my memory index is short enough to scan in full.

Corollary, learned the same day: a claim about the runtime is only settled by **running it**. Two
opus agents reading the official docs produced a confident mechanism that a 40k-token probe partly
refuted — the docs explained LSP and ListAgents, but not the missing `Grep`/`Glob`, which turned
out to be auto mode. When the question is "what does this actually do", spend the probe.
