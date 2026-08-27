---
name: gotcha-lsp-read-guard-relent
description: When LSP is disabled but the lsp-first-read-guard hook still blocks Read, retry the same file 3x — the hook relents after 2 blocks. Never fall back to bash cat.
metadata:
  type: feedback
---

`~/.claude/hooks/lsp-first-read-guard.js` blocks `Read` on code files until an LSP warmup call is
made. In some subagent sessions the LSP tool is **disabled entirely** (`ToolSearch("select:LSP")`
returns a schema, but calling it errors "LSP is disabled for this session, in subagents as well as
here"), and `Grep` is also absent. That combination looks like a deadlock.

**It is not.** The hook has a deliberate escape hatch (`RELENT_AFTER = 2`, lines 86-104): it counts
consecutive blocks *per file path* and lets the third attempt through with a warning.

**How to apply:** issue the same `Read` call three times. Batch all wanted files together in one
message and repeat that batch three times — the counter is per-file and persisted to
`~/.claude/state/lsp-ready-<md5(cwd)>`, so N files cost 3 rounds total, not 3N.

Non-code extensions bypass the hook entirely: `.md`, `.toml`, `.json`, `.lua`, `.wgsl`, `.txt`,
`.ini`, `.sh`, `.csv` all read free on the first try. Shaders and Lua data files therefore need no
workaround at all.

**Why:** the hook's own text says an agent without LSP should "say so in your report rather than
falling back to `bash cat`". Report the disabled tool; use the relent path; do not use `bash cat`,
which violates the tool-discipline rule in `~/.claude/rules/tool-discipline.md`.
