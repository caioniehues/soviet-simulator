# Domain Docs

How the engineering skills should consume this repo's domain documentation when exploring the codebase.

## Before exploring, read these

- **`CONTEXT.md`** at the repo root. It is a redirect: the actual vocabulary is **`docs/reference/glossary.md`** — read that before naming a domain concept.
- **`docs/decisions/`** — the decision register. Read decisions that touch the area you're about to work in. A decision is binding only when its status is **accepted**; drafts, superseded records, and archived pre-fork ADRs carry no current authority. New decisions use `docs/templates/decision.md`.

This repo has no `docs/adr/`; do not create one. If a file listed above is missing, **proceed silently**. The `/domain-modeling` skill (reached via `/grill-with-docs` and `/improve-codebase-architecture`) creates domain docs lazily when terms or decisions actually get resolved.

## File structure

Single-context repo:

```
/
├── CONTEXT.md                     ← redirect to the glossary
├── docs/
│   ├── reference/glossary.md      ← canonical vocabulary
│   ├── decisions/                 ← decision register (accepted / draft / superseded / archived)
│   └── templates/decision.md
└── simulation/, native_app/, …
```

## Use the glossary's vocabulary

When your output names a domain concept (in an issue title, a refactor proposal, a hypothesis, a test name), use the term as defined in `docs/reference/glossary.md`. Don't drift to synonyms the glossary explicitly avoids.

If the concept you need isn't in the glossary yet, that's a signal — either you're inventing language the project doesn't use (reconsider) or there's a real gap (note it for `/domain-modeling`).

## Flag decision conflicts

If your output contradicts an accepted decision, surface it explicitly rather than silently overriding:

> _Contradicts decision `<id>` (event-sourced orders) — but worth reopening because…_
