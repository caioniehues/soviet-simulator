# Claude Code Orchestration Primitives — Live Map (2026-08-28)

This document maps every orchestration primitive Claude Code offers today. All claims are cited to live documentation fetched 2026-08-28.

---

## 1. Subagents

**Reference:** https://code.claude.com/docs/en/sub-agents.md

### Frontmatter Fields

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `name` | string | Yes | Unique identifier (lowercase, hyphens only); enables `@-mention` invocation and `SendMessage` addressing |
| `description` | string | Yes | When Claude should delegate to this subagent; displayed in CLI autocomplete |
| `tools` | array of strings | No | Allowlist of tools. Omitting means inherit all. Syntax: `Read, Glob, Grep` or `Agent(worker, researcher)` for spawnable subagents |
| `model` | string | No | `sonnet`, `opus`, `haiku`, `inherit` (default). Picked by: `CLAUDE_CODE_SUBAGENT_MODEL` env > spawn prompt > definition > lead model |
| `permissionMode` | string | No | `default`, `acceptEdits`, `auto`, `dontAsk`, `bypassPermissions`, `plan`. Subagent inherits lead's mode except where explicitly set |
| `skills` | array of strings | No | **Pre-loaded skills into subagent context.** Directory names, e.g., `["my-skill", "../shared-skill"]`; relative to agent definition file |
| `memory` | string | No | `user`, `project`, or `local`. Enables persistent auto-memory for this subagent across sessions in a separate directory |
| `mcpServers` | array of objects | No | MCP servers available only to this subagent. Each entry: `{name: "server-name", url: "..."}` or command format |
| `maxTurns` | integer | No | Stop after N turns; Claude can later resume the subagent |
| `isolation` | string | No | `worktree` to isolate in a separate git worktree under `.claude/worktrees/` |
| `hooks` | object | No | Lifecycle hooks for tool validation, same format as settings.json |
| `color` | string | No | (mentioned in docs as available field, but specific values not documented) |
| `initialPrompt` | string | No | (mentioned in docs as available field, purpose not fully detailed) |
| `background` | boolean | No | Run in background by default; subagents respect lead's `/bg` commands and can't spawn background subagents |

### Tool Inheritance & Allowlist Semantics

- **Omit `tools:`** → subagent inherits all tools from lead
- **`tools: [Read, Glob]`** → allowlist only those; subagent cannot use Write, Edit, Bash, etc.
- **`tools: [Agent(worker)]`** → allowlist spawning of `worker` subagent type; subagent itself is not an Agent tool until spawned
- **Subagents cannot have:** `ListAgents`, `SendMessage` (added by agent-teams code, not user-configurable in definition)
- **Foreground subagents** get full tool access minus those explicitly denied
- **Background subagents** get a reduced tool set (exact set not fully specified in public docs, but excludes interactive approval)

### Execution Modes & Nesting

- **Foreground:** blocks main conversation, prompts pass through from lead
- **Background (default):** runs concurrently; permission prompts surface in main session; cannot spawn background children (they run foreground instead)
- **Fork:** inherits full conversation history + tools for side tasks; separate session ID from parent
- **Nesting:** subagents can spawn subagents up to 3 layers deep (default, configurable via runtime)
- **Resume:** Claude can resume a completed subagent with `"Continue the code review and analyze the database layer"`

### Context Inheritance

Non-fork subagents start fresh with:
- Their own system prompt
- CLAUDE.md files and git status (except Explore/Plan built-in subagents)
- Preloaded `skills` (if specified in definition)
- Task delegation message
- **Do NOT see:** conversation history, main session's output style, main session's auto memory

Fork subagents inherit:
- Full parent conversation history
- Same context window and tools

### Where to Define

1. `.claude/agents/<name>.md` — project-specific (checked into version control)
2. `~/.claude/agents/<name>.md` — user-scope (all projects)
3. `--agents` CLI flag with JSON — session-only

### Built-in Subagents

- `Explore` — fast, read-only codebase search (capped at Opus even if session uses stronger model)
- `Plan` — read-only research for plan mode
- Disable with: `permissions.deny: ["Agent(Explore)"]`

### Verification

