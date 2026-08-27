---
name: verification-procedures
description: How to verify a CI job locally without GitHub Actions on this machine, and which tools are genuinely unavailable
metadata:
  type: reference
---

**This machine cannot run GitHub Actions.** `act` is not installed and Docker is
unusable (`docker info` fails). `gh` 2.98.0 IS available and is enough for
read-only remote facts. Verified 2026-08-27.

**Simulating a workflow job locally.** Run each `run:` step body under GitHub's
documented default Linux shell, `bash -e -o pipefail`, in the repo root, with the
pinned tool on `PATH`. Check the exit status per step. This proves the command's
exit behaviour and the step ordering. It does NOT prove the job runs on the
runner, that the install step succeeds there, or that a red step renders as a
failed PR check - never report those as verified.

**Useful read-only `gh` probes before claiming a workflow will gate anything:**

```sh
gh api repos/<owner>/<repo>/actions/permissions   # {"enabled":true,"allowed_actions":"all",...}
gh api repos/<owner>/<repo>/actions/workflows     # total_count
gh api repos/actions/checkout/git/ref/tags/v4     # resolve a tag to a commit sha for pinning
```

A workflow on a repo with Actions disabled gates nothing. Check first.

**Proving a workflow adds no unrelated jobs:** strip comments
(`sed 's/#.*//'`), drop blank lines, print the WHOLE remainder, and grep the
stripped text for the forbidden constructs. Do not `grep` the raw file - comments
that say "there is no `|| true` here" produce false positives, and `| head -N`
under fff is a relevance-ranked sample that never proves coverage.

**MCP code-review-graph tools are NOT reachable from a subagent in this repo.**
Measured 2026-08-27, four ways: `ToolSearch("select:mcp__code-review-graph__query_graph_tool,...")`,
a `select:` on `get_architecture_overview_tool`, and two keyword searches all
returned "No matching deferred tools found". The lead asserted twice that MCP
tools survive the subagent filter; they did not here. Read path is `cat`,
`sed -n`, `grep -n`/`rg` via Bash. Do not spend turns retrying.
