---
name: gotcha-herdr-friction
description: Herdr orchestration pain points observed 2026-08-26 — lost prompts on restart, model pins ignored, summary-only reports
metadata:
  type: feedback
---

Friction log for herdr-based orchestration (user asked these be recorded):

1. **Restarting a herdr agent races queued prompts.** ctrl+c → `agent start` → `agent prompt`
   all returned success, but the prompt was silently swallowed (virgin session, 0% context).
   After any restart, verify the pane actually shows the prompt landed before trusting
   `agent_prompted`.
2. **Interactive `claude --agent X` panes ignore the agent definition's model pin** — came up
   on Opus 5 where the roster pins sonnet. Root of [[feedback-herdr-codex-only]].
3. **Codex has no reasoning-effort switch mid-session** — needed a kill+restart with
   `-c model_reasoning_effort=high`, which caused (1).
4. **Subagent finished reports don't reach the lead on their own** — a gate's one-line
   "APPROVE" idle summary arrived but the full report did not; the teammate couldn't push it
   and the lead had to SendMessage to pull the auditable version. Always require the full
   report in the brief ("your final message is your report") AND expect to pull it anyway.
   NOT herdr-specific: plain Agent-tool subagents did the same 4× in one wave (sov-00c,
   2026-08-27) — idle notification, no report, one SendMessage pull recovered each. Budget
   one pull per worker as normal cost, not an anomaly.
5. **`herdr agent wait` resolves on momentary idle** — a worker that pauses to think settles
   the wait; a follow-up stability recheck is needed (moot while [[feedback-herdr-no-polling]]
   holds, but bites any future wait).

**How to apply:** when driving codex via herdr, treat every lifecycle transition as lossy —
verify state by reading the pane, not by trusting command exit.