- Run `/list-agents` to see available subagents
- Use `@"subagent-name (agent)"` for guaranteed delegation

---

## 2. Agent Teams / Teammates (Multi-Session Collaboration)

**Reference:** https://code.claude.com/docs/en/agent-teams.md

### What Exists — Requires Opt-In

**Status:** EXPERIMENTAL, disabled by default  
**Enable:** Set `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1` in `settings.json` `env` or shell environment

When enabled:
- Claude spawns teammates automatically when you ask for a team or when Claude names a subagent
- Teammates are separate Claude Code sessions that coordinate through a shared task list and messaging

### Key Primitives

#### Spawn a Team

```text
Spawn 3 teammates to review PR #142:
- One focused on security
- One checking performance  
- One validating tests
```

Claude decides count; you can name specifics. Models follow: `CLAUDE_CODE_SUBAGENT_MODEL` env > spawn prompt > lead's model.

#### SendMessage / ListAgents

- `ListAgents`: discover teammates, local sessions, cloud sessions, Remote Control sessions
- `SendMessage(<name>, message)`: message a teammate by name; teammate can reply; subagents spawn with `SendMessage` auto-added

#### Shared Task List

- Teammates claim tasks from a shared list
- Dependencies supported: `bd dep <blocker-id> --blocks <blocked-id>`
- Leads can assign or teammates self-claim
- Completion auto-unblocks dependents

#### Team Lead vs. Teammates

- **Lead:** main session, spawns teammates, coordinates work, receives idle notifications
- **Teammates:** peer sessions with independent contexts, message each other directly, self-coordinate through task list
- **Idle notification:** teammate sends to lead when it goes idle (finished a turn with nothing queued)

#### Permission Modes

- Teammates inherit lead's permission mode at spawn
- Can change individual teammate modes after spawning
- Cannot set per-teammate modes at spawn time
- Permission prompts surface in lead session; teammate prompts aren't approved by teammates

#### Display Modes

1. **In-process (default):** all teammates in one terminal, use arrow keys to select, Enter to view
2. **Split panes (tmux or iTerm2):** each teammate in its own pane
3. **Auto:** split panes if in tmux or iTerm2, else in-process

Set via `--teammate-mode` flag or `settings.json` `teammateMode` key.

### Pairing / Competing Hypotheses

Multiple teammates can work on the same problem:
- Each teammate investigates a different angle/hypothesis
- Teammates message each other to debate findings
- Lead synthesizes when done

Example: "Spawn 5 teammates with competing hypotheses to debug why users get disconnected; have them argue theories with each other."

### Broadcast & Announce

- No built-in broadcast; send individual `SendMessage` to each teammate by name
- Lead can announce via plain prompt; teammates read it and respond

### Teammate Idle Behavior

- Teammate stops after finishing a turn (no pending work, no background tasks)
- Notifies lead with `notify_when_idle` (one-shot, expires after 12 hours if not delivered)
- In-process teammates hide their row after 30 seconds idle if others are still working

### Limits (Known)

- **One team per session:** a session has exactly one team, scoped to that session
- **No session resumption with in-process teammates:** `/resume` does not restore in-process teammates
- **No nested teams:** teammates cannot spawn their own teammates
- **No background subagents from in-process teammates:** `background: true` fails in teammate subagents
- **Task status can lag:** teammates may fail to mark completion, blocking dependents
- **Team directories:** `~/.claude/teams/{team-name}/` (cleaned up at session end) + `~/.claude/tasks/{team-name}/` (persists for resumed sessions)

---

## 3. Workflows (Dynamic Orchestration)

**Reference:** https://code.claude.com/docs/en/workflows.md

### What It Is

A JavaScript file that orchestrates many subagents at scale. Claude writes the script; a runtime executes it in background while session stays responsive.

**Requirements:** Claude Code v2.1.154+; available on all paid plans, Anthropic API, Bedrock, Agent Platform, Foundry

### Triggering Workflows

1. **Ask in prompt:** include keyword `ultracode` or say "use a workflow"
   ```text
   ultracode: audit every API endpoint under src/routes/ for missing auth checks
   ```

