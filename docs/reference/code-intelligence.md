# Code intelligence — which tool answers which question

**Kind:** reference
**Authority:** operational
**Status:** active
**Owner:** project lead
**Last verified:** 2026-08-27

This repo has two code-intelligence tools. They are complements, not rivals, and the
`CLAUDE.md` MCP block on its own does not say which wins where. This document does.

Every claim below was measured in this repo on 2026-08-27. Nothing here is quoted from
upstream marketing.

## The division of labour

| Question | Tool |
|---|---|
| Who calls X? Where is X defined? | **LSP** — compiler-backed, exact, per-reference |
| What is this type? What warnings exist? | **LSP** — the graph does not type-check |
| Is this rename safe? | **LSP** — graph call edges are heuristic |
| Which tests cover X? | **graph** — `query_graph_tool` `tests_for`; LSP has no such concept |
| What breaks if I change these files? | **graph** — `get_impact_radius_tool` |
| Which execution flows does this touch? | **graph** — `get_affected_flows_tool`; no LSP equivalent |
| `base_mod/*.lua` → `prototypes/*.rs` seams | **graph** — no single LSP server spans both |
| Anything, while rust-analyzer is still cold | **graph** — instant, no warm-up |
| Where is the code that does X, when you do not know its name? | **graph** — `semantic_search_nodes_tool` |

**LSP stays first for symbol-level intelligence.** The MCP block in `CLAUDE.md` says to start
with the graph to narrow scope; that is about *scope*, not about *precision*. Narrow with the
graph, then confirm in the source or with LSP before you change behaviour.

## Semantic search — reach for it when you cannot name the thing

`semantic_search_nodes_tool` embeds your query and ranks nodes by vector similarity. It is the
right first move whenever you know **what the code does** but not **what it is called** — the one
question neither LSP nor grep can answer, because both need a name or a string you already have.

Ask it in a sentence describing behaviour, not in identifiers:

- good — `"a company asks for more input than its recipe needs and keeps the surplus"`
- bad — `"hoarding"` (a name; `query_graph_tool` or grep finds names better and cheaper)

Local model, no API call, no per-query cost. Runs on the GPU in well under a second.

### What it is measured to do, and not do

Benchmarked 2026-08-28 on this repo: 32 hand-authored paraphrase queries, each deliberately
sharing no distinctive token with its target symbol, against `Alibaba-NLP/gte-modernbert-base`
over 3501 embedded nodes.

| | Result |
|---|---|
| Correct symbol ranked #1 | 11 / 32 |
| In the top 5 | 17 / 32 |
| Found anywhere in the top 20 | 21 / 32 |
| **Not returned at all** | **11 / 32 — a 34% miss rate** |

`semantic_search_nodes_tool` defaults to `limit: int = 20`, so that last row is the failure rate
you actually experience.

**Therefore: it is a lead generator, not an oracle.** Two rules follow, and they are the same
discipline the graph zero already demands:

1. **A miss is never evidence of absence.** One in three of these queries returned nothing useful
   for code that provably exists. "Semantic search found nothing" means *unknown*, never *not
   there*. Fall back to `query_graph_tool`, grep, or LSP before concluding anything.
2. **Confirm every hit in the source.** Similarity ranks by description, not by behaviour. A
   plausible-looking top hit can be the wrong function.

A larger model was tested and rejected: `Qodo-Embed-1-1.5B` cut the miss rate to 19% but tied
exactly on recall@1 and recall@5, cost 6.17 GB of resident VRAM against 0.56 GB, and doubled
query latency. The gain sat entirely in ranks 6–20. Not worth the VRAM alongside the running game.

## Trap 1 — never trust the FIRST LSP call

A cold language server answers `findReferences` with **"No references found."** That is not an
error. It reads exactly like a true negative.

Measured on `Market::set_requested` (`simulation/src/economy/market.rs:441`):

| | Result |
|---|---|
| Cold `rust-analyzer` | `No references found` — **wrong** |
| Warm, identical call | 4 references across 3 files |
| Graph, first call | 2 callers, correct |

This matters here specifically. The `sov-lpj` Phase-0 fact-sheet claimed "`Market::set_requested`
has ZERO production callers". That claim was true at the time — but a cold LSP would have
"confirmed" it on any day, including days when it was false.

Rules:

- Warm the server before the first load-bearing query. Any call starts it; one cheap
  `documentSymbol` is enough. **This applies to the main session only** — see below.
- An empty LSP result is **"unknown"**, never **"none"**, until a second call or a second tool
  agrees.
- Compiler diagnostics arrive free as a side effect of any LSP call. Read them — they are
  compiler truth, and the graph can never produce them.

## Trap 1b — subagents have no LSP at all

**Measured 2026-08-27, Claude Code 2.1.247. Everything above about LSP applies to the main
session. A subagent cannot use LSP, whatever its definition says.**

