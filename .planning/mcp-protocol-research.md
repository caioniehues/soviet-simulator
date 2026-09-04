# MCP protocol research (2026-09-04)

Why MCP fits a verification-tool integration, per primary sources
(spec revision **2026-07-28**; legacy `initialize` handshakes superseded).
All URLs verified 2026-09-04.

## 1. Architecture
- **Host** (Claude Code, VS Code, this harness): owns the model, policy,
  and one MCP client per server. Decides what the model sees, when to
  confirm, how results re-enter context.
  https://modelcontextprotocol.io/docs/2026-07-28/learn/architecture
- **Client**: per-server connection (`tools/list`, `tools/call`,
  `resources/read`, …). One host fans out to N clients. Also serves
  client-side primitives back (elicitation).
- **Server**: a role, not a location — stdio subprocess and remote HTTPS
  endpoint are both servers. Declares capabilities via `server/discover`;
  every request carries version + capabilities + identity inline in `_meta`
  (stateless → any request retryable against a fresh process).
- **Tools vs resources vs prompts** (who decides): tools are
  **model-controlled** (the agent invokes: `cargo test`); resources are
  **application-driven** (host injects: build-log excerpts, spec text);
  prompts are **user-controlled** (slash-command templates: repro recipes).
  https://modelcontextprotocol.io/specification/2026-07-28/server/tools ·
  …/server/resources · …/server/prompts
- **Transports**: stdio (subprocess, newline JSON-RPC, logs on stderr,
  crash recovery = restart + retry) vs streamable HTTP (one POST per
  message, optional SSE progress, needs Origin validation + auth; legacy
  HTTP+SSE superseded). **Local coding loop: stdio, unambiguously** — no
  ports, no auth ceremony, no DNS-rebinding surface.
  https://modelcontextprotocol.io/specification/2026-07-28/basic/transports/stdio ·
  …/transports/streamable-http

## 2. Server SDKs (Sept 2026) → Rust + rmcp
| Option | Standing |
|---|---|
| TypeScript SDK | v2 stable, Tier 1 (`@modelcontextprotocol/server`) |
| Python official `mcp` | 2.0.0 default (`MCPServer`; v1 maintenance) |
| Python FastMCP (PrefectHQ) | 3.x GA; composition/proxying/OpenAPI strengths |
| **Rust `rmcp`** | **3.2.0, Tier 1 since 2026-08-21** (67/67 server + 50/50 client conformance), MSRV 1.88 |

https://crates.io/crates/rmcp · https://github.com/modelcontextprotocol/rust-sdk ·
tiers https://modelcontextprotocol.io/community/sdk-tiers
Recommendation: rmcp — full conformance evidence, `tokio::process` fits
thin cargo wrappers, single static stdio binary (trivially sandboxed,
pinned, reproducible). FastMCP only if server composition is ever needed;
TypeScript only if the harness owns server code in its own language.

