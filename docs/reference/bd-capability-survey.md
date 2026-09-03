# bd (beads) capability survey and adoption recommendations

**Kind:** survey report
**Authority:** advisory — recommends, changes nothing; task-tracking conventions in CLAUDE.md remain operative
**Status:** delivered; recommendations adopted 2026-08-26 (`sov-xie`) — see CLAUDE.md § Adopted conventions
**Owner:** project lead
**Last verified:** 2026-09-03 (bd 1.2.2, §§2–3, 6 reconciled against live state; §§0–1, 4–5 unchanged)

Sources: live CLI exploration of the installed binary and this workspace (lead), plus upstream
docs/releases at `github.com/gastownhall/beads` (sonnet researcher, citations noted inline).
Every local claim below was run against this repo; upstream claims are cited to their doc page.

---

## 0. The one fact that reframes everything: what 1.2.2 actually is

**v1.2.2 is a recovery release — it re-releases the tested 1.1.2 code** after v1.2.0/1.2.1
shipped by accident without release testing (GitHub release notes, `gh release view v1.2.2`).
The 1.2.x-only features — **work leases, events journal, sync federation, HTTP API server,
provenance events — are NOT in our binary**, even though the docs site documents them as if
shipped (`docs/reference/events-journal.md` etc.). Live-confirmed: `bd config set
events.enabled true` → "not a recognized config key" (reverted).

Consequences:
- Treat any upstream doc page about leases/events-journal/sync-federation/HTTP-server as
  describing a future version.
- **Never run `bd upgrade` casually.** A machine that ran v1.2.1 even once had its DB schema
  silently migrated v53→v65, which 1.2.2 cannot read (`docs/RECOVERY-1.2.1.md` has the fix).
  Our DB is clean; keep it that way by pinning until a properly tested release lands.
- The Dolt 2.3.0 `DOLT_RESET` regression (upstream pins Dolt 2.2.0) does **not** affect us:
  embedded mode links its own Dolt engine (`bd info` → "Mode: direct").

## 1. Verified local state (this workspace, 2026-08-26)

| Fact | Evidence |
|---|---|
| bd 1.2.2 (6c124203e), embedded Dolt, 36 issues, prefix `sov` | `bd version`, `bd info` |
| All 5 git hooks installed as thin shims (pre-commit, post-merge, pre-push, post-checkout, prepare-commit-msg), 300s timeout, **fail open** | `bd hooks list`, read `.git/hooks/pre-commit` |
| `.beads/issues.jsonl` **in sync** with the live DB | `diff <(sort issues.jsonl) <(sort <fresh export>)` — identical |
| `export.auto` NOT set → default `false` → **the pre-commit hook does NOT auto-export jsonl**; our manual `bd export -o .beads/issues.jsonl` convention is load-bearing, keep it | `bd config list` + upstream `docs/reference/git-integration.md` |
| **Telemetry is ON**: `metrics.disabled = false`, endpoint `gastownhall-eventsapi.com/mp/collect` | `bd config list` — user decision whether to disable (`bd config set metrics.disabled true`) |
| `sync.remote` wired to `github.com/caioniehues/soviet-simulator.git` → `bd dolt push/pull` is available (history under `refs/dolt/data`, no clash with branch protection) | `bd config list`, `docs/core-concepts/sync-concepts.md` |
| No formulas, no memories stored | `bd formula list`, `bd memories` |
| **`bd doctor` does not work in embedded mode** ("not yet supported") — every doctor-based recipe in upstream docs is unavailable to us | live run |

Upstream's named sync anti-pattern, for the record: routine `bd import issues.jsonl` is NOT a
substitute for `bd dolt pull` — jsonl import is upsert-only and cannot see deletions. Our
model (Dolt = truth, jsonl = versioned passive export) matches upstream's intended design.

## 2. Full capability map

Grouped by what it is for. **Bold** = candidate for adoption (§3). Struck = ruled out.

