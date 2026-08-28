# Subagent tooling — what a spawned agent actually has

**Kind:** reference
**Authority:** operational
**Status:** active
**Owner:** project lead
**Last verified:** 2026-08-28

What a subagent can reach differs from what its definition lists, and the difference is
not recoverable from inside the subagent. This document records what was measured, not
what the harness documentation implies.

Every claim below comes from probes run in this repo on 2026-08-27 and 2026-08-28,
Claude Code 2.1.247. Companion document: `docs/reference/code-intelligence.md`, which
covers which code-intelligence tool answers which question.

## The settled question — subagents have no LSP

**Six probes agree. The question is closed.**

Five probes on 2026-08-27 found `LSP` absent from every subagent arm, with
`ToolSearch("select:LSP")` answering `No matching deferred tools found`. The one
remaining doubt was configuration: the user had added `"LSP"` to
`~/.claude/settings.json` `permissions.allow` and changed `defaultMode` from `"auto"` to
`"default"`, and it was not established whether that edit landed before or after the
session restart. A probe measuring the baseline rather than the lever would look
identical.

The sixth probe, on **2026-08-28**, settled it. With `"LSP"` present in
`permissions.allow`, a `general-purpose` opus subagent still got
`No matching deferred tools found` from `ToolSearch("select:LSP")`, while its control
call `ToolSearch("select:Agent,WebFetch")` returned full schemas — proving the probe
itself worked and the absence was real.

**Conclusion: `permissions.allow` is not a lever for this. Subagents do not have LSP,
period.** No brief should tell a subagent to use or warm LSP; the instruction cannot be
followed. Symbol-level intelligence is the **lead's** job, resolved in the main session
where LSP demonstrably works and pasted into the brief as `file:line`.

## The probe matrix

| Arm | LSP in list | `select:LSP` | `Agent` | `WebFetch` | `Grep`/`Glob` | graph MCP |
|---|---|---|---|---|---|---|
| `general-purpose` | absent | not found | yes | yes | absent | yes |
| `sim-implementer` (definition claims no LSP) | absent | not found | yes | yes | absent | yes |
| `reviewer` (pins a `tools:` allowlist) | absent | not found | **no** | **no** | absent | yes |
| post-restart retest | absent | not found | yes | yes | absent | yes |
| `general-purpose`, `permissions.allow` contains `LSP` (2026-08-28) | absent | **not found** | yes | yes | absent | yes |
| **main session (control)** | deferred | **schema returns** | yes | yes | yes | yes |

An exhaustive arm enumerated the **complete 74-tool subagent surface** (8 loaded, 66
deferred) and ran 10 keyword framings plus 7 name variants — `LSP`, `Lsp`, `lsp`,
`mcp__lsp__lsp`, `mcp__language-server__definition`, `LanguageServer`,
`CodeIntelligence`. No language-server tool exists under any name. `"rust analyzer"` and
`"documentSymbol workspaceSymbol"` both returned nothing.

## The six findings

### 1. `tools:` allowlists narrow; they never widen

The `reviewer` arm, the only one pinning a `tools:` list, lost `Agent` and `WebFetch`
that every unrestricted arm kept. A `tools:` list can only remove. Pinning one cannot
grant a tool the subagent would not otherwise have, and it silently costs whatever the
list forgot to name.

Practical rule: pin `tools:` only to deny. Never pin it hoping to grant, and always
include `ToolSearch` — it is what reaches `SendMessage`, and a worker without it cannot
report at all.

### 2. `Agent` and `WebFetch` ARE reachable

Both are present in any subagent that does not pin a `tools:` allowlist.
`~/.claude/rules/tool-discipline.md` listed both under **Absent**; that table was wrong
and was corrected on 2026-08-28.

Consequence: a subagent can fan out further. Our agent definitions constrain that by
policy, not by capability — fan out to READ, never to write, and declare how many
helpers were spawned so the lead's cost estimate stays honest.

### 3. The `Read` guard taxes three calls per code file, and prescribes an impossible remedy