## 3. Tool mechanics
- **Schemas**: `name` (1–128 chars, unique; aggregators prefix on
  collision), `title`/`description` (the model's selection prompt),
  `inputSchema` (JSON Schema object; no-param = `{"type":"object",
  "additionalProperties":false}`), optional `outputSchema`.
- **Results**: `content[]` (text / base64 `image` / resource_link) for
  models + `structuredContent` (validated JSON) for machines — return
  **both**. Verification mapping: summary + failure excerpt as text,
  counts/durations as structured, screenshots as image blocks.
- **Progress**: client opts in per request (`_meta.progressToken`); server
  MAY emit `notifications/progress` (monotonic). A 3-minute suite streams
  total = test count. Both sides rate-limit.
  https://modelcontextprotocol.io/specification/2026-07-28/basic/patterns/progress
- **Timeouts/cancellation**: no protocol timeouts — implementations SHOULD
  set per-request timeouts, reset on progress, enforce a max. Cancel is
  transport-specific (stdio: `notifications/cancelled`; HTTP: close SSE).
  Servers SHOULD stop work and free resources; races must be tolerated.
- **Errors**: protocol errors (bad tool/args) = JSON-RPC `error`; tool
  execution errors (test failures) = normal result with `isError: true` +
  actionable text. **A failing suite is `isError:true`, never a protocol
  error.** Servers MUST validate inputs, rate-limit, sanitize outputs.
- **Tasks extension (experimental, 2025-11-25+)**: `tools/call` with
  `task={ttl}` returns immediately; client polls `tasks/get`. Elegant for
  3-minute suites but two-sided — ship `optional` + progress fallback
  until the harness client commits.
  https://modelcontextprotocol.io/specification/2025-11-25/basic/utilities/tasks

## 4. Sandboxing command-executing tools
(Converged guidance: spec + OWASP cheat sheet
https://cheatsheetseries.owasp.org/cheatsheets/MCP_Security_Cheat_Sheet.html ·
https://modelcontextprotocol.io/docs/2026-07-28/tutorials/security/security_best_practices)
1. **No shell, ever** — argv-spawn; never interpolate model strings
   (the `shell=True` CVE class: CVE-2025-5277, CVE-2025-53818 pattern).
2. **Confinement + allowlist** — one tool per check, cwd pinned, reject
   `..`/absolute/symlink escapes, no generic `run_command`, strict schemas.
3. **Least privilege** — unprivileged uid, read-only FS except `target/` +
   temp, scrubbed env, per-tool timeouts, concurrency cap 1 for GPU work.
4. **Inputs AND outputs untrusted** — LLM-originated inputs get validated;
   outputs re-enter context (strip instruction-like patterns); tool
   definitions themselves are an injection surface (pin by hash, consider
   `mcp-scan`: https://github.com/invariantlabs-ai/mcp-scan).
5. **Local hardening** — stdio removes the remote surface; if HTTP ever:
   127.0.0.1, Origin/Host validation, TLS + OAuth2/PKCE.
6. **Auditability** — log every invocation (args, context, timestamps);
   human confirmation for anything destructive (a verify server should
   have nothing destructive — that is the point).

## 5. Discovery, annotations, verify-vs-fix
- **Discovery**: `tools/list` (paginated, cacheable, deterministic order
  for prompt-cache hits) → `tools/call`; `list_changed` notifications.
  Set MUST NOT vary per-connection.
- **Annotations are hints, not contracts**
  (`readOnlyHint` default false, `destructiveHint` default true,
  `idempotentHint` false, `openWorldHint` true — worst-case defaults).
  A lying server can claim read-only and delete anyway; clients MUST treat
  them as untrusted unless first-party. Real uses: confirmation gating,
  UX filtering, policy-engine input.
  https://blog.modelcontextprotocol.io/posts/2026-03-16-tool-annotations/
- **Verify-vs-fix**: (A) one server with honest annotations (auto-approve
  read-only, gate mutating on confirmation); or (B, RECOMMENDED) **two
  toolsets** — a verify-only server mounted in exploration agents
  (physically incapable of mutation) and mutating tools only in edit
  agents. Annotations reinforce an architectural boundary instead of
  being one. Mid-call confirmation uses `elicitation/create`.

## 6. Long-verification gotchas
1. No server timeouts/heartbeats — contract progress-every-N-seconds +
   client max-timeout, or healthy runs get killed.
2. Cancellation is cooperative and racy — forward to the child process
   group (SIGTERM→SIGKILL), reap zombies, ignore late responses, TEST the
   cancel path.
3. stdio multiplexes everything on one pipe — summarize in-server,
   rate-limit, full logs to files + `resource_link`; NEVER non-protocol
   bytes on stdout.
4. Tasks extension needs both ends — don't require it yet.
5. No distributed lock — GPU serialization lives outside the protocol
   (server mutex, busy-then-poll as `isError` text).
6. Screenshots: `image` blocks exist but bloat context — downscale,
   diffs or on-demand full frames; no screenshot-stream primitive;
   gate on display availability.
7. Context bloat — distill server-side (last-N + failures + counts);
   duplicate JSON in text and `structuredContent`.
8. Statelessness — crash recovery free, but multi-step flows need opaque
   handles + TTL or harness-side orchestration.
9. Trust stays at the host — pin first-party servers, hash definitions,
   treat open-world output as tainted.