### Issue lifecycle (in daily use already)
`create` (rich: `--acceptance`, `--design`, `--notes`, `--defer`, `--due`, `--deps
'type:id'`, `--dry-run`, batch from markdown `-f` or JSON graph `--graph`, `--body-file`),
`show`, `update --claim`, `close --reason`, `reopen`, `comments add`, `comment` (shorthand,
`--stdin`/`--file`), `note` (append to notes field), `ready`, `blocked`, `list`, `query`
(full boolean language: AND/OR/NOT, parens, `<`/`>` on dates), `search`, `count`, `status`.

Quirk worth knowing: `query status=blocked` only matches *stored* status — dependency-blocked
issues stay `open`; only `bd blocked` finds them.

### Structure
`dep add|remove|tree|cycles|relate`, **`bd dep <blocker> --blocks <blocked>`** (the
direction-safe form we already standardized on), parent-child via `create --parent`, `epic
status`, **`epic close-eligible`** (epics do NOT auto-close when the last child closes),
`graph` (terminal DAG / `--dot` / **`--html` self-contained D3 view**), `label` (incl.
`propagate` parent→children), `supersede --with`, `duplicate`, `find-duplicates`
(Jaccard, or AI with an API key).

### Hygiene & quality
- **`bd lint`** — per-type required sections: bug → Steps to Reproduce + Acceptance Criteria;
  task/feature → Acceptance Criteria; epic → Success Criteria. Extensible via
  `lint.sections.<type>`.
- **`validation.on-create|on-close|on-sync`** = `none|warn|error` (YAML-only keys) — makes
  lint run automatically at create/close time.
- **`bd stale`** (`--days`, default 30) — abandoned in_progress / forgotten open.
- **`bd orphans [--fix]`** — issues whose id appears in a **commit message** but are still
  open in the DB. Direct complement to our "cite the id in the commit" convention.
- ~~`bd doctor --check=conventions`~~ and all other doctor modes — embedded-mode blocked.
- `bd preflight` — mostly bd-repo-specific checklist (Go formatting, nix hashes); its `--fix`
  is documented but "not yet implemented". Low value here.

### Batch & scripting
- **`bd batch`** — many writes (create/close/update/dep) from stdin/file in ONE transaction +
  one Dolt commit; rollback on any error. Built exactly for the "lead sets up a wave" case.
- **`bd q "title"`** — create, print only the id. Scripting/quick capture.
- `--json` on nearly everything; `--readonly` and `--sandbox` global flags for worker
  sandboxes (block writes / block Dolt auto-push).
- `bd todo` — thin sugar over task issues; adds nothing for us.
- `bd sql` — raw SQL escape hatch against the Dolt DB.

### Agent coordination & provenance

(Superseded in part 2026-08-27 — see §5. The `--actor` / `$BEADS_ACTOR` and
`Executed-By:` trailer bullets below describe the pre-adoption hypothesis, kept for
provenance. Operative convention: attribution is `--author <roster-name>` on
`bd comments add`; do not set `BEADS_ACTOR`.)
- **`--actor` / `$BEADS_ACTOR`** on every command — audit-trail identity (defaults to git
  user.name). [SUPERSEDED by §5: the `BEADS_ACTOR`/`Executed-By:` convention was deleted
  2026-08-27 — bd 1.2.2's `prepare-commit-msg` hook is inert. Workers pass `--author`
  on comments only.]
- **`prepare-commit-msg` hook adds an `Executed-By:` trailer** when `BD_ACTOR` is set —
  free per-commit agent provenance (hook already installed). [SUPERSEDED by §5: verified
  inert — 0 of 60 commits carry the trailer. Do not cite `Executed-By:` trailers.]
- `bd audit record|label` — append-only `.beads/interactions.jsonl` for interaction logging
  / dataset generation. Niche; not needed.
- `set-state` / `state` — labeled state dimensions (`health:failing`) + event beads. Built
  for long-running daemon agents ("patrol"); overkill for our wave model.
- `gate` (human/timer/gh:run/gh:pr/bead waits), `merge-slot` (exclusive lock for merge-queue
  agents), `swarm` (parallel epic execution) — Gas-Town multi-rig machinery. Our lead-driven
  waves with an interactive user don't need async gates; skip unless we ever run unattended
  multi-session automation.
- **`bd human`** — flag-for-human-decision label + `human list|respond|dismiss|stats`.
  In-session we have AskUserQuestion; this is for **cross-session** parked questions.