`~/.claude/hooks/lsp-first-read-guard.js` is a `PreToolUse` hook on `Read`. Measured in a
subagent: the **first two `Read` calls on a code file are blocked and the third
succeeds** — 3 calls to open 1 file, every file, for the whole session.

Worse, the block text told the agent to run `ToolSearch("select:LSP")`, which provably
cannot succeed in a subagent. **This hook text is the most likely origin of the belief
that subagents have LSP.** It was corrected on 2026-08-28 to name the real remedies
first: `ct view <file> --range A:B` and `ct search`, neither of which is gated, plus the
fact that the block relents on its own.

If you are a subagent and you hit this block: do not retry the warmup. Read again, or
use `ct view` / `ct search`.

### 4. Hook coverage is not uniform — `ct-steer` does not fire in subagents

The `Read` guard fires in subagents; `ct-steer` does not. Cause not investigated —
**recorded as ambiguous, not as a rule.** Do not infer from it that any particular hook
will or will not run. (Separately, `ct-steer` was verified as not installed at user scope
on 2026-08-28, so its absence in a subagent may simply be that.)

### 5. The graph IS reachable — and it gave a confidently wrong zero

These two facts have to travel as a pair. The graph is the answer to "what code
intelligence CAN a subagent use", and it is also the tool that lied.

**MCP tools survive the subagent tool filter.** Every `mcp__code-review-graph__*` tool is
reachable in a subagent — confirmed live in this session's wave. `LSP` is not, `Grep` and
`Glob` are not, but the graph is. It is therefore **the only code-intelligence tool a
subagent can actually call**, and a brief should name it alongside `grep -n` as the read
path, not treat it as optional.

Schemas arrive **deferred**, so they are absent from the visible tool list until loaded:
`ToolSearch("select:mcp__code-review-graph__query_graph_tool,mcp__code-review-graph__get_impact_radius_tool")`.
Only a `no matching deferred tools found` result proves absence.

**And now the trap.** `references_to` on `Market::set_requested` returned **0**, with the
accompanying string `"this 0 is a real absence"`. It was not. LSP found **4** references
across 3 files; `callers_of` found 2; `grep` found 4.

**Never close a question on a graph zero.** An empty graph result means "not indexed" or
"not statically visible", never "does not exist" — and the graph's own phrasing may
assert otherwise. This sits alongside the two traps already in
`docs/reference/code-intelligence.md` (`head_matches_build` compares SHAs not content;
edges carry `EXTRACTED`/`INFERRED`/`AMBIGUOUS` confidence).

#### The server's five PROMPTS are a different mechanism, and they are NOT reachable

Prompts are not tools. The server registers five with `@mcp.prompt()`
(`main.py:1059-1101`): `review_changes`, `architecture_map`, `debug_issue`,
`onboard_developer`, `pre_merge_check`. They surface in the **main session only**, as
slash commands of the form `/mcp__code-review-graph__debug_issue`. A subagent cannot
invoke a slash command, so a lead who wants a worker to follow one of these workflows
must **paste the expanded steps into the brief**.

Bodies live in `.venv/lib/python3.13/site-packages/code_review_graph/prompts.py`.
`debug_issue` expands to, verbatim in substance:

1. `get_minimal_context(task="debug: <description>")`
2. `semantic_search_nodes(query=<keywords>, detail_level="minimal", limit=5)`
3. for the top 1–2 results, `query_graph(pattern="callers_of", target=<name>, detail_level="minimal")`
4. if execution flow matters, `get_flow(name=<the single most relevant flow>)`
5. `get_review_context` / `get_impact_radius` **only** to trace the blast radius of a
   specific change

Every prompt carries the same token preamble, and it is good practice regardless of which
prompt you are following: `get_minimal_context` first; `detail_level="minimal"` unless
minimal is insufficient; escalate to `standard`/`verbose` only for the specific entities
that need it; **never more than 3 graph calls per turn**; prefer targeted queries
(`query_graph` on a symbol) over broad scans (`list_communities` with full members).

#### The install is FINE. The real failure is staleness, and it is LOUD

`.venv/bin/code-review-graph` exists, is executable, and answers: **Nodes 3732, Edges
30017, Files 333**. The graph hooks are not broken. Do not write that they are.