2. **Let Claude decide:** set `/effort ultracode`
   - Claude plans a workflow for every substantive task
   - Combines `xhigh` reasoning effort with auto-orchestration
   - More tokens, longer time per task

3. **Run bundled workflow:**
   ```text
   /deep-research What changed in Node.js permissions between v20 and v22?
   ```

4. **Run saved workflow:**
   ```text
   /my-saved-workflow argument
   ```

### Script API

Plain JavaScript with top-level `await`:

```javascript
export const meta = {
  name: 'audit-routes',
  description: 'Audit every route for missing auth',
}

// Spawn one agent, get structured result
const found = await agent('List every .ts file under src/routes/.', {
  schema: { type: 'object', required: ['files'], properties: { files: { type: 'array', items: { type: 'string' } } } },
})

// Run one agent per item in parallel
const audits = await pipeline(found.files, file =>
  agent(`Audit ${file} for missing authentication checks.`, { label: file }),
)

return audits.filter(Boolean)
```

**Functions:**
- `agent(prompt, options)` → spawns one subagent; resolves to result or `null` if stopped/error
- `pipeline(items, async fn)` → runs `fn` once per item in parallel; returns array of results
- `parallel(...tasks)` → runs multiple async tasks concurrently
- `phase(name, fn)` → groups work into logical phases (for UI display)
- `args` → global containing input passed at invocation time

**Options on `agent()`:**
- `schema` — JSON Schema for structured output
- `label` — display name in progress view
- `model` — override session model for this agent
- `tools` — restrict tools available to this agent

### Approval & Permissions

**In interactive sessions:**
- First run: approval dialog shown
- Approved: saved to user settings, skipped on later runs
- You can select "View raw script" before approving

**In `claude -p` / Agent SDK:**
- Runs via `Workflow` tool call
- Permission rules applied: `Workflow` in allow rules, or `Workflow(<name>)` for specific saved workflows
- Auto mode classifier can approve
- Bypass permissions mode auto-approves
- `PreToolUse` hook can approve

### Save for Reuse

Run `/workflows`, select a completed run, press `s`:
- `.claude/workflows/` — project-level (shared via repo)
- `~/.claude/workflows/` — personal (all projects)

Workflow runs as `/<name>` in future sessions. In monorepos, saves to closest existing `.claude/workflows/` directory.

### Input via `args`

Pass input to saved workflow at invocation:
```text
Run /triage-issues on issues 1024, 1025, 1030
```

Structured as JSON; script accesses as global `args` with `args.length`, `args[0]`, etc.

### Runtime Constraints

| Constraint | Limit |
|-----------|-------|
| Concurrent agents | 16 (fewer on low-CPU hosts / containers) |
| Total agents per run | 1,000 |
| No mid-run user input | Only permission prompts pause run |
| No `import()` in script | Must be plain JS |
| Max TTL (hold before first release) | 5 seconds (`CLAUDE_CODE_WORKFLOW_PREFIX_STAGGER_MS=5000` default) |

### Prompt Caching in Fan-Outs

Agents in same run can read each other's prefix cache:
- Same model + effort + agent type + tools + schema + cwd → shared prefix
- By default cache lasts 5 minutes; set `subagentPromptCacheTtl: 1h` for 1 hour (higher billing)

### Resume After Pause

`/workflows` → select paused run → `p`:
- Completed agents return cached results (unless prompt changed)
- Failed agents rerun, along with all agents after them
- Editing script → relaunch reruns first agent with different prompt + all after

### Cost Tracking

- Each agent uses your session's model (or `CLAUDE_CODE_SUBAGENT_MODEL` override)
- `Large workflow` warning at >25 agents or 1.5M projected tokens (advisory only)
- `/workflows` view shows per-agent token usage in real time
- Runs stop when cost ceiling hit (exit code 2)

---

## 4. Hooks

**Reference:** https://code.claude.com/docs/en/hooks-guide.md (96KB+, includes full reference at `/docs/en/hooks`)

### Event Types

Complete list of hook events:

| Event | When | Can Block? | Runs in Subagents? |
|-------|------|-----------|-------------------|
| `PreToolUse` | Before a tool call | **Yes** (exit 2 blocks) | Yes |
| `PostToolUse` | After tool result returns | No | Yes |
| `UserPromptSubmit` | Before processing user input | **Yes** (exit 2 blocks) | No (lead only) |
| `Stop` | Session ending | No | No |
| `SubagentStop` | Subagent stopping | No | No |
| `SessionStart` | Session initializing | **Yes** (exit 2 blocks) | No (fires before subagent starts) |
| `SessionEnd` | Session exiting | No | No |
| `PreCompact` | Before `/compact` | No | No |
| `Notification` | Internal event | No | No |
| `TaskCompleted` | Task marked complete (agent teams) | **Yes** (exit 2 blocks) | No |
| `TeammateIdle` | Teammate about to idle (agent teams) | **Yes** (exit 2 blocks) | No |
| `InstructionsLoaded` | CLAUDE.md files loaded | No | No (fires at startup) |
| `TaskCreated` | Task created (agent teams) | **Yes** (exit 2 blocks) | No |
| `PermissionRequest` | Permission prompt (Agent SDK) | **Yes** (exit 2 blocks) | No |

### Hook Configuration

Place in `settings.json` (any scope: user, project, local, managed):

```json
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "command",
            "command": "./validate-bash.sh"
          }
        ]
      }
    ]
  }
}
```

### Matcher Types

- **Tool name:** `"Bash"`, `"Read"`, `"Write"`, `"Edit"`, `"Bash:curl"`
- **Argument patterns:** match specific tool arguments
- **Wildcard patterns:** match multiple tools

### Exit Codes & Blocking

- **Exit 0:** hook succeeded, allow operation
- **Exit 1:** hook failed, show error, block operation
- **Exit 2:** hook wants to send feedback/decision; include JSON output with:
  ```json
  {
    "decision": "allow|block|ask",
    "reason": "human-readable reason",
    "additionalContext": "optional extra detail"
  }
  ```

### Hook Input Format

Hooks receive JSON via stdin with:
- `tool`: tool name
- `arguments`: tool's input args
- `context`: session state (model, transcript_path, working_directory, etc.)

### Which Events Block?

Events marked "Can Block? Yes" can exit 2 to prevent the action:
- `PreToolUse` → block tool call
- `UserPromptSubmit` → block prompt processing
- `SessionStart` → block session start
- `TaskCompleted`, `TeammateIdle`, `TaskCreated` → gate quality

### Subagent Behavior

- Most hooks fire in subagents (`PreToolUse`, `PostToolUse`, `SessionStart` of the subagent)
- Lead-only events: `UserPromptSubmit`, `Stop`, `SubagentStop`, `SessionEnd`, `PreCompact`

---

## 5. Skills (SKILL.md)

**Reference:** https://code.claude.com/docs/en/skills.md

### Frontmatter

| Field | Type | Purpose |
|-------|------|---------|
| `name` | string | Skill name; creates `/skill-name` command |
| `description` | string | When Claude should invoke this skill; shown in autocomplete |
| `context` | string | `fork` to inherit parent conversation context |
| `agent` | string | (mentioned as available; purpose not fully detailed in public docs) |
| `model` | string | Override model for this skill (e.g., `sonnet`) |
| `allowed-tools` | array | Restrict tools available when running skill (e.g., `["Read", "Bash"]`) |
| `user-invocable` | boolean | Allow user to run `/skill-name` explicitly; default true |
| `disable-model-invocation` | boolean | Claude cannot invoke; only user can run `/skill-name` |
| `argument-hint` | string | Describe expected input, shown in CLI help |

### Directory Structure

```
.claude/skills/my-skill/
├── SKILL.md          # Frontmatter + body
├── helper.sh         # Supporting files
└── data.json
```

Or single file: `.claude/skills/my-skill.md`

### Progressive Disclosure

- Skill body loads only when invoked or when Claude determines it's relevant
- Description is always loaded (consumed into context)
- Long reference material costs nothing until used

### Directory-Scoped Skills

Skills in a subdirectory automatically load when Claude reads files in that directory:

```
src/auth/
├── CLAUDE.md
└── .claude/skills/auth-helper/SKILL.md
```

When Claude reads `src/auth/**`, the skill is available.

### Preloading into Agents

Subagent definition can preload skills:

