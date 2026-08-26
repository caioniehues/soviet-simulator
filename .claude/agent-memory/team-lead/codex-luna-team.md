---
name: codex-luna-team
description: The codex parity roster — 15 roles in .codex/agents/*.toml pinned gpt-5.6-luna/medium, driven via codex exec; Fable orchestrates
metadata:
  type: project
---

The project has a full codex mirror of the Claude roster: `.codex/agents/*.toml` (15 roles,
same names) pinned `model = "gpt-5.6-luna"`, `model_reasoning_effort = "medium"` (user
decisions 2026-08-26). Personas are NOT duplicated — each TOML's developer_instructions says
to read `.codex/agents/ROLE_ADAPTER.md` then the matching `.claude/agents/<role>.md`, so the
Claude files stay the single source of truth. Sandbox split: advisors/gates read-only,
implementers workspace-write. `~/.codex/agents/` holds 6 generics; `~/.codex/config.toml` has
multi_agent enabled with luna/medium subagent defaults, plus v2 profiles
(`~/.codex/{deep,fast,research}.config.toml`, `--profile <name>`).

**Why:** codex bills OpenAI — near-zero Anthropic tokens ([[feedback-herdr-codex-only]]).
Cross-vendor review already proved out: found 2 real pre-fork P2 bugs on its first gate run.

**How to apply:** scripted workers via `codex exec --profile <role> --json -o <file>`
(deterministic output, profiles+AGENTS.md apply same as TUI; `codex review` exists for the
review lane); interactive via herdr pane per [[feedback-herdr-no-polling]]. A single codex
session can itself spawn all 15 roles (verified live 2026-08-26). Codex reads CLAUDE.md as
project doc (`project_doc_fallback_filenames`). Trap: codex reviewers skip HEAD-vs-diff
attribution — every cross-vendor finding gets `git show HEAD:` scope adjudication before a
send-back is honored.
