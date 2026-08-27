# Controlled documentation rewrite plan

**Kind:** plan
**Authority:** operational
**Status:** superseded — rewrite completed, cutover commit `b6381a5`; see `docs/plan/iterations/RESUME.md`
**Owner:** project lead
**Last verified:** 2026-08-24

## Outcome

Archive the pre-fork and derived corpus unchanged, then author one current Egregoria-aware source
of truth. This is a rewrite, not a path substitution. No old behavior claim survives without one of
these inputs:

1. binding scope from the rewritten 1.0 charter;
2. current Rust, Lua, prototype, save, UI, and test evidence in a Phase 0 fact-sheet;
3. an explicitly labelled external observation or research source;
4. a newly ratified project decision;
5. a clearly unresolved question that makes no implementation claim.

## Lead rulings that bind the rewrite

| Conflict | Ruling |
|---|---|
| README's domestic nal/beznal circuits versus the charter and glossary | Domestic clearing has no money. The single rouble exists only at the border. |
| Bevy ladder items marked complete | Preserve their design scope, but reset completion for the Egregoria fork until current code and player-facing evidence prove it. |
| `1 deferred` in generated roadmap versus `19 deferred` in RESUME | Preserve neither count as authority. Recompute from the rewritten charter and requirements. |
| CS1 and W&R research described as mechanism authority | Retain as comparison/provenance evidence only. Current fork behavior comes from current code; adopted design requires a current spec or decision. |
| Save encode/decode described as determinism proof | Call it serialization round-trip stability. Determinism requires separate repeat-run evidence. |

The charter's Post-1.0 and Never lists remain binding. Crime, dual currency, vehicle manufacture and
fuel lifecycle, voltage tiers, electric-heating fallback, perishables, and the other named cuts may
be recorded as deferred direction but cannot acquire 1.0 acceptance criteria.

## Rewrite firewall

Every active specification uses stable claim anchors such as `SPEC-LOGISTICS-001`. Requirements
cite those anchors instead of mutable line numbers. Runtime claims additionally cite current
`path:line` evidence. References flow in one direction:

```text
charter + glossary + current substrate fact-sheets
                    ↓
             specifications
                    ↓
              requirements
                    ↓
       scenarios and evidence bindings
                    ↓
          generated roadmap and status
```

No generated file, RESUME handoff, old ADR, or archived document can establish scope or mechanism
upstream. `bd` remains the only task-state authority.

## Wave 1 — archive, foundations, and physical core

**Estimated effort:** 6–9 agent-hours. **Writers:** at most two concurrently.

### Phase 0 mapping

Before a rewrite brief makes substrate claims, dispatch `substrate-cartographer` on these seams:

- fixed tick, commands, scheduler, save/load, and presentation consumption;
- items, recipes, companies, market matching, border behavior, and quantity ownership;
- roads, lanes, routing, traffic, parking, dispatch, trucks, trains, and freight stations;
- Lua/prototype loading and which declarations have production consumers.

`kornai-economist` signs off the clearing/economy model. `logistics-modeller` signs off physical
transfer and finite-vehicle contracts.

### Outputs

| Lane | Files |
|---|---|
| Archive | Execute the historical/provenance section of `documentation-migration-manifest.md`; old content is moved without semantic edits and marked historical. |
| Foundations | `docs/README.md`, `docs/reference/glossary.md`, root `CONTEXT.md` redirect, `docs/plan/charter-1.0.md`, `docs/reference/architecture/substrate.md`, `docs/decisions/README.md`, and five templates. |
| Core A | Rewrite `resources.md`, `production.md`, `logistics.md`, `vehicles.md`, and `trade.md`. |
| Core B | Rewrite `needs.md`, `roads.md`, `pathfinding.md`, and `traffic.md`. |
| Index | Rewrite `docs/reference/specifications/README.md` with authority, evidence, lifecycle, and claim-anchor rules. |

Core A and Core B may run concurrently after the foundation contract is committed. Each writer owns
only its listed files. Shared indexes and glossary changes are serialized through the lead.

### Gate

1. Every runtime assertion has a current fact-sheet citation; every external comparison is labelled.
2. The ledger checker reviews every ownership transition stated by the five physical-economy specs.
3. Reviewer and both domain advisors disposition every finding as fixed, accepted, or filed.
4. `doc-reality-auditor` finds no active Bevy path, pre-fork ADR authority, or reverse authority link.
5. `cargo test -p simulation -- --test-threads=1` establishes no substrate regression; it does not by itself validate prose.

## Wave 2 — construction, settlement, services, and utilities

**Estimated effort:** 8–12 agent-hours. **Writers:** at most two concurrently.

### Authoring order

