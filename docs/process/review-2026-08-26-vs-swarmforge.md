# Process review — our dev cycle vs SwarmForge

**Kind:** process audit
**Authority:** advisory (findings feed decisions; `development-cycle.md` remains sovereign)
**Date:** 2026-08-26
**Evidence:** three extraction sweeps — full SwarmForge read (main + two/four/six-pack branches, plus gate-verification spot-reads of `squad` and `adversaries`), full read of `development-cycle.md` + all 15 agent files + `gate-review.js` + gate-chain formula, and practice evidence from live `bd`, `git log -60`, and the team-lead memory/friction files. Discrepancy claims were re-verified by the lead before writing; the whole document then passed an opus gate that re-derived every factual claim from source (verdict: approve-with-fixes; all fixes applied below).

---

## Verdict in one paragraph

Our cycle's core bet — **coordination lives in a reasoning lead; quality lives in adversarial, re-deriving gates** — is working and measurably so. SwarmForge's core bet — **coordination lives in mechanism (daemon, inboxes, cards); quality lives in a fixed tool-enforced pipeline** — solves exactly the problems we have logged as friction, and fails at exactly the thing we are best at. The right move is not to adopt its shape but to steal its mechanization of the boring parts: completion signalling, machine-filled commit metadata, and tool-enforced mutation testing. Meanwhile our own process docs have drifted from our own artifacts in three confirmed places — embarrassing for a cycle that fields a `doc-reality-auditor`.

---

## Part 1 — Critical audit of our cycle

### What is provably working (keep, do not touch)

1. **The adversarial gate catches real defects.** Measured, repeatedly: the specialist `ledger-invariant-checker` found 5 CONFIRMED findings for 100.8k tokens where the general opus gate found 2 for 112k on the same seam, and the three missed ones were substantive (unreleased reservation, dispatch wedge, free-goods credit). `sov-dispatch-wedge-ab4`'s thread shows a genuine multi-round cycle: implementer → ledger checker "BROKEN" with a concrete conservation failure → reviewer SEND-BACK → fixes → APPROVE-WITH-FIXES that still surfaced a new truck leak → codex cross-vendor SEND-BACK → lead adjudication (findings real but pre-fork; filed as sov-jcl/sov-xyx). That is the system doing what it claims.
2. **Close-reason discipline is near-universal.** Of 33 closed issues, one discipline failure (`sov-xie`, docs left uncommitted, no sha) plus three scope *retirements* (`sov-iter0000-wrapup-xwe`, `sov-charter-amend-130-nl0`, and borderline `sov-scenario-coverage-bt0`) closed by pointer to the deciding document rather than by evidence — a small gap of its own: a retirement close reason should cite the deciding artifact's id. Every other close cites a commit sha, a red-then-green differential test with real output, or both. Tracker hygiene is clean: 0 stale, 0 orphans, no P1 inflation among open issues.
3. **Phase 0 grounding earns its cost.** The three founding failures (commented-out truck registration, the false "copy the train pattern" brief, the unread `optout_exttrade` flag) all share one root — briefs asserting substrate facts nobody had read — and the fact-sheet requirement is the direct fix. The inherited-claims incident (~20 dispatches poisoned by two false RESUME.md facts) proves the failure mode is real and expensive.
4. **Cheap-to-expensive gate ordering.** The wiring-auditor blind-reproducing two opus findings plus one new one, at a fraction of the cost, validates both the ordering and the "narrow scope, never depth" rule.

### What is broken or drifting (findings)

**F1 — The process doc contradicts its own artifacts. CONFIRMED, three instances.**
- `development-cycle.md`'s roster table lists the substrate-cartographer and four domain advisors at **opus** (soviet-authenticity is correctly sonnet); every one of those files' frontmatter pins `model: sonnet` (lead-verified by grep, gate-reverified across all 15 files). The doc's own framing quote — "sonnet implements, opus reviews and advises" — is contradicted by the files for the *advises* half, and these agents hold Phase-4 sign-off gates.
- `.beads/formulas/gate-chain.formula.toml` pours wiring → domain → review; the Phase 4 table lists `ledger-invariant-checker` as gate #2. The formula silently drops a gate for exactly the diffs (economy) where the specialist demonstrably outperforms the general reviewer (F-context: 5-vs-2 measurement above).
- The `Executed-By:` commit-trailer convention in CLAUDE.md is dead: **0 of the last 60 commits carry it** (lead-verified). A documented convention that never fires trains agents to distrust the doc.

The irony is structural: `doc-reality-auditor` sweeps docs against code in Phase 6, but the process layer (roster table, formulas, CLAUDE.md conventions, hooks) is evidently not in its effective sweep, or the sweep hasn't run since these drifted.

**F2 — The micro layer only exists under wave pressure.** The multi-agent stories carry rich comment threads; **all ten currently open issues have no comments at all** (gate-verified per id). The convention "log a comment when you learn something the next agent would rediscover" holds when the lead is orchestrating a wave and evaporates for single-agent work. Either that's fine (comments are a multi-agent coordination artifact) and the doc should say so, or it's drift.

