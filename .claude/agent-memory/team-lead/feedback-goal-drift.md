---
name: feedback-goal-drift
description: When the user pivots mid-goal, price the pivot against the open goal out loud before accepting it — do not let a stated goal stay open silently for a whole session
metadata:
  type: feedback
---

**When the user asks for something that pivots away from an open `/goal`, say what it costs the
goal before doing it.** The form is one sentence: *"Yes — and it means ITER-0000 doesn't close
today. Still want it?"*

**Why:** on 2026-08-23 the goal was "ITER-0000 end to end". It stayed open for the entire session
while the work drifted, at the user's direction, into building 15 agents, two LSP plugins, a dev
cycle document and a four-framework comparison. Every pivot was requested and each was individually
worth doing — the agents found a game-breaking pre-existing bug. But the session ended with the goal
still open at 8/9 stories and 2/5 iteration deliverables, and **I never once offered the trade.**

The measured shape of the drift: ~400 lines of production logic against ~2,900 lines of process
documentation, in a session whose stated goal was implementation.

**How to apply:**

- The user is allowed to change priorities freely; that is not the problem. The problem is an
  unnamed trade. Name it in one line, then do what they say.
- A `/goal` is a standing commitment. If it is still open after several pivots, **say so unprompted**
  rather than waiting to be asked for a status.
- Prefer proposing the closing order when it is cheap: *"ITER-0000 needs the panel, the journey and
  the video — perhaps two dispatches. Want those first, or the agents first?"* Offering an order is
  not resisting the pivot.
- Infrastructure built mid-goal is not wasted, but it is borrowed against the goal. Say whose budget
  it comes out of.

See [[feedback-agent-thoroughness-over-cost]] — the reverse case, where cost discipline was NOT
wanted. The distinction: never cap an agent's depth to save tokens; do name what a scope change
costs a commitment already made.
