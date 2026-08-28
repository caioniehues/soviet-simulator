---
name: graph-zero-callers-is-a-lie
description: code-review-graph returned 0 callers for `unpark` when grep found 3 — always cross-check a graph zero with grep before concluding "no callers"
metadata:
  type: feedback
---

Never close a "who calls this?" question on a zero from `query_graph_tool`. Cross-check
every graph zero with `grep -rn '\b<symbol>\b' --include=*.rs .` before acting on it.

**Why:** measured 2026-08-28 on sov-6qx. Both `callers_of` and `references_to` on
`unpark` returned `result_count: 0` for
`simulation/src/transportation/vehicle.rs::unpark`. grep found **three** production
callers (`map_dynamic/router.rs`, `economy/market.rs`, `world_command.rs`). The ticket
required handling every caller; acting on the graph's zero would have left two unfixed
and the bug half-landed. The graph did flag itself stale in its `confidence` field
(`built_at_sha` 2cc7331 vs HEAD 345a79a) — but a zero reads like a true negative whether
or not that field is there. Same failure shape as the earlier `Market::set_requested`
incident recorded in the repo's `docs/reference/code-intelligence.md`.

**How to apply:** the graph is for narrowing scope, never for proving absence. Prefer it
for "what is nearby / what flows touch this"; use grep for exhaustive caller enumeration.
A compiler-enforced signal beats both: making `unpark` return `#[must_use] bool` turned
"did I find every caller?" into a build error rather than a search problem.

Related: [[binfos-is-not-a-liveness-oracle]].
