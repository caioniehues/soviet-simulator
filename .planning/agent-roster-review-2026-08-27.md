# Agent roster review — 2026-08-27

**Kind:** review record
**Authority:** findings only; the lead dispositions them
**Status:** findings open, none fixed except where noted
**Scope:** 16 project agents in `.claude/agents/`, 5 global in `~/.claude/agents/`
**Reviewed at:** HEAD `4084518`
**Reviewers:** `doc-reality-auditor` (bodies, opus/high, 170k tokens) · `claude-code-guide` (frontmatter, opus, 115k tokens) · `wiring-auditor` (runtime probe, opus, 40k tokens)

---

## 0. What was PROVEN by running it, not read in a doc

These three came from a live probe and from two agents' own experience. They outrank every doc-derived claim below.

### 0.1 LSP does not exist in subagents — CONFIRMED

Probe (`wiring-auditor`, whose definition lists `LSP`), verbatim:

```
Error: No such tool available: LSP. LSP is disabled for this session, in subagents as well as here.
ToolSearch("select:LSP,ListAgents") -> No matching deferred tools found
```

Not recoverable. The `doc-reality-auditor` hit the identical wall independently and fell back to `grep -n`.

**The message is misleading.** LSP is NOT disabled for this session — `documentSymbol` on `market.rs` returned a full symbol table in the main session minutes later. It is absent in subagents specifically.

Subagent toolset, measured:

| Present | Absent |
|---|---|
| `Read`, `Bash`, `ToolSearch`, `Skill`, `Write`, `Edit`, `SendMessage` (deferred) | `LSP`, `ListAgents`, `Grep`, `Glob`, `Agent`, `WebFetch` |

`Grep` and `Glob` are on the documented background-survivor list and were still absent, so the background filter does not explain them. `autoMode` is the likely second cause.

### 0.2 `memory: project` grants `Write` — CONFIRMED

`wiring-auditor.md` lists neither `Write` nor `Edit`. The probe called `Write` and it succeeded:
`File created successfully at: /tmp/wiring-auditor-probe.txt`

Docs: enabling memory auto-enables `Read`, `Write`, `Edit`. Path scope is **undocumented**.
Affects 12 agents whose prompts promise read-only: `debugger`, `doc-reality-auditor`, `evidence-auditor`,
`kornai-economist`, `ledger-invariant-checker`, `logistics-modeller`, `settlement-modeller`,
`soviet-authenticity`, `substrate-cartographer`, `utilities-modeller`, `wiring-auditor`, global `reviewer`.

Caveat: all of them have `Bash`, so the guarantee was never real at the tool layer.

### 0.3 New agent definitions are not hot-loaded

Created `~/.claude/agents/zz-probe-disallowed.md`, dispatched it, got
`Agent type 'zz-probe-disallowed' not found`. The roster loads at session start.
**Consequence:** the `disallowedTools`-vs-memory experiment needs a session restart.

---

## 1. Consequences for what this repo already asserts

Three surfaces are wrong because of 0.1. All three are mine, written today.

| Surface | Wrong text | Reality |
|---|---|---|
| `~/.claude/rules/tool-discipline.md` | "Subagents whose definition lists LSP in `tools:` have it preloaded" | They do not. It is stripped |
| `docs/reference/code-intelligence.md:52` | "Every agent definition in `.claude/agents/` already says this" | 13 of 16 — and the instruction is unfollowable anyway |
| 16 agent bodies (commit `e8744c5`) | "warm LSP with one `documentSymbol` call" | Impossible in a subagent |
| `~/.claude/hooks/lsp-pre-delegation.js` header | "the original premise ('subagents can't access MCP') doesn't hold for the built-in LSP tool" | The original premise was right |

Every brief written this session carried the unfollowable instruction. Two agents burned turns on it.

---

## 2. Body findings (`doc-reality-auditor`) — 17 total

### RANK 1 — costs a whole dispatch