```yaml
---
name: my-agent
skills: ["my-skill", "../shared-skill"]
---
```

Relative to the agent definition file. Preloaded skills are always in context (not lazy).

### Custom Commands

Merged into skills as of recent versions:
- `.claude/commands/deploy.md` → creates `/deploy` command
- `.claude/skills/deploy/SKILL.md` → also creates `/deploy` command

Both work identically.

### User vs. Claude Invocation

- `user-invocable: true` (default) → user can run `/skill-name`
- `disable-model-invocation: true` → Claude cannot invoke, only user can
- `context: fork` → skill inherits parent conversation context

---

## 6. Memory (CLAUDE.md, Auto-Memory, Agent Memory)

**Reference:** https://code.claude.com/docs/en/memory.md

### CLAUDE.md Hierarchy & Load Order

Files load in this order (root to working directory):

1. **Managed policy:** `/Library/Application Support/ClaudeCode/CLAUDE.md` (macOS) / `/etc/claude-code/CLAUDE.md` (Linux) / `C:\Program Files\ClaudeCode\CLAUDE.md` (Windows) — cannot be excluded
2. **User instructions:** `~/.claude/CLAUDE.md`
3. **Project instructions:** `./CLAUDE.md` or `./.claude/CLAUDE.md`
4. **Local instructions:** `./CLAUDE.local.md` (typically gitignored)
5. **Nested CLAUDE.md** in subdirectories load on-demand when Claude reads files there
6. **`.claude/rules/`** — path-scoped rules load when matching files are read

All discovered files concatenate into context (not override).

### @Imports

Import files using `@path/to/import` syntax:

```markdown
See @README for overview and @package.json for commands.

# Rules
@docs/git-instructions.md
```

Relative paths resolve relative to the importing file. Max depth: 4 hops. Supports recursive imports.

### .claude/rules/ Directory

Organize instructions into topic files:

```
.claude/rules/
├── code-style.md
├── testing.md
└── security.md
```

Rules without `paths:` frontmatter load at launch. Rules with `paths: ["src/**/*.ts"]` load on-demand.

### Auto-Memory (MEMORY.md)

Claude accumulates learnings automatically. Stored per repository in `~/.claude/projects/<project>/memory/`:

```
memory/
├── MEMORY.md              # Index, max 200 lines or 25KB loaded at startup
├── user_role.md           # Memory file
├── feedback_testing.md    # Memory file
└── ...
```

**Types:**
- `user` — your role, expertise, preferences
- `feedback` — corrections you gave Claude, confirmed approaches
- `project` — ongoing work, deadlines, decisions not in git history
- `reference` — external info (issue tracker URLs, dashboards)

**Enable/Disable:** `/memory` command or `autoMemoryEnabled` setting

**Storage:** `~/.claude/projects/<project>/memory/MEMORY.md` + topic files; persists across sessions but is machine-local (not synced)

### Agent Memory (Subagent-Specific)

Subagent frontmatter: `memory: user | project | local`

