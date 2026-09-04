# Verification MCP server — design (sov-verify)

Status: design, unimplemented. Inputs: repo verification catalog
(scout 2026-09-04) + MCP protocol research (spec 2026-07-28, verified
2026-09-04). Problem it solves: agents verify today by shelling bespoke
commands (slow to discover, easy to get subtly wrong — wrong CWD, vacuous
filters, GPU contention, 250-line pastes into context). A mounted toolset
makes the right check the easy check.

## Decision 1: real MCP, not custom
Use the actual Model Context Protocol over **stdio**. The harness already
mounts MCP servers (code-review-graph), so discovery, invocation, and result
handling exist. A custom bridge would re-implement listing, schemas,
progress, cancellation, and annotations worse. Hedge: implement the toolset
as plain Rust functions behind a dual frontend — `tools/call` over stdio
**and** `sov-verify <tool> --json` CLI subcommands sharing the same code —
so a harness without MCP mounting still gets the catalog.

## Decision 2: Rust + rmcp, new workspace member `sov-verify`
- `rmcp` 3.2.0 (Tier 1, 67/67 server conformance, MSRV 1.88) + tokio::process.
  Single static binary, no new runtime (repo has no Node/Python runtime
  dependency otherwise). Reject TypeScript (harness-language coupling for no
  gain) and Python FastMCP (composition/proxying unneeded for a fixed set).
- New member `sov-verify/` (binary crate). Constraints from deny.toml:
  `publish = false` (load-bearing for the private/wildcard rules), no
  wildcard deps, crates.io only, pin versions; `cargo-deny check` must stay
  green with the addition. Confirm repo rustc ≥ 1.88 (no pinned toolchain).
- No shell, ever: argv-spawn only (`Command::new("cargo").args([...])`),
  cwd pinned to repo root, one tool per check (no generic `run_command`),
  strict input schemas (`additionalProperties: false`).

## Tool catalog (v1)
Mapped 1:1 from the repo catalog; each returns distilled `text` (summary +
failure excerpt) + `structuredContent` under `outputSchema` (counts,
durations, exit codes), full logs to artifact files via `resource_link`.
Failing checks are `isError: true` results, never protocol errors.

| Tool | Wraps | Notes |
|---|---|---|
| `verify_crate_tests` {crate, filter?, threads?} | `cargo test -p <crate> [filter]` | Rejects zero-match filters (vacuous-green rule); streams progress per test binary; 300k-tick test annotated slow |
| `verify_build` {package?, features?} | `cargo build/check` | Includes the multiplayer-feature build path agents forget |
| `verify_docs` {} | `check_docs.py && mdbook build` | Fast (~seconds); returns error count + warnings |
| `verify_deny` {} | `cargo-deny check` | `isError` with install reason when the binary is absent (known agent-hostile gap) |
| `capture_run` {scene?, out?, gpu_timings?, validation?} | `engine_demo capture …` | Returns PNG as `image` block (downscaled) + record JSON structured; enforces the fixed contract server-side |
| `verify_golden` {} | golden test | Same-machine scoping stated in description; suggests lavapipe path elsewhere |
| `verify_validation` {} | `run_validation_gate.py` on a fresh validation capture | Owns the freshness/record/adapter preconditions agents get wrong |
| `verify_gpu_timing` {} | timing capture + `check_gpu_timing.py` | GPU mutex (below); re-run-quiet-first documented in description |
| `verify_oom` {} | oom-guard workflow command | Reports systemd-scope requirement honestly on hosts without it |
| `board_state` {} | `bd stats` + `bd ready` | readOnly; lets agents check the board without shell |

Annotations: every tool `readOnlyHint: true` (the server CANNOT mutate —
no mutating tool exists, so the boundary is architectural, not advisory),
`idempotentHint: true`, `destructiveHint: false`, `openWorldHint: false`
except `capture_run` (GPU/sidecars: openWorld true, documented).
Resources (host-injected): spec text (charter/glossary excerpts), gate
output excerpts. Prompts: `verify-before-close` workflow, `repro-recipe`.

## Hard problems, decided
- **GPU serialization:** no protocol lock primitive → server-side mutex,
  one GPU slot; busy returns `isError: true` "GPU busy, retry in Xs" so
  models back off instead of hammering.
- **Long runs:** stream `notifications/progress` (total = test count) at
  least every 15 s; client timeout = ceiling + margin, reset on progress.
  Tasks extension stays `optional` until the harness client commits.
- **Cancellation:** forward to the child process group (SIGTERM→SIGKILL),
  reap, tolerate cancel-after-complete races. Test the cancel path.
- **Context bloat:** distill server-side (last 30 lines + failure list +
  counts); never paste full suite output.
- **stdout purity:** protocol bytes only; all logs to stderr.
- **Screenshots (phase 2):** `image` blocks work; downscale server-side,
  full frames via resource link; gate on display availability
  (Xvfb recipe documented this session); headless CI reports absence with
  reason. Game-frame capture (Xvfb + input driving) stays a human recipe
  until phase 2 — no fake "screenshot tool" that returns black frames.

## Rollout
- **Phase 0 (tracer):** `verify_docs` + `verify_crate_tests` + `board_state`.
  Acceptance: an agent verifies a docs+unit wave using ONLY these tools.
- **Phase 1:** GPU tools (`capture_run`, `verify_golden`,
  `verify_validation`, `verify_gpu_timing`) + mutex + progress contract.
  Acceptance: red-prove each (break the gate, watch `isError`).
- **Phase 2:** `verify_oom`, screenshot tooling, harness mount config +
  CI parity note (which gates stay human/CI-run).
- Each phase: spec-version pin, `server/discover` snapshot test, cancel-path
  test, deny-green. File follow-ups as beads, not as doc TODOs.

## Open questions (for the harness owner, not blockers)
1. Does the host render `image` results and `notifications/progress`?
   (Decides phase-1 UX; text+structured fallback covers no.)
2. Client timeout policy for 3-minute suites — configurable per call?
3. Mount config location for a first-party stdio server in this harness.