**B1. `doc-reality-auditor.md:47` — the drift-catcher carries the drift**
Says "sonnet implements, opus reviews and does open-ended verification".
Reality: `grep -c '^model: opus' .claude/agents/*.md` = 16 of 16; `delegation.md` and
`development-cycle.md:54` both record the 2026-08-27 uniform-opus decision.
Effect: every Phase-6 wrap flags all 16 agents as mis-tiered and dispatches a correction
against a decision made the same day.
*Fix:* delete the parenthetical, or replace with "(uniform opus/high, 2026-08-27)".

**B2. `logistics-modeller.md:88-92` — advisor sent to re-litigate a shipped decision**
Says `sov-dispatch-wedge-ab4` is open and "**This is your design question** ... Answer it."
Reality: `bd show` → CLOSED, commit `7e4b82f`, "Option C (no store->consumer dispatches,
eat-time settlement)". `sim-implementer.md:94` and `ledger-invariant-checker.md:37-39` both
correctly say FIXED. Only logistics still calls it open.
*Fix:* rewrite as decided precedent; move live follow-ups `sov-jcl`/`sov-xyx`/`sov-abs` into the slot.

**B3. Five bench gates attributed to a charter that names none**
`perf-engineer.md:21-25`, `settlement-modeller.md:74`, `development-cycle.md:238` and `:52` all say
the charter names `bench_services`/`bench_terrain`/`bench_chains`/`bench_rail`/`bench_save` at 250k.
Reality: `grep -ni bench docs/plan/charter-1.0.md` → ONE hit, saying the opposite —
"the relevant implementation and release plans define the benchmark gates" (charter:55-57).
No `[[bench]]` in any Cargo.toml, no `benches/` dir, no bench name anywhere in `.rs`.
`sov-1ae` ("Build the fixed-seed 250k benchmark contract") is OPEN.
*Fix:* say the benches do not exist yet and point at `sov-1ae`. Drop the charter attribution.

**B4. `evidence-auditor.md:92-94` — wrong path (my error from earlier today)**
Says `evid-spec-bindings.json` is in `docs/generated/evidence/`.
Reality: it is at `docs/plan/iterations/evidence/evid-spec-bindings.json` — deliberately, because
it is a generator INPUT (`build_evidence.py:17`), and inputs stayed under `docs/plan/`.
Effect: the Phase-3 binding check — the one that catches unexecuted evidence — is silently skipped.
*Fix:* `docs/generated/evidence/target-scenarios.json` and `docs/plan/iterations/evidence/evid-spec-bindings.json`.

**B5. Lane collision on `simulation/src/economy/market.rs` — three gates, one roster row**
`development-cycle.md:178` has one "domain advisor" row for Phase 4. But
`kornai-economist.md:50` claims market.rs (capital, trade matching, dispatch ledger),
`logistics-modeller.md:80` claims market.rs (the `Dispatch` state machine),
`ledger-invariant-checker.md:3` runs on any diff touching "economy, market, dispatch, storage or trade",
and `development-cycle.md:262-263` says kornai is "consulted wherever" allocation or shortage is touched.
**Concrete, and open right now:** `sov-jcl` (unbounded Loading retry) lands in
`Market::advance_dispatches` (market.rs:736). All three gates fit. A lead cannot tell whether the
single advisor row means one of them or all three.
*Fix:* split the Phase-4 advisor row by file, not by cluster.

**B6. `engine/`, `geom/`, `networking/`, `common/` have no implementer lane**
`development-cycle.md:142-144` assigns exactly three lanes: `simulation/`, `native_app/`,
`base_mod`+`prototypes`. The workspace has 13 crates.
Unowned: engine 12,461 lines, geom 10,523, networking 2,055, common 1,292 — ~26,000 lines,
more than all of `native_app/`. Commit `bc555d9` (two commits ago) edited two of them.
Today it silently falls to the global `implementer`, against the scope-precedence rule.
*Fix:* extend `sim-implementer`'s scope, or add an explicit "no in-repo owner" line.

### RANK 2 — costs a gate pass or a wrong ruling