A probe agent whose definition listed `LSP` got, verbatim:

```
Error: No such tool available: LSP. LSP is disabled for this session, in subagents as well as here.
ToolSearch("select:LSP,ListAgents")  ->  No matching deferred tools found
```

Not recoverable from inside. A second agent hit the same wall independently the same hour.
The message is misleading — LSP kept working in the main session minutes later.

Measured subagent toolset: `Read`, `Bash`, `ToolSearch`, `Skill`, `Write`, `Edit`, and
`SendMessage` (deferred). Absent: `LSP`, `ListAgents`, `Grep`, `Glob`, `Agent`, `WebFetch`.

Two mechanisms stack. Background subagents — the default — keep only a fixed built-in list
that excludes `LSP` and `ListAgents`, and the removal reports no error. Auto mode
additionally removes `Grep` and `Glob`.

**What this means for how work is split here:**

| Job | Who |
|---|---|
| Resolve a symbol, find callers, check a type | **The lead**, in the main session |
| Act on those facts | A subagent, given `file:line` in its brief |
| Structural questions inside a subagent | **The graph** — MCP tools survive the filter |

The graph is the only code-intelligence tool a subagent can actually reach. That is a
stronger reason to keep it than any token-saving argument.

Never brief a subagent to "use LSP" or to "warm LSP". Name `grep -n` via Bash as its read
path, explicitly, because its own definition may still claim otherwise.

## Trap 2 — `head_matches_build` compares SHAs, not content

The graph stamps every response with provenance:

```json
"built_at_sha": "ba7e8e7...", "head_sha": "ba7e8e7...", "head_matches_build": true
```

That flag compares **git SHAs**. It says nothing about uncommitted files. Measured: with a dirty
tree, the graph reported a `CALLS` edge from `recipe_init` to `set_requested` at
`goods_company.rs:24` while `head_matches_build` was `true` — but at HEAD that line was
`.unwrap_or(item.amount as u32)`. The edge existed only in uncommitted code.

The graph shows **your working tree**. That is often what you want. Just know which one you are
reading before you cite it.

A second-order effect: the `pre-commit` hook updates the graph *before* the commit exists, so
immediately after any commit the graph is stamped one commit behind. The next `Edit` corrects it.

## Trap 3 — edges carry confidence, and it is not always high

Call resolution is Tree-sitter AST, not a compiler frontend. Every edge carries a tier —
`EXTRACTED`, `INFERRED`, `AMBIGUOUS` — and every node carries `target_resolution`. A value of
`unresolved` means the edge matched **by name**, not by type.

Do not treat an `AMBIGUOUS` or `unresolved` edge as proof of anything. Confirm in the source.

## When NOT to use the graph

Upstream's own guidance (`docs/FAQ.md`), and it applies to us:

- Trivial single-file diffs — the structural response can cost more tokens than it saves.
- Repos under a few hundred files. **We are at 332 files, right on that line.** The graph earns
  its keep here on the second axis: how often you ask multi-file questions. Every diff in this
  repo crosses `simulation/`, `prototypes/` and `base_mod/`.
- One-hop lookups you already know the answer shape of. Grep is fine and fresh.

## Install — what has to be true for it to work

| Piece | Where | Versioned? |
|---|---|---|
| MCP server config | `.mcp.json` | yes |
| SessionStart + PostToolUse hooks | `.claude/settings.json` | yes, via the `!.claude/settings.json` allowlist |
| `pre-commit` graph update | `.git/hooks/pre-commit` | no — git hooks never are |
| The graph itself | `.code-review-graph/graph.db` | no — gitignored, rebuilt locally |
| The binary | `~/.local/bin/code-review-graph` | no — install separately |

**The binary must be on `PATH` outside any virtualenv.** Every hook is guarded with
`command -v code-review-graph >/dev/null 2>&1 || exit 0`, so a missing binary makes all of them
**silently do nothing** — no error, no output. Install it where a clean login shell can find it:

```bash
uv tool install "code-review-graph==2.3.8"   # lands in ~/.local/bin
```

Pin the version. The graph database has a schema, and a version mismatch is the same class of
failure as the beads 1.2.1 / 1.2.2 schema skew.

Verify the guard actually passes, rather than assuming:

```bash
env -i HOME="$HOME" zsh -l -c 'command -v code-review-graph' || echo "hooks are inert"
```

After a fresh clone: `uv sync`, install the binary as above, then
`code-review-graph build` to create the graph.

## Related

- `~/.claude/rules/tool-discipline.md` — the global LSP-first rule and the same two traps
- `docs/process/development-cycle.md` — which agent owns which lane
- Upstream FAQ: <https://github.com/tirth8205/code-review-graph/blob/main/docs/FAQ.md>
