---
name: verification-procedures
description: How to verify a CI job on this repo — the real-run procedure that proved the dependency-policy gate, the local simulation fallback, and the tool availability facts
metadata:
  type: reference
---

## The CI gate is PROVEN, not simulated (2026-08-27)

The dependency-policy gate was verified on real GitHub Actions. Do not re-derive.
See [[dependency-policy-baseline]] for the policy itself.

- Green baseline: run `33103168877` on `main`, success, 4m19s (almost all of it
  the from-source `cargo install`).
- Red under Mutation B: run `33106713230`, job `98638382820`, on PR #120.
  `gh pr checks` -> `fail`; `statusCheckRollup.conclusion` -> `FAILURE`;
  `##[error]Process completed with exit code 8`. **The gate does not fail open.**

**The procedure that worked, reuse it verbatim for any future gate:**

1. Work in a worktree on a throwaway branch. Apply the mutation, run the checker
   locally with `CARGO_TERM_COLOR=never`, save the full output to a file.
2. Commit, push, `gh pr create`. Two runs appear (the `push` and `pull_request`
   triggers are separate concurrency groups) — the `pull_request` one is the
   check that matters.
3. `gh run watch <id> --exit-status --interval 20` blocks until done. Foreground
   `sleep` is denied; this is the sanctioned wait.
4. `gh run view --job <jobid> --log > file`. The log is TSV:
   `<job>\t<step>\t<RFC3339 timestamp> <line>`.
5. **Prove same-finding by diff, not by eyeball.** Filter to the step's rows,
   strip the `job\tstep\ttimestamp ` prefix, drop `##[...]` lines, and normalise
   the absolute workspace path on BOTH sides
   (`/home/runner/work/<repo>/<repo>/` and the local worktree path -> `WORKSPACE/`).
   `diff` then comes back empty. It did: 3428 identical lines. The only residue
   is GitHub's 6-line `##[group]Run …` header, which is log furniture, not output.
6. Revert, prove restoration with `git diff main -- <file>` + `git status` +
   `sha256sum` against `git cat-file blob main:<file>`, re-run the checker.
7. `gh pr close <n> --comment "…"`. **`--delete-branch` FAILS in a worktree**
   (`fatal: 'main' is already used by worktree at …`) — follow with
   `git push origin --delete <branch>` and confirm with `git ls-remote --heads origin`.

Use a mutation whose output is machine-independent. `sources` derives only from
`Cargo.lock`, so byte-equality is achievable; `advisories` depends on the RustSec
snapshot and can differ on time alone, which makes a same-finding diff meaningless.

## Local simulation — the fallback when there is no run

This machine cannot run GitHub Actions: `act` is not installed, Docker is
unusable. Run each `run:` step body under GitHub's default Linux shell for `run:`
steps and check exit status per step. **Measured: the runner reports
`shell: /usr/bin/bash -e {0}` — `-e`, without `-o pipefail`.** Earlier notes said
`bash -e -o pipefail`; that is the `shell: bash` form, not the default.

This proves exit behaviour and step ordering only. It does NOT prove the job runs
on the runner, that the install succeeds there, or that a red step renders as a
failed PR check. Never report those as verified from a simulation. The previous
agent was right to refuse to sign off criteria (ii) and (iii) without a real run.

**Proving a workflow adds no unrelated jobs:** parse the YAML with
`python3 -c "import yaml; …"` and enumerate `jobs` and step names — that beats
grepping, because comments saying "there is no `|| true` here" are false
positives, and `| head -N` under fff is a relevance-ranked sample.

## Tool availability — CORRECTED 2026-08-27

**MCP `code-review-graph` tools ARE reachable from a subagent.** An earlier
version of this memory said they were not. That measurement was real but its
cause was a `tools:` allowlist in the agent definitions naming no `mcp__`
pattern; the user has since removed every `tools:` key. Re-measured:
`ToolSearch("select:mcp__code-review-graph__query_graph_tool,mcp__code-review-graph__get_impact_radius_tool")`
returned both schemas. Of limited use in this lane — the graph models code
structure, not workflow YAML or dependency policy — but do not repeat the claim
that it is unreachable.

`LSP` remains absent in subagents. `gh` 2.98.0 is available and is enough for
every remote fact this lane needs.