**B7. `wiring-auditor.md:38` and `substrate-cartographer.md:99-100` never got the LSP warm rule.**
13 of 16 have it; these two carry only the older "preloaded, no ToolSearch needed" variant.
These are the two worst places: `wiring-auditor` is the FIRST Phase-4 gate and its whole output is
REACHABLE/TEST-ONLY/DEAD from `findReferences`; `substrate-cartographer` is Phase 0 and its
fact-sheet is pasted into every brief. **Superseded by finding 0.1** — LSP is absent for both anyway.

**B8. `debugger` and `evidence-auditor` both mutate-and-revert production code; neither knows the other exists.**
`debugger.md:53-58` ("scratch probe prints ... revert exactly ... `git diff` must show no residue")
and `evidence-auditor.md:41-45,55` ("flip a condition, delete a guard ... Always revert").
Collision: whichever reverts second wipes the other's live mutation, and `debugger.md:56` will read
the other's edits as its own residue. Verdict PLAUSIBLE — structural, no observed instance.
*Fix:* one line in each: never instrument a file another agent is mutating; check with the lead.

**B9. The three implementer bodies do not know `debugger` exists.**
`grep -rn debugger` over sim/ui/data-implementer → no output. `sim-implementer.md:60` still says
"Bug fix = root cause". A sim-implementer handed an unknown-cause bug is told to root-cause in-lane
with no hand-back path — the exact behaviour `debugger` was created to stop.
*Fix:* add "cause unknown → stop and report; that is `debugger`'s lane".

**B10. `soviet-authenticity` is a Phase-4 gate in one table and refuses to gate in its own body.**
`development-cycle.md:255,258-259` lists it as an advisor holding Phase-4 sign-off.
`development-cycle.md:41` gives it phase "0 / —". `soviet-authenticity.md:106-107`: "Advisory,
always. **You never gate a merge.**"
*Fix:* state advisory-only in the domain-advisors table.

**B11. `settlement-modeller.md` — two stale citation blocks.**
`:34` cites `souls/human.rs:267-269` and `market.rs:216` for job-opening barter. Real locations:
`human.rs:272` (`m.buy(... "job-opening" ...)`) and `market.rs:585`/`:605`.
`:67-70` pins constants (school 12/cycle, university 3/cycle, seats `× 5/4`, hospital beds 100,
serve rate 3) against `requirements/settlement.md` — which contains none of those numbers.
Same defect at `utilities-modeller.md:74-77` (0.99/0.85/0.93/0.97/0.60 absent from `utilities.md`).
*Fix:* repoint at the archived legacy corpus and mark unratified, or delete.

**B12. `utilities-modeller.md:80-82` — correct conclusion, false stated premise.**
Says `grep -rniE "weather|climate|temperature|season" simulation/src` "returns **zero hits**".
It returns 5, all surnames in `souls/names.txt`. The conclusion (no weather subsystem) holds.
*Fix:* "returns 5 hits, all surnames in `souls/names.txt` — no weather code exists."

**B13. `soviet-authenticity.md:56-58,93-95` — a fabricated quotation and an unusable fix.**
Quotes art direction as "Gritty, weathered, materially honest" — `grep -rn` over `docs/` finds it
nowhere. Real text is `art-direction.md:10-13`. Also `:59` paraphrases the asset rule as "no
extracted assets"; the real rule (`art-direction.md:13`) permits them with a recorded receipt.
Its worked example prescribes "the oxide/ochre/concrete triple from `colors.lua`" — `colors.lua`
has no such entries; those hex values live in `art-direction.md`, which itself says "No single
current module enforces" them.
Related: `bd show sov-z9x` (OPEN) already records two false claims in `art-direction.md`, and
`soviet-authenticity.md:55` says "read this first" without warning of them.

### RANK 3 — drifted duplicates and quality

**B14. Duplicate-block map, with drift**

| Block | Copies | Drift |
|---|---|---|
| LSP preamble | 13/16 | 2 stale variants (B7), 1 correct exemption |
| `cargo test` + static-mut + `init.rs:85-86` | 9 | **none** — all 9 agree and the citation is exact |
| W&R install path + grammar counts | 6 | **"1,472 `.ini` files"** — real: 1,472 total files, **488** `.ini`. Only `substrate-cartographer.md:55` is right; 4 others drifted |
| `optout_exttrade` 1-of-21 | 6 | none |
| `TestCtx::tick()` limits | 5 | **drifted — see below** |
| truck-vs-train substrate | 3 | `goods_company.rs:129` → real line `:132` |
| sentinel anecdote | 3 | told in past tense; still live (B15) |

