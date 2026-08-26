---
name: feedback-herdr-codex-only
description: Herdr panes are for codex (OpenAI billing) only; Claude teammates run as Agent-tool subagents, not herdr claude sessions
metadata:
  type: feedback
---

Do not spawn Claude teammates as herdr claude panes — use normal Agent-tool subagents.
Herdr is reserved for codex sessions (they bill OpenAI, which is the whole point).

**Why:** user, 2026-08-26: "using herdr is waay too expansive for using claude team
mates/agents; lets use it only for prompting codex". Interactive claude panes came up on
Opus 5 ignoring the agent-definition model pins, and each pane is a full session with its
own overhead — measurably pricier than pinned-tier subagents.

**How to apply:** wave workers, gates, advisors → Agent tool (models per
[[delegation]] tiers). Cross-vendor review or codex implementation → herdr codex pane,
briefed per [[feedback-herdr-no-polling]].