**F3 — Completion signalling is prompt-discipline, not mechanism.** The friction log records it directly: "Subagent finished reports don't reach the lead on their own — a gate's one-line 'APPROVE' idle summary arrived but the full report did not"; herdr restarts silently swallowed prompts; `agent wait` resolves on momentary idle. Every worker report reaching `main` depends on the brief remembering to say so. This is our largest structural gap vs SwarmForge (see Part 2).

**F4 — No documented light path.** The goal-drift incident is on record: ~400 lines of production logic against ~2,900 lines of process documentation in one session while "ITER-0000 end to end" stayed open; the cycle bills ~675k tokens per iteration of which Build is ~360k. The gap: the cycle has an answer for everything except "is this iteration too small to deserve the full cycle?" — no two-pack equivalent, no light path for a one-file fix. In practice the lead improvises one; undocumented improvisation is where the F1-style drift comes from.

**F5 — No named dispute procedure.** The codex SEND-BACK adjudication worked, and the "not a veto" clause in advisor files covers advisor-vs-lead. But reviewer-vs-reviewer disagreement, or a worker disputing a gate verdict, has no written path — it resolves as "the lead decides," which is fine until the lead is the one who's wrong. (SwarmForge has even less here; noted for honesty, not as a comparative loss.)

**F6 — Infra can starve the fleet silently.** The LSP-guard incident: 4 of 15 workers hard-blocked, 7 degraded, by a hook defect the workers couldn't see or name — they fell back to `cat`/grep and soldiered on. The patch is installed (sov-lsp-guard-deadlock-qak closed with a differential test), but the lesson generalizes: nothing in the cycle audits *the harness itself*. A worker that silently degrades under a broken hook produces plausible-but-weaker work, which is the hardest defect class for the gates to see.

---

## Part 2 — SwarmForge, critically