**The `TestCtx::tick()` drift matters most.** Three bodies deny it proves determinism —
`evidence-auditor.md:69-70` ("**cannot detect a simulation desync at all**"), `debugger.md:83-85`
("**Do not accept it as a determinism proof**"), `sim-implementer.md:39-40`. But
`logistics-modeller.md:107-108` uses it AS the determinism proof, inside a hard-sign-off criterion:
"The sim bincode-round-trips and hash-compares every tick."
Also unmentioned by all five: `sov-qi8` is OPEN — the determinism check is known-flaky on
`transport_grid` (FnvHashMap serialize order not round-trip-stable).

**B15. The sentinel exemplar is still an open defect; two bodies imply it was closed.**
`wiring-auditor.md:32` and `evidence-auditor.md:27-28` tell it as history. Ran it:
```
cargo test -p simulation sentinel
running 0 tests
test result: ok. 0 passed; 45 filtered out
```
`scenarios/mod.rs:8-10` still documents the scheme; zero test functions match.
`sov-journey-sentinels-rxa` is OPEN.

**B16. Three vague instructions with no observable output** (out of 16 bodies — unusually disciplined).
`soviet-authenticity.md:83` "Be blunt" · `settlement-modeller.md:96` "Cite demographic and queueing
models where they sharpen a decision" · `perf-engineer.md:74` "record the machine state" (never defined;
the Report section at `:90` asks for different fields).

**B17. Two bodies name no failure mode**, against `development-cycle.md:10-11`.
`perf-engineer.md` (borrows another agent's incident) and `settlement-modeller.md` (states rules with
no incident behind them). B3 hands perf-engineer a real one.

---

## 3. Frontmatter findings (`claude-code-guide`) — 8 total

**F1. Adding `Skill` was NECESSARY, not redundant.** `tools:` is an allowlist; omission is denial, and
the docs name `Skill` as the example. Before the edit all 16 could not invoke ANY skill — including
`/dev-cycle`, which `CLAUDE.md` calls the process front door.
**But** the docs recommend `skills:` for disciplines you always want: "To preload Skills into context,
use the `skills` field rather than listing `Skill` here." None of the 16 has a `skills:` field.

**F2. An unrecognised `tools:` entry is silently ignored.** The launch refusal fires only when EVERY
entry fails. One typo among ten good entries evaporates with no warning. `claude plugin validate`
passed while `Skills` was present — it checks parseability, not tool-name resolution.
**FIXED** — `settlement-modeller.md:4` `Skills` → `Skill`, 16/16 now consistent.

**F3. `memory: project` re-enables `Write`/`Edit`.** See 0.2. Proven by probe.

**F4. `Agent` in a subagent works.** Depth 3 below main by default; `CLAUDE_CODE_MAX_SUBAGENT_SPAWN_DEPTH`
not set. `Agent` is exempt from the background filter. `subagent_type` resolves from the same scopes.
**Trap:** `Agent(type1, type2)` parentheses are IGNORED in a subagent definition — it grants
unrestricted spawning while looking restricted. Restrict via `permissions.deny: ["Agent(name)"]`.
**Cost:** only the top-level summary returns; depth-2 token attribution is **undocumented**. Nested
agents share the same concurrency pool (`CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS = 16`).

**F5. `Workflow` would be a guaranteed no-op** — stripped from every subagent, foreground or background.

**F6. `color: magenta` is not a valid value.** `kornai-economist.md:8`, `ui-implementer.md:8`.
Accepted: red, blue, green, yellow, purple, orange, pink, cyan.

**F7. `skills:` is ignored when a definition runs as a teammate.** Latent — agent teams need
`CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1`, which is not set. `teammateMode: "auto"` only picks the
display mode. Worst case is global `reviewer`: `skills:` AND no `Skill` in tools — as a teammate it
would get no playbook and no way to fetch one.