```text
construction → buildings → zoning
needs → households → citizens → education → healthcare
resources + production + buildings → electricity + water → sewage + heating
resources + logistics + healthcare → waste
crime → Post-1.0 direction only
```

The arrows are document-contract prerequisites, not claims about current implementation.

### Phase 0 and ownership lanes

| Lane | Fact-sheet and advisor | Owned specifications |
|---|---|---|
| 2A built world | Map placement, building activation, construction state, material delivery, prototype declarations, UI tools; `logistics-modeller`. | `construction.md`, `buildings.md`, `zoning.md` |
| 2B settlement | Soul identity/lifecycle, household/residence/work bindings, needs, trips, education and health UI; `settlement-modeller`. | `households.md`, `citizens.md`, `education.md`, `healthcare.md`, `crime.md` |
| 2C utilities | Current network/cache types, producers/consumers, save coverage, Lua connections, dispatch reuse and consequences; `utilities-modeller`. | `electricity.md`, `water.md`, `sewage.md`, `heating.md`, `waste.md` |

Run 2A and 2B concurrently. Run 2C after their shared building, production, household, and health
interfaces are fixed. `ledger-invariant-checker` reviews construction materials, tanker/waste
transfers, and every other stated conservation seam.

### Gate

1. `evidence-auditor` proves any test promoted as evidence can fail under a relevant mutation.
2. `wiring-auditor` confirms cited APIs and data declarations are reachable from the running game.
3. Reviewer enforces physical causality, queue/shortage failure, no domestic price clearing, and no game-over path.
4. Each relevant domain advisor signs only its cluster after independent source review.
5. `doc-reality-auditor` verifies metadata, claim anchors, cross-links, and explicit Post-1.0 cuts across all 22 subsystem specs.

## Wave 3 — re-derive the plan and cut over discovery

**Estimated effort:** 6–10 agent-hours. **Writers:** at most two concurrently.

### Rebuild rather than patch

| Output | Rule |
|---|---|
| `docs/plan/traceability/story-migration.md` | Map every old STORY ID to retained, rewritten, split, deferred, or retired; no silent disappearance. |
| `docs/plan/iterations/requirements/` | Derive acceptance criteria from charter scope and stable spec claim anchors; preserve an ID only when its meaning survives. |
| `docs/plan/iterations/extract/` | Re-extract structured inputs and validate all domain files; schema success is not semantic proof. |
| `docs/plan/iterations/evidence/` | Rebuild scenarios, corpus, and coverage with real commands/test bindings; no promoted sentinel may remain `TBD`. |
| `docs/generated/iterations/roadmap.md` | Rewrite the generator to consume requirement metadata, validate totals, and regenerate the file at its new path. |

Reconstruct `docs/plan/iterations/RESUME.md` from current `bd` state, verified commits, executed test
commands, and regenerated counts. Do not copy narrative status forward.

### Final reconciliation and cutover

- Rewrite `docs/process/development-cycle.md` only where the new artifact paths or generation rules changed.
- Reconcile `docs/reference/art-direction.md` with the current renderer and asset pipeline.
- Update root `README.md`, `AGENTS.md`, `CLAUDE.md`, and `NOTICE.md` in the same commit as canonical path cutover.
- Move the documentation research and this plan to their approved final locations.
- Keep `.claude/**`, `.codex/**`, `.beads/**`, package-local READMEs, and runtime `.txt` data in place.

### Final gate order

1. `evidence-auditor` verifies executable evidence and rejects zero-test filters.
2. `wiring-auditor`, then conditional `ledger-invariant-checker`, then `reviewer` and domain sign-offs.
3. Lead dispositions every finding; no orphaned review comments.
4. `doc-reality-auditor` checks docs, agent definitions, `bd`, generated counts, paths, and current code.
5. `scribe` captures only durable enforcement lessons after the corpus is clean.

## Mechanical completion checks

- No active reference to root `spec/`, root `architecture/`, `docs/adr/`, or
  `docs/superpowers/iterations/` remains outside archive snapshots and migration history.
- Every active document has Kind, Authority, Status, Owner, and Last verified fields.
- Every live requirement resolves to charter scope and at least one stable specification claim.
- Every promoted scenario resolves to an executable command that runs at least one test.
- Generated artifacts reproduce byte-for-byte in a clean temporary output location.

The existing simulation verification command remains:

```bash
cargo test -p simulation -- --test-threads=1
```

The total estimate is **20–31 agent-hours across three waves**. This replaces the earlier
12–20-hour estimate because the authority audit confirmed all 22 subsystem specs and the entire
derived planning corpus require reconstruction, not editing.

## Execution boundary

Approval of this plan authorizes Wave 1 only after its Phase 0 briefs exist. It does not authorize
rewriting all three waves in parallel, changing simulation code, or silently deciding unresolved
domain behavior. Each wave closes its gates before the next wave begins.
