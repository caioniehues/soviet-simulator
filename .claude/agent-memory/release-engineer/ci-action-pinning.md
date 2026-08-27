---
name: ci-action-pinning
description: Why GitHub Actions here are pinned to commit shas, the Node 20 deprecation this created, and the resolved checkout sha to bump to
metadata:
  type: project
---

Every `uses:` in this repo's workflows is pinned to a **commit sha**, never a
tag, for the same reason `cargo-deny` is version-pinned: a moving tag is not a
reproducible input.

**Why:** the whole dependency-policy gate exists because this project already
depends on upstream git HEADs that someone else can move. A workflow that
re-pointed itself on a tag push would reintroduce exactly that failure into the
gate meant to prevent it.

**How to apply:** the cost of that choice is that runtime deprecations need a
MANUAL bump. Both real runs on 2026-08-27 carried:

```
Node.js 20 is deprecated. The following actions target Node.js 20 but are being
forced to run on Node.js 24: actions/checkout@11d5960a326750d5838078e36cf38b85af677262.
```

It is a **warning today, a hard breakage later** — the runner force-runs the v4
action on Node 24, and the workflow breaks when GitHub removes that forcing.

Resolved shas (`action.yml` `runs: using:` read from each tag):

| Tag | sha | runtime |
| --- | --- | --- |
| v4.4.0 | `11d5960a326750d5838078e36cf38b85af677262` | node20 (deprecated) |
| v5.1.0 | `fbc6f3992d24b796d5a048ff273f7fcc4a7b6c09` | node24 |
| v6.1.0, v7.0.1 | — | node24 |

v5.1.0 is the smallest step that clears the warning. Resolve a tag to a sha with
`gh api repos/actions/checkout/git/ref/tags/<tag> --jq .object.sha`, and read the
runtime with `gh api repos/actions/checkout/contents/action.yml?ref=<tag> --jq .content | base64 -d | tail -8`.

Related: [[verification-procedures]], [[dependency-policy-baseline]].