### Templates: formulas → protos → molecules
(`docs/workflows/formulas.md`, `molecules.md`; live-verified command surface)
- Formula = TOML/JSON workflow template: `[vars]` (required/default/enum/pattern),
  `[[steps]]` with `needs=[...]` deps and per-step issue type, human-approval `[steps.gate]`,
  cross-cutting `[[advice]]` aspects (e.g. inject a review step before every `*.deploy`).
- `bd cook` compiles (placeholders kept, or `--var` substituted), `bd mol pour` instantiates
  as a real epic+children DAG, `bd mol wisp` as ephemeral (promote later with `bd promote`).
  `bd mol distill` extracts a formula from an ad-hoc epic that worked. `bd mol squash/burn`
  condense/discard.
- A molecule is *just an epic with deps* — the template layer is optional, and upstream says
  most work needs only epics and dependencies.
- Search paths: workspace `formulas/` → repo `.beads/formulas/` → `~/.beads/formulas/`.
- Doc-admitted gap: step-completion hooks (`on_complete.run`) are documented in old examples
  but **not wired end to end** — formulas structure work, they do not execute anything.
- Upstream's own named pitfall matches our CLAUDE.md warning verbatim in spirit: *"Temporal
  language inverts dependencies… use requirement language"* — the `bd dep` direction trap is
  upstream-acknowledged.

### Lifecycle beyond open/closed
**`bd defer [--until +1d|tomorrow] [--reason]`** (hidden from `ready`, visible in `list`;
`bd undefer` reverses), `supersede`, `prune`/`purge`/`gc`/`compact`/`flatten` (space
reclamation — irrelevant at 36 issues), `backup`, `restore`.

### Sync, memory, integrations
- `bd dolt push|pull` — real tracker sync via `refs/dolt/data` (remote already configured).
  `bd bootstrap` rebuilds a fresh clone (proven here 2026-08-26).
- `bd remember` / `memories` / `recall` / `forget` — keyed freeform text, injected into
  `bd prime` output (capped by `prime.max-memories` / `prime.max-memory-chars`). **Whether
  memories sync via `bd dolt push` is not documented anywhere the researcher could find** —
  unverified.
- `bd prime` (~1–2k tokens) via SessionStart hook is upstream's deliberate alternative to an
  MCP server (10–50k tokens of schemas); MCP server exists for shell-less clients only, and
  has **no sync tool** at all. Our CLI+hooks setup is the upstream-recommended pattern.
- `github`/`gitlab`/`jira`/`linear`/`ado` sync, `federation` (peer workspaces), `repo`
  (multi-repo hydration) — no current need.
- `bd rules audit|compact` — scans Claude rules for contradictions; curiosity, untested.

## 3. Recommendations (ranked; nothing applied)