**Correction, 2026-08-28.** An earlier revision of this section claimed the
`.claude/settings.json` change had replaced a loud `SessionStart` "binary NOT FOUND"
check with a silent `[ -x "$CRG" ] || exit 0` guard in `PostToolUse`, leaving a future
breakage unannounced. **That was false, and it was the lead's error, asserted and then
confirmed by a worker rather than checked.** Both hooks exist and always did: the
`SessionStart` check still carries the loud warning, and the `PostToolUse` updater was
an addition, not a replacement. Hook counts are identical between `HEAD` and the working
tree — `CwdChanged` 1, `PostToolUse` 1, `SessionStart` 2 — so that diff is a key
reordering with no semantic change. Method note: a claim like this is settled by parsing
the JSON per hook event, never by eyeballing a reformatted diff.

**The failure that IS real: a stale graph refuses to answer, and the refusal is easy to
misread as a working tool.** `get_minimal_context_tool` returns
`status: not_ready, reason: stale_graph` whenever `head_matches_build` is false, and
that flag compares **git SHAs, not file content**. Measured on 2026-08-28: `main`
advanced from `2cc7331` to `345a79a` with a docs-only commit, so
`code-review-graph update` correctly reported `0 files updated` — and left the recorded
SHA at the old value, which kept every graph query blocked. `update` did not clear it;
only a full `code-review-graph build` did. Note the CLI spelling: `build` is the
subcommand, and `update --full-rebuild` is not a valid flag.

Consequences to carry into a brief:

- A lead who checks `status` and sees plausible node counts can wrongly tell workers the
  graph is current. That happened twice in one session, and **three separate agents
  independently corrected the lead**, which is the only reason it was caught.
- Verify freshness with `head_matches_build`, never with the node counts or the
  `Last updated` timestamp.
- A docs-only or config-only commit is enough to block the graph, because the check is
  on the SHA and not on whether any indexed file changed.

### 6. `fff` is OFF

The typo probe `grep -rn "set_requsted"` returned empty in every arm — no `rg-fff`
header, no `[~approx]` lines. Bash `grep`/`find` are the real engines: results come back
in file order, `| head -N` is a genuine first-N, and the `[~approx]` false-positive trap
cannot fire.

This is a user-controlled toggle. Re-run the typo probe before relying on either state;
several agent definitions still carry fuzzy-wrapper warnings written while it was on.
`ct search` sidesteps the question entirely — it never routes through `fff`, so its
exit 1 is a trustworthy absence.

## A correction recorded so it is not repeated

Mid-study the lead claimed its own `findReferences` had reproduced the cold-server trap.
It had not: the call pointed at `market.rs:440`, a doc comment, not the symbol. The
correct call at line 441 returned 4 references. A wrong position explained the empty
result, and the claim was withdrawn.

The cold-server trap is real and documented in `docs/reference/code-intelligence.md` —
but an empty LSP result has a second, more common cause, and the cheap check is to
confirm the position points at the symbol before concluding anything.

## What this means for briefs

| Job | Who |
|---|---|
| Resolve a symbol, find callers, check a type | **The lead**, in the main session |
| Act on those facts | A subagent, given `file:line` in its brief |
| Structural questions inside a subagent | **The graph** — MCP tools survive the filter |
| Fan-out reads | A subagent may spawn helpers; the verdict stays with it |

Name the read path in every brief: `Read`, plus `grep -n` / `rg` through `Bash`, plus
`ct view` / `ct search`, plus **the graph MCP tools**, which are the worker's only code
intelligence. Say explicitly that the worker has no LSP, because the worker's own
definition and the `Read` guard's block text may both claim otherwise. And if you want a
worker to follow one of the server's five prompts, paste the expanded steps in — it
cannot run the slash command.

## Related

- `docs/reference/code-intelligence.md` — which tool answers which question, and the graph's other traps
- `~/.claude/rules/tool-discipline.md` — the global rule; its Absent/Present table was corrected from this study
- `~/.claude/hooks/lsp-first-read-guard.js` — the `Read` guard, corrected from this study
- `docs/process/development-cycle.md` — which agent owns which lane
