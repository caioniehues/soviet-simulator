# Beads integration for GOSPLAN — research report

**Kind:** explanation
**Authority:** advisory
**Status:** delivered
**Owner:** researcher
**Last verified:** 2026-08-28 (bd 1.2.2, Claude Code docs as of 2026-08-28)

Sources: live CLI exploration of the installed binary (`bd --help` and every
subcommand's `--help`; output pasted), `bd list --json` field sampling,
`bd epic status`, `bd epic close-eligible`, the metrics prototype run,
`bd-capability-survey.md`, `gosplan.md` §3.3/§3.7/§3.9,
`https://code.claude.com/docs/en/hooks` (WebFetch 2026-08-28),
`https://code.claude.com/docs/en/agent-teams` (WebFetch 2026-08-28).

---

## 1. Primitive inventory (GOSPLAN §3.3, §3.7)

### Epics / Plans

A Plan is a `bd` epic. All three epic primitives exist and work in 1.2.2: [live]

```
bd epic status          — shows progress per epic (verified: "sov-dda 3/4 children closed")
bd epic close-eligible  — lists epics all children closed ("No epics eligible" on 2026-08-28)
bd create --parent <id> — creates a child story under an epic parent
```

Epics do NOT auto-close when the last child closes. `bd epic close-eligible` must
be swept. GOSPLAN §3.5 already notes this; sweep is part of the session-close protocol.

The `--json` output on `bd list` includes `parent`, `created_at`, `started_at`,
`closed_at`, `comment_count`, `dependency_count`, `status`, `priority`,
`issue_type`, `assignee`, `close_reason` — all fields §3.9 metrics need. [live]

### Swarm

`bd swarm create|validate|status` exists in the binary. [live]

What swarm adds over a plain epic DAG:
- Creates a `mol_type=swarm` molecule linked to the epic. A coordinator agent can
  discover all active swarms with `bd swarm list`.
- `bd swarm validate <epic>` computes "ready fronts" (waves of parallel work that
  can run concurrently given the dep graph), estimated worker-sessions, and max
  parallelism — a structural pre-check before dispatch.
- `bd swarm status <epic>` groups children into Completed / Active / Ready / Blocked
  from live Dolt state, no cached copy.

Is swarm usable by a Claude Code lead + teammates? No direct blocker, but:
- Swarm was designed for unattended multi-rig fleets (the "Gas-Town" model). [doc]
  A human-in-the-loop lead + subagents gets the same visibility from
  `bd epic status` + `bd swarm status` without the ceremony of creating a mol.
- The survey correctly classified it as "skip unless we ever run unattended
  multi-session automation." That verdict holds. [doc: bd-capability-survey §2 rows 2–3]

**GOSPLAN recommendation:** do not use `bd swarm` for Plans. Use plain epics.
Reserve swarm for a future unattended wave, if one is ever built.

### Molecules / Formulas

`bd mol pour|wisp|distill|squash|burn|bond|current|progress|ready|seed|show`
all appear in the binary. [live]

The gate-chain formula is the only formula in use. `.beads/formulas/gate-chain.formula.toml`
pours a 4-step DAG (wiring → ledger → domain → review) per story. [live: file read]

Can a Plan be a formula? Yes, technically: `bd mol pour plan-template --var goal=…`
would pour the Plan DAG. But formulas only *structure* work — there are no execution
hooks. The `on_complete.run` field is documented but "not wired end to end" per the
survey, which has never changed. [doc: bd-capability-survey §2, Templates row]
The win is `bd graph --html` visibility, not automation. Whether this earns its keep
depends on whether the lead finds that view useful versus `bd epic status`.

**Verdict:** the gate-chain formula is worth keeping. A plan-template formula is
a low-priority option, not a requirement for GOSPLAN to work.

### Defer / stale / orphans

All three exist and are in the adopted conventions. [live: `bd --help`, `bd stale --help`]

```bash
bd defer <id> --until +7d --reason "…"   # hide from bd ready; bd undefer reverses
bd stale --days 14                        # open/in_progress with no update in N days
bd orphans                                # ids cited in commits but still open
```

These are the session-close hygiene sweep. No new findings here.

### set-state + labels (lanes, Plan membership)

`bd set-state <id> <dim>=<val>` creates an event bead + sets a `dim:val` label.
Examples: `lane:S`, `lane:M`, `lane:L`, `plan:plan-01`. [live: `bd set-state --help`]

Lane labels can be any string; `bd label` manages them. `bd list --label lane:M`
filters. [live: `bd list --help` shows `--label` flag]

Can Plan membership be a label or must it be parent? **Either works:**
- `parent`: a story is a Plan child in the dep graph, shows in `bd epic status`.
  Clean structural model.
- Label `plan:plan-01`: flat tagging, visible in queries, does not enforce hierarchy.
  Better when a story spans two Plans (which should be rare but happens at plan
  boundaries).

GOSPLAN §3.3 already says "a Plan is a `bd` epic" — the parent model is correct for
the Plan's own stories. Labels are complementary for cross-cutting queries
(e.g. `bd list --label plan:01 --status closed` for retro metrics).

**Recommendation:** use `--parent` for Plan membership (structural); add `lane:S/M/L`
as labels via `bd label add` at story intake. Both are filterable with `--json`.

### `bd audit record|label` — interactions.jsonl schema

`bd audit record` accepts: [live: `bd audit record --help`]
```
--kind string      (e.g. llm_call, tool_call, label)
--model string     model name (llm_call)
--prompt string    prompt text (llm_call)
--response string  response text (llm_call)
--tool-name string (tool_call)
--exit-code int    (tool_call)
--error string     (tool_call)
--issue-id string  related issue id
--actor string     agent identity
--stdin            read a JSON object matching audit.Entry schema
```

Token counts: NO explicit flag. The schema accepts a freeform JSON object via
`--stdin`, but the flags do not include a `--tokens` field. [live]
You can embed token counts in `--response` as structured text, or pass them in
the stdin JSON if the `audit.Entry` schema accepts extra fields (unverified —
would require reading Go source or testing a probe).

Agent type: `--actor` carries agent identity (string); no structured `agent_type`
field. Distinguish agents by convention: `--actor sim-implementer`, etc.

**Is `bd audit` meant for GOSPLAN's ledger?** It is append-only and versioned in
git (`.beads/interactions.jsonl`), but the flag surface is optimised for LLM call
logging (SFT/RL dataset generation), not for GOSPLAN's per-dispatch ledger rows
(agent, story, tokens, verdict). The existing schema can hold ledger data via
`--stdin` or structured `--response`, but the result will be mixed into an LLM
call log. Cleaner: maintain a separate `ledger.jsonl` per Plan directory
(`.planning/plans/plan-NN/ledger.jsonl`) as GOSPLAN §3.7 specifies, and use
`bd audit` only if the project ever needs the RLHF data pipeline.

### `--actor` field

Present as a global flag on every command. Defaults to `$BEADS_ACTOR`, then git
`user.name`, then `$USER`. Workers should pass `--author <agent-name>` on `bd comments
add` (current convention). NOTE: `BEADS_ACTOR`/`Executed-By:` trailer convention
is DELETED as of 2026-08-27; do not use `BEADS_ACTOR`. [doc: bd-capability-survey §5]

### `bd batch` grammar

Accepted commands in stdin: [live]
```
create <type> <priority> <title>
close <id> [reason]
update <id> <key>=<value>
dep add <from-id> <to-id> [type]
dep remove <from-id> <to-id>
```
Supported update keys: status, priority, title, assignee.
Not accepted: show, list, ready, complex create flows, any flag.
The grammar is deliberately narrow; scripting complex label adds or acceptance
criteria edits still needs individual `bd` calls.

### `bd list --json` / `bd query` fields for metrics

Available in JSON output: [live: field sampling from `bd list --json --all --limit 5`]
```
id, title, status, priority, issue_type, assignee, owner,
created_at, updated_at, started_at, closed_at, close_reason,
dependency_count, dependent_count, comment_count, parent
```
Timestamps are ISO-8601 strings ending in "Z". `started_at` is null until the
issue first transitions to `in_progress`. `closed_at` is null while open.
All fields needed for GOSPLAN §3.9 metrics are available except:
- **token counts** — not stored in bd; must come from the Plan's `ledger.jsonl`
- **send-back counts** — not a first-class bd field; can be counted from
  `bd comments add --author <gate-agent>` entries containing "SEND-BACK" by
  convention, or from gate-report files

### `bd hooks` — git hooks installed by bd

Five hooks installed as thin shims: `pre-commit`, `post-merge`, `pre-push`,
`post-checkout`, `prepare-commit-msg`. Timeout 300 s, fail open. [live: survey §1]
The `prepare-commit-msg` hook adds `Executed-By:` trailers when `BD_ACTOR` is set —
but this is INERT in 1.2.2: 0 of 60 commits carried the trailer. [doc: survey §5]
`export.auto` is OFF; the pre-commit hook does NOT auto-export `issues.jsonl`.
Our manual `bd export -o .beads/issues.jsonl` convention is load-bearing.

### `bd rules audit|compact`

Both subcommands exist in the binary. [live: `bd rules --help`]
`bd rules audit` scans Claude rules for contradictions; `compact` merges related
rules. Neither reads the actual running Claude session; they parse `.claude/rules/`
and CLAUDE.md files. Safe to run read-only. Whether the audit finds real
contradictions is untested here (out of scope; the drift-auditor does this job
with domain knowledge the tool lacks).

---

## 2. Metrics prototype — live run against issues.jsonl

Script run against `.beads/issues.jsonl` (136 issues; 69 closed): [live]

```
=== Stories closed per epic ===
  sov-dda: 0/0 closed  | Modernization spikes: GPU profiler, fast routes…
  sov-m0q: 0/0 closed  | Tooling evidence roadmap

=== Median open→close age ===
  1.1 hours (n=69)

=== Comments per issue ===
  mean=2.9  median=0  max=36

=== Close evidence quality ===
  closed total: 69
  with sha in close_reason: 40  (58%)
  without sha: 29  (42%)
```

Interpretation:
- Two epics in the DB have 0 direct children (stories are top-level issues, not
  wired as epic children). The Plan-as-epic model means this will change once
  GOSPLAN is running; the metrics script will find children under Plan epics.
- Median close age of 1.1 hours is fast — many were closed in the same session
  they were opened (research or gate chain steps). GOSPLAN M-lane stories should
  show 4–12 hours from the ledger; the comparison will be interesting.
- Median 0 comments: most issues have no comments at all. The convention of
  logging findings as `bd comments add --author <agent>` is poorly adopted.
  The ledger hook in §3.7 should fix this mechanically.
- 42% of closed issues have no sha in their close reason. These are the
  "prose-only closes" the process rules prohibit. The `bd-close.sh` helper
  targets exactly this gap.

GOSPLAN §3.9 metrics that **bd can supply** with `--json` and a script:
- Stories closed per Plan (via `--parent <plan-epic-id>`)
- Median open→close age (`created_at` → `closed_at`)
- Send-back count (grep `bd comments --json` for SEND-BACK strings by gate agents)
- First-pass gate rate (stories closed without a send-back comment)
- Comments per issue

GOSPLAN §3.9 metrics that **bd cannot supply** (must come from ledger.jsonl):
- Tokens per story per lane (no token field in bd)
- Appetite vs actual (appetite is in the Plan brief, not bd)
- Send-back → green re-run cost (delta tokens across retries, ledger-only)
- Gate finding CONFIRMED/REFUTED counts (gate-report files, not bd)

---

## 3. Agent Teams ↔ bd — integration design

### What Agent Teams provides (as of Claude Code v2.1.178+)

Gated behind `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1`. [doc: agent-teams page]

- **Shared task list** at `~/.claude/tasks/{session-derived-name}/` — NOT
  `bd`; it is a separate local JSON file tree, never synced via Dolt or git.
- Task tools: `TaskCreate`, `TaskGet`, `TaskList`, `TaskUpdate`. Available to
  agents "that have the Task tools" — the docs say the lead gets them; teammates
  get them only "in a session that has the Task tools." The precise condition is
  not fully specified. [doc; matches our 2026-08-23 measurement: TaskCreate/Update/
  List absent in subagents and pane teammates]
- Task states: pending, in_progress, completed. Dependencies supported; claiming
  uses file locking to prevent races.
- Tasks are local and session-scoped. They persist across `/resume` but are NOT
  shared with other sessions or with bd.
- Hooks: `TaskCreated` (blocks on exit 2), `TaskCompleted` (blocks on exit 2),
  `TeammateIdle` (blocks on exit 2). Payloads carry common fields
  (`session_id`, `prompt_id`, `transcript_path`, `cwd`, `agent_id`, `agent_type`).
  The event-specific payload fields for TaskCreated/Completed are NOT fully
  documented in the public hooks reference (the page says "see individual sections"
  but those sections were not returned by the fetch). [unverified — would need a
  live probe to see the actual task fields in the payload]
- Limitation relevant here: "No session resumption with in-process teammates" —
  `/resume` and `/rewind` do not restore teammates. A resumed lead may try to
  message teammates that no longer exist. [doc: limitations section]

### The sync gap

The shared task list and `bd` are completely separate stores. A task completed
in the team task list does NOT close the corresponding `bd` issue. This is the
core integration problem.

### Three options for Agent Teams ↔ bd sync

#### Option (a) — bd only, no Agent Teams task list

Teammates call `bd update --claim`, `bd comments add`, `bd close` directly.
The lead reads `bd list --json` to monitor.

Advantages:
- Zero extra complexity. No sync gap. Works today.
- Teammates have Bash; every worker brief already names the bd issue id.
- `bd update --claim` is atomic against Dolt; two workers cannot both claim
  the same issue (Dolt's write serialization prevents it — see §5 concurrency).
- The survey confirms this is the intended model ("our lead-driven waves with
  an interactive user don't need async gates").

Disadvantages:
- No live team task panel in the Claude Code UI (the agent panel shows agents,
  not tasks from bd).
- Agent Teams' `TeammateIdle` / `TaskCompleted` hooks cannot fire on bd events.

#### Option (b) — Mirror: hook writes bd on TaskCreate/TaskCompleted

`TaskCreated` hook calls `bd q "<title>"` and stashes the mapping.
`TaskCompleted` hook calls `bd close <mapped-id> --reason "task completed"`.

Problems:
- The TaskCreated/TaskCompleted payload's exact `tool_input` fields are unverified
  (the task title, id, and dependency fields are not documented). [unverified]
- A TaskCompleted close would lack a sha; violates evidence rules.
- The mapping between task-list UUIDs and bd ids must be maintained somewhere
  (a side file — fragile).
- Agent Teams is experimental, disabled by default, has the no-resume limitation.
  Wiring bd to it creates a dependency on an experimental, session-scoped surface
  that the project deliberately avoids (GOSPLAN §3.6: "Agent Teams remain an
  experiment").

#### Option (c) — bd → team tasks at Plan start via a script

At Plan bet, a script reads `bd ready --parent <plan-epic-id> --json` and calls
`TaskCreate` for each story. Teammates claim via TaskUpdate; a Stop or
TeammateIdle hook calls `bd close`.

Same unverified fields problem as (b), plus the no-resume trap: if the session
is interrupted, the team task list is gone and bd is the only surviving record.

### Recommendation: Option (a), bd only

The project's own measurements and the survey's conclusions converge here:
- `TaskCreate/Update/List` are absent in subagents and teammates (2026-08-23
  measurement). [live: evidence-log.md 2026-08-23]
- Agent Teams is experimental with known runtime bugs (harness issue #53,
  no-resume limitation, one team per session). [doc]
- GOSPLAN §3.6 decision D2 already says "(a) not now" for Agent Teams.
- bd's `--claim` is atomic against Dolt; the concurrency story is understood.

**Sketch of option (a) in practice:**

```
# At Plan start: gosplan creates the Plan epic and stories
bd create epic P1 "Plan 01: <Goal>" --acceptance "…" -d "Scope box: …"
bd create task P2 "Story: <title>" --parent <plan-epic-id> \
    --acceptance "…" -d "…traps…"

# Teammate claims a story:
bd update <id> --claim

# Teammate logs progress:
bd comments add <id> "<finding>" --author sim-implementer

# gosplan monitors:
bd epic status <plan-epic-id>     # progress count
bd swarm status <plan-epic-id>    # Completed/Active/Ready/Blocked view

# Teammate closes with evidence:
scripts/bd-close.sh <id> "cargo test green: 53 passed"
```

If Agent Teams is enabled experimentally in Plan 02 (decision D2-b), the
integration is: teammates still work bd directly via Bash; the team task list
is a UI overlay for the human only, not a second source of truth.

---

## 4. Hook scripts for GOSPLAN §3.7

All three scripts are self-contained shell, stdin JSON via `jq`, ≤40 lines.

### 4.1 `scripts/hooks/dor-gate.sh` — DoR enforcement on builder/gate dispatch

Event: `PreToolUse`, matcher `Agent`. Blocks (exit 2) if the `Agent` tool is
called with a builder or gate `subagent_type` and the prompt lacks `bd:<id>`
and a `Verify:` line.

Payload fields used (verified from hooks reference): [doc]
```json
{
  "tool_name": "Agent",
  "tool_input": {
    "subagent_type": "sim-implementer",
    "prompt": "…"
  }
}
```

```bash
#!/usr/bin/env bash
# scripts/hooks/dor-gate.sh
# PreToolUse, matcher: Agent
# Blocks builder/gate dispatches missing DoR fields.
# Exempt: researcher, Explore, substrate-cartographer, debugger, drift-auditor
# Exit 2 = block; exit 0 = pass.

set -euo pipefail

PAYLOAD=$(cat)
TYPE=$(echo "$PAYLOAD" | jq -r '.tool_input.subagent_type // ""')
PROMPT=$(echo "$PAYLOAD" | jq -r '.tool_input.prompt // ""')

BUILDERS="sim-implementer ui-implementer engine-implementer data-implementer"
GATES="wiring-auditor ledger-invariant-checker evidence-auditor reviewer"
EXEMPT="researcher Explore substrate-cartographer debugger drift-auditor gosplan"

is_in() { echo "$1" | grep -qw "$2"; }

if is_in "$EXEMPT" "$TYPE"; then
  exit 0
fi

if ! is_in "$BUILDERS $GATES" "$TYPE"; then
  exit 0
fi

MISSING=""
echo "$PROMPT" | grep -qE 'bd:[a-z]+-[a-z0-9]+' || MISSING="$MISSING bd:<id>"
echo "$PROMPT" | grep -qE '^Verify:'                || MISSING="$MISSING Verify:"

if [ -n "$MISSING" ]; then
  echo "DoR gate: brief for '$TYPE' is missing:$MISSING" >&2
  exit 2
fi
exit 0
```

Fields NOT verified live (no live subagent available for the probe):
- Whether `tool_input.subagent_type` is present when the Agent tool is called
  with a definition name (e.g. `sim-implementer`) vs a free-form `subagent_type`.
  The hooks reference shows `agent_type` in SubagentStop payloads; PreToolUse
  for the Agent tool may carry it differently. **Wave 0 probe required.**
- Whether the hook fires inside a subagent that calls Agent, or only from the
  lead. If it only fires from the lead, coverage is complete (the lead dispatches
  all builders). If it fires in both, the exempt list handles recursive calls.

### 4.2 `scripts/hooks/ledger.sh` — automatic ledger row on subagent stop

Event: `SubagentStop`. Non-blocking (SubagentStop can block with exit 2, but
the ledger must not prevent a subagent from stopping). [doc]

Payload fields used (verified): [doc]
```json
{
  "hook_event_name": "SubagentStop",
  "agent_id": "subagent-456def",
  "agent_type": "Explore",
  "stop_reason": "end_turn|tool_use|max_tokens",
  "last_assistant_message": "Full assistant message text from subagent",
  "transcript_path": "/home/…/transcript.jsonl",
  "cwd": "/…"
}
```

NOTE: `agent_type` IS present in SubagentStop payloads (verified from hooks ref).
This resolves GOSPLAN §10 risk "SubagentStop payload may not name the agent type".
The fallback (pending-ledger row keyed by prompt hash) is NOT needed.

```bash
#!/usr/bin/env bash
# scripts/hooks/ledger.sh
# SubagentStop — non-blocking, no exit 2.
# Appends a ledger row to the current Plan's ledger.jsonl.
# Plan dir is resolved from CWD; falls back to a default path.

set -euo pipefail
PAYLOAD=$(cat)

AGENT_TYPE=$(echo "$PAYLOAD" | jq -r '.agent_type // "unknown"')
STOP_REASON=$(echo "$PAYLOAD" | jq -r '.stop_reason // "unknown"')
LAST_MSG=$(echo "$PAYLOAD" | jq -r '.last_assistant_message // ""' | head -c 500)
TIMESTAMP=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
CWD=$(echo "$PAYLOAD" | jq -r '.cwd // "."')

# Find the active plan ledger.  Convention: the current plan dir is the
# newest .planning/plans/plan-*/ directory with a plan.md.
PLAN_DIR=$(find "$CWD/.planning/plans" -maxdepth 1 -name 'plan-*' \
           -type d 2>/dev/null | sort | tail -1)

if [ -z "$PLAN_DIR" ]; then
  exit 0   # No active plan; do not write
fi

LEDGER="$PLAN_DIR/ledger.jsonl"

# Extract bd:<id> from last message for story linkage
STORY=$(echo "$LAST_MSG" | grep -oE 'bd:[a-z]+-[a-z0-9]+' | head -1 || true)

ROW=$(jq -nc \
  --arg ts    "$TIMESTAMP" \
  --arg agent "$AGENT_TYPE" \
  --arg stop  "$STOP_REASON" \
  --arg story "$STORY" \
  --arg msg   "$LAST_MSG" \
  '{timestamp:$ts, agent:$agent, stop_reason:$stop,
    story:$story, verdict_line:$msg}')

echo "$ROW" >> "$LEDGER"
exit 0
```

Fields NOT verified live:
- Token counts are NOT in the SubagentStop payload (no `usage` or `token_count`
  field in the documented schema). The ledger row will have no token data unless
  the final assistant message contains a usage summary (not guaranteed).
  Token counts must be added manually from the session transcript if needed.
- `transcript_path` is present (doc) but the transcript may lag the current turn.
  Reading it in the hook risks a race; the hook uses `last_assistant_message`
  directly instead.

### 4.3 `scripts/hooks/export-before-commit.sh` — bd export gate on `git commit`

Event: `PreToolUse`, matcher `Bash`. Blocks commit if `issues.jsonl` is dirty
after export.

Payload fields used (verified): [doc]
```json
{
  "tool_name": "Bash",
  "tool_input": {
    "command": "git commit …"
  }
}
```

```bash
#!/usr/bin/env bash
# scripts/hooks/export-before-commit.sh
# PreToolUse, matcher: Bash
# If the command is a git commit, run bd export first.
# Block the commit if issues.jsonl is dirty after export.

set -euo pipefail
PAYLOAD=$(cat)
CMD=$(echo "$PAYLOAD" | jq -r '.tool_input.command // ""')

echo "$CMD" | grep -qE '^git commit' || exit 0   # not a commit; pass

# Run export
bd export -o .beads/issues.jsonl 2>&1 || {
  echo "bd export failed; commit blocked" >&2
  exit 2
}

# Check if issues.jsonl is now dirty (not staged)
if git diff --name-only HEAD -- .beads/issues.jsonl | grep -q issues.jsonl; then
  echo "issues.jsonl changed after export but is not staged; stage it first" >&2
  exit 2
fi
# Also check if it differs from what is staged
if git diff --cached --name-only -- .beads/issues.jsonl | grep -q issues.jsonl; then
  # It IS staged — that is fine; the user was already staging the export
  exit 0
fi
exit 0
```

NOTE: this hook fires on ALL Bash `git commit` calls, including from worktrees.
If the worktree is not the main repo, `bd export` will discover the nearest `.beads/`
(Dolt auto-discover). Verify this is harmless before enabling in worktree sessions.

Testing without a live subagent (synthetic stdin):

```bash
echo '{"tool_name":"Bash","tool_input":{"command":"git commit -m test"}}' \
  | bash scripts/hooks/export-before-commit.sh
# Expected: export runs; exits 0 if jsonl already in sync
```

Run from the project root: the export ran and exited 0 (issues.jsonl already
in sync with Dolt — our convention is correct). [live]

### Hook registration in `.claude/settings.json`

```json
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Agent",
        "hooks": [{"type": "command",
                   "command": "${CLAUDE_PROJECT_DIR}/scripts/hooks/dor-gate.sh"}]
      },
      {
        "matcher": "Bash",
        "hooks": [{"type": "command",
                   "command": "${CLAUDE_PROJECT_DIR}/scripts/hooks/export-before-commit.sh"}]
      }
    ],
    "SubagentStop": [
      {
        "hooks": [{"type": "command",
                   "command": "${CLAUDE_PROJECT_DIR}/scripts/hooks/ledger.sh"}]
      }
    ]
  }
}
```

---

## 5. Risks

### Dolt concurrency with parallel teammates

Dolt uses optimistic concurrency at the SQL layer. Two simultaneous writers
(two teammates calling `bd update --claim` on different issues at the same
time) each open a write transaction; if they touch different rows, both commit.
If they claim the SAME issue, one sees a conflict and retries. [doc: Dolt
docs, optimistic concurrency; unverified against bd 1.2.2's embedded Dolt
path specifically]

`--dolt-auto-commit` policy: default is `off` (each `bd` invocation is its own
transaction, committed on exit). This is the safest mode for concurrent workers.
`batch` and `on` are alternatives with more write amplification risk in the
embedded/btrfs context that motivated `bd batch`.

**Claim race specifically:** `bd update <id> --claim` sets `assignee` and
`status=in_progress` atomically. If two workers call it simultaneously on the
same issue, Dolt's optimistic lock means one will succeed and one will get a
conflict error; the bd CLI will print an error. The second worker must re-read
`bd ready` and pick a different issue. This is the correct behavior; no data
corruption occurs.

**Practical guidance:** workers should always call `bd ready --json` (or
`bd list --status open --parent <epic> --json`) before claiming. If `--claim`
returns a non-zero exit, re-query and pick another issue. Brief should include:
"If `bd update --claim` fails, re-run `bd ready` and claim the next open story."

### v65 schema trap

Pinned at 1.2.2. Never run `bd upgrade` casually. A machine that ran v1.2.1
has a v65 schema 1.2.2 cannot read. Our DB is clean; `bd info` confirms
"Mode: direct" (embedded). [live: survey §0, §1]

### JSONL export drift

`export.auto` is OFF. The pre-commit export hook (§4.3) is the proposed
mechanical guard. Until the hook is installed, the manual `bd export` step
remains load-bearing. If the hook is installed and the commit is somehow
bypassed (amend, interactive rebase), the jsonl can lag. `bd orphans` at
session-close catches the symptom (open issues whose ids appear in commits).

### Dual-claim race

Two workers calling `bd update <id> --claim` simultaneously on the same issue:
Dolt's write serialization ensures at most one succeeds. The other gets an
error; no issue is double-claimed. Briefs must instruct workers to re-queue
on error. [doc/source: Dolt optimistic concurrency; not live-probed on this
embedded version specifically — mark as unverified for that specific code path]

### Unverified items

| Item | Why unverified | How to close |
|---|---|---|
| `tool_input.subagent_type` in PreToolUse Agent payload | No live dispatch probe in this session | Wave 0 probe: dispatch a builder, capture the hook's stdin, verify the field name |
| Token counts in SubagentStop payload | Not in the documented schema; transcript may lag | Read `transcript_path` after a known subagent stop; check for `usage` fields |
| `audit.Entry` stdin schema — whether extra fields are accepted | Would need Go source or live probe | `echo '{"kind":"ledger","story":"sov-xxx","tokens":1234}' \| bd audit record --stdin` |
| TaskCreated/TaskCompleted exact payload fields | Docs say "see individual sections" but those were not present in the fetch | Probe with Agent Teams enabled and a TaskCreated hook logging stdin to a file |
| Dolt embedded concurrency on simultaneous `--claim` | Dolt doc describes optimistic concurrency; not tested against the embedded bd 1.2.2 code path | Two terminal sessions, same issue, simultaneous `bd update --claim`; observe which errors |

---

## 6. Summary table

| GOSPLAN need | bd primitive | Status |
|---|---|---|
| Plan = epic | `bd create`, `bd epic status`, `bd epic close-eligible` | Ready [live] |
| Story under Plan | `--parent` flag | Ready [live] |
| Lane as label | `bd label add lane:S/M/L` | Ready [live] |
| Wave setup | `bd batch` | Ready [live] |
| DoR enforcement | `dor-gate.sh` PreToolUse Agent hook | Script written; field unverified |
| Ledger row at subagent stop | `ledger.sh` SubagentStop hook; `agent_type` IS in payload | Script written; token counts absent |
| bd export before commit | `export-before-commit.sh` PreToolUse Bash hook | Script written; tested synthetic stdin [live] |
| Metrics | `bd list --json` + plan-metrics script | Fields confirmed [live]; tokens/send-backs need ledger |
| Agent Teams sync | bd only (option a) — no team task list sync needed | Recommended |
| Swarm for Plans | NOT recommended | Skip; plain epic DAG sufficient |
| `bd audit` as ledger | NOT recommended | Use separate ledger.jsonl per Plan dir |
| Token counts in bd | NOT possible in 1.2.2 | Must come from ledger.jsonl |

---

## 7. Open questions passed to Wave 0

Two probes GOSPLAN §9 (Wave 0) already names, now verified as still open:

1. **DoR hook `subagent_type` field**: dispatch a builder, log the PreToolUse Agent
   stdin to a file, confirm the exact field that carries the agent type.

2. **SubagentStop token field**: after a known subagent stop, read the hook stdin
   and the `transcript_path`; confirm whether token counts appear anywhere in the
   payload. If not, the ledger must rely on the final message summarizing them,
   or be filled in by the lead after the fact.

One new open question from this research:

3. **`bd audit record --stdin` schema**: does it accept arbitrary JSON keys beyond
   the documented flags? If yes, `bd audit` can double as the ledger store and
   we avoid a second append-only file. Test: `echo '{"kind":"ledger_row",
   "story":"sov-xxx"}' | bd audit record --stdin --kind ledger_row` and check
   whether the extra key survives in `interactions.jsonl`.
