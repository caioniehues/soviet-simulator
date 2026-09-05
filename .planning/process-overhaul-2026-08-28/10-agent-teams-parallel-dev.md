# Agent Teams for Integrated Parallel Development

**Kind:** research
**Authority:** advisory
**Status:** active
**Owner:** researcher (global)
**Last verified:** 2026-08-28
**Feeds:** GOSPLAN §7 D2 — whether and how to adopt Agent Teams

Evidence base: live docs fetched 2026-08-28 (code.claude.com/docs/en/agent-teams.md v2.1.178+,
hooks reference, tools-reference); GitHub issues #34750, #42999, #28048, #58762, #48160, #56449
on anthropics/claude-code (all 2026); report 04 in this directory (baseline).

---

## Answer first

Agent Teams are buildable today with strict constraints. The V1 Lane Team is the only design that
survives the known limits. V2 Race and V3 Pair can both be implemented _without_ teams — via files
and gosplan relay — at comparable quality and lower risk. The three open runtime bugs (#42999
SendMessage name-routing; #58762 tmux mailbox mismatch; #48160 subagent SendMessage) make
unguarded production use unreliable. The recommended path is GOSPLAN §7 D2 option (a): do not
adopt teams now; schedule one probe story in Plan 02 to verify the five riskiest assumptions under
controlled conditions.

---

## Q1 — Exact mechanics

### How a lead spawns teammates

Spawning requires `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1` in `settings.json` `env` or the shell.
Once enabled, Claude spawns a teammate whenever it calls the `Agent` tool with a `name` parameter
— no separate setup step as of v2.1.178 (TeamCreate/TeamDelete no longer exist). [doc]
https://code.claude.com/docs/en/agent-teams.md

**Prompt form:** plain natural language.

```
Spawn three teammates:
- One focused on security
- One checking performance
- One validating tests
```

Claude decides count unless you specify. You can name models inline ("Use Sonnet for each
teammate"). The `team_name` input on the Agent tool is accepted but ignored since v2.1.178. [doc]

**Model selection priority** (verified against live docs, v2.1.234+):

1. `CLAUDE_CODE_SUBAGENT_MODEL` env var (if not `inherit`)
2. Model named in spawn prompt for that teammate
3. Subagent definition's `model` field (in-process only)
4. Lead's current model

`teammateDefaultModel` was removed in v2.1.234; a leftover value is ignored. [doc]

**Subagent definition as teammate:** Yes — you can reference any `.claude/agents/<name>.md`
definition by name in the spawn instruction:

```
Spawn a teammate using the security-reviewer agent type to audit the auth module.
```

Claude Code reads the definition and applies these parts:

| Definition field | In-process teammate | Split-pane teammate |
|---|---|---|
| `tools` | Enforced; SendMessage + Task tools added automatically | Enforced; SendMessage added |
| `model` | Used when env/prompt don't name a model | NOT used; teammate picks its own |
| Body | Appended to default system prompt | Replaces default system prompt |
| `skills` | NOT applied — teammate loads skills from project/user settings | NOT applied |
| `mcpServers` | NOT applied — loads from project/user settings | Applied per subagent field rules |

**Critical:** `skills:` in the definition does NOT preload into a teammate. The teammate loads
skills from project and user settings only — the same set a fresh session would load. [doc]
Similarly `memory:` frontmatter applies to subagents; the docs do not confirm it applies to
in-process teammates.

### What a teammate sees at start

- Own context window (lead's conversation history does NOT carry over)
- CLAUDE.md from all applicable scopes (project, user, managed)
- MCP servers from project and user settings
- Skills from project and user settings (lazy, not preloaded unless the definition is applied in
  split-pane mode with its own system prompt)
- Spawn prompt from the lead (the task description you gave)
- `.claude/rules/*.md` with `paths:` frontmatter: **unverified whether these load in teammates.**
  Report 04 gap 6 flags this for subagents; it is equally unverified for teammates. [unverified]

### Shared task list API

Task tools (`TaskCreate`, `TaskGet`, `TaskList`, `TaskUpdate`) are available to agents that have
them — model-dependent as of v2.1.233+. They are absent on Claude Opus 4.8, Sonnet 5, Fable 5,
Mythos 5 and later families. Enable explicitly with `CLAUDE_CODE_ENABLE_TODO_TOOLS=1` or
`--allowedTools TaskCreate`. [doc] https://code.claude.com/docs/en/tools-reference

When a session has Task tools, Claude Code automatically adds them to an in-process teammate's
tool set along with `SendMessage`. Without them, coordination is through `SendMessage` only.

**Dependencies:** Claude Code manages them automatically. When a teammate completes a task that
other tasks depend on, dependents unblock without any action. [doc]

**Task claiming:** file locking prevents race conditions when multiple teammates try to claim the
same task simultaneously. [doc]

**bd as replacement/mirror:** `bd` is a separate CLI that writes to a Dolt DB. It is NOT
integrated with the Agent Teams task list. A teammate can run `bd` via Bash to update the `bd`
tracker, but it cannot self-claim the Agent Teams shared task list using `bd`, nor does completing
a `bd` issue unblock an Agent Teams task dependency. The two systems are parallel and must be kept
in sync manually (or by the ledger hook). [live — derived from architecture; no native bridge in docs]

### Messaging semantics

- **Delivery:** Messages are written to `~/.claude/teams/{team-name}/inboxes/{agent-name}.json`.
  Delivery is confirmed only when the write to the mailbox file succeeds. [doc]
- **Busy teammate:** A message queued while a teammate is mid-turn is delivered on the teammate's
  next read of its mailbox. No interruption of current turn. [doc — inferred from mailbox
  architecture; no explicit "interrupt" path documented]
- **Idle notifications:** When a teammate finishes and stops, it notifies the lead automatically.
  The notification does NOT carry the teammate's output. Teammates share results by messaging the
  lead explicitly or updating the shared task list. [doc]
- **notify_when_idle:** This is a conceptual behavior, not a separate API call. It is the
  automatic idle notification described above (one-shot, expires after 12 hours if not delivered
  per report 04; the live doc confirms automatic notification but does not give the 12h expiry —
  treat as **partially verified**).
- **as of v2.1.198:** A teammate whose turn ends on an API error notifies the lead with the error
  text instead of appearing to finish normally. [doc]
- **Malformed mailbox fix:** Before v2.1.207, a single malformed entry caused a repeated error
  every second and blocked all delivery. Fixed in v2.1.207. [doc]
- **Bug #42999 (closed not planned, v2.1.92):** `SendMessage` with a teammate name returns
  `{"success":true}` but delivers nothing. Only the raw agent ID actually works. Workaround: a
  `PreToolUse` hook that resolves names to IDs by scanning `subagents/*.meta.json`. This is a
  PRODUCTION BUG if name-based messaging is relied upon. [source]
  https://github.com/anthropics/claude-code/issues/42999
- **Bug #48160 (open):** Spawned subagents cannot send `SendMessage` even with the flag enabled
  and `name=` parameter set. [source]
  https://github.com/anthropics/claude-code/issues/48160

### TaskCreated / TaskCompleted / TeammateIdle hooks — stdin payload

The hooks reference confirms all three events are supported and all can block (exit code 2). [doc]
The exact field-level schemas were not fully extractable from the fetched content — the page
carries the event table but the per-event schema tables were truncated. **Unverified fields:**
task_id, task_title, task_description, teammate_name.

Confirmed common fields (across all hook events): `session_id`, `prompt_id`, `transcript_path`,
`cwd`, `permission_mode`, `hook_event_name`, `agent_id`, `agent_type`. [doc]

**SubagentStop** delivers: common fields + `agent_type` (the subagent's name, e.g. `"Explore"` or
`"security-reviewer"`) + `last_assistant_message` (the final assistant text of the turn). Hooks
should use `last_assistant_message` rather than reading the transcript. SubagentStop CAN block
(exit code 2 prevents the subagent from stopping). [doc]

**Exit code 2 effect by event:**

| Event | Exit 2 effect |
|---|---|
| TaskCreated | Rolls back task creation |
| TaskCompleted | Prevents task from being marked complete |
| TeammateIdle | Prevents teammate from going idle; it continues working |
| SubagentStop | Prevents the subagent from stopping |

The ledger.sh hook proposed in GOSPLAN §3.7 uses SubagentStop and reads `last_assistant_message`
— this design is confirmed sound. The agent_type field confirms that the hook can identify which
builder or gate is stopping. [doc + live]

### Worktree isolation per teammate

Teammates do NOT automatically get their own worktrees. Worktree isolation (`isolation: worktree`
in a subagent definition) applies to subagents. For teammates, the docs describe split-pane mode
as separate Claude Code processes but do not specify automatic worktree creation per teammate.
If two teammates edit the same file, they will conflict. [doc — confirmed as a known limitation:
"Two teammates editing the same file leads to overwrites. Break the work so each teammate owns a
different set of files."]

To use worktrees with teammates: pre-create worktrees manually (git worktree add) and give each
teammate its working directory in the spawn prompt. The teammate can then `cd` into it.

### Permission prompt routing

All permission prompts from teammates surface in the **lead session** for human approval. [doc]
The human must approve them there. Plan approval is the designed exception: the lead session
auto-approves teammate plan approvals without a separate prompt. [doc]

### Session end / /resume

- **In-process teammates:** `/resume` and `/rewind` do NOT restore in-process teammates. After
  resuming, the lead may message teammates that no longer exist. Recovery: spawn new teammates.
  [doc — confirmed limitation]
- **Split-pane teammates:** Not explicitly addressed for resume in the docs, but the task list
  directory persists (governed by `cleanupPeriodDays`). Team config directory is removed at
  session end. [doc]
- **Task list:** Persists locally across sessions. [doc]

### AskUserQuestion from a teammate

The docs do not list `AskUserQuestion` as a teammate or subagent tool. It is available in main
sessions. Whether a teammate can invoke it is **unverified**. Practically: the gosplan design
should route all Planner decision points through the lead session only, which is the documented
safe path. [doc — absence of confirmation]

### Workflow from a teammate

Workflows are triggered by Claude using the `Workflow` tool or the `ultracode` keyword. The docs
state teammate spawning requires an interactive session (`-p` mode suppresses it). Whether an
in-process teammate can itself launch a Workflow is **unverified**. Background subagents from
in-process teammates are explicitly blocked — a teammate's own subagents run foreground. [doc]
This suggests a teammate cannot run a Workflow that spawns background workers.

### Limits (verified against v2.1.178+ live docs)

| Limit | Status |
|---|---|
| One team per session | Confirmed |
| No nested teams (teammates cannot spawn teammates) | Confirmed |
| No background subagents from in-process teammates | Confirmed — returns error |
| Lead is fixed; cannot transfer leadership | Confirmed |
| No per-teammate permission mode at spawn; only after | Confirmed |
| No session resumption with in-process teammates | Confirmed |
| Task status can lag (blocking dependents) | Confirmed as known limitation |
| Split panes require tmux or iTerm2; not VS Code terminal, Windows Terminal, Ghostty | Confirmed |
| Agent Teams unavailable in non-interactive (-p) mode | Confirmed |
| Agent Teams unavailable in VS Code extension (as of #28048) | Bug, open as of Feb 2026 |
| Agent Teams unavailable on Claude Code on the web | Bug, open as of May 2026 (#56449) |

---

## Q2 — Failure modes reported in practice

### #42999 — SendMessage silent name-routing failure [source, April 2026]
https://github.com/anthropics/claude-code/issues/42999  
SendMessage with teammate name silently succeeds but delivers nothing. Only raw agent ID works.
Closed as "not planned." Workaround: PreToolUse hook resolving names to IDs. **This is the most
dangerous failure mode for any orchestration design that relies on inter-agent messaging.**

### #58762 — tmux mailbox routing mismatch [source, May 2026]
https://github.com/anthropics/claude-code/issues/58762  
With `team_name` parameter, agent registers for tmux mailbox but runs as native process. Messages
route to a nonexistent pane; tasks hang indefinitely. Closed as "not planned" (duplicate).
Workaround: omit `team_name` parameter. The live doc confirms `team_name` is now ignored (v2.1.178+),
so this may be resolved by the architecture change — **verify with a probe.**

### #48160 — Subagents cannot SendMessage [source, open]
https://github.com/anthropics/claude-code/issues/48160  
Spawned subagents cannot originate SendMessage even with the flag enabled and name= set.
This affects V2 Race if evidence-auditor is a subagent rather than a full teammate.

### #34750 — Tools not available despite flag [source, March 2026]
https://github.com/anthropics/claude-code/issues/34750  
TeamCreate/TeamDelete/SendMessage not in tool set despite flag. Likely resolved by v2.1.178
architecture (TeamCreate/TeamDelete removed; SendMessage is added automatically). **Verify.**

### #28048 — VS Code extension blocks teams [source, Feb 2026]
https://github.com/anthropics/claude-code/issues/28048  
Teams don't work in VS Code extension. Not relevant if using terminal (kitty/tmux on CachyOS).

### #56449 — Web unavailability [source, May 2026]
https://github.com/anthropics/claude-code/issues/56449  
Not applicable to local CLI use.

### Context blowup in the lead

Each teammate has its own context window, but idle notification messages and task list updates
accumulate in the lead's context. For long-running plans with many teammate turns, the lead's
context will grow. No hard limit documented beyond the model's context window. [doc — inferred]

### Cost

"Agent teams use significantly more tokens than a single session. Each teammate has its own context
window, and token usage scales with the number of active teammates." [doc] The docs explicitly
recommend 3–5 teammates max for practical use.

---

## Q3 — Design variants for GOSPLAN integrated parallel development

### Mapping bd ↔ team task

The bd tracker and the Agent Teams task list are separate systems. Proposed protocol:

1. gosplan (lead) creates both: `bd create` for the macro layer, then `TaskCreate` for the
   in-session coordination layer. The task description references the `bd` issue id.
2. A teammate that claims a task runs `bd update <id> --claim` via Bash at start.
3. At stop: `bd comments add <id> "<findings>" --author <teammate-name>` — this can be enforced
   by the SubagentStop hook reading `agent_type` and `last_assistant_message`.
4. `bd close` is performed by gosplan after reviewing the teammate's output — gosplan holds the
   sha and the proof, consistent with the principle that close evidence is the lead's
   responsibility.
5. `bd export -o .beads/issues.jsonl` runs before the commit hook as today.

`bd swarm` and `bd mol` are unrelated to Agent Teams mechanics; they are bd CLI features for
batching issue creation. They remain useful for pre-populating the bd backlog before a Plan starts.

### V1 — Lane Team

**Structure:** One builder teammate per lane (sim/ui/data) on disjoint files in dedicated
worktrees; a scrum-master in-process as gosplan; lead = coordinator.

**Turn protocol:**

1. gosplan spawns 1–3 builder teammates from subagent definitions, each with a spawn prompt
   containing: bd issue id, verification command, file ownership list, worktree path.
2. Each builder teammate claims its task from the shared list, reads its `bd` issue, implements,
   runs `cargo test -p simulation`, and messages the lead with results.
3. gosplan collects results via idle notifications + direct messages, synthesises, dispatches gate
   agents as ordinary subagents (NOT teammates — gates are sequential and read-only).
4. Human ratifies commits as today.

**File-ownership rules:** Pre-declared in each spawn prompt. gosplan pre-declares the scenario
module in `scenarios/mod.rs` before spawning (the shared-file clobber trap from GOSPLAN §3.6).
Worktrees are created manually by gosplan before spawning: `git worktree add .claude/worktrees/<lane> <branch>`.

**bd source of truth:** bd is the macro layer. The teammate's bd comments are the micro log. The
Agent Teams task list is the in-session coordination layer. Both are valid simultaneously.

**Who writes bd comments:** The teammate (via Bash in the worktree session). Enforced by
SubagentStop hook reading `last_assistant_message`.

**Gates:** Dispatched by gosplan as ordinary subagents after the Lane Team finishes — not as
teammates, because gates are sequential (cheap-to-expensive ordering) and must not see each other's
outputs. The blind parallel review from GOSPLAN §3.5 runs as two subagents (not teammates) via
`gate-review.js` as today.

**Human decision points:** gosplan uses `AskUserQuestion` in the lead session. Teammates do not
ask the human directly.

**Token multiplier:** ~1.5–2× vs. solo build (each builder has its own context; inter-teammate
messaging is minimal in the lane design). Comparable to the Race play without teams.

**Against known limits:**

| Limit | Impact |
|---|---|
| One team per session | Fits: one Plan, one team |
| No nested teams | Fits: gates are subagents, not teammates |
| No background subagents from in-process teammates | Risk: if a builder tries to spawn a background helper. Mitigate: builder definitions restrict `background: true`. |
| No resume | Risk: if lead session dies mid-build. Mitigate: teammate writes progress to bd comments before each major step; resume by reading bd. |
| SendMessage name routing (#42999) | Risk: HIGH — builders messaging gosplan by name may silently fail. Mitigate: PreToolUse hook OR builders write to a shared file instead of SendMessage. |
| Task status lag | Risk: LOW — gosplan polls via idle notification rather than polling task status. |

**Verdict: Buildable today.** The SendMessage bug is the only showstopper; it is mitigatable with
a PreToolUse hook. The lane design minimises inter-teammate messaging, reducing exposure.

### V2 — Race Team

**Structure:** Builder and evidence-auditor launched in parallel from the same brief; builder
writes production code; auditor writes adversarial tests before seeing implementation.

**Can this use Agent Teams?** Only if both are teammates (not subagents), because subagents cannot
send SendMessage (#48160). But the Race play does not actually require messaging between builder
and auditor — they work independently on disjoint files; gosplan integrates via `cargo test -p
simulation`. The Race play is fully implementable as two independent subagents (NOT teammates)
launched in parallel, exactly as GOSPLAN §3.6 specifies: "Live agent-to-agent messaging (Agent
Teams) is not required by any adopted play."

**Verdict: Implement without teams.** The current GOSPLAN design is correct; Agent Teams add
no value and introduce the SendMessage bug risk.

### V3 — Pair Team

**Structure:** Driver/navigator. Builder drives; the cluster's advisor reads each checkpoint diff
and returns PASS or RETURN with the deviation named.

**Can this use Agent Teams?** The navigator sends targeted feedback to the driver at each
checkpoint. This is the one pattern where SendMessage between teammates has clear value — the
navigator's PASS/RETURN verdict is a short structured message, not a file handoff. However:
- The navigator is an advisor (e.g., `kornai-economist`, `logistics-modeller`) — it must remain
  read-only on source. An advisor teammate would have full tool access unless restricted.
- The SendMessage name-routing bug means the navigator's verdict may never arrive.
- The current GOSPLAN design uses "gosplan relays via subagent resume" — this works without teams.

**Verdict: Implement without teams** using the file relay pattern. Navigator writes
`refine_navigator.md` at each checkpoint; gosplan resumes the builder with the contents.
Agent Teams could be revisited for the Pair play in Plan 02 as the §7 D2 probe.

### Summary: which is buildable today

| Variant | Buildable with teams | Notes |
|---|---|---|
| V1 Lane Team | Yes, with #42999 mitigation | SendMessage hook required; worktrees manual |
| V2 Race Team | Not needed — file handoff is correct design | Teams add risk with no gain |
| V3 Pair Team | Not needed — file relay via gosplan is correct design | Teams revisit in Plan 02 |

---

## Q4 — Coordinator roles: teammates vs lead

**Only the lead can spawn teammates.** This is an architectural fact: "Only the lead can manage
the team." Teammates cannot spawn their own teammates. [doc]

Therefore: gosplan MUST be the lead session. A thin "human-facing shell" that spawns both gosplan
and a scrum-master as teammates is impossible — the shell would then be the lead, and it would
need to hold all the Planner decision points. This adds a layer with no benefit.

**The correct architecture:**

```
Human (Planner)
    │ (seven decision points, AskUserQuestion)
    ▼
gosplan (lead session, spawns teammates, holds all coordination)
    │
    ├── builder-teammate-sim (in-process or split-pane)
    ├── builder-teammate-ui  (in-process or split-pane)
    └── [gates dispatched as subagents after team finishes]
```

**A "scrum-master teammate" is not a separate role in this architecture.** gosplan is the
coordinator. DoR and DoD enforcement is via hooks (dor-gate.sh, ledger.sh) and gosplan's own
judgment. Adding a scrum-master teammate adds coordination overhead and another context window
with no new capability.

**AskUserQuestion:** Available in the lead session. Not confirmed available for teammates. Route
all Planner decision points through gosplan (lead). [doc — confirmed for lead; unverified for
teammates]

**Workflow from the lead:** Confirmed available. The gate-review.js workflow is dispatched by
gosplan as today.

**Can a teammate run a Workflow?** Unverified. Given that background subagents from in-process
teammates are blocked, a teammate-launched workflow that fans out to 16 concurrent workers is
likely unsupported. [unverified — treat as blocked until probed]

---

## Q5 — Probe plan

**Goal:** Verify the 5 riskiest assumptions before committing to V1 Lane Team in a real Plan.

**Candidate bd story:** A small M-lane story (e.g., `sov-snw` S-lane or a synthetic probe story).
Budget: ≤150k tokens, ≤3 teammates.

**Assumption 1: SendMessage name routing works (or the mitigation is reliable)**

Risk: HIGH. Confirmed bug in #42999 (closed, not planned).

Probe:
1. Enable `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1`.
2. gosplan spawns one teammate named `probe-worker` using a definition that does one task.
3. Lead sends: `SendMessage("probe-worker", "Report your current status.")`.
4. Teammate replies; lead confirms receipt.

**Pass:** Teammate receives and responds. **Fail:** Lead reports success but no reply.

If fail: implement the `PreToolUse` hook from #42999 workaround and re-probe.

**Assumption 2: .claude/rules/ path-scoped rules load inside a teammate**

Risk: HIGH (same as report 04 gap 6 for subagents, equally unverified for teammates).

Probe (combines the Wave 0 subagent probe from GOSPLAN §4.1 with a teammate variant):
1. `.claude/rules/probe.md` with `paths: ["simulation/**"]` and sentinel: "PROBE-SENTINEL-42".
2. Spawn a teammate, tell it to read `simulation/src/lib.rs` and report whether "PROBE-SENTINEL-42"
   appears in its context.

**Pass:** Teammate reports the sentinel. **Fail:** Sentinel absent — rules don't load in teammates;
use per-lane skills preloaded by definition body instead.

**Assumption 3: Worktree isolation prevents file conflicts in a 2-builder lane team**

Risk: MEDIUM. Documented limit: "Two teammates editing the same file leads to overwrites."

Probe:
1. gosplan creates two worktrees: `git worktree add .claude/worktrees/sim-probe sim-probe-branch`
   and `...ui-probe ui-probe-branch`.
2. Spawn two builder teammates with `cwd` set to respective worktrees.
3. Both write to distinct files; gosplan then merges the branches.

**Pass:** No conflicts; merge succeeds. **Fail:** Branch divergence or clobber.

**Assumption 4: SubagentStop hook receives agent_type correctly for named builder definitions**

Risk: MEDIUM. The ledger.sh hook must identify which builder stopped.

Probe:
1. Implement `scripts/hooks/ledger.sh` as a `SubagentStop` hook that writes `agent_type` and
   `last_assistant_message` to a local file.
2. Dispatch a subagent of type `sim-implementer` (the named definition).
3. Check the log file after stop.

**Pass:** `agent_type` = `"sim-implementer"` in the log. **Fail:** `agent_type` absent or wrong —
adjust the hook to use `transcript_path` fallback.

**Assumption 5: Task tools are available to teammates on the current model**

Risk: MEDIUM. As of v2.1.233+, Task tools are absent on newer model families.

Probe:
1. With `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1`, spawn one teammate.
2. Tell the teammate to run `/list-tools` (or `TaskList`) and report the result.

**Pass:** TaskCreate/TaskList visible. **Fail:** Tools absent — add `CLAUDE_CODE_ENABLE_TODO_TOOLS=1`
to settings or coordinate through SendMessage only.

**Output that proves each assumption:**

| Assumption | Proof |
|---|---|
| A1: SendMessage routing | Teammate reply received; or hook log shows ID resolution succeeded |
| A2: path rules in teammates | Sentinel found in teammate report |
| A3: worktree isolation | `git log --oneline sim-probe-branch ui-probe-branch` shows no shared commit conflicts |
| A4: SubagentStop agent_type | Log file contains `"agent_type": "sim-implementer"` |
| A5: Task tools available | `TaskList` runs without error in teammate |

Total probe cost estimate: ≤150k tokens (3 small teammates, 5 probes, mostly read-only).

---

## Gaps and what would close them

1. **SubagentStop exact stdin schema** (task_id, task_title, teammate_name fields): fetch the
   full hooks reference at https://code.claude.com/docs/llms.txt with a targeted extraction.
   Needed before writing the final ledger.sh hook.

2. **AskUserQuestion in teammates**: run `agent.run("List your available tools")` in a teammate
   and check the output. Needed before any design that routes decision points through a teammate.

3. **Workflow from a teammate**: spawn a teammate and attempt `/deep-research` or `ultracode` from
   within it. Needed only if the Pair play later adopts Agent Teams.

4. **.claude/rules/ in teammates**: the Wave 0 probe (GOSPLAN §4.1) covers subagents; extend it
   to a teammate session (Probe A2 above).

5. **#42999 status on current version** (v2.1.178+): the bug was filed at v2.1.92. The mailbox
   architecture changed significantly in v2.1.178. Probe A1 determines whether it is still live.

6. **`skills:` in teammates**: the live doc explicitly says skills are NOT applied from a
   definition to a teammate. This contradicts report 04 which cited skills preload as a mechanism.
   The live doc is authoritative. Confirmed closed gap — don't rely on skills preload for teammates.

---

## Sources

- [doc] https://code.claude.com/docs/en/agent-teams.md (fetched 2026-08-28, v2.1.178+)
- [doc] https://code.claude.com/docs/en/hooks (fetched 2026-08-28)
- [doc] https://code.claude.com/docs/en/tools-reference (fetched 2026-08-28)
- [doc] https://code.claude.com/docs/en/hooks-guide.md (fetched 2026-08-28)
- [source] https://github.com/anthropics/claude-code/issues/42999 (April 2026, closed not planned)
- [source] https://github.com/anthropics/claude-code/issues/58762 (May 2026, closed not planned)
- [source] https://github.com/anthropics/claude-code/issues/48160 (open)
- [source] https://github.com/anthropics/claude-code/issues/34750 (March 2026, likely resolved by v2.1.178)
- [source] https://github.com/anthropics/claude-code/issues/28048 (Feb 2026, VS Code only)
- [source] https://github.com/anthropics/claude-code/issues/56449 (May 2026, web only)
- [live] `.planning/process-overhaul-2026-08-28/04-claude-code-orchestration.md` (baseline, 2026-08-28)