**Adopt now — zero setup, immediate fit:**
1. **`bd orphans` + `bd stale --days 14` in the session-close protocol and the Phase-6
   doc-reality sweep** (doc-reality-auditor's lane). Orphans mechanically closes the gap our
   commit-sha convention creates; RESUME.md's stale `.15` mystery is exactly the class of
   drift `bd stale` surfaces.
2. **`bd batch` for wave setup** — one transaction for N creates + deps instead of N×2 CLI
   calls; also faster and atomically rollback-safe.
3. ~~**`BEADS_ACTOR=<agent-name>` in every worker brief**~~ — **SUPERSEDED 2026-08-27
   by §5 (adopted conventions).** The hook this relied on is inert; attribution is
   `--author <roster-name>` on `bd comments add`. Do not set `BEADS_ACTOR`.
4. **`bd q` and `bd dep <blocker> --blocks <blocked>`** as the house idioms in briefs (the
   latter already is).

**Adopt with one config decision (user call):**
5. **`bd config set validation.on-create warn`** — mechanizes the existing
   "--acceptance always" convention; `warn`, not `error`, so it never blocks a worker.
6. **Telemetry**: decided — telemetry is **disabled** (`metrics.disabled=true`,
   user-level config; see §5). The "Currently ON" state recorded here at survey time
   (2026-08-26) is historical. No action remains.
7. **`bd defer --until`** to replace informal "not now" issues (the roadmap already has a
   deferred cohort; ❄ status renders in `bd ready`/`list`).

**Evaluate with a cheap prototype (real design question, don't decide on paper):**
8. **One formula for the Phase-4 gate chain** (wiring-auditor → domain gate → opus reviewer
   as a 3-step proto, poured per story). If it earns its keep, consider encoding the full
   8-phase dev cycle per iteration. Counter-argument: our phases are judgment-gated by the
   lead, not structurally gated, and molecules only *structure* work (no execution hooks) —
   the win is visibility (`bd graph --html`), not automation. Prototype before committing.
9. **`bd human`** for cross-session parked decisions (asset-spend confirmations, scope
   calls found mid-wave when the user is away). Overlaps AskUserQuestion in-session; only
   worth it if unattended runs become common.

**Do not adopt:**
10. **`bd remember` stays banned; MEMORY.md remains the memory system** (honest
    re-evaluation, as agreed): bd memories are flat keyed strings, injection-capped and
    truncatable, with an **unverified sync story**, no structure, no cross-links, no
    per-agent scoping. Our MEMORY.md system is git-versioned, structured, per-agent, and
    already read by every agent. Running both creates a second memory surface that will
    drift. Revisit only if a properly tested release ships the events/sync features and
    documents memory sync.
11. `gate`/`merge-slot`/`swarm`/`set-state`/`audit`/`federation`/external-tracker syncs —
    machinery for unattended multi-rig fleets we don't run.
12. `bd doctor` anything — embedded-mode blocked, full stop.

**Standing cautions:**
- Pin bd at 1.2.2; treat `bd upgrade` as a deliberate, checked action (schema-skew trap).
- Docs pages for events journal / work leases / sync federation / HTTP API describe an
  unreleased version — do not brief agents from them.
- Keep the manual `bd export -o .beads/issues.jsonl` step; `export.auto` is off and the
  hook will not do it for us. (Alternative: `bd config set export.auto true` and let the
  pre-commit hook own it — viable, but verify once before trusting, per house rule.)
- `bd rename-prefix --repair` remains forbidden on this workspace (2026-08-26 incident).

## 4. Open questions / unverified

- `bd remember` sync mechanism (Dolt-carried or local-only) — needs a two-clone test or Go
  source read; moot while it stays unadopted.
- Whether the beads Claude Code plugin is installed anywhere here — if it ever is, `bd setup
  claude` must NOT also install its SessionStart hook (double `bd prime` = double tokens).
- `export.auto=true` behavior — untested live; verify once if we switch to it.

## 5. Adopted conventions (2026-08-26; moved here from CLAUDE.md 2026-08-28)

These were resident in the root `CLAUDE.md` and are recorded here instead. `CLAUDE.md` keeps a
pointer plus the two direction-reversal traps that a session must not be able to miss.

- **Attribution is `--author <roster-name>` on `bd comments add`** — that works. The old
  `BEADS_ACTOR`/`Executed-By:` trailer convention was DELETED 2026-08-27: bd 1.2.2's
  `prepare-commit-msg` hook is inert (verified — 0 of 60 commits carry the trailer). Do not
  set `BEADS_ACTOR`; do not cite `Executed-By:` trailers as provenance.
- **Wave setup goes through `bd batch`**: N creates + deps as one transaction (stdin grammar:
  `create <type> <priority> <title>`, `dep add <from> <to>`, `close <id> [reason]`).
- **Session close adds a drift sweep**: `bd stale --days 14` and `bd orphans` (issues cited in
  commit messages but never closed — the failure our commit-sha convention creates).
- **Postponed ≠ blocked**: use `bd defer <id> --until <date> --reason "…"` instead of an
  open issue worded "not now". `bd undefer` reverses.
- **`validation.on-create = warn` is active** (config.yaml): creating without `--acceptance`
  warns, never blocks. Keep acceptance criteria first-class.
- **Gate-chain formula**: `.beads/formulas/gate-chain.formula.toml` encodes the Phase-4 chain
  (wiring → domain → reviewer). Pour per story: `bd mol pour gate-chain --var story=<id>
  --var scope=<range>`. Molecules structure work only — no execution hooks; epics do not
  auto-close, sweep with `bd epic close-eligible`.
- **Version is pinned at 1.2.2** — a recovery re-release of 1.1.2. Never run `bd upgrade`
  casually (1.2.1 schema-skew trap); `bd doctor` does not work in embedded mode; upstream doc
  pages on work leases / events journal / sync federation / HTTP API describe an unreleased
  version. Telemetry is disabled (`metrics.disabled=true`, user-level config).

## 6. Open question from §4 — RESOLVED and FIXED 2026-08-28 (double `bd prime`)

§4 asked whether the beads Claude Code plugin was installed, and warned that `bd setup claude`
must not also install a SessionStart hook, because "double `bd prime` = double tokens".

**The prediction was correct.** The plugin is installed (`beads@beads-marketplace`, 31 lifetime
uses) and two SessionStart hooks were each running `bd prime`:

| # | Registered in | Command | Scope |
|---|---|---|---|
| 1 | beads plugin `.claude-plugin/plugin.json` → `hooks.SessionStart` | `bd prime` | user — every project |
| 2 | this repo's `.claude/settings.json` → `SessionStart` | `bd prime --hook-json` | project — checked in |

Payloads are the same content, 5,573 vs 5,574 chars (one trailing newline). The duplicate cost
**~1,390 est. tokens per session**.

**Fix applied 2026-08-28:** hook #2 was deleted from `.claude/settings.json`. The plugin now owns
`bd prime` alone, and it covers **both** `SessionStart` and `PreCompact` (the plugin registers
both; the project settings registered only SessionStart, so PreCompact recovery is a net gain).
Verified: `bd prime` in a directory with no bd workspace prints nothing and exits 0, so the
**Addendum 2026-09-03 — the duplication is back.** The installed beads plugin
(1.2.2, `beads-marketplace`) still registers its own user-scope `SessionStart`
`bd prime` (plus `PreCompact`), verified in its
`.claude-plugin/plugin.json` today — and this repo's `.claude/settings.json`
`SessionStart` again carries a project-scope `bd prime` entry (line 9). Both fire
every session, so the ~1,390-token duplicate cost measured above applies again;
the 2026-08-28 fix has regressed. One truthful state: **two `bd prime` hooks are
currently registered, not one.** This survey is advisory and changes no hook
config: either re-delete the project entry (restoring the §6 fix and re-accepting
its no-plugin-clone consequence) or keep it deliberately for self-containment and
record the duplicate cost as accepted. Lead's call.

**Known consequence (of the 2026-08-28 plugin-only state; not current while the
project entry above exists):** this repo is no longer self-contained for `bd prime`. A clone without the

**Trap for whoever audits hooks next — this is why it went unnoticed for so long.** A plugin's
`hooks` key takes **two different shapes**: an inline object (beads) or a string path to a hooks
file (compass, ponytail). Searching for `hooks.json` files finds the second kind and misses the
first entirely — and under `~/.claude/plugins` it also surfaces `.codex-plugin` and
`.cursor-plugin` variants that Claude Code never reads. Enumerate both shapes, and restrict to
`.claude-plugin`, with:

```bash
find ~/.claude/plugins/cache -path '*/.claude-plugin/plugin.json' -exec jq -r '
  select(.hooks != null)
  | .name as $n
  | if (.hooks|type) == "object"
    then "\($n): INLINE -> \(.hooks|keys|join(","))"
    else "\($n): FILE   -> \(.hooks)"
    end' {} + | sort -u
```

Verified output 2026-08-28 — three plugins register hooks, and only one does it inline:

```
beads:    INLINE -> PreCompact,SessionStart
compass:  FILE   -> ./hooks/hooks.json
ponytail: FILE   -> ./hooks/claude-codex-hooks.json
```

Do not use a bare `-maxdepth` here; the cache nests as
`<marketplace>/<plugin>/<version>/.claude-plugin/plugin.json`, and a depth guess silently
returns nothing. `-path` is what makes the sweep exhaustive.

An earlier draft of this section blamed the `--hook-json` output envelope. That was wrong:
`bd prime --hook-json` emits one clean `{"hookSpecificOutput":{"additionalContext":…}}` object
with the text exactly once. The duplication was always two registered hooks.