**F8. Three descriptions break strict YAML** (unquoted `: `): `data-implementer.md:3`,
`kornai-economist.md:3`, `team-lead.md:3`. Claude Code parses them fine; portability nit only.

**Verified correct:** all 20 `name` fields; all `description` fields in the right form; `model: opus`
valid; **`effort` is a real field** with valid values (and not a no-op — session effort is `medium`);
`memory` scoping correct; every tool spelling except `Skills`; `SendMessage` and `ToolSearch` present
in all 20 — the delegation rule is not violated anywhere.

---

## 4. Clean bill — checked and accurate

- **Every Workers & Resources grammar count is exact** (14 tokens verified by recount):
  `$STORAGE` 314, `$WORKERS_NEEDED` 156, `$CONSUMPTION` 146, `$PRODUCTION` 89, `$VEHICLE_STATION` 558,
  `$CONNECTION_ADVANCED_POINT` 2180, `$CONNECTION_ROAD_DEAD` 1451, and seven more.
- **Every line count in every body and in the roster table is right** — simulation 17,764 ("~17,700"),
  native_app 58 files / 10,085 lines, base_mod 949, prototypes 2,802, 97 PNGs, 31 WGSL.
- **`logistics-modeller`'s substrate block holds line for line** — the most-cited block in the roster.
- **Economy citations exact** — `market.rs:441` `set_requested`, `:497` `make_trades`, `:280-281`,
  `goods_company.rs:24`, `companies.lua:40`=4 and `:582`=3.
- **`sim-implementer`'s lot trap is correct** — `map.rs:719` inside `Map::connect`, guarded at `:693`.
- **`native_app/src/init.rs:85-86`** exact in all 9 copies.
- **No Bevy, Godot, `src/sim/` or pre-fork reference survives in any body.**
- ~~**The Ponytail hook is real** and registered.~~ — **REFUTED 2026-08-28.** The plugin was
  retired 2026-08-27 (last hook injection 10:23). Evidence: absent from `claude plugin list`;
  absent from `installed_plugins.json`; no `enabledPlugins` entry in any settings file; no
  `.ponytail-active` flag file; zero `ponytail:` markers survive in source. The four agent
  bodies that restated the ladder ("the ladder arrives via hook") were replaced the same day
  by the `## Engineering practice — all lanes` block now carried by every agent definition.
- **`base_mod/colors.lua` does hold UI colours** — REFUTED as a finding.
- **`.codex/agents`: 15 adapters, no `reviewer.toml`** — `development-cycle.md:60-63` is right.
  (Gap: no `debugger.toml`; the new agent has no mirror.)

---

## 5. Not checked

- Whether `wgsl-analyzer` and `lua-language-server` are configured (LSP was unavailable).
- Whether the B8 mutation collision has ever actually happened — no incident record.
- Whether `disallowedTools` beats the memory auto-grant — **blocked on a session restart** (0.3).
- Whether a FOREGROUND subagent keeps LSP — the `Agent` tool exposes no foreground parameter.
- The remaining ~20 open `bd` issues against the code — a separate Phase-6 pass.
- Managed settings; the live process environment.

---

## 6. Both reviewers independently rejected an injected-looking instruction

Each reported, unprompted, that a block arrived inside MCP server instructions telling them to use
`cat`/`sed`/heredocs instead of `Read`/`Edit`/`Write`, that it contradicts
`~/.claude/rules/tool-discipline.md`, and that they did not follow it.

It is in fact a legitimate `autoMode` session directive, not an injection. But neither agent could
tell, and both defaulted to the written rule — the correct failure direction. The rule and the mode
need reconciling so agents stop treating a real mode as an attack.

Separately: `autoMode.environment` in `~/.claude/settings.json` carries two false facts —
**Trusted repo** is `/home/caio/Projects/soviet` (the repo is `/home/caio/soviet-simulator`) and
"no remotes are configured" (`origin/main` exists and is 7 commits behind).