Each scope gets its own directory:
- `user` — shared across all projects
- `project` — shared within repo (same as lead's auto-memory directory)
- `local` — scoped to lead session's auto-memory

Subagent memory is separate from lead's auto-memory and is not loaded into the lead's context.

### /memory Command

Lists all CLAUDE.md and auto-memory files; select to open in editor. Allows toggle of auto-memory on/off.

### Limits & Enforcement

- CLAUDE.md max loaded size: 4 MiB full file (skipped if larger); 200 lines or 25KB read limit per CLAUDE.md at startup
- MEMORY.md: 200 lines or 25KB read limit; if over, Claude Code returns error and Claude rewrites to comply
- Topic files (`user_role.md`, etc.) load on-demand, no read limit
- Block-level HTML comments in CLAUDE.md are stripped

---

## 7. Sessions & Scheduling

**Reference:** https://code.claude.com/docs/en/sessions.md, `/scheduled-tasks`

### Session Resume

| Command | Behavior |
|---------|----------|
| `claude --continue` | Resume most recent interactive session in cwd |
| `claude --resume` | Open session picker |
| `claude --resume <name>` | Resume named session directly |
| `claude --from-pr <number>` | Filter picker to sessions linked to PR |
| `/resume` | Switch to different conversation inside active session |

**Restoration on resume:**
- Full conversation history
- Model (except if retired or blocked by `availableModels`)
- Permission mode (terminal only; `-p` resets to default)
- Active goal state
- Scheduled tasks (if not expired)

**Not restored:**
- `--mcp-config`, `--settings`, `--plugin-dir`, `--fallback-model`, `--add-dir` flags (pass again)
- Configuration from CLI (re-read from settings files)

### Headless Mode (`claude -p`)

Run non-interactive:

```bash
claude -p "describe the architecture"
claude -p --resume <session-id> "what's next?"
claude -p --output-format json "summarize the changes" | jq -r '.result'
```

**Permissions:** starts in permission mode a new `-p` would use (usually auto if classifier available, else manual); `--permission-mode` or `--dangerously-skip-permissions` override

**Sessions:** `-p` sessions are NOT shown in interactive session picker; resume only via `claude -p --resume <session-id>`

### /loop & Scheduled Tasks

**`/loop <interval> <prompt>`** — run prompt on cron-like interval:
```text
/loop 5m "run the test suite and report results"
/loop daily "check the CI status"
```

Intervals: `30s`, `5m`, `daily`, cron expressions. Expires after 7 days.

**`ScheduleWakeup(sessionId, delay)` & `CronCreate()`** — Agent SDK primitives for scheduling

### Cloud Sessions & Remote Control

- Sessions on **Claude Code on the web** (claude.ai cloud environment)
- Connect via `/remote-control` and message from CLI
- `ListAgents` shows cloud sessions while connected to Remote Control
- `SendMessage` can reach cloud sessions
- Messages travel through Anthropic servers (not local socket)

### Checkpointing & Rewind

- `/rewind` — rewind to earlier checkpoint
- `fileCheckpointingEnabled` setting — snapshots files for `/rewind` recovery
- Checkpoints auto-created at key points

### Worktrees & Isolation

- Git worktrees under `.claude/worktrees/`
- Background sessions auto-move into worktrees before editing
- `isolation: worktree` in subagent definition isolates subagent
- Disable isolation: `worktree.bgIsolation: "none"` in settings

---

## 8. Agent SDK (Programmatic Orchestration)

**Reference:** https://code.claude.com/docs/en/agent-sdk/typescript (and Python equivalent)

### Package Names

- **TypeScript:** `@anthropic-ai/claude-agent-sdk`
- **Python:** `claude-agent-sdk`

### Key Distinction from Claude API Tool Runner

The Agent SDK is a **full harness** you host; includes:
- Agent loop (turn-by-turn orchestration)
- Built-in tools: Read, Write, Edit, Bash, Glob, Grep, WebSearch, WebFetch, Bash
- Session management and context window
- Hooks, permissions, MCP integration
- Subagents and teams support
- Workflows

The **Claude API Tool Runner** (`client.beta.messages.tool_runner`) loops over **your custom tools only** — no built-in tools, but has per-turn hooks for approval, error interception, result modification.

**Managed Agents** (Anthropic-hosted) is separate: server-hosted stateful sessions with a sandbox.

### SDK Features

- Create agent instances programmatically
- Send messages, receive structured results
- Receive events: `message`, `toolUse`, `toolResult`, `stop`
- Built-in tools available without you defining them
- Permission callbacks (`canUseTool`)
- Hook support (PermissionRequest, etc.)
- MCP server integration
- Sessions & persistence

### Example (TypeScript)

```typescript
import { Agent } from "@anthropic-ai/claude-agent-sdk";

const agent = new Agent({
  model: "claude-opus-4-1",
  tools: ["Read", "Bash", "WebSearch"],
});

for await (const event of agent.run("audit src/ for security issues")) {
  if (event.type === "message") console.log(event.content);
  if (event.type === "toolUse") console.log(`Using ${event.toolName}`);
}
```

### Permissions in SDK

- `canUseTool` callback: `(toolName: string, toolInput: any) => boolean | Promise<boolean>`
- `PermissionRequest` hook: can block tool calls
- Same permission system as CLI

---

## 9. Settings (Delegation & Team-Related)

**Reference:** https://code.claude.com/docs/en/settings-reference.md

### Key Settings for Delegation & Teams

| Setting | Scope | Purpose |
|---------|-------|---------|
| `agent` | Any file | Start sessions as named subagent; picks tool set, model, system prompt from definition |
| `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1` | `env` in settings / shell | Enable agent teams (experimental) |
| `permissions.defaultMode` | User or managed | Default permission mode: `default`, `auto`, `acceptEdits`, `plan`, `dontAsk`, `bypassPermissions` |
| `permissions.allow` | Any file | Auto-approve listed tool uses: `["Read", "Bash", "Workflow"]` |
| `permissions.deny` | Any file | Block listed tool uses |
| `permissions.ask` | Any file | Always prompt for these tools |
| `crossSessionInbound` | Any file | Control incoming peer messages: `accept`, `hold`, `refuse` |
| `isolatePeerMachines` | Any file | Require approval before messages leave this machine |
| `autoMemoryEnabled` | Any file | Turn auto-memory on/off |
| `autoMemoryDirectory` | Any file | Custom directory for auto-memory storage |
| `skillOverrides` | Any file | Hide/collapse skills without editing SKILL.md |
| `enabledMcpjsonServers` | Project or local | Approve specific `.mcp.json` servers |
| `disabledMcpjsonServers` | Project or local | Reject specific `.mcp.json` servers |
| `enableAllProjectMcpServers` | Project or local | Auto-approve all project MCP servers |
| `workflowSizeGuideline` | Any file | Guide workflow agent count: `small`, `medium`, `large`, `unrestricted` |
| `disableWorkflows` | Any file | Turn off workflows entirely |
| `claude -p` / headless mode | CLI | Non-interactive, uses API model default permissions |
| `--permission-mode` | CLI flag | Override permission mode at launch |
| `--dangerously-skip-permissions` | CLI flag | Bypass all permission checks |
| `--fork-session` | CLI flag | Branch conversation (copy + new ID) |
| `--add-dir <path>` | CLI flag | Grant access to additional directories outside cwd |

### Precedence Rules

Settings precedence (highest to lowest):
1. **CLI flags** (`--settings`, `--model`, etc.)
2. **Managed settings** (organization-deployed)
3. **Local settings** (`.claude/settings.local.json`)
4. **Project settings** (`.claude/settings.json`)
5. **User settings** (`~/.claude/settings.json`)
6. **Built-in defaults**

Within a file, more specific keys override general ones.

---

## 10. Known Limits (Verified Against Live Docs)

### Subagents

- **No LSP:** subagents do not have language server; resolve symbols in main session
- **No ListAgents:** subagents cannot call `ListAgents` (only lead can)
- **tools: allowlist only narrows:** omitting `tools:` gives all; listing tools restricts to that set
- **New agent files not hot-loaded:** restart session to load new `.claude/agents/*.md` files
- **TaskCreate/TaskUpdate/TaskList unavailable in subagents:** only available to lead session (with Task tools enabled)

### Agent Teams

- **Requires opt-in:** `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1`
- **Experimental limitations:**
  - No session resumption with in-process teammates (split-pane OK)
  - Task status can lag
  - No nested teams (teammates cannot spawn teammates)
  - No background subagents from in-process teammates

### Workflows

- **Max 1,000 agents per run** (prevents runaway loops)
- **Up to 16 concurrent agents** (fewer on low-CPU hosts)
- **No mid-run user input** (only permission prompts pause)
- **No `import()` in script** (plain JavaScript only)
- **`args` input only on saved workflows** (not on ad-hoc `ultracode` runs)

### Hooks

- **No async hooks** (all run synchronously; stdin/stdout only)
- **Subagents can fire PreToolUse/PostToolUse** but not UserPromptSubmit, Stop, SessionEnd

### Memory

- **CLAUDE.md read limit:** 200 lines or 25KB per file at startup (full file still processed, but only first limit worth is loaded)
- **Auto-memory index limit:** 200 lines or 25KB for MEMORY.md; files over that limit trigger error asking Claude to rewrite
- **Machine-local only:** auto-memory does not sync across machines

### Sessions

- **No cross-repo resumption with `-p`:** `claude -p --resume <session-id>` only works if session is in a worktree of current repo
- **Permission mode reset on `-p` resume:** unless `--permission-prompt-tool` AND other conditions met

### Teams

- **One team per session** (not multiple named teams)
- **Lead is fixed** (cannot transfer leadership)

---

## Orchestration Menu

| Primitive | Primary Use Case | When to Use | Limit |
|-----------|------------------|------------|-------|
| **Subagents** | Focused, delegated work; read-only research | One task per agent; results feed back to lead | ~3-5 practical (more → context bloat) |
| **Agent Teams** | Parallel, peer-coordinated exploration | Multiple independent angles on same problem | Experimental; ~3-5 teammates practical |
| **Workflows** | Many agents at scale; repeatable orchestration | >5 agents; fan-out-collect-verify patterns | 16 concurrent, 1000 total per run |
| **Hooks** | Automation at fixed lifecycle points | Format code, validate commands, gate quality | Synchronous only; runs before/after tool calls |
| **Skills** | Packaged, reusable procedures | Multi-step instructions; context-on-demand | Lazy-loaded; no per-skill limit |
| **Memory (CLAUDE.md)** | Persistent project context | Build instructions, conventions, architecture | <200 lines per file for adherence |
| **Auto-Memory** | Accumulate learnings session-to-session | Preferences, corrections, project context Claude derives from code | <200 lines in MEMORY.md index; topic files unlimited |
| **Scheduled Tasks** | Recurring work | Check CI, run tests on schedule | 7-day expiry; interval-based only |
| **`claude -p` (headless)** | Non-interactive automation | Scripting, CI/CD integration, batches | Loses interactive approval UI |

---

## Docs Gaps

1. **Agent SDK full API reference:** TypeScript/Python SDK pages exist but are sparse on examples. No comprehensive API surface documented.
2. **Exact built-in tools in background subagents:** docs say "reduced tool set" but don't list which tools are unavailable.
3. **Workflow `args` JSON schema:** docs don't show the exact shape of structured `args` passed to saved workflows.
4. **Hooks in agent teams:** whether `TeammateIdle` and `TaskCompleted` hooks can return decisions (exit code 2); no example.
5. **MCP in subagents:** `mcpServers:` field exists but no examples or full specification.
6. **Subagent context merging:** when a subagent loads CLAUDE.md, exactly which project-level rules load?
7. **Workflow agent quota on Bedrock/Vertex:** unclear if 16-concurrent and 1000-total limits apply on cloud providers.
8. **Managed settings enforcement:** which keys prevent override vs. which allow exceptions per-scope?
9. **Team task dependencies:** exact format of `bd dep` and how resolution works in multi-teammate scenarios.
10. **Subagent model aliasing:** when `CLAUDE_CODE_SUBAGENT_MODEL=sonnet` but only Sonnet 3.5 is allowed, which version spawns?

---

## Summary

Claude Code offers **ten major orchestration primitives** as of 2026-08-28:

1. **Subagents** — isolated workers within one session; rich tool control; lazy context loading
2. **Agent Teams** — experimental peer sessions with shared task list; broadcast/announce patterns
3. **Workflows** — fan-out-collect-verify at scale; repeatable scripts; 16 concurrent agents
4. **Hooks** — deterministic automation at lifecycle points; blocking quality gates
5. **Skills** — packaged procedures; progressive disclosure; preloadable into agents
6. **CLAUDE.md** — persistent instructions; hierarchy supports monorepos; path-scoped rules
7. **Auto-Memory** — Claude's learnings; session-to-session knowledge accumulation
8. **Cross-Session Messaging** (SendMessage/ListAgents) — reach other sessions, subagents, teammates
9. **Scheduled Tasks** — `/loop` and cron-like invocation; 7-day expiry
10. **Sessions & Resumption** — save/restore, branching, headless `-p` mode, checkpointing

A team process can layer these: **use workflows for large audits**, **subagents for focused workers**, **agent teams for competing hypotheses**, **hooks for gates**, **skills for repeatable procedures**, **CLAUDE.md for persistent context**, **auto-memory for learnings**. The combination gives a single human lead + N subagents the orchestration surface of a small team.

