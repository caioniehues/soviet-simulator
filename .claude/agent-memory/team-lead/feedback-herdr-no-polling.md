---
name: feedback-herdr-no-polling
description: Don't poll/watch herdr teammate panes — instruct them to message the lead when done; lead stays idle
metadata:
  type: feedback
---

Don't run background `herdr agent wait` loops on teammate panes. Brief every herdr-spawned
agent to actively report in (SendMessage to the lead session, or `herdr agent prompt` the
lead's pane) on finish or block, then go idle until they do.

**Why:** user, 2026-08-26: "yo dont watch them; they will communicate with you" — polling
burns tokens and adds nothing once workers know the comms channel.

**How to apply:** every herdr worker brief ends with "message me when done/blocked"; the
lead never schedules waits on agent panes. Related: [[persistent-teammates]].