SwarmForge (Uncle Bob's tmux orchestration platform) runs fixed role pipelines (two/four/six-pack) in per-role git worktrees, coordinated by a daemon that moves validated handoff files between per-agent inbox/outbox directories, with a kanban cockpit whose cards move only on delivered handoffs. A layered "constitution" (shared articles + per-branch `local-*` additions or same-name overrides) encodes the engineering rules.

### Where it is genuinely better than us

1. **Completion is structural, not reported.** A card reaches Done only when the terminal role's `git_handoff` — addressed to every other role — is *delivered by the daemon*. There is no "the worker forgot to message the lead." The handoff file schema is validated (two message types, exactly-10-char commit abbrev, priority), written atomically (tmp → rename), and the helper **fills the commit sha from the sender's worktree HEAD — "do not type a SHA."** Compare our F3 and our dead Executed-By trailer: they mechanized precisely the two things we left to agent memory.
2. **Machine-stamped provenance works.** Their commit-msg hook auto-appends "By \<role\>." to every commit. Same shape as our Executed-By hook — except theirs is load-bearing and ours provably never fires.
3. **Tool-enforced quality floors.** CRAP thresholds are numbers a tool checks (the "CRAP ≤ 6" figure lives on the `squad` branch; the shipped packs reference the CRAP tool without that number); mutation testing is a tool run, differential against a manifest — `--max-workers 8` in four-pack's architect and six-pack's hardender prompts, while main's `engineering.prompt` says 4; the packs and main disagree — not an agent promising it mutated something. Their constitution even bans homegrown substitutes: "Do not invent project-local CRAP, DRY, mutation, or coverage proxies." Our evidence-auditor's seen-failing rule is *deeper* per guard, but it is prompt-discipline executed by a sonnet agent; theirs cannot be skipped by a lazy or starved agent.
4. **Constitution composition beats precedence prose.** Shared articles install unless a same-named local file overrides; `local-*.prompt` files extend without replacing. That is cleaner and more auditable than our stack of "this section OVERRIDES two rules in the managed block below, per that block's own precedence clause" prose, which requires a careful read to even parse.
5. **Right-sized pipelines exist.** Two-pack for small tasks, six-pack for UI-heavy work — the operator picks the weight. We have exactly one weight (see F4).

### Where it is clearly worse than us

1. **No blind re-derivation in any shipped pack.** Every reviewer in the chain (cleaner, refactorer, architect, hardender, QA) reviews the pipeline's own output *sequentially, with full knowledge of the producer's work*. An `adversaries` branch exists (a coder+reviewer pack with an explicitly adversarial reviewer prompt), but it is stale (last commit 2026-06-26, two months behind main), undocumented in the README's pack list, and even its reviewer works sequentially with full sight of the producer. Nowhere: blind reviewers, skeptics prompted to refute, cross-vendor checks, or a "re-derive from primary sources" rule. By our own claudelic axiom — a verifier that sees the producer's conclusion grades its own work — the entire SwarmForge quality chain is graded homework. Our `gate-review.js` (blind dimensions → 2 refuting skeptics per finding → completeness critic → decisive verdict) is a generation ahead, and our 5-vs-2 measurement suggests the gap is real, not aesthetic.
2. **Zero domain knowledge in roles.** Their roles are generic engineering functions. Our roster's actual moat is agents that *know things*: the kornai-economist's queue-not-price rulings, the cartographer's three-source fact-sheets, the trap inventories in every implementer file ("trucks are not trains", `Lot::generate_along_road` is NOT disabled). Nothing in SwarmForge can tell an agent its brief is factually wrong about the substrate.
3. **Every task pays the full pipeline.** Within a pack, no phase-skipping — contrast our "a diff that lands exactly what Phase 0 approved skips the domain gate." Their only flexibility is choosing the pack up front.
4. **Failure handling is thinner than ours, by their own record.** The single entry in their issues.md is a wake-up deadlock (approval clears the sender's block but only the receiver is notified; tasks sat stuck until a human prompted). No stall detection, no timeout, no arbitration — "ask the operator" is the entire escalation path. Interestingly this is the *same defect class* as our herdr friction #1/#5 (lost prompts, waits resolving on momentary idle): event-driven agent wake-up is hard for everyone, and their daemon didn't solve it either — it just made the failure legible in an inbox instead of invisible in a pane.
5. **No practice evidence.** Our bd trail lets this audit check claimed-vs-actual. Their repo has one logged bug and no equivalent audit surface; the constitution may be honored or ignored and nothing would show it.
6. **Tmux-pane economics.** We already ran this experiment: herdr panes ignore model pins and cost too much; the standing rule is panes for codex only. Adopting SwarmForge's runtime would re-fight a settled decision.

---

## Part 3 — Recommendations

Ordered by confidence.

1. **Fix the three F1 drifts.**
   a. Reconcile advisor tier — **this is a cost decision, not a mechanical fix**. The table says opus, all files pin sonnet. Arguments for files → opus: delegation.md's "opus for gates" and the advisors' Phase-4 sign-off role. Argument against: the project's own strongest measurement (feedback-specialist-gates.md) concludes "build the specialist rather than upgrading the generalist", and five standing opus advisors are a real bill on a one-person project. Either way, make table and files agree and record the why.
   b. Add a conditional ledger step to `gate-chain.formula.toml` (or a second `gate-chain-economy` formula) so the highest-yield gate isn't the one the template omits.
   c. `Executed-By`: the gate tested this directly — **bd 1.2.2's prepare-commit-msg hook is inert even with `BEADS_ACTOR` set** (direct invocation leaves the message byte-identical; the env var isn't in the binary's strings; three commits made after the hook's install carry no trailer). This is the known 1.2.2 recovery-release gap. Fix: either a three-line project-local prepare-commit-msg hook that appends the trailer itself (SwarmForge's "By \<role\>." hook proves the shape), or delete the convention from CLAUDE.md. Do not send anyone to "set BEADS_ACTOR and re-check."
2. **Point doc-reality-auditor at the process layer.** Add to its Phase 6 sweep: roster table vs agent frontmatter, formulas vs the Phase 4 table, CLAUDE.md conventions vs observable git/bd state (trailers, export freshness). This audit found all three drifts with three greps; make those greps standing.
3. **Resolve F2 explicitly.** Recommended: document the narrower true rule — bd comments are mandatory for multi-agent stories and for any discovered-wrong-premise, optional otherwise. Matches actual practice; costs one paragraph.
4. **Steal: mechanized completion metadata.** A tiny helper (or brief boilerplate) that makes the worker's closing act mechanical: `bd close <id> --reason "commit $(git rev-parse --short=10 HEAD): <proof>"` with the sha command substituted, never typed. Same spirit as their "helper fills commit; do not type a SHA." Cheap, kills the sov-xie failure shape.
5. **Steal: tool-enforced mutation testing.** Trial `cargo-mutants` on `simulation/` as a Phase 3 *floor* under evidence-auditor (who keeps the deeper seen-failing work on new guards). A tool that runs cannot be starved by a broken hook (F6) or a lazy agent. Adopt their guardrail too: no homegrown proxy for what the tool measures.
6. **Steal: a documented light path.** A "two-pack equivalent": for a single-file, non-economy, non-charter-touching change — implementer → wiring-auditor → opus reviewer, no Phase 0 re-grounding (fact-sheets already exist), no domain gate. One table row in development-cycle.md. This removes the incentive to improvise off-book, which is where drift breeds.
7. **Decision needed — nothing else.** Do **not** adopt: the daemon/tmux runtime (settled: herdr codex-only), fixed pipelines (our phase-skipping is smarter), the cockpit (bd + Claude tasks cover it), Gherkin (our acceptance criteria + evidence bindings serve the same role with less ceremony). The platoon concept (their unbuilt multi-squad layer) is worth one glance if this project ever splits into independently deployable components; it hasn't.

