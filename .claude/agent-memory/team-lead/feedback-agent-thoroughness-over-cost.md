---
name: feedback-agent-thoroughness-over-cost
description: On soviet-simulator, never cap an agent's depth to save tokens — agents do as much as the job needs; cost discipline applies to wave SIZE and phase ORDER, not to how thoroughly a dispatched agent works
metadata:
  type: feedback
---

**Do not put token or tool-call budgets in an agent prompt to constrain how thoroughly it works.**
Agents "do what they need, as much as they need."

**Why:** stated directly by the user on 2026-08-23, after I flagged that `wiring-auditor` used 29
tool calls and 78.8k tokens against a designed target of ~15 calls / ~15-30k. I framed that as a
prompt failure needing "teeth." The user rejected the framing outright. In that same run the agent
independently reproduced two known findings AND surfaced one the 112k opus gate had missed
(`Market::dispatches()` has no in-game observation surface). The extra depth is what found the
extra finding — capping it would have bought a cheaper, worse audit.

This overrides the general "token economy — treat as a top priority" instruction in the global
CLAUDE.md *for the depth of a dispatched agent's work on this project*. It does not repeal it
elsewhere.

**How to apply:**

- **Never** write "target ~N tool calls", "tool-call budget: ~N", or "stop and report a partial if
  you run low" as a *thoroughness* constraint in an agent prompt. Say "be exhaustive within your
  scope" instead.
- Scope constraints are still right and still wanted: *narrow in scope, never in depth.* Telling an
  auditor "do not review logic, other agents own that" is good. Telling it "only make 15 calls" is
  not.
- "Stop early and report an honest partial" remains correct guidance for a genuinely **blocked**
  agent — one that discovered its brief was wrong (two agents did exactly this on STORY-0149 AC-4
  and both were right to). That is about accuracy, not economy. Keep it; just do not tie it to a
  budget.
- Cost discipline still applies where it does not trade against quality: **how many agents** a wave
  spawns, **which phase** runs first (cheap mechanical filters before expensive reasoning, because
  a reachability defect makes later review moot — not because of tokens), and not re-deriving what
  is already in context.
- Announcing scale before a wave is still wanted. Announcing is information; capping is damage.
- The cost table in `docs/dev-cycle.md` is an **observation** of what phases have actually cost, for
  planning. It is not a budget any agent should be held to.

See [[gotcha-inherited-claims]] for the related rule that a cheap unchecked claim is worse than an
expensive verified one.
